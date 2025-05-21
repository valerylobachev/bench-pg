use bench_pg_rs::config::Config;
use bench_pg_rs::run;

#[tokio::main]
async fn main() {
    let config = Config::new();
    dbg!(&config);
    run(config).await;
}
