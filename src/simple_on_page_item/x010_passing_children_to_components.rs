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

        <TakesChildren render_prop = || view! { <p>"Hi, there!"</p> }>
            "Some text "
            <span>"A span"</span>
        </TakesChildren>


    }
}

#[component]
fn TakesChildren<F, IV>(render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! {
        <h1> <code> "<TakesChildren/>" </code> </h1>

        <h2>"Render Prop"</h2>
        { render_prop() }

        <hr/> // 分割线

        <h2>"Children"</h2>
        { children() } // 在children的命名上，必须是children，否则报错
    }
}

#[component]
pub fn HiddenMessage() -> impl IntoView {
    let (rx_data, tx_data) = signal(0i32);
    let is_even = Signal::derive(move || rx_data.get() % 2 == 0);
    let is_div5 = Signal::derive(move || rx_data.get() % 5 == 0);
    let is_div7 = Signal::derive(move || rx_data.get() % 7 == 0);

    view! {
        <CCFTSectionIndicator title_name = "HiddenMessage"/>
        <p> "rx_data: " {rx_data} </p>
        <ButtonPlusI32 write_signal = tx_data button_msg = "+1" plus_value = 1/>
        <ButtonPlusI32 write_signal = tx_data button_msg = "+3" plus_value = 3/>
        <br/>

        <SlotIf cond = is_even>
            <Then slot>"even"</Then>
            // <ElseIf slot cond=is_div5>"divisible by 5"</ElseIf>
            // <ElseIf slot cond=is_div7>"divisible by 7"</ElseIf>
            <Fallback slot>"odd"</Fallback>
        </SlotIf>
    }
}

#[rustfmt::skip]
#[slot]
struct Then { children: ChildrenFn }

#[rustfmt::skip]
#[slot]
struct ElseIf { cond: Signal<bool>, children: ChildrenFn }

#[rustfmt::skip]
#[slot]
struct Fallback { children: ChildrenFn }

#[component]
fn SlotIf(
    cond: Signal<bool>,
    then: Then,
    #[prop(default=vec![])] else_if: Vec<ElseIf>,
    #[prop(optional)] fallback: Option<Fallback>,
) -> impl IntoView {
    move || {
        if cond.get() {
            (then.children)().into_any()
        } else if let Some(else_if) = else_if.iter().find(|i| i.cond.get()) {
            (else_if.children)().into_any()
        } else if let Some(fallback) = &fallback {
            (fallback.children)().into_any()
        } else {
            ().into_any()
        }
    }
}

// #[slot]
// struct Then {
//     pub children: ChildrenFn,
// }

// #[slot]
// pub struct Else {
//     pub children: ChildrenFn,
// }

// #[component]
// fn If(
//     condition: ReadSignal<bool>,
//     // Component slot, should be passed through the <Then slot> syntax
//     then_slot: Then,
//     #[prop(optional)] else_slot: Else,
// ) -> impl IntoView {
//     move || {
//         if condition.get() {
//             (then_slot.children)().into_any()
//         } else if let Some(else_slot) = &else_slot {
//             (else_slot.children)().into_any()
//         } else {
//             ().into_any()
//         }
//     }
// }

#[component]
pub fn WrapsChildren(children: ChildrenFragment) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li> {child} </li>})
        .collect::<Vec<_>>();

    view! {
        <h1> <code> "<WrapsChildren/>" </code> </h1>
        <ul> {children} </ul>
    }
}
