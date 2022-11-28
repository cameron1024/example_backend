use std::sync::Arc;

use color_eyre::Result;
use static_assertions::assert_impl_all;

#[cfg(test)]
use crate::db::mock::MockDb;
use crate::{
    config::{Config, DbKind},
    db::{sql::SqlDb, Db},
};

use self::{
    auth::AuthService,
    hasher::{BcryptHasher, Hasher},
    jwt::JwtService,
    random::{Random, SystemRandom},
    time::{SystemTime, Time},
};

pub mod auth;
pub mod hasher;
pub mod jwt;
pub mod random;
pub mod time;

pub struct Dependencies {
    pub time: Arc<dyn Time>,
    pub random: Arc<dyn Random>,
    pub hasher: Arc<dyn Hasher>,
    pub config: Arc<Config>,
}

impl Dependencies {
    pub fn load() -> Result<Self> {
        let config = Config::from_env()?;

        let deps = Self {
            time: Arc::new(SystemTime),
            random: Arc::new(SystemRandom),
            hasher: Arc::new(BcryptHasher),
            config: Arc::new(config),
        };

        Ok(deps)
    }
}

pub fn make_services(
    Dependencies {
        time,
        random,
        hasher,
        config,
    }: Dependencies,
) -> Result<Services> {
    let db: Arc<dyn Db> = match &config.db {
        DbKind::Real(config) => Arc::new(SqlDb::connect(config.clone())?),
        #[cfg(test)]
        DbKind::InMemory => Arc::new(MockDb::new()),
    };

    let jwt = JwtService::new(time.clone(), random.clone(), config);
    let jwt = Arc::new(jwt);

    let auth = AuthService::new(time, random, hasher, jwt, db.clone());

    Ok(Services {
        auth,
        #[cfg(test)]
        db,
    })
}

#[derive(Debug, Clone)]
pub struct Services {
    pub auth: AuthService,
    #[cfg(test)]
    pub db: Arc<dyn Db>,
}

assert_impl_all!(Services: Send, Sync);
