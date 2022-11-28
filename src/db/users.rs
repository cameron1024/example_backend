use diesel::{delete, insert_into, ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::{
    db::schema::users,
    model::{
        types::{Email, UserId},
        user::User,
    },
};

use super::sql::{DbError, SqlDb};

#[axum::async_trait]
pub trait UserDao {
    async fn user_by_id(&self, user_id: UserId) -> Result<Option<User>, DbError>;

    async fn user_by_email(&self, email: Email) -> Result<Option<User>, DbError>;

    async fn create_user(&self, user: User) -> Result<(), DbError>;

    async fn delete_user(&self, user_id: UserId) -> Result<(), DbError>;
}

#[axum::async_trait]
impl UserDao for SqlDb {
    async fn user_by_id(&self, user_id: UserId) -> Result<Option<User>, DbError> {
        let user = self
            .exec(move |conn| {
                users::table
                    .filter(users::id.eq(user_id.0))
                    .first(conn)
                    .optional()
            })
            .await?;

        Ok(user)
    }

    async fn user_by_email(&self, email: Email) -> Result<Option<User>, DbError> {
        let user = self
            .exec(move |conn| {
                users::table
                    .filter(users::email.eq(email))
                    .first(conn)
                    .optional()
            })
            .await?;
        Ok(user)
    }

    async fn create_user(&self, user: User) -> Result<(), DbError> {
        let rows_modified = self
            .exec(move |conn| insert_into(users::table).values(user).execute(conn))
            .await?;

        match rows_modified {
            1 => Ok(()),
            n => Err(DbError::RowsModified {
                expected: 1,
                actual: n,
            }),
        }
    }

    async fn delete_user(&self, user_id: UserId) -> Result<(), DbError> {
        self.exec(move |conn| delete(users::table.filter(users::id.eq(user_id))).execute(conn))
            .await?;
        Ok(())
    }
}
