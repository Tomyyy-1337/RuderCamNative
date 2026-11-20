use std::time::Duration;

use nsh_base::{button, carousel, carousel_content, carousel_item, nav_page, theme};
use nsh_core::prelude::*;
use shared_data::FinishedFahrt;

use crate::Route;
use crate::pages::components::map::{map_html, route_js};
use crate::stores::fahrtenbuch::Fahrtenbuch;

const SLIDE_WINDOW: usize = 3;

/// The detail screen: a carousel of all Fahrtenbuch entries, opened at `id`.
pub(crate) fn fahrt_eintrag(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    fahrtenbuch: Signal<Fahrtenbuch>,
    id: usize,
    map_open: Signal<bool>,
) -> Screen {
    map_open.set(false);
    // Newest first, the same order as the list.
    let ids = computed(move || {
        fahrtenbuch.with(|book| book.entries.iter().rev().map(|entry| entry.index).collect::<Vec<usize>>())
    });
    let start = fahrtenbuch.with(|book| {
        book.entries
            .iter()
            .rev()
            .position(|entry| entry.index == id)
            .unwrap_or(0) as u32
    });
    let position = signal(start);
    let points = computed(move || {
        let slide_id = ids.with(|ids| ids.get(position.get() as usize).copied());
        fahrtenbuch.with(|book| {
            book.entries
                .iter()
                .find(|entry| Some(entry.index) == slide_id)
                .map(|entry| {
                    entry
                        .value
                        .gps_history
                        .iter()
                        .map(|p| (p.latitude, p.longitude))
                        .collect::<Vec<(f64, f64)>>()
                })
                .unwrap_or_default()
        })
    });

    nav_page()
        .header_visible(false)
        .animation(ScreenAnimation::None)
        .on_dismiss(move || {
            map_open.set(false);
            routes.update(|r| r.truncate(index.get_untracked()));
        })
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
                        .style_when(move || !map_open.get(), move |s| s.px_2().pt_4())
                        .gap_4()
                        .child(map_section(routes, points, map_open))
                        .child(when(move || !map_open.get(), move || {
                            scroll()
                                .flex_1()
                                .w_full()
                                .items_center()
                                .child(
                                    view()
                                        .w_full()
                                        .gap_4()
                                        .child(nav_buttons(ids, position))
                                        .child(when(move || !map_open.get(), move || {
                                            carousel()
                                .w_full()
                                .index(move || Some(position.get()))
                                .on_change(move |i: &u32| position.set(*i))
                                .accessibility_label("Fahrten")
                                .child(carousel_content().gap_x_2().child(each(
                                    move || ids.get(),
                                    |slide_id: &usize| slide_id.to_string(),
                                    move |slide_id, _| {
                                        let id = slide_id.get_untracked();
                                        let near = computed(move || {
                                            let pos = position.get() as usize;
                                            ids.with(|ids| {
                                                ids.iter().position(|s| *s == id).is_some_and(|i| i.abs_diff(pos) <= SLIDE_WINDOW)
                                            })
                                        });
                                        carousel_item()
                                            .w_full()
                                            .items_center()
                                            .child(
                                                when(move || near.get(), move || {
                                                    fahrt_slide(routes, index, fahrtenbuch, id, position)
                                                })
                                                .otherwise(|| view().w_full()),
                                            )
                                    },
                                )))
                        })),
                                )
                        }))
                )
        )
}

/// The previous and next buttons, shown below the map and outside the carousel.
fn nav_buttons(ids: Memo<Vec<usize>>, position: Signal<u32>) -> impl IntoElement {
    view()
        .flex_row()
        .w_full()
        .max_w(px(840.0))
        .gap_2()
        .child(nav_button("Nächste", move || position.get() == 0, move || {
            position.set(position.get_untracked().saturating_sub(1));
        }))
        .child(nav_button("Vorherige", move || position.get() as usize + 1 >= ids.with(Vec::len), move || {
            let last = ids.with(Vec::len).saturating_sub(1) as u32;
            if position.get_untracked() < last {
                position.set(position.get_untracked() + 1);
            }
        }))
}

/// A labelled pill button, used for slide navigation.
fn nav_button(
    label: &'static str,
    disabled: impl Fn() -> bool + 'static,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    button()
        .flex_1()
        .h(px(40.0))
        .px_4()
        .rounded(px(20.0))
        .items_center()
        .justify_center()
        .bg(move || theme().muted)
        .active(|s| s.scale(0.95))
        .disabled(move || disabled())
        .disabled_style(|s| s.opacity(0.4))
        .on_press(move || on_press())
        .child(
            text(label)
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(15.0))
                .text_color(move || theme().foreground),
        )
}

