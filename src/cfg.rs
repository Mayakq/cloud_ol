use dotenv::dotenv;
use tracing::{info, warn};

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        
        dotenv().ok();
        let database_url = std::fs::read_to_string("./cfg").expect("Error parse string database");
        info!("Database url - {}", database_url);
        
        return Config {
            database_url,
        };
    }
}
