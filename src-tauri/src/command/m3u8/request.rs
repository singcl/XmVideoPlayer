use super::error;
use std::io::Write;
use std::path::PathBuf;

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

pub async fn get_ts(url: String, id: usize) -> Result<(), error::M3u8Error> {
    println!("thread {} created", id);
    match reqwest::get(url).await {
        Ok(response) => match response.status() {
            reqwest::StatusCode::OK => match response.bytes().await {
                Ok(bytes) => {
                    let mut f =
                        std::fs::File::create(PathBuf::from(format!("./temp/{}.ts", id))).unwrap();
                    f.write_all(&bytes)
                        .expect(format!("failed to write {}.ts", id).as_str());
                    Ok(())
                }
                Err(cause) => Err(error::M3u8Error::HTTPError(cause)),
            },
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

pub async fn get_all_ts(url_list: &Vec<String>) {
    // pass
    let mut handlers = Vec::new();
    for id in 0..url_list.len() {
        handlers.push(tokio::spawn(get_ts(format!("{}", url_list[0]), id)));
    }
    println!("All {} thread(s) created", url_list.len());
    for handle in handlers {
        handle.await.unwrap().unwrap();
    }
    println!("All threads finished");
}
