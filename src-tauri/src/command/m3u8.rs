// use std::fs::File;
// use std::io::prelude::*;
use std::path::PathBuf;
// use std::thread;
// use tauri::Runtime;
// use futures_util::StreamExt;

pub mod error;
pub mod merge;
pub mod parse;
pub mod request;

// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};

#[tauri::command]
pub(crate) async fn m3u8_download(
    save_path: String,
    m3u8_url: &str,
) -> Result<String, error::M3u8Error> {
    let temp_dir = PathBuf::from(format!("{}/temp", save_path));
    let temp_dir_str = temp_dir.to_str().unwrap();
    std::fs::create_dir_all(&temp_dir).unwrap();

    let url_list = request::get_m3u8_list(m3u8_url).await?;
    let url_list_entity = parse::parse_m3u8_list(&url_list, m3u8_url);
    request::get_all_ts(&url_list_entity, temp_dir_str).await;
    merge::merge_ts(temp_dir_str);

    Ok("Success!".into())
}
