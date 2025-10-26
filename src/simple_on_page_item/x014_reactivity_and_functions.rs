use leptos::{
    ev,
    html::{button, div, h3, p},
    logging,
    prelude::*,
};

#[component]
pub fn InterludeAndFunctions() -> impl IntoView {
    let (count, set_count) = signal(0i32);

    let is_count_zero = move || count.get() == 0;
    let is_count_odd = move || count.get() % 2 == 1;
    // let is_count_even = move || count.get() % 2 == 0;

    let text = move || {
        if is_count_odd() {
            "odd"
        } else if is_count_zero() {
            "zero"
        } else {
            "even"
        }
    };

    Effect::new(move |_| logging::log!("the count is {}", text()));

    div().child((
        h3().child("---> InterludeAndFunctions()"),
        button()
            .on(ev::click, move |_click| set_count.update(|x| *x += 1))
            .child("+1"), // 在这里+1后也会触发Effect的运行
        p().child(move || text().to_uppercase()),
        p().child("closure is a necessary part of leptos"),
        p().child("virtual DOM like React, Yew, Dioxus"),
        p().child("they run the function repeatly to generate and to be fixed into VDOM"),
        p().child("Angular, Svelte are seperate [create] and [update]"),
        p().child("SolidJs, Sycamore and leptos can run the samll function"),
    ))
}
