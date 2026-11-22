use std::cell::Cell;
use std::rc::Rc;

use nsh_base::theme;
use nsh_core::prelude::*;

use crate::Route;
use crate::icons::eye::eye_toggle;
use crate::pages::settings::settings_page_wrapper::settings_screen;
use crate::stores::overlay_settings::{OverlayElement, OverlaySettings};

const ROW_HEIGHT: f32 = 56.0;

/// The Settings subpage for the overlay elements: order and visibility.
pub(crate) fn overlay_elemente(
    routes: Signal<Vec<Route>>,
    index: ReadSignal<usize>,
    overlay: Signal<OverlaySettings>,
) -> Screen {
    settings_screen(
        routes,
        index,
        "Overlay-Elemente",
        "Elemente anordnen",
        view().gap_2().child(each(
            move || overlay.get().elements().to_vec(),
            |element: &OverlayElement| format!("{:?}", element.element_type),
            move |element, index| element_row(overlay, element, index),
        )),
    )
}

/// One element row. Dragging it vertically moves it through the list; the
/// eye button toggles its visibility.
fn element_row(
    overlay: Signal<OverlaySettings>,
    element: ReadSignal<OverlayElement>,
    index: ReadSignal<usize>,
) -> impl IntoElement {
    let label = element.get_untracked().element_type.label();
    let origin = Rc::new(Cell::new(0usize));
    let origin_on_grant = origin.clone();
    let drag_offset = signal(0.0_f32);

    view()
        .h(px(ROW_HEIGHT))
        .w_full()
        .flex_row()
        .items_center()
        .gap_3()
        .px_4()
        .rounded(px(12.0))
        .bg(move || theme().muted)
        .translate_y(move || px(drag_offset.get()))
        .claim_touches(TouchClaim::on_pan(PanActivation {
            active_offset_x: None,
            active_offset_y: Some(px(8.0)),
            fail_offset_x: Some(px(8.0)),
            fail_offset_y: None,
        }))
        .on_responder_grant(move |_: &ResponderEvent| origin_on_grant.set(index.get_untracked()))
        .on_responder_move(move |event: &ResponderEvent| {
            let len = overlay.get_untracked().elements().len() as isize;
            let origin_index = origin.get();
            let min_dy = -(origin_index as f32) * ROW_HEIGHT;
            let max_dy = (len - 1 - origin_index as isize) as f32 * ROW_HEIGHT;
            let dy = event.gesture.delta.y.0.clamp(min_dy, max_dy);
            let steps = (dy / ROW_HEIGHT).round() as isize;
            let target = (origin_index as isize + steps).clamp(0, len - 1) as usize;
            let from = index.get_untracked();
            if from != target {
                overlay.update(|s| s.move_element(from, target));
            }
            let moved = target as isize - origin.get() as isize;
            drag_offset.set(dy - moved as f32 * ROW_HEIGHT);
        })
        .on_responder_release(move |_: &ResponderEvent| drag_offset.set(0.0))
        .on_responder_terminate(move |_: &ResponderEvent| drag_offset.set(0.0))
        .child(
            text("⋮⋮")
                .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                .text_size(px(16.0))
                .text_color(move || theme().muted_foreground),
        )
        .child(
            view().flex_1().child(
                text(label)
                    .max_font_size_multiplier(crate::MAX_FONT_SCALE_MULTIPLIER)
                    .text_size(px(16.0))
                    .text_color(move || theme().card_foreground),
            ),
        )
        .child(eye_toggle(
            move || element.get().visible,
            move || {
                let visible = element.get_untracked().visible;
                overlay.update(|s| s.set_element_visibility(index.get_untracked(), !visible));
            },
        ))
}
