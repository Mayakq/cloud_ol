use crate::database::AppState;
use crate::handlers::response::CustomResponse;
use crate::models::registration::Body;

use actix_web::{post, web, Responder};
use actix_web::http::StatusCode;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use tracing::log::error;

#[post("reg/")]
pub async fn registration(body: web::Json<Body>, data: web::Data<AppState>) -> impl Responder {
    let query = sqlx::query!("select login from users where login = $1", body.username)
        .fetch_all(&data.pool)
        .await
        .unwrap();
    if query.len() >= 1 {
        return CustomResponse::init(
            StatusCode::ACCEPTED,
            "login or password already using".to_string(),
        );
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(body.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let query = sqlx::query!(
            "insert into users (login, password) values ($1, $2) RETURNING login",
            body.username,
            hashed_password
        )
            .fetch_one(&data.pool)
            .await;
        match query {
            Ok(_) => { return CustomResponse::init(StatusCode::CREATED, "success".to_string()); }
            Err(err) => {
                error!("Error insert into users - {}", err);
                return CustomResponse::init(StatusCode::SERVICE_UNAVAILABLE, "fail".to_string());
            }
        }
    }
}
