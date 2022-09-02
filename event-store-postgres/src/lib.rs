#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate diesel;

use std::env;
use std::error::Error;

use diesel::pg::Pg;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenv::dotenv;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::entity::EventEntity;
use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::store::EventStore;

mod entity;
pub(crate) mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub type EventStoreConnectionPool = Pool<ConnectionManager<EventStoreConnection>>;
pub type EventStoreConnection = PgConnection;

pub fn get_connection_pool() -> Result<EventStoreConnectionPool, Box<dyn Error>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    Pool::builder().build(manager).map_err(|e| e.into())
}

pub fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    connection
        .run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|e| e.into())
}

#[derive(Debug, Clone, new)]
pub struct PostgresEventStore {
    connection_pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait::async_trait]
impl EventStore for PostgresEventStore {
    async fn read<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        agg_id: &String,
    ) -> Result<Vec<EventEnvelope<Data>>, Box<dyn Error + Send + Sync>> {
        use crate::schema::event::dsl::*;
        self.connection_pool
            .get()?
            .transaction::<_, diesel::result::Error, _>(|connection| {
                let event_envelopes: Vec<
                    Result<EventEnvelope<Data>, Box<dyn Error + Send + Sync>>,
                > = event
                    .filter(aggregate_id.eq(agg_id))
                    .order(version.asc())
                    .load::<EventEntity>(connection)?
                    .into_iter()
                    .map(|entity| entity.envelope())
                    .collect();
                Ok(event_envelopes.into_iter().collect())
            })?
    }

    async fn persist<Data: Send + Sync + Clone + Serialize + DeserializeOwned>(
        &self,
        event_envelope: EventEnvelope<Data>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        use crate::schema::event::dsl::*;
        let entity = &EventEntity::entity(event_envelope)?;
        self.connection_pool
            .get()?
            .transaction::<_, diesel::result::Error, _>(|connection| {
                diesel::insert_into(event)
                    .values(entity)
                    .execute(connection)
            })
            .map(|_| ())
            .map_err(|e| e.into())
    }
}
