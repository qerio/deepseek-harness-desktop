use std::sync::OnceLock;

#[cfg(desktop)]
use std::{process::Child, sync::Mutex};

static HARNESS_URL: OnceLock<String> = OnceLock::new();

#[cfg(desktop)]
struct HarnessProcess(Mutex<Option<Child>>);

#[tauri::command]
fn harness_url() -> Result<String, String> {
    HARNESS_URL
        .get()
        .cloned()
        .ok_or_else(|| "Harness URL is not initialized".to_string())
}

#[cfg(desktop)]
fn start_harness(app: &tauri::AppHandle) -> Result<(String, Child), String> {
    use std::path::PathBuf;
    use std::process::{Command, Stdio};
    use tauri::Manager;

    let development_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../vendor/deepseek-harness");
    let packaged_root = app
        .path()
        .resource_dir()
        .map_err(|error| error.to_string())?
        .join("vendor/deepseek-harness");
    let harness_root = if development_root.exists() {
        development_root
    } else {
        packaged_root
    };
    let entry = harness_root.join("apps/cli/lib/bin.js");

    let child = Command::new("node")
        .arg(entry)
        .args(["web", "--host", "127.0.0.1", "--port", "43921"])
        .current_dir(&harness_root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|error| format!("无法启动 Node.js 后端：{error}"))?;

    Ok(("http://127.0.0.1:43921".to_string(), child))
}

#[cfg(mobile)]
fn start_harness(_app: &tauri::AppHandle) -> Result<String, String> {
    option_env!("DSH_REMOTE_URL")
        .map(str::to_string)
        .ok_or_else(|| "移动端构建前必须设置 DSH_REMOTE_URL（例如 https://harness.example.com）".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    #[cfg(desktop)]
    let builder = builder.manage(HarnessProcess(Mutex::new(None)));

    builder
        .setup(|app| {
            #[cfg(desktop)]
            let url = {
                use tauri::Manager;

                let (url, child) = start_harness(app.handle())?;
                *app.state::<HarnessProcess>().0.lock().map_err(|error| error.to_string())? = Some(child);
                url
            };

            #[cfg(mobile)]
            let url = start_harness(app.handle())?;

            HARNESS_URL
                .set(url)
                .map_err(|_| "Harness URL was already initialized")?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![harness_url])
        .build(tauri::generate_context!())
        .expect("error while building DeepSeek Harness Desktop")
        .run(|app, event| {
            #[cfg(desktop)]
            if matches!(event, tauri::RunEvent::Exit | tauri::RunEvent::ExitRequested { .. }) {
                use tauri::Manager;

                if let Ok(mut process) = app.state::<HarnessProcess>().0.lock() {
                    if let Some(mut child) = process.take() {
                        let _ = child.kill();
                        let _ = child.wait();
                    }
                }
            }
        })
}
