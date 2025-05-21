use crate::model::domain::{Cost, Document, User};
use super::post_document::post_document;
use tokio_postgres::Client;

pub async fn account_cost(client: &mut Client, op: &Cost, user: User) {
    let mut tx = client
        .transaction()
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
