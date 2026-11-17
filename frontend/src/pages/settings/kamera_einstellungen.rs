use nsh_base::{SelectItem, theme};
use nsh_core::prelude::*;
use shared_data::CameraInterface;

use crate::Route;
use crate::pages::settings::settings_page_wrapper::settings_screen;
use crate::pages::settings::widget_select::widget_select;

/// The Settings subpage for camera options, pushed on top of the tabs.
pub(crate) fn kamera_einstellungen(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
) -> Screen {
    let focal_length = signal(CameraInterface::new().focal_length);
    settings_screen(
        routes,
        index,
        "Kamera-Einstellungen",
        "Kamera anpassen",
        view()
            .gap_6()
            .child(
                focal_length_section(focal_length)
            ),
    )
}

fn focal_length_section(focal_length: Signal<u8>) -> impl IntoElement {
    const FOCAL_LENGTHS: [u8; 5] = [28, 32, 35, 42, 50];

    let items = FOCAL_LENGTHS
        .iter()
        .map(|&length| SelectItem::new(length.to_string(), format!("{length} mm")))
        .collect::<Vec<_>>();
    
    view()
        .gap_2()
        .child(text("Brennweite")
            .text_size(px(13.0))
            .text_color(move || theme().muted_foreground),
        )
        .child(widget_select(
            "Brennweite",
            items,
            move || Some(SharedString::from(focal_length.get().to_string())),
            move |next: &Option<SharedString>| {
                let Some(length) = next.as_deref().and_then(|v| v.parse::<u8>().ok()) else {
                    return;
                };
                focal_length.set(length);
                shared_data::api::set_focal_length(length);
            },
            move || format!("{} mm", focal_length.get()),
        ))
}
