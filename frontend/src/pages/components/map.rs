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
    MAP_HTML
        .replace("__ROUTE__", &route)
        .replace(
            "__INTERACTIVE__",
            if interactive { "true" } else { "false" },
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

const MAP_HTML: &str = r##"<!doctype html>
<html>
<head>
<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1">
<style>__MAPLIBRE_CSS__</style>
<script>__MAPLIBRE_JS__</script>
<script>__PMTILES_JS__</script>
<style>html,body,#map{margin:0;height:100%;width:100%;overflow:hidden}html,body{background:#f2efe9}#loading{position:absolute;inset:0;display:flex;align-items:center;justify-content:center;background:#f2efe9;color:#555;font:16px sans-serif}</style>
</head>
<body>
<div id="map"></div>
<div id="loading">Loading...</div>
<script>
function report(s) { if (window.nsh) window.nsh.postMessage(s); }
window.onerror = function (m) { document.getElementById("loading").textContent = "Map error: " + m; report("error: " + m); };
if (!__INTERACTIVE__) document.addEventListener("click", function () { report("open"); });
</script>
<script>
const ARCHIVE = "https://maps.passberg.net/germany.pmtiles";
const ROUTE = [__ROUTE__];
const interactive = __INTERACTIVE__;
const protocol = new pmtiles.Protocol();
maplibregl.addProtocol("pmtiles", protocol.tile);
const archive = new pmtiles.PMTiles(ARCHIVE);
protocol.add(archive);
archive.getTileJson("pmtiles://" + ARCHIVE).then(function (tilejson) {
  const layers = [{ id: "background", type: "background", paint: { "background-color": "#f2efe9" } }];
  (tilejson.vector_layers || []).forEach(function (layer) {
    layers.push({ id: layer.id + "-fill", type: "fill", source: "basemap", "source-layer": layer.id,
      filter: ["==", ["geometry-type"], "Polygon"], paint: { "fill-color": "#dcd6c8" } });
    layers.push({ id: layer.id + "-line", type: "line", source: "basemap", "source-layer": layer.id,
      filter: ["in", ["geometry-type"], ["literal", ["LineString", "Polygon"]]], paint: { "line-color": "#9a9485", "line-width": 0.8 } });
  });
  const map = new maplibregl.Map({
    container: "map",
    style: { version: 8, sources: { basemap: { type: "vector", url: "pmtiles://" + ARCHIVE } }, layers: layers },
    interactive: interactive,
    attributionControl: false
  });
  map.on("idle", function () { report("map idle"); });
  map.on("load", function () {
    report("map loaded " + innerWidth + "x" + innerHeight);
    document.getElementById("loading").remove();
    map.addSource("route", { type: "geojson", data: { type: "Feature", properties: {}, geometry: { type: "LineString", coordinates: ROUTE } } });
    map.addLayer({ id: "route", type: "line", source: "route", layout: { "line-join": "round", "line-cap": "round" }, paint: { "line-color": "#e11d48", "line-width": 5 } });
    const bounds = ROUTE.reduce(function (b, c) { return b.extend(c); }, new maplibregl.LngLatBounds(ROUTE[0], ROUTE[0]));
    map.fitBounds(bounds, { padding: 24, animate: false });
  });
  new ResizeObserver(function () { map.resize(); }).observe(document.getElementById("map"));
  document.addEventListener("visibilitychange", function () { map.resize(); });
}).catch(function (e) {
  document.getElementById("loading").textContent = "Map failed to load: " + e;
});
</script>
</body>
</html>"##;
