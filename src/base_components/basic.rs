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

#[derive(Clone, PartialEq)]
pub struct Rocket {
    pub type_code: char,
    pub code: u32,
    pub name: String,
}

pub static ROCKET_DEMO: once_cell::sync::Lazy<Rocket> = once_cell::sync::Lazy::new(|| Rocket {
    type_code: 'A',
    code: 1405,
    name: String::from("Pole"),
});

#[component]
pub fn props(fire_rocket: ReadOnlySignal<Rocket>) -> Element {
    let r = &*fire_rocket.read();

    rsx!(
        h1 { "This is Rocket {r.type_code.to_string()}{r.code.to_string()} named {r.name.clone()}" }
    )
}

#[component]
pub fn conditional_render(show_flag: bool) -> Element {
    rsx!(
        div {
            if show_flag {
                p { "show flag is {show_flag}" }
            } else {
                p { "show flag bool is {show_flag}" }
            }
        }
    )
}

// #[component]
// pub fn onclick_event() -> Element {
//     rsx!(
//         button {
//             onclick : move |_e| { dioxus_logger::tracing::info!("onclick event") }, "click"
//         }
//     )
// }

#[component]
pub fn use_signal_to_click() -> Element {
    let mut count = use_signal(|| 0isize);

    rsx!(
        div {
            h3 { "count: {count}" }
            button { onclick: move |_e| count += 1, "+= 1" }
            button { onclick: move |_e| count.set(0), "reset" }
            button { onclick: move |_e| count -= 1, "-= 1"}
        }
    )
}

pub static STRING_NOTHING: once_cell::sync::Lazy<String> =
    once_cell::sync::Lazy::new(|| "nothing".to_string());

#[component]
pub fn use_context_from_app(value: String) -> Element {
    rsx!( h3 { "string value from app: {value}" } )
}
