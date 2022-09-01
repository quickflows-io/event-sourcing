use crate::event::Event;

#[derive(Debug, Clone)]
enum EventStoreError {
    OptimisticLockingError,
    InfrastructureError(&'static str),
}

#[async_trait::async_trait]
pub trait EventStore {
    async fn read<Data: Send + Sync + Clone>(
        &self,
        aggregate_id: Self::ID,
    ) -> Result<Vec<Event<Data>>, EventStoreError>;
    async fn persist<Data: Send + Sync + Clone>(
        &self,
        event: Event<Data>,
    ) -> Result<(), EventStoreError>;
}
