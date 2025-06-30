-- Add migration script here
CREATE TABLE IF NOT EXISTS current_node_statuses (
    node_id VARCHAR(36) PRIMARY KEY NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);