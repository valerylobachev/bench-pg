mod init;
mod schema;
mod open_period;

use crate::api::ExecutorApi;
use crate::model::domain::{Account, Cost, Period, Purchase, Sale, User};
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DieselExecutor {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DieselExecutor {
    pub async fn new(
        username: &str,
        password: &str,
        host: &str,
        port: u16,
        db: &str,
        connections: u32,
    ) -> DieselExecutor {
        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            username, password, host, port, db
        );
        let manager = ConnectionManager::<PgConnection>::new(connection_string);
        // Refer to the `r2d2` documentation for more methods to use
        // when building a connection pool
        let pool = Pool::builder()
            .max_size(connections)
            .test_on_check_out(true)
            .build(manager)
            .expect("Failed to create postgres connection pool");

        DieselExecutor { pool }
    }
}

#[async_trait::async_trait]
impl ExecutorApi for DieselExecutor {
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
        let conn = &mut pool.get().unwrap();

        init::clear_tables(conn).await;
        init::load_business_partners(customers, vendors, conn).await;
        init::load_materials(materials, start_year, conn).await;
        init::load_accounts(conn, accounts).await;
        //  for purchase in purchases {
        //      crate::executors::sqlx::purchase::purchase_material(conn, &purchase, User(0)).await;
        //  }
        open_period::open_period(conn, Period::new(start_year as i32, 1), User(0)).await;
    }

    async fn purchase_material(&self, operation: &Purchase, user: User) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::purchase::purchase_material(conn, operation, user).await;
    }

    async fn sell_material(&self, operation: &Sale, user: User) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::sale::sell_material(conn, operation, user).await;
    }

    async fn account_cost(&self, operation: &Cost, user: User) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::cost::account_cost(conn, operation, user).await;
    }

    async fn open_period(&self, period: Period, user: User) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::open_period::open_period(conn, period.next_period(), user).await;
    }

    async fn close_period(&self, period: Period, user: User) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::close_period::close_period(conn, period.prev_period(), user).await;
    }

    async fn period_report(&self, period: Period) {
        // let pool = self.pool.clone();
        // crate::executors::sqlx::report::report(conn, period, period).await;
    }

    async fn year_report(&self, period: Period) {
        // let pool = self.pool.clone();
        // let start_period = period.first_period();
        // let end_period = period.last_period();
        // crate::executors::sqlx::report::report(conn, start_period, end_period).await;
    }
}
