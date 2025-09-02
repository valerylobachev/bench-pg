use diesel::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::result::Error;

use super::post_document::post_document;
use crate::model::domain::{Cost, Document, User};

pub async fn account_cost(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    op: &Cost,
    user: User,
) {
    conn.transaction(|tx| {
        let document = Document::new_cost_document(
            op.posting_date,
            op.doc_no.clone(),
            op.material,
            op.vendor,
            op.amount,
            user,
        );

        post_document(tx, document).expect("Cannot post cost document");

        Ok::<(), Error>(())
    })
    .expect("Cannot commit transaction for account material cost");
}
