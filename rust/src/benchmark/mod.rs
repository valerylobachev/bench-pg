use self::init::init_purchases;
use self::prepare_operations::prepare_operations;
use self::task::{Task, TaskResult};
use self::user_worker::user_worker;
use super::api::ExecutorApi;
use super::config::Config;
use super::model::domain::{Account, Period, User};
use super::model::metrics::{Action, DomainMetric};
use async_channel::bounded;
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
    for month in 1..=12 {
        let period_metrics = run_period(Period::new(year as i32, month), executor, config).await;
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
    println!("Processing period: {}", period);

    let operations = prepare_operations(period, config);
    let operations_count = operations.len();

    let start = SystemTime::now();

    // create channels
    let (task_sender, task_receiver) = bounded::<Task>(operations_count);
    let (result_sender, result_receiver) = bounded::<TaskResult>(operations_count);

    // start workers
    let mut workers = Vec::new();
    for u in 0..config.users {
        let user = User(u);
        let receiver = task_receiver.clone();
        let sender = result_sender.clone();
        let executor = executor.clone();
        workers.push(tokio::spawn(async move {
            user_worker(user, receiver, sender, executor).await
        }));
    }

    tokio::spawn(async move {
        // send tasks
        for (index, operation) in operations.into_iter().enumerate() {
            task_sender
                .send(Task { index, operation })
                .await
                .expect("Failed to send task");
        }
        // close task channel
        drop(task_sender);
        drop(task_receiver);
    });

    tokio::spawn(async move {
        // waiting for workers to complete
        for worker in workers {
            worker.await.expect("Worker failed");
        }
        // close result channel
        drop(result_sender);
    });

    // collect results
    let mut metrics = Vec::with_capacity(operations_count);
    while let Ok(res) = result_receiver.recv().await {
        metrics.push(res.0);
    }

    let duration = start.elapsed().unwrap();
    println!("Processing period {} done in {:?}", period, duration);
    metrics.push(DomainMetric {
        year: period.year(),
        period: Some(period.month()),
        index: 0,
        user_no: 0,
        action: Action::ProcessPeriod,
        duration: duration.as_secs_f64(),
    });
    metrics
}
