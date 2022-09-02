-- Your SQL goes here
CREATE TABLE IF NOT EXISTS snapshot
(
    id             uuid DEFAULT uuid_generate_v4() PRIMARY KEY,
    aggregate_id   VARCHAR   NOT NULL,
    aggregate_type VARCHAR   NOT NULL,
    state          jsonb     NOT NULL,
    "version"      bigint    not null,
    "timestamp"    timestamp not null,
    metadata       jsonb,
    UNIQUE (aggregate_id, version)
)