use leptos::prelude::*;

#[component]
pub fn VecViewBasic(times: usize) -> impl IntoView {
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

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// struct Kounter {
//     id: usize,
//     count: RwSignal<i32>,
// }

#[component]
pub fn Forview() -> impl IntoView {
    view! {
        <p> "---> For_view() : for" </p>
        <p> "The document of this chapter is confused" </p>
    }

    // let kk = Kounter {
    //     id: 3,
    //     count: RwSignal::new(0),
    // };

    // let (show_y, _send_x) = signal::<Vec<Kounter>>(vec![]);

    // view! {
    //     <p> "---> For_view() : for" </p>

    //     <ForEnumerate
    //         each = move || show_y.get()
    //         key = |k| k.id
    //         children = { move |index: ReadSignal<usize>, counter: Kounter| { view!
    //             { <button> {move || index.get()} ". Value: " {move || counter.count.get()} </button>
    //         } } }
    //     />
    // }
}
