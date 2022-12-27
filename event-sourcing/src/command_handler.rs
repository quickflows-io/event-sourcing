use crate::command::Command;
use serde::{Deserialize, Serialize};

/// The aggregate_id and event_time is returned to the client to
/// ping a projection to check if the event has been processed
/// before doing a full query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub aggregate_id: String,
    pub sequence: i64,
}

#[async_trait::async_trait]
pub trait CommandHandler<C>
where
    C: Command,
{
    type Error: Send + Sync;

    async fn handle(&self, command: C) -> Result<CommandResponse, Self::Error>;
}
