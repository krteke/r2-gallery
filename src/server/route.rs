#[cfg(feature = "server")]

pub mod client {
    use crate::views::*;
    use dioxus::prelude::*;

    #[derive(Debug, Clone, PartialEq, Routable)]
    #[rustfmt::skip]
    pub enum Route {
        #[route("/")]
        Home {},
        #[route("/login")]
        Login {},

        #[route("/:..path")]
        NotFound { path: Vec<String> },
    }
}

#[cfg(feature = "server")]
pub mod server {
    use axum::{routing::post, Router};
    use dioxus::server::FullstackState;
    use std::sync::Arc;

    use crate::{server::login_handler, state::app::AppState};

    pub fn router(app_state: Arc<AppState>) -> Router<FullstackState> {
        use axum::routing::get;

        use crate::server::{auth_middleware, verify_token_handler};

        let protected_route = axum::Router::new()
            .route("/verify", get(verify_token_handler))
            .route_layer(axum::middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            ));

        axum::Router::new()
            .route("/api/login", post(login_handler))
            .nest("/api", protected_route)
            .with_state(app_state)
    }
}
