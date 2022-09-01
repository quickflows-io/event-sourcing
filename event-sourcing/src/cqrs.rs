#[async_trait::async_trait]
pub trait CommandHandler<Command>
where
    Command: Send + Sync,
{
    type Error: Sync + Send;
    async fn handle(&self, command: Command) -> Result<(), Self::Error>;
}

#[async_trait::async_trait]
pub trait QueryHandler<Query, Data>
where
    Query: Send + Sync,
    Query: Send + Sync,
{
    type Error: std::error::Error + Sync + Send;
    async fn handle(&self, query: Query) -> Result<Data, Self::Error>;
}
