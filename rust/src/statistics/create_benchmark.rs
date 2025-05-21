use chrono::Utc;
use sqlx::PgPool;
use crate::config::Config;

pub async fn create_benchmark(pool: PgPool, config: &Config) -> i64 {

    sqlx::query!(
        r#" insert into bm_benchmarks (name, date, db_lib, customers, vendors, materials,
                                        users, start_year, years, operations)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) 
              returning id"#,
        &config.name,
        Utc::now(),
        format!("{:?}", config.lib),
        config.customers as i32,
        config.vendors as i32,
        config.materials as i32,
        config.users as i32,
        config.start_year as i32,
        config.years as i32,
        config.operations as i32,
    )
        .fetch_one(&pool)
        .await
        .expect("Cannot insert benchmark")
        .id
}