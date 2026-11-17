use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::Route;

pub(crate) fn einstellungen(routes: Signal<Vec<Route>>) -> impl IntoElement {
    view()
        .flex_1()
        .pt_2()
        .gap_6()
        .bg(move || theme().background)
        .text_color(move || theme().foreground)
        .child(
            view()
                .gap_1()
                .child(text("Lokale Einstellungen").text_size(px(28.0)))
                .child(
                    text("Einstellungen für die App")
                        .text_size(px(14.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
        .child(
            button()
                .w_full()
                .px_5()
                .py_4()
                .rounded(px(16.0))
                .bg(move || theme().muted)
                .on_press(move || routes.update(|r| r.push(Route::Anzeige)))
                .child(
                    view()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(
                            view()
                                .flex_1()
                                .child(text("Anzeige-Einstellungen").text_size(px(16.0)))
                                .text_color(move || theme().card_foreground),
                        )
                        .child(
                            text("›")
                                .text_size(px(22.0))
                                .text_color(move || theme().muted_foreground),
                        ),
                ),
        )
}
