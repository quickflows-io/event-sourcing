use rocket::serde::json::Json;
use rocket::State;
use serde::Deserialize;
use uuid::Uuid;

use event_sourcing::cqrs::command_handler::CommandHandler;
use event_store_postgres::EventStoreConnectionPool;
use event_store_postgres::PostgresEventStore as EventStore;

use crate::application::deposit_handler::{DepositCommand, DepositCommandHandler};
use crate::application::open_bank_account_handler::{
    OpenBankAccountCommand, OpenBankAccountCommandHandler,
};
use crate::application::withdraw_handler::{WithdrawCommand, WithdrawCommandHandler};

#[derive(Debug, Deserialize, FromForm)]
pub(crate) struct OpenBankAccountRequest {
    id: Uuid,
}

#[derive(Debug, Deserialize, FromForm)]
pub(crate) struct DepositRequest {
    id: Uuid,
    amount: i64,
}

#[derive(Debug, Deserialize, FromForm)]
pub(crate) struct WithdrawRequest {
    id: Uuid,
    amount: i64,
}

#[post("/bank/account", data = "<request>")]
pub(crate) async fn open_bank_account(
    request: Json<OpenBankAccountRequest>,
    connection_pool: &State<EventStoreConnectionPool>,
) -> Result<(), String> {
    OpenBankAccountCommandHandler {
        event_store: EventStore::new(connection_pool.inner().clone()),
    }
    .handle(OpenBankAccountCommand { id: request.id })
    .await
    .map_err(|e| e.to_string())
}

#[post("/bank/account/deposit", data = "<request>")]
pub(crate) async fn deposit(
    request: Json<DepositRequest>,
    connection_pool: &State<EventStoreConnectionPool>,
) -> Result<(), String> {
    DepositCommandHandler {
        event_store: EventStore::new(connection_pool.inner().clone()),
    }
    .handle(DepositCommand {
        id: request.id,
        amount: request.amount,
    })
    .await
    .map_err(|e| e.to_string())
}

#[post("/bank/account/withdraw", data = "<request>")]
pub(crate) async fn withdraw(
    request: Json<WithdrawRequest>,
    connection_pool: &State<EventStoreConnectionPool>,
) -> Result<(), String> {
    WithdrawCommandHandler {
        event_store: EventStore::new(connection_pool.inner().clone()),
    }
    .handle(WithdrawCommand {
        id: request.id,
        amount: request.amount,
    })
    .await
    .map_err(|e| e.to_string())
}
