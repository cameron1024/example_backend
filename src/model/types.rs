use diesel::{AsExpression, FromSqlRow};
use microtype::microtype;
use schemars::JsonSchema;
use uuid::Uuid;

microtype! {
    #[derive(Debug, Clone, Copy, PartialEq, AsExpression, FromSqlRow, JsonSchema)]
    #[diesel(sql_type = diesel::sql_types::Uuid)]
    pub Uuid {
        UserId
    }

    #[derive(Debug, Clone, PartialEq, AsExpression, FromSqlRow, JsonSchema)]
    #[diesel(sql_type = diesel::sql_types::Text)]
    #[string]
    pub String {
        Email
    }

    #[secret]
    #[string]
    pub String {
        Password,
    }

    #[secret]
    #[string]
    #[derive(AsExpression, FromSqlRow)]
    #[diesel(sql_type = diesel::sql_types::Text)]
    pub String {
        PasswordHash,
    }


}

impl JsonSchema for Password {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

#[cfg(test)]
pub mod mock {
    use microtype::SecretMicrotype;
    use once_cell::sync::Lazy;

    use crate::state::{
        hasher::{BcryptHasher, Hasher},
        random::{mock::MockRandom, Random},
    };

    use super::{Email, Password, PasswordHash, UserId};

    pub static DEFAULT_USER_IDS: Lazy<Vec<UserId>> = Lazy::new(|| {
        let random = MockRandom::new();
        std::iter::from_fn(|| Some(random.user_id()))
            .take(10)
            .collect()
    });
    pub static DEFAULT_USER_ID: Lazy<UserId> = Lazy::new(|| DEFAULT_USER_IDS[0]);
    pub static DEFAULT_EMAIL: Lazy<Email> = Lazy::new(|| Email(String::from("default@email.com")));
    pub static DEFAULT_PASSWORD: Lazy<Password> =
        Lazy::new(|| Password::new("bad password".into()));
    pub static DEFAULT_PASSWORD_HASH: Lazy<PasswordHash> =
        Lazy::new(|| BcryptHasher.hash(&*DEFAULT_PASSWORD).unwrap());
}
