use tauri::Manager;
// https://tauri.app/zh/v1/guides/features/splashscreen
// Waiting for Rust
// If you are waiting for Rust code to run, put it in the setup function handler so you have access to the App instance:
#[tauri::command]
pub async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}


