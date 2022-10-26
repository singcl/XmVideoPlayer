#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use std::io::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::{
    cmp::min,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
    // process::{Command, Stdio},
};
use tauri::http::{HttpRange, ResponseBuilder};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

pub mod command;
pub mod s;
pub mod state;
pub mod utils;

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let visible = CustomMenuItem::new("visible".to_string(), "隐藏");
    let tray_menu = SystemTrayMenu::new()
        .add_item(visible) // insert the menu items here
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            let window_visible = app.state::<state::WindowVisible>();
            let window = app.get_window("main").unwrap();
            let item_handle = app.tray_handle().get_item("visible");
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    window_visible.0.store(true, Ordering::Relaxed);
                    window.show().unwrap();
                    item_handle.set_title("隐藏").unwrap();
                    println!(
                        "system tray received a left click, and {:?}",
                        window_visible
                    );
                }
                SystemTrayEvent::RightClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    window_visible.0.store(true, Ordering::Relaxed);
                    window.show().unwrap();
                    item_handle.set_title("隐藏").unwrap();
                    println!("system tray received a right click");
                }
                SystemTrayEvent::DoubleClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    window_visible.0.store(true, Ordering::Relaxed);
                    window.show().unwrap();
                    item_handle.set_title("隐藏").unwrap();
                    println!("system tray received a double click");
                }
                SystemTrayEvent::MenuItemClick { id, .. } => {
                    // get a handle to the clicked menu item
                    // note that `tray_handle` can be called anywhere,
                    // just get a `AppHandle` instance with `app.handle()` on the setup hook
                    // and move it to another function or thread
                    let item_handle = app.tray_handle().get_item(&id);
                    match id.as_str() {
                        "quit" => {
                            std::process::exit(0);
                        }
                        "visible" => {
                            let visible = window_visible.0.load(Ordering::Relaxed);
                            if visible {
                                window_visible.0.store(false, Ordering::Relaxed);
                                window.hide().unwrap();
                                // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                                item_handle.set_title("显示").unwrap();
                            } else {
                                window_visible.0.store(true, Ordering::Relaxed);
                                window.show().unwrap();
                                // you can also `set_selected`, `set_enabled` and `set_native_image` (macOS only).
                                item_handle.set_title("隐藏").unwrap();
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        })
        .setup(|_app| {
            if let Some(mut home_dir) = tauri::api::path::home_dir() {
                home_dir.push(".xmvideoplayer");
                std::fs::create_dir_all(&home_dir)?;
            }
            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                api.prevent_close();
                let window = event.window();
                let app = window.app_handle();
                let item_handle = app.tray_handle().get_item("visible");
                let window_visible = app.state::<state::WindowVisible>();
                window_visible.0.store(false, Ordering::Relaxed);
                window.hide().unwrap();
                item_handle.set_title("显示").unwrap();
            }
            _ => {}
        })
        // register_uri_scheme_protocol 不支持返回数据流？？？
        .register_uri_scheme_protocol("stream", move |_app, request| {
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
            // println!("current request.uri(): {:#?}", request.uri());
            println!("current resource: {:#?}", &path);

            let video_file = PathBuf::from(&path);
            if !video_file.exists() {
                // return error 404 if it's not out video
                return response.mimetype("text/plain").status(404).body(Vec::new());
            }
            // read our file
            let mut content = std::fs::File::open(&video_file)?;
            let mut buf = Vec::new();

            // default status code
            let mut status_code = 200;

            // if the webview sent a range header, we need to send a 206 in return
            // Actually only macOS and Windows are supported. Linux will ALWAYS return empty headers.
            if let Some(range) = request.headers().get("range") {
                // Get the file size
                let file_size = content.metadata().unwrap().len();
                // we parse the range header with tauri helper
                let range = HttpRange::parse(range.to_str().unwrap(), file_size).unwrap();
                // let support only 1 range for now
                let first_range = range.first();

                if let Some(range) = first_range {
                    let mut real_length = range.length;

                    // prevent max_length;
                    // specially on webview2
                    if range.length > file_size / 3 {
                        // max size sent (400ko / request)
                        // as it's local file system we can afford to read more often
                        real_length = min(file_size - range.start, 1024 * 400);
                    }

                    // last byte we are reading, the length of the range include the last byte
                    // who should be skipped on the header
                    let last_byte = range.start + real_length - 1;
                    // partial content
                    status_code = 206;

                    // Only macOS and Windows are supported, if you set headers in linux they are ignored
                    response = response
                        .header("Connection", "Keep-Alive")
                        .header("Accept-Ranges", "bytes")
                        .header("Content-Length", real_length)
                        .header(
                            "Content-Range",
                            format!("bytes {}-{}/{}", range.start, last_byte, file_size),
                        );
                    // FIXME: Add ETag support (caching on the webview)

                    // seek our file bytes
                    content.seek(SeekFrom::Start(range.start))?;
                    content.take(real_length).read_to_end(&mut buf)?;
                } else {
                    content.read_to_end(&mut buf)?;
                }
            }
            response.mimetype("video/mp4").status(status_code).body(buf)
        })
        .manage(state::Database(Default::default()))
        .manage(state::Connection(Default::default()))
        .manage(state::Counter(AtomicUsize::new(0)))
        .manage(state::WindowVisible(AtomicBool::new(true)))
        .invoke_handler(tauri::generate_handler![
            command::normal::greet,
            command::normal::init_process,
            command::normal::db_read,
            command::normal::db_insert,
            command::normal::increment_counter,
            command::normal::connect,
            command::normal::disconnect,
            command::normal::connection_send,
            command::splashscreen::close_splashscreen,
            command::media::video_download,
            command::m3u8::m3u8_download,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
