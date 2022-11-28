use std::sync::{Arc, Mutex};

use crate::model::{
    types::{Email, UserId},
    user::User,
};

use super::{sql::DbError, users::UserDao, Db};

#[derive(Debug, Clone)]
pub struct MockDb {
    pub users: Arc<Mutex<Vec<User>>>,
}

impl MockDb {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(vec![])),
        }
    }
}

impl Db for MockDb {
    fn as_mock(&self) -> MockDb {
        self.clone()
    }
}

#[axum::async_trait]
impl UserDao for MockDb {
    async fn user_by_id(&self, user_id: UserId) -> Result<Option<User>, DbError> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|&u| u.id == user_id).cloned();
        Ok(user)
    }

    async fn user_by_email(&self, email: Email) -> Result<Option<User>, DbError> {
        let users = self.users.lock().unwrap();
        let user = users.iter().find(|&u| u.email == email).cloned();
        Ok(user)
    }

    async fn create_user(&self, user: User) -> Result<(), DbError> {
        let mut users = self.users.lock().unwrap();
        if users.iter().any(|u| u.id == user.id) {
            return Err(DbError::AlreadyExists {
                table: Some("users".into()),
                col: Some("id".into()),
            });
        }

        users.push(user);

        Ok(())
    }

    async fn delete_user(&self, user_id: UserId) -> Result<(), DbError> {
        let mut users = self.users.lock().unwrap();
        users.retain(|user| user.id != user_id);
        Ok(())
    }
}
