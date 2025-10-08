use leptos::prelude::*;

#[component]
pub fn Vec_View_Basic(times: usize) -> impl IntoView {
    let vek = (1..=times).step_by(1).collect::<Vec<usize>>();

    let counters = (1..=times).map(|idx| RwSignal::new(idx));
    let counters_buttons = counters
        .map(|count| {
            view! {
                <li> <button on:click=move |_| *count.write() += 1> {count} </button> </li>
            }
        })
        .collect_view();

    view! {
        <p> "---> Vec_view()" </p>
        <p > {vek.clone()} </p> // 渲染123...times
        <ul>
            {vek.into_iter().map(|x| view! { <li>{x}</li>}).collect_view() }
            // 渲染 ·1 ·2 ·3 ...... ·times
        </ul>
        <br/>
        <ul> {counters_buttons} </ul>
    }
}

// #[component]
// pub fn For_view(times: usize) -> impl IntoView {
//     view! {}
// }
