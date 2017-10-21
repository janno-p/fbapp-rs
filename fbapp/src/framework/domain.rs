use super::event_store::PersistedEvent;
use std::fmt::Debug;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use uuid::Uuid;

pub trait AggregateRoot {
    type Command;
    type CommandError: Debug;
    type State;
    type Event: DeserializeOwned + Serialize;

    fn id(&self) -> Uuid;
    fn uncommited_events(&self) -> Vec<Self::Event>;
    fn decide(&mut self, command: Self::Command) -> Result<(), Self::CommandError>;
    fn evolve(&mut self, event: &Self::Event);
    fn add_uncommited_event(&mut self, event: Self::Event);

    fn emit(&mut self, event: Self::Event) {
        self.evolve(&event);
        self.add_uncommited_event(event);
    }

    fn rebuild<FnLoad>(&mut self, load_events: FnLoad)
        where FnLoad: (Fn(Uuid) -> Vec<PersistedEvent<Self::Event>>)
    {
        for persisted_event in load_events(self.id()) {
            self.evolve(&persisted_event.payload);
        }
    }

    fn handle_command<FnLoad, FnStore>(&mut self, command: Self::Command, load_events: FnLoad, store_events: FnStore) -> Result<(), Self::CommandError>
        where FnLoad: (Fn(Uuid) -> Vec<PersistedEvent<Self::Event>>),
            FnStore: (Fn(Uuid, Vec<Self::Event>) -> ())
    {
        self.rebuild(load_events);
        self.decide(command)?;
        store_events(self.id(), self.uncommited_events());
        Ok(())
    }
}