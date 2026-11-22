use nsh_base::{button, theme};
use nsh_core::{element::ImageSrc, prelude::*};

/// Eye button: filled while `visible`, outlined with a slash otherwise.
pub fn eye_toggle(
    visible: impl Fn() -> bool + Clone + 'static,
    on_press: impl Fn() + 'static,
) -> impl IntoElement {
    let visible_bg = visible.clone();
    let visible_border = visible.clone();
    let visible_color = visible.clone();
    let visible_icon = visible.clone();

    button()
        .w(px(32.0))
        .h(px(32.0))
        .rounded(px(6.0))
        .items_center()
        .justify_center()
        .border_1()
        .border_color(move || {
            if visible_border() { AdaptiveColor::TRANSPARENT } else { theme().border }
        })
        .bg(move || {
            if visible_bg() { theme().primary.opacity(0.2) } else { AdaptiveColor::TRANSPARENT }
        })
        .text_color(move || {
            if visible_color() { theme().foreground } else { theme().foreground.opacity(0.6) }
        })
        .active(|s| s.scale(0.95))
        .on_press(on_press)
        .child(
            svg(move || ImageSrc::svg_markup(if visible_icon() { EYE_OPEN_SVG } else { EYE_CLOSED_SVG }))
                .w(px(16.0))
                .h(px(16.0)),
        )
}

const EYE_OPEN_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/></svg>"#;

const EYE_CLOSED_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7Z"/><circle cx="12" cy="12" r="3"/><line x1="2" y1="2" x2="22" y2="22"/></svg>"#;
