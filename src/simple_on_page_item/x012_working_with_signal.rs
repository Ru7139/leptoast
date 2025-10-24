use leptos::{
    ev,
    html::{br, button, div, h3, span},
    prelude::*,
};

#[component]
pub fn UseTheSignal() -> impl IntoView {
    let (read_x, write_x) = signal(0i32);
    let (names, set_names) = signal(Vec::<String>::new());

    div().child((
        h3().child("---> Working with signal ---> UseTheSignal()"),
        span().child(("rx_value .get(): ", move || read_x.get())),
        br(), // 会克隆信号的当前值，并自动更新，最常用
        span().child(("rx_value .read(): ", move || *read_x.read())),
        br(), // 不加上*则无法通过编译
        // 因为拿到的是&Signal<T>，在不想clone的时候很有用，比如数组长度
        span().child(("rx_value .with(): ", move || read_x.with(|x| *x))),
        br(), // 与.read()相似，但可以用一个闭包进行操作
        // 要确保持有锁的时间不超过想要渲染的时间
        button()
            .on(ev::click, move |_click| write_x.set(0))
            .child(" .set() to 0"),
        br(), // 整体上与rx data的api相同
        button()
            .on(ev::click, move |_click| *write_x.write() += 1)
            .child(" .write() to plus 1"),
        br(),
        button()
            .on(ev::click, move |_click| write_x.update(|x| *x *= 2))
            .child(" .update to 2x"),
        br(),
        span().child(("name: ", move || names.get())), // 这个地方如果用read()会造成&和&mut冲突？
        button()
            .on(ev::click, move |_click| {
                if names.read().is_empty() {
                    set_names.write().push("Alice".to_string());
                    // 比起set()，这种方式不用克隆整个Vec
                }
            })
            .child("set name as Alice"),
        // 当需要!Send类型的数据时，可以使用以下几个方法
        // signal --> signal_local
        // RwSignal::new --> RwSignal::new_local
        // Resource --> LocalResource
        // Action::new --> Action::new_local, Action::new_unsync
    ))
}
