#[cfg(test)]
use crate::db::mock::MockDb;
use std::fmt::Debug;

use self::{sql::SqlDb, users::UserDao};

pub mod schema;
pub mod sql;
pub mod users;

#[cfg(test)]
pub mod mock;

pub trait Db: UserDao + Send + Sync + Debug {
    #[cfg(test)]
    fn as_mock(&self) -> MockDb;
}

impl Db for SqlDb {
    #[cfg(test)]
    fn as_mock(&self) -> MockDb {
        unimplemented!("this is not a mock db");
    }
}
