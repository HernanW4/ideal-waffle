use actix_web::middleware::Logger;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use log::{error, info};

use std::env;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/index")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello index")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));


    let port = match env::var("PORT") {
        Ok(port) => port,
        Err(_) => {
            info!("NO PORT VARIABLE, DEFAULTING TO 80");
            8080.to_string()
        }
    };

    let addr = ("0.0.0.0", port.parse().unwrap());

    HttpServer::new(|| App::new().service(hello).service(index))
        .bind(addr)?
        .run()
        .await
}
