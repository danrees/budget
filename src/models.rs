use serde::Deserialize;
use super::schema::transactions;
use bigdecimal::BigDecimal;

#[derive(Debug,Deserialize,Insertable)]
pub struct Transaction {
    #[serde(rename = "Date")]
    transaction_date: String,
    #[serde(rename = "Transaction Details")]
    transaction_details: String,
    #[serde(rename = "Funds Out")]
    funds_out: Option<BigDecimal>,
    #[serde(rename = "Funds In")]
    funds_in: Option<BigDecimal>,
}

#[derive(Debug,Queryable)]
pub struct SavedTransaction {
    id: i32,
    transaction_date: String,
    transaction_details: String,
    funds_out: BigDecimal,
    funds_in: BigDecimal,
}

#[derive(Debug,Queryable)]
pub struct Category {
    id: i32,
    name: String,
}

#[derive(Debug,Insertable)]
pub struct NewCategory {
    id: i32,
    name: String,
}

#[derive(Debug,Queryable)]
#[belongs_to(Transaction)]
#[belongs_to(Category)]
pub struct TransactionCategory {
    id: i32,
    transaction_id: i32,
    category_id: i32
}

#[derive(Debug,Insertable)]
#[belongs_to(Transaction)]
#[belongs_to(Category)]
pub struct NewTransactionCategory {
    transaction_id: i32,
    category_id: i32
}