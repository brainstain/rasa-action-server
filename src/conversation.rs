use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Conversation {
    pub next_action: String,
    pub sender_id: String,
    pub tracker: Tracker,
    pub domain: Domain
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tracker {
    pub conversation_id: String,
    pub slots: HashMap<String, String>,
    pub latest_message: Message,
    pub latest_event_time: f64,
    pub followup_action: String,
    pub paused: bool,
    pub events: Vec<Event>,
    pub latest_input_channel: String,
    pub latest_action_name: String,
    pub latest_action: Action,
    pub active_loop: Loop
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub config: Config,
    pub intents: Vec<HashMap<String, DomainIntentProperty>>,
    pub entities: Vec<String>,
    pub slots: HashMap<String, SlotProperty>,
    pub responses: HashMap<String, Vec<Response>>,
    pub actions: Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SlotProperty {
    pub auto_fill: bool,
    pub initial_value: String,
    pub r#type: String,
    pub values: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub entities: Vec<Entity>,
    pub intent: Intent,
    pub intent_ranking: Vec<Intent>,
    pub text: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    pub start: u32,
    pub end: u32,
    pub value: String,
    pub entity: String,
    pub confidence: f32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intent {
    pub confidence: f32,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub event: String,
    pub timestamp: u32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub action_name: String,
    pub action_text: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Loop {
    pub name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub store_entities_as_slots: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomainIntentProperty {
    pub use_entities: bool
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub text: String
}