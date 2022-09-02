use std::fmt::{Display, Formatter, Result};
use strum_macros::EnumString;

#[derive(Debug, Clone, Copy, PartialEq, EnumString, serde::Serialize, serde::Deserialize)]
pub enum BankAccountEvent {
    OpenBankAccount { id: uuid::Uuid },
    Deposit { id: uuid::Uuid, amount: i64 },
    Withdraw { id: uuid::Uuid, amount: i64 },
}

impl Display for BankAccountEvent {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            BankAccountEvent::OpenBankAccount { .. } => write!(f, "OpenBankAccount"),
            BankAccountEvent::Deposit { .. } => write!(f, "Deposit"),
            BankAccountEvent::Withdraw { .. } => write!(f, "Withdraw"),
        }
    }
}
