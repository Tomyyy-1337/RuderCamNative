use std::{
    fmt::Display,
    fs, io,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    sync::{Arc, Mutex},
};

use axum::{Router, body::{Body, to_bytes}, extract::State, http::StatusCode, routing::post};
use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use tokio::runtime::LocalOptions;
use tower_http::cors::CorsLayer;

const UPDATE_DIR: &str = "update_bin";
const HASH_FILE: &str = "hash.txt";
const MAPS_DIR: &str = "maps";
const BACKEND_SERVICE: &str = "backend.service";
const MEDIAMTX_SERVICE: &str = "mediamtx.service";
const BACKEND_BINARY: &str = "server";

type UpdateResult = Result<(), (StatusCode, String)>;

#[derive(Clone)]
struct AppState {
    update_lock: Arc<Mutex<()>>,
    base_dir: PathBuf,
}

fn main() {  
    tokio::runtime::Builder::new_current_thread()
        .max_blocking_threads(4)
        .enable_all()
        .build_local(LocalOptions::default())
        .unwrap()
        .block_on(async {
            let base_dir = PathBuf::from(std::env::var("TREIBER_DIR").unwrap_or_else(|_| "/home/pi".to_string()));
            let app = Router::new()
                .route("/update", post(handle_update))
                .layer(CorsLayer::permissive())
                .with_state(AppState {
                    update_lock: Arc::new(Mutex::new(())),
                    base_dir,
                });
            
            let addr = SocketAddr::from(([0, 0, 0, 0], 8141));

            let listener = tokio::net::TcpListener::bind(addr).await?;
            axum::serve(listener, app).await
        }).expect("Failed to start the server");    
}

async fn handle_update(State(state): State<AppState>, body: Body) -> (StatusCode, String) {
    let body = match to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => return (StatusCode::BAD_REQUEST, format!("failed to read body: {e}")),
    };
    let result = tokio::task::spawn_blocking(move || {
        let Ok(_lock) = state.update_lock.try_lock() else {
            return Err((StatusCode::CONFLICT, "Update already in progress".to_string()));
        };
        install_update(&state.base_dir, &body)
    })
    .await;

    match result {
        Ok(Ok(())) => (StatusCode::OK, "Update installed successfully".to_string()),
        Ok(Err(err)) => err,
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Update task failed".to_string()),
    }
}

fn install_update(base_dir: &Path, body: &[u8]) -> UpdateResult {
    let staging = base_dir.join("update_staging");
    let result = stage_and_install(base_dir, &staging, body);
    let _ = fs::remove_dir_all(&staging);
    result
}

fn stage_and_install(base_dir: &Path, staging: &Path, body: &[u8]) -> UpdateResult {
    let _ = fs::remove_dir_all(staging);
    fs::create_dir_all(staging).map_err(internal)?;
    tar::Archive::new(GzDecoder::new(body))
        .unpack(staging)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("invalid update archive: {e}")))?;

    let update_dir = staging.join(UPDATE_DIR);
    verify_checksum(&update_dir)?;

    let treiber_dir = base_dir.join("treiber");
    stop_backend();
    clear_treiber_dir(&treiber_dir).map_err(internal)?;
    let binary = treiber_dir.join(BACKEND_BINARY);
    fs::copy(update_dir.join("backend"), &binary).map_err(internal)?;
    make_executable(&binary).map_err(internal)?;
    copy_dir_all(&update_dir.join("static"), &treiber_dir.join("static")).map_err(internal)?;
    copy_mediamtx_config(&update_dir.join("mediamtx_config"), &base_dir.join("mediamtx")).map_err(internal)?;

    let status = systemctl("start", BACKEND_SERVICE).map_err(internal)?;
    if !status.success() {
        return Err(internal(format!("failed to start {BACKEND_SERVICE}: {status}")));
    }

    let status = systemctl("restart", MEDIAMTX_SERVICE).map_err(internal)?;
    if !status.success() {
        return Err(internal(format!("failed to restart {MEDIAMTX_SERVICE}: {status}")));
    }
    Ok(())
}

fn verify_checksum(update_dir: &Path) -> UpdateResult {
    let hash_path = update_dir.join(HASH_FILE);
    let expected = fs::read_to_string(&hash_path)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("missing {HASH_FILE}: {e}")))?;

    let mut files = Vec::new();
    collect_files(update_dir, &mut files).map_err(internal)?;
    files.retain(|p| p != &hash_path);
    files.sort();

    let mut hasher = Sha256::new();
    for file in &files {
        let rel = file.strip_prefix(update_dir).map_err(internal)?.to_string_lossy().replace('\\', "/");
        hasher.update(rel.as_bytes());
        hasher.update([0u8]);
        hasher.update(fs::read(file).map_err(internal)?);
        hasher.update([0u8]);
    }

    let actual: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();
    if actual != expected.trim() {
        return Err((StatusCode::BAD_REQUEST, "checksum mismatch".to_string()));
    }
    Ok(())
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_files(&path, out)?;
        } else {
            out.push(path);
        }
    }
    Ok(())
}

// Removes everything in the treiber dir except the maps folder.
fn clear_treiber_dir(treiber_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(treiber_dir)?;
    for entry in fs::read_dir(treiber_dir)? {
        let entry = entry?;
        if entry.file_name() == MAPS_DIR {
            continue;
        }
        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(entry.path())?;
        } else {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_all(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn copy_mediamtx_config(src_dir: &Path, mediamtx_dir: &Path) -> io::Result<()> {
    fs::create_dir_all(mediamtx_dir)?;
    fs::copy(src_dir.join("mediamtx.yml"), mediamtx_dir.join("mediamtx.yml"))?;
    Ok(())
}

fn make_executable(path: &Path) -> io::Result<()> {
    let status = Command::new("chmod").arg("+x").arg(path).status()?;
    if !status.success() {
        return Err(io::Error::other(format!("chmod +x failed: {status}")));
    }
    Ok(())
}

fn systemctl(action: &str, service: &str) -> io::Result<ExitStatus> {
    Command::new("systemctl").arg(action).arg(service).status()
}

// A failed stop is logged but does not abort the update.
fn stop_backend() {
    match systemctl("stop", BACKEND_SERVICE) {
        Ok(status) if status.success() => {}
        Ok(status) => eprintln!("failed to stop {BACKEND_SERVICE}: {status}"),
        Err(e) => eprintln!("failed to run systemctl: {e}"),
    }
}

fn internal(err: impl Display) -> (StatusCode, String) {
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
    

    




   