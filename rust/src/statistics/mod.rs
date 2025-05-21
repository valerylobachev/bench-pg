mod pool;
mod create_benchmark;
mod save_metrics;
mod save_statistics;

use crate::config::Config;
use crate::model::metrics::DomainMetric;
use crate::statistics::create_benchmark::create_benchmark;
use crate::statistics::pool::get_pool;
use crate::statistics::save_metrics::save_metrics;
use crate::statistics::save_statistics::save_statistics;

pub async fn run(config: Config, metrics: Vec<DomainMetric>) {
    let pool = get_pool(
        &config.username,
        &config.password,
        &config.host,
        config.port,
        &config.db,
        config.connections,
    ).await;

    let benchmark_id = create_benchmark(pool.clone(), &config).await;
    save_metrics(pool.clone(), benchmark_id, &metrics).await;
    save_statistics(pool.clone(), benchmark_id, &metrics, config.start_year, config.years).await;

    
}



