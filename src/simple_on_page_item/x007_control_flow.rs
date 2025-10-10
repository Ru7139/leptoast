use leptos::prelude::*;

#[component]
pub fn IfControl() -> impl IntoView {
    let (rx_data, tx_data) = signal(0u32);
    let data_recieved_is_odd = move || -> bool { rx_data.get() % 2 != 0 };

    view! {
        <h3> "---> IfControl()" </h3>

    }
}
