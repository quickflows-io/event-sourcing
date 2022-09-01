#[async_trait::async_trait]
pub trait Aggregate: Sized + Send + Sync + Clone {
    type Event: Send + Sync + Clone;
    type Error: Send + Sync;
    async fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
}
