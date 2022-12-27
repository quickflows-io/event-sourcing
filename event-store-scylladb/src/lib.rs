use event_sourcing::Error;
use futures::executor;
use log::error;
use retry::delay::Fixed;
use retry::retry;
use scylla::{Session, SessionBuilder};

pub mod event;
pub(crate) mod query;
pub mod snapshot;

#[derive(Debug, Clone)]
pub struct ScyllaDbConnection {
    pub host: String,
    pub replication_factor: i8,
}

pub async fn run_migration(connection: &ScyllaDbConnection) -> Result<(), Error> {
    let session: Session = retry(Fixed::from_millis(1000), || {
        let result = executor::block_on(SessionBuilder::new().known_node(&connection.host).build());
        if result.is_err() {
            error!("Failed to run migration: {:?}", result)
        }
        result
    })?;
    session
        .query(query::create_keyspace(&connection.replication_factor), &[])
        .await?;
    session.query(query::CREATE_EVENTS_TABLE, &[]).await?;
    session.query(query::CREATE_SNAPSHOT_TABLE, &[]).await?;
    Ok(())
}
