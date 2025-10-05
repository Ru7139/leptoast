mod base_components;

use dioxus::prelude::*;

use base_components::basic as bcb;

use crate::document::Stylesheet;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! (
        head { Stylesheet { href: asset!("assets/style_0.css") } }

        bcb::hello_string {}
        bcb::dynamic_text {}
        bcb::html_header {}
        bcb::attributes {}
        bcb::additional_css_file {}
        bcb::props { fire_rocket: bcb::ROCKET_DEMO.clone() }
        bcb::conditional_render { show_flag: true }
        // bcb::onclick_event {}
        bcb::use_signal_to_click {}
        bcb::use_context_from_app { value: bcb::STRING_NOTHING.clone() }
        bcb::hidden_attribute_and_conditional_class {}
        bcb::list_cop {}
        bcb::oninput_event {}

        Router::<bcb::Route> {}
    )
}
