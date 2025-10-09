use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn ControlledInputsV0() -> impl IntoView {
    let (rx, tx) = signal("Cooperation".to_string());

    view! {
        <h3> "---> ControlledInputsV0()" </h3>
        <input type = "text"
            on:input:target = move |ev| { tx.set(ev.target().value()); }
            prop:value = rx // HTML attribute 和 DOM property存在着区别
            // leptos没有选择像其他前端框架一样去混淆，而是展示给开发者
        />
        <p> "The word is: " {rx} </p> // 每次输入时都会更改，无需按键
    }
}

