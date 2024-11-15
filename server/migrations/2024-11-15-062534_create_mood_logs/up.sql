-- Your SQL goes here
-- create_mood_logs/up.sql
CREATE TABLE mood_logs (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    mood VARCHAR NOT NULL, -- happy, sad, stressed, etc.
    log_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
