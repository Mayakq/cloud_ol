use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize)]
pub struct Body {
    pub username: String,
    pub password: String,
}