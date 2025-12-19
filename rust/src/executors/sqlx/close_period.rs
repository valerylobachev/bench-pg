use crate::model::domain::{
    COGS_ACCOUNT, CREDIT, DEBT, INVENTORY_ACCOUNT, INVENTORY_DIFF_ACCOUNT, Period, User,
};
use chrono::NaiveDate;
use sqlx::types::Decimal;
use sqlx::{Pool, Postgres, Transaction};

pub async fn close_period(pool: &Pool<Postgres>, period: Period, user: User){
    let user_id = user.id();

    let materials = sqlx::query!("select id from fin_materials")
        .fetch_all(pool)
        .await
        .expect("Error loading materials");

    for material in materials {
        let material_id = material.id;
        let curr_yp = period.year_period();
        let next_yp = period.next_period().year_period();

        let mut tx = pool.begin().await.expect(
            format!(
                "Cannot start close period transaction for material {} in period {}",
                &material_id, period,
            )
            .as_str(),
        );

        let mp = sqlx::query!(
            r#"select std_price from fin_material_periods
                 where material_id = $1 and period = $2 for update;"#,
            &material_id,
            curr_yp,
        )
        .fetch_one(&mut *tx)
        .await
        .expect("Error getting material period");

        let cd = sqlx::query!(
            r#"select 
                 (select stock from fin_material_periods
                   where material_id = $2 and period = $4 ) as prev_stock,
                 (select COALESCE(sum(quantity),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.01' and debt_credit = 'D') as receipt,
                 (select  COALESCE(sum(quantity),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.01' and debt_credit = 'C')  as consumption,
                 (select  COALESCE(sum(amount),0)
                   from fin_ledger_items
                   where period = $1 and material_id = $2 and account_id = '10.02')  as diff_amount,
                 (select std_price from fin_material_periods
                   where material_id = $2 and period = $3 ) as next_std_price"#,
            curr_yp,
            &material_id,
            next_yp,
            period.prev_period().year_period()
        )
            .fetch_one(&mut *tx)
            .await
            .expect("Error calculate closing data");

        let std_price = mp.std_price;
        let prev_stock = cd.prev_stock.unwrap_or(Decimal::from(0));
        let receipt = cd.receipt.unwrap_or(Decimal::from(0));
        let consumption = cd.consumption.unwrap_or(Decimal::from(0));
        let diff_amount = cd.diff_amount.unwrap_or(Decimal::from(0));
        let next_std_price = cd.next_std_price.unwrap_or(Decimal::from(0));

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
            &mut tx,
            period.last_date(),
            &doc_no,
            &material_id,
            &diff_to_stock_amount,
            INVENTORY_ACCOUNT,
            INVENTORY_DIFF_ACCOUNT,
            &user_id,
        )
        .await;

        // Отклонения на COGS: dc2 = d2 - ds2
        // Проводка Dt 90.02 Ct 10.02 dc2
        let diff_to_cogs_amount = diff_amount - diff_to_stock_amount;
        post_differences(
            &mut tx,
            period.last_date(),
            &doc_no,
            &material_id,
            &diff_to_cogs_amount,
            COGS_ACCOUNT,
            INVENTORY_DIFF_ACCOUNT,
            &user_id,
        )
        .await;

        let doc_no = format!("OPEN-{}-{}", next_yp, &material_id);
        // Отклонение след. периода ds3 = s3 * sp2 + ds2 - s3 * sp3
        // Проводка Dt 10.02 Ct 10.01 ds3
        let next_diff_amount =
            curr_stock * std_price + diff_to_stock_amount - curr_stock * next_std_price;
        post_differences(
            &mut tx,
            period.next_period().first_date(),
            &doc_no,
            &material_id,
            &next_diff_amount,
            INVENTORY_DIFF_ACCOUNT,
            INVENTORY_ACCOUNT,
            &user_id,
        )
        .await;

        // update next_std_price material period
        sqlx::query!(
            r#"update fin_materials
                 set next_std_price = $1,
                     updated_by = $3,
                     updated_at = now()
                 where id = $2"#,
            &actual_price,
            &material_id,
            &user_id
        )
        .execute(pool)
        .await
        .expect("Error updating next_std_price in material");

        // update actual price
        sqlx::query!(
            r#"update fin_material_periods set
              actual_price = $1,
              updated_by = $2,
              updated_at = now()
              where material_id = $3 and period = $4"#,
            &actual_price,
            &user_id,
            &material_id,
            period.year_period(),
        )
        .execute(&mut *tx)
        .await
        .expect("Error updating std_price in material period");

        tx.commit().await.expect(
            format!(
                "Cannot commit close period transaction for material {} in period {}",
                &material_id, period,
            )
            .as_str(),
        );
    }
}

async fn post_differences(
    tx: &mut Transaction<'_, Postgres>,
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

    sqlx::query!(
        r#" insert into fin_ledger_items (period, doc_no, posting_date,
                           account_id, material_id, debt_credit,
                           amount, debt, credit, quantity,
                           updated_by, updated_at)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now()) "#,
        period.year_period(),
        doc_no,
        &posting_date,
        debt_account,
        &material_id,
        DEBT,
        &amount,
        &amount,
        Decimal::from(0),
        Decimal::from(0),
        user_id,
    )
    .execute(&mut **tx)
    .await
    .expect("Cannot insert post_differences line 1");

    sqlx::query!(
        r#" insert into fin_ledger_items ( period, doc_no, posting_date,
                           account_id, material_id, debt_credit,
                           amount, debt, credit, quantity,
                           updated_by, updated_at)
              values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, now()) "#,
        period.year_period(),
        doc_no,
        &posting_date,
        credit_account,
        &material_id,
        CREDIT,
        &-amount,
        Decimal::from(0),
        &amount,
        Decimal::from(0),
        user_id,
    )
    .execute(&mut **tx)
    .await
    .expect("Cannot insert post_differences line 2");
}
