use config::{ Config, ConfigError, Environment };
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub server_addr: String,
    pub port: String,
    pub database_url: String,
    pub db_max_connections: u32,
    pub use_ssl: bool,
}

impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }

    pub fn db_connection_string(self: &Self) -> String {
        let addendum = if self.use_ssl { "requiressl=1" } else { "" };
        format!("{}?{}", self.database_url, addendum)
    }
}
