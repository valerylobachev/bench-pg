use crate::model::domain::{DebtCredit, Document, Period};
use chrono::Utc;
use rust_decimal::Decimal;
use tokio_postgres::Transaction;

pub async fn post_document(tx: &Transaction<'_>, document: Document) -> Result<(), String> {
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

    let len = document.lines.len();
    let periods = (0..len).map(|_| period_yp).collect::<Vec<_>>();
    let doc_nos = (0..len).map(|_| &document.doc_no).collect::<Vec<_>>();
    let posting_dates = (0..len).map(|_| &document.posting_date).collect::<Vec<_>>();
    let user_ids = (0..len).map(|_| &user_id).collect::<Vec<_>>();
    let updated_ats = (0..len).map(|_| &updated_at).collect::<Vec<_>>();
    let accounts = document
        .lines
        .iter()
        .map(|line| line.account)
        .collect::<Vec<_>>();
    let business_partners = document
        .lines
        .iter()
        .map(|line| line.business_partner.as_ref().map(|bp| bp.id()))
        .collect::<Vec<_>>();
    let materials = document
        .lines
        .iter()
        .map(|line| line.material.as_ref().map(|m| m.id()))
        .collect::<Vec<_>>();
    let debt_credits = document
        .lines
        .iter()
        .map(|line| line.debt_credit.to_db_option())
        .collect::<Vec<_>>();
    let amounts = document
        .lines
        .iter()
        .map(|line| line.amount)
        .collect::<Vec<_>>();
    let debts = document
        .lines
        .iter()
        .map(|line| {
            if line.debt_credit == DebtCredit::Debt {
                line.amount
            } else {
                Decimal::from(0)
            }
        })
        .collect::<Vec<_>>();
    let credits = document
        .lines
        .iter()
        .map(|line| {
            if line.debt_credit == DebtCredit::Credit {
                -line.amount
            } else {
                Decimal::from(0)
            }
        })
        .collect::<Vec<_>>();
    let quantities = document
        .lines
        .iter()
        .map(|line| line.quantity.unwrap_or(Decimal::from(0)))
        .collect::<Vec<_>>();

    tx.execute(
        r#"
                insert into fin_ledger_items (
                  period, doc_no, posting_date,
                  account_id, business_partner_id, material_id, debt_credit,
                  amount, debt, credit, quantity,
                  updated_by, updated_at
                ) select * from unnest(
                    $1::int4[],
                    $2::varchar[],
                    $3::date[],
                    $4::varchar[],
                    $5::varchar[],
                    $6::varchar[],
                    $7::varchar[],
                    $8::numeric[],
                    $9::numeric[],
                    $10::numeric[],
                    $11::numeric[],
                    $12::varchar[],
                    $13::timestamptz[]
                )"#,
        &[
            &periods,
            &doc_nos,
            &posting_dates,
            &accounts,
            &business_partners,
            &materials,
            &debt_credits,
            &amounts,
            &debts,
            &credits,
            &quantities,
            &user_ids,
            &updated_ats,
        ],
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
