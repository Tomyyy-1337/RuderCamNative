use nsh_core::prelude::*;
use nsh_base::{tabs, tab, theme};

use crate::{PAGES, Page};

pub fn navbar(page: Signal<Page>) -> impl IntoElement {
    let mut bar = tabs()
        .flex_row()
        .p_1()
        .rounded(px(10.0))
        .bg(move || theme().muted)
        .indicator_class("inset-y-1 rounded-(--radius) bg-background shadow-sm");
    for (item, label) in PAGES {
        let selected = move || page.get() == item;
        bar = bar.child(
            tab()
                .flex_1()
                .items_center()
                .py_2()
                .rounded(px(8.0))
                .selected(selected)
                .on_press(move || page.set(item))
                .child(text(label).allow_font_scaling(false).text_size(px(16.0)).text_color(move || {
                    if selected() {
                        theme().foreground
                    } else {
                        theme().muted_foreground
                    }
                })),
        );
    }
    bar
}