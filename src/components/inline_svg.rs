use std::borrow::Cow;

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InlineSvgProps {
    #[props(default, into)]
    class: Cow<'static, str>,
    content: &'static str,
}

#[component]
pub fn InlineSvg(props: InlineSvgProps) -> Element {
    let class_attr = if props.class.is_empty() {
        None
    } else {
        Some(props.class.as_ref())
    };

    rsx! {
        div { class: class_attr, dangerous_inner_html: "{props.content}" }
    }
}
