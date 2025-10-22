use leptos::{
    attr::span,
    html::{br, div},
    prelude::*,
};

#[component]
pub fn UseTheSignal() -> impl IntoView {
    let (read_x, write_x) = signal(0i32);

    div().child((span().child(("read value: ", move || read_x.get())), br()))
}
