use leptos::{html::div, logging, prelude::*};

#[component]
pub fn UseEffect() -> impl IntoView {
    let (a, set_a) = signal("a:part".to_string());
    let (b, set_b) = signal("b:part".to_string());

    Effect::new(move |_| logging::log!("Value a_b = {}_{}", a.get(), b.get()));
}
