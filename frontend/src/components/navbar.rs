use nsh_core::prelude::*;
use nsh_base::{tabs, tab, theme};

use crate::{PAGES, Page};

pub fn navbar(page: Signal<Page>) -> impl IntoElement {
    let mut bar = tabs()
        .flex_row()
        .p_2()
        .rounded(px(10.0))
        .bg(move || theme().muted);
    for (item, label) in PAGES {
        bar = bar.child(
            tab()
                .flex_1()
                .items_center()
                .py_2()
                .rounded(px(8.0))
                .bg(move || if page.get() == item { theme().primary } else { theme().muted })
                .selected(move || page.get() == item)
                .on_press(move || page.set(item))
                .child(text(label).allow_font_scaling(false).text_size(px(16.0)).text_color(move || {
                    if page.get() == item {
                        theme().primary_foreground
                    } else {
                        theme().foreground
                    }
                })),
        );
    }
    bar
}