use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Aggregate: Sized + Send + Sync + Clone + Serialize + DeserializeOwned {
    type AggregateID: Send + Sync + Clone;
    type Event: Send + Sync + Clone;
    type Error: Send + Sync;

    fn aggregate_id(&self) -> &Self::AggregateID;
    fn aggregate_type(&self) -> String;
    fn apply(state: Option<Self>, event: Self::Event) -> Result<Self, Self::Error>;
    fn apply_all(events: Vec<Self::Event>) -> Result<Self, Self::Error>;
}
