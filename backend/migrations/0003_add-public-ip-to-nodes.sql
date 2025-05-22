-- Add migration script here
ALTER TABLE nodes
ADD COLUMN public_ipv4 VARCHAR(15) DEFAULT NULL;