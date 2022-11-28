use axum::{http::StatusCode, response::IntoResponse, Json};
use color_eyre::Report;
use serde::Serialize;
use thiserror::Error;

use crate::db::sql::DbError;

pub type ApiResponse<T> = Result<Json<T>, ApiError>;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("auth")]
    Auth,
    #[error("unknown")]
    Unknown(#[from] Report),
    #[error("db")]
    Db(#[from] DbError),
}

#[derive(Serialize)]
struct ErrorResponse {
    key: &'static str,
}

impl ApiError {
    fn response(&self) -> ErrorResponse {
        let key = match self {
            ApiError::Auth => "auth",
            ApiError::Db(DbError::AlreadyExists { .. }) => "already_exists",
            ApiError::Db(_) | ApiError::Unknown(_) => "unknown",
        };
        ErrorResponse { key }
    }

    fn code(&self) -> StatusCode {
        match self {
            ApiError::Auth => StatusCode::FORBIDDEN,
            ApiError::Db(DbError::AlreadyExists { .. }) => StatusCode::BAD_REQUEST,
            ApiError::Db(_) | ApiError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let code = self.code();
        let response = self.response();

        let mut response = Json(response).into_response();
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
            Err(ApiError::Auth)
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
