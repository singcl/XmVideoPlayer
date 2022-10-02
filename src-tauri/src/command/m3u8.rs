// use std::fs::File;
// use std::io::prelude::*;
// use std::path::PathBuf;
// use std::thread;
// use tauri::Runtime;
// use futures_util::StreamExt;

pub mod error;
pub mod parse;
pub mod request;

// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};

#[tauri::command]
pub(crate) async fn m3u8_download(
    save_path: String,
    m3u8_url: &str,
) -> Result<String, error::M3u8Error> {
    let url_list = request::get_m3u8_list(m3u8_url).await?;
    let url_list_entity = parse::parse_m3u8_list(&url_list, m3u8_url);
    // for i in &url_list_entity {
    //     println!("{}", i);
    // }
    Ok(url_list)
}
