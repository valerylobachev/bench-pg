use crate::model::domain::{Material, Purchase, Vendor};
use chrono::NaiveDate;
use sqlx::types::Decimal;

pub fn init_purchases(start_year: u32, vendors: u32, materials: u32) -> Vec<Purchase> {
    let mut res: Vec<Purchase> = vec![];
    for i in 0..materials {
        let material = Material(i);
        let vendor = Vendor(rand::random_range(0..vendors));
        let price = Decimal::from(rand::random_range(100..200));
        let quantity = Decimal::from(1_000 * rand::random_range(100..200));
        res.push(Purchase {
            material,
            vendor,
            price,
            quantity,
            posting_date: NaiveDate::from_ymd_opt(start_year as i32 - 1, 12, 31).unwrap(),
            doc_no: format!("INIT-{:08}", i),
        })
    }
    res
}


