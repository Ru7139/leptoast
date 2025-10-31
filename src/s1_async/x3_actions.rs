use gloo_timers::future::TimeoutFuture;
use leptos::prelude::*;

#[component]
pub fn UseActions() -> impl IntoView {}

async fn add_todo_request(new_title: &str) -> String {
    TimeoutFuture::new(800).await;
    return "server side is OK".to_string();
}
