use crate::model::domain::Period;
use crate::model::metrics::{Action, DomainMetric, Statistics};
use lazy_static::lazy_static;
use sqlx::{Pool, Postgres};

pub async fn save_statistics(
    pool: Pool<Postgres>,
    benchmark_id: i64,
    metrics: &Vec<DomainMetric>,
    start_year: u32,
    years: u32,
) {

    for rule in PROCESSING_RULES.iter() {
        let action_metrics = metrics.iter().filter(rule.filter_fn).collect::<Vec<_>>();
        let durations = action_metrics
            .iter()
            .map(|m| m.duration)
            .collect::<Vec<_>>();
        let total_stat = calculate_statistics(durations);
        save_stat(pool.clone(), benchmark_id, None, None, rule.action, total_stat).await;

        for year in start_year..(start_year + years) {
            let year_metrics = action_metrics
                .iter()
                .filter(|m| m.year == year)
                .collect::<Vec<_>>();
            let durations = year_metrics.iter().map(|m| m.duration).collect::<Vec<_>>();
            let stat = calculate_statistics(durations);
            save_stat(
                pool.clone(),
                benchmark_id,
                Some(year as i32),
                None,
                rule.action,
                stat,
            )
                .await;

            if rule.calc_periods {
                for month in 1..=12 {
                    let period = Period::new(year as i32, month).month();
                    let durations = year_metrics
                        .iter()
                        .filter(|m| m.period.map(|p| p == period).unwrap_or(false))
                        .map(|m| m.duration)
                        .collect::<Vec<_>>();
                    let stat = calculate_statistics(durations);
                    save_stat(
                        pool.clone(),
                        benchmark_id,
                        Some(year as i32),
                        Some(month),
                        rule.action,
                        stat,
                    )
                        .await;
                }
            }
        }
    }
}


pub struct StatCondition {
    action: &'static str,
    filter_fn: fn(&&DomainMetric) -> bool,
    calc_periods: bool,
}

impl StatCondition {
    fn new(
        action: &'static str,
        filter_fn: fn(&&DomainMetric) -> bool,
        calc_periods: bool,
    ) -> StatCondition {
        StatCondition {
            action,
            filter_fn,
            calc_periods,
        }
    }
}

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref PROCESSING_RULES: Vec<StatCondition> = vec![
        StatCondition::new("PROCESS_YEAR", filter_by_process_year, false),
        StatCondition::new("PROCESS_PERIOD", filter_by_process_period, false),
        StatCondition::new("YEAR_REPORT", filter_by_year_report, true),
        StatCondition::new("PERIOD_REPORT", filter_by_period_report, true),
        StatCondition::new("OPEN_PERIOD", filter_by_open_period, false),
        StatCondition::new("CLOSE_PERIOD", filter_by_close_period, false),
        StatCondition::new("COST", filter_by_cost, true),
        StatCondition::new("SALE", filter_by_sale, true),
        StatCondition::new("PURCHASE", filter_by_purchase, true),
    ];
}


async fn save_stat(
    pool: Pool<Postgres>,
    benchmark_id: i64,
    year: Option<i32>,
    month: Option<i32>,
    action: &str,
    s: Statistics,
) {
    sqlx::query!(
        r#" insert into bm_statistics (
                            benchmark_id, action, year, month, total_count, total_duration, ops_per_sec, 
                            min, p50, p75, p95, p99, p99_9, max, mean, std_dev
                )
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)"#,
        benchmark_id,
        action,
        year,
        month,
        s.total_count as i64,
        s.total_duration,
        s.ops_per_sec,
        s.min,
        s.p50,
        s.p75,
        s.p95,
        s.p99,
        s.p99_9,
        s.max,
        s.mean,
        s.std_dev,
    )
        .execute(&pool)
        .await
        .expect("Cannot insert statistics");
}

fn calculate_statistics(durations: Vec<f64>) -> Statistics {
    let mut percs = inc_stats::Percentiles::new();
    let mut stats = inc_stats::SummStats::new();
    durations.iter().for_each(|d| {
        percs.add(*d);
        stats.add(*d);
    });
    let total_count = percs.count();
    let total_duration = durations.iter().sum::<f64>();
    let p50 = percs.percentile(0.5).unwrap_or(Some(0.0)).unwrap_or(0.0);
    let p75 = percs.percentile(0.75).unwrap_or(Some(0.0)).unwrap_or(0.0);
    let p95 = percs.percentile(0.95).unwrap_or(Some(0.0)).unwrap_or(0.0);
    let p99 = percs.percentile(0.99).unwrap_or(Some(0.0)).unwrap_or(0.0);
    let p99_9 = percs.percentile(0.999).unwrap_or(Some(0.0)).unwrap_or(0.0);
    let statistics = Statistics {
        total_count,
        total_duration,
        ops_per_sec: total_count as f64 / total_duration,
        min: stats.min().unwrap_or(0.0),
        p50,
        p75,
        p95,
        p99,
        p99_9,
        max: stats.max().unwrap_or(0.0),
        mean: stats.mean().unwrap_or(0.0),
        std_dev: stats.standard_deviation().unwrap_or(0.0),
    };
    statistics
}

fn filter_by_process_year(m: &&DomainMetric) -> bool {
    match m.action {
        Action::ProcessYear => true,
        _ => false,
    }
}

fn filter_by_process_period(m: &&DomainMetric) -> bool {
    match m.action {
        Action::ProcessPeriod => true,
        _ => false,
    }
}
fn filter_by_year_report(m: &&DomainMetric) -> bool {
    match m.action {
        Action::YearReport(_) => true,
        _ => false,
    }
}
fn filter_by_period_report(m: &&DomainMetric) -> bool {
    match m.action {
        Action::PeriodReport(_) => true,
        _ => false,
    }
}
fn filter_by_open_period(m: &&DomainMetric) -> bool {
    match m.action {
        Action::OpenPeriod(_) => true,
        _ => false,
    }
}
fn filter_by_close_period(m: &&DomainMetric) -> bool {
    match m.action {
        Action::ClosePeriod(_) => true,
        _ => false,
    }
}
fn filter_by_cost(m: &&DomainMetric) -> bool {
    match m.action {
        Action::Cost(_) => true,
        _ => false,
    }
}
fn filter_by_sale(m: &&DomainMetric) -> bool {
    match m.action {
        Action::Sale(_) => true,
        _ => false,
    }
}
fn filter_by_purchase(m: &&DomainMetric) -> bool {
    match m.action {
        Action::Purchase(_) => true,
        _ => false,
    }
}
