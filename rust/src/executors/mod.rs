use crate::api::ExecutorApi;
use crate::config::DbLib;
use std::sync::Arc;

mod diesel;
mod sqlx;
mod tokio;

pub async fn create_executor(
    username: &str,
    password: &str,
    host: &str,
    port: u16,
    db: &str,
    connections: u32,
    lib: &DbLib,
) -> Arc<dyn ExecutorApi> {
    match lib {
        DbLib::Sqlx => Arc::new(
            sqlx::SqlxExecutor::new(&username, &password, &host, port, &db, connections).await,
        ),
        DbLib::Diesel => Arc::new(
            diesel::DieselExecutor::new(&username, &password, &host, port, &db, connections).await,
        ),
        DbLib::Tokio => Arc::new(
            tokio::TokioExecutor::new(&username, &password, &host, port, &db, connections).await,
        ),
    }
}
