use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref KEY_VALUE_STORE: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

#[derive(Debug, Clone)]
pub struct InMemoryEventStore;

#[async_trait::async_trait]
impl EventStore for InMemoryEventStore {
    async fn read<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        aggregate_id: &String,
    ) -> Result<Vec<EventEnvelope<Data>>, Box<dyn Error + Send + Sync>> {
        KEY_VALUE_STORE
            .lock()
            .unwrap()
            .get(aggregate_id)
            .map(|ses| {
                ses.into_iter()
                    .map(|se| serde_json::from_str(se.as_str()))
                    .collect()
            })
            .unwrap_or(Ok(vec![]))
            .map_err(|e| e.into())
    }

    async fn persist<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        event_envelope: EventEnvelope<Data>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let key = event_envelope.aggregate_id.clone();

        let serialized_events: Result<Vec<String>, serde_json::Error> =
            [self.read(&key).await?, vec![event_envelope.clone()]]
                .concat()
                .into_iter()
                .map(|event| serde_json::to_string(&event))
                .collect();

        KEY_VALUE_STORE
            .lock()
            .unwrap()
            .insert(key, serialized_events?);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::str::FromStr;
    use uuid::Uuid;

    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Event {
        id: Uuid,
        name: String,
        value: i64,
    }

    #[tokio::test]
    async fn it_persists_and_reads_events() {
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
        let event_store = InMemoryEventStore;
        let _inserted_event_envelope_result = event_store.persist(event_envelope).await;
        let event_envelopes_result: Result<
            Vec<EventEnvelope<Event>>,
            Box<dyn Error + Send + Sync>,
        > = event_store.read(&String::from("aggregate_id")).await;
        assert_eq!(event_envelopes_result.expect("expected events").len(), 1)
    }
}
