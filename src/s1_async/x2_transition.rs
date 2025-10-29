use gloo_timers::future::TimeoutFuture;
use leptos::{
    ev,
    html::{br, button, div, h3, p},
    prelude::*,
};

#[allow(non_snake_case)]
async fn Nameofabc(id: usize) -> String {
    TimeoutFuture::new(500).await;
    match id {
        0 => "Aeop",
        1 => "Boopey",
        2 => "Cathort",
        _ => "NO NAME FOR THIS(unreachable)",
    }
    .to_string()
}

#[component]
pub fn UseTransition() -> impl IntoView {
    let (tab, set_tab) = signal(0usize);
    let (pend, set_pend) = signal(false);

    let user_data = LocalResource::new(move || Nameofabc(tab.get()));

    // let data_text = move || user_data.get().as_deref().map(ToString::to_string);

    div().attr("class", "UseTransition").child((
        h3().child("---> UseTransition()"),
        div().attr("class", "buttons").child((
            button()
                .on(ev::click, move |_click| set_tab.set(0))
                .class(("selected", move || tab.get() == 0))
                // 当后面的bool闭包为true时，添加selected这个class
                .child("Tap A"),
            br(),
            button()
                .on(ev::click, move |_click| set_tab.set(1))
                .class(("selected", move || tab.get() == 1))
                .child("Tap B"),
            br(),
            button()
                .on(ev::click, move |_click| set_tab.set(2))
                .class(("selected", move || tab.get() == 2))
                .child("Tap C"),
            br(),
        )),
        div().child((
            button()
                .on(ev::click, move |_click| set_pend.update(|x| *x = !*x))
                .child("change pend(bool)"),
            p().child(move || if pend.get() { "Hang on..." } else { "Ready" }),
            Transition(
                TransitionProps::builder()
                    .fallback(|| p().child("Loading initial data"))
                    .set_pending(set_pend)
                    .children(ToChildren::to_children(move || {
                        p().child(move || user_data.get().as_deref().map(ToString::to_string))
                    }))
                    .build(),
            ),
        )),
    ))
}
