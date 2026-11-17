use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::pages::components::map::{ROUTE, map_html};

/// The full-screen route map, shown in place of the tabs; pan and zoom are enabled.
pub(crate) fn karte_view(karte_open: Signal<bool>) -> impl IntoElement {
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
                .child(web_view_html(map_html(true, &ROUTE), None).w_full().h_full()),
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
