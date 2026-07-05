CREATE TABLE local_apps (
    name                TEXT PRIMARY KEY NOT NULL,
    version             TEXT NOT NULL,
    internet_url        TEXT,
    local_network_url   TEXT,
    instance_id         TEXT,
    created_at          INTEGER NOT NULL DEFAULT (unixepoch())
);
