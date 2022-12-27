#[async_trait::async_trait]
pub trait QueryHandler<Query, Response>
where
    Query: Send + Sync,
{
    type Error: Send + Sync;

    async fn handle(&self, query: Query) -> Result<Response, Self::Error>;
}
