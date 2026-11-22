use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::Route;
use crate::pages::settings::settings_page_wrapper::settings_screen;
use crate::stores::theme::ThemeMode;

/// The Settings subpage for display options, pushed on top of the tabs.
pub(crate) fn anzeige_einstellungen(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    theme_mode: Signal<ThemeMode>,
) -> Screen {
    settings_screen(
        routes,
        index,
        "Anzeige-Einstellungen",
        "Erscheinungsbild anpassen",
        view()
            .gap_6()
            .child(
                theme_section(theme_mode)
            ),
    )
}

fn theme_section(theme_mode: Signal<ThemeMode>) -> impl IntoElement {
    view()
        .gap_2()
        .child(
            text("Design").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            view()
                .flex_row()
                .gap_1()
                .p_1()
                .rounded(px(14.0))
                .bg(move || theme().muted)
                .child(theme_option(theme_mode, ThemeMode::Light, "Hell"))
                .child(theme_option(theme_mode, ThemeMode::Dark, "Dunkel")),
        )
}

fn theme_option(
    theme_mode: Signal<ThemeMode>,
    mode: ThemeMode,
    label: &'static str,
) -> impl IntoElement {
    button()
        .flex_1()
        .px_4()
        .py_3()
        .rounded(px(10.0))
        .on_press(move || theme_mode.set(mode))
        .bg(move || {
            if theme_mode.get() == mode {
                theme().card
            } else {
                theme().muted
            }
        })
        .child(text(label)
            .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
            .text_size(px(15.0))
            .text_color(move || {
                if theme_mode.get() == mode {
                    theme().card_foreground
                } else {
                    theme().muted_foreground
                }
            }
        ))
}
