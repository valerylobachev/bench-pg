mod init;
mod open_period;
mod post_document;
mod purchase;
mod sale;
mod report;
mod cost;
mod close_period;

use crate::api::ExecutorApi;
use crate::model::domain::{Account, Cost, Period, Purchase, Sale, User};
use deadpool_postgres::tokio_postgres::NoTls;
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod, tokio_postgres};

pub struct TokioExecutor {
    pool: Pool,
}

impl TokioExecutor {
    pub async fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        db: &str,
        connections: u32,
    ) -> TokioExecutor {
        let mut pg_config = tokio_postgres::Config::new();
        pg_config.user(username);
        pg_config.password(password);
        pg_config.host(host);
        pg_config.port(port);
        pg_config.dbname(db);
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
        let pool = Pool::builder(mgr)
            .max_size(connections as usize)
            .build()
            .unwrap();

        TokioExecutor { pool }
    }
}

#[async_trait::async_trait]
impl ExecutorApi for TokioExecutor {
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
        let mut conn = pool.get().await.unwrap();

        init::clear_tables(&mut conn).await;
        init::load_business_partners(customers, vendors, &mut conn).await;
        init::load_materials(materials, start_year, &mut conn).await;
        init::load_accounts(&mut conn, accounts).await;
        for purchase in purchases {
            purchase::purchase_material(&mut conn, &purchase, User(0)).await;
        }
        open_period::open_period(&mut conn, Period::new(start_year as i32, 1), User(0)).await;
    }

    async fn purchase_material(&self, operation: &Purchase, user: User) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
        purchase::purchase_material(&mut conn, operation, user).await;
    }

    async fn sell_material(&self, operation: &Sale, user: User) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
        sale::sell_material(&mut conn, operation, user).await;
    }

    async fn account_cost(&self, operation: &Cost, user: User) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
       cost::account_cost(&mut conn, operation, user).await;
    }

    async fn open_period(&self, period: Period, user: User) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
        open_period::open_period(&mut conn, period.next_period(), user).await;
    }

    async fn close_period(&self, period: Period, user: User) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
       close_period::close_period(&mut conn, period.prev_period(), user).await;
    }

    async fn period_report(&self, period: Period) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
        report::report(&mut conn, period, period).await;
    }

    async fn year_report(&self, period: Period) {
        let pool = self.pool.clone();
        let mut conn = pool.get().await.unwrap();
        let start_period = period.first_period();
        let end_period = period.last_period();
        report::report(&mut conn, start_period, end_period).await;
    }
}
