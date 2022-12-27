use crate::domain::account::event::AccountEvents;
use crate::domain::account::event::AccountEvents::{Closed, Deposited, Opened, Withdrew};
use event_sourcing::aggregate::Aggregate;
use event_sourcing::Error;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Account {
    pub name: String,
    pub account_id: String,
    pub balance: i128,
}

impl Aggregate for Account {
    type AggregateID = String;
    type Event = AccountEvents;
    type Error = Error;

    fn aggregate_id(&self) -> &Self::AggregateID {
        &self.account_id
    }

    fn aggregate_type(&self) -> String {
        String::from("BankAccount")
    }

    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
        match (state, event) {
            (None, Opened { account_id, name }) => Ok(Self {
                name,
                account_id,
                balance: 0,
            }),
            (
                Some(mut bank_account),
                Deposited {
                    account_id: _,
                    amount,
                },
            ) => {
                bank_account.balance += amount;
                Ok(bank_account)
            }
            (
                Some(mut bank_account),
                Withdrew {
                    account_id: _,
                    amount,
                },
            ) => {
                let balance = bank_account.balance - amount;
                Self::validate_balance(balance)?;
                bank_account.balance = balance;
                Ok(bank_account)
            }
            (Some(bank_account), Closed { .. }) => Ok(bank_account),
            _ => Err(Error::from("Invalid event")),
        }
    }

    fn apply_all(state: Option<Self>, events: Vec<Self::Event>) -> Result<Self, Self::Error> {
        match events.into_iter().fold(Ok(state), |state, event| {
            Self::apply(state?, event).map(Some)
        }) {
            Ok(Some(state)) => Ok(state),
            Ok(None) => Err(Error::from("Aggregate must not be None")),
            Err(error) => Err(error),
        }
    }
}

impl Account {
    fn validate_balance(balance: i128) -> Result<(), Error> {
        if balance < 0 {
            Err(Error::from("Withdraw amount exceeds available balance."))
        } else {
            Ok(())
        }
    }
}
