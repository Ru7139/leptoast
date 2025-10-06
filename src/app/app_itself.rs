use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<super::super::router::router_itself::Routeeth> {}
    }
}
