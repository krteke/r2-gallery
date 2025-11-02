#[cfg(feature = "server")]
mod common;
#[cfg(feature = "server")]
mod handlers;
#[cfg(feature = "server")]
mod middleware;
mod route;

#[cfg(feature = "server")]
pub use handlers::login_handler;
#[cfg(feature = "server")]
pub use handlers::verify_token_handler;
#[cfg(feature = "server")]
pub use middleware::auth_middleware;
pub use route::client::Route;
#[cfg(feature = "server")]
pub use route::server::router;
