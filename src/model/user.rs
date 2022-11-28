use chrono::{DateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};

use crate::db::schema::users;

use super::{
    types::{Email, PasswordHash, UserId},
};

#[derive(Debug, Clone, Selectable, Queryable, Insertable)]
pub struct User {
    pub id: UserId,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
pub mod mock {
    use crate::{
        model::types::mock::{DEFAULT_EMAIL, DEFAULT_PASSWORD_HASH, DEFAULT_USER_ID},
        state::time::mock::DEFAULT_DATE_TIME,
    };

    use super::*;

    pub fn default_user() -> User {
        User {
            id: *DEFAULT_USER_ID,
            email: DEFAULT_EMAIL.clone(),
            password_hash: DEFAULT_PASSWORD_HASH.clone(),
            created_at: *DEFAULT_DATE_TIME,
        }
    }
}
