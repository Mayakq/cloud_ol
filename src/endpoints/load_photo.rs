use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;
use crate::{Data, middleware};

#[derive(Serialize, Deserialize)]
struct Body {
    name: String,
    tags: Vec<String>,
    bytes: Vec<u8>,
    ext: String,
}

#[post("/lphoto")]
pub async fn load_photo(body: web::Json<Body>, jwt: middleware::jwt::JwtMiddleware, data: Data) -> impl Responder {
    let uuid_photo = Uuid::new_v4();
    tokio::fs::create_dir_all(format!("./{}/photo/", jwt.user)).await.expect("TODO: panic message");
    let path = format!("./{}/photo/{}-{}", jwt.user, body.name, uuid_photo);
    let query = sqlx::query!("insert into photo (id, path, owner, extension) values ($1, $2,$3,$4)", uuid_photo, path, jwt.user, body.ext).fetch_all(&data.pool).await;
    match query {
        Ok(_) => {
            let fs = tokio::fs::write(path, body.bytes.clone()).await;
            return match fs {
                Ok(_) => {
                    HttpResponse::Created()
                }
                Err(err) => {
                    error!("{}", err.to_string());
                    HttpResponse::Conflict()
                }
            };
        }
        Err(err) => {
            error!("{}", err.to_string());
            HttpResponse::Conflict()
        }
    }
}