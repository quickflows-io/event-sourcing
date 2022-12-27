use event_sourcing::event::Event;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub enum AccountEvents {
    Opened { account_id: String, name: String },
    Closed { account_id: String },
    Withdrew { account_id: String, amount: i128 },
    Deposited { account_id: String, amount: i128 },
}

impl Event for AccountEvents {
    fn event_type(&self) -> String {
        match &self {
            AccountEvents::Opened { .. } => String::from("Opened"),
            AccountEvents::Closed { .. } => String::from("Closed"),
            AccountEvents::Withdrew { .. } => String::from("Withdrew"),
            AccountEvents::Deposited { .. } => String::from("Deposited"),
        }
    }

    fn revision(&self) -> i64 {
        match &self {
            AccountEvents::Opened { .. } => 1,
            AccountEvents::Closed { .. } => 1,
            AccountEvents::Withdrew { .. } => 1,
            AccountEvents::Deposited { .. } => 1,
        }
    }
}
