use nsh_core::prelude::*;

const WHEP_URL: &str = "http://192.168.50.1:8889/stream/whep";

/// The live video, shown only while the websocket is connected.
pub(crate) fn video_player(socket_connected: Signal<bool>) -> impl IntoElement {
    when(move || socket_connected.get(), move || {
        web_view_html(video_html(), None)
            .w_full()
            .aspect_ratio(16.0_f32 / 9.0)
            .rounded(px(16.0))
            .overflow_hidden()
    })
}

/// The player page; `reader.js` and the WHEP URL are inlined.
fn video_html() -> String {
    include_str!("webview_assets/video.html")
        .replace("__READER_JS__", include_str!("webview_assets/reader.js"))
        .replace("__WHEP_URL__", WHEP_URL)
}
