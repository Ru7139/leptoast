use dioxus::prelude::*;

#[component]
pub fn hello_string() -> Element {
    rsx!("Hello from rust")
}

#[component]
pub fn dynamic_text() -> Element {
    let msg = "Hello";
    let num = 100;

    rsx!(
        div {
            p { "message: {msg}, number: {num}" }

        }

    )
}

#[component]
pub fn html_header() -> Element {
    rsx!(
        div {
            h1 { "h1 size" }
            h2 { "h2 size" }
            p { "p size" }
        }
    )
}

#[component]
pub fn attributes() -> Element {
    rsx!(
        div {
            background_color: "#C0C4C3",
            h1 { color: "red", "h1 size" }
            h2 { color: "blue", "h2 size" }
            p { color: "green", "p size" }
        }
    )
}

#[component]
pub fn additional_css_file() -> Element {
    rsx!(
        div {
            h1 { class: "abc", "additional css file" }
        }
    )
}
