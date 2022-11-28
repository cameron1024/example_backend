use self::requests::{CreateUserRequest, CreateUserResponse, LoginRequest, LoginResponse};
use super::errors::ApiResponse;
use crate::state::{
    jwt::claims::{Claims, Validated},
    Services,
};
use axum::{extract::State, Json};

pub mod requests;

#[instrument]
pub(super) async fn create_user(
    State(services): State<Services>,
    Json(CreateUserRequest { email, password }): Json<CreateUserRequest>,
) -> ApiResponse<CreateUserResponse> {
    let jwt = services.auth.create_user(email, password).await?;
    Ok(CreateUserResponse { jwt }.into())
}

#[instrument]
pub(super) async fn login(
    State(services): State<Services>,
    Json(LoginRequest { email, password }): Json<LoginRequest>,
) -> ApiResponse<LoginResponse> {
    let jwt = services.auth.login(email, password).await?;
    Ok(LoginResponse { jwt }.into())
}

#[instrument]
pub(super) async fn delete_user(
    State(services): State<Services>,
    claims: Claims<Validated>,
) -> ApiResponse<()> {
    services.auth.delete_user(&claims).await?;
    Ok(Json(()))
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use microtype::secrecy::ExposeSecret;
    use serde_json::{json, Value};

    use crate::{
        model::types::mock::{DEFAULT_EMAIL, DEFAULT_PASSWORD, DEFAULT_USER_ID},
        testing::{default_test_client, test_client_with, test_data::TEST_DATA},
    };

    #[tokio::test]
    async fn create_user_test() {
        let (client, services) = default_test_client();
        let body = json!({
            "email": DEFAULT_EMAIL.clone(),
            "password": DEFAULT_PASSWORD.expose_secret().clone(),
        });
        let resp = client.post("/auth/create-user").json(&body).send().await;

        assert_eq!(resp.status(), StatusCode::OK);
        let resp: Value = resp.json().await;
        assert!(resp.as_object().unwrap().get("jwt").is_some());

        let user = services
            .auth
            .user_with_id(*DEFAULT_USER_ID)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(&user.email, &*DEFAULT_EMAIL);
    }

    #[tokio::test]
    async fn login_test() {
        let (client, _) = test_client_with(TEST_DATA.clone());
        let body = json!({
            "email": DEFAULT_EMAIL.clone(),
            "password": DEFAULT_PASSWORD.expose_secret().clone(),
        });
        let resp = client.post("/auth/login").json(&body).send().await;
        assert_eq!(resp.status(), StatusCode::OK);
        let resp: Value = resp.json().await;

        assert!(matches!(resp, Value::Object(obj) if obj.contains_key("jwt")));
    }
}
