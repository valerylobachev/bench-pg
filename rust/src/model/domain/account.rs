pub const INVENTORY_ACCOUNT: &str = "10.01";
pub const INVENTORY_DIFF_ACCOUNT: &str = "10.02";
pub const RECEIVABLE_ACCOUNT: &str = "62.01";
pub const PAYABLE_ACCOUNT: &str = "60.01";
pub const SALES_ACCOUNT: &str = "90.01";
pub const COGS_ACCOUNT: &str = "90.02";

pub const DEBT: &str = "D";
pub const CREDIT: &str = "C";

#[derive(Debug, Clone)]
pub struct Account{
    pub id: String,
    pub name: String,
}
impl Account {
    pub fn new(aid: &str, aname: &str) -> Account {
        Account { id: aid.to_string(), name: aname.to_string() }
    }

    pub fn chart_of_accounts() -> Vec<Account> {
        vec![
            Account::new(INVENTORY_ACCOUNT, "Inventory"),
            Account::new(INVENTORY_DIFF_ACCOUNT, "Inventory differences"),
            Account::new(RECEIVABLE_ACCOUNT, "Account receivable"),
            Account::new(PAYABLE_ACCOUNT, "Account payable"),
            Account::new(SALES_ACCOUNT, "Sales"),
            Account::new(COGS_ACCOUNT, "Cost of goods sold"),
        ]
    }
}