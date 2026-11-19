// Placeholder route, replace with the real GPS points as (latitude, longitude).
pub(crate) const ROUTE: [(f64, f64); 5] = [
    (52.5200, 13.4050),
    (52.5124, 13.4220),
    (52.5095, 13.4480),
    (52.4930, 13.4650),
    (52.4760, 13.4850),
];

/// The map page's HTML; `interactive` enables pan and zoom.
pub(crate) fn map_html(interactive: bool, route: &[(f64, f64)]) -> String {
    let route = route
        .iter()
        .map(|(lat, lon)| format!("[{lon},{lat}]"))
        .collect::<Vec<_>>()
        .join(",");
    let (theme, background) = match crate::stores::theme::load() {
        // Must match the palette background in map_style.js.
        crate::stores::theme::ThemeMode::Dark => ("dark", "#121518"),
        crate::stores::theme::ThemeMode::Light => ("light", "#e0e0e0"),
    };
    include_str!("map.html")
        .replace("__ROUTE__", &route)
        .replace("__THEME__", theme)
        .replace("__BACKGROUND__", background)
        .replace(
            "__INTERACTIVE__",
            if interactive { "true" } else { "false" },
        )
        .replace(
            "__MAP_STYLE__",
            include_str!("map_style.js"),
        )
        .replace(
            "__MAPLIBRE_CSS__",
            include_str!("../../../vendor/maplibre-gl.css"),
        )
        .replace(
            "__MAPLIBRE_JS__",
            include_str!("../../../vendor/maplibre-gl.js"),
        )
        .replace(
            "__PMTILES_JS__",
            include_str!("../../../vendor/pmtiles.js"),
        )
}

