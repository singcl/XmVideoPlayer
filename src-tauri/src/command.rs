use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use tauri::Manager;
// use std::thread;
// use tauri::Runtime;
// use futures_util::StreamExt;

use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};
// use tauri::api::http::{ClientBuilder, HttpRequestBuilder, ResponseType};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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

#[tauri::command]
pub async fn video_download(
    // app: tauri::AppHandle<R>,
    // window: tauri::Window<R>,
    path: String,
    url: String,
) -> Result<String, tauri::Error> {
    // 下载线程
    // thread 'tokio-runtime-worker' panicked at 'Cannot drop a runtime in a context where blocking is not allowed.
    // This happens when a runtime is dropped from within an asynchronous context.
    // let client = reqwest::blocking::Client::new();

    // // 下载线程
    // thread::spawn(move || {
    //     let client = reqwest::blocking::Client::new();
    //     let resp = client.get(path).headers(construct_headers()).send();
    //     let mut buf: Vec<u8> = Vec::new();
    //     let mut r = resp.unwrap();
    //     println!("{:#?}", r.headers());
    //     let video_file = PathBuf::from("test_video.mp4");
    //     let mut f = File::create(video_file).unwrap();
    //     let _ = r.copy_to(&mut f);
    // });

    // 异步stream写入
    let client = reqwest::Client::new();
    let mut response = client
        .get(url)
        .headers(construct_headers())
        .send()
        .await
        .unwrap();
    // let mut buf: Vec<u8> = Vec::new();
    println!("{:#?}", response.headers());
    let mut buf = File::create(PathBuf::from(path))?;

    while let Some(chunk) = response.chunk().await.unwrap() {
        let write_size = buf.write(&chunk)?;
        println!("已写入:{:?}", write_size);
    }
    println!("写入完成，Success!");
    Ok(String::from("Download Success"))
}

// FIXME: bytes_stream IS pub(crate) fun
// #[tauri::command]
// pub (crate) async fn source_download(
//     // app: tauri::AppHandle<R>,
//     // window: tauri::Window<R>,
//     path: String,
//     url: String,
// ) -> Result<String, tauri::Error> {
//     let client = ClientBuilder::new().max_redirections(3).build()?;
//     let request = HttpRequestBuilder::new("GET", url)?;
//     // .response_type(ResponseType::Text);
//     // println!("{:#?}", response.headers());
//     let mut buf = File::create(PathBuf::from(path))?;
//     let mut stream = client.send(request).await?.bytes_stream();
//     while let Some(chunk) = stream.next().await {
//         // println!("Chunk: {:?}", chunk?);
//         let write_size = buf.write(&chunk)?;
//         println!("已写入:{:?}", write_size);
//     }
//     println!("写入完成，Success!");
//     Ok(String::from("Download Success"))
// }

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("video/mp4"));
    headers.insert(RANGE, HeaderValue::from_static("bytes=0-"));
    headers
}
