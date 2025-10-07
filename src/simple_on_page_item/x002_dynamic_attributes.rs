use leptos::prelude::*;

#[component]
pub fn ShowDifferentColor() -> impl IntoView {
    let (rx, wx) = signal(0u32);

    view! {
        <div>
            <p> "value: " <span
                class:red=move || rx.get() % 2 == 0
                class:blue=move || rx.get() % 2 == 1
                > {rx} </span>
            </p>

            <button on:click = move |_| {
                     *wx.write() += 1
                }
                class:green
                // class:blue = move || rx.get() % 2 = 1
            > "+ 1" </button>
        </div>
    }
}
