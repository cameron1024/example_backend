use super::errors::ApiResponse;

#[instrument]
pub(super) async fn health() -> String {
    "OK".into()
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use serde_json::{json, Value};

    use crate::testing::default_test_client;

    #[tokio::test]
    async fn health_test() {
        let (client, _) = default_test_client();
        let resp = client.get("/health").send().await;

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(resp.json::<Value>().await, json!("OK"));
    }
}
