CREATE TABLE app_installations (
    app_name        TEXT NOT NULL,
    installation_id BLOB NOT NULL,
    region_id       TEXT NOT NULL,
    first_seen_at   INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (app_name, installation_id)
);
