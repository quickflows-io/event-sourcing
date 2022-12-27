use crate::domain::account::aggregate::Account;
use crate::domain::account::event::AccountEvents;
use crate::domain::account::event::AccountEvents::Withdrew;
use chrono::{DateTime, Utc};
use event_sourcing::aggregate::{next_sequence, Aggregate};
use event_sourcing::command::Command;
use event_sourcing::command::EventStoreError::MissingTargetAggregateIdentifier;
use event_sourcing::command_handler::{CommandHandler, CommandResponse};
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;
use event_sourcing::event::Event;
use event_sourcing::snapshot::envelope::SnapshotEnvelope;
use event_sourcing::snapshot::store::SnapshotStore;
use event_sourcing::Error;
use std::collections::HashMap;
use uuid::Uuid;

pub(crate) struct WithdrawCommand {
    pub account_id: String,
    pub amount: i128,
}

impl Command for WithdrawCommand {
    fn target_aggregate_identifier(&self) -> Option<&String> {
        Some(&self.account_id)
    }
}

pub(crate) struct WithdrawCommandHandler<E, S>
where
    E: EventStore<AccountEvents>,
    S: SnapshotStore<Account>,
{
    pub event_store: E,
    pub snapshot_store: S,
}

#[async_trait::async_trait]
impl<E: EventStore<AccountEvents>, S: SnapshotStore<Account>> CommandHandler<WithdrawCommand>
    for WithdrawCommandHandler<E, S>
{
    type Error = Error;

    async fn handle(&self, command: WithdrawCommand) -> Result<CommandResponse, Self::Error> {
        let withdrew_event = Withdrew {
            account_id: command.account_id.clone(),
            amount: command.amount,
        };
        let aggregate_id: &String = command
            .target_aggregate_identifier()
            .ok_or(MissingTargetAggregateIdentifier)?;
        let optional_snapshot_envelope: Option<SnapshotEnvelope<Account>> =
            self.snapshot_store.read(aggregate_id).await?;
        let snapshot_sequence: i64 = optional_snapshot_envelope
            .clone()
            .map(|envelope| envelope.sequence)
            .unwrap_or(0);
        let event_envelopes: Vec<EventEnvelope<AccountEvents>> = self
            .event_store
            .read_from(aggregate_id, snapshot_sequence + 1)
            .await?;
        let events: Vec<AccountEvents> = event_envelopes
            .clone()
            .into_iter()
            .map(|envelope| envelope.event)
            .chain(vec![withdrew_event.clone()]) // Adding our new event
            .collect();
        let bank_account: Account = optional_snapshot_envelope
            .clone()
            .map(|envelope| Account::apply_all(Some(envelope.state), events.clone()))
            .unwrap_or(Account::apply_all(None, events.clone()))?;
        let sequence: i64 = next_sequence(event_envelopes, optional_snapshot_envelope);
        let time: DateTime<Utc> = Utc::now();
        let event_envelope = &EventEnvelope::new(
            bank_account.aggregate_id().clone(), // ID of the aggregate that the envelope belongs to.
            bank_account.aggregate_type(), // Type of the aggregate that the envelope can be applied to.
            withdrew_event.clone(),        // Event attached to the envelope.
            withdrew_event.event_type(),   // Type of the envelope.
            time,                          // Timestamp of when the event was created.
            sequence,                      // Location in a sequence of events.
            withdrew_event.revision(),     // Revision of the event.
            HashMap::from([
                (String::from("trace-id"), Uuid::new_v4().to_string()),
                (String::from("correlation-id"), String::from("")),
            ]),
        );
        self.event_store.persist(event_envelope).await?;
        // Create a snapshot every three events
        if events.len() >= 3 {
            let snapshot_envelope = &SnapshotEnvelope::new(
                bank_account.aggregate_id().clone(),
                bank_account.aggregate_type(),
                bank_account.clone(),
                time,
                sequence,
            );
            self.snapshot_store.persist(snapshot_envelope).await?;
        }

        Ok(CommandResponse {
            aggregate_id: bank_account.aggregate_id().to_string(),
            sequence,
        })
    }
}
