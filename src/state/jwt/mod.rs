use std::sync::Arc;

use crate::{config::Config, model::user::User};

use self::claims::{Claims, Unvalidated, Validated};
use jsonwebtoken::{decode, encode, Header, TokenData, Validation};
use microtype::{secrecy::ExposeSecret, SecretMicrotype};
use schemars::JsonSchema;

use super::{random::Random, time::Time};

pub mod claims;
mod error;
mod from_request;

pub use error::JwtError;

microtype::microtype! {
    #[secret(serialize)]
    #[string]
    pub String {
        Jwt,
    }
}

impl JsonSchema for Jwt {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

#[derive(Debug)]
pub struct JwtService {
    time: Arc<dyn Time>,
    random: Arc<dyn Random>,
    config: Arc<Config>,
}

impl JwtService {
    pub fn new(time: Arc<dyn Time>, random: Arc<dyn Random>, config: Arc<Config>) -> Self {
        Self {
            time,
            random,
            config,
        }
    }

    #[instrument]
    pub fn validate(&self, jwt: &Jwt) -> Result<Claims<Validated>, JwtError> {
        let mut validation = Validation::default();
        validation.validate_exp = false;
        validation.validate_nbf = false;

        let key = self.config.jwt.key.decoding();
        let token: TokenData<Claims<Unvalidated>> =
            decode(jwt.expose_secret(), key, &validation).map_err(|_| JwtError::InvalidSig)?;

        let now = self.time.now();

        let not_before = token.claims.not_before.as_date_time();
        let expiration = token.claims.expiration.as_date_time();

        if now < not_before {
            return Err(JwtError::TooEarly);
        }

        if now > expiration {
            return Err(JwtError::TooLate);
        }
        Ok(token.claims.insecure_assert_valid())
    }

    pub fn create_jwt(&self, user: User) -> Result<Jwt, JwtError> {
        let claims = self.claims_from_user(user);
        let jwt = encode(&Header::default(), &claims, self.config.jwt.key.encoding())?;
        Ok(Jwt::new(jwt))
    }

    fn claims_from_user(&self, user: User) -> Claims<Validated> {
        let ttl = self.config.jwt.ttl();
        let jwt_id = self.random.uuid().to_string().into();
        let issuer = self.config.hostname.clone().into();
        let now = self.time.now();

        Claims::new(user, ttl, now, jwt_id, issuer)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        config::testing::test_config,
        model::{types::mock::DEFAULT_EMAIL, user::mock::default_user},
        state::{random::mock::MockRandom, time::mock::MockTime},
    };

    use super::*;

    fn make_service() -> JwtService {
        JwtService {
            time: Arc::new(MockTime::default()),
            random: Arc::new(MockRandom::new()),
            config: Arc::new(test_config()),
        }
    }

    #[test]
    fn can_create_and_validate_jwt() {
        let service = make_service();
        let user = default_user();
        let jwt = service.create_jwt(user).unwrap();

        let claims = service.validate(&jwt).unwrap();
        assert_eq!(claims.email.as_str(), DEFAULT_EMAIL.as_str());
    }
}
