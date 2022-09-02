#[async_trait::async_trait]
pub trait CommandHandler<Command>
where
    Command: Send + Sync,
{
    async fn handle(
        &self,
        command: Command,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
