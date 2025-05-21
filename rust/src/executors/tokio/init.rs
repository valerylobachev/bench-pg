use crate::model::domain::{Account, Customer, Material, Period, User, Vendor};
use sqlx::types::Decimal;
use tokio_postgres::Client;

pub async fn clear_tables(client: &mut Client) {
    let clear_stmts = vec![
        "TRUNCATE TABLE fin_ledger_items cascade;",
        "TRUNCATE TABLE fin_accounts cascade;",
        "TRUNCATE TABLE fin_material_periods cascade;",
        "TRUNCATE TABLE fin_materials cascade;",
        "TRUNCATE TABLE fin_business_partners cascade;",
    ];
    for stmt in clear_stmts {
        // let stmt = client.prepare_cached(stmt).await.unwrap();
        client
            .execute(stmt, &[])
            .await
            .expect(format!("Failed to execute {}", stmt).as_str());
    }
}

pub async fn load_business_partners(customers: u32, vendors: u32, client: &mut Client) {
    let user = User(0);

    for i in 0..customers {
        let customer = Customer(i);
        client
            .execute(
                r#"INSERT INTO fin_business_partners (id, name, updated_by, updated_at) 
                   VALUES ($1, $2, $3, now())"#,
                &[&customer.id(), &customer.name(), &user.id()],
            )
            .await
            .expect(format!("Failed to insert customer {}", customer.id()).as_str());
    }

    for i in 0..vendors {
        let vendor = Vendor(i);
        client
            .execute(
                r#"INSERT INTO fin_business_partners (id, name, updated_by, updated_at) 
                   VALUES ($1, $2, $3, now())"#,
                &[&vendor.id(), &vendor.name(), &user.id()],
            )
            .await
            .expect(format!("Failed to insert vendor {}", vendor.id()).as_str());
    }
}

pub async fn load_materials(materials: u32, start_year: u32, client: &mut Client) {
    let user = User(0);
    let user_id = &user.id();
    let period_yp = Period::new(start_year as i32, 1)
        .prev_period()
        .year_period();

    for i in 0..materials {
        let material = Material(i);
        let material_id = &material.id();
        let std_price = Decimal::from(rand::random_range(100..200));
        client
            .execute(
                r#"insert into fin_materials (id, name, next_std_price, updated_by, updated_at)
                 values ($1, $2, $3, $4, now())"#,
                &[&material_id, &material.name(), &std_price, &user_id],
            )
            .await
            .expect(format!("Failed to insert material {}", material.id()).as_str());

        let sell_price = std_price * Decimal::from(2);
        client
            .execute(
                r#"insert into fin_material_periods
                 (material_id, period, std_price, mov_avg_price,
                   actual_price, sell_price, stock, updated_by, updated_at)
                 values ($1, $2, $3, $4, $5, $6, $7, $8, now());"#,
                &[
                    &material_id,
                    &period_yp,
                    &std_price,
                    &std_price,
                    &std_price,
                    &sell_price,
                    &Decimal::from(0),
                    &user.id(),
                ],
            )
            .await
            .expect(format!("Failed to insert material {}", &material_id).as_str());
    }
}

pub async fn load_accounts(client: &mut Client, accounts: Vec<Account>) {
    let user = User(0);

    for account in accounts {
        client
            .execute(
                r#"INSERT INTO fin_accounts (id, name, updated_by, updated_at) 
                         VALUES ($1, $2, $3, now())"#,
                &[&account.id, &account.name, &user.id()],
            )
            .await
            .expect(format!("Failed to insert account {} - {}", account.id, account.name).as_str());
    }
}
