#[macro_use]
extern crate rocket;
extern crate core;

use api::controller::{deposit, open_bank_account, withdraw};
use event_sourcing::event::stream::EventStream;
use event_store_postgres::{get_connection_pool, run_migrations, EventStoreConnectionPool};
use event_stream_kafka::KafkaEventStream;
use futures::executor::block_on;

use crate::domain::bank_account_balance_projection::BankAccountBalance;
use crate::event::BankAccountEvent;
use event_sourcing::projection::Projection;
use std::thread;

mod api;
mod application;
mod domain;
mod event;

#[launch]
fn rocket() -> _ {
    thread::spawn(|| {
        let stream: KafkaEventStream<BankAccountEvent> = KafkaEventStream {
            group: "bank-account-balance-group".to_owned(),
            topic: "event_store_db.public.event".to_owned(),
            brokers: vec!["localhost:9092".to_owned()],
            apply: |event| BankAccountBalance::apply(event),
        };
        block_on(stream.start())
    });
    dotenv::from_filename("examples/postgres-kafka/bank-account/.env").ok();
    let pool: EventStoreConnectionPool = get_connection_pool().expect("connection pool expect");
    run_migrations(&mut pool.get().expect("connection expected")).expect("migration failed");
    rocket::build()
        .manage(pool)
        .mount("/", routes![open_bank_account, deposit, withdraw])
}
