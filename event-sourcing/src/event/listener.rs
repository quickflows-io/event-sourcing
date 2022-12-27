use crate::event::envelope::EventEnvelope;
use crate::event::Event;

pub trait EventListener: Sized + Send + Sync {
    type Error: Send + Sync;

    fn on<E: Event>(&self, event_envelope: EventEnvelope<E>) -> Result<(), Self::Error>;
}

pub trait EventListenerContainer: Sized + Send + Sync {
    type Error: Send + Sync;

    fn start<L: EventListener>(&self, event_listener: L) -> Result<(), Self::Error>;
}
