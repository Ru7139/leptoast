use leptos::{ev::MouseEvent, prelude::*};

#[component]
pub fn PassTXSignalPlusOne(
    rx_signal: ReadSignal<u64>,
    tx_signal: WriteSignal<u64>,
) -> impl IntoView {
    view! {
        <div>
            <h3> "---> PassTXSignalPlusOne(rx_signal: ReadSignal<i32>, tx_signal: WriteSignal<i32>)" </h3>
            <p> {rx_signal} </p>
            <button on:click = move |_user_input| {
                tx_signal.update(|x| *x += 1 );
            }> "pass the tx_signal from app to component"
            </button>

            <br/>
            <UseCallback mouse_click = move |_mouse_click| tx_signal.update(|x| *x *= 2)/>

            <br/>
            <UseEventListener on:click = move |_mouse_click| tx_signal.update(|x| *x *= 10)/>


        </div>
    }
}

#[component]
fn UseCallback(mouse_click: impl FnMut(MouseEvent) + 'static) -> impl IntoView {
    view! {
        <button on:click = mouse_click>
            "mouse click 2x !!!"
        </button>
    }
}

#[component]
fn UseEventListener() -> impl IntoView {
    view! {
        <button> "Multiply 10x" </button>
    }
}
