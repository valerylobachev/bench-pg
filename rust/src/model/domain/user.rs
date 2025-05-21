
#[derive(Debug, Clone, Copy)]
pub struct User(pub u32);

impl User {
    pub fn id(&self) -> String {
        format!("USER-{:05}", self.0)
    }
}
