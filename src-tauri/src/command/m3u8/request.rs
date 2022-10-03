use super::error;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time;

pub(crate) async fn get_m3u8_list(m3u8_url: &str) -> Result<String, error::M3u8Error> {
    match reqwest::get(m3u8_url).await {
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

pub async fn get_ts(url: &String, id: &usize, path: &str) -> Result<(), error::M3u8Error> {
    match reqwest::get(url).await {
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

pub async fn get_all_ts(url_list: &Vec<String>, temp_dir: &str) {
    for item in 0..url_list.len() {
        let link = url_list.get(item).unwrap();
        get_ts(link, &item, &temp_dir).await.unwrap();
    }
    println!("下载完成");
}
