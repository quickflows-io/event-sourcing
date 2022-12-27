pub(crate) fn create_keyspace(replication_factor: &i8) -> String {
    // language=cassandraql
    format!("CREATE KEYSPACE IF NOT EXISTS event_store WITH REPLICATION = {{'class' : 'SimpleStrategy', 'replication_factor' : {replication_factor}}}")
}
// language=cassandraql
pub(crate) const CREATE_EVENTS_TABLE: &str = "
CREATE TABLE IF NOT EXISTS event_store.events
(
    aggregate_id   VARCHAR,
    aggregate_type VARCHAR,
    event          TEXT,
    event_type     VARCHAR,
    event_time     TIMESTAMP,
    sequence       BIGINT,
    revision       BIGINT,
    metadata       MAP<VARCHAR, TEXT>,
    primary key (aggregate_id, sequence)
) WITH cdc = {'enabled': true}
";
// language=cassandraql
pub(crate) const INSERT_EVENT: &str = "
INSERT INTO event_store.events (aggregate_id, aggregate_type, event, event_type, event_time, sequence, revision, metadata)
VALUES (?, ?, ?, ?, ?, ?, ?, ?) IF NOT EXISTS
";
// language=cassandraql
pub(crate) const READ_EVENTS: &str = "
SELECT aggregate_id, aggregate_type, event, event_type, event_time, sequence, revision, metadata
FROM event_store.events
WHERE aggregate_id = ?
";
// language=cassandraql
pub(crate) const READ_EVENTS_FROM: &str = "
SELECT aggregate_id, aggregate_type, event, event_type, event_time, sequence, revision, metadata
FROM event_store.events
WHERE aggregate_id = ? AND sequence >= ?
";
// language=cassandraql
pub(crate) const CREATE_SNAPSHOT_TABLE: &str = "
CREATE TABLE IF NOT EXISTS event_store.snapshots
(
    aggregate_id   VARCHAR,
    aggregate_type VARCHAR,
    state          TEXT,
    state_time     TIMESTAMP,
    sequence       BIGINT,
    primary key (aggregate_id, sequence)
) WITH cdc = {'enabled': true}
";
// language=cassandraql
pub(crate) const INSERT_SNAPSHOT: &str = "
INSERT INTO event_store.snapshots (aggregate_id, aggregate_type, state, state_time, sequence)
VALUES (?, ?, ?, ?, ?) IF NOT EXISTS
";
// language=cassandraql
pub(crate) const READ_SNAPSHOT: &str = "
SELECT aggregate_id, aggregate_type, state, state_time, sequence
FROM event_store.snapshots
WHERE aggregate_id = ?
ORDER BY sequence DESC
LIMIT 1
";
