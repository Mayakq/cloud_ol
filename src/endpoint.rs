use crate::claims::TokenClaims;
use crate::database::AppState;
use crate::Data;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{post, web, FromRequest, HttpMessage, HttpRequest, HttpResponse, Responder};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use http::StatusCode;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::fmt;
use std::fmt::Display;
use std::future::{ready, Ready};
use actix_web::body::BoxBody;
use actix_web::middleware::ErrorHandlerResponse::Response as OtherResponse;

#[derive(Deserialize, Serialize)]
struct Body {
    username: String,
    password: String,
}




#[post("reg/")]
pub async fn registration(body: web::Json<Body>, data: web::Data<AppState>) -> impl Responder {
    let query = sqlx::query!("select name from users where name = $1", body.username)
        .fetch_all(&data.pool)
        .await.unwrap();
    if query.len() >= 1 {
        return CustomResponse::init(StatusCode::ACCEPTED, "login or password already using".to_string());
    } else {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = Argon2::default()
            .hash_password(body.password.as_bytes(), &salt)
            .unwrap()
            .to_string();
        let query = sqlx::query!("insert into users (name, password) values ($1, $2)", body.username, hashed_password).fetch_one(&data.pool).await;
        if let Ok(_) = query {
            return CustomResponse::init(StatusCode::CREATED, "success".to_string());
        } else {
            return CustomResponse::init(StatusCode::SERVICE_UNAVAILABLE, "fail".to_string());
        }
    }
}

pub struct CustomResponse {
    pub status: StatusCode,
    pub message: String,
}

impl CustomResponse {
    pub fn init(status: StatusCode, message: String) -> Self {
        Self {
            status,
            message,
        }
    }
}

impl Responder for CustomResponse {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::build(self.status).body(self.message)
    }
}

