use actix_web::{App, get, HttpServer, Responder, web};
use actix_web::web::ServiceConfig;
use env_logger::Env;
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::PgPool;
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
#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let cfg = cfg::Config::init();
    //let app_state = web::Data::new(AppState::init(&cfg).await);
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("Application started ♥");
    let state = web::Data::new(AppState { pool });
    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(app_state.clone())
            .configure(config)
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await.expect("TODO: panic message");
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/")
                .wrap(TracingLogger::default())
                .app_data(state)
                .configure(config)
        );
    };

    Ok(config.into())
}
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(healthy)
        .service(registration)
        .service(login);

    conf.service(scope);
}
#[get("/hello")]
async fn healthy(info: middleware::jwt::JwtMiddleware) -> impl Responder {
    return info.user.to_string()
}