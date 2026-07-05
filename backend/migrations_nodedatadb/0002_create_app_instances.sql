CREATE TABLE app_instances (
    app_name        TEXT NOT NULL,
    instance_id     TEXT NOT NULL,
    first_seen_at   INTEGER NOT NULL DEFAULT (unixepoch()),
    PRIMARY KEY (app_name, instance_id)
);
