mod controller;
mod data;
mod schema;
mod utils;

use crate::controller::user_controller::post_user;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use tracing_actix_web::TracingLogger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();
    HttpServer::new(|| App::new().wrap(TracingLogger::default()).service(post_user))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
