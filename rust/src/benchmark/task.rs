use crate::model::domain::Operation;
use crate::model::metrics::DomainMetric;

pub enum Task {
    Process{
        index: usize,
        operation: Operation
    },
    Done,
}
pub struct TaskResult(pub DomainMetric);