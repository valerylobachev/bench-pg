use rust_decimal::Decimal;
use tokio_postgres::Client;
use super::post_document::post_document;
use crate::model::domain::{Document, Period, Sale, User};

pub async fn sell_material(client: &mut Client, op: &Sale, user: User) {
    let mut tx = client
        .transaction()
        .await
        .expect("Cannot start transaction for sell material");

    let period = Period::from(op.posting_date);
    let material_id = op.material.id();
    let user_id = user.id();

    let mp = tx
        .query_one(
            r#"select std_price, sell_price, stock
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
    let sell_price = mp.get::<usize, Decimal>(1);
    let stock = mp.get::<usize, Decimal>(2);

    if stock < op.quantity {
        println!(
            "Cannot sell material {} in period {}, stock ({}) lower than required quantity ({})",
            &material_id, period, &stock, &op.quantity,
        );
        tx.rollback().await.expect("Failed to rollback transaction");
        return;
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
    post_document(&mut tx, cogs_document)
        .await
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
    post_document(&mut tx, sale_document)
        .await
        .expect("Cannot post cogs document");

    tx.execute(
        r#"update fin_material_periods set
                    stock = stock - $1,  
                    updated_by = $2,   
                    updated_at = now()   
                 where material_id = $3 and period >= $4"#,
        &[&op.quantity, &user_id, &material_id, &period.year_period()],
    )
    .await
    .expect("Cannot update material_periods for sale");

    tx.commit()
        .await
        .expect("Cannot commit transaction for sale material");
}
