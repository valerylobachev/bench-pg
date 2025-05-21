use crate::model::domain::Period;
use tokio_postgres::Client;

pub async fn report(client: &mut Client, start_period: Period, end_period: Period) {
    let start_period_yp = start_period.year_period();
    let end_period_yp = end_period.year_period();

    let _start_balance = client
        .query(
            r#"select account_id, sum(amount) as amount 
                 from fin_ledger_items
                 where period < $1
                 group by account_id
               "#,
            &[&start_period_yp],
        )
        .await
        .expect("Unable to retrieve ledger items");

    let _turnaround = client
        .query(
            r#"select account_id, sum(debt) as debt, sum(credit) as credit 
                 from fin_ledger_items
                 where period >= $1 and period <= $2
                 group by account_id
               "#,
            &[&start_period_yp, &end_period_yp],
        )
        .await
        .expect("Unable to retrieve ledger items");
}
