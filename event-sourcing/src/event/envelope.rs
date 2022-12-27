use chrono::{DateTime, Utc};
use std::collections::HashMap;

use crate::event::Event;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

/// Event is a domain envelope describing a change that has happened to an aggregate.
#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct EventEnvelope<E: Event> {
    // ID of the aggregate that the envelope belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the envelope can be applied to.
    pub aggregate_type: String,
    // Event attached to the envelope.
    #[serde(deserialize_with = "deserialize_event")]
    #[serde(bound(deserialize = "E: FromStr, E::Err: Display"))]
    pub event: E,
    // Type of the envelope.
    pub event_type: String,
    // Timestamp of when the event was created.
    pub event_time: DateTime<Utc>,
    // Location in a sequence of events.
    pub sequence: i64,
    // Revision of the event.
    pub revision: i64,
    // Metadata for the event.
    pub metadata: HashMap<String, String>,
}

fn deserialize_event<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
