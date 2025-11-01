use leptos::{
    ev,
    html::{button, div, h3, p},
    logging,
    prelude::*,
};

#[component]
pub fn UseResource() -> impl IntoView {
    let (count, set_count) = signal(0i32);

    // CSR
    let local_async_data = LocalResource::new(move || load_count_data(count.get()));

    // SSR
    let server_async_data = Resource::new(
        move || count.get(),
        |c| async move { fetch_data_from_server(c.to_string()).await },
    );

    // LocalResource和都Resource实现了各种信号访问方法
    // .read()、.with()、.get()，但返回Option<T>而不是T
    // 它们将None一直保持到异步数据加载完成

    div().attr("class", "UseResource").child((
        h3().child("---> UseResource()"),
        button()
            .on(ev::click, move |_click| set_count.update(|x| *x += 1))
            .child("+1 button"),
        p().child(("local async data: ", move || {
            local_async_data.with(|val| match val {
                None => "(No data from local async)".to_string(),
                Some(valued) => match &**valued {
                    Ok(v) => format!("value is {}", v),
                    Err(e) => format!("(WRONG: {})", e),
                },
            })
        })),
        p().child(("server async data: ", move || {
            server_async_data.with(|val| match val {
                None => "(No data from server async)".to_string(),
                Some(valued) => match valued {
                    Ok(v) => format!("value is {}", v),
                    Err(e) => format!("(WRONG: {})", e),
                },
            })
        })),
    ))
}

async fn load_count_data(read_data_count: i32) -> Result<i32, Box<dyn std::error::Error>> {
    logging::log!("Count Data is loading");
    // tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    Ok(read_data_count)
}

async fn fetch_data_from_server(url: String) -> Result<String, ServerFnError> {
    logging::log!("begin to fetch data from server");
    // tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    Ok(format!("bsv.ufort.{}", url))
}
