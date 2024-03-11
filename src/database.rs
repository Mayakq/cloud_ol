use std::process;
use sqlx::{Error, PgPool, Pool, Postgres};
use tracing::error;
use crate::cfg::Config;

#[derive(Clone)]
pub struct AppState {
    db_connect: Pool<Postgres>,
}

impl AppState {
    pub async fn init(cfg: &Config) -> AppState {
        let pool = PgPool::connect(&*cfg.database_url).await;
        match pool {
            Ok(pool) => {
                AppState {
                    db_connect: pool
                }
            }
            Err(err) => {
                error!("Failed connection to the database | {}", err);
                process::exit(1);
            }
        }
    }
}