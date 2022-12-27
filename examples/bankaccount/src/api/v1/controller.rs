use rocket::{post, State};

use crate::application::account::deposit_command_handler::{DepositCommand, DepositCommandHandler};
use crate::application::account::open_account_command_handler::{
    OpenAccountCommand, OpenAccountCommandHandler,
};
use crate::application::account::withdraw_command_handler::{
    WithdrawCommand, WithdrawCommandHandler,
};
use event_sourcing::command_handler::{CommandHandler, CommandResponse};

use event_store_scylladb::event::store::ScyllaDbEventStore;
use event_store_scylladb::snapshot::store::ScyllaDbSnapshotStore;
use rocket::serde::json::Json;

#[derive(serde::Deserialize)]
pub(crate) struct OpenAccount {
    name: String,
}

#[derive(serde::Deserialize)]
pub(crate) struct Deposit {
    amount: i128,
}

#[derive(serde::Deserialize)]
pub(crate) struct Withdraw {
    amount: i128,
}

#[post("/open", data = "<request>")]
pub(crate) async fn open(
    request: Json<OpenAccount>,
    event_store: &State<ScyllaDbEventStore>,
    _snapshot_store: &State<ScyllaDbSnapshotStore>,
) -> Result<Json<CommandResponse>, String> {
    let command = OpenAccountCommand {
        name: request.into_inner().name,
    };
    let account_opened_command_handler = OpenAccountCommandHandler {
        event_store: event_store.inner().clone(),
    };
    account_opened_command_handler
        .handle(command)
        .await
        .map(Json::from)
        .map_err(|error| error.to_string())
}

#[post("/deposit/<account_id>", data = "<request>")]
pub(crate) async fn deposit(
    request: Json<Deposit>,
    account_id: String,
    event_store: &State<ScyllaDbEventStore>,
    snapshot_store: &State<ScyllaDbSnapshotStore>,
) -> Result<Json<CommandResponse>, String> {
    let command = DepositCommand {
        account_id,
        amount: request.into_inner().amount,
    };
    let deposited_command_handler = DepositCommandHandler {
        event_store: event_store.inner().clone(),
        snapshot_store: snapshot_store.inner().clone(),
    };
    deposited_command_handler
        .handle(command)
        .await
        .map(Json::from)
        .map_err(|error| error.to_string())
}

#[post("/withdraw/<account_id>", data = "<request>")]
pub(crate) async fn withdraw(
    request: Json<Withdraw>,
    account_id: String,
    event_store: &State<ScyllaDbEventStore>,
    snapshot_store: &State<ScyllaDbSnapshotStore>,
) -> Result<Json<CommandResponse>, String> {
    let command = WithdrawCommand {
        account_id,
        amount: request.into_inner().amount,
    };
    let withdraw_command_handler = WithdrawCommandHandler {
        event_store: event_store.inner().clone(),
        snapshot_store: snapshot_store.inner().clone(),
    };
    withdraw_command_handler
        .handle(command)
        .await
        .map(Json::from)
        .map_err(|error| error.to_string())
}
