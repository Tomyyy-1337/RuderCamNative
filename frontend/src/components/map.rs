/// The route as JavaScript `[lon,lat]` pairs, without the surrounding brackets.
pub(crate) fn route_js(route: &[(f64, f64)]) -> String {
    route
        .iter()
        .map(|(lat, lon)| format!("[{lon},{lat}]"))
        .collect::<Vec<_>>()
        .join(",")
}

/// The map page's HTML; `interactive` enables pan and zoom.
pub(crate) fn map_html(interactive: bool, route: &[(f64, f64)]) -> String {
    let route = route_js(route);
    let (theme, background) = match crate::stores::theme::load() {
        // Must match the palette background in map_style.js.
        crate::stores::theme::ThemeMode::Dark => ("dark", "#121518"),
        crate::stores::theme::ThemeMode::Light => ("light", "#e0e0e0"),
    };
    include_str!("webview_assets/map.html")
        .replace("__ROUTE__", &route)
        .replace("__THEME__", theme)
        .replace("__BACKGROUND__", background)
        .replace(
            "__INTERACTIVE__",
            if interactive { "true" } else { "false" },
        )
        .replace(
            "__MAP_STYLE__",
            include_str!("webview_assets/map_style.js"),
        )
        .replace(
            "__MAPLIBRE_CSS__",
            include_str!("../../vendor/maplibre-gl.css"),
        )
        .replace(
            "__MAPLIBRE_JS__",
            include_str!("../../vendor/maplibre-gl.mjs"),
        )
        .replace(
            "__MAPLIBRE_WORKER__",
            include_str!("../../vendor/maplibre-gl-worker.mjs"),
        )
        .replace(
            "__PMTILES_JS__",
            include_str!("../../vendor/pmtiles.js"),
        )
}

