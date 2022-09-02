use crate::event::envelope::EventEnvelope;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::error::Error;

#[async_trait::async_trait]
pub trait EventStore: Sized + Send + Sync + Clone {
    async fn read<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        aggregate_id: &String,
    ) -> Result<Vec<EventEnvelope<Data>>, Box<dyn Error + Send + Sync>>;
    async fn persist<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        event_envelope: EventEnvelope<Data>,
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}
