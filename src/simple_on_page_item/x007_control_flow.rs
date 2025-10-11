use leptos::prelude::*;

#[component]
pub fn IfControl() -> impl IntoView {
    let (rx_data, tx_data) = signal(0u32);
    let data_recieved_is_odd_0 = move || -> bool { rx_data.get() % 2 != 0 };
    let odd_or_even_msg_v1 = move || -> Option<&'static str> {
        if data_recieved_is_odd_0() {
            Some("it is odd")
        } else {
            Some("it is not odd")
        }
    };

    let odd_or_even_msg_v2 = move || -> Option<&'static str> {
        data_recieved_is_odd_0().then(|| "that is an odd number")
    };

    let odd_or_even_msg_v3 = move || -> &'static str {
        match rx_data.get() {
            0 => "Zero",
            1 => "One",
            _x if data_recieved_is_odd_0() => "x -> Odd",
            _y if !data_recieved_is_odd_0() => "x -> Even",
            _ => unreachable!(),
        }
    };

    view! {
        <h3> "---> IfControl()" </h3>

        <p> { rx_data } </p>

        <span>   "---> if itself: "  </span>
        <p style="display: inline;"> {
            move || if data_recieved_is_odd_0() { "Odd" }
                else { "Even" }
            } </p> <br/>

        <span> "---> if with Option<T>: " </span>
        <p style="display: inline;"> {
            odd_or_even_msg_v1 }
        </p> <br/>

        <span> "---> if with Option<T> with bool::then(): " </span>
        <p style="display: inline;"> {
            odd_or_even_msg_v2 }
        </p> <br/>

        <span> "---> also match: " </span>
        <p style="display: inline;"> {
            odd_or_even_msg_v3 }
        </p> <br/>

        <button on:click = move |_|
            tx_data.update(|x| *x += 1)>
                "+1 on data"
        </button>
    }
}

#[rustfmt::skip]
#[component]
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
