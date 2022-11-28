use std::fmt::Debug;

use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::{de::Visitor, Deserialize};

#[derive(Clone)]
pub struct KeyPair {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl KeyPair {
    pub fn decoding(&self) -> &DecodingKey {
        &self.decoding
    }

    pub fn encoding(&self) -> &EncodingKey {
        &self.encoding
    }

    pub fn from_secret(bytes: &[u8]) -> Self {
        let encoding = EncodingKey::from_secret(bytes);
        let decoding = DecodingKey::from_secret(bytes);

        KeyPair { encoding, decoding }
    }
}

impl Debug for KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("KeyPair(REDACTED)")
    }
}

impl<'de> Deserialize<'de> for KeyPair {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl Visitor<'_> for V {
            type Value = KeyPair;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing a JWT secret")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(KeyPair::from_secret(v.as_bytes()))
            }
        }

        deserializer.deserialize_str(V)
    }
}

#[cfg(test)]
mod tests {
    use jsonwebtoken::{decode, encode, Header, Validation};
    use serde_json::{json, Value};

    use super::*;

    #[test]
    fn can_deserialize() {
        let claims = json!({ "exp": i64::MAX });
        let KeyPair { encoding, decoding } = serde_json::from_str("\"secret\"").unwrap();
        let jwt = encode(&Header::default(), &claims, &encoding).unwrap();
        let claims_again: Value = decode(&jwt, &decoding, &Validation::default())
            .unwrap()
            .claims;

        assert_eq!(claims, claims_again);
    }
}
