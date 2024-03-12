use actix_web::{App, get, HttpServer, Responder, web};
use env_logger::Env;
use tracing::info;
use tracing_actix_web::TracingLogger;
use crate::{database::AppState, endpoints::registration::registration};
use crate::endpoints::login::login;

mod handlers;
mod cfg;
mod database;
mod claims;
mod endpoints;
mod middleware;
mod models;

pub type Data = web::Data<AppState>;
#[actix_web::main]
async fn main() {
    
    let cfg = cfg::Config::init();
    let app_state = web::Data::new(AppState::init(&cfg).await);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Application started ♥");
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            .service(healthy)
            .service(registration)
            .service(login)
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await.expect("TODO: panic message");
}
#[get("/hello")]
async fn healthy() -> impl Responder {
    return "hello world"
}