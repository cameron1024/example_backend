use std::fmt::Debug;

use color_eyre::Result;
use microtype::{secrecy::ExposeSecret, SecretMicrotype};

use crate::model::types::{Password, PasswordHash};

pub trait Hasher: Debug + Send + Sync + 'static {
    fn hash(&self, password: &Password) -> Result<PasswordHash>;

    fn verify(&self, password: &Password, hash: &PasswordHash) -> bool;
}

#[cfg(not(test))]
const COST: u32 = bcrypt::DEFAULT_COST;

#[cfg(test)]
const COST: u32 = 4; // min cost

#[derive(Debug, Clone)]
pub struct BcryptHasher;

impl Hasher for BcryptHasher {
    fn hash(&self, password: &Password) -> Result<PasswordHash> {
        let hash = bcrypt::hash(password.expose_secret(), COST)?;
        Ok(PasswordHash::new(hash))
    }

    fn verify(&self, password: &Password, hash: &PasswordHash) -> bool {
        let result = bcrypt::verify(password.expose_secret(), hash.expose_secret());
        matches!(result, Ok(true))
    }
}

#[cfg(test)]
mod noop {

    use super::*;

    #[derive(Debug, Clone)]
    pub struct NoopHasher;

    impl Hasher for NoopHasher {
        fn hash(&self, password: &Password) -> Result<PasswordHash> {
            Ok(PasswordHash::new(password.expose_secret().clone()))
        }

        fn verify(&self, password: &Password, hash: &PasswordHash) -> bool {
            password.expose_secret() == hash.expose_secret()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hasher_works() {
        let hasher = BcryptHasher;

        let password = Password::new("123".into());
        let hash = hasher.hash(&password).unwrap();
        assert!(hasher.verify(&password, &hash));

        assert!(!hasher.verify(&password, &PasswordHash::new("something".into())));
    }
}
