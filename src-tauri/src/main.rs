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

            println!("current request path: {:#?}", &path);
            response = response
                .header("Connection", "Keep-Alive")
                .header("Accept-Ranges", "bytes")
                .header("Content-Length", 3)
                .header("Content-Range", "bytes 0-3/3");
            response
                .mimetype("video/mp4")
                .status(206)
                .body(vec![0, 1, 2])
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
