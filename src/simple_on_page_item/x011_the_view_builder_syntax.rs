use leptos::{
    ev,
    html::{br, button, div, p, span},
    prelude::*,
};

#[component]
pub fn NoViewAtensionButtonI32(start_value: i32, step_value: i32) -> impl IntoView {
    let (read_count, write_count) = signal(start_value);

    div().child((
        span().child(("value: ", move || read_count.get(), " right now")),
        br(),
        button()
            .on(ev::click, move |_click| {
                write_count.update(|x| *x += step_value);
            })
            .child("add a step"),
        button()
            .on(ev::click, move |_click| {
                write_count.update(|x| *x = 0);
            })
            .child("reset"),
        button()
            .on(ev::click, move |_click| {
                write_count.update(|x| *x -= step_value)
            })
            .child("take back a step"),
        Show(
            ShowProps::builder()
                .when(move || read_count.get() > 5)
                .fallback(|| p().child("I show up until value bigger than 5"))
                .children(ToChildren::to_children(|| {
                    p().child("value is bigger than 5")
                }))
                .build(),
        ),
    ))
}
