use crate::config::Config;
use crate::model::domain::{Cost, Customer, Material, Operation, Period, Purchase, Sale, Vendor};
use rand::random_range;
use sqlx::types::Decimal;

pub fn prepare_operations(period: Period, config: &Config) -> Vec<Operation> {
    let mut operations = Vec::with_capacity(config.operations);
    
    for _ in 0..config.operations * 20 / 100 {
        let order = random_range(0..config.operations);
        let operation = Operation::PeriodReport(period);
        operations.push((operation, order));
    }

    for _ in 0..config.operations * 10 / 100 {
        let order = random_range(0..config.operations);
        let operation =  Operation::YearReport(period);
        operations.push((operation, order));
    } 
    for n in 0..config.operations * 20 / 100 {
        let order = random_range(0..config.operations);
        let posting_date = period.date(order as u32 * period.last_day() / config.operations as u32);
        let operation =  Operation::Cost(Cost {
            material: Material(random_range(0..config.materials)),
            vendor: Vendor(random_range(0..config.vendors)),
            amount: Decimal::from(random_range(1000..2000)),
            doc_no: format!("COST-{}-{:08}", period, n),
            posting_date,
        });
        operations.push((operation, order));
    }

    for n in 0..config.operations * 25 / 100 {
        let order = random_range(0..config.operations);
        let posting_date = period.date(order as u32 * period.last_day() / config.operations as u32);
        let operation =  Operation::Purchase(Purchase {
            material: Material(random_range(0..config.materials)),
            vendor: Vendor(random_range(0..config.vendors)),
            quantity: Decimal::from(random_range(1000..2000)),
            price: Decimal::from(random_range(100..200)),
            doc_no: format!("PURCH-{}-{:08}", period, n),
            posting_date,
        });
        operations.push((operation, order));
    } 
    
    for n in 0..config.operations * 25 / 100 {
        let order = random_range(0..config.operations);
        let posting_date = period.date(order as u32 * period.last_day() / config.operations as u32);
        let operation =  Operation::Sale(Sale {
            material: Material(random_range(0..config.materials)),
            customer: Customer(random_range(0..config.customers)),
            quantity: Decimal::from(random_range(100..200)),
            sale_doc_no: format!("SALE-{}-{:08}", period, n),
            cogs_doc_no: format!("COGS-{}-{:08}", period, n),
            posting_date,
        });
        operations.push((operation, order));
    }
    
    operations.push((Operation::OpenPeriod(period), config.operations));
    let close_order = random_range(1..=5) * config.operations / period.last_day() as  usize;
    operations.push((Operation::ClosePeriod(period), close_order));
    operations.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    let operations = operations
        .into_iter()
        .map(|(operation, _)| operation)
        .collect::<Vec<Operation>>();
    operations
}
