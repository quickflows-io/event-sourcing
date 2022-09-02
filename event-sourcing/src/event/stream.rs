use std::error::Error;

#[async_trait::async_trait]
pub trait EventStream {
    async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}
