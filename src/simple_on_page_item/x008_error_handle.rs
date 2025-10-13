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
    let (rx_data, tx_data) = signal(Ok(0i32));
    let _ = rx_data;

    view! {
        <div>
            <h3> "---> ErrorBoundary()" </h3>
            <label>
                "Type a number(or something is not a number)"
                <br/>
                <input type = "number" on:input:target = move |user_input| {
                   tx_data.set(user_input.target().value().parse::<i32>());
                }/> // 如果使用Some<T>的方式，输入字母也是无效的，和上面的相同

                // Something wrong with this part
                // <ErrorBoundary
                //     fallback = |errors| view! {
                //         <div class = "error">
                //             <p> "Not a number! Errors: " </p>
                //             <ul>
                //                 {
                //                     move || errors.get()
                //                                 .into_iter()
                //                                 .map(|(_,e)| view! { <li> {e.to_string()} </li> })
                //                                 .collect::<Vec<_>>()
                //                 }
                //             </ul>
                //         </div>
                //     }
                // >
                //     <p> "You entered: " <strong> {rx_data} </strong> </p>
                // </ErrorBoundry>

            </label>
        </div>
    }
}
