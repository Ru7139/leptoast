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

    )
}
