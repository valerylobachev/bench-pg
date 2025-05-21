use crate::model::domain::{Account, Cost, Period, Purchase, Sale, User};

#[allow(async_fn_in_trait)]
#[async_trait::async_trait]
pub trait ExecutorApi: Sync + Send {
    async fn init(
        &self,
        start_year: u32,
        customers: u32,
        vendors: u32,
        materials: u32,
        accounts: Vec<Account>,
        purchases: Vec<Purchase>,
    );
    async fn purchase_material(&self, operation: &Purchase, user: User);
    async fn sell_material(&self, operation: &Sale, user: User);
    async fn account_cost(&self, operation: &Cost, user: User);
    async fn open_period(&self, period: Period, user: User);
    async fn close_period(&self, period: Period, user: User);
    async fn period_report(&self, period: Period);
    async fn year_report(&self, period: Period);
}
