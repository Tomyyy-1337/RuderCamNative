use std::{env, fs, path::PathBuf};

const ASSETS: [(&str, &str); 3] = [
    (
        "https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.css",
        "maplibre-gl.css",
    ),
    (
        "https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.js",
        "maplibre-gl.js",
    ),
    (
        "https://unpkg.com/pmtiles@3.2.0/dist/pmtiles.js",
        "pmtiles.js",
    ),
];

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is set by cargo"));

    for (url, file) in ASSETS {
        let path = out_dir.join(file);
        if path.exists() {
            continue;
        }
        let body = ureq::get(url)
            .call()
            .unwrap_or_else(|e| panic!("failed to download {url}: {e}"))
            .into_body()
            .read_to_string()
            .unwrap_or_else(|e| panic!("failed to read {url}: {e}"));
        fs::write(&path, body).unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
    }
}
