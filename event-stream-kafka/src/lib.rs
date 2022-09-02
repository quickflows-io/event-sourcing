use std::error::Error;

use event_sourcing::event::envelope::{deserialize, EventEnvelope};
use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::{Consumer, MessageSets};
use retry::delay::Fixed;
use retry::retry;
use serde::de::DeserializeOwned;
use serde::Serialize;

use event_sourcing::event::stream::EventStream;

use crate::KafkaEventStreamError::InternalError;

#[derive(Debug, Clone)]
pub struct KafkaEventStream<Event>
where
    Event: Send + Sync + Clone + Serialize + DeserializeOwned,
{
    pub group: String,
    pub topic: String,
    pub brokers: Vec<String>,
    pub apply: fn(event: EventEnvelope<Event>) -> Result<(), Box<dyn Error>>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub(crate) enum KafkaEventStreamError {
    #[error("Error `{0}`")]
    InternalError(String),
}

#[async_trait::async_trait]
impl<Event> EventStream for KafkaEventStream<Event>
where
    Event: Send + Sync + Clone + Serialize + DeserializeOwned,
{
    async fn start(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        retry(Fixed::from_millis(1000), || {
            match Consumer::from_hosts(self.brokers.clone())
                .with_topic(self.topic.clone())
                .with_group(self.group.clone())
                .with_fallback_offset(FetchOffset::Earliest)
                .with_offset_storage(GroupOffsetStorage::Kafka)
                .create()
            {
                Ok(consumer) => Self::start_consumer(consumer, self.apply),
                Err(e) => Err(InternalError(format!("{:?}", e))),
            }
        })
        .map_err(|e| e.into())
    }
}

impl<Event> KafkaEventStream<Event>
where
    Event: Send + Sync + Clone + Serialize + DeserializeOwned,
{
    fn start_consumer(
        mut consumer: Consumer,
        apply: fn(EventEnvelope<Event>) -> Result<(), Box<dyn Error>>,
    ) -> Result<(), KafkaEventStreamError> {
        loop {
            let message_sets: MessageSets = consumer
                .poll()
                .map_err(|e| InternalError(format!("{:?}", e)))?;
            for message_set in message_sets.iter() {
                for message in message_set.messages() {
                    let serialized_event_envelope = String::from_utf8_lossy(message.value)
                        .to_string()
                        .replace("\\\"", "\"")
                        .replace("\"{", "{")
                        .replace("}\"", "}");
                    let event_envelope = deserialize(serialized_event_envelope)
                        .map_err(|e| InternalError(format!("{:?}", e)))?;
                    apply(event_envelope).map_err(|e| InternalError(format!("{:?}", e)))?
                }
                consumer
                    .consume_messageset(message_set)
                    .map_err(|e| InternalError(format!("{:?}", e)))?;
            }
            consumer
                .commit_consumed()
                .map_err(|e| InternalError(format!("{:?}", e)))?;
        }
    }
}
