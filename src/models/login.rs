use serde::Deserialize;
use serde::Serialize;


#[derive(Deserialize, Serialize)]
pub struct Body{
    pub name: String,
    pub password: String,
}