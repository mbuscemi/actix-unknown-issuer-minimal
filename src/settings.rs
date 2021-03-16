use config::{ Config, ConfigError, Environment };
use serde::Deserialize;
use deadpool_postgres::Config as DeadpoolConfig;

#[derive(Deserialize)]
pub struct Settings {
    pub server_addr: String,
    pub port: String,
    pub pg: DeadpoolConfig,
    pub use_ssl: bool,
    pub db_ca_cert: Option<String>,
    pub use_rustls_root_store: bool,
    pub add_webpki_roots: bool,
    pub use_custom_cert_resolver: bool,
}

impl Settings {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = Config::new();
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }
}
