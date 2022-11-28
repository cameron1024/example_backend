use std::sync::Arc;

use crate::{
    db::Db,
    model::{
        types::{Email, Password, UserId},
        user::User,
    },
    routing::errors::ApiError,
};

use super::{
    hasher::Hasher,
    jwt::{
        claims::{Claims, Validated},
        Jwt, JwtError, JwtService,
    },
    random::Random,
    time::Time,
};

#[derive(Debug, Clone)]
pub struct AuthService {
    time: Arc<dyn Time>,
    random: Arc<dyn Random>,
    hasher: Arc<dyn Hasher>,
    jwt: Arc<JwtService>,
    db: Arc<dyn Db>,
}

impl AuthService {
    pub fn new(
        time: Arc<dyn Time>,
        random: Arc<dyn Random>,
        hasher: Arc<dyn Hasher>,
        jwt: Arc<JwtService>,
        db: Arc<dyn Db>,
    ) -> Self {
        Self {
            time,
            random,
            hasher,
            jwt,
            db,
        }
    }

    #[instrument]
    pub async fn login(&self, email: Email, password: Password) -> Result<Jwt, ApiError> {
        let user = self.user_with_email(email).await?.ok_or(ApiError::Auth)?;
        match self.hasher.verify(&password, &user.password_hash) {
            true => Ok(self.jwt.create_jwt(user).map_err(|_| ApiError::Auth)?),
            false => Err(ApiError::Auth),
        }
    }

    #[instrument]
    pub async fn create_user(&self, email: Email, password: Password) -> Result<Jwt, ApiError> {
        let id = self.random.user_id();
        let created_at = self.time.now();
        let password_hash = self.hasher.hash(&password).map_err(|_| ApiError::Auth)?;

        let user = User {
            id,
            email,
            password_hash,
            created_at,
        };

        self.db.create_user(user.clone()).await?;

        let jwt = self.jwt.create_jwt(user).map_err(|_| ApiError::Auth)?;

        Ok(jwt)
    }

    #[instrument]
    pub fn validate_jwt(&self, jwt: &Jwt) -> Result<Claims<Validated>, JwtError> {
        self.jwt.validate(jwt)
    }

    #[allow(dead_code)]
    #[instrument]
    pub async fn user_with_id(&self, user_id: UserId) -> Result<Option<User>, ApiError> {
        Ok(self.db.user_by_id(user_id).await?)
    }

    #[instrument]
    async fn user_with_email(&self, email: Email) -> Result<Option<User>, ApiError> {
        Ok(self.db.user_by_email(email).await?)
    }

    #[instrument]
    pub async fn delete_user(&self, claims: &Claims<Validated>) -> Result<(), ApiError> {
        self.db.delete_user(claims.subject).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        model::types::mock::{DEFAULT_EMAIL, DEFAULT_PASSWORD},
        state::Services,
        testing::{test_data::TEST_DATA, test_services, test_services_with},
    };

    #[tokio::test]
    async fn can_create_user() {
        let Services { auth, .. } = test_services();

        let jwt = auth
            .create_user(DEFAULT_EMAIL.clone(), DEFAULT_PASSWORD.clone())
            .await
            .unwrap();

        let claims = auth.jwt.validate(&jwt).unwrap();

        assert_eq!(claims.email, DEFAULT_EMAIL.clone());

        let user = auth
            .user_with_email(DEFAULT_EMAIL.clone())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(user.email, claims.email)
    }

    #[tokio::test]
    async fn can_login() {
        let Services { auth, .. } = test_services_with(TEST_DATA.clone());
        let jwt = auth
            .login(DEFAULT_EMAIL.clone(), DEFAULT_PASSWORD.clone())
            .await
            .unwrap();

        let claims = auth.jwt.validate(&jwt).unwrap();
        assert_eq!(claims.email, DEFAULT_EMAIL.clone());
    }
}
