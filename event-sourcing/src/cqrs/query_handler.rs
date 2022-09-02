#[async_trait::async_trait]
pub trait QueryHandler<Query, Data>
where
    Query: Send + Sync,
    Query: Send + Sync,
{
    async fn handle(&self, query: Query) -> Result<Data, Box<dyn std::error::Error + Send + Sync>>;
}
