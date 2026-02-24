-- Add migration script here
CREATE TABLE regions (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    creator_node_id VARCHAR(36) NOT NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    organisation_name TEXT NOT NULL,
    url TEXT NOT NULL
);

CREATE TABLE nodes (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    name VARCHAR(50) NOT NULL,
    public_ipv4 VARCHAR(15) DEFAULT NULL,
    domain_on_local_network TEXT DEFAULT NULL,
    domain_on_internet TEXT DEFAULT NULL
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

CREATE TABLE apps (
  name TEXT NOT NULL UNIQUE,
  PRIMARY KEY (name)
);

CREATE TABLE app_installations (
  app_name TEXT NOT NULL,
  node_id TEXT NOT NULL,
  version TEXT NOT NULL,
  FOREIGN KEY (app_name) REFERENCES apps(name),
  FOREIGN KEY (node_id) REFERENCES nodes(id),
  PRIMARY KEY (app_name, node_id)
);