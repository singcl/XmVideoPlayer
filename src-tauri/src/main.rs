#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};
use std::fs::File;
use std::path::PathBuf;
use std::thread;
use tauri::Manager;
use tauri::Runtime;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// https://tauri.app/zh/v1/guides/features/splashscreen
// Waiting for Rust
// If you are waiting for Rust code to run, put it in the setup function handler so you have access to the App instance:
#[tauri::command]
async fn close_splashscreen(window: tauri::Window) {
    // Close splashscreen
    if let Some(splashscreen) = window.get_window("splashscreen") {
        splashscreen.close().unwrap();
    }
    // Show main window
    window.get_window("main").unwrap().show().unwrap();
}

// TODO: 异步stream写入
#[tauri::command]
fn video_download<R: Runtime>(
    app: tauri::AppHandle<R>,
    window: tauri::Window<R>,
    path: String,
) -> Result<String, String> {
    // 下载线程
    // thread 'tokio-runtime-worker' panicked at 'Cannot drop a runtime in a context where blocking is not allowed.
    // This happens when a runtime is dropped from within an asynchronous context.
    // let client = reqwest::blocking::Client::new();

    // 下载线程
    thread::spawn(move || {
        let client = reqwest::blocking::Client::new();
        let resp = client.get(path).headers(construct_headers()).send();
        let mut buf: Vec<u8> = Vec::new();
        let mut r = resp.unwrap();
        println!("{:#?}", r.headers());
        let video_file = PathBuf::from("test_video.mp4");
        let mut f = File::create(video_file).unwrap();
        let _ = r.copy_to(&mut f);
    });
    Ok(String::from("Download Success"))
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("video/mp4"));
    headers.insert(RANGE, HeaderValue::from_static("bytes=0-"));
    headers
}

fn main() {
    use tauri::http::ResponseBuilder;
    // use std::io::prelude::*;

    tauri::Builder::default()
        // register_uri_scheme_protocol 不支持返回数据流？？？
        .register_uri_scheme_protocol("stream", move |_app, request| {
            let mut response = ResponseBuilder::new();
            // get the wanted path
            #[cfg(target_os = "windows")]
            let path = request.uri().strip_prefix("stream://localhost/").unwrap();
            #[cfg(not(target_os = "windows"))]
            let path = request.uri().strip_prefix("stream://").unwrap();
            let path = percent_encoding::percent_decode(path.as_bytes())
                .decode_utf8_lossy()
                .to_string();

            // debug log
            println!("current request.uri(): {:#?}", request.uri());
            println!("current web request url: {:#?}", &path);

            response = response
                .header("Connection", "Keep-Alive")
                .header("Access-Control-Allow-Origin", "*")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", 3)
                .header("Content-Range", "bytes 0-3/3");
            response.mimetype("video/mp4").status(206).body(vec![0])
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            close_splashscreen,
            video_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
