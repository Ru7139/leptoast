use leptos::{
    ev,
    html::{button, div, h3, p},
    logging,
    prelude::*,
};

#[component]
pub fn UseSuspense() -> impl IntoView {
    let (count, set_count) = signal(0i32);
    let once = Resource::new(
        move || count.get(),
        |count| async move { load_a(count).await },
    );

    div().child((
        h3().child("---> UseSuspense()"),
        button()
            .on(ev::click, move |_click| set_count.update(|x| *x += 1))
            .child("+1"),
        p().child(("server 1 : ", once)), // no error handle
        p().child(("server 2 : ", move || {
            once.with(|val| match val {
                None => "fetch failed".to_string(),
                Some(v) => match v {
                    Err(e) => format!("Error: {}", e),
                    Ok(s) => format!("value: {}", s),
                },
            })
        })),
        // p().child((
        //     "server 3 : ",
        //     Suspense(|| p().child("loading ......")).child(p().child(move || {
        //         once.with(|val| match val {
        //             None => String::new(), // Suspense 在 None 时自动显示 fallback
        //             Some(v) => match v {
        //                 Err(e) => format!("Error: {}", e),
        //                 Ok(s) => format!("value: {}", s),
        //                 // SuspenseProps::builder()
        //                 //     .fallback(|| p().child("loading ......"))
        //                 //     .children(p().child(move || {
        //                 //         once.with(|val| match val {
        //                 //             None => String::new(),
        //                 //             Some(v) => match v {
        //                 //                 Err(e) => format!("Error: {}", e),
        //                 //                 Ok(s) => format!("value: {}", s),
        //             },
        //         })
        //     })),
        // )),
    ))
}

async fn load_a(the_num: i32) -> Result<String, ServerFnError> {
    logging::log!("get the sentence from server");
    Ok(format!("the num is {}", the_num))
}
