use super::payload::Payload;
use std::time::Duration;
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn init_process(window: Window) {
    std::thread::spawn(move || loop {
        window
            .emit(
                "ping",
                Payload {
                    message: "XmVideoPlayer@singcl<https://github.com/singcl>".into(),
                },
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(2000));
    });
}
