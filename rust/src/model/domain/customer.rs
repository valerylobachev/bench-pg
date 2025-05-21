#[derive(Debug, Clone, Copy)]
pub struct Customer(pub u32);

impl Customer {
    pub fn id(&self) -> String {
        format!("CUST-{:05}", self.0)
    }

    pub fn name(&self) -> String {
        format!("Customer {:05}", self.0)
    }
}