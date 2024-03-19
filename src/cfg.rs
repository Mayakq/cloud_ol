use dotenv::dotenv;

pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn init() -> Config {
        dotenv().ok();
        let database_url = "postgresql://postgres:postgres@localhost:6500/postgres?schema=public".to_string();
        return Config {
            database_url,
        };
    }
}
