-- Your SQL goes here
CREATE TABLE transations_category (
    id SERIAL PRIMARY KEY,
    categories_id SERIAL references categories(id),
    transactions_id SERIAL references transactions(id)
)