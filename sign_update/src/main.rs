use flate2::{Compression, write::GzEncoder};
use sha2::{Digest, Sha256};
use std::{
    error::Error,
    fs::{self, File},
    io::{self, Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::{Duration, Instant},
};

type AppResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

const IMAGE_TAG: &str = "pi-backend";
const CONTAINER_NAME: &str = "nsh-update-extract";

fn main() -> AppResult<()> {
    let sign_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo_root = sign_dir.parent().ok_or("sign_update has no parent directory")?;
    let update_dir = sign_dir.join("target/update_bin");
    let archive_path = sign_dir.join("target/update.tar.gz");

    prepare_update_dir(&update_dir)?;

    let frontend_dir = repo_root.join("frontend");
    let static_dir = update_dir.join("static");
    let (backend_result, frontend_result) = thread::scope(|s| {
        let backend = s.spawn(|| task("backend", |log| build_backend(repo_root, &update_dir, log)));
        let frontend = s.spawn(|| task("frontend", |log| package_frontend(&frontend_dir, &static_dir, log)));
        (backend.join(), frontend.join())
    });
    backend_result.map_err(|_| "backend thread panicked")??;
    frontend_result.map_err(|_| "frontend thread panicked")??;

    copy_mediamtx_config(repo_root, &update_dir)?;
    write_hash(&update_dir)?;
    create_archive(&update_dir, &archive_path)?;

    println!("Created {}", archive_path.display());

    if confirm_send()? {
        let ssid = select_wifi()?;
        connect_wifi(&ssid)?;
        send_to_pi(&archive_path)?;
    }
    Ok(())
}

fn prepare_update_dir(dir: &Path) -> AppResult<()> {
    if dir.exists() {
        fs::remove_dir_all(dir)?;
    }
    fs::create_dir_all(dir)?;
    Ok(())
}

// Runs a task, buffering its log and printing it once the task has finished or failed.
fn task<F>(name: &str, f: F) -> AppResult<()>
where
    F: FnOnce(&mut String) -> AppResult<()>,
{
    println!("Starting task: {name}");
    let mut log = format!("== {name} ==\n");
    let result = f(&mut log);
    if let Err(e) = &result {
        log.push_str(&format!("error: {e}\n"));
    }
    log.push_str(format!("Finished task: {name}\n").as_str());
    let _ = io::stdout().lock().write_all(log.as_bytes());
    result
}

fn run(program: &str, args: &[&str], cwd: &Path, log: &mut String) -> AppResult<()> {
    // nsh is a .cmd shim on Windows, which Command cannot launch directly.
    let mut cmd = if cfg!(windows) {
        let mut c = Command::new("cmd");
        c.arg("/C").arg(program).args(args);
        c
    } else {
        let mut c = Command::new(program);
        c.args(args);
        c
    };
    let output = cmd
        .current_dir(cwd)
        .output()
        .map_err(|e| format!("failed to start `{program}`: {e}"))?;

    log.push_str(&format!("$ {program} {}\n", args.join(" ")));
    log.push_str(&String::from_utf8_lossy(&output.stdout));
    log.push_str(&String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(format!("`{program} {}` failed with {}", args.join(" "), output.status).into());
    }
    Ok(())
}

fn build_backend(repo_root: &Path, update_dir: &Path, log: &mut String) -> AppResult<()> {
    run("docker", &["build", "-f", "backend/Dockerfile", "-t", IMAGE_TAG, "."], repo_root, log)?;

    // Remove a container left over from an earlier failed run.
    let _ = run("docker", &["rm", "-f", CONTAINER_NAME], repo_root, log);
    run("docker", &["create", "--name", CONTAINER_NAME, IMAGE_TAG], repo_root, log)?;

    let src = format!("{CONTAINER_NAME}:/backend");
    let dest = update_dir.join("backend").to_string_lossy().into_owned();
    let copy_result = run("docker", &["cp", src.as_str(), dest.as_str()], repo_root, log);

    run("docker", &["rm", "-f", CONTAINER_NAME], repo_root, log)?;
    copy_result
}

fn package_frontend(frontend_dir: &Path, static_dir: &Path, log: &mut String) -> AppResult<()> {
    run("nsh", &["package", "--target", "web"], frontend_dir, log)?;

    let out_dir = frontend_dir.join(".nsh").join("out").join("nsh-app-frontend");
    if !out_dir.is_dir() {
        return Err(format!("nsh output not found at {}", out_dir.display()).into());
    }
    copy_dir_all(&out_dir, static_dir)
}

fn copy_mediamtx_config(repo_root: &Path, update_dir: &Path) -> AppResult<()> {
    let src = repo_root.join("backend").join("mediamtx_config").join("mediamtx.yml");
    let dst_dir = update_dir.join("mediamtx_config");
    fs::create_dir_all(&dst_dir)?;
    fs::copy(&src, dst_dir.join("mediamtx.yml"))
        .map_err(|e| format!("failed to copy {}: {e}", src.display()))?;
    Ok(())
}

fn copy_dir_all(src: &Path, dst: &Path) -> AppResult<()> {
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

fn write_hash(update_dir: &Path) -> AppResult<()> {
    let hash_path = update_dir.join("hash.txt");

    let mut files = Vec::new();
    collect_files(update_dir, &mut files)?;
    files.retain(|p| p != &hash_path);
    files.sort();

    let mut hasher = Sha256::new();
    for file in &files {
        let rel = file.strip_prefix(update_dir)?.to_string_lossy().replace('\\', "/");
        hasher.update(rel.as_bytes());
        hasher.update([0u8]);
        hasher.update(fs::read(file)?);
        hasher.update([0u8]);
    }

    let hex: String = hasher.finalize().iter().map(|b| format!("{b:02x}")).collect();
    fs::write(&hash_path, hex + "\n")?;
    Ok(())
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) -> AppResult<()> {
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

fn create_archive(update_dir: &Path, archive_path: &Path) -> AppResult<()> {
    let file = File::create(archive_path)?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut builder = tar::Builder::new(encoder);
    builder.append_dir_all("update_bin", update_dir)?;
    builder.into_inner()?.finish()?;
    Ok(())
}

fn confirm_send() -> AppResult<bool> {
    print!("Send update to the Pi? Press Enter to continue, or type anything else to skip: ");
    io::stdout().flush()?;
    Ok(read_line()?.trim().is_empty())
}

fn read_line() -> AppResult<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim_end_matches(['\r', '\n']).to_string())
}

fn select_wifi() -> AppResult<String> {
    let networks = list_wifi_networks()?;
    if networks.is_empty() {
        return Err("no wifi networks found".into());
    }
    for (i, ssid) in networks.iter().enumerate() {
        println!("{:>3}) {ssid}", i + 1);
    }

    print!("Select network number: ");
    io::stdout().flush()?;
    let choice: usize = read_line()?.trim().parse().map_err(|_| "invalid selection")?;
    networks
        .get(choice.wrapping_sub(1))
        .cloned()
        .ok_or_else(|| "invalid selection".into())
}

fn list_wifi_networks() -> AppResult<Vec<String>> {
    let output = Command::new("netsh").args(["wlan", "show", "networks", "mode=ssid"]).output()?;
    let text = String::from_utf8_lossy(&output.stdout);

    let mut ssids: Vec<String> = Vec::new();
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix("SSID ") else { continue };
        let Some((_, name)) = rest.split_once(':') else { continue };
        let name = name.trim();
        if !name.is_empty() && !ssids.iter().any(|s| s == name) {
            ssids.push(name.to_string());
        }
    }
    Ok(ssids)
}

