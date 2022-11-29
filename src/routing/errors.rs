use axum::{http::StatusCode, response::IntoResponse, Json};
use schemars::JsonSchema;
use serde::Serialize;
use thiserror::Error;

pub type ApiResponse<T> = Result<Json<T>, Json<ApiError>>;

#[derive(Debug, Error, Serialize, JsonSchema)]
pub enum ApiError {
    #[error("auth")]
    Auth,
    #[error("unknown")]
    Unknown,
    #[error("already in use")]
    AlreadyInUse,
}

impl ApiError {
    fn code(&self) -> StatusCode {
        match self {
            ApiError::Auth => StatusCode::FORBIDDEN,
            ApiError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::AlreadyInUse => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let code = self.code();
        let mut response = Json(self).into_response();
        *response.status_mut() = code;

        response
    }
}

#[cfg(test)]
mod tests {
    use axum::{http::StatusCode, routing::get, Router};
    use axum_test_helper::TestClient;
    use serde_json::{json, Value};

    use crate::routing::errors::{ApiError, ApiResponse};

    #[tokio::test]
    async fn error_response_conforms() {
        async fn handler() -> ApiResponse<()> {
            Err(ApiError::Auth.into())
        }

        let router = Router::new().route("/", get(handler));
        let client = TestClient::new(router);

        let response = client.get("/").send().await;

        assert_eq!(response.status(), StatusCode::FORBIDDEN);
        assert_eq!(
            response.json::<Value>().await,
            json!({
                "key": "auth"
            })
        );
    }
}
