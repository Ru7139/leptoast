use leptos::prelude::*;
use leptos::{IntoView, mount};

fn main() {
    mount::mount_to_body(app_0);
}

fn app_0() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <p>"Here is the leptos"</p>
        <p>"Count: " {count}</p>
        <button on:click = move |_| set_count.set(count.get() + 1)> "Click me"  </button>
    }
}
