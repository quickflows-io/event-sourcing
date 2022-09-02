use chrono::{DateTime, Utc};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

/// Event is a domain envelope describing a change that has happened to an aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct EventEnvelope<Event>
where
    Event: Send + Sync + Clone + Serialize,
{
    // Unique identifier of the envelope.
    #[new(value = "Uuid::new_v4()")]
    pub id: Uuid,
    // ID of the aggregate that the envelope belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the envelope can be applied to.
    pub aggregate_type: String,
    // Event attached to the envelope.
    pub data: Event,
    // Type of the envelope.
    pub event_type: String,
    // Version of the aggregate after the envelope has been applied.
    pub version: i64,
    // Timestamp of when the envelope was created.
    #[new(value = "Utc::now()")]
    pub timestamp: DateTime<Utc>,
}

impl<Event> EventEnvelope<Event>
where
    Event: Send + Sync + Clone + Serialize,
{
    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        serde_json::to_string(self).map_err(|error| error.into())
    }
}

pub fn deserialize<Event: Send + Sync + Clone + Serialize + DeserializeOwned>(
    event_envelope: String,
) -> Result<EventEnvelope<Event>, Box<dyn std::error::Error>> {
    serde_json::from_str(event_envelope.as_str()).map_err(|error| error.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Event {
        id: Uuid,
        name: String,
        value: i64,
    }

    #[test]
    fn it_serializes_and_deserializes() {
        let data = Event {
            id: Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid"),
            name: String::from("name"),
            value: 1,
        };
        let event_envelope: EventEnvelope<Event> = EventEnvelope::new(
            String::from("aggregate_id"),
            String::from("aggregate_type"),
            data,
            String::from("event_type"),
            0,
        );
        let serialized_event_envelope = event_envelope
            .serialize()
            .expect("expected serialized struct");
        let event_envelope: EventEnvelope<Event> =
            deserialize(String::from(serialized_event_envelope))
                .expect("expected deserialized struct");
        assert_eq!(event_envelope.aggregate_id, String::from("aggregate_id"));
        assert_eq!(
            event_envelope.aggregate_type,
            String::from("aggregate_type")
        );
        assert_eq!(event_envelope.event_type, String::from("event_type"));
        assert_eq!(event_envelope.version, 0);
        assert_eq!(
            event_envelope.data.id,
            Uuid::from_str("2e996ba1-03a6-47af-8fd1-2039c6708dd4").expect("expected uuid")
        );
        assert_eq!(event_envelope.data.name, "name");
        assert_eq!(event_envelope.data.value, 1);
    }
}
