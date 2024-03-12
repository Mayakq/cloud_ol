use actix_web::{body::BoxBody, HttpRequest, HttpResponse, Responder};
use actix_web::http::StatusCode;

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