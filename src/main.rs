extern crate actix_web;

use actix_web::{App, HttpServer};

mod webhook;
mod conversation;

use crate::webhook::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(webhook)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
