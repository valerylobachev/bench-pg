use crate::model::domain::{Period, User};
use sqlx::{Pool, Postgres};

pub async fn open_period(pool: &Pool<Postgres>, period: Period, user: User) {
    let user_id = user.id();

    let materials = sqlx::query!("select id from fin_materials")
        .fetch_all(pool)
        .await
        .expect("Error loading materials");

    for material in materials {
        let material_id = material.id;
        let r = sqlx::query!(
            "select count(*) from fin_material_periods \
                   where material_id = $1 and period = $2",
            &material_id,
            period.year_period(),
        )
            .fetch_one(pool)
            .await
            .expect("Error getting material period");
        
        if r.count.unwrap_or(0) == 0 {
            sqlx::query!(
                r#"insert into fin_material_periods
                      select material_id, $1, 
                             (select next_std_price from fin_materials where id = $2) as std_price,
                             mov_avg_price, actual_price, sell_price, stock, 
                             $3 as updated_by, now() as updated_at 
                      from fin_material_periods
                      where material_id = $2 and period = $4"#,
                period.year_period(),
                &material_id,
                &user_id,
                period.prev_period().year_period(),
            )
                .execute(pool)
                .await
                .expect("Error inserting material period");
        } else {
            sqlx::query!(
                r#"update fin_material_periods set
                      std_price = (select next_std_price from fin_materials where id = $1),
                      stock = (select stock from fin_material_periods where material_id = $1 and period = $4 ),
                      updated_by = $3, 
                      updated_at = now() 
                   where material_id = $1 and period = $2"#,
                &material_id,
                period.year_period(),
                &user_id,
                period.prev_period().year_period(),
            )
                .execute(pool)
                .await
                .expect("Error updating material period");
        }
    }
}
