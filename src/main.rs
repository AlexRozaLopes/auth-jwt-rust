mod controller;
mod data;
mod schema;
mod utils;

use crate::controller::user_controller::{get_token, post_user};
use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(post_user)
            .service(hello)
            .service(get_token)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
