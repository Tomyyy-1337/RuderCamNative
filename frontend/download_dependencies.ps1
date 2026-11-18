$ErrorActionPreference = "Stop"

$dir = Join-Path $PSScriptRoot "vendor"
New-Item -ItemType Directory -Force -Path $dir | Out-Null

$files = @(
    @{ Url = "https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.css"; File = "maplibre-gl.css" },
    @{ Url = "https://unpkg.com/maplibre-gl@4.7.1/dist/maplibre-gl.js"; File = "maplibre-gl.js" },
    @{ Url = "https://unpkg.com/pmtiles@3.2.0/dist/pmtiles.js"; File = "pmtiles.js" }
)

foreach ($f in $files) {
    $target = Join-Path $dir $f.File
    Write-Host "Downloading $($f.File)"
    Invoke-WebRequest -Uri $f.Url -OutFile $target -UseBasicParsing
}

Write-Host "Done: $dir"
