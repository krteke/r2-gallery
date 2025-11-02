use std::sync::Arc;

use axum::extract::State;
use reqwest::StatusCode;

use crate::{server::middleware::Auth, state::app::AppState};

pub async fn verify_token_handler(
    Auth(claims): Auth,
    State(_state): State<Arc<AppState>>,
) -> StatusCode {
    tracing::debug!("Token verification successful for user: {}", claims.sub);

    StatusCode::OK
}
