use leptos::{
    html::{b, div, h1, h6, i, li, ol, p, table, th, tr, ul},
    prelude::*,
};

#[component]
pub fn Hello() -> impl IntoView {
    view! {
        <div>
            <p> "---> Hello()" </p>
            <h3> "おハヨウございます☀️leptosより" </h3>
        </div>
    }
}

#[component]
pub fn HtmlExercise() -> impl IntoView {
    div().child((
        h1().child("Using h1"),
        h6().child("using h6"),
        p().child(("段落标签", " ", b().child("加粗"), " ", i().child("斜体"))),
        ul().child((
            li().child("无序列表元素1"),
            li().child("无序列表元素2"),
            li().child("无序列表元素3"),
        )),
        ol().child((
            li().child("有序列表元素1"),
            li().child("有序列表元素2"),
            li().child("有序列表元素3"),
        )),
        table()
            .attr(
                "style",
                "border-collapse: collapse; border: 1px solid black;",
            )
            .child((
                tr().child((
                    th().attr("style", "border: 1px solid black;")
                        .child("列标题1"),
                    th().attr("style", "border: 1px solid black;")
                        .child("列标题2"),
                    th().attr("style", "border: 1px solid black;")
                        .child("列标题3"),
                )),
                tr().child((
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_x 1"),
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_x 2"),
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_x 3"),
                )),
                tr().child((
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_y 1"),
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_y 2"),
                    th().attr("style", "border: 1px solid black;")
                        .child("elem_y 3"),
                )),
            )),
    ))
}
