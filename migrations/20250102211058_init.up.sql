-- Add up migration script here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) UNIQUE,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    locked BOOLEAN DEFAULT FALSE,
    admin BOOLEAN DEFAULT FALSE
);

INSERT INTO users (username, email, password, admin) VALUES ('admin', 'admin@admin.com', '$argon2id$v=19$m=19,t=2,p=1$UXZBNjVlQW5YTTdDZDRpdA$S9lPULwfN3/rSDgqITDzrQ', TRUE);