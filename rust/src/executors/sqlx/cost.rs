use crate::model::domain::{Cost, Document, User};
use super::post_document::post_document;
use sqlx::{Pool, Postgres};

pub async fn account_cost(pool: &Pool<Postgres>, op: &Cost, user: User) {
    let mut tx = pool
        .begin()
        .await
        .expect("Cannot start transaction for material cost");

    let document = Document::new_cost_document(
        op.posting_date,
        op.doc_no.clone(),
        op.material,
        op.vendor,
        op.amount,
        user
    );
    
    post_document(&mut tx, document)
        .await
        .expect("Cannot post cost document");
    

    tx.commit()
        .await
        .expect("Cannot commit transaction for material cost");
}
