use actix_web::{get, HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::Data;
#[derive(Serialize, Deserialize)]
pub struct Body{
    user: String
}
#[get("/paphoto")]
pub async fn get_path_photos(data: Data, body: web::Json<Body>) -> impl Responder{
    let paths = sqlx::query!("select path from photo where (public = true and owner = (select id from users where login = $1))", body.user.clone()).fetch_all(&data.pool).await.unwrap()
        .iter().map(|value|{value.path.clone()}).collect::<Vec<String>>();
    HttpResponse::Ok().json(json!({"paths": paths, "length": paths.len()}))
}
