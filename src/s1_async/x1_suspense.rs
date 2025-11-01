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

    div().attr("class", "UseSuspense").child((
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
        Suspense(
            SuspenseProps::builder()
                .fallback(|| p().child("Loading... ..."))
                .children(ToChildren::to_children(move || {
                    p().child(move || {
                        once.with(|val| match val {
                            None => String::new(),
                            Some(v) => match v {
                                Err(e) => format!("Error happened: {}", e),
                                Ok(s) => format!("Value: {}", s),
                            },
                        })
                    })
                }))
                .build(),
        ),
    ))
}

async fn load_a(the_num: i32) -> Result<String, ServerFnError> {
    logging::log!("get the sentence from server");
    Ok(format!("the num is {}", the_num))
}

#[component]
pub fn UseAwait() -> impl IntoView {
    div().attr("class", "UseAwait").child((
        h3().child("---> UseAwait()"),
        Await(
            AwaitProps::builder()
                .future(fetch_a_monkey(3))
                .children(|valued: &i32| {
                    p().child(format!("little monkey on the water -> {}", *valued))
                })
                .build(),
        ),
    ))
}

async fn fetch_a_monkey(banana: i32) -> i32 {
    banana * 2
}
