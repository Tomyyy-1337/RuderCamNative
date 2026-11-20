use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::Route;
use crate::stores::fahrtenbuch::Fahrtenbuch;

#[derive(Clone, PartialEq)]
struct Summary {
    count: usize,
    distance: f32,
}

pub(crate) fn fahrtenbuch(routes: Signal<Vec<Route>>, store: Signal<Fahrtenbuch>) -> impl IntoElement {
    view()
        .flex_1()
        .pt_4()
        .child(entry_list(routes, store))
}

fn overview(store: Signal<Fahrtenbuch>) -> impl IntoElement {
    each(
        move || {
            let book = store.get();
            vec![Summary {
                count: book.entries.len(),
                distance: book.entries.iter().map(|entry| entry.value.distance_traveled).sum::<f32>(),
            }]
        },
        |summary: &Summary| format!("{}-{}", summary.count, summary.distance),
        move |summary, _| overview_card(summary.get_untracked()),
    )
}

fn overview_card(summary: Summary) -> impl IntoElement {
    view()
        .w_full()
        .gap_2()
        .child(
            view()
                .flex_row()
                .gap_2()
                .child(stat_box(summary.count.to_string(), "Fahrten".to_string()))
                .child(stat_box(format!("{:.1}", summary.distance), "Gesamt km".to_string())),
        )
}

fn stat_box(value: String, label: String) -> impl IntoElement {
    view()
        .flex_1()
        .px_4()
        .py_3()
        .gap_1()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .child(
            text(value).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(24.0))
                .text_color(move || theme().card_foreground),
        )
        .child(
            text(label).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
        )
}

fn entry_list(routes: Signal<Vec<Route>>, store: Signal<Fahrtenbuch>) -> impl IntoElement {
    let list_rows = computed(move || rows(&store.get()));
    let list = virtual_list(move || list_rows.with(Vec::len), move |at| {
        let row = computed(move || list_rows.with(|r| r[at.index()].clone()));
        when(computed(move || row.with(|r| matches!(r, Row::Month(_)))), move || {
            month_label(row)
        })
        .otherwise(move || {
            fahrtenbuch_entry(routes, row)
        })
    })
    .header(view().gap_4().child(overview(store)).child(divider()))
    .footer(view().h(px(24.0)))
    .keys(move || list_rows.with(Vec::clone))
    .estimated_item_size(px(72.0))
    .row_gap(px(14.0))
    .flex_1();

    view()
        .flex_1().rounded(px(16.0)).overflow_hidden()
        .child(list)
}

fn fahrtenbuch_entry(routes: Signal<Vec<Route>>, row: Memo<Row>) -> nsh_base::ButtonElement {
    button()
        .w_full()
        .px_5()
        .py_3()
        .rounded(px(16.0))
        .bg(move || theme().muted)
        .active(|s| s.scale(0.95))
        .on_press(move || {
            if let Some(id) = row.with(Row::trip_id) {
                routes.update(|r| r.push(Route::FahrtEintrag(id)));
            }
        })
        .child(
            view()
                .flex_row()
                .items_center()
                .gap_3()
                .child(
                    view()
                        .flex_1()
                        .gap_2()
                        .child(
                            text(move || row.with(Row::title)).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                                .text_size(px(16.0))
                                .text_color(move || theme().card_foreground),
                        )
                        .child(
                            view()
                                .flex_row()
                                .gap_2()
                                .child(pill(move || row.with(Row::distance)))
                                .child(pill(move || row.with(Row::time))),
                        ),
                )
                .child(
                    text("›").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(22.0))
                        .text_color(move || theme().muted_foreground),
                ),
        )
}

fn month_label(row: Memo<Row>) -> View {
    view().px_1().pt_2().child(
        text(move || row.with(Row::title)).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
            .text_size(px(16.0))
            .text_color(move || theme().muted_foreground),
    )
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Row {
    Month(String),
    Trip { id: usize, title: String, distance: String, time: String },
}

impl Row {
    fn title(&self) -> String {
        match self {
            Row::Month(title) | Row::Trip { title, .. } => title.clone(),
        }
    }

    fn trip_id(&self) -> Option<usize> {
        match self {
            Row::Trip { id, .. } => Some(*id),
            Row::Month(_) => None,
        }
    }

    fn distance(&self) -> String {
        match self {
            Row::Trip { distance, .. } => distance.clone(),
            Row::Month(_) => String::new(),
        }
    }

    fn time(&self) -> String {
        match self {
            Row::Trip { time, .. } => time.clone(),
            Row::Month(_) => String::new(),
        }
    }
}

fn rows(book: &Fahrtenbuch) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut last_month = None;
    for entry in book.entries.iter().rev() {
        let start = &entry.value.client_start_time;
        let month = start.year_month();
        if last_month != Some(month) {
            last_month = Some(month);
            rows.push(Row::Month(start.month_label()));
        }
        let fahrt = &entry.value;
        rows.push(Row::Trip {
            id: entry.index,
            title: format!(
                "Fahrt am {} um {} Uhr",
                start.date_string(),
                start.hour_minute_string(),
            ),
            distance: format!("{:.1} km", fahrt.distance_traveled),
            time: format!("{} h", fahrt.active_time_string()),
        });
    }
    rows
}

fn divider() -> impl IntoElement {
    view().w_full().h(px(1.0)).bg(move || theme().muted)
}

fn pill(label: impl Fn() -> String + 'static) -> impl IntoElement {
    view()
        .px_3()
        .py_1()
        .rounded(px(999.0))
        .bg(move || theme().background)
        .child(
            text(label).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
        )
}
