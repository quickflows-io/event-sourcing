use crate::Error;

use crate::event::envelope::EventEnvelope;
use crate::event::Event;
use custom_error::custom_error;

custom_error! {pub EventStoreError
    Concurrency = "concurrency error",
}

#[async_trait::async_trait]
pub trait EventStore<E>: Sized + Send + Sync + Clone
where
    E: Event,
{
    // Fetch all events for the aggregate.
    async fn read(&self, aggregate_id: &String) -> Result<Vec<EventEnvelope<E>>, Error>;
    // Fetch all events on and after the specified event_time for the aggregate.
    async fn read_from(
        &self,
        aggregate_id: &String,
        sequence: i64,
    ) -> Result<Vec<EventEnvelope<E>>, Error>;
    // Persist the event for the aggregate.
    async fn persist(&self, event_envelope: &EventEnvelope<E>) -> Result<(), Error>;
}
