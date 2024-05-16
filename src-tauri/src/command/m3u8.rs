// use std::fs::File;
// use std::io::prelude::*;
use std::path::PathBuf;
// use std::thread;
// use tauri::Runtime;
// use futures_util::StreamExt;
use crate::utils;
use crate::vsd;
use tauri::Window;

pub mod error;
pub mod merge;
pub mod parse;
pub mod request;

// use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};

#[tauri::command]
pub(crate) async fn m3u8_download(
    window: Window,
    app_handle: tauri::AppHandle,
    save_path: String,
    m3u8_url: &str,
) -> Result<String, ()> {
    let m3_hash = utils::hash_str(m3u8_url);
    let mut home_dir = tauri::api::path::home_dir().unwrap();
    home_dir.push(".xmvideoplayer");

    // 临时目录
    let mut temp_dir = home_dir.clone();
    temp_dir.push(format!("rs_xm_{}", &m3_hash));

    std::fs::create_dir_all(&temp_dir).unwrap();
    let temp_dir_str = temp_dir.to_str().unwrap();

    //
    let save = vsd::commands::Save::new(m3u8_url, Some(temp_dir), Some(save_path));
    // println!("---{:?}", save);
    let r = save.execute().await;

    match r {
        Ok(_) => Ok("Success!".into()),
        Err(e) => {
            println!("==download error==: {:?}", e);
            Err(())
        }
    }

    // // 保存目录
    // let mut out_path = PathBuf::from(save_path);
    // if !out_path.exists() {
    //     std::fs::create_dir_all(&out_path).unwrap();
    // }
    // out_path.push(format!("xm_{}.ts", &m3_hash));
    // let out_path_str = out_path.to_str().unwrap();

    // let url_list_str = request::get_m3u8_list(m3u8_url).await?;
    // let url_list_entity = parse::parse_m3u8_list(&url_list_str, m3u8_url);

    // println!("---{:?}", url_list_entity);

    // let url_list_entity_hash = url_list_entity
    //     .iter()
    //     .map(|r| utils::hash_str(r))
    //     .collect::<Vec<_>>();

    // let url_list_entity_hash_dl = utils::entity_hash_filter(&url_list_entity_hash, temp_dir_str);
    // let start = url_list_entity.len() - url_list_entity_hash_dl.len();

    // // println!("---{:?}", url_list_entity_hash_dl);
    // // println!("---{:?}", start);

    // // 断点续下
    // request::get_all_ts(
    //     &url_list_entity,
    //     &url_list_entity_hash,
    //     &url_list_entity_hash_dl,
    //     start,
    //     temp_dir_str,
    //     &window,
    // )
    // .await;
    // merge::merge_ts(&url_list_entity_hash, temp_dir_str, out_path_str);

    // Ok("Success!".into())
}
