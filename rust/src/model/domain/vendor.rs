#[derive(Debug, Clone, Copy)]
pub struct Vendor(pub u32);

impl Vendor {
    pub fn id(&self) -> String {
        format!("VEND-{:05}", self.0)
    }

    pub fn name(&self) -> String {
        format!("Vendor {:05}", self.0)
    }
}