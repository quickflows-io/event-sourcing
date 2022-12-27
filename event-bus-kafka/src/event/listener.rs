use event_sourcing::event::envelope::EventEnvelope;
use event_sourcing::event::Event;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use log::{debug, error};
use retry::delay::Fixed;
use retry::retry;

use event_sourcing::event::listener::{EventListener, EventListenerContainer};

pub struct KafkaEventListener;

impl EventListener for KafkaEventListener {
    type Error = event_sourcing::Error;

    fn on<E: Event>(&self, _event_envelope: EventEnvelope<E>) -> Result<(), Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct KafkaConnection {
    pub brokers: Vec<String>,
    pub topic: String,
    pub group: String,
}

pub struct KafkaEventListenerContainer {
    pub connection: KafkaConnection,
}

impl EventListenerContainer for KafkaEventListenerContainer {
    type Error = event_sourcing::Error;

    fn start<L: EventListener>(&self, _event_listener: L) -> Result<(), Self::Error> {
        let brokers = &self.connection.brokers;
        let topic = &self.connection.topic;
        let group = &self.connection.group;
        let mut consumer = retry(Fixed::from_millis(1000), || {
            let result = Consumer::from_hosts(brokers.clone())
                .with_topic(topic.clone())
                .with_group(group.clone())
                .with_fallback_offset(FetchOffset::Earliest)
                .with_offset_storage(GroupOffsetStorage::Kafka)
                .create();
            if result.is_err() {
                error!("Failed consuming messages: {:?}", result)
            }
            result
        })?;
        loop {
            let message_sets = consumer.poll()?;
            for message_set in message_sets.iter() {
                for message in message_set.messages() {
                    debug!(
                        "{}:{}@{}: {:?}",
                        message_set.topic(),
                        message_set.partition(),
                        message.offset,
                        String::from_utf8(message.value.to_vec())
                    );
                }
                let _ = consumer.consume_messageset(message_set);
            }
            consumer.commit_consumed()?;
        }
    }
}
