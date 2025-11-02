use std::sync::Arc;

use axum::extract::{FromRequestParts, Request};
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};
use reqwest::StatusCode;
use secrecy::ExposeSecret;

use crate::server::handlers::ACCESS_TOKEN_NAME;
use crate::{models::Claims, state::app::AppState};

pub struct Auth(pub Claims);

impl FromRequestParts<Arc<AppState>> for Auth {
    type Rejection = StatusCode;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        tracing::debug!("Auth Middleware");

        let jar = CookieJar::from_request_parts(parts, state)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let token = jar
            .get(ACCESS_TOKEN_NAME)
            .map(|cookie| cookie.value().to_owned())
            .ok_or_else(|| StatusCode::UNAUTHORIZED)?;

        let secret = state.jwt_secret.expose_secret().as_bytes();
        let decoding_key = DecodingKey::from_secret(secret);
        let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

        let token_data = decode::<Claims>(token, &decoding_key, &validation).map_err(|e| {
            tracing::debug!("JWT Decode/Validation Failed: {}", e);

            StatusCode::UNAUTHORIZED
        })?;

        Ok(Auth(token_data.claims))
    }
}

pub async fn auth_middleware(
    Auth(_clains): Auth,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    Ok(next.run(request).await)
}
