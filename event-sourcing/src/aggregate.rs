use crate::event::envelope::EventEnvelope;
use crate::event::Event;
use crate::snapshot::envelope::SnapshotEnvelope;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

/// An aggregate is a cluster of associated events that is treated as a unit for the purpose of data changes.
pub trait Aggregate: Sized + Send + Sync + Clone + Serialize + DeserializeOwned {
    type AggregateID: Send + Sync + Clone;
    type Event: Send + Sync + Clone + Debug;
    type Error: Send + Sync;

    fn aggregate_id(&self) -> &Self::AggregateID;
    fn aggregate_type(&self) -> String;
    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
    fn apply_all(state: Option<Self>, events: Vec<Self::Event>) -> Result<Self, Self::Error>;
}

pub fn current_sequence<E: Event, A: Aggregate>(
    mut event_envelopes: Vec<EventEnvelope<E>>,
    snapshot_envelope: Option<SnapshotEnvelope<A>>,
) -> i64 {
    event_envelopes.sort_by_key(|event_envelope| event_envelope.sequence);
    event_envelopes
        .last()
        .map(|envelope| envelope.sequence)
        .unwrap_or(
            snapshot_envelope
                .map(|snapshot_envelope| snapshot_envelope.sequence)
                .unwrap_or(0),
        )
}

pub fn next_sequence<E: Event, A: Aggregate>(
    event_envelopes: Vec<EventEnvelope<E>>,
    snapshot_envelope: Option<SnapshotEnvelope<A>>,
) -> i64 {
    current_sequence(event_envelopes, snapshot_envelope) + 1
}
