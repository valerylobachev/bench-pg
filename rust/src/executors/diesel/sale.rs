use super::post_document::post_document;
use crate::executors::diesel::schema::fin_material_periods;
use crate::model::domain::{Document, Period, Sale, User};
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;
use diesel::{Connection, PgConnection};
use rust_decimal::Decimal;
pub async fn sell_material(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    op: &Sale,
    user: User,
) {
    conn.transaction(|tx| {
        let period = Period::from(op.posting_date);
        let material_id = op.material.id();
        let user_id = user.id();

        let (std_price, sell_price, stock) = fin_material_periods::dsl::fin_material_periods
            .select((
                fin_material_periods::dsl::std_price,
                fin_material_periods::dsl::sell_price,
                fin_material_periods::dsl::stock,
            ))
            .filter(fin_material_periods::dsl::material_id.eq(&material_id))
            .filter(fin_material_periods::dsl::period.eq(period.year_period()))
            .for_update()
            .first::<(Decimal, Decimal, Decimal)>(tx)
            .expect(
                format!(
                    "Cannot lock material {} for sale in period {}",
                    &material_id, period,
                )
                .as_str(),
            );

        if stock < op.quantity {
            println!(
                "Cannot sell material {} in period {}, stock ({}) lower than required quantity ({})",
                &material_id, period, &stock, &op.quantity,
            );
            return Err(Error::RollbackTransaction)
        }

        let cogs_document = Document::new_cogs_document(
            op.posting_date,
            op.cogs_doc_no.clone(),
            op.customer,
            op.material,
            std_price,
            op.quantity,
            user,
        );
        post_document(tx, cogs_document)
            .expect("Cannot post cogs document");

        let sale_document = Document::new_sale_document(
            op.posting_date,
            op.sale_doc_no.clone(),
            op.customer,
            op.material,
            sell_price,
            op.quantity,
            user,
        );
        post_document(tx, sale_document)
            .expect("Cannot post cogs document");


        diesel::update(
            fin_material_periods::dsl::fin_material_periods
                .find((&material_id, period.year_period())),
        )
            .set((
                fin_material_periods::dsl::stock.eq(fin_material_periods::dsl::stock - op.quantity),
                fin_material_periods::dsl::updated_by.eq(&user_id),
                fin_material_periods::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(tx)
            .expect("Cannot update material_periods for sale");

        Ok::<(), Error>(())
    })
    .expect("Cannot commit transaction for sale material");
}
