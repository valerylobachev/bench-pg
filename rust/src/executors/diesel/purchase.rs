use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, RunQueryDsl};
use diesel::result::Error;
use rust_decimal::Decimal;
use super::post_document::post_document;
use crate::model::domain::{Document, Period, Purchase, User};
use crate::executors::diesel::schema::fin_material_periods;

pub async fn purchase_material(
    pool: &mut PooledConnection<ConnectionManager<PgConnection>>,
    op: &Purchase,
    user: User,
) {
    pool.transaction(|tx| {
        let period = Period::from(op.posting_date);
        let material_id = op.material.id();
        let user_id = user.id();
        
        let (std_price, mov_avg_price, stock) = fin_material_periods::dsl::fin_material_periods
            .select((
                fin_material_periods::dsl::std_price,
                fin_material_periods::dsl::mov_avg_price,
                fin_material_periods::dsl::stock,
            ))
            .filter(fin_material_periods::dsl::material_id.eq(&material_id))
            .filter(fin_material_periods::dsl::period.eq(period.year_period()))
            .for_update()
            .first::<(Decimal, Decimal, Decimal)>(tx)
            .expect(
                format!(
                    "Cannot lock material {} for purchase in period {}",
                    &material_id, period,
                )
                    .as_str(),
            );

        let document = Document::new_purchase_document(
            op.posting_date,
            op.doc_no.clone(),
            op.material,
            op.vendor,
            op.price,
            std_price.into(),
            op.quantity,
            user,
        );

        post_document(tx, document)
            .expect("Cannot post purchase document");

        let amount = op.price * op.quantity;
        let new_mov_avg_price =
            ((mov_avg_price * stock + amount) / (stock + op.quantity)).round_dp(2);

        diesel::update(
            fin_material_periods::dsl::fin_material_periods
                .find((&material_id, period.year_period())),
        )
            .set((
                fin_material_periods::dsl::mov_avg_price.eq(new_mov_avg_price),
                fin_material_periods::dsl::stock.eq(fin_material_periods::dsl::stock + op.quantity), 
                fin_material_periods::dsl::updated_by.eq(&user_id),
                fin_material_periods::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(tx)
            .expect("Error updating material period");

        Ok::<(), Error>(())
    }).expect("Cannot commit transaction for purchase material");

}
