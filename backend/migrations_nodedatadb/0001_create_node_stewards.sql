CREATE TABLE node_stewards (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    hashed_password TEXT,
    password_reset_token TEXT,
    active BOOLEAN DEFAULT TRUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);