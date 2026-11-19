use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::pages::components::map::{ROUTE, map_html};

/// The full-screen route map, shown in place of the tabs; pan and zoom are enabled.
pub(crate) fn karte_view(karte_open: Signal<bool>) -> impl IntoElement {
    let map_ready = signal(false);
    view()
        .flex_1()
        .p_safe_area()
        .gap_4()
        .bg(move || theme().background)
        .child(
            view()
                .flex_1()
                .w_full()
                .rounded(px(10.0))
                .overflow_hidden()
                .child(
                    web_view_html(map_html(true, &ROUTE), None)
                        .bg(move || theme().background)
                        .opacity(move || if map_ready.get() { 1.0_f32 } else { 0.0_f32 })
                        .on_message(move |data: &str| {
                            if data == "map ready" || data.starts_with("error") {
                                map_ready.set(true);
                            }
                        })
                        .w_full()
                        .h_full(),
                ),
        )
        .child(
            view()
                .absolute()
                .bottom(px(0.0))
                .right(px(0.0))
                .p_safe_area()
                .child(
                    button()
                        .m_4()
                        .w(px(40.0))
                        .h(px(40.0))
                        .rounded(px(20.0))
                        .items_center()
                        .justify_center()
                        .bg(move || theme().muted)
                        .on_press(move || karte_open.set(false))
                        .child(
                            text("⛶")
                                .text_size(px(18.0))
                                .text_color(move || theme().foreground),
                        ),
                ),
        )
}
