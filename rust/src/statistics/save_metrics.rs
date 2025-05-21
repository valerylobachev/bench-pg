use crate::model::metrics::{Action, DomainMetric};
use chrono::NaiveDate;
use sqlx::PgPool;
use sqlx::types::Decimal;

const BATCH_SIZE: usize = 100;

pub async fn save_metrics(pool: PgPool, benchmark_id: i64, metrics: &Vec<DomainMetric>) {
    let mut offset: usize = 0;
    while offset < metrics.len() {
        let batch = get_batch(benchmark_id, offset, metrics);
        sqlx::query(
            r#"
                insert into bm_metrics (
                            benchmark_id, year, period, index, user_no,
                            action, material_id, business_partner_id, quantity, price,
                            amount, doc_no, sale_doc_no, cogs_doc_no, posting_date, duration
                ) select * from unnest(
                    $1::int8[],
                    $2::int4[],
                    $3::int4[],
                    $4::int8[],
                    $5::int4[],
                    $6::varchar[],
                    $7::varchar[],
                    $8::varchar[],
                    $9::numeric[],
                    $10::numeric[],
                    $11::numeric[],
                    $12::varchar[],
                    $13::varchar[],
                    $14::varchar[],
                    $15::date[],
                    $16::float8[]
                )"#,
        )
        .bind(batch.iter().map(|r| r.benchmark_id).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.year).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.period).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.index).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.user_no).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| &r.action).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| &r.material_id).collect::<Vec<_>>())
        .bind(
            batch
                .iter()
                .map(|r| &r.business_partner_id)
                .collect::<Vec<_>>(),
        )
        .bind(batch.iter().map(|r| r.quantity).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.price).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.amount).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| &r.doc_no).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| &r.sale_doc_no).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| &r.cogs_doc_no).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.posting_date).collect::<Vec<_>>())
        .bind(batch.iter().map(|r| r.duration).collect::<Vec<_>>())
        .execute(&pool)
        .await
        .expect(format!("failed to insert metrics batch from {}", offset).as_str());
        offset += BATCH_SIZE;
    }
}

fn get_batch(benchmark_id: i64, offset: usize, metrics: &Vec<DomainMetric>) -> Vec<MetricRecord> {
    let size = if offset + BATCH_SIZE < metrics.len() {
        BATCH_SIZE
    } else {
        metrics.len() - offset
    };
    let mut records: Vec<MetricRecord> = Vec::with_capacity(size);
    for i in offset..(offset + size) {
        records.push(MetricRecord::from(
            metrics.get(i).unwrap().clone(),
            benchmark_id,
        ))
    }
    records
}

struct MetricRecord {
    pub benchmark_id: i64,
    pub year: i32,
    pub period: Option<i32>,
    pub index: i64,
    pub user_no: i32,
    pub action: &'static str,
    pub material_id: Option<String>,
    pub business_partner_id: Option<String>,
    pub quantity: Option<Decimal>,
    pub price: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub doc_no: Option<String>,
    pub sale_doc_no: Option<String>,
    pub cogs_doc_no: Option<String>,
    pub posting_date: Option<NaiveDate>,
    pub duration: f64,
}

impl MetricRecord {
    fn from(metric: DomainMetric, benchmark_id: i64) -> Self {
        let mut record = MetricRecord {
            benchmark_id,
            year: metric.year as i32,
            period: metric.period.map(|p| p as i32),
            index: metric.index as i64,
            user_no: metric.user_no as i32,
            action: metric.action.code(),
            material_id: None,
            business_partner_id: None,
            quantity: None,
            price: None,
            amount: None,
            doc_no: None,
            sale_doc_no: None,
            cogs_doc_no: None,
            posting_date: None,
            duration: metric.duration,
        };

        match metric.action {
            Action::Purchase(p) => {
                record.price = Some(p.price);
                record.quantity = Some(p.quantity);
                record.doc_no = Some(p.doc_no);
                record.business_partner_id = Some(p.vendor.id());
                record.material_id = Some(p.material.id());
                record.posting_date = Some(p.posting_date);
            }
            Action::Sale(p) => {
                record.quantity = Some(p.quantity);
                record.sale_doc_no = Some(p.sale_doc_no);
                record.cogs_doc_no = Some(p.cogs_doc_no);
                record.business_partner_id = Some(p.customer.id());
                record.material_id = Some(p.material.id());
                record.posting_date = Some(p.posting_date);
            }
            Action::Cost(p) => {
                record.amount = Some(p.amount);
                record.doc_no = Some(p.doc_no);
                record.business_partner_id = Some(p.vendor.id());
                record.material_id = Some(p.material.id());
                record.posting_date = Some(p.posting_date);
            }
            Action::ClosePeriod(p) => record.period = Some(p.period() as i32),
            Action::OpenPeriod(p) => record.period = Some(p.period() as i32),
            Action::PeriodReport(p) => record.period = Some(p.period() as i32),
            Action::YearReport(p) => record.period = Some(p.period() as i32),
            _ => {}
        }

        record
    }
}
