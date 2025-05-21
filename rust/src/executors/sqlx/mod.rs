mod close_period;
mod cost;
mod init;
mod open_period;
mod post_document;
mod purchase;
mod report;
mod sale;

use crate::api::ExecutorApi;
use crate::model::domain::{Account, Cost, Period, Purchase, Sale, User};
use self::close_period::close_period;
use self::cost::account_cost;
use self::open_period::open_period;
use self::purchase::purchase_material;
use self::report::report;
use self::sale::sell_material;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub struct SqlxExecutor {
    pool: PgPool,
}

impl SqlxExecutor {
    pub async fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        db: &str,
        connections: u32,
    ) -> SqlxExecutor {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, db
        );
        let pool = PgPoolOptions::new()
            .max_connections(connections)
            .connect(connection_string.as_str())
            .await
            .expect("Failed to create postgres connection pool");

        SqlxExecutor { pool }
    }
}

#[async_trait::async_trait]
impl ExecutorApi for SqlxExecutor {
    async fn init(
        &self,
        start_year: u32,
        customers: u32,
        vendors: u32,
        materials: u32,
        accounts: Vec<Account>,
        purchases: Vec<Purchase>,
    ) {
        let pool = self.pool.clone();

        init::clear_tables(&pool).await;
        init::load_business_partners(customers, vendors, &pool).await;
        init::load_materials(materials, start_year, &pool).await;
        init::load_accounts(&pool, accounts).await;
        for purchase in purchases {
            purchase_material(&pool, &purchase, User(0)).await;
        }
        open_period(&pool, Period::new(start_year as i32, 1), User(0)).await;
    }

    async fn purchase_material(&self, operation: &Purchase, user: User) {
        let pool = self.pool.clone();
        purchase_material(&pool, operation, user).await;
    }

    async fn sell_material(&self, operation: &Sale, user: User) {
        let pool = self.pool.clone();
        sell_material(&pool, operation, user).await;
    }

    async fn account_cost(&self, operation: &Cost, user: User) {
        let pool = self.pool.clone();
        account_cost(&pool, operation, user).await;
    }

    async fn open_period(&self, period: Period, user: User) {
        let pool = self.pool.clone();
        open_period(&pool, period.next_period(), user).await;
    }

    async fn close_period(&self, period: Period, user: User) {
        let pool = self.pool.clone();
        close_period(&pool, period.prev_period(), user).await;
    }

    async fn period_report(&self, period: Period) {
        let pool = self.pool.clone();
        report(&pool, period, period).await;
    }

    async fn year_report(&self, period: Period) {
        let pool = self.pool.clone();
        let start_period = period.first_period();
        let end_period = period.last_period();
        report(&pool, start_period, end_period).await;
    }
}
