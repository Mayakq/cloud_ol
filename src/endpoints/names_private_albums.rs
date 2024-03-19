use actix_web::{get, HttpResponse, Responder, web};
use serde_json::json;
use crate::Data;

#[get("/ppalbum")]
pub async fn get_names_private_alb(data: Data) -> impl Responder{
    let names = sqlx::query!("select name from albums where (public = true or public = false)").fetch_all(&data.pool).await.unwrap()
        .iter().map(|value|{value.name.clone()}).collect::<Vec<String>>();
    HttpResponse::Ok().json(json!({"name": names, "length": names.len()}))
}
