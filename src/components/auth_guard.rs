use dioxus::prelude::*;

use crate::{components::Loading, server::Route, utils::get_browser_origin};

#[derive(Clone, Copy, PartialEq)]
enum AuthStatus {
    Loading,
    Authenticated,
    Unauthenticated,
}

#[component]
pub fn AuthGuard(children: Element) -> Element {
    let mut auth_status = use_signal(|| AuthStatus::Loading);
    let navigator = use_navigator();

    use_effect(move || {
        auth_status.set(AuthStatus::Loading);

        spawn(async move {
            if perform_auth_check().await {
                auth_status.set(AuthStatus::Authenticated);
            } else {
                auth_status.set(AuthStatus::Unauthenticated);
            }
        });
    });

    match auth_status() {
        AuthStatus::Loading => {
            return rsx! {
                Loading {}
            };
        }
        AuthStatus::Authenticated => {
            return rsx! {
                {children}
            }
        }
        AuthStatus::Unauthenticated => {
            navigator.push(Route::Login {});
            return rsx! {
                div {}
            };
        }
    }
}

async fn perform_auth_check() -> bool {
    let client = reqwest::Client::new();
    let base_url = get_browser_origin().unwrap_or_else(|| "".to_string());
    let url = format!("{}/api/verify", base_url);

    let response = client.get(&url).send().await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                tracing::info!("User verified");

                true
            } else if res.status() == 401 {
                false
            } else {
                tracing::error!("Verification failed with status: {}", res.status());

                false
            }
        }
        Err(e) => {
            tracing::error!("Failed to verify user: {}", e);

            false
        }
    }
}
