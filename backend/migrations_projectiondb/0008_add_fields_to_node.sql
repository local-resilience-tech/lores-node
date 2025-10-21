-- Add migration script here

ALTER TABLE nodes
ADD COLUMN domain_on_local_network TEXT DEFAULT NULL;

ALTER TABLE nodes
ADD COLUMN domain_on_internet TEXT DEFAULT NULL;