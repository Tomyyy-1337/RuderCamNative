use nsh_base::{SelectEntry, SelectItem, select, theme};
use nsh_core::prelude::*;

/// A themed dropdown whose closed control shows `selected_label`.
pub(crate) fn widget_select(
    accessibility_label: &'static str,
    items: Vec<SelectItem>,
    value: impl Fn() -> Option<SharedString> + 'static,
    on_change: impl Fn(&Option<SharedString>) + 'static,
    selected_label: impl Fn() -> String + 'static,
) -> impl IntoElement {
    select()
        .items(items)
        .value(value)
        .on_change(on_change)
        .accessibility_label(accessibility_label)
        .offset(px(4.0))
        .popup_style(|s| {
            s.p_1()
                .rounded_lg()
                .border_1()
                .border_color(move || theme().border)
                .bg(move || theme().popover)
                .shadow_md()
        })
        .item_class("rounded-md focus:bg-accent selected:font-medium disabled:opacity-50")
        .render_item(|item: &SelectEntry| {
            view()
                .h(px(48.0))
                .w_full()
                .flex_row()
                .items_center()
                .px_3()
                .child(
                    text(item.title.clone()).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(15.0))
                        .text_color(move || theme().popover_foreground),
                )
        })
        .trigger(
            view()
                .h(px(44.0))
                .w_full()
                .flex_row()
                .items_center()
                .justify_between()
                .px_4()
                .rounded(px(10.0))
                .bg(move || theme().muted)
                .active(|s| s.opacity(0.6))
                .child(
                    text(move || selected_label()).max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                        .text_size(px(15.0))
                        .text_color(move || theme().card_foreground),
                )
                .child(text("▾").max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER).text_color(move || theme().muted_foreground)),
        )
}
