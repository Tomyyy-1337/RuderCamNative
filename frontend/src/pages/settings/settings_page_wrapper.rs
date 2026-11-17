use nsh_base::{button, theme};
use nsh_core::prelude::*;

/// Title, subtitle and a round close button, followed by the page content inside a centred column.
pub(crate) fn settings_page_wrapper(
    title: impl Into<String>,
    subtitle: impl Into<String>,
    on_close: impl Fn() + 'static,
    content: impl IntoElement,
) -> impl IntoElement {
    let title: String = title.into();
    let subtitle: String = subtitle.into();
    view()
        .flex_1()
        .items_center()
        .p_safe_area()
        .bg(move || theme().background)
        .child(
        view()
            .flex_1()
            .w_full()
            .max_w(px(840.0))
            .px_2()
            .pt_4()
            .gap_6()
            .bg(move || theme().background)
            .text_color(move || theme().foreground)
            .child(
                view()
                    .flex_row()
                    .items_center()
                    .child(
                        view()
                            .flex_1()
                            .gap_1()
                            .child(text(title).text_size(px(28.0)))
                            .child(
                                text(subtitle)
                                    .text_size(px(14.0))
                                    .text_color(move || theme().muted_foreground),
                            ),
                    )
                    .child(
                        button()
                            .w(px(40.0))
                            .h(px(40.0))
                            .rounded(px(20.0))
                            .bg(move || theme().muted)
                            .on_press(move || on_close())
                            .child(
                                view()
                                    .flex_1()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        text("✕")
                                            .text_size(px(16.0))
                                            .text_color(move || theme().foreground),
                                    ),
                            ),
                    ),
            )
            .child(content),
    )
}
