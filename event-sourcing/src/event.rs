use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub mod envelope;
pub mod listener;
pub mod store;

pub trait Event: Serialize + DeserializeOwned + Sized + Send + Sync + Clone + Debug {
    fn event_type(&self) -> String;
    fn revision(&self) -> i64;
}
