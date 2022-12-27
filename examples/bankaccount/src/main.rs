mod api;
mod application;
mod domain;
mod infrastructure;

use event_bus_kafka::event::listener::{
    KafkaConnection, KafkaEventListener, KafkaEventListenerContainer,
};
use rocket::routes;
use std::thread;

use event_sourcing::event::listener::EventListenerContainer;
use event_sourcing::Error;
use event_store_scylladb::event::store::ScyllaDbEventStore;
use event_store_scylladb::snapshot::store::ScyllaDbSnapshotStore;
use event_store_scylladb::{run_migration, ScyllaDbConnection};

use crate::api::v1::controller::{deposit, open, withdraw};

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%dT%H:%M:%SZ]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    setup_logger()?;
    let scylla_db_connection = ScyllaDbConnection {
        host: String::from("localhost:9042"),
        replication_factor: 1,
    };
    let event_store = ScyllaDbEventStore {
        connection: scylla_db_connection.clone(),
    };
    let snapshot_store = ScyllaDbSnapshotStore {
        connection: scylla_db_connection.clone(),
    };

    let kafka_connection = KafkaConnection {
        brokers: vec!["localhost:9092".to_owned()],
        topic: "bankaccount.event_store.events".to_string(),
        group: "bankaccount.event_store.events".to_string(),
    };

    thread::spawn(|| {
        let event_listener = KafkaEventListener;
        let event_listener_container = KafkaEventListenerContainer {
            connection: kafka_connection,
        };
        if let Err(e) = event_listener_container.start(event_listener) {
            println!("Failed consuming messages: {e}");
        }
    });

    run_migration(&scylla_db_connection).await?;

    let _rocket = rocket::build()
        .manage(event_store)
        .manage(snapshot_store)
        .mount("/api/v1", routes![open, deposit, withdraw])
        .launch()
        .await?;

    Ok(())
}
