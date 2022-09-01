use crate::event::store::{EventStore, EventStoreError};
use crate::event::stream::EventStream;
use crate::event::Event;

#[derive(Debug, Clone)]
pub struct PostgresConfiguration;

#[derive(Debug, Clone)]
pub struct PostgresEventStore<EventStream>
where
    EventStream: EventStream,
{
    event_stream: EventStream,
    configuration: PostgresConfiguration,
}

#[async_trait::async_trait]
impl<EventStream> EventStore for PostgresEventStore<EventStream>
where
    EventStream: EventStream,
{
    async fn read<Data: Send + Sync + Clone>(
        &self,
        aggregate_id: String,
    ) -> Result<Vec<Event<Data>>, EventStoreError> {
        todo!()
    }

    async fn persist<Data: Send + Sync + Clone>(
        &self,
        event: Event<Data>,
    ) -> Result<(), EventStoreError> {
        todo!()
    }
}
