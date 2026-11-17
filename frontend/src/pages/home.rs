use nsh_core::prelude::*;

pub(crate) fn home() -> impl IntoElement {
    placeholder("Home placeholder")
}

fn placeholder(label: &'static str) -> impl IntoElement {
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .child(text(label).text_size(px(17.0)))
}
