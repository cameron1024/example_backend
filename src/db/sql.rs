use color_eyre::Result;
use diesel::{
    pg::Pg,
    query_builder::QueryFragment,
    query_dsl::LoadQuery,
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection, QueryResult,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub name: String,
    pub username: String,
    pub password: Option<String>,
}

#[derive(Debug)]
pub struct SqlDb {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl SqlDb {
    pub fn connect(
        DbConfig {
            host,
            name,
            username,
            password,
        }: DbConfig,
    ) -> Result<Self> {
        let separator = match password {
            Some(_) => ":",
            None => "",
        };
        let url = format!(
            "postgres://{}{}{}@{}/{}",
            username,
            separator,
            password.unwrap_or_default(),
            host,
            name
        );

        let manager = ConnectionManager::new(&url);
        let pool = Pool::builder().build(manager)?;

        Ok(Self { pool })
    }

    pub async fn exec<T, F>(&self, f: F) -> Result<T, DbError>
    where
        T: Send + 'static,
        F: Send
            + 'static
            + FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> QueryResult<T>,
    {
        let mut conn = self.pool.get()?;
        let result = tokio::task::spawn_blocking(move || f(&mut conn)).await??;

        Ok(result)
    }
}

pub(super) trait DbQuery<'a, T>
where
    Self: LoadQuery<'a, PgConnection, T> + QueryFragment<Pg> + Send + 'a,
{
    fn sql_string(&self) -> String {
        let debug = diesel::debug_query(self);
        format!("{debug}")
    }
}

impl<'a, T, S> DbQuery<'a, T> for S where
    S: LoadQuery<'a, PgConnection, T> + QueryFragment<Pg> + Send + 'a
{
}

#[derive(Debug, Error)]
pub enum DbError {
    #[error("db error: {0}")]
    Db(diesel::result::Error), // no #[from] here, since we want to capture some diesel errors
    #[error("pool error: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),
    #[error("error joining thread: {0}")]
    Join(#[from] tokio::task::JoinError),
    #[error("entity already exists")]
    AlreadyExists {
        table: Option<String>,
        col: Option<String>,
    },
    #[error("rows modified, expected: {expected}, actual: {actual}")]
    RowsModified { expected: usize, actual: usize },
}

mod diesel_impl {
    use diesel::result::{DatabaseErrorKind, Error};

    use super::DbError;

    impl From<Error> for DbError {
        fn from(e: Error) -> Self {
            match e {
                Error::DatabaseError(DatabaseErrorKind::UniqueViolation, info) => {
                    Self::AlreadyExists {
                        table: info.table_name().map(From::from),
                        col: info.column_name().map(From::from),
                    }
                }
                e => Self::Db(e),
            }
        }
    }
}