/// One entry's stats and delete button, shown as a slide of the carousel.
fn fahrt_slide(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    fahrtenbuch: Signal<Fahrtenbuch>,
    id: usize,
    position: Signal<u32>,
) -> impl IntoElement {
    let fahrt = fahrtenbuch.with(|book| {
        book.entries
            .iter()
            .find(|entry| entry.index == id)
            .map(|entry| entry.value.clone())
            .unwrap_or_default()
    });

    let delete_armed = signal(false);

    view()
        .flex_1()
        .w_full()
        .max_w(px(840.0))
        .child(
            view()
                .child(
                    view()
                        .gap_4()
                        .pb_6()
                        .child(heading_card(
                            format!("Fahrt am {}", fahrt.client_start_time.date_string()),
                            format!("{} Uhr", fahrt.client_start_time.hour_minute_string()),
                        ))
                        .child(distance_card(fahrt.distance_traveled))
                        .child(stats(fahrt.clone()))
                        .child(delete_button(delete_armed, move || {
                            fahrtenbuch.update(|book| book.remove_entry(id));
                            let remaining = fahrtenbuch.get_untracked().entries.len() as u32;
                            if remaining == 0 {
                                routes.update(|r| r.truncate(index.get_untracked()));
                            } else {
                                position.set(position.get_untracked().min(remaining - 1));
                            }
                        })),
                ),
        )
}

fn heading_card(title: String, subtitle: String) -> impl IntoElement {
    view()
        .w_full()
        .p_4()
        .gap_1()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .child(
            text(title)
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(24.0))
                .text_color(move || theme().card_foreground),
        )
        .child(
            text(subtitle)
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
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

/// The one map, shared by all slides; it follows the current slide's route.
fn map_section(
    routes: Signal<Vec<Route>>,
    points: Memo<Vec<(f64, f64)>>,
    map_open: Signal<bool>,
) -> impl IntoElement {
    let map_idle = signal(false);
    let map_ready = signal(false);
    let hint_visible = signal(false);
    let web = element_ref();
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
        })
        .child(
            web_view_html(map_html(true, &points.with(Vec::clone)), None)
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
        .child(when(move || points.with(Vec::is_empty), empty_points))
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
        }))
        .child(
            view()
                .absolute()
                .bottom(px(0.0))
                .right(px(0.0))
                .child(
                    button()
                        .m_2()
                        .w(px(46.0))
                        .h(px(46.0))
                        .rounded(px(23.0))
                        .items_center()
                        .justify_center()
                        .bg(move || theme().muted)
                        .on_press(move || map_open.set(!map_open.get_untracked()))
                        .child(
                            text("⛶")
                                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                                .text_size(px(22.0))
                                .text_color(move || theme().foreground),
                        ),
                ),
        )
        .child(when(move || !map_open.get(), move || close_button(routes)));

    effect(move || {
        let _ = map_open.get();
        hint_visible.set(false);
    });
    effect(move || {
        let route = points.get();
        if map_ready.get() {
            web.inject_java_script(&format!("window.setRoute([{}]);", route_js(&route)));
        }
    });
    section
}

/// The close button in the top right of the map, shown only when not fullscreen.
fn close_button(routes: Signal<Vec<Route>>) -> impl IntoElement {
    view()
        .absolute()
        .top(px(0.0))
        .right(px(0.0))
        .child(
            button()
                .m_2()
                .w(px(46.0))
                .h(px(46.0))
                .rounded(px(23.0))
                .items_center()
                .justify_center()
                .bg(move || theme().muted)
                .active(|s| s.scale(0.95))
                .on_press(move || {
                    routes.update(|r| {
                        r.pop();
                    })
                })
                .child(
                    text("✕")
                        .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(18.0))
                        .text_color(move || theme().foreground),
                ),
        )
}

fn empty_points() -> impl IntoElement {
    view()
        .absolute()
        .bottom(px(0.0))
        .right(px(0.0))
        .w_full()
        .h_full()
        .items_center()
        .justify_center()
        .bg(move || theme().muted)
        .child(
            text("Keine Wegpunkte").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(15.0))
                .text_color(move || theme().muted_foreground),
        )
}


