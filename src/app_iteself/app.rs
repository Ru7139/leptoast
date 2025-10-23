use leptos::prelude::*;

use super::super::simple_on_page_item::{
    x000_aisatsu::{Hello, HtmlExercise},
    x001_counter::TheCouter,
    x002_dynamic_attributes::ShowDifferentColor,
    x004_iteration::{Forview, VecViewBasic},
    x005_iterating_over_complex_data::{
        BetterIterA, BetterIterMemo, ComplexDataIter, MaybeBestIterStores,
    },
    x006_forms_and_inputs::{
        ControlledInputsV0, ControlledInputsV1, ControlledInputsV2, SelectSpecial, TextareaSpecial,
    },
    x007_control_flow::{BetterUseShow, IfControl},
    x008_error_handle::{ErrorBoundary, NumericInput},
    x009_parentchild_communication::{DeeplyNestedComponent, PassTXSignalPlusOne},
    x010_passing_children_to_components::{ComponentChildren, HiddenMessage, WrapsChildren},
    x011_the_view_builder_syntax::NoViewAtensionButtonI32,
    x012_working_with_signal::UseTheSignal,
};

#[component]
pub fn App() -> impl IntoView {
    let (rx_data, tx_data) = signal(0u64);

    view! {
        <div>
            <UseTheSignal/>
            <NoViewAtensionButtonI32 start_value = 10 step_value = 2/>
            <WrapsChildren> "A" "B" "C" </WrapsChildren>
            <HiddenMessage/>
            <ComponentChildren/>
            <DeeplyNestedComponent/>
            <PassTXSignalPlusOne rx_signal=rx_data tx_signal=tx_data/>
            <ErrorBoundary/>
            <NumericInput/>
            <BetterUseShow/>
            <IfControl/>
            <SelectSpecial/>
            <TextareaSpecial/>
            <ControlledInputsV2/>
            <ControlledInputsV1/>
            <ControlledInputsV0/>
            <MaybeBestIterStores/>
            <BetterIterMemo/>
            <BetterIterA/>
            <ComplexDataIter/>
            <VecViewBasic times=3/>
            <Forview/>
            <ShowDifferentColor/>
            <TheCouter initial_value=0 step_value=1/>
            <Hello/>
            <br/>
            <HtmlExercise/>
        </div>
    }
}
