pub mod envelope;
pub mod store;

pub enum SnapshotStrategy {
    // Snapshot every N number of events.
    NumberOfEvents(i32),
}
