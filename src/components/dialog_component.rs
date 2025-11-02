use dioxus::prelude::*;

use crate::components::dialog::*;

#[derive(Debug, Clone, PartialEq, Props)]
pub struct DialogProps {
    open: Signal<bool>,
    title: String,
    description: String,
}

#[component]
pub fn Dialog(DialogProps { mut open, title, description} : DialogProps) -> Element {
    let open_prop = use_memo(move || Some(open()));

    rsx! {
        DialogRoot {
            open: open_prop,
            on_open_change: move |v| {
                open.set(v);
            },
            DialogContent {
                DialogTitle { "{title}" }
                DialogDescription { "{description}" }
            }
        }
    }
}
