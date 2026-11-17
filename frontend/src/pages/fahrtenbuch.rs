use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::Route;
use crate::stores::fahrtenbuch::{Fahrtenbuch, FinishedFahrt, Indexed};

#[derive(Clone, PartialEq)]
struct Summary {
    count: usize,
    distance: f32,
}

pub(crate) fn fahrtenbuch(routes: Signal<Vec<Route>>, store: Signal<Fahrtenbuch>) -> impl IntoElement {
    view().flex_1().pt_4().child(entry_list(routes, store))
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
            text(value)
                .text_size(px(24.0))
                .text_color(move || theme().card_foreground),
        )
        .child(
            text(label)
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
        )
}

fn entry_list(routes: Signal<Vec<Route>>, store: Signal<Fahrtenbuch>) -> impl IntoElement {
    let list_rows = computed(move || rows(&store.get()));
    let list = virtual_list(move || list_rows.with(Vec::len), move |at| {
        let row = computed(move || list_rows.with(|r| r[at.index()].clone()));
        let is_month = computed(move || row.with(|r| r.is_month));
        when(is_month, move || {
            view().px_1().pt_2().child(
                text(move || row.with(|r| r.title.clone()))
                    .text_size(px(16.0))
                    .text_color(move || theme().muted_foreground),
            )
        })
        .otherwise(move || {
            button()
                .w_full()
                .px_3()
                .py_3()
                .rounded(px(16.0))
                .bg(move || theme().muted)
                .on_press(move || routes.update(|r| r.push(Route::FahrtEintrag(row.with(|r| r.id)))))
                .child(
                    view()
                        .gap_2()
                        .child(
                            text(move || row.with(|r| r.title.clone()))
                                .text_size(px(16.0))
                                .text_color(move || theme().card_foreground),
                        )
                        .child(
                            view()
                                .flex_row()
                                .gap_2()
                                .child(pill(move || row.with(|r| r.distance.clone())))
                                .child(pill(move || row.with(|r| r.time.clone()))),
                        ),
                )
        })
    })
    .header(view().gap_4().child(overview(store)).child(divider()))
    .keys(move || list_rows.with(|r| r.iter().map(|r| r.key.clone()).collect::<Vec<_>>()))
    .estimated_item_size(px(72.0))
    .row_gap(px(14.0))
    .flex_1();
    view().flex_1().rounded(px(16.0)).overflow_hidden().child(list)
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum RowKey {
    Month(String),
    Trip(usize),
    BottomSpacer,
}

#[derive(Clone, PartialEq)]
struct Row {
    key: RowKey,
    id: usize,
    is_month: bool,
    title: String,
    distance: String,
    time: String,
}

fn rows(book: &Fahrtenbuch) -> Vec<Row> {
    let mut rows = Vec::new();
    let mut last_month = None;
    for entry in book.entries.iter().rev() {
        let start = &entry.value.client_start_time;
        let month = start.year_month();
        if last_month != Some(month) {
            last_month = Some(month);
            rows.push(Row {
                key: RowKey::Month(format!("{}-{}", month.0, month.1)),
                id: 0,
                is_month: true,
                title: start.month_label(),
                distance: String::new(),
                time: String::new(),
            });
        }
        let fahrt = &entry.value;
        rows.push(Row {
            key: RowKey::Trip(entry.index),
            id: entry.index,
            is_month: false,
            title: format!(
                "Fahrt am {} um {} Uhr",
                start.date_string(),
                start.hour_minute_string(),
            ),
            distance: format!("{:.1} km", fahrt.distance_traveled),
            time: format!("{} h", fahrt.active_time_string()),
        });
    }
    rows.push(Row {
        key: RowKey::BottomSpacer,
        id: 0,
        is_month: true,
        title: String::new(),
        distance: String::new(),
        time: String::new(),
    });
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
            text(label)
                .text_size(px(14.0))
                .text_color(move || theme().muted_foreground),
        )
}
