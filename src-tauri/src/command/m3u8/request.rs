use super::error;
use std::io::Write;

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
