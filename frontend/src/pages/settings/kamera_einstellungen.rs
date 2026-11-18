use nsh_base::{SelectItem, theme};
use nsh_core::prelude::*;
use shared_data::{CameraInterface, FocusMode, Metering};

use crate::Route;
use crate::pages::settings::settings_page_wrapper::settings_screen;
use crate::pages::settings::widget_select::widget_select;

const HDR_OPTIONS: &[(bool, &str)] = &[(true, "An"), (false, "Aus")];
const BITRATE_OPTIONS: &[(u32, &str)] = &[
    (1_600_000, "1,6 Mbit/s"),
    (3_200_000, "3,2 Mbit/s"),
    (6_400_000, "6,4 Mbit/s"),
    (12_800_000, "12,8 Mbit/s"),
];
const FOCUS_MODE_OPTIONS: &[(FocusMode, &str)] = &[(FocusMode::Auto, "Automatisch"), (FocusMode::Fixed, "Fest")];
const METERING_OPTIONS: &[(Metering, &str)] = &[(Metering::Center, "Mitte"), (Metering::Average, "Durchschnitt")];
const FOCAL_LENGTH_OPTIONS: &[(u8, &str)] = &[
    (28, "28 mm"),
    (32, "32 mm"),
    (35, "35 mm"),
    (42, "42 mm"),
    (50, "50 mm"),
];

/// The Settings subpage for camera options, pushed on top of the tabs.
pub(crate) fn kamera_einstellungen(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
) -> Screen {
    let initial = CameraInterface::new();
    let focal_length = signal(initial.focal_length);
    let hdr_enabled = signal(initial.hdr_enabled);
    let bitrate = signal(initial.bitrate);
    let focus_mode = signal(initial.focus_mode);
    let metering_mode = signal(initial.metering_mode);
    settings_screen(
        routes,
        index,
        "Kamera-Einstellungen",
        "Kamera anpassen",
        view()
            .gap_6()
            .child(enum_section("Brennweite", FOCAL_LENGTH_OPTIONS, focal_length, move |v: u8| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_focal_length(v).await;
                });
            }))
            .child(enum_section("HDR", HDR_OPTIONS, hdr_enabled, move |v: bool| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_hdr_active(v).await;
                });
            }))
            .child(enum_section("Bitrate", BITRATE_OPTIONS, bitrate, move |v: u32| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_bitrate(v).await;
                });
            }))
            .child(enum_section("Fokusmodus", FOCUS_MODE_OPTIONS, focus_mode, move |v: FocusMode| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_focus_mode(v).await;
                });
            }))
            .child(enum_section("Belichtungsmessung", METERING_OPTIONS, metering_mode, move |v: Metering| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_metering_mode(v).await;
                });
            })),
    )
}

fn enum_section<T>(
    accessibility_label: &'static str,
    options: &'static [(T, &'static str)],
    value: Signal<T>,
    on_change: impl Fn(T) + 'static,
) -> impl IntoElement
where
    T: Copy + PartialEq + 'static,
{
    let items = options
        .iter()
        .enumerate()
        .map(|(i, (_, label))| SelectItem::new(i.to_string(), label.to_string()))
        .collect::<Vec<_>>();

    view()
        .gap_2()
        .child(
            text(accessibility_label)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(widget_select(
            accessibility_label,
            items,
            move || {
                let current = value.get();
                options
                    .iter()
                    .position(|(v, _)| *v == current)
                    .map(|i| SharedString::from(i.to_string()))
            },
            move |next: &Option<SharedString>| {
                let Some(index) = next.as_deref().and_then(|v| v.parse::<usize>().ok()) else {
                    return;
                };
                let (selected, _) = options[index];
                value.set(selected);
                on_change(selected);
            },
            move || {
                let current = value.get();
                options
                    .iter()
                    .find(|(v, _)| *v == current)
                    .map(|(_, label)| label.to_string())
                    .unwrap_or_default()
            },
        ))
}
