use leptos::prelude::*;

use super::super::simple_on_page_item::{
    x000_aisatsu::Hello, x001_counter::TheCouter, x002_dynamic_attributes::ShowDifferentColor,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Hello/>
            <TheCouter initial_value=0 step_value=1/>
            <ShowDifferentColor/>
        </div>
    }
}
