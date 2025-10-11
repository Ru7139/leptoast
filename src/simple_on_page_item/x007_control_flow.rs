use leptos::prelude::*;

#[component]
pub fn IfControl() -> impl IntoView {
    let (rx_data, tx_data) = signal(0u32);
    let data_recieved_is_odd = move || -> bool { rx_data.get() % 2 != 0 };

    view! {
        <h3> "---> IfControl()" </h3>

pub fn BetterUseShow() -> impl IntoView {
    let (rx_data, tx_data) = signal(0u32);
    fn DataIsBig() -> impl IntoView { view! {<p> "Big" </p>} }
    fn DataIsSmall() -> impl IntoView { view! {<p> "Small" </p>} }

    view! {
        <h3> "---> BetterUseShow()" </h3>
        <Show // 只会渲染1次，不会反复渲染
            when = move || { rx_data.get() <= 5 }
            fallback = || view! { <DataIsBig/> }
        >
            <DataIsSmall/> // 包括这里也只会渲染1次
        </Show>

        <span> {rx_data} " " </span>

        <button on:click = move |_|
                tx_data.update(|x| *x += 1)>
            " +1 button"
        </button>
    }
}
