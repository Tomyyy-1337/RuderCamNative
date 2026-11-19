use nsh_base::{button, nav_page, theme};
use nsh_core::prelude::*;
use shared_data::FinishedFahrt;

use crate::Route;
use crate::pages::components::map::map_html;
use crate::pages::settings::settings_page_wrapper::settings_page_wrapper;
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
    let preview_points = points.clone();
    let delete_armed = signal(false);

    nav_page()
        .header_visible(false)
        .animation(ScreenAnimation::None)
        .on_dismiss(move || routes.update(|r| r.truncate(index.get_untracked())))
        .child(
            view()
                .flex_1()
                .child(when(move || !map_open.get(), move || {
                    settings_page_wrapper(
                        format!("Fahrt am {}", fahrt.client_start_time.date_string()),
                        format!("{} Uhr", fahrt.client_start_time.hour_minute_string()),
                        move || {
                            routes.update(|r| {
                                r.pop();
                            })
                        },
                        scroll()
                            .flex_1()
                            .min_h(px(0.0))
                            .child(
                                view()
                                    .gap_6()
                                    .child(route_section(preview_points.clone(), map_open))
                                    .child(distance_card(fahrt.distance_traveled))
                                    .child(stats(fahrt.clone()))
                                    .child(delete_button(delete_armed, move || {
                                        fahrtenbuch.update(|book| book.remove_entry(id));
                                        routes.update(|r| r.truncate(index.get_untracked()));
                                    })),
                            ),
                    )
                }))
                .child(when(move || map_open.get(), move || {
                    fullscreen_map(map_open, points.clone())
                })),
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
            text("Distanz")
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(format!("{:.1} km", distance))
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
            text(label)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            text(value)
                .text_size(px(20.0))
                .text_color(move || theme().card_foreground),
        )
}

fn route_section(points: Vec<(f64, f64)>, map_open: Signal<bool>) -> impl IntoElement {
    let map_idle_count = signal(0_u32);
    let section = view()
        .w_full()
        .h(px(320.0))
        .rounded(px(10.0))
        .overflow_hidden()
        .bg(move || {
            if map_idle_count.get() > 0 {
                theme().background
            } else {
                theme().muted
            }
        });
    let map_ready = signal(false);
    if points.is_empty() {
        section.items_center().justify_center().child(
            text("Keine Wegpunkte")
                .text_size(px(15.0))
                .text_color(move || theme().muted_foreground),
        )
    } else {
        section.child(
            web_view_html(map_html(false, &points), None)
                .bg(move || theme().background)
                .opacity(move || if map_ready.get() { 1.0_f32 } else { 0.0_f32 })
                .scrollable(false)
                .on_message(move |data: &str| {
                    if data == "open" {
                        map_open.set(true);
                    } else if data == "map idle" {
                        map_idle_count.update(|n| *n += 1);
                    } else if data == "map ready" {
                        map_ready.set(true);
                    }
                })
                .w_full()
                .h_full(),
        )
    }
}

fn fullscreen_map(map_open: Signal<bool>, points: Vec<(f64, f64)>) -> impl IntoElement {
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
                    web_view_html(map_html(true, &points), None)
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
                        .on_press(move || map_open.set(false))
                        .child(
                            text("⛶")
                                .text_size(px(18.0))
                                .text_color(move || theme().foreground),
                        ),
                ),
        )
}
