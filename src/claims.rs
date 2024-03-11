use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize)]
pub struct TokenClaims{
    pub sub: Uuid,
    pub iat: usize,
    pub exp: usize,
}