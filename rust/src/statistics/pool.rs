use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn get_pool(
    username: &str,
    password: &str,
    host: &str,
    port: u16,
    db: &str,
    connections: u32,
) -> PgPool {
    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable",
        username, password, host, port, db
    );
    PgPoolOptions::new()
        .max_connections(connections)
        .connect(connection_string.as_str())
        .await
        .expect("Failed to create postgres connection pool")
}
