use leptos::prelude::*;

#[derive(Debug, Clone)]
struct DatabaseEntry {
    key: String,
    value: i32,
}
impl DatabaseEntry {
    fn new(key_str: &str, value: i32) -> DatabaseEntry {
        DatabaseEntry {
            key: key_str.into(),
            value,
        }
    }
}

#[component]
pub fn complex_data_iter() -> impl IntoView {
    let (data, set_data) = signal(Vec::from([
        DatabaseEntry::new("aeiot", 15),
        DatabaseEntry::new("bezwy", 32),
        DatabaseEntry::new("cothey", 7),
    ]));

    view! {
        <p> "---> complex_data_iter()" </p>
        <button on:click = move |_| {
            set_data.update(|data| {
                for row in data { row.value += 13; }
            });

            leptos::logging::log!("{:?}", data.get());

        }> "Update Values" </button>

        <For
            each = move || data.get()
            key = |state| (state.key.clone(), state.value) // 这里有其他两种写法，这种效率最低
            let(child) // 每次值发生变化时会丢弃之前的<p>，再重新渲染，这导致效率与UI复杂程度成反比
        >
            <p>{child.value}</p>
        </For>

    }
}
