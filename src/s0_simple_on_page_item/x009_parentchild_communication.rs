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

#[component]
pub fn DeeplyNestedComponent() -> impl IntoView {
    view! {
        <div>
            <Layout header_msg = "header: ---> DeeplyNestedComponent()"/>
        </div>
    }
}

#[component]
fn Layout(header_msg: &'static str) -> impl IntoView {
    view! {
        <header>
            <h1> {header_msg} </h1>
        </header>
        <main>
            <Content/>
        </main>
    }
}

#[component]
fn Content() -> impl IntoView {
    view! {
        <div class="content">
            <ButtonD/>
        </div>
    }
}

#[component]
fn ButtonD() -> impl IntoView {
    let (rx_data, tx_data) = signal(0i32);

    view! {
        <p> {rx_data} </p>
        <button on:click = move |_user_click|
            tx_data.update(|x| { *x += 2})
        > "use this button to +2" </button>
    }
}
