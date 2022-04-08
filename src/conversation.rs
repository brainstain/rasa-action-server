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
    pub conversation_id: Option<String>,
    pub slots: HashMap<String, Option<String>>,
    pub latest_message: Message,
    pub latest_event_time: f64,
    pub followup_action: Option<String>,
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
    pub initial_value: Option<String>,
    pub r#type: String,
    #[serde(default)]
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
    #[serde(default)]
    pub start: u32,
    #[serde(default)]
    pub end: u32,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub entity: String,
    #[serde(default)]
    pub confidence: f32,
    pub confidence_entity: Option<f32>,
    #[serde(default)]
    pub extractor: String,
    #[serde(default)]
    pub additional_info: HashMap<String, String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intent {
    pub confidence: f32,
    pub name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    pub event: String,
    pub timestamp: Option<f32>,
    pub name: Option<String>,
    #[serde(default)]
    pub value: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub action_name: String,
    pub action_text: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Loop {
    pub name: Option<String>
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