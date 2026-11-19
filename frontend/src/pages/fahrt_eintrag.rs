use std::time::Duration;

use nsh_base::{button, nav_page, theme};
use nsh_core::prelude::*;
use shared_data::FinishedFahrt;

use crate::Route;
use crate::pages::components::map::map_html;
use crate::pages::settings::settings_page_wrapper::settings_page_header;
use crate::stores::fahrtenbuch::Fahrtenbuch;

/// The detail screen for one Fahrtenbuch entry, pushed on top of the tabs.
pub(crate) fn fahrt_eintrag(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    fahrtenbuch: Signal<Fahrtenbuch>,
    id: usize,
    map_open: Signal<bool>,
) -> Screen {
    let fahrt = fahrtenbuch
        .get_untracked()
        .entries
        .iter()
        .find(|entry| entry.index == id)
        .map(|entry| entry.value.clone())
        .unwrap_or_default();

    let points: Vec<(f64, f64)> = fahrt
        .gps_history
        .iter()
        .map(|p| (p.latitude, p.longitude))
        .collect();
    let title = format!("Fahrt am {}", fahrt.client_start_time.date_string());
    let subtitle = format!("{} Uhr", fahrt.client_start_time.hour_minute_string());
    let delete_armed = signal(false);

    nav_page()
        .header_visible(false)
        .animation(ScreenAnimation::None)
        .on_dismiss(move || routes.update(|r| r.truncate(index.get_untracked())))
        .child(
            view()
                .flex_1()
                .items_center()
                .p_safe_area()
                .bg(move || theme().background)
                .child(
                    view()
                        .flex_1()
                        .w_full()
                        .max_w(px(840.0))
                        .px_2()
                        .pt_4()
                        .gap_6()
                        .child(when(move || !map_open.get(), move || {
                            view()
                                .gap_6()
                                .child(settings_page_header(title.clone(), subtitle.clone(), move || {
                                    routes.update(|r| {
                                        r.pop();
                                    })
                                }))
                                .child(view().w_full().h(px(1.0)).bg(move || theme().muted))
                        }))
                        .child(map_section(points, map_open))
                        .child(when(move || map_open.get(), move || {
                            view().w_full().h(px(1.0)).bg(move || theme().muted)
                        }))
                        .child(when(move || !map_open.get(), move || {
                            scroll()
                                .flex_1()
                                .min_h(px(0.0))
                                .child(
                                    view()
                                        .gap_6()
                                        .pb_6()
                                        .child(distance_card(fahrt.distance_traveled))
                                        .child(stats(fahrt.clone()))
                                        .child(delete_button(delete_armed, move || {
                                            fahrtenbuch.update(|book| book.remove_entry(id));
                                            routes.update(|r| r.truncate(index.get_untracked()));
                                        })),
                                )
                        })),
                ),
        )
}


fn distance_card(distance: f32) -> impl IntoElement {
    view()
        .w_full()
        .p_4()
        .gap_1()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .child(
            text("Distanz").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(format!("{:.1} km", distance)).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(32.0))
                .text_color(move || theme().card_foreground),
        )
}

fn stats(fahrt: FinishedFahrt) -> impl IntoElement {
    view()
        .gap_2()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(stat_cell("Startzeit", fahrt.client_start_time.hour_minute_string()))
                .child(stat_cell("Aktive Zeit", fahrt.active_time_string())),
        )
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(stat_cell("Schläge", fahrt.schlag_count.to_string()))
                .child(stat_cell("Wegpunkte", fahrt.gps_history.len().to_string())),
        )
}

