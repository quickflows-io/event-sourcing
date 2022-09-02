use serde::{Deserialize, Serialize};
use uuid::Uuid;

use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::projection::Projection;

use crate::event::BankAccountEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BankAccountBalance {
    id: Uuid,
    balance: i64,
}

impl Projection for BankAccountBalance {
    type Event = EventEnvelope<BankAccountEvent>;

    fn apply(event: Self::Event) -> Result<(), Box<dyn std::error::Error>> {
        Ok(match event.data {
            BankAccountEvent::OpenBankAccount { id } => {
                println!("{}", id)
            }
            BankAccountEvent::Deposit { id, amount } => {
                println!("{} depositing {}", id, amount)
            }
            BankAccountEvent::Withdraw { id, amount } => {
                println!("{} withdrawing {}", id, amount)
            }
        })
    }
}
