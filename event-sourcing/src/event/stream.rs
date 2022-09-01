#[async_trait::async_trait]
pub trait EventStream: Sized + Send + Sync + Clone {
    type Event: Send + Sync + Clone;
    type Error: Send + Sync;
    async fn send(&self, event: Self::Event) -> Result<(), Self::Error>;
}
