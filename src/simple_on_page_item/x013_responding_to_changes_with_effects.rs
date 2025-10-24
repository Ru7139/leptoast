use leptos::{html::div, logging, prelude::*};

#[component]
pub fn UseEffect() -> impl IntoView {
    let (a, set_a) = signal(String::new());
    let (b, set_b) = signal(String::new());

    Effect::new(move |_| logging::log!("Value: {}", a.get()));
}
