use axum::{
    routing::{get, post},
    Router,
};

use crate::state::Services;

mod auth;
mod health;

pub mod errors;

pub fn attach_routes(router: Router<Services>) -> Router<Services> {
    let auth = router
        .clone()
        .route("/create-user", post(auth::create_user))
        .route("/login", post(auth::login))
        .route("/delete-user", post(auth::delete_user));

    router
        .route("/health", get(health::health))
        .nest("/auth", auth)
}
