use leptos::prelude::*;

#[component]
pub fn TheCouter(initial_value: i32, step_value: i32) -> impl leptos::IntoView {
    // fn signal<T>(value: T) -> (ReadSignal<T>, WriteSignal<T>)
    let (show_value, send_value) = signal(initial_value);

    view! {
        <div>
            <span>"count :" {show_value} </span>
            <p></p>
            <button on:click = move |_| send_value.set(show_value.get() - step_value)> "-"  </button>
            <button on:click = move |_| send_value.set(show_value.get() + step_value)> "+"  </button>
            <button on:click = move |_| send_value.set(0)> "re-step" </button>

        </div>
    }
}
