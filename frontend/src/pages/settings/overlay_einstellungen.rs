use nsh_base::{SelectItem, button, switch, theme};
use nsh_core::prelude::*;

use crate::Route;
use crate::pages::settings::settings_page_wrapper::settings_screen;
use crate::pages::settings::widget_select::widget_select;
use crate::stores::overlay_settings::{OverlayPosition, OverlaySettings};

const POSITION_OPTIONS: &[(OverlayPosition, &str)] = &[
    (OverlayPosition::Top, "Oben"),
    (OverlayPosition::Bottom, "Unten"),
];
// The switch has no look of its own, so its parts are styled here.
const SWITCH_TRACK: &str = "w-12 h-7 p-0.5 rounded-full justify-center bg-gray-400 checked:bg-green-500";
const SWITCH_THUMB: &str = "w-6 h-6 rounded-full bg-white checked:translate-x-5";

/// The Settings subpage for the overlay: visibility, position and element order.
pub(crate) fn overlay_einstellungen(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    overlay: Signal<OverlaySettings>,
) -> Screen {
    settings_screen(
        routes,
        index,
        "Overlay-Einstellungen",
        "Overlay anpassen",
        view()
            .gap_6()
            .child(show_overlay_row(overlay))
            .child(position_section(overlay))
            .child(elements_link(routes)),
    )
}

fn section_label(label: &'static str) -> impl IntoElement {
    text(label)
        .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
        .text_size(px(13.0))
        .text_color(move || theme().muted_foreground)
}

fn show_overlay_row(overlay: Signal<OverlaySettings>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .justify_between()
        .child(
            text("Overlay anzeigen")
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(16.0))
                .text_color(move || theme().foreground),
        )
        .child(
            switch()
                .checked(move || overlay.get().show_overlay())
                .track_class(SWITCH_TRACK)
                .thumb_class(SWITCH_THUMB)
                .accessibility_label("Overlay anzeigen")
                .on_change(move |next: &bool| {
                    let next = *next;
                    overlay.update(|s| s.set_show_overlay(next));
                }),
        )
}

fn position_section(overlay: Signal<OverlaySettings>) -> impl IntoElement {
    let items = POSITION_OPTIONS
        .iter()
        .enumerate()
        .map(|(i, (_, label))| SelectItem::new(i.to_string(), label.to_string()))
        .collect::<Vec<_>>();

    view()
        .gap_2()
        .child(section_label("Position"))
        .child(widget_select(
            "Position",
            items,
            move || {
                let current = overlay.get().overlay_position();
                POSITION_OPTIONS
                    .iter()
                    .position(|(v, _)| *v == current)
                    .map(|i| SharedString::from(i.to_string()))
            },
            move |next: &Option<SharedString>| {
                let Some(index) = next.as_deref().and_then(|v| v.parse::<usize>().ok()) else {
                    return;
                };
                let (selected, _) = POSITION_OPTIONS[index];
                overlay.update(|s| s.set_overlay_position(selected));
            },
            move || {
                let current = overlay.get().overlay_position();
                POSITION_OPTIONS
                    .iter()
                    .find(|(v, _)| *v == current)
                    .map(|(_, label)| label.to_string())
                    .unwrap_or_default()
            },
        ))
}

fn elements_link(routes: Signal<Vec<Route>>) -> impl IntoElement {
    button()
        .w_full()
        .flex_row()
        .items_center()
        .justify_between()
        .px_5()
        .py_4()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .on_press(move || routes.update(|r| r.push(Route::OverlayElemente)))
        .active(|s| s.scale(0.95))
        .child(
            text("Elemente")
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(16.0))
                .text_color(move || theme().foreground),
        )
        .child(
            text(">")
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(16.0))
                .text_color(move || theme().muted_foreground),
        )
}

