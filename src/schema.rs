table! {
    transactions (id) {
        id -> Int4,
        transaction_date -> Varchar,
        transaction_details -> Varchar,
        funds_out -> Numeric,
        funds_in -> Numeric,
    }
}
