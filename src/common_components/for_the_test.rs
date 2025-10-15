use leptos::prelude::*;

#[component]
pub fn SectionIndicator(title_name: &'static str) -> impl IntoView {
    view! {
        <h3> "---> " {title_name} "()" </h3>
    }
}

#[component]
pub fn ButtonPlusI32(
    write_signal: WriteSignal<i32>,
    plus_value: i32,
    button_msg: &'static str,
) -> impl IntoView {
    view! {
        <button on:click = move |_user_click| {
            write_signal.update(|x| *x += plus_value);
        } >
            {button_msg}
        </button>
    }
}

#[component]
pub fn ButtonToggle(write_signal: WriteSignal<bool>, button_msg: &'static str) -> impl IntoView {
    view! {
        <button on:click = move |_user_click| {
            write_signal.update(|x| *x = !*x);
        } >
            {button_msg}
        </button>
    }
}
