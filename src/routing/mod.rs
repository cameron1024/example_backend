use std::sync::Arc;

use aide::axum::routing::{get, post};
use aide::axum::ApiRouter;
use aide::{axum::IntoApiResponse, openapi::OpenApi};
use axum::{Extension, Json, Router};

use crate::state::Services;

mod auth;
mod health;

pub mod errors;

pub fn attach_routes(router: ApiRouter<Services>) -> ApiRouter<Services> {
    aide::gen::infer_responses(false);

    router
        // .api_route("/auth/create-user", post(auth::create_user))
        // .api_route("/auth/login", post(auth::login))
        .api_route("/auth/delete-user", get(auth::delete_user))
        // .api_route("/health", get(health::health))
        // .route("/docs", get(docs))
}

async fn docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api)
}
