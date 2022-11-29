use chrono::Duration;
use serde::Deserialize;

pub use keypair::KeyPair;

use crate::db::sql::DbConfig;

mod io;
mod keypair;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub jwt: JwtConfig,
    pub hostname: String,
    pub db: DbKind,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum DbKind {
    Real(DbConfig),
    #[cfg(test)]
    InMemory,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JwtConfig {
    pub ttl_seconds: i64,
    pub key: KeyPair,
}

impl JwtConfig {
    pub fn ttl(&self) -> Duration {
        Duration::seconds(self.ttl_seconds)
    }
}

#[cfg(test)]
pub mod testing {
    use super::{Config, DbKind, JwtConfig, KeyPair};

    pub fn test_config() -> Config {
        Config {
            hostname: "localhost".into(),
            jwt: JwtConfig {
                ttl_seconds: 1000,
                key: KeyPair::from_secret(b"bad secret"),
            },
            db: DbKind::InMemory,
        }
    }
}
