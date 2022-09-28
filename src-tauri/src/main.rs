#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, RANGE, USER_AGENT};
    use tauri::http::ResponseBuilder;
    tauri::Builder::default()
        .register_uri_scheme_protocol("stream", |_app, request| {
            let mut response = ResponseBuilder::new();
            // get the wanted path
            #[cfg(target_os = "windows")]
            let path = request.uri().strip_prefix("stream://localhost/").unwrap();
            #[cfg(not(target_os = "windows"))]
            let path = request.uri().strip_prefix("stream://").unwrap();
            let path = percent_encoding::percent_decode(path.as_bytes())
                .decode_utf8_lossy()
                .to_string();

            // debug log
            println!("current request.uri(): {:#?}", request.uri());
            println!("current web request url: {:#?}", &path);

            fn construct_headers() -> HeaderMap {
                let mut headers = HeaderMap::new();
                headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
                headers.insert(CONTENT_TYPE, HeaderValue::from_static("image/png"));
                headers.insert(RANGE, HeaderValue::from_static("bytes=0-1"));
                headers
            }
            let client = reqwest::blocking::Client::new();
            let resp = client.get(&path).headers(construct_headers()).send();
            let mut buf: Vec<u8> = vec![];
            let _ = &resp?.copy_to(&mut buf);
            // println!("{:#?}", &mut &resp?.headers());
            // let mut buf = Vec::new();
            // resp.write_to(&mut buf)?;

            response = response
                .header("Connection", "Keep-Alive")
                .header("Access-Control-Allow-Origin", "*")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", 3)
                .header("Content-Range", "bytes 0-3/3");
            response.mimetype("video/mp4").status(206).body(buf)
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
