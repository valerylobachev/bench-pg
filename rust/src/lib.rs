use crate::config::Config;
use crate::executors::create_executor;

mod benchmark;
pub mod config;
mod api;
mod model;
mod executors;
mod statistics;

pub async fn run(config: Config) {
    let executor = create_executor(
        &config.username,
        &config.password,
        &config.host,
        config.port,
        &config.db,
        config.connections,
        &config.lib,
    )
    .await;

    let metrics = benchmark::run(config.clone(), executor).await;
    
    statistics::run(config, metrics).await;
}
