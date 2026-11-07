CREATE TABLE registered_domain_providers (
    registration_id     TEXT PRIMARY KEY,
    domain_provider     TEXT NOT NULL,
    credential          TEXT NOT NULL,
    username            TEXT NOT NULL,
    registered_at       TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);
