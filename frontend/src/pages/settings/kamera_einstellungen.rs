use nsh_base::{SelectItem, SliderValue, slider, theme};
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
const EXPOSURE_MIN: f64 = -3.0;
const EXPOSURE_MAX: f64 = 3.0;
const EXPOSURE_STEP: f64 = 0.3;

/// The Settings subpage for camera options, pushed on top of the tabs.
pub(crate) fn kamera_einstellungen(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    camera_interface: Signal<CameraInterface>,
) -> Screen {
    settings_screen(
        routes,
        index,
        "Kamera-Einstellungen",
        "Kamera anpassen",
        view()
            .gap_6()
            .child(enum_section("Brennweite", FOCAL_LENGTH_OPTIONS, camera_interface, |c| c.focal_length, move |v: u8| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_focal_length(v).await;
                });
            }))
            .child(enum_section("HDR", HDR_OPTIONS, camera_interface, |c| c.hdr_enabled, move |v: bool| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_hdr_active(v).await;
                });
            }))
            .child(enum_section("Bitrate", BITRATE_OPTIONS, camera_interface, |c| c.bitrate, move |v: u32| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_bitrate(v).await;
                });
            }))
            .child(enum_section("Fokusmodus", FOCUS_MODE_OPTIONS, camera_interface, |c| c.focus_mode, move |v: FocusMode| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_focus_mode(v).await;
                });
            }))
            .child(enum_section("Belichtungsmessung", METERING_OPTIONS, camera_interface, |c| c.metering_mode, move |v: Metering| {
                nsh_core::element::spawn(async move {
                    let _ = shared_data::api::set_metering_mode(v).await;
                });
            }))
            .child(exposure_compensation_section(camera_interface)),
    )
}

// The local signal follows the drag; the server is only told the value on release.
fn exposure_compensation_section(camera_interface: Signal<CameraInterface>) -> impl IntoElement {
    let exposure = signal(camera_interface.get().exposure_compenstion as f64);

    // One tick per step; ticks on every half unit are taller.
    let tick_count = ((EXPOSURE_MAX - EXPOSURE_MIN) / EXPOSURE_STEP).round() as usize;
    let mut ticks = view()
        .flex_row()
        .justify_between()
        .items_center()
        .h(px(10.0))
        .px(px(8.0));
    for i in 0..=tick_count {
        let value = EXPOSURE_MIN + i as f64 * EXPOSURE_STEP;
        let major = ((value * 2.0).round() - value * 2.0).abs() < 1e-6;
        ticks = ticks.child(
            view()
                .w(px(2.0))
                .h(px(if major { 10.0 } else { 5.0 }))
                .rounded_full()
                .bg(move || theme().muted_foreground),
        );
    }

    view()
        .gap_2()
        .child(
            view()
                .flex_row()
                .justify_between()
                .items_center()
                .child(
                    text("Belichtungskorrektur").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(13.0))
                        .text_color(move || theme().muted_foreground),
                )
                .child(
                    text(move || format!("{:+.1}", exposure.get())).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(13.0))
                        .tabular_nums()
                        .text_color(move || theme().primary),
                ),
        )
        .child(
            slider()
                .value(move || SliderValue::Single(exposure.get()))
                .min(EXPOSURE_MIN)
                .max(EXPOSURE_MAX)
                .step(EXPOSURE_STEP)
                .accessibility_label("Belichtungskorrektur")
                .on_change(move |next: &SliderValue| {
                    if let SliderValue::Single(v) = next {
                        exposure.set(*v);
                    }
                })
                .on_change_end(move |next: &SliderValue| {
                    let SliderValue::Single(v) = next else {
                        return;
                    };
                    let v = *v as f32;
                    nsh_core::element::spawn(async move {
                        let _ = shared_data::api::set_exposure_compensation(v).await;
                    });
                })
                .flex_row()
                .items_center()
                .track_style(|s| s.h(px(8.0)).flex_1().px(px(8.0)).rounded_full().bg(move || theme().muted))
                .indicator_style(|s| s.h_full())
                .range_style(|s| s.ml(-px(8.0)).rounded_full().bg(move || theme().primary))
                .thumb_style(|s| {
                    s.w(px(20.0))
                        .h(px(20.0))
                        .rounded_full()
                        .bg(move || theme().muted_foreground)
                }),
        )
        .child(ticks)
}

fn enum_section<T>(
    accessibility_label: &'static str,
    options: &'static [(T, &'static str)],
    camera_interface: Signal<CameraInterface>,
    select: fn(&CameraInterface) -> T,
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
            text(accessibility_label).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(13.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(widget_select(
            accessibility_label,
            items,
            move || {
                let current = select(&camera_interface.get());
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
                on_change(selected);
            },
            move || {
                let current = select(&camera_interface.get());
                options
                    .iter()
                    .find(|(v, _)| *v == current)
                    .map(|(_, label)| label.to_string())
                    .unwrap_or_default()
            },
        ))
}
