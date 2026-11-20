use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::Route;

pub(crate) fn einstellungen(routes: Signal<Vec<Route>>) -> impl IntoElement {
    view()
        .flex_1()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            scroll()
                .flex_1()
                .w_full()
                .child(
            view()
                .w_full()
                .pt_2()
                .gap_6()
                .child(
            view()
                .gap_1()
                .child(text("Lokale Einstellungen").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER).text_size(px(28.0)))
                .child(
                    text("Einstellungen für die App").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(14.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
        .child(settings_link("Anzeige-Einstellungen", move || {
            routes.update(|r| r.push(Route::Anzeige))
        }))
        .child(settings_link("Kamera-Einstellungen", move || {
            routes.update(|r| r.push(Route::Kamera))
        }))
                )
        )
}

fn settings_link(label: &'static str, on_press: impl Fn() + 'static) -> impl IntoElement {
    button()
        .w_full()
        .px_5()
        .py_4()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .on_press(on_press)
        .active(|s| s.scale(0.95))
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    view()
                        .flex_1()
                        .child(text(label).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER).text_size(px(16.0)))
                        .text_color(move || theme().card_foreground),
                )
                .child(
                    text("›").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(22.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
}
