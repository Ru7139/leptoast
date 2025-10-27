use leptos::{ev::SubmitEvent, prelude::*};

#[component]
pub fn ControlledInputsV0() -> impl IntoView {
    let (rx, tx) = signal("Cooperation".to_string());

    view! {
        <div>
            <h3> "---> ControlledInputsV0()" </h3>
            <input type = "text"
                on:input:target = move |ev| { tx.set(ev.target().value()); }
                prop:value = rx // HTML attribute 和 DOM property存在着区别
                // leptos没有选择像其他前端框架一样去混淆，而是展示给开发者
            />
            <p> "The word is: " {rx} </p> // 每次输入时都会更改，无需按键
        </div>
    }
}

#[component]
pub fn ControlledInputsV1() -> impl IntoView {
    let (rx, tx) = signal("Cooperation 2".to_string());
    let email = RwSignal::new("".to_string());
    let favorite_color = RwSignal::new("red".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <div>
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
        </div>
    }
}

#[component]
pub fn ControlledInputsV2() -> impl IntoView {
    let (rx_name, tx_name) = signal("Uncontrolled".to_string());
    let input_element: NodeRef<leptos::html::Input> = NodeRef::new();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element
            .get()
            .expect("<input> should be mounted")
            .value();
        tx_name.set(value);
    };

    view! {
        <div>
            <h3> "---> ControlledInputsV2()" </h3> // 不立即显示的<input>
            <form on:submit = on_submit>
                <input type = "text"
                    value = rx_name
                    node_ref = input_element
                />
                <input type = "submit" value= "Submit"/> // 贴在<input>旁边，显示Submit
            </form>

            <p> "Name is: " {rx_name} </p>
        </div>
    }
}

#[component]
pub fn TextareaSpecial() -> impl IntoView {
    let (rx_data, tx_data) = signal("Put your idea here".to_string());

    view! {
        <div>
            <h3> "---> TextareaSpecial()" </h3>
            <textarea
                prop:value = move || rx_data.get()
                on:input:target = move |ev| tx_data.set(ev.target().value())
            >
                {rx_data}
            </textarea>
        </div>
    }
}

#[component]
pub fn SelectSpecial() -> impl IntoView {
    let (rx_data, tx_data) = signal(0i32);

    view! {
        <div>
            <h3> "---> SelectSpecial()" </h3>
            <select
                on:change:target = move |ev| {
                    tx_data.set(ev.target().value().parse().unwrap());
                }
                prop:value = move || rx_data.get().to_string()
            >
                <option value = "0"> "0" </option>
                <option value = "1"> "1" </option>
                <option value = "2"> "2" </option>
            </select>
            <button on:click = move |_| tx_data.update(|n| {
                if *n == 2 {*n = 0;} else { *n += 1;}
                // 显示1的时候，点击后n+1，显示2，再点击后2->0
            })> "Next Option"
            </button>
        </div>
    }
}
