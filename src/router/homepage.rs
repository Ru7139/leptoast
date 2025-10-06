use dioxus::prelude::*;

#[component]
pub fn Homepage() -> Element {
    rsx!(
        h1 { "Here is the Homepage" }
    )
}
