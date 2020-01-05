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