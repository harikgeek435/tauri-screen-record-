#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::{Command, Child, Stdio};         // ✅ Add `Stdio`!
use std::sync::Mutex;
use std::path::PathBuf;
use std::io::Write;                                 // ✅ Needed for `.write_all()`
use tauri::{AppHandle, Runtime, State};


struct RecordingState(Mutex<Option<Child>>);

#[tauri::command]
async fn start_recording<R: Runtime>(
    state: State<'_, RecordingState>,
    _app: AppHandle<R>,
    output: String,
) -> Result<(), String> {
let ffmpeg_path = std::path::PathBuf::from("../node_modules/ffmpeg-static/ffmpeg.exe");

    let mut proc_guard = state.0.lock().unwrap();
    if proc_guard.is_some() {
        return Err("Recording already in progress".into());
    }

    let child = Command::new(ffmpeg_path)
    .args([
        "-y",
        "-f", "gdigrab",           // Windows screen grab
        "-i", "desktop",           // Capture entire screen
        "-framerate", "30",
        "-video_size", "1920x1080",
// Record for 5 seconds (for testing)
        "-c:v", "libx264",         // ✅ Use a proper codec
        "-pix_fmt", "yuv420p",     // ✅ For compatibility
        "-preset", "ultrafast",
        &output,                   // ✅ Save to public path
    ])
     .stdin(Stdio::piped())
    .spawn()
    .map_err(|e| format!("Failed to start ffmpeg: {}", e))?;


    *proc_guard = Some(child);
    Ok(())
}
#[tauri::command]
async fn stop_recording(state: State<'_, RecordingState>) -> Result<(), String> {
    let mut proc_guard = state.0.lock().unwrap();

    if let Some(mut child) = proc_guard.take() {
        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            if let Err(e) = stdin.write_all(b"q\n") {
                println!("Failed to write to ffmpeg stdin: {}", e);
            } else {
                println!("Sent 'q' to ffmpeg");
            }
        } else {
            println!("No stdin available on child");
        }

        let _ = child.wait(); // Wait for FFmpeg to exit
        println!("FFmpeg stopped");
        Ok(())
    } else {
        Err("No recording in progress".into())
    }
}



fn main() {
    tauri::Builder::default()
        .manage(RecordingState(Mutex::new(None)))
        .invoke_handler(tauri::generate_handler![start_recording, stop_recording])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
