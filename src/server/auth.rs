#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use anyhow::Result;
#[cfg(feature = "server")]
use axum::{Json, extract::State, response::IntoResponse};
#[cfg(feature = "server")]
use axum_extra::extract::CookieJar;
use dioxus::prelude::*;
#[cfg(feature = "server")]
use secrecy::SecretString;

#[cfg(feature = "server")]
use crate::{models::LoginInfo, state::AppState};

#[post("/api/auth")]
pub async fn auth() -> Result<String> {
    Ok("Auth".to_string())
}

#[cfg(feature = "server")]
pub async fn login_handler(jar: CookieJar, State(state): State<Arc<AppState>>, Json(json): Json<LoginInfo>) -> impl IntoResponse {
    use axum::Json;
    use secrecy::ExposeSecret;
    use serde_json::json;

    tracing::debug!("Received login attempt for user: {}", json.username);

    Json(json!({
        "username": json.username,
        "password": json.password,
    }))
}

#[cfg(feature = "server")]
fn verify_password(password:&str,hashed_password: SecretString) -> Result<bool>  {
    todo!()
}