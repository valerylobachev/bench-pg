use crate::model::domain::{COGS_ACCOUNT, CREDIT, DEBT, INVENTORY_ACCOUNT, INVENTORY_DIFF_ACCOUNT, Period, User,};
use chrono::{NaiveDate, Utc};
use diesel::dsl::*;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use rust_decimal::Decimal;
use diesel::{PgConnection, RunQueryDsl};
use crate::executors::diesel::schema::{fin_ledger_items, fin_material_periods, fin_materials};

pub async fn close_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    period: Period,
    user: User,
) {
    let user_id = user.id();

    let materials = fin_materials::dsl::fin_materials
        .select(fin_materials::dsl::id)
        .load::<String>(conn)
        .expect("Error loading materials");

    for material_id in materials {
        let curr_yp = period.year_period();
        let next_yp = period.next_period().year_period();
        let prev_yp = period.prev_period().year_period();

        conn.transaction(|tx| {

            let std_price = fin_material_periods::dsl::fin_material_periods
                .select(
                    fin_material_periods::dsl::std_price,
                )
                .filter(fin_material_periods::dsl::material_id.eq(&material_id))
                .filter(fin_material_periods::dsl::period.eq(curr_yp))
                .for_update()
                .first::<Decimal>(tx)
                .expect("Error getting material period");

            let prev_stock_res = fin_material_periods::dsl::fin_material_periods
                .select(
                    fin_material_periods::dsl::stock,
                )
                .filter(fin_material_periods::dsl::material_id.eq(&material_id))
                .filter(fin_material_periods::dsl::period.eq(prev_yp))
                .first::<Decimal>(tx);

            let prev_stock = match prev_stock_res {
                Ok(ps) => { ps}
                Err(Error::NotFound) => {Decimal::from(0)}
                _ => panic!("Error calculate closing data")
            };

            let receipt = fin_ledger_items::dsl::fin_ledger_items
                .select(
                    sum(fin_ledger_items::dsl::quantity),
                )
                .filter(fin_ledger_items::dsl::period.eq(curr_yp))
                .filter(fin_ledger_items::dsl::material_id.eq(&material_id))
                .filter(fin_ledger_items::dsl::account_id.eq("10.01"))
                .filter(fin_ledger_items::dsl::debt_credit.eq("D"))
                .first::<Option<Decimal>>(tx)
                .expect("Error calculate closing data")
                .unwrap_or(Decimal::from(0));

            let consumption = fin_ledger_items::dsl::fin_ledger_items
                .select(
                    sum(fin_ledger_items::dsl::quantity),
                )
                .filter(fin_ledger_items::dsl::period.eq(curr_yp))
                .filter(fin_ledger_items::dsl::material_id.eq(&material_id))
                .filter(fin_ledger_items::dsl::account_id.eq("10.01"))
                .filter(fin_ledger_items::dsl::debt_credit.eq("C"))
                .first::<Option<Decimal>>(tx)
                .expect("Error calculate closing data")
                .unwrap_or(Decimal::from(0));

            let diff_amount = fin_ledger_items::dsl::fin_ledger_items
                .select(
                    sum(fin_ledger_items::dsl::amount),
                )
                .filter(fin_ledger_items::dsl::period.eq(curr_yp))
                .filter(fin_ledger_items::dsl::material_id.eq(&material_id))
                .filter(fin_ledger_items::dsl::account_id.eq("10.02"))
                .first::<Option<Decimal>>(tx)
                .expect("Error calculate closing data")
                .unwrap_or(Decimal::from(0));

            let next_std_price = fin_material_periods::dsl::fin_material_periods
                .select(
                    fin_material_periods::dsl::std_price,
                )
                .filter(fin_material_periods::dsl::material_id.eq(&material_id))
                .filter(fin_material_periods::dsl::period.eq(next_yp))
                .first::<Decimal>(tx)
                .expect("Error calculate closing data");

            // Общая стоимость: ta2 = s1 * sp2 + r2 * sp2 +d2
            let total_amount = prev_stock * std_price + receipt * std_price + diff_amount;
            // Общий запас: ts2 = s1 + r2
            let total_stock = prev_stock + receipt;
            // Факт цена: ap = ta2 / ts2
            let actual_price = total_amount / total_stock;
            // Запас на конец периода: s3 = s1 + r2 + c2
            let curr_stock = prev_stock + receipt + consumption;

            let doc_no = format!("CLOSE-{}-{}", period, material_id);

            // Отклонения на запас: ds2 = d2 * s3 / (s1 + r2)
            // Проводка Dt 10.01 Ct 10.02 ds2
            let diff_to_stock_amount = diff_amount * curr_stock / (prev_stock + receipt);
            post_differences(
                tx,
                period.last_date(),
                &doc_no,
                &material_id,
                &diff_to_stock_amount,
                INVENTORY_ACCOUNT,
                INVENTORY_DIFF_ACCOUNT,
                &user_id,
            );

            // Отклонения на COGS: dc2 = d2 - ds2
            // Проводка Dt 90.02 Ct 10.02 dc2
            let diff_to_cogs_amount = diff_amount - diff_to_stock_amount;
            post_differences(
                tx,
                period.last_date(),
                &doc_no,
                &material_id,
                &diff_to_cogs_amount,
                COGS_ACCOUNT,
                INVENTORY_DIFF_ACCOUNT,
                &user_id,
            );

            let doc_no = format!("OPEN-{}-{}", next_yp, &material_id);
            // Отклонение след. периода ds3 = s3 * sp2 + ds2 - s3 * sp3
            // Проводка Dt 10.02 Ct 10.01 ds3
            let next_diff_amount =
                curr_stock * std_price + diff_to_stock_amount - curr_stock * next_std_price;
            post_differences(
                tx,
                period.next_period().first_date(),
                &doc_no,
                &material_id,
                &next_diff_amount,
                INVENTORY_DIFF_ACCOUNT,
                INVENTORY_ACCOUNT,
                &user_id,
            );

            // update next_std_price material period
            diesel::update(
                fin_materials::dsl::fin_materials
                    .find(&material_id),
            )
                .set((
                    fin_materials::dsl::next_std_price.eq(actual_price),
                    fin_materials::dsl::updated_by.eq(&user_id),
                    fin_materials::dsl::updated_at.eq(&Utc::now()),
                ))
                .execute(tx)
                .expect("Error updating material period");

            // update actual price
            diesel::update(
                fin_material_periods::dsl::fin_material_periods
                    .find((&material_id, period.year_period())),
            )
                .set((
                    fin_material_periods::dsl::actual_price.eq(actual_price),
                    fin_material_periods::dsl::updated_by.eq(&user_id),
                    fin_material_periods::dsl::updated_at.eq(&Utc::now()),
                ))
                .execute(tx)
                .expect("Error updating material period");

            Ok::<(), Error>(())
        })
            .expect(
            format!(
                "Cannot commit close period transaction for material {} in period {}",
                &material_id, period,
            )
                .as_str(),
        );
    }
}

