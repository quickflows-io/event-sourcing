use chrono::{NaiveDateTime, Utc};
use serde_json::Value;

use crate::schema::event;
use event_sourcing::event::envelope::EventEnvelope;
use serde::de::DeserializeOwned;
use serde::Serialize;

use std::error::Error;

#[derive(Debug, Clone, PartialEq, Insertable, Queryable)]
#[diesel(table_name = event)]
pub struct EventEntity {
    pub id: uuid::Uuid,
    pub aggregate_id: String,
    pub aggregate_type: String,
    pub data: Value,
    pub event_type: String,
    pub version: i64,
    pub timestamp: NaiveDateTime,
}

impl EventEntity {
    pub(crate) fn envelope<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
    ) -> Result<EventEnvelope<Data>, Box<dyn Error + Send + Sync>> {
        let entity = self.clone();
        let deserialized_data: Data = serde_json::from_value(entity.data)?;
        Ok(EventEnvelope {
            id: entity.id,
            aggregate_id: entity.aggregate_id,
            aggregate_type: entity.aggregate_type,
            data: deserialized_data,
            event_type: entity.event_type,
            version: entity.version,
            timestamp: chrono::DateTime::from_utc(entity.timestamp, Utc),
        })
    }

    pub(crate) fn entity<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        event_envelope: EventEnvelope<Data>,
    ) -> Result<EventEntity, Box<dyn Error + Send + Sync>> {
        let deserialized_data: Value = serde_json::to_value(event_envelope.data)?;
        Ok(EventEntity {
            id: event_envelope.id,
            aggregate_id: event_envelope.aggregate_id,
            aggregate_type: event_envelope.aggregate_type,
            data: deserialized_data,
            event_type: event_envelope.event_type,
            version: event_envelope.version,
            timestamp: event_envelope.timestamp.naive_local(),
        })
    }
}
