use nsh_core::prelude::*;
use nsh_base::theme;

use crate::icons::{battery::battery_indicator, gps::gps_indicator};

pub fn main_title_bar(
    battery_state: Signal<u8>,
    gps_count: Signal<u8>,
    socket_connected: Signal<bool>,
) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .h(px(50.0))
        .px_2()
        .py_3()
        .rounded(px(10.0))
        .bg(move || theme().muted)
        .child(
            view()
                .flex_1()
                .items_start()
                .child(battery_indicator(battery_state, socket_connected)),
        )
        .child(text("Ruder Cam Beta").allow_font_scaling(false).text_size(px(20.0)))
        .child(
            view()
                .flex_1()
                .items_end()
                .child(gps_indicator(gps_count, socket_connected)),
        )
}