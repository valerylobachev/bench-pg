use self::init::init_purchases;
use self::prepare_operations::prepare_operations;
use self::task::{Task, TaskResult};
use self::user_worker::user_worker;
use super::config::Config;
use super::api::ExecutorApi;
use super::model::domain::{Account, Period, User};
use super::model::metrics::{Action, DomainMetric};
use std::sync::Arc;
use std::time::SystemTime;

mod init;
mod prepare_operations;
mod task;
mod user_worker;

pub async fn run(config: Config, executor: Arc<dyn ExecutorApi>) -> Vec<DomainMetric> {
    executor
        .init(
            config.start_year,
            config.customers,
            config.vendors,
            config.materials,
            Account::chart_of_accounts(),
            init_purchases(config.start_year, config.vendors, config.materials),
        )
        .await;

    let mut metrics = Vec::new();
    let start = SystemTime::now();
    for year in config.start_year..(config.start_year + config.years) {
        let year_metrics = run_year(year, &executor, &config).await;
        metrics.extend(year_metrics);
    }
    let duration = start.elapsed().unwrap();
    println!("Processing {} years done in {:?}", config.years, duration);
    metrics
}

async fn run_year(
    year: u32,
    executor: &Arc<dyn ExecutorApi>,
    config: &Config,
) -> Vec<DomainMetric> {
    let mut metrics = vec![];
    println!("Processing year {}", year);
    let start = SystemTime::now();
    for period in 1..=12 {
        let period_metrics =
            run_period(Period::new(year as i32, period), executor, config).await;
        metrics.extend(period_metrics);
    }
    let duration = start.elapsed().unwrap();
    println!("Processing year {} done in {:?}", year, duration);
    metrics.push(DomainMetric {
        year,
        period: None,
        index: 0,
        user_no: 0,
        action: Action::ProcessYear,
        duration: duration.as_secs_f64(),
    });
    metrics
}

async fn run_period(
    period: Period,
    executor: &Arc<dyn ExecutorApi>,
    config: &Config,
) -> Vec<DomainMetric> {
    let mut metrics = vec![];
    println!("Processing period: {}", period);

    let operations = prepare_operations(period, config);

    let start = SystemTime::now();
    let (result_sender, mut result_receiver) = tokio::sync::mpsc::channel::<TaskResult>(10000);
    let mut set = tokio::task::JoinSet::new();
    let user_senders = (0..config.users)
        .map(|u| {
            let (task_sender, task_receiver) = tokio::sync::mpsc::channel::<Task>(10000);
            let user = User(u);
            let result_sender = result_sender.clone();
            let executor = executor.clone();
            set.spawn(async move {
                user_worker(user, task_receiver, result_sender, executor).await
            });
            task_sender
        })
        .collect::<Vec<_>>();

    let mut operation_index = 0;

    while operation_index < operations.len() && operation_index < user_senders.len() {
        let task_sender = user_senders[operation_index].clone();
        let operation = operations[operation_index].clone();
        task_sender
            .send(Task::Process {
                index: operation_index,
                operation,
            })
            .await
            .expect(
                format!(
                    "Error sending task operation to user {}",
                    User(operation_index as u32).id()
                )
                .as_str(),
            );
        operation_index += 1;
    }
    
    let in_process = operation_index;

    while operation_index < operations.len() {
        match result_receiver.recv().await {
            Some(TaskResult(metric)) => {
                let user_no = metric.user_no as usize;
                // println!("Metric: {:?}", &metric);
                metrics.push(metric);
                let operation = operations[operation_index].clone();
                user_senders[user_no]
                    .send(Task::Process {
                        index: operation_index,
                        operation,
                    })
                    .await
                    .expect(
                        format!("Error sending task operation to user no {}", user_no).as_str(),
                    );
                operation_index += 1;
            }
            None => break,
        }
    }
    
    for _ in 0..in_process {
        match result_receiver.recv().await {
            Some(TaskResult(metric)) => {
                // println!("Metric: {:?}", &metric);
                metrics.push(metric);
            }
            None => break,
        }
    }

    for (i, user) in user_senders.iter().enumerate() {
        user.send(Task::Done).await.expect(
            format!(
                "Error sending task operation to user {}",
                User(i as u32).id()
            )
            .as_str(),
        );
    }

    set.join_all().await;
    let duration = start.elapsed().unwrap();
    println!("Processing period {} done in {:?}", period, duration);
    metrics.push(DomainMetric {
        year: period.year(),
        period: Some(period.period()),
        index: 0,
        user_no: 0,
        action: Action::ProcessPeriod,
        duration: duration.as_secs_f64(),
    });
    metrics
}
