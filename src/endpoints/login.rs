use actix_web::{get, web::{self, Json}, Responder, HttpResponse};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use argon2::password_hash::PasswordHashString;
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use serde_json::json;
use tracing::info;

use crate::{handlers::response::CustomResponse, models::login::Body, Data};
use crate::claims::TokenClaims;

#[get("log/")]
pub async fn login(data: Data, body: web::Json::<Body>) -> impl Responder {
    let query = sqlx::query!("select id, password from users where login = $1", body.name)
        .fetch_all(&data.pool).await;
    match query {
        Ok(value) => {
            if value.len() == 1 {
                let hash = value[0].password.clone();
                let argon = Argon2::default().verify_password(body.password.as_ref(), &PasswordHash::new(&*hash).unwrap());
                match argon {
                    Ok(_) => {
                        let now = Utc::now();
                        let iat = now.timestamp() as usize;
                        let exp = (now + Duration::try_days(31).unwrap()).timestamp() as usize;
                        let claims = TokenClaims {
                            sub: value[0].id,
                            exp: exp,
                            iat: iat,
                        };
                        let token = jsonwebtoken::encode(
                            &Header::default(),
                            &claims,
                            // todo(Error)
                            &EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_ref()),
                        ).unwrap();
                        let query = sqlx::query!("update users set jswt = $1 where login = $2", token, body.name).execute(&data.pool).await;
                        return HttpResponse::Ok().json(json!({"token": token}));
                    }
                    Err(err) => {
                        info!("User {} authorization error incorrect login. - {}", body.name, err.to_string());
                    }
                }
            }
        }
        Err(err) => {
            info!("User {} authorization incorrect password. - {}", body.name, err.to_string());
        }

    }

    return HttpResponse::Conflict().json(json!({"message": "name or password incorrect"}));

}


