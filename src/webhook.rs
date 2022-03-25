use actix_web::web::Json;
use actix_web::{get, post, HttpResponse, Responder};

use crate::conversation::*;

#[post("/webhook")]
pub async fn webhook(payload: Json<Conversation>) -> HttpResponse {
    HttpResponse::Created()
    .content_type("application/json")
    .json(payload)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}