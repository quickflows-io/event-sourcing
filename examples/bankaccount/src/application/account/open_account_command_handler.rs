use crate::domain::account::aggregate::Account;
use crate::domain::account::event::AccountEvents;
use crate::domain::account::event::AccountEvents::Opened;
use chrono::Utc;
use event_sourcing::aggregate::{next_sequence, Aggregate};
use event_sourcing::command::Command;
use event_sourcing::command_handler::{CommandHandler, CommandResponse};
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;
use event_sourcing::event::Event;
use event_sourcing::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) struct OpenAccountCommand {
    pub name: String,
}

impl Command for OpenAccountCommand {
    fn target_aggregate_identifier(&self) -> Option<&String> {
        None // Not present during creation of the aggregate.
    }
}

pub(crate) struct OpenAccountCommandHandler<E>
where
    E: EventStore<AccountEvents>,
{
    pub event_store: E,
}

#[async_trait::async_trait]
impl<E: EventStore<AccountEvents>> CommandHandler<OpenAccountCommand>
    for OpenAccountCommandHandler<E>
{
    type Error = Error;

    async fn handle(&self, command: OpenAccountCommand) -> Result<CommandResponse, Self::Error> {
        let open_account_event = Opened {
            account_id: Uuid::new_v4().to_string(),
            name: command.name,
        };
        let bank_account: Account = Account::apply(None, open_account_event.clone())?;
        let sequence = next_sequence::<AccountEvents, Account>(vec![], None);
        let event_envelope: &EventEnvelope<AccountEvents> = &EventEnvelope::new(
            bank_account.aggregate_id().to_string(), // ID of the aggregate that the envelope belongs to.
            bank_account.aggregate_type(), // Type of the aggregate that the envelope can be applied to.
            open_account_event.clone(),    // Event attached to the envelope.
            open_account_event.event_type(), // Type of the envelope.
            Utc::now(),                    // Timestamp of when the event was created.
            sequence,                      // Location in a sequence of events.
            open_account_event.revision(), // Revision of the event.
            HashMap::from([
                (String::from("trace-id"), Uuid::new_v4().to_string()),
                (String::from("correlation-id"), String::from("")),
            ]),
        );
        self.event_store.persist(event_envelope).await?;

        Ok(CommandResponse {
            aggregate_id: bank_account.aggregate_id().to_string(),
            sequence,
        })
    }
}
