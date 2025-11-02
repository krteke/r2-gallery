use anyhow::Result;
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::{extract::State, Json};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;
use jsonwebtoken::{encode, EncodingKey, Header};
use reqwest::StatusCode;
use secrecy::ExposeSecret;
use secrecy::SecretString;
use std::sync::Arc;

use crate::models::Claims;
use crate::{models::LoginInfo, state::app::AppState};

pub const ACCESS_TOKEN_NAME: &str = "access_token";

pub async fn login_handler(
    jar: CookieJar,
    State(state): State<Arc<AppState>>,
    Json(info): Json<LoginInfo>,
) -> Result<impl IntoResponse, StatusCode> {
    tracing::debug!("Received login attempt for user: {}", info.username);

    let is_valid = is_valid_user(&state, &info);

    if is_valid {
        let token_duration = chrono::Duration::minutes(30);
        let token_exp = (chrono::Utc::now() + token_duration).timestamp();
        let claims = Claims {
            sub: info.username,
            exp: token_exp,
        };

        let token = match encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&state.jwt_secret.expose_secret().as_bytes()),
        ) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!("Error Generating Token {}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };

        let cookie = Cookie::build((ACCESS_TOKEN_NAME, token.clone()))
            .path("/")
            .max_age(time::Duration::seconds(token_duration.num_seconds()))
            .http_only(true)
            .secure(true)
            .same_site(axum_extra::extract::cookie::SameSite::Strict)
            .build();

        let jar_with_cookie = jar.add(cookie);

        Ok((jar_with_cookie, HeaderMap::new(), StatusCode::OK).into_response())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

fn is_valid_user(state: &Arc<AppState>, info: &LoginInfo) -> bool {
    let is_valid_username = verify_username(&state.username, &info.username);
    let is_valid_password = verify_password(&info.password, &state.password);

    match is_valid_password {
        Ok(valid) => return valid && is_valid_username,
        Err(e) => {
            tracing::error!("Error verifying password: {}", e);
            return false;
        }
    }
}

fn verify_username(expected: &SecretString, provided: &str) -> bool {
    use secrecy::ExposeSecret;
    use subtle::ConstantTimeEq;

    let expected_bytes = expected.expose_secret().as_bytes();
    let provided_bytes = provided.as_bytes();

    if expected_bytes.len() != provided_bytes.len() {
        return false;
    }

    let username_eq = expected_bytes.ct_eq(provided_bytes).unwrap_u8() == 1;

    if !username_eq {
        tracing::debug!("Username verification failed");
        return false;
    } else {
        return true;
    }
}

fn verify_password(password: &str, hashed_password: &SecretString) -> Result<bool> {
    use argon2::{password_hash, Argon2, PasswordHash, PasswordVerifier};
    use secrecy::ExposeSecret;

    let password = password.as_bytes();

    let parsed_hash = PasswordHash::new(hashed_password.expose_secret()).map_err(|e| {
        tracing::debug!("{}", e);
        return anyhow::anyhow!("Failed to parse password hash: {}", e);
    })?;

    let verification_result = Argon2::default().verify_password(password, &parsed_hash);

    match verification_result {
        Ok(()) => Ok(true),
        Err(password_hash::Error::Password) => Ok(false),
        Err(e) => Err(anyhow::anyhow!("Password verification failed: {}", e)),
    }
}
