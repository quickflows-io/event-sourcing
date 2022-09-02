-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE EXTENSION IF NOT EXISTS hstore;

CREATE TABLE IF NOT EXISTS event
(
    id             uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    aggregate_id   VARCHAR   NOT NULL,
    aggregate_type VARCHAR   NOT NULL,
    "data"         jsonb     NOT NULL,
    event_type     VARCHAR   NOT NULL,
    "version"      bigint    not null,
    "timestamp"    timestamp not null,
    UNIQUE (aggregate_id, version)
)