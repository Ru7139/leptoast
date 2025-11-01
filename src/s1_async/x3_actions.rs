use gloo_timers::future::TimeoutFuture;

use leptos::{
    ev,
    html::{Input, br, button, div, form, h3, input, label, p},
    prelude::*,
};

#[component]
pub fn UseActions() -> impl IntoView {
    let action1 = Action::new(|input: &String| {
        let i = input.to_owned();
        async move { add_todo_request(&i).await }
    });

    let submitted = action1.input();
    let pending = action1.pending();
    let todo_id = action1.value();

    // Action接受一个async函数，这个函数接受对单个参数的引用
    // let _action2 = Action::new(|_input: &String| async { todo!() });
    // let _action3 = Action::new(|_input: &(usize, String)| async { todo!() });

    let input_ref = NodeRef::<Input>::new();

    div().attr("class", "UseActions").child((
        h3().child("---> UseActions()"),
        form()
            .on(ev::submit, move |evs| {
                evs.prevent_default();
                let input = input_ref.get().expect("input to exist");
                action1.dispatch(input.value());
            })
            .child(label().child((
                p().child("What do you need to do?"),
                input().attr("type", "text").node_ref(input_ref),
                br(),
                button().attr("type", "submit").child("Add Todo"),
            ))),
        p().child(("Pending:", move || pending.get().then_some("Loading..."))),
        p().child(("Submit: ", move || format!("{:#?}", submitted.get()))),
        p().child(("Pending: ", move || format!("{:#?}", pending.get()))),
        p().child(("Todo ID: ", move || format!("{:#?}", todo_id.get()))),
    ))
}

async fn add_todo_request(new_title: &str) -> Result<String, ServerFnError> {
    // TimeoutFuture::new(800).await;
    // future cannot be sent between threads safely
    // the trait Send is not implemented for (dyn FnMut() + 'static)
    send_wrapper::SendWrapper::new(TimeoutFuture::new(800)).await;
    return Ok(format!("server side is OK with {}", new_title));
}
