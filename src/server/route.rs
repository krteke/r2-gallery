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
