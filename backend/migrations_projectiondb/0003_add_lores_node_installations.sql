CREATE TABLE lores_node_installations(
    node_id TEXT PRIMARY KEY NOT NULL,
    lores_version TEXT NOT NULL,
    posted_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
