use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use serde_json;
use std::env;
use uuid::Uuid;

use event_store::models::{Event as DbEvent};
use framework::event_store::{PersistedEvent};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn store_events<Event>(source_id: Uuid, events: Vec<Event>)
    where Event: Serialize
{
    if events.is_empty() {
        return;
    }

    let new_events: Vec<models::NewEvent> = events
        .into_iter()
        .map(|ev| {
            models::NewEvent {
                source_id,
                sequence_number: 1,
                payload: serde_json::to_value(&ev).unwrap(),
            }
        })
        .collect();

    let connection = establish_connection();

    diesel::insert(&new_events)
        .into(schema::events::table)
        .execute(&connection)
        .unwrap();
}

pub fn load_events<Event>(source_id: Uuid) -> Vec<PersistedEvent<Event>>
    where Event: DeserializeOwned
{
    use event_store::schema::events::dsl;

    let connection = establish_connection();

    let result = dsl::events.filter(dsl::source_id.eq(source_id).and(dsl::sequence_number.gt(0)))
        .order(dsl::sequence_number.asc())
        .load::<DbEvent>(&connection)
        .expect("Error loading events");

    result.into_iter()
        .map(|event| {
            PersistedEvent {
                source_id: source_id,
                sequence_number: event.sequence_number as u32,
                payload: serde_json::from_value::<Event>(event.payload.clone()).unwrap(),
            }
        })
        .collect::<Vec<PersistedEvent<Event>>>()
}
