use leptos::prelude::*;

use super::x003_signal_props::{
    ProgressBar_Default_100, ProgressBar_Generic_Default_100, ProgressBar_Into_D100,
};

#[component]
pub fn ShowDifferentColor() -> impl IntoView {
    let (rx, wx) = signal(0u32);

    let double_rx = move || rx.get() * 2;
    let trible_rx = Signal::derive(move || rx.get() * 3);

    view! {
        <div>
                <p> "---> ShowDifferentColor()" </p>
            <div>
                <p> "value: " <span
                    class:red = move || rx.get() % 2 == 0
                    class:blue = move || rx.get() % 2 == 1
                    > {rx} </span>
                </p>
                <p> "double count: " {double_rx} </p>

                <button on:click = move |_| {
                        *wx.write() += 5
                    }
                    class:green = move || rx.get() % 20 == 0 && rx.get() != 0
                > "+ 5" </button>
            </div>
            <br/> // 直接换行 临时分隔小元素
            <ProgressBar_Default_100 signal_value=rx/> // 可以使用default的方式不填写max值
            <br/>
            <ProgressBar_Generic_Default_100 signal_value=double_rx/> //通过这种方式可以传入一个闭包
            <br/>
            <ProgressBar_Into_D100 signal_value = rx/> //ReadSignal<T> into为 Signal<T>
            <br/>
            <ProgressBar_Into_D100 signal_value = trible_rx/> // 可以直接传入 Signal<T>
        </div>

    }
}
