-- Your SQL goes here
-- create_transactions/up.sql
CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    amount DECIMAL NOT NULL,
    category VARCHAR NOT NULL,
    transaction_type VARCHAR NOT NULL, -- income or expense
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
