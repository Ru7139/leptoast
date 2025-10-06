mod db;

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        h2 { "Try This Test" }
    }
}

#[derive(PartialEq, Debug, Deserialize, Serialize)]
struct TargetLocation {
    x: f64,
    y: f64,
    z: f64,
    location_name: String,
}
