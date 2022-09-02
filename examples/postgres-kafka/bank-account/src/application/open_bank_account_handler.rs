use event_sourcing::aggregate::Aggregate;
use std::error::Error;
use uuid::Uuid;

use crate::domain::bank_account_aggregate::BankAccountAggregate;
use crate::event::BankAccountEvent;
use event_sourcing::cqrs::command_handler::CommandHandler;
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;

use crate::event::BankAccountEvent::OpenBankAccount;

#[derive(Debug, Clone)]
pub(crate) struct OpenBankAccountCommand {
    pub(crate) id: Uuid,
}

#[derive(Debug, Clone)]
pub(crate) struct OpenBankAccountCommandHandler<ES>
where
    ES: EventStore,
{
    pub(crate) event_store: ES,
}

#[async_trait::async_trait]
impl<ES> CommandHandler<OpenBankAccountCommand> for OpenBankAccountCommandHandler<ES>
where
    ES: EventStore,
{
    async fn handle(
        &self,
        command: OpenBankAccountCommand,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        let event = OpenBankAccount { id: command.id };
        match BankAccountAggregate::apply(None, event.clone()) {
            Ok(aggregate) => {
                let event_envelope: EventEnvelope<BankAccountEvent> = EventEnvelope::new(
                    aggregate.aggregate_id().to_string(),
                    aggregate.aggregate_type(),
                    event,
                    event.to_string(),
                    0,
                );
                self.event_store.persist(event_envelope).await
            }
            Err(error) => Err(error.into()),
        }
    }
}
