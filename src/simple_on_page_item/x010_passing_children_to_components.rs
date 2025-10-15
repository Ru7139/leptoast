use leptos::prelude::*;

use crate::common_components::for_the_test::{
    ButtonPlusI32, ButtonToggle, SectionIndicator as CCFTSectionIndicator,
};

#[component]
pub fn ComponentChildren() -> impl IntoView {
    let (rx_data_0, tx_data_0) = signal(0i32);
    let (rx_data_1, tx_data_1) = signal(false);

    view! {
        <CCFTSectionIndicator title_name = "ComponentChildren"/>
        <p>
        "rx_data_0: " {rx_data_0} " "
        <ButtonPlusI32 write_signal = tx_data_0 plus_value = 3 button_msg = "+3"/>
        <br/>
        "rx_data_1: " {rx_data_1} " "
        <ButtonToggle write_signal = tx_data_1 button_msg = "toggle"/>
        </p>
    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, the_children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {}
}
