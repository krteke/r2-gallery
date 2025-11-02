use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use axum_extra::extract::cookie::Cookie;
use time::Duration;

use crate::server::handlers::ACCESS_TOKEN_NAME;

fn clear_cookie_header() -> HeaderValue {
    let expired_cookie = Cookie::build((ACCESS_TOKEN_NAME, ""))
        .path("/")
        .max_age(Duration::seconds(0))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .build();

    // TODO: remove unwrap()
    expired_cookie.to_string().parse().unwrap()
}

pub async fn cookie_cleanup_middleware(request: Request, next: Next) -> Response {
    let mut response = next.run(request).await;

    let header_value = clear_cookie_header();
    todo!()
}
