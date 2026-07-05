CREATE TABLE local_apps (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    name                TEXT NOT NULL,
    version             TEXT NOT NULL,
    internet_url        TEXT,
    local_network_url   TEXT,
    instance_id         TEXT,
    bound_to_region_id  TEXT,
    created_at          INTEGER NOT NULL DEFAULT (unixepoch()),
    UNIQUE (name, instance_id)
);
