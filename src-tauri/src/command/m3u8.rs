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
    let mut home_dir = tauri::api::path::home_dir().unwrap();
    home_dir.push(".xmvideoplayer");

    // 临时目录
    let mut temp_dir = home_dir.clone();
    temp_dir.push("temp");

    std::fs::create_dir_all(&temp_dir).unwrap();
    let temp_dir_str = temp_dir.to_str().unwrap();

    // 保存目录
    let mut out_path = PathBuf::from(save_path);
    if !out_path.exists() {
        std::fs::create_dir_all(&out_path).unwrap();
    }
    out_path.push("output.ts");
    let out_path_str = out_path.to_str().unwrap();

    let url_list = request::get_m3u8_list(m3u8_url).await?;
    let url_list_entity = parse::parse_m3u8_list(&url_list, m3u8_url);
    // TODO:断点续下
    request::get_all_ts(&url_list_entity, temp_dir_str).await;
    merge::merge_ts(temp_dir_str, out_path_str);

    Ok("Success!".into())
}