fn post_differences(
    tx: &mut PooledConnection<ConnectionManager<PgConnection>>,
    posting_date: NaiveDate,
    doc_no: &String,
    material_id: &String,
    amount: &Decimal,
    debt_account: &str,
    credit_account: &str,
    user_id: &String,
) {
    if *amount == Decimal::from(0) {
        return;
    }

    let (amount, debt_account, credit_account) = if *amount < Decimal::from(0) {
        (&-amount, credit_account, debt_account)
    } else {
        (amount, debt_account, credit_account)
    };
    let period = Period::from(posting_date);
    let updated_at = Utc::now();

    diesel::insert_into(fin_ledger_items::dsl::fin_ledger_items)
        .values((
            fin_ledger_items::dsl::period.eq(period.year_period()),
            fin_ledger_items::dsl::doc_no.eq(doc_no),
            fin_ledger_items::dsl::posting_date.eq(&posting_date),
            fin_ledger_items::dsl::account_id.eq(debt_account),
            fin_ledger_items::dsl::material_id.eq(material_id),
            fin_ledger_items::dsl::debt_credit.eq(DEBT),
            fin_ledger_items::dsl::amount.eq(&amount),
            fin_ledger_items::dsl::debt.eq(&amount),
            fin_ledger_items::dsl::credit.eq(Decimal::from(0),),
            fin_ledger_items::dsl::quantity.eq(Decimal::from(0)),
            fin_ledger_items::dsl::updated_by.eq(&user_id),
            fin_ledger_items::dsl::updated_at.eq(&updated_at),
        ))
        .execute(tx)
        .expect("Cannot insert post_differences line 1");

    diesel::insert_into(fin_ledger_items::dsl::fin_ledger_items)
        .values((
            fin_ledger_items::dsl::period.eq(period.year_period()),
            fin_ledger_items::dsl::doc_no.eq(doc_no),
            fin_ledger_items::dsl::posting_date.eq(&posting_date),
            fin_ledger_items::dsl::account_id.eq(credit_account),
            fin_ledger_items::dsl::material_id.eq(material_id),
            fin_ledger_items::dsl::debt_credit.eq(CREDIT),
            fin_ledger_items::dsl::amount.eq(&-amount),
            fin_ledger_items::dsl::debt.eq(Decimal::from(0)),
            fin_ledger_items::dsl::credit.eq(&amount),
            fin_ledger_items::dsl::quantity.eq(Decimal::from(0)),
            fin_ledger_items::dsl::updated_by.eq(&user_id),
            fin_ledger_items::dsl::updated_at.eq(&updated_at),
        ))
        .execute(tx)
        .expect("Cannot insert post_differences line 2");
}
