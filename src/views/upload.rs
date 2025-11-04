use dioxus::prelude::*;

use crate::components::AuthGuard;

#[component]
pub fn Upload() -> Element {
    rsx! {
        AuthGuard {
            div { "Upload" }
        }
    }
}
