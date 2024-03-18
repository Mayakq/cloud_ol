// use actix_web::{get, HttpRequest, HttpResponse, Responder};
// use actix_web::http::StatusCode;
// 
// #[get("logout/")]
// pub async fn logout(req: &HttpRequest) -> impl Responder {
//     let token = req.headers()
//         .get(actix_web::http::header::AUTHORIZATION)
//         .map(|h| h.to_str().unwrap().split_at(7).1.to_string());
//     HttpResponse::Ok()
// }