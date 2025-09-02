use super::post_document::post_document;
use crate::model::domain::{Document, Period, Purchase, User};
use rust_decimal::Decimal;
use tokio_postgres::Client;

pub async fn purchase_material(client: &mut Client, op: &Purchase, user: User) {
    let tx = client
        .transaction()
        .await
        .expect("Cannot start transaction for purchase material");

    let period = Period::from(op.posting_date);
    let material_id = op.material.id();
    let user_id = user.id();

    let mp = tx
        .query_one(
            r#"select std_price, mov_avg_price, stock
                 from fin_material_periods
                 where material_id=$1 and period=$2 for update"#,
            &[&material_id, &period.year_period()],
        )
        .await
        .expect(
            format!(
                "Cannot lock material {} for purchase in period {}",
                &material_id, period,
            )
            .as_str(),
        );
    let std_price = mp.get::<usize, Decimal>(0);
    let mov_avg_price = mp.get::<usize, Decimal>(1);
    let stock = mp.get::<usize, Decimal>(2);

    let document = Document::new_purchase_document(
        op.posting_date,
        op.doc_no.clone(),
        op.material,
        op.vendor,
        op.price,
        std_price,
        op.quantity,
        user,
    );

    post_document(&tx, document)
        .await
        .expect("Cannot post purchase document");

    let amount = op.price * op.quantity;
    let new_mov_avg_price = ((mov_avg_price * stock + amount) / (stock + op.quantity)).round_dp(2);

    tx.execute(
        r#"update fin_material_periods set 
                    mov_avg_price = $1, 
                    stock = stock + $2,  
                    updated_by = $3,   
                    updated_at = now()   
                 where material_id = $4 and period >= $5"#,
        &[
            &new_mov_avg_price,
            &op.quantity,
            &user_id,
            &material_id,
            &period.year_period(),
        ],
    )
    .await
    .expect("Cannot update material_periods for purchase");

    tx.commit()
        .await
        .expect("Cannot commit transaction for purchase material");
}
