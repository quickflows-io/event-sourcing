use chrono::{DateTime, Utc};

use crate::aggregate::Aggregate;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, derive_new::new)]
pub struct SnapshotEnvelope<A: Aggregate> {
    // ID of the aggregate that the envelope belongs to.
    pub aggregate_id: String,
    // Type of the aggregate that the envelope can be applied to.
    pub aggregate_type: String,
    // Event attached to the envelope.
    #[serde(deserialize_with = "deserialize_state")]
    #[serde(bound(deserialize = "A: FromStr, A::Err: Display"))]
    pub state: A,
    // Timestamp of when the state was created.
    pub state_time: DateTime<Utc>,
    // Location in a sequence of events that has been captured.
    pub sequence: i64,
}

fn deserialize_state<'de, S, D>(deserializer: D) -> Result<S, D::Error>
where
    S: FromStr,
    S::Err: Display,
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    S::from_str(&s).map_err(de::Error::custom)
}
