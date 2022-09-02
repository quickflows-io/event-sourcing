use std::error::Error;

use uuid::Uuid;

use event_sourcing::aggregate::Aggregate;
use event_sourcing::cqrs::command_handler::CommandHandler;
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;

use crate::domain::bank_account_aggregate::BankAccountAggregate;
use crate::event::BankAccountEvent;
use crate::event::BankAccountEvent::Deposit;

#[derive(Debug, Clone)]
pub(crate) struct DepositCommand {
    pub(crate) id: Uuid,
    pub(crate) amount: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct DepositCommandHandler<ES>
where
    ES: EventStore,
{
    pub(crate) event_store: ES,
}

#[async_trait::async_trait]
impl<ES> CommandHandler<DepositCommand> for DepositCommandHandler<ES>
where
    ES: EventStore,
{
    async fn handle(&self, command: DepositCommand) -> Result<(), Box<dyn Error + Send + Sync>> {
        let previous_event_envelopes: Vec<EventEnvelope<BankAccountEvent>> = self
            .event_store
            .read::<BankAccountEvent>(&command.id.to_string())
            .await?;
        let previous_events: Vec<BankAccountEvent> = previous_event_envelopes
            .into_iter()
            .map(|event| event.data)
            .collect();
        let new_event = Deposit {
            id: command.id,
            amount: command.amount,
        };
        let current_state: BankAccountAggregate =
            BankAccountAggregate::apply_all(previous_events.clone())?;
        let new_state = BankAccountAggregate::apply(Some(current_state), new_event);
        match new_state {
            Ok(aggregate) => {
                let event_envelope: EventEnvelope<BankAccountEvent> = EventEnvelope::new(
                    aggregate.aggregate_id().to_string(),
                    aggregate.aggregate_type(),
                    new_event,
                    new_event.to_string(),
                    previous_events.len() as i64,
                );
                self.event_store.persist(event_envelope).await
            }
            Err(error) => Err(error.into()),
        }
    }
}
