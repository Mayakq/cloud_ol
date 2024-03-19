use actix_cors::Cors;
use actix_web::{App, get, HttpServer, Responder, web};
use env_logger::Env;
use tracing::info;
use tracing::log::Level;
use tracing_actix_web::TracingLogger;
use crate::{database::AppState, endpoints::registration::registration};
use crate::endpoints::load_photo::load_photo;
use crate::endpoints::login::login;
use crate::endpoints::names_private_albums::get_names_private_alb;

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
    let app_state = Data::new(AppState::init(&cfg).await);
    env_logger::init_from_env(Env::default().default_filter_or(Level::Trace.as_str()));
    info!("Application started ♥");
    let _ = HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            
            .configure(config)
    })
        .bind(("localhost", 8080))
        .unwrap()
        .run()
        .await.unwrap();
}
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(healthy)
        .service(registration)
        .service(login)
        .service(load_photo)
        .service(get_names_private_alb);

    conf.service(scope);
}
#[get("/hello")]
async fn healthy(info: middleware::jwt::JwtMiddleware) -> impl Responder {
    return info.user.to_string()
}