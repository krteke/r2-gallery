use dioxus::fullstack::AsStatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Clone)]
pub enum AppError {
    #[cfg(feature = "server")]
    #[error(transparent)]
    ServerError(#[from] dioxus::server::ServerFnError),
}

pub type Result<T> = std::result::Result<T, AppError>;
