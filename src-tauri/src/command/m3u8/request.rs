use super::error;
use crate::command::payload::PayloadDownload;
// use crate::utils;
// use reqwest::header;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tauri::Window;
use tokio::time;

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub(crate) async fn get_m3u8_list(m3u8_url: &str) -> Result<String, error::M3u8Error> {
    let client = reqwest::Client::new();
    match client
        .get(m3u8_url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => {
                let response_text = response.text().await?;
                Ok(response_text)
            }
            status_code => {
                println!(
                    "Error getting M3u8 List, Message: {} Try again...",
                    status_code.canonical_reason().unwrap()
                );
                Err(error::M3u8Error::HTTPCode(status_code))
            }
        },
        Err(error) => {
            println!("Error sending HTTP request during getting M3u8 List");
            println!("{}", error.to_string());
            // Err(error::M3u8Error::HTTPError(error))
            Err(error.into())
        }
    }
}

pub async fn get_ts(url: &String, id: &u64, path: &str) -> Result<(), error::M3u8Error> {
    let client = reqwest::Client::new();
    match client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Ok(mut response) => match response.status() {
            reqwest::StatusCode::OK => {
                let mut ts_path = PathBuf::from(path);
                ts_path.push(format!("{}.ts", id));
                let mut f = std::fs::File::create(ts_path).unwrap();
                while let Some(chunk) = response.chunk().await.unwrap() {
                    let write_size = f.write(&chunk).unwrap();
                    time::sleep(Duration::from_millis(10)).await;
                    println!("已写入:{:?}", write_size);
                }
                Ok(())
            }
            status_code => {
                println!(
                    "Error getting M3u8 List, Message: {} Try again...",
                    status_code.canonical_reason().unwrap()
                );
                Err(error::M3u8Error::HTTPCode(status_code))
            }
        },
        Err(error) => {
            println!("Error sending HTTP request during getting M3u8 List");
            println!("{}", error.to_string());
            Err(error.into())
        }
    }
}

//  TODO: url_list_entity_hash, url_list_entity id 可以用一个结构体承载
pub async fn get_all_ts(
    url_list_entity: &Vec<String>,
    url_list_entity_hash: &Vec<u64>,
    url_list_entity_hash_dl: &Vec<u64>,
    start: usize,
    temp_dir: &str,
    window: &Window,
) {
    for item in 0..url_list_entity_hash_dl.len() {
        let id = url_list_entity_hash_dl.get(item).unwrap();
        let link = get_dl_url(url_list_entity, url_list_entity_hash, *id);
        println!("---uri---:{}/{}", link, id);
        get_ts(&link, id, &temp_dir).await.unwrap();
        window
            .emit(
                "download",
                PayloadDownload {
                    download_type: "m3u8".into(),
                    message: "下载中...".into(),
                    total: url_list_entity.len().to_string(),
                    current: (start + item + 1).to_string(),
                },
            )
            .unwrap();
    }
    println!("下载完成");
}

fn get_dl_url(url_list_entity: &Vec<String>, url_list_entity_hash: &Vec<u64>, id: u64) -> String {
    let mut res = String::new();
    for item in 0..url_list_entity_hash.len() {
        let c_id = url_list_entity_hash.get(item).unwrap();
        if *c_id == id {
            let r = url_list_entity.get(item).unwrap();
            res = String::from(r);
            break;
        }
    }
    res
}
