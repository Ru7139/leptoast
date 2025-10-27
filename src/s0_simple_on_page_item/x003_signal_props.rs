use leptos::prelude::*;

#[component]
pub fn ProgressBar_Default_100(
    #[prop(default = 100)] max_value: u32,
    signal_value: ReadSignal<u32>,
) -> impl IntoView {
    view! {
        <div>
            <progress
                max = max_value
                value = move || signal_value.get() // 需要添加move让leptos知道这是一个动态值
                style="width:320px; height:16px;" // default width:160px, height:16px
            />
        </div>
    }
}

#[component]
pub fn ProgressBar_Generic_Default_100<T>(
    #[prop(default = 100)] max_value: u32,
    signal_value: T,
) -> impl IntoView
where
    T: Fn() -> u32 + Send + Sync + 'static,
{
    view! {
        <div>
            <progress
                max = max_value
                value = signal_value
                style="width:320px; height:16px;"
            />
        </div>
    }
}

#[component]
pub fn ProgressBar_Into_D100(
    #[prop(default = 100)] max_value: u32,
    #[prop(into)] signal_value: Signal<u32>,
) -> impl IntoView {
    view! {
        <div>
            <progress
                max = max_value
                value = signal_value // 需要添加move让leptos知道这是一个动态值
                style="width:320px; height:16px;" // default width:160px, height:16px
            />
        </div>
    }
}
