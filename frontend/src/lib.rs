//! The app's Rust UI: its root component, which `nsh run`, `nsh package`
//! and `cargo run` mount.

mod pages;
mod socket;
mod stores;
mod icons;
mod components;

use nsh_base::{nav_page, nav_stack, theme};
use nsh_core::element::Orientation;
use nsh_core::prelude::*;
use pages::{
    anzeige_einstellungen, einstellungen, fahrt_eintrag, fahrtenbuch, home, kamera_einstellungen,
    overlay_einstellungen, overlay_elemente,
};
use stores::fahrtenbuch::Fahrtenbuch;
use stores::overlay_settings::OverlaySettings;

use crate::{components::{navbar::navbar, title_bar::main_title_bar}, stores::store::Store};

#[derive(Clone, Copy, PartialEq)]
enum Page {
    Home,
    Fahrtenbuch,
    Einstellungen,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Route {
    Anzeige,
    Kamera,
    Overlay,
    OverlayElemente,
    FahrtEintrag(usize),
}

const PAGES: [(Page, &str); 3] = [
    (Page::Home, "Home"),
    (Page::Fahrtenbuch, "Fahrtenbuch"),
    (Page::Einstellungen, "Settings"),
];

pub(crate) const MAX_FONT_SCALE_MULTIPLIER: f32 = 1.15;

/// The app's UI set (`nsh.json` `"rust"`): what a TSX entry may import from
/// `treiber-nsh/native` (`README.md`, "Adding a TSX entry").
#[native_module("treiber-nsh/native")]
fn native(registry: &mut Registry) {
    registry.register::<App>();
}

/// The application's root component.
#[component]
pub fn App() -> impl IntoElement {
    let page = signal(Page::Home);
    let routes = signal(Vec::<Route>::new());
    let fahrt_map_open = signal(false);
    
    let fahrtenbuch_store = signal(Fahrtenbuch::load().unwrap_or_default());
    effect(move || fahrtenbuch_store.get().save());

    let camera_interface = signal(shared_data::CameraInterface::new());

    let overlay_settings = signal(OverlaySettings::load().unwrap_or_default());
    effect(move || overlay_settings.get().save());

    let socket_connected = signal(false);
    let battery_state = signal(0u8);
    let gps_count = signal(0u8);
    let current_stats = signal(shared_data::CurrentStats { bpm: 0.0, velocity: 0.0 });
    let fahrt_info = signal(None::<shared_data::FahrtInfo>);
    socket::start(socket_connected, camera_interface, battery_state, gps_count, fahrtenbuch_store, current_stats, fahrt_info);

    let theme_mode = signal(stores::theme::ThemeMode::load().unwrap_or_default());
    effect(move || {
        let mode = theme_mode.get();
        mode.save();
        set_color_scheme(mode.appearance());
    });

    effect(move || {
        on_back_press(move || {
            if fahrt_map_open.get_untracked() {
                fahrt_map_open.set(false);
                return true;
            }
            if !routes.get_untracked().is_empty() {
                return false;
            }
            page.set(Page::Home);
            true
        });
    });

    nav_stack()
        .flex_1()
        .child(
            nav_page()
                .key("tabs")
                .supported_orientations([Orientation::Portrait])
                .header_visible(false)
                .status_bar(move || {
                    let style = match theme_mode.get() {
                        stores::theme::ThemeMode::Light => StatusBarStyle::Dark,
                        stores::theme::ThemeMode::Dark => StatusBarStyle::Light,
                    };
                    StatusBarSetting::default().foreground(style)
                })
                .child(
                    view()
                        .flex_1()
                        .bg(move || theme().background)
                        .text_color(move || theme().foreground)
                        .child(tabs_content(page,routes,fahrtenbuch_store,battery_state,gps_count,socket_connected,fahrt_info)),
                ),
        )
        .child(each(
            move || routes.get(),
            |route: &Route| format!("{route:?}"),
            move |route, index| match route.get_untracked() {
                Route::Anzeige => anzeige_einstellungen(routes, index, theme_mode),
                Route::Kamera => kamera_einstellungen(routes, index, camera_interface),
                Route::Overlay => overlay_einstellungen(routes, index, overlay_settings),
                Route::OverlayElemente => overlay_elemente(routes, index, overlay_settings),
                Route::FahrtEintrag(id) => {
                    fahrt_eintrag(routes, index, fahrtenbuch_store, id, fahrt_map_open, theme_mode)
                }
            },
        ))
}

fn tabs_content(
    page: Signal<Page>,
    routes: Signal<Vec<Route>>,
    fahrtenbuch_store: Signal<Fahrtenbuch>,
    battery_state: Signal<u8>,
    gps_count: Signal<u8>,
    socket_connected: Signal<bool>,
    fahrt_info: Signal<Option<shared_data::FahrtInfo>>,
) -> impl IntoElement {
    view().flex_1().p_safe_area().bg(move || theme().background).child(
        view().flex_1().items_center().px_2().pt_2().child(
            view()
                .flex_1()
                .w_full()
                .max_w(px(840.0))
                .gap_4()
                .child(main_title_bar(battery_state, gps_count, socket_connected))
                .child(navbar(page))
                .child(
                    view()
                        .flex_1()
                        .child(when(move || page.get() == Page::Home, move || home(socket_connected, fahrt_info)))
                        .child(when(
                            move || page.get() == Page::Fahrtenbuch,
                            move || fahrtenbuch(routes, fahrtenbuch_store),
                        ))
                        .child(when(
                            move || page.get() == Page::Einstellungen,
                            move || einstellungen(routes),
                        )),
                ),
        ),
    )
}
