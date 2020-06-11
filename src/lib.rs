#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{Transaction,SavedTransaction};

pub mod models;
pub mod schema;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("database url must be set");
    PgConnection::establish(&database_url).expect(&format!("problem connecting to database {}", database_url))
}

pub fn create_transaction(conn: &PgConnection, transaction: Transaction) -> SavedTransaction {
    use schema::transactions;

    diesel::insert_into(transactions::table).values(&transaction).get_result(conn).expect("Error saving transaction")
}
