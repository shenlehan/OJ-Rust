use actix_web::{App, HttpServer, Responder, get, middleware::Logger, post, web};
use oj::configs::read_config_file;
use std::fs;
use std::fs::File;
use std::io::Read;
use oj::types::*;
use serde_json::from_str;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    log::info!(target: "greet_handler", "Greeting {}", name);
    format!("Hello {name}!")
}

// DO NOT REMOVE: used in automatic testing
#[post("/internal/exit")]
#[allow(unreachable_code)]
async fn exit() -> impl Responder {
    log::info!("Shutdown as requested");
    std::process::exit(0);
    "Exited"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut config_file_path = String::from("");
    read_config_file(&mut config_file_path)?;
    let mut config_json = std::fs::read_to_string(config_file_path)?;
    let config: OJConfig = from_str(&config_json)?;

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            // DO NOT REMOVE: used in automatic testing
            .service(exit)
    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}
