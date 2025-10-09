use leptos::prelude::*;

use super::super::simple_on_page_item::{
    x000_aisatsu::Hello,
    x001_counter::TheCouter,
    x002_dynamic_attributes::ShowDifferentColor,
    x004_iteration::{Forview, VecViewBasic},
    x005_iterating_over_complex_data::{
        BetterIterA, BetterIterMemo, ComplexDataIter, MaybeBestIterStores,
    },
};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div>
            <MaybeBestIterStores/>
            <BetterIterMemo/>
            <BetterIterA/>
            <ComplexDataIter/>
            <VecViewBasic times=3/>
            <Forview/>
            <ShowDifferentColor/>
            <TheCouter initial_value=0 step_value=1/>
            <Hello/>


        </div>
    }
}
