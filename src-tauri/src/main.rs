#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};
    use std::path::PathBuf;
    use std::thread;
    use tauri::http::ResponseBuilder;
    use std::fs::File;
    // use std::io::prelude::*;

    tauri::Builder::default()
    // register_uri_scheme_protocol 不支持返回数据流？？？
        .register_uri_scheme_protocol("stream", |_app, request| {
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

            // 下载线程
            thread::spawn(|| {
                fn construct_headers() -> HeaderMap {
                    let mut headers = HeaderMap::new();
                    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
                    headers.insert(CONTENT_TYPE, HeaderValue::from_static("video/mp4"));
                    headers.insert(RANGE, HeaderValue::from_static("bytes=0-"));
                    headers
                }
                let client = reqwest::blocking::Client::new();
                let resp = client.get(path).headers(construct_headers()).send();
                // println!("{:#?}", resp.unwrap().headers());
                // let mut buf = Vec::new();
                let video_file = PathBuf::from("test_video.mp4");
                let f = File::create(video_file);
                let _ = resp.unwrap().copy_to(&mut f.unwrap());
            });

            response = response
                .header("Connection", "Keep-Alive")
                .header("Access-Control-Allow-Origin", "*")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", 3)
                .header("Content-Range", "bytes 0-3/3");
            response.mimetype("video/mp4").status(206).body(vec![0])
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
