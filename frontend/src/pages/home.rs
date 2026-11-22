use nsh_base::{button, theme};
use nsh_core::prelude::*;

use crate::components::video::video_player;

pub(crate) fn home(
    socket_connected: Signal<bool>,
    fahrt_info: Signal<Option<shared_data::FahrtInfo>>,
) -> impl IntoElement {
    view()
        .flex_1()
        .items_center()
        .justify_center()
        .gap_4()
        .child(video_player(socket_connected))
        .child(
            button()
                .w(px(200.0))
                .py_3()
                .items_center()
                .justify_center()
                .rounded(px(16.0))
                .bg(move || theme().primary)
                .active(|s| s.scale(0.95))
                .disabled(move || !socket_connected.get())
                .disabled_style(|s| s.bg(move || theme().muted).scale(1.0))
                .on_press(move || {
                    nsh_core::element::spawn(async move {
                        let now = shared_data::SerializableDateTime::from(chrono::Local::now());
                        let _ = shared_data::api::toggle_fahrt(now).await;
                    });
                })
                .child(
                    text(move || {
                        String::from(if !socket_connected.get() {
                            "Nicht verfügbar"
                        } else if fahrt_info.with(Option::is_some) {
                            "Fahrt beenden"
                        } else {
                            "Fahrt starten"
                        })
                    })
                    .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER).text_size(px(17.0))
                    .text_color(move || {
                        if socket_connected.get() { theme().primary_foreground } else { theme().muted_foreground }
                    }),
                ),
        )
}