fn connect_wifi(ssid: &str) -> AppResult<()> {
    println!("Connecting to {ssid}...");
    request_connect(ssid)?;
    if wait_for_ssid(ssid, Duration::from_secs(20))? {
        return Ok(());
    }

    print!("No saved profile worked. Password for {ssid}: ");
    io::stdout().flush()?;
    add_profile(ssid, &read_line()?)?;
    request_connect(ssid)?;
    if wait_for_ssid(ssid, Duration::from_secs(60))? {
        Ok(())
    } else {
        Err(format!("could not connect to {ssid}").into())
    }
}

fn request_connect(ssid: &str) -> AppResult<()> {
    // Failure is detected by polling the connection state, not by the exit code.
    Command::new("netsh").args(["wlan", "connect"]).arg(format!("name={ssid}")).output()?;
    Ok(())
}

fn wait_for_ssid(ssid: &str, timeout: Duration) -> AppResult<bool> {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if current_ssid()?.as_deref() == Some(ssid) {
            return Ok(true);
        }
        thread::sleep(Duration::from_secs(2));
    }
    Ok(false)
}

fn current_ssid() -> AppResult<Option<String>> {
    let output = Command::new("netsh").args(["wlan", "show", "interfaces"]).output()?;
    let text = String::from_utf8_lossy(&output.stdout);
    Ok(text.lines().find_map(|line| {
        let (key, value) = line.trim_start().split_once(':')?;
        (key.trim_end() == "SSID").then(|| value.trim().to_string())
    }))
}

fn add_profile(ssid: &str, password: &str) -> AppResult<()> {
    let ssid_hex: String = ssid.bytes().map(|b| format!("{b:02X}")).collect();
    let xml = format!(
        r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{name}</name>
    <SSIDConfig><SSID><hex>{ssid_hex}</hex><name>{name}</name></SSID></SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>auto</connectionMode>
    <MSM><security>
        <authEncryption><authentication>WPA2PSK</authentication><encryption>AES</encryption><useOneX>false</useOneX></authEncryption>
        <sharedKey><keyType>passPhrase</keyType><protected>false</protected><keyMaterial>{key}</keyMaterial></sharedKey>
    </security></MSM>
</WLANProfile>"#,
        name = xml_escape(ssid),
        key = xml_escape(password),
    );

    let profile_path = std::env::temp_dir().join("treiber_wifi_profile.xml");
    fs::write(&profile_path, xml)?;
    let output = Command::new("netsh")
        .args(["wlan", "add", "profile"])
        .arg(format!("filename={}", profile_path.display()))
        .output()?;
    let _ = fs::remove_file(&profile_path);

    if !output.status.success() {
        let msg = String::from_utf8_lossy(&output.stdout);
        return Err(format!("failed to add wifi profile: {msg}").into());
    }
    Ok(())
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn send_to_pi(archive: &Path) -> AppResult<()> {
    let payload = fs::read(archive)?;
    let addr = SocketAddr::from(([192, 168, 50, 1], 8141));
    println!("Sending update to {addr}...");

    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(10))?;
    // The Pi installs the update before it replies.
    stream.set_read_timeout(Some(Duration::from_secs(900)))?;
    write!(
        stream,
        "POST /update HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/gzip\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    )?;
    stream.write_all(&payload)?;

    let mut raw = Vec::new();
    stream.read_to_end(&mut raw)?;
    let reply = String::from_utf8_lossy(&raw);
    let status_line = reply.lines().next().unwrap_or_default();
    let status = status_line.split_whitespace().nth(1).unwrap_or_default();
    let reply_body = reply.split_once("\r\n\r\n").map(|(_, b)| b.trim()).unwrap_or_default();

    if status != "200" {
        return Err(format!("update rejected: {status_line} {reply_body}").into());
    }
    println!("{reply_body}");
    Ok(())
}
