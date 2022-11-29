use axum::{
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    Extension, TypedHeader,
};
use microtype::SecretMicrotype;

use crate::{routing::errors::ApiError, state::Services};

use super::{
    claims::{Claims, Validated},
    Jwt,
};

type Header = TypedHeader<Authorization<Bearer>>;

#[axum::async_trait]
impl<S> FromRequestParts<S> for Claims<Validated>
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = Header::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::Auth)?;

        let Extension(services): Extension<Services> = Extension::from_request_parts(parts, state)
            .await
            .map_err(|_| ApiError::Unknown)?;

        let jwt = Jwt::new(auth_header.token().to_string());

        let claims = services
            .auth
            .validate_jwt(&jwt)
            .map_err(|_| ApiError::Auth)?;

        Ok(claims)
    }
}
