use leptos::prelude::*;

#[component]
pub fn NumericInput() -> impl IntoView {
    let (rx_data, tx_data) = signal(0i32);

    view! {
        <div>
            <h3> "---> BetterUseShow()" </h3>

            <label>
                "Type an integer (or try a sentence)"
                <br/>
                <input type = "number" on:input:target = move |user_input|  {
                    tx_data.set(user_input.target().value().parse::<i32>().unwrap_or_default())
                    // 这种方式只允许数字输入和一些类似于e一样的东西输入，并且实时显示
                } />
                <p> "You entered: " <strong> {rx_data} </strong> </p>
            </label>
        </div>
    }
}

#[component]
pub fn ErrorBoundary() -> impl IntoView {
    let (rx_data, tx_data) = signal(0i32);

    view! {}
}
