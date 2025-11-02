use dioxus::prelude::*;

#[component]
pub fn NotFound(path: Vec<String>) -> Element {
    rsx! {
        main { class: "flex flex-col items-center justify-center h-dvh p-8",
            h1 { class: "mb-4 text-4xl font-bold", "404 Not Found" }
            p { class: "text-2xl text-gray-500", "The page you are looking for does not exist." }
        }
    }
}
