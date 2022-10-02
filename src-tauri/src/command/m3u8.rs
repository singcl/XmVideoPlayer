// use std::fs::File;
// use std::io::prelude::*;
// use std::path::PathBuf;
// use std::thread;
// use tauri::Runtime;
// use futures_util::StreamExt;

pub mod request;
pub mod error;

// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};

#[tauri::command]
pub(crate) async fn m3u8_download(path: String, m3u8_url: &str) -> Result<String, error::M3u8Error> {
    let url_list = request::get_m3u8_list(m3u8_url).await?;
    Ok(url_list)
}
