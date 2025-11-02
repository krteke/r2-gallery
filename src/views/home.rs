use dioxus::prelude::*;

use crate::{server::Route, utils::get_browser_origin};

#[component]
pub fn Home() -> Element {
    let navigator = use_navigator();
    let mut is_verifying = use_signal(|| true);

    use_effect(move || {
        is_verifying.set(true);

        spawn(async move {
            let client = reqwest::Client::new();
            let base_url = get_browser_origin().unwrap_or_else(|| "".to_string());
            let url = format!("{}/api/verify", base_url);

            let response = client.get(&url).send().await;

            match response {
                Ok(response) => {
                    if response.status() == 200 {
                        tracing::info!("User verified");
                    } else if response.status() == 401 {
                        tracing::info!("User not verified");

                        navigator.push(Route::Login {});
                    }
                }

                Err(e) => {
                    tracing::error!("Failed to verify user: {}", e);

                    navigator.push(Route::Login {});
                }
            }

            is_verifying.set(false);
        });
    });

    if is_verifying() {
        return rsx! {
            div {}
        };
    }

    rsx! {
        div { "Welcome!" }
    }
}
