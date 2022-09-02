use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use event_sourcing::aggregate::Aggregate;

use crate::event::BankAccountEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BankAccountAggregate {
    id: Uuid,
    balance: i64,
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum BankAccountError {
    #[error("invalid envelope")]
    InvalidEvent,
    #[error("invalid state")]
    InvalidState,
    #[error("withdrawal exceeds balance")]
    WithdrawalExceedsBalance,
    #[error("ID mismatch")]
    IdMismatch,
}

impl Aggregate for BankAccountAggregate {
    type AggregateID = Uuid;
    type Event = BankAccountEvent;
    type Error = BankAccountError;

    fn aggregate_id(&self) -> &Self::AggregateID {
        &self.id
    }

    fn aggregate_type(&self) -> String {
        return String::from("BankAccount");
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (None, BankAccountEvent::OpenBankAccount { id }) => Ok(Self { id, balance: 0 }),
            (Some(bank_account), BankAccountEvent::Deposit { id, amount }) => {
                if id != bank_account.id {
                    return Err(BankAccountError::IdMismatch);
                }
                Ok(Self {
                    id: bank_account.id,
                    balance: bank_account.balance + amount,
                })
            }
            (Some(bank_account), BankAccountEvent::Withdraw { id, amount }) => {
                if amount > bank_account.balance {
                    return Err(BankAccountError::WithdrawalExceedsBalance);
                }
                if id != bank_account.id {
                    return Err(BankAccountError::IdMismatch);
                }
                Ok(Self {
                    id: bank_account.id,
                    balance: bank_account.balance - amount,
                })
            }
            _ => Err(BankAccountError::InvalidEvent),
        }
    }

    fn apply_all(events: Vec<Self::Event>) -> Result<Self, Self::Error> {
        let state_result: Result<Option<BankAccountAggregate>, BankAccountError> =
            events.into_iter().fold(Ok(None), |state, event| {
                Self::apply(state?, event).map(|new_state| Some(new_state))
            });
        match state_result {
            Ok(Some(state)) => Ok(state),
            Ok(None) => Err(BankAccountError::InvalidState),
            Err(error) => Err(error),
        }
    }
}
