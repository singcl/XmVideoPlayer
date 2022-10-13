use super::payload::Payload;
use crate::state::Database;
use std::time::Duration;
use tauri::State;
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
                "pong",
                Payload {
                    message: "XmVideoPlayer@singcl<https://github.com/singcl>".into(),
                },
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(5000));
    });
}

#[tauri::command]
pub fn db_insert(key: String, value: String, db: State<'_, Database>) {
    db.0.lock().unwrap().insert(key, value);
}

#[tauri::command]
pub fn db_read(key: String, db: State<'_, Database>) -> Option<String> {
    db.0.lock().unwrap().get(&key).cloned()
}
