mod aggregate;
mod cqrs;
mod event;

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::aggregate::Aggregate;
    use crate::cqrs::CommandHandler;
    use uuid::Uuid;

    #[derive(Debug, Clone)]
    enum BankAccountError {
        InvalidEvent,
    }

    #[derive(Debug, Clone)]
    enum BankAccountEvent {
        OpenBankAccount,
    }

    #[derive(Debug, Clone)]
    struct OpenBankAccountCommand {
        user_id: Uuid,
    }

    #[derive(Debug, Clone)]
    struct OpenBankAccountCommandHandler;

    #[async_trait::async_trait]
    impl CommandHandler<OpenBankAccountCommand> for OpenBankAccountCommandHandler {
        type Error = BankAccountError;

        async fn handle(&self, _command: OpenBankAccountCommand) -> Result<(), Self::Error> {
            BankAccount::apply(None, BankAccountEvent::OpenBankAccount)
                .await
                .map(|_| ())
        }
    }

    #[derive(Debug, Clone)]
    struct BankAccount {
        id: Uuid,
        balance: u64,
    }

    #[async_trait::async_trait]
    impl Aggregate for BankAccount {
        type Event = BankAccountEvent;
        type Error = BankAccountError;

        async fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error> {
            match (state, event) {
                (None, BankAccountEvent::OpenBankAccount) => Ok(Self {
                    id: Uuid::new_v4(),
                    balance: 0,
                }),
                _ => Err(BankAccountError::InvalidEvent),
            }
        }
    }

    #[tokio::test]
    async fn open_bank_account() {
        let open_bank_account_command = OpenBankAccountCommand {
            user_id: Uuid::new_v4(),
        };
        let open_bank_account_command_handler = OpenBankAccountCommandHandler
            .handle(open_bank_account_command)
            .await;
        println!("{:?}", open_bank_account_command_handler);
        assert_eq!(open_bank_account_command_handler.is_ok(), true);
    }
}
