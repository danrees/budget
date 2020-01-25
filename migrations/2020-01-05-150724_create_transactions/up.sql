-- Your SQL goes here
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    transaction_date DATE NOT NULL,
    transaction_details VARCHAR NOT NULL,
    funds_out NUMERIC DEFAULT 0 NOT NULL,
    funds_in NUMERIC DEFAULT 0 NOT NULL
);

