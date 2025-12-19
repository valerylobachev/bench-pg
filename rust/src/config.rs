use clap::Parser;
use serde::Serialize;

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DbLib {
    #[default]
    Tokio,
    Sqlx,
    Diesel,
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short = 'U', long, default_value = "postgres")]
    pub username: String,
    #[arg(short = 'P', long, default_value = "postgres")]
    pub password: String,
    #[arg(short = 'H', long, default_value = "localhost")]
    pub host: String,
    #[arg(short, long, default_value_t = 5432)]
    pub port: u16,
    #[arg(short = 'C', long, default_value_t = 20)]
    pub connections: u32,
    #[arg(short, long, default_value = "benchmark")]
    pub db: String,
    #[arg(short, long, default_value_t, value_enum)]
    pub lib: DbLib,
    #[arg(short, long, default_value_t = 100)]
    pub customers: u32,
    #[arg(short, long, default_value_t = 100)]
    pub vendors: u32,
    #[arg(short, long, default_value_t = 100)]
    pub materials: u32,
    #[arg(short, long, default_value_t = 12)]
    pub users: u32,
    #[arg(short, long, default_value_t = 2025)]
    pub start_year: u32,
    #[arg(short, long, default_value_t = 1)]
    pub years: u32,
    #[arg(short, long, default_value_t = 20000)]
    pub operations: usize,
    #[arg(short, long, default_value = "")]
    pub name: String,
}

impl Config {
    pub fn new() -> Config {
        let mut config = Config::parse();
        if config.name.is_empty() {
            config.name = format!(
                "Benchmark {:?} with {} users, {} operations, {} materials, {} years",
                &config.lib, config.users, config.operations, config.materials, config.years
            );
        }
        config
    }
}
