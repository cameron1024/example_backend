use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    model::types::{Email, Password},
    state::jwt::Jwt,
};

static_assertions::assert_impl_all!(Result<CreateUserRequest, String>: JsonSchema);

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct CreateUserRequest {
    pub email: Email,
    pub password: Password,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct CreateUserResponse {
    pub jwt: Jwt,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub struct LoginRequest {
    pub email: Email,
    pub password: Password,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct LoginResponse {
    pub jwt: Jwt,
}
#[cfg(test)]
mod tests {
    use crate::model::types::mock::{DEFAULT_EMAIL, DEFAULT_PASSWORD};

    use super::*;
    use microtype::{secrecy::ExposeSecret, SecretMicrotype};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn create_user_request_test() {
        let request = json!({
            "email": DEFAULT_EMAIL.clone(),
            "password": DEFAULT_PASSWORD.expose_secret().clone(),
        });

        from_value::<CreateUserRequest>(request).unwrap();
    }

    #[test]
    fn create_user_response_test() {
        let response = CreateUserResponse {
            jwt: Jwt::new("foo".to_string()),
        };

        assert_eq!(to_value(response).unwrap(), json!({"jwt": "foo"}));
    }

    #[test]
    fn login_request_test() {
        let request = json!({
            "email": DEFAULT_EMAIL.clone(),
            "password": DEFAULT_PASSWORD.expose_secret().clone(),
        });

        from_value::<LoginRequest>(request).unwrap();
    }

    #[test]
    fn login_response_test() {
        let response = LoginResponse {
            jwt: Jwt::new("foo".to_string()),
        };

        assert_eq!(to_value(response).unwrap(), json!({"jwt": "foo"}));
    }
}
