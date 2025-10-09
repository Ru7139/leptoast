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

#[component]
pub fn ControlledInputsV1() -> impl IntoView {
    let (rx, tx) = signal("Cooperation 2".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <h3> "---> ControlledInputsV1()" </h3>
        <p>
            "Name: "
            <input type = "text" bind:value = (rx,tx) />
        </p>

        <p>
            "Email: "
            <input type = "email" bind:value = email />
            <br> </br>
            <label>
                " Please send me lots of spam email"
                <input type = "checkbox" bind:value = spam_me/>
            </label>
        </p>



        <fieldset>
            <legend> "Favorite color" </legend>
            <label>
                "Red"
                <input type="radio" name="color" value="Red" bind:group = favorite_color/>
            </label>
            <label>
                "Green"
                <input type="radio" name="color" value="Green" bind:group = favorite_color/>
            </label>
            <label>
                "Blue"
                <input type="radio" name="color" value="Blue" bind:group = favorite_color/>
            </label>
        </fieldset>

        <p> "Your favorite color is: " {favorite_color} </p>
        <p> "Name is: " {rx} </p>
        <p> "Email is: " {email} </p>
        <Show when = move || spam_me.get()>
            <p> "You'll receive coll bonus content!" </p>
        </Show>
    }
}

