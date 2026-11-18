use nsh_core::{element::ImageSrc, prelude::*};

/// Battery icon and percentage, shown while the socket is connected.
pub fn battery_indicator(percentage: Signal<u8>, connected: Signal<bool>) -> impl IntoElement {
    when(move || connected.get(), move || battery_view(percentage))
}

fn battery_view(percentage: Signal<u8>) -> impl IntoElement {
    view()
        .flex_row()
        .items_center()
        .gap_2()
        .px_2()
        .text_color(move || battery_color(percentage.get()))
        .child(
            svg(move || ImageSrc::svg_markup(&battery_svg(percentage.get())))
                .w(px(28.0))
                .h(px(28.0)),
        )
        .child(
            text(move || format!("{}%", percentage.get()))
                .text_size(px(13.0))
                .font_weight(FontWeight::BOLD),
        )
}

fn battery_color(percentage: u8) -> Rgba {
    match percentage {
        0..=14 => rgb(0xef4444),
        15..=30 => rgb(0xf97316),
        31..=50 => rgb(0xeab308),
        _ => rgb(0x22c55e),
    }
}

/// The battery outline in `currentColor` with the charge level filled in.
fn battery_svg(percentage: u8) -> String {
    let level = u8::min(12, percentage / 8);
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.8"><rect x="3" y="7" width="16" height="10" rx="2"/><rect x="19" y="10" width="2" height="4" rx="1"/><rect x="5" y="9" width="{level}" height="6" rx="1" fill="currentColor"/></svg>"#
    )
}