fn delete_button(armed: Signal<bool>, on_confirm: impl Fn() + 'static) -> impl IntoElement {
    button()
        .w_full()
        .py_3()
        .rounded(px(16.0))
        .items_center()
        .border_2()
        .border_color(move || theme().destructive)
        .bg(move || if armed.get() { theme().destructive } else { theme().background })
        .active(|s| s.scale(0.95))
        .on_press(move || {
            if armed.get_untracked() {
                on_confirm();
            } else {
                armed.set(true);
            }
        })
        .child(
            text(move || {
                if armed.get() { "Löschen bestätigen" } else { "Fahrt löschen" }.to_string()
            })
            .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
            .text_size(px(16.0))
            .text_color(move || {
                if armed.get() { theme().background } else { theme().destructive }
            }),
        )
}

fn stat_cell(label: &'static str, value: String) -> impl IntoElement {
    view()
        .flex_1()
        .gap_0()
        .px_3()
        .py_2()
        .rounded(px(12.0))
        .bg(move || theme().muted)
        .child(
            text(label).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(value).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(20.0))
                .text_color(move || theme().card_foreground),
        )
}

fn map_hint() -> impl IntoElement {
    view()
        .absolute()
        .bottom(px(0.0))
        .right(px(0.0))
        .w_full()
        .h_full()
        .items_center()
        .justify_center()
        .child(
            view()
                .absolute()
                .bottom(px(0.0))
                .right(px(0.0))
                .w_full()
                .h_full()
                .bg(move || rgb(0x000000))
                .opacity(move || 0.6_f32),
        )
        .child(
            text("Noch mal drücken um Vollbild zu aktivieren").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(15.0))
                .text_color(move || rgb(0xffffff)),
        )
}

fn map_section(points: Vec<(f64, f64)>, map_open: Signal<bool>) -> impl IntoElement {
    let map_idle = signal(false);
    let map_ready = signal(false);
    let hint_visible = signal(false);
    let section = view()
        .w_full()
        .h(px(260.0))
        .rounded(px(10.0))
        .overflow_hidden()
        .style_when(move || map_open.get(), |s| s.flex_1())
        .bg(move || {
            if map_idle.get() {
                theme().background
            } else {
                theme().muted
            }
        });
    if points.is_empty() {
        section.items_center().justify_center().child(
            text("Keine Wegpunkte").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(15.0))
                .text_color(move || theme().muted_foreground),
        )
    } else {
        let web = element_ref();
        effect(move || {
            let open = map_open.get();
            hint_visible.set(false);
            web.post_message(if open { "fullscreen on" } else { "fullscreen off" });
        });
        section
            .child(
                web_view_html(map_html(true, &points), None)
                    .element_ref(&web)
                    .bg(move || theme().background)
                    .opacity(move || if map_ready.get() { 1.0_f32 } else { 0.0_f32 })
                    .on_message(move |data: &str| match data {
                        "map idle" => map_idle.set(true),
                        "map ready" => map_ready.set(true),
                        _ if data.starts_with("error") => map_ready.set(true),
                        _ => {}
                    })
                    .w_full()
                    .h_full(),
            )
            .child(when(move || hint_visible.get() && !map_open.get(), map_hint))
            .child(when(move || !map_open.get(), move || {
                button()
                    .absolute()
                    .bottom(px(0.0))
                    .right(px(0.0))
                    .w_full()
                    .h_full()
                    .on_press(move || {
                        if hint_visible.get_untracked() {
                            map_open.set(true);
                        } else {
                            hint_visible.set(true);
                            nsh_core::element::spawn(async move {
                                timer(Duration::from_millis(750)).await;
                                hint_visible.set(false);
                            });
                        }
                    })
                    .child(view())
            }))
            .child(
                view()
                    .absolute()
                    .bottom(px(0.0))
                    .right(px(0.0))
                    .child(
                        button()
                            .m_2()
                            .w(px(36.0))
                            .h(px(36.0))
                            .rounded(px(18.0))
                            .items_center()
                            .justify_center()
                            .bg(move || theme().muted)
                            .on_press(move || map_open.set(!map_open.get_untracked()))
                            .child(
                                text("⛶")
                                    .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                                    .text_size(px(16.0))
                                    .text_color(move || theme().foreground),
                            ),
                    ),
            )
    }
}

