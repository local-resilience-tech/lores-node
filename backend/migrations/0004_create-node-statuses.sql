-- Add migration script here
CREATE TABLE IF NOT EXISTS node_statuses (
    operation_id VARCHAR(64) PRIMARY KEY NOT NULL,
    node_id VARCHAR(36) NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);