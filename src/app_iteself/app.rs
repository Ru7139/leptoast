use leptos::prelude::*;

use super::super::simple_on_page_item::{
    x000_aisatsu::Hello, x001_counter::TheCouter, x002_dynamic_attributes::ShowDifferentColor,
    x004_iteration::Vec_View_Basic,
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <Vec_View_Basic times=3/>
            <ShowDifferentColor/>
            <TheCouter initial_value=0 step_value=1/>
            <Hello/>


        </div>
    }
}
