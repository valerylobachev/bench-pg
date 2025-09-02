use bench_pg_rs::config::Config;
use bench_pg_rs::run;

// #[tokio::main(flavor = "multi_thread", worker_threads = 56)]
#[tokio::main]
async fn main() {
    let config = Config::new();
    dbg!(&config);
    run(config).await;
}
