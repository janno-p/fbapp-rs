use futures::Stream;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct PersistedEvent<Event> {
    pub source_id: Uuid,
    pub sequence_number: u32,
    pub payload: Event,
}

impl<Event> PersistedEvent<Event> {
    pub fn as_ref(&self) -> PersistedEvent<&Event> {
        PersistedEvent {
            source_id: self.source_id,
            sequence_number: self.sequence_number,
            payload: &self.payload,
        }
    }

    pub fn map<NewEvent, F>(self, f: F) -> PersistedEvent<NewEvent>
        where F: FnOnce(Event) -> NewEvent
    {
        PersistedEvent {
            source_id: self.source_id,
            sequence_number: self.sequence_number,
            payload: f(self.payload),
        }
    }
}

pub trait EventStore {
    type Event;
    type EventsStream: Stream<Item = PersistedEvent<Self::Event>>;
    fn store_events(&self, source_id: Uuid, events: Vec<Self::Event>);
    fn load_events(&self, source_id: Uuid, offset: usize) -> Self::EventsStream;
}
