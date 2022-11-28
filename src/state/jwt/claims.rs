use std::marker::PhantomData;

use chrono::{DateTime, Duration, TimeZone, Utc};
use microtype::microtype;
use serde::{Deserialize, Serialize};

use crate::model::{
    types::{Email, UserId},
    user::User,
};

#[derive(Debug)]
pub struct Validated;
#[derive(Debug)]
pub struct Unvalidated;

trait Untrusted {}
impl Untrusted for Unvalidated {}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(bound(deserialize = "Validity: Untrusted"))]
pub struct Claims<Validity> {
    pub _marker: PhantomData<Validity>,

    #[serde(rename = "iss")]
    pub issuer: Issuer,
    #[serde(rename = "sub")]
    pub subject: UserId,
    #[serde(rename = "exp")]
    pub expiration: Expiration,
    #[serde(rename = "nbf")]
    pub not_before: NotBefore,
    #[serde(rename = "iat")]
    pub issued_at: IssuedAt,
    #[serde(rename = "jti")]
    pub jwt_id: JwtID,

    pub email: Email,
}

impl Claims<Validated> {
    /// Create a new set of claims
    ///
    /// These claims are `Validated` by default, since we always trust ourselves
    pub(super) fn new(
        User { id, email, .. }: User,
        ttl: Duration,
        now: DateTime<Utc>,
        jwt_id: JwtID,
        issuer: Issuer,
    ) -> Self {
        let expiration = now + ttl;
        let expiration = expiration.timestamp().into();
        let issued_at = now.timestamp().into();
        let not_before = now.timestamp().into();

        Self {
            _marker: PhantomData,
            issuer,
            subject: id,
            expiration,
            issued_at,
            not_before,
            jwt_id,
            email,
        }
    }
}

impl Claims<Unvalidated> {
    /// Assert that these claims are valid
    ///
    /// This is the only mechanism to create a `Claims<Validated>`, and should only be used after
    /// the JWT has been checked for validity
    pub(super) fn insecure_assert_valid(self) -> Claims<Validated> {
        let Claims {
            _marker,
            issuer,
            subject,
            expiration,
            not_before,
            issued_at,
            jwt_id,
            email,
        } = self;
        Claims {
            _marker: PhantomData,
            issuer,
            subject,
            expiration,
            not_before,
            issued_at,
            jwt_id,
            email,
        }
    }
}

microtype! {
    #[derive(Debug, Clone, PartialEq)]
    #[string]
    pub String {
        Issuer,
        Subject,
        JwtID,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub i64 {
        Expiration,
        NotBefore,
        IssuedAt,
    }
}

macro_rules! date_time {
    ($t:ty) => {
        impl $t {
            #[allow(unused)]
            pub fn as_date_time(self) -> DateTime<Utc> {
                Utc.timestamp(self.0, 0)
            }
        }
    };
}

date_time!(Expiration);
date_time!(NotBefore);
date_time!(IssuedAt);

#[cfg(test)]
mod tests {
    use serde_json::from_str;

    use super::*;

    #[test]
    fn can_deserialize_untrusted() {
        let _: Result<Claims<Unvalidated>, _> = from_str("");
    }
}
