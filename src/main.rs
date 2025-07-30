use dioxus::prelude::*;

// const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        CounterBlock { }
    }
}

#[component]
fn CounterBlock() -> Element {
    let mut count = use_signal(|| 0usize);
    let mut message = use_signal(|| "Goooooood".to_string());
    let mut number = use_signal(|| 725usize);

    rsx!(
        h1 { "High-Five counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }

        h1 { "message: {message}" }
        button { onclick: move |_| message.set("Good".to_string()), "Good" }
        button { onclick: move |_| message.set("Bad".to_string()), "Bad" }

        h1 { "number: {number}" }
        button { onclick: move |_| number += 5, "Next" }
    )
}
