use std::str::FromStr;
use actix_web::{HttpResponse, post, Responder, web};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;
use crate::{Data, middleware};

#[derive(Serialize, Deserialize)]
struct Body {
    name: String,
    tags: String,
    bytes: String,
    ext: String,
}

#[post("/lphoto")]
pub async fn load_photo(body: web::Json<Body>, jwt: middleware::jwt::JwtMiddleware, data: Data) -> impl Responder {
    let uuid_photo = Uuid::new_v4();
    tokio::fs::create_dir_all(format!("./{}/photo/", jwt.user)).await.expect("TODO: panic message");
    let path = format!("./{}/photo/{}-{}", jwt.user, body.name, uuid_photo);
    let query = sqlx::query!("insert into photo (id, path, owner, extension) values ($1, $2,$3,$4)", uuid_photo, path, jwt.user, body.ext).fetch_all(&data.pool).await;
    let bytes = body.bytes.split(",").map(|val|{u8::from_str(&*val).unwrap()}).collect::<Vec<u8>>();
    match query {
        Ok(_) => {
            let fs = tokio::fs::write(path.clone(), bytes).await;
            return match fs {
                Ok(_) => {
                    info!("File {} - saved", path);
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