table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    transactions (id) {
        id -> Int4,
        transaction_date -> Date,
        transaction_details -> Varchar,
        funds_out -> Numeric,
        funds_in -> Numeric,
    }
}

table! {
    transations_category (id) {
        id -> Int4,
        categories_id -> Int4,
        transactions_id -> Int4,
    }
}

joinable!(transations_category -> categories (categories_id));
joinable!(transations_category -> transactions (transactions_id));

allow_tables_to_appear_in_same_query!(
    categories,
    transactions,
    transations_category,
);
