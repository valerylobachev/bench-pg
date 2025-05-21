use super::Action;

#[derive(Debug, Clone)]
pub struct DomainMetric {
    pub year: u32,
    pub period: Option<u32>,
    pub index: usize,
    pub user_no: u32,
    pub action: Action,
    pub duration: f64,
}
