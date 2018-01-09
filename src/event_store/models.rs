use serde_json::{Value as JsonValue};
use diesel::data_types::PgTimestamp;
use uuid::Uuid;

use event_store::schema::events;

#[derive(Debug, Clone, Insertable)]
#[table_name="events"]
pub struct NewEvent {
    pub source_id: Uuid,
    pub sequence_number: i64,
    pub payload: JsonValue,
}

#[derive(Debug, Clone, Queryable)]
pub struct Event {
    pub source_id: Uuid,
    pub sequence_number: i64,
    pub payload: JsonValue,
    pub created_at: PgTimestamp,
}
