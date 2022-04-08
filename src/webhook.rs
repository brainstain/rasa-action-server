use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};
use chrono::{Local, DateTime, Timelike};

use crate::conversation::*;

#[post("/webhook")]
pub async fn webhook(payload: Json<Conversation>) -> HttpResponse {
    let mut conversation = normalize_time(payload.into_inner());

    //Get events for Next Action
    let new_events = match conversation.next_action.as_str() {
        "action_get_time" => get_time(),
        "action_lights" => action_lights(&conversation.tracker.latest_message.entities),
        "action_nodered" => action_nodered(&conversation.tracker.latest_message.entities),
        _ => Vec::new()
    };
    conversation.tracker.events.extend(new_events);

    HttpResponse::Ok()
    .content_type("application/json")
    .json(conversation)
}

#[get("/status")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

fn normalize_time(mut conversation: Conversation) -> Conversation {
    let duration = match conversation.tracker.latest_message.entities.iter().find(|x| x.entity.as_str() == "duration") {
        Some(i) => i,
        _ => return conversation
    };

    let unit = match duration.additional_info.get("unit") {
        Some(i) => i,
        _ => return conversation
    };
    
    let multiplier = match unit.as_str() {
        "second" => 1,
        "minute" => 60,
        "hour" => 3600,
        "day" => 86400,
        "week" => 604800,
        _ => 1,
    };

    let event: Event = Event {
        event: String::from("slot"), 
        timestamp: None, 
        name: Some(String::from("duration")),
        value: (duration.value.trim().parse().unwrap_or(0) * multiplier).to_string()
    };
    conversation.tracker.events.push(event);
    return conversation;
}

fn get_time() -> Vec<Event> {
    let mut times = Vec::new();
    let dt: DateTime<Local> = Local::now();
    let (is_pm, hour) = dt.hour12();
    let time_string = format!("{:02}:{:02} {}", hour, dt.minute(), if is_pm {"PM"} else {"AM"});
    let spoken_time_string = format!("{:02} {:02} {}", hour, dt.minute(), if is_pm {"PM"} else {"AM"});
    
    times.push(
        Event {
            event: String::from("slot"), 
            timestamp: None, 
            name: Some(String::from("current_time")),
            value: time_string
        }
    );

    times.push(
        Event {
            event: String::from("slot"), 
            timestamp: None, 
            name: Some(String::from("current_spoken_time")),
            value: spoken_time_string
        }
    );

    return times;
}

fn action_lights(entities: &Vec<Entity>) -> Vec<Event> {
    return action_entity_generic(entities, "light")
}

fn action_nodered(entities: &Vec<Entity>) -> Vec<Event> {
    return action_entity_generic(entities, "media_player")
}

fn action_entity_generic(entities: &Vec<Entity>, entity: &str) -> Vec<Event> {
    if let Some(light) = entities.iter().find(|x| x.entity.as_str() == entity) {
        if let Some(conf) = light.confidence_entity {
            if conf > 0.7 {
                return vec![
                    Event {
                        event: String::from("slot"), 
                        timestamp: None, 
                        name: Some(String::from(entity)),
                        value: light.value.clone()
                    }
                ]
            }
        } 
    } 
    return Vec::new();
}