use chrono::{DateTime, NaiveDateTime, Utc};
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;
use event_sourcing::event::Event;
use event_sourcing::Error;

use std::collections::HashMap;

use crate::{query, ScyllaDbConnection};
use event_sourcing::event::store::EventStoreError::Concurrency;
use scylla::frame::response::result::Row;
use scylla::{IntoTypedRows, Session, SessionBuilder};

#[derive(Debug, Clone)]
pub struct ScyllaDbEventStore {
    pub connection: ScyllaDbConnection,
}

#[async_trait::async_trait]
impl<E: Event> EventStore<E> for ScyllaDbEventStore {
    async fn read(&self, aggregate_id: &String) -> Result<Vec<EventEnvelope<E>>, Error> {
        let session: Session = SessionBuilder::new()
            .known_node(&self.connection.host)
            .build()
            .await?;
        match session
            .query(query::READ_EVENTS, [aggregate_id].as_ref())
            .await?
            .rows
        {
            Some(rows) => Self::map_event_envelope(rows),
            None => Ok(vec![]),
        }
    }

    async fn read_from(
        &self,
        aggregate_id: &String,
        sequence: i64,
    ) -> Result<Vec<EventEnvelope<E>>, Error> {
        let session: Session = SessionBuilder::new()
            .known_node(&self.connection.host)
            .build()
            .await?;
        match session
            .query(query::READ_EVENTS_FROM, (aggregate_id, sequence))
            .await?
            .rows
        {
            Some(rows) => Self::map_event_envelope(rows),
            None => Ok(vec![]),
        }
    }

    async fn persist(&self, event_envelope: &EventEnvelope<E>) -> Result<(), Error> {
        let session: Session = SessionBuilder::new()
            .known_node(&self.connection.host)
            .build()
            .await?;
        match session
            .query(
                query::INSERT_EVENT,
                (
                    &event_envelope.aggregate_id,
                    &event_envelope.aggregate_type,
                    &serde_json::to_string(&event_envelope.event)?,
                    &event_envelope.event_type,
                    &event_envelope.event_time.timestamp_millis(),
                    &event_envelope.sequence,
                    &event_envelope.revision,
                    &event_envelope.metadata,
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
                    Option<String>,
                    Option<HashMap<String, String>>,
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

impl ScyllaDbEventStore {
    fn map_event_envelope<E: Event>(rows: Vec<Row>) -> Result<Vec<EventEnvelope<E>>, Error> {
        rows.into_typed::<(
            String,
            String,
            String,
            String,
            i64,
            i64,
            i64,
            Option<HashMap<String, String>>,
        )>()
        .map(|row| {
            let (agg_id, agg_type, event, event_type, event_time, sequence, revision, metadata) =
                row?;
            let ts_secs = event_time / 1000;
            let ts_ns = (event_time % 1000) * 1_000_000;
            let date_time = DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(ts_secs, ts_ns as u32)
                    .ok_or(Error::from("Failed to get timestamp"))?,
                Utc,
            );
            Ok(EventEnvelope::new(
                agg_id,
                agg_type,
                serde_json::from_str(&event)?,
                event_type,
                date_time,
                sequence,
                revision,
                metadata.unwrap_or(HashMap::new()),
            ))
        })
        .collect()
    }
}
