use std::{collections::HashMap, path::Path, time::Duration, thread};
use pbr::{Units, ProgressBar};
use rand::{distributions::Alphanumeric, Rng};
use tokio::{fs, io::{AsyncWriteExt}};
use url::{Url};
use clap::Parser;

const M3U8_EXT_HEADER: &str = "#EXTM3U";
const M3U8_EXT_INF: &str = "#EXTINF";
const M3U8_EXT_ENDLIST: &str = "#EXT-X-ENDLIST";
const M3U8_EXT_KEY: &str = "#EXT-X-KEY:";
const M3U8_EXT_KEY_METHOD: &str = "METHOD=";
const M3U8_EXT_KEY_URI: &str = "URI=";
const M3U8_EXT_KEY_IV:&str = "IV=";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, default_value_t=String::from("http://localhost:9999/playlist.m3u8"))]
    url: String,
}

async fn get_m3u8_content(url:String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    Ok(resp.to_string())
}

// validate m3u8 content
fn validate_m3u8_content(text:String) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(text.starts_with(M3U8_EXT_HEADER))
}

/// get ts list
fn get_ts_list(link:String, text:String) -> Result<Vec::<String>,  Box<dyn std::error::Error>> {
    let mut v = Vec::<String>::new();
    let mut flag = false;
    let url = Url::parse(link.as_str())?;
    for item in text.lines() {
        if item.starts_with(M3U8_EXT_INF) {
            flag = true;
            continue;
        }

        if flag {
            let ts = url.join(item.clone())?.to_string();
            v.push(ts);
            flag = false;
        }

        if item.starts_with(M3U8_EXT_ENDLIST) {
            break;
        }
    }
    Ok(v)
}

// get key info
fn get_key_info(content: String) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    
    let mut m = HashMap::<String, String>::new();
    
    for item in content.lines() {
        if !item.starts_with(M3U8_EXT_KEY){
            continue;
        }
        let data = item.replace(M3U8_EXT_KEY, "").replace('"', "");
        for it in data.split(",") {
            
            if it.starts_with(M3U8_EXT_KEY_METHOD){
                let tmp = it.to_string().replace(M3U8_EXT_KEY_METHOD, "");
                m.insert("method".to_string(), tmp);
            }
            
            if it.starts_with(M3U8_EXT_KEY_IV){
                let tmp = it.to_string().replace(M3U8_EXT_KEY_IV, "");
                m.insert("iv".to_string(), tmp);
            }

            if it.starts_with(M3U8_EXT_KEY_URI){
                let tmp = it.to_string().replace(M3U8_EXT_KEY_URI, "");
                m.insert("uri".to_string(), tmp);
            }
            
        }
    }
    Ok(m)
}

// get key content
async fn get_key_content(url: String) ->Result<String, Box<dyn std::error::Error>> {
    let text = reqwest::get(&url).await?.text().await?;
    Ok(text)
}

// download file
async fn download(link: String) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
    .build()?;

    let dirname = "download".to_string();
    if !Path::new(&dirname).exists(){
        fs::create_dir(&dirname).await?;
    }
    
    let url = Url::parse(&link)?;

    let respone = client.get(&link).send().await?;
    let content_length = respone.content_length().unwrap(); 
    
    let mut pb = ProgressBar::new(content_length);
    pb.set_units(Units::Bytes);
    
    let filename = String::from("./download") + url.path();
  
    let mut source = client.get(&link).header("content-type", "application/octet-stream").send().await?;
 
    let mut dest = fs::OpenOptions::new().create(true).append(true).open(&filename).await?;
    
    while let Some(chunk) = source.chunk().await? {
        dest.write_all(&chunk).await?;
        pb.add(chunk.len() as u64);
        thread::sleep(Duration::from_millis(10));
    }
    pb.finish_println(format!("{}:下载成功!\n", filename).as_str());
    
    Ok(filename)

}

// download all
async fn download_all(ts_list: Vec<String>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut filename_list:Vec<String> = Vec::new();

    for item in ts_list.iter() {
        let link = item.to_string().clone();
        let filename = download(link).await?;
        filename_list.push(filename);
    }
    Ok(filename_list)
}

// merge file
async fn merge(filename_list: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut result = vec![];
    let result_name = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(7)
    .map(char::from)
    .collect::<String>() + ".ts";
    for filename in filename_list.iter() {
        let content = fs::read(filename).await?;
        result.extend_from_slice(&content);
    }
    fs::write(&result_name, result).await?;
    println!("文件合并至: ./{}", result_name);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let content = get_m3u8_content(args.url.clone()).await?;

    validate_m3u8_content(content.clone())?;

    let ts_list = get_ts_list(args.url.clone(), content.clone())?;

    let key_info = get_key_info(content.clone())?;

    let method = match key_info.get("method") {
        Some(method) => method.to_string(),
        None => "".to_string(),
    };

    let _iv = match key_info.get("iv") {
        Some(method) => method.to_string(),
        None => "".to_string(),
    };

    let _key = match key_info.get("uri") {
        Some(key) => get_key_content(key.clone()).await?,
        None => "".to_string(),
    };

    if let Ok(filename_list) = download_all(ts_list).await {
        if method == ""{
            merge(filename_list).await?;
        }
    }else {
        println!("下载失败...");
    }
    
    Ok(())
}