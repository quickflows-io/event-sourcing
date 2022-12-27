use chrono::{DateTime, NaiveDateTime, Utc};

use scylla::frame::response::result::Row;
use scylla::{IntoTypedRows, Session, SessionBuilder};

use event_sourcing::aggregate::Aggregate;
use event_sourcing::event::store::EventStoreError::Concurrency;
use event_sourcing::snapshot::envelope::SnapshotEnvelope;
use event_sourcing::snapshot::store::SnapshotStore;
use event_sourcing::Error;

use crate::{query, ScyllaDbConnection};

#[derive(Debug, Clone)]
pub struct ScyllaDbSnapshotStore {
    pub connection: ScyllaDbConnection,
}

#[async_trait::async_trait]
impl<A: Aggregate> SnapshotStore<A> for ScyllaDbSnapshotStore {
    async fn read(&self, aggregate_id: &String) -> Result<Option<SnapshotEnvelope<A>>, Error> {
        let session: Session = SessionBuilder::new()
            .known_node(&self.connection.host)
            .build()
            .await?;
        match session
            .query(query::READ_SNAPSHOT, [aggregate_id].as_ref())
            .await?
            .rows
        {
            Some(rows) => Self::map_snapshot_envelope(rows),
            None => Ok(None),
        }
    }

    async fn persist(&self, snapshot_envelope: &SnapshotEnvelope<A>) -> Result<(), Error> {
        let session: Session = SessionBuilder::new()
            .known_node(&self.connection.host)
            .build()
            .await?;
        match session
            .query(
                query::INSERT_SNAPSHOT,
                (
                    &snapshot_envelope.aggregate_id,
                    &snapshot_envelope.aggregate_type,
                    &serde_json::to_string(&snapshot_envelope.state)?,
                    &snapshot_envelope.state_time.timestamp_millis(),
                    &snapshot_envelope.sequence,
                ),
            )
            .await?
            .rows
        {
            Some(rows) => rows
                .into_typed::<(
                    bool,
                    Option<String>,
                    Option<i64>,
                    Option<String>,
                    Option<String>,
                    Option<i64>,
                )>()
                .try_for_each(|row| {
                    let applied: bool = row?.0;
                    match applied {
                        true => Ok(()),
                        false => Err(Concurrency.into()),
                    }
                }),
            None => Ok(()),
        }
    }
}

impl ScyllaDbSnapshotStore {
    fn map_snapshot_envelope<A: Aggregate>(
        rows: Vec<Row>,
    ) -> Result<Option<SnapshotEnvelope<A>>, Error> {
        let result: Result<Vec<SnapshotEnvelope<A>>, Error> = rows
            .into_iter()
            .map(|row| {
                let (agg_id, agg_type, state, state_time, sequence) =
                    row.into_typed::<(String, String, String, i64, i64)>()?;

                let ts_secs = state_time / 1000;
                let ts_ns = (state_time % 1000) * 1_000_000;
                let date_time = DateTime::<Utc>::from_utc(
                    NaiveDateTime::from_timestamp_opt(ts_secs, ts_ns as u32)
                        .ok_or(Error::from("Failed to get timestamp"))?,
                    Utc,
                );
                Ok(SnapshotEnvelope::new(
                    agg_id,
                    agg_type,
                    serde_json::from_str(&state)?,
                    date_time,
                    sequence,
                ))
            })
            .collect();
        result.map(|r| Some(r.first()?.clone()))
    }
}
