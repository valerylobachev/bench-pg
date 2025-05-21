use super::{Customer, Vendor};

#[derive(Debug, Clone)]
pub enum BusinessPartner {
    Customer(Customer),
    Vendor(Vendor),
}

impl BusinessPartner {
    pub fn id(&self) -> String {
        match self {
            BusinessPartner::Customer(customer) => customer.id(),
            BusinessPartner::Vendor(vendor) => vendor.id(),
        }
    }
}
