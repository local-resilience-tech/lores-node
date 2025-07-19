-- Add migration script here
CREATE TABLE nodes (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    public_ipv4 VARCHAR(15) DEFAULT NULL
);

CREATE TABLE network_configs (
    id INT PRIMARY KEY NOT NULL,
    network_name VARCHAR(255),
    bootstrap_node_id VARCHAR(64),
    bootstrap_node_ip4 VARCHAR(15)
);

CREATE TABLE node_configs (
    id INT PRIMARY KEY NOT NULL,
    public_key_hex VARCHAR(36),
    private_key_hex VARCHAR(64)
);

CREATE TABLE node_statuses (
    operation_id VARCHAR(64) PRIMARY KEY NOT NULL,
    node_id VARCHAR(36) NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE current_node_statuses (
    node_id VARCHAR(36) PRIMARY KEY NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

INSERT INTO node_configs (id) VALUES (0);
INSERT INTO network_configs (id) VALUES (0);