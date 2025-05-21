use crate::executors::diesel::schema::{fin_material_periods, fin_materials};
use crate::model::domain::{Period, User};
use chrono::Utc;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::{PgConnection, RunQueryDsl};
use rust_decimal::Decimal;

pub async fn open_period(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    period: Period,
    user: User,
) {
    let user_id = user.id();

    let materials = fin_materials::dsl::fin_materials
        .select((fin_materials::dsl::id, fin_materials::dsl::next_std_price))
        .load::<(String, Decimal)>(conn)
        .expect("Error loading materials");

    for (material_id, next_std_price) in materials {
        let r = fin_material_periods::dsl::fin_material_periods
            .filter(fin_material_periods::dsl::material_id.eq(&material_id))
            .filter(fin_material_periods::dsl::period.eq(period.year_period()))
            .count()
            .get_result(conn);

        if r.unwrap_or(0) == 0 {
            let (mov_avg_price, actual_price, sell_price, stock) =
                fin_material_periods::dsl::fin_material_periods
                    .find((&material_id, period.prev_period().year_period()))
                    .select((
                        fin_material_periods::dsl::mov_avg_price,
                        fin_material_periods::dsl::actual_price,
                        fin_material_periods::dsl::sell_price,
                        fin_material_periods::dsl::stock,
                    ))
                    .first::<(Decimal, Decimal, Decimal, Decimal)>(conn)
                    .expect("Error loading materials");

            diesel::insert_into(fin_material_periods::dsl::fin_material_periods)
                .values((
                    fin_material_periods::dsl::material_id.eq(&material_id),
                    fin_material_periods::dsl::period.eq(period.year_period()),
                    fin_material_periods::dsl::std_price.eq(next_std_price),
                    fin_material_periods::dsl::mov_avg_price.eq(mov_avg_price),
                    fin_material_periods::dsl::actual_price.eq(actual_price),
                    fin_material_periods::dsl::sell_price.eq(sell_price),
                    fin_material_periods::dsl::stock.eq(stock),
                    fin_material_periods::dsl::updated_by.eq(&user_id),
                    fin_material_periods::dsl::updated_at.eq(&Utc::now()),
                ))
                .execute(conn)
                .expect("Error inserting material period");
        } else {
            let stock = fin_material_periods::dsl::fin_material_periods
                .select(fin_material_periods::dsl::stock)
                .find((&material_id, period.prev_period().year_period()))
                .first::<Decimal>(conn)
                .expect("Error loading materials");
            diesel::update(
                fin_material_periods::dsl::fin_material_periods
                    .find((&material_id, period.prev_period().year_period())),
            )
            .set((
                fin_material_periods::dsl::std_price.eq(next_std_price),
                fin_material_periods::dsl::stock.eq(stock),
                fin_material_periods::dsl::updated_by.eq(&user_id),
                fin_material_periods::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(conn)
            .expect("Error updating material period");
        }
    }
}
