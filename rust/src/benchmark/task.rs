use crate::model::domain::Operation;
use crate::model::metrics::DomainMetric;

pub struct Task {
    pub index: usize,
    pub operation: Operation,
}
pub struct TaskResult(pub DomainMetric);
