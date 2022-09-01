mod store;
mod stream;

use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use uuid::Uuid;

/// Event is a domain event describing a change that has happened to an aggregate.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event<Data> {
    // Unique identifier of the event.
    pub id: Uuid,
    // ID of the aggregate that the event belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the event can be applied to.
    pub aggregate_type: String,
    // Data attached to the event.
    #[serde(deserialize_with = "deserialize_generic")]
    #[serde(bound(deserialize = "Data: FromStr, Data::Err: Display"))]
    pub data: Data,
    // Type of the event.
    pub event_type: String,
    // Version of the aggregate after the event has been applied.
    pub version: u64,
    // Timestamp of when the event was created.
    pub timestamp: DateTime<Utc>,
    // App-specific metadata such as request ID, originating user etc.
    pub metadata: HashMap<String, String>,
}

fn deserialize_generic<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
