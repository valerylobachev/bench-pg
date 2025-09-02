use crate::executors::diesel::schema::fin_ledger_items;
use crate::model::domain::{DebtCredit, Document, Period};
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, RunQueryDsl};
use sqlx::types::Decimal;

pub fn post_document(
    tx: &mut PooledConnection<ConnectionManager<PgConnection>>,
    document: Document,
) -> Result<(), String> {
    let balance = document
        .lines
        .iter()
        .map(|line| line.amount)
        .sum::<Decimal>();
    if balance != Decimal::from(0) {
        return Err(format!("Balance = {} must be zero.", balance));
    }

    let period_yp = Period::from(document.posting_date).year_period();
    let user_id = document.updated_by.id();
    let updated_at = Utc::now();

    for line in document.lines  {
        let (debt, credit) = match line.debt_credit {
            DebtCredit::Debt => (&line.amount.clone(), &Decimal::from(0)),
            DebtCredit::Credit => (&Decimal::from(0), &-line.amount.clone()),
        };

        diesel::insert_into(fin_ledger_items::dsl::fin_ledger_items)
            .values((
                fin_ledger_items::dsl::period.eq(period_yp),
                fin_ledger_items::dsl::doc_no.eq(&document.doc_no),
                fin_ledger_items::dsl::posting_date.eq(document.posting_date),
                fin_ledger_items::dsl::account_id.eq(line.account),
                fin_ledger_items::dsl::business_partner_id
                    .eq(line.business_partner.clone().map(|bp| bp.id())),
                fin_ledger_items::dsl::material_id.eq(line.material.map(|m| m.id())),
                fin_ledger_items::dsl::debt_credit.eq(line.debt_credit.to_db_option()),
                fin_ledger_items::dsl::amount.eq(&line.amount),
                fin_ledger_items::dsl::debt.eq(debt),
                fin_ledger_items::dsl::credit.eq(credit),
                fin_ledger_items::dsl::quantity.eq(line.quantity.unwrap_or(Decimal::from(0))),
                fin_ledger_items::dsl::updated_by.eq(&user_id),
                fin_ledger_items::dsl::updated_at.eq(&updated_at),
            ))
            .execute(tx)
            .map_err(|e| e.to_string())?;
    };

    Ok(())
}
