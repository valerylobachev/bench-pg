use crate::executors::diesel::schema::*;
use crate::model::domain::{Account, Customer, Material, Period, User, Vendor};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::PgConnection;
use rust_decimal::Decimal;

pub async fn clear_tables(conn: &mut PooledConnection<ConnectionManager<PgConnection>>) {
    diesel::delete(fin_ledger_items::table)
        .execute(conn)
        .expect("Error deleting ");
    diesel::delete(fin_accounts::table)
        .execute(conn)
        .expect("Error deleting ");
    diesel::delete(fin_material_periods::table)
        .execute(conn)
        .expect("Error deleting ");
    diesel::delete(fin_materials::table)
        .execute(conn)
        .expect("Error deleting ");
    diesel::delete(fin_business_partners::table)
        .execute(conn)
        .expect("Error deleting ");
}

#[derive(Insertable)]
#[diesel(table_name = fin_business_partners)]
pub struct BusinessPartner<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub updated_by: &'a str,
    pub updated_at: &'a DateTime<Utc>,
}

pub async fn load_business_partners(
    customers: u32,
    vendors: u32,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let user = User(0);
    for i in 0..customers {
        let customer = Customer(i);
        let bp = BusinessPartner {
            id: &customer.id(),
            name: &customer.name(),
            updated_by: &user.id(),
            updated_at: &Utc::now(),
        };
        diesel::insert_into(fin_business_partners::dsl::fin_business_partners)
            .values(&bp)
            .execute(conn)
            .expect(format!("Failed to insert customer {}", customer.id()).as_str());
    }
    for i in 0..vendors {
        let vendor = Vendor(i);
        let bp = BusinessPartner {
            id: &vendor.id(),
            name: &vendor.name(),
            updated_by: &user.id(),
            updated_at: &Utc::now(),
        };
        diesel::insert_into(fin_business_partners::dsl::fin_business_partners)
            .values(&bp)
            .execute(conn)
            .expect(format!("Failed to insert vendor {}", vendor.id()).as_str());
    }
}

pub async fn load_materials(
    materials: u32,
    start_year: u32,
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
) {
    let user = User(0);
    let user_id = &user.id();
    let period_yp = Period::new(start_year as i32, 1)
        .prev_period()
        .year_period();

    for i in 0..materials {
        let material = Material(i);
        let material_id = &material.id();
        let std_price = Decimal::from(rand::random_range(100..200));
        diesel::insert_into(fin_materials::dsl::fin_materials)
            .values((
                fin_materials::dsl::id.eq(material_id),
                fin_materials::dsl::name.eq(material.name()),
                fin_materials::dsl::next_std_price.eq(&std_price),
                fin_materials::dsl::updated_by.eq(user_id),
                fin_materials::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(conn)
            .expect(format!("Failed to insert material {}", material.id()).as_str());

        let sell_price = std_price * Decimal::from(2);
        diesel::insert_into(fin_material_periods::dsl::fin_material_periods)
            .values((
                fin_material_periods::dsl::material_id.eq(material_id),
                fin_material_periods::dsl::period.eq(period_yp),
                fin_material_periods::dsl::std_price.eq(std_price),
                fin_material_periods::dsl::mov_avg_price.eq(std_price),
                fin_material_periods::dsl::actual_price.eq(std_price),
                fin_material_periods::dsl::sell_price.eq(sell_price),
                fin_material_periods::dsl::stock.eq(Decimal::from(0)),
                fin_material_periods::dsl::updated_by.eq(user_id),
                fin_material_periods::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(conn)
            .expect(format!("Failed to insert material {}", material.id()).as_str());
    }
}

pub async fn load_accounts(
    conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
    accounts: Vec<Account>,
) {
    let user = User(0);
    let user_id = &user.id();

    for account in accounts {
        diesel::insert_into(fin_accounts::dsl::fin_accounts)
            .values((
                fin_accounts::dsl::id.eq(&account.id),
                fin_accounts::dsl::name.eq(&account.name),
                fin_accounts::dsl::updated_by.eq(user_id),
                fin_accounts::dsl::updated_at.eq(&Utc::now()),
            ))
            .execute(conn)
            .expect(format!("Failed to insert account {} - {}", account.id, account.name).as_str());
    }
}
