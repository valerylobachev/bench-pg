use crate::benchmark::task::{Task, TaskResult};
use crate::api::ExecutorApi;
use crate::model::domain::{Operation, Period, User};
use crate::model::metrics::{Action, DomainMetric};
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::mpsc::{Receiver, Sender};

pub async fn user_worker(
    user: User,
    mut task_receiver: Receiver<Task>,
    result_sender: Sender<TaskResult>,
    executor: Arc<dyn ExecutorApi>,
) {
    loop {
        match task_receiver.recv().await {
            Some(Task::Process { index, operation }) => {
                let start = SystemTime::now();
                let result = process_operation(user, operation, executor.clone()).await;
                let duration = start.elapsed().unwrap().as_secs_f64();
                let metric = DomainMetric {
                    year: result.year,
                    period: Some(result.period),
                    index,
                    user_no: user.0,
                    action: result.action,
                    duration,
                };
                result_sender
                    .send(TaskResult(metric))
                    .await
                    .expect(format!("Error sending task result from user {}", user.id()).as_str());
            }
            _ => {
                break;
            }
        }
    }
}

async fn process_operation(
    user: User,
    operation: Operation,
    executor: Arc<dyn ExecutorApi>,
) -> OperationResult {
    match operation {
        Operation::Purchase(operation) => {
            executor.purchase_material(&operation, user).await;
            let yp = Period::from(operation.posting_date);
            OperationResult::new(yp.year(), yp.period(), Action::Purchase(operation))
        }
        Operation::Sale(operation) => {
            executor.sell_material(&operation, user).await;
            let yp = Period::from(operation.posting_date);
            OperationResult::new(yp.year(), yp.period(), Action::Sale(operation))
        }
        Operation::Cost(operation) => {
            executor.account_cost(&operation, user).await;
            let yp = Period::from(operation.posting_date);
            OperationResult::new(yp.year(), yp.period(), Action::Cost(operation))
        }
        Operation::PeriodReport(period) => {
            executor.period_report(period).await;
            OperationResult::new(period.year(), period.period(), Action::PeriodReport(period))
        }
        Operation::YearReport(period) => {
            executor.year_report(period).await;
            OperationResult::new(period.year(), period.period(), Action::YearReport(period))
        }
        Operation::OpenPeriod(period) => {
            let start = SystemTime::now();
            executor.open_period(period, user).await;
            let duration = start.elapsed().unwrap();
            println!("Open period {} done in {:?}", period.next_period(), duration);
            OperationResult::new(period.year(), period.period(), Action::OpenPeriod(period))
        }
        Operation::ClosePeriod(period) => {
            let start = SystemTime::now();
            executor.close_period(period, user).await;
            let duration = start.elapsed().unwrap();
            println!("Closing period {} done in {:?}", period.prev_period(), duration);
            OperationResult::new(period.year(), period.period(), Action::ClosePeriod(period))
        }
    }
}

struct OperationResult {
    pub year: u32,
    pub period: u32,
    pub action: Action,
}

impl OperationResult {
    fn new(year: u32, period: u32, action: Action) -> Self {
        OperationResult {
            year,
            period,
            action,
        }
    }
}
