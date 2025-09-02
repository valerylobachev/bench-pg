use crate::executors::diesel::schema::fin_ledger_items;
use crate::model::domain::Period;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rust_decimal::Decimal;

pub async fn report(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    start_period: Period,
    end_period: Period,
) {
    let start_period_yp = start_period.year_period();
    let end_period_yp = end_period.year_period();

    let _start_balance = fin_ledger_items::dsl::fin_ledger_items
        .group_by(fin_ledger_items::dsl::account_id)
        .select((
            fin_ledger_items::dsl::account_id,
            sum(fin_ledger_items::dsl::amount),
        ))
        .filter(fin_ledger_items::dsl::period.lt(start_period_yp))
        .load::<(String,  Option<Decimal>)>(conn)
        .expect("Unable to retrieve ledger items");

    let _turnaround = fin_ledger_items::dsl::fin_ledger_items
        .group_by(fin_ledger_items::dsl::account_id)
        .select((
            fin_ledger_items::dsl::account_id,
            sum(fin_ledger_items::dsl::debt),
            sum(fin_ledger_items::dsl::credit),
        ))
        .filter(fin_ledger_items::dsl::period.ge(start_period_yp))
        .filter(fin_ledger_items::dsl::period.le(end_period_yp))
        .load::<(String, Option<Decimal>, Option<Decimal>)>(conn)
        .expect("Unable to retrieve ledger items");
}
