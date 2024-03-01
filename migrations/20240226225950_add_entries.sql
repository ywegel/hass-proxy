-- Add migration script here

CREATE TABLE entries (
    id SERIAL PRIMARY KEY,
    entity_id VARCHAR(255) NOT NULL,
    state VARCHAR(255) NOT NULL,
    area VARCHAR(255) NOT NULL,
    timestamp TIMESTAMPTZ NOT NULL
);

-- TODO constraints for state and area