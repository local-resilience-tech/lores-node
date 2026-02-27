-- Add migration script here

CREATE TABLE regions (
    id VARCHAR(36) PRIMARY KEY NOT NULL,
    creator_node_id VARCHAR(36) NOT NULL,
    slug TEXT NOT NULL,
    name TEXT NOT NULL,
    organisation_name TEXT NULL,
    organisation_url TEXT NULL,
    node_steward_conduct_url TEXT NULL,
    user_conduct_url TEXT NULL,
    user_privacy_url TEXT NULL
);

CREATE TABLE nodes (
    id VARCHAR(36) PRIMARY KEY NOT NULL
);

CREATE TABLE region_nodes (
    id INTEGER PRIMARY KEY NOT NULL,
    node_id VARCHAR(36) NOT NULL,
    region_id VARCHAR(36) NOT NULL,
    name VARCHAR(50) NOT NULL,
    public_ipv4 VARCHAR(15) DEFAULT NULL,
    domain_on_local_network TEXT DEFAULT NULL,
    domain_on_internet TEXT DEFAULT NULL,

    FOREIGN KEY (node_id) REFERENCES nodes(id),
    FOREIGN KEY (region_id) REFERENCES regions(id),
    UNIQUE(node_id, region_id)
);

CREATE TABLE node_statuses (
    operation_id VARCHAR(64) PRIMARY KEY NOT NULL,
    node_id VARCHAR(36) NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE current_node_statuses (
    region_node_id INTEGER PRIMARY KEY NOT NULL,
    text VARCHAR(255) NULL,
    state VARCHAR(50) NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (region_node_id) REFERENCES region_nodes(id)
);

CREATE TABLE apps (
  name TEXT NOT NULL UNIQUE,
  PRIMARY KEY (name)
);

CREATE TABLE app_installations (
  app_name TEXT NOT NULL,
  region_node_id INTEGER NOT NULL,
  version TEXT NOT NULL,
  FOREIGN KEY (app_name) REFERENCES apps(name),
  FOREIGN KEY (region_node_id) REFERENCES region_nodes(id),
  PRIMARY KEY (app_name, region_node_id)
);