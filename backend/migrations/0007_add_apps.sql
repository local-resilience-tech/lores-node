-- Add migration script here
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