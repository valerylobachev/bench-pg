#[derive(Debug, Clone, Copy)]
pub struct Material(pub u32);

impl Material {
    pub fn id(&self) -> String {
        format!("MAT-{:05}", self.0)
    }

    pub fn name(&self) -> String {
        format!("Material {:05}", self.0)
    }
}