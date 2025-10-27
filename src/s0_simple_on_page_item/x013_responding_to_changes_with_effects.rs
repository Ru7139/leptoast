use leptos::{
    ev,
    html::{br, button, div, h3, input, p},
    logging,
    prelude::*,
};

use std::cell::RefCell;
use std::rc::Rc;

#[component]
pub fn UseEffect() -> impl IntoView {
    let (a, set_a) = signal("a:part".to_string());
    let (b, set_b) = signal("b:part".to_string());
    let (c, set_c) = signal(1f64);

    // let (temp_a, set_temp_a) = signal(String::new());
    // let (temp_b, set_temp_b) = signal(String::new());
    let temp_a = Rc::new(RefCell::new(String::new()));
    let temp_b = Rc::new(RefCell::new(String::new()));

    // 尽量不写val_i.set(val_j.get() * 2)
    let c_double = move || c.get() * 2.0;
    // let pass_string_to_ab = move || {
    //     set_a.set(temp_a.get());
    //     set_b.set(temp_b.get());
    // };
    // let pass_string_to_ab = {
    //     set_a.set(temp_a.borrow().clone());
    //     set_b.set(temp_a.borrow().clone());
    // };
    let temp_a_clone = Rc::clone(&temp_a);
    let temp_b_clone = Rc::clone(&temp_b);
    let pass_string_to_ab = move || {
        set_a.set(temp_a_clone.borrow().clone());
        set_b.set(temp_b_clone.borrow().clone());
    };

    Effect::new(move |_| logging::log!("Value a with b = {} <--> {}", a.get(), b.get()));
    // 在组件挂载后立即执行的副作用，不用于事件

    div().child((
        h3().child("---> UseEffect()"),
        p().child("input something"),
        input()
            .attr("type", "text")
            .attr("placeholder", "input string for a")
            .on(ev::input, move |ev_input| {
                *temp_a.borrow_mut() = event_target_value(&ev_input)
                // temp_a = event_target_value(&ev_input)
                // set_temp_a.update(|x| *x = event_target_value(&ev_input))
            }),
        br(),
        input()
            /*
            每个字符都会触发一次 .on(ev::input) 事件
            而 event_target_value() 会调用：web_sys::EventTarget → HtmlInputElement::value()
            这实际上是一个 JS → WASM 的跨境调用
            对于 WebAssembly 而言，JS ↔ WASM 之间的交互非常昂贵。
            每次输入一个字母，浏览器都要触发一次
            input 事件 → 传递 event → WASM → 拿字符串 → clone → borrow_mut → 替换
            当输入的字符串到达 几万字符时，
            每次 value() 调用都要把整个字符串从 JS 拷贝进 WASM 内存中！
            这是 O(n) 操作 × n 次输入，总复杂度接近 O(n²)
            */
            .attr("type", "text")
            .attr("placeholder", "input string for b")
            .on(ev::input, move |ev_input| {
                *temp_b.borrow_mut() = event_target_value(&ev_input)
                // temp_b = event_target_value(&ev_input)
                // set_temp_b.update(|x| *x = event_target_value(&ev_input))
            }),
        br(),
        button()
            .on(ev::click, move |_click| pass_string_to_ab())
            .child("pass the value to ab"),
        br(),
        button()
            .on(ev::click, move |_click| {
                logging::log!("\ta = {}\n\tb = {}", a.get(), b.get())
            })
            .child("show a,b value on log"),
        br(),
        input()
            .attr("type", "number")
            .attr("placeholder", "input a number value for c")
            .on(ev::input, move |ev_input| {
                let value_string = event_target_value(&ev_input);
                if let Ok(val) = value_string.parse::<f64>() {
                    set_c.set(val);
                } else {
                    logging::log!("wrong number input for c")
                }
            }),
        br(),
        button()
            .on(ev::click, move |_click| {
                logging::log!("c_double = {}", c_double())
            })
            .child("show c value on log"),
        br(),
    ))
}

#[component]
pub fn UseEffectWatch() -> impl IntoView {
    let (num, set_num) = signal(0i32);

    let effct = Effect::watch(
        move || num.get(),
        move |num, prev_num, _| logging::log!("num = {}, pre_num = {:?}", num, prev_num),
        false,
    );

    div().child((
        h3().child("---> UseEffectWatch()"),
        button()
            .on(ev::click, move |_click| set_num.set(1))
            .child("set num = 1"),
        br(),
        button()
            .on(ev::click, move |_click| set_num.update(|x| *x += 1))
            .child("set num += 1"),
        br(),
        button()
            .on(ev::click, move |_click| effct.stop())
            .child("to stop effect watch"),
        br(),
    ))
}
