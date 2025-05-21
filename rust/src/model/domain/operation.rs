use super::{Customer, Material, Period, Vendor};
use chrono::NaiveDate;
use sqlx::types::Decimal;

#[derive(Debug, Clone)]
pub struct Purchase {
    pub material: Material,
    pub vendor: Vendor,
    pub quantity: Decimal,
    pub price: Decimal,
    pub doc_no: String,
    pub posting_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct Sale {
    pub material: Material,
    pub customer: Customer,
    pub quantity: Decimal,
    pub sale_doc_no: String,
    pub cogs_doc_no: String,
    pub posting_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub struct Cost {
    pub material: Material,
    pub vendor: Vendor,
    pub amount: Decimal,
    pub doc_no: String,
    pub posting_date: NaiveDate,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Purchase(Purchase),
    Sale(Sale),
    Cost(Cost),
    ClosePeriod(Period),
    OpenPeriod(Period),
    PeriodReport(Period),
    YearReport(Period),
}
