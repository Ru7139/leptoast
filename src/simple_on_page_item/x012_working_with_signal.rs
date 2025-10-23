use leptos::{
    ev,
    html::{br, button, div, h3, span},
    prelude::*,
};

#[component]
pub fn UseTheSignal() -> impl IntoView {
    let (read_x, write_x) = signal(0i32);

    div().child((
        h3().child("---> UseTheSignal()"),
        span().child(("rx_value .get(): ", move || read_x.get())),
        br(), // 会克隆信号的当前值，并自动更新，最常用
        span().child(("rx_value .read(): ", move || *read_x.read())),
        br(), // 不加上*则无法通过编译
        // 因为拿到的是&Signal<T>，在不想clone的时候很有用，比如数组长度
        span().child(("rx_value .with(): ", move || read_x.with(|x| *x))),
        br(), // 与.read()相似，但可以追踪信号，可以用一个闭包进行操作
        // 要确保持有锁的时间不超过想要渲染的时间
        button()
            .on(ev::click, move |_click| write_x.set(0))
            .child(" .set() to 0"),
        // span().child("wx_value .set(): ", move || ),
        br(),
        button()
            .on(ev::click, move |_click| *write_x.write() += 1)
            .child(" .write() to plus 1"),
        br(),
        button()
            .on(ev::click, move |_click| write_x.update(|x| *x *= 2))
            .child(" .update to 2x"),
        br(),
    ))
}
