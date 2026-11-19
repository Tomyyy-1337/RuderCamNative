use nsh_core::{element::ImageSrc, prelude::*};

/// Satellite icon and count, shown while the socket is connected.
pub fn gps_indicator(satellites: Signal<u8>, connected: Signal<bool>) -> impl IntoElement {
    when(move || connected.get(), move || gps_view(satellites))
}

fn gps_view(satellites: Signal<u8>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .px_2()
        .text_color(move || gps_color(satellites.get()))
        .child(
            svg(move || ImageSrc::svg_markup(GPS_SVG))
                .w(px(28.0))
                .h(px(28.0)),
        )
        .child(
            text(move || format!("{}", satellites.get()))
                .allow_font_scaling(false)
                .text_size(px(13.0))
                .font_weight(FontWeight::BOLD),
        )
}

fn gps_color(satellites: u8) -> Rgba {
    match satellites {
        0..=3 => rgb(0xef4444),
        4..=7 => rgb(0xeab308),
        _ => rgb(0x22c55e),
    }
}

const GPS_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><rect x="9" y="9" width="6" height="6" rx="1" transform="rotate(45 12 12)"/><rect x="2" y="6.5" width="3" height="11" rx="1.2"/><rect x="19" y="6.5" width="3" height="11" rx="1.2"/></svg>"#;
