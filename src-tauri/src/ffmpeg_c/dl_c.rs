use anyhow::Context;
use std::{
    path::{Path, PathBuf},
    // process::{Command, ExitStatus},
};

use ffmpeg_sidecar::{
    download::{
        // auto_download,
        // download_ffmpeg_package
        check_latest_version,
        ffmpeg_download_url,
        unpack_ffmpeg,
    },
    paths::sidecar_dir,
    version::ffmpeg_version,
};

pub use ffmpeg_sidecar::command::ffmpeg_is_installed;

use std::io::Write;
// use std::time::Duration;
use tauri::Window;
// use tokio::time;

use kdam::{tqdm, BarExt, Column, RichProgress};

use crate::tools::payload::Payload;

//
/// Check if FFmpeg is installed, and if it's not, download and unpack it.
/// Automatically selects the correct binaries for Windows, Linux, and MacOS.
/// The binaries will be placed in the same directory as the Rust executable.
///
/// If FFmpeg is already installed, the method exits early without downloading
/// anything.
pub async fn auto_download(wd: &Window) -> anyhow::Result<()> {
    if ffmpeg_is_installed() {
        println!("FFmpeg is already installed! 🎉");
        println!("For demo purposes, we'll re-download and unpack it anyway.");
        println!("TIP: Use `auto_download()` to skip manual customization.");
        return Ok(());
    }

    // Short version without customization:
    // ```rust
    // ffmpeg_sidecar::download::auto_download().unwrap();
    // ```
    // Checking the version number before downloading is actually not necessary,
    // but it's a good way to check that the download URL is correct.
    match check_latest_version() {
        Ok(version) => println!("Latest available version: {}", version),
        Err(_) => println!("Skipping version check on this platform."),
    }

    // These defaults will automatically select the correct download URL for your
    // platform.
    let download_url = ffmpeg_download_url()?;
    let destination = sidecar_dir()?;

    // By default the download will use a `curl` command. You could also write
    // your own download function and use another package like `reqwest` instead.
    println!("Downloading from: {:?}", download_url);
    let archive_path = download_ffmpeg_package(download_url, &destination, wd).await?;
    println!("Downloaded package: {:?}", archive_path);

    // Extraction uses `tar` on all platforms (available in Windows since version 1803)
    println!("Extracting...");
    unpack_ffmpeg(&archive_path, &destination)?;

    // Use the freshly installed FFmpeg to check the version number
    let version = ffmpeg_version()?;
    println!("FFmpeg version: {}", version);

    println!("Done! 🏁");

    if !ffmpeg_is_installed() {
        anyhow::bail!("FFmpeg failed to install, please install manually.");
    }
    Ok(())
}

/// Invoke `curl` to download an archive (ZIP on windows, TAR on linux and mac)
/// from the latest published release online.
pub(self) async fn download_ffmpeg_package(
    url: &str,
    download_dir: &Path,
    wd: &Window,
) -> anyhow::Result<PathBuf> {
    let filename = Path::new(url)
        .file_name()
        .context("Failed to get filename")?;

    let archive_path = download_dir.join(filename);

    let archive_filename = archive_path.to_str().context("invalid download path")?;

    get_package(url, archive_filename, wd).await?;

    // if !exit_status.success() {
    //     anyhow::bail!("Failed to download ffmpeg");
    // }
    Ok(archive_path)
}

// /// Invoke cURL on the command line to download a file, writing to a file.
// pub(self) fn curl_to_file(url: &str, destination: &str) -> anyhow::Result<ExitStatus> {
//     Command::new("curl")
//         .args(["-L", url])
//         .args(["-o", destination])
//         .status()
//         .map_err(Into::into)
// }

const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub(self) async fn get_package(url: &str, destination: &str, wd: &Window) -> anyhow::Result<()> {
    let client = reqwest::Client::new();
    match client
        .get(url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await
    {
        Ok(mut response) => {
            let t = response.content_length().unwrap_or(0);
            let mut pb = RichProgress::new(
                tqdm!(
                    total = t as usize,
                    unit_scale = true,
                    unit_divisor = 1024,
                    unit = "B"
                ),
                vec![
                    // Column::Text("[bold blue]?".to_owned()),
                    Column::Animation,
                    Column::Percentage(1),
                    Column::Text("•".to_owned()),
                    Column::CountTotal,
                    Column::Text("•".to_owned()),
                    Column::Rate,
                    Column::Text("•".to_owned()),
                    Column::RemainingTime,
                ],
            );
            match response.status() {
                reqwest::StatusCode::OK => {
                    let mut f = std::fs::File::create(destination).unwrap();
                    while let Some(chunk) = response.chunk().await.unwrap() {
                        let write_size = f.write(&chunk).unwrap();
                        // time::sleep(Duration::from_millis(10)).await;
                        pb.update(write_size).unwrap();
                        let _ = wd.emit(
                            "init_resources",
                            Payload {
                                message: pb.render(),
                            },
                        );
                    }
                    Ok(())
                }
                status_code => {
                    println!(
                        "Error getting package, Message: {} Try again...",
                        status_code.canonical_reason().unwrap()
                    );
                    Err(anyhow::anyhow!("Error getting package"))
                }
            }
        }
        Err(error) => {
            println!("Error sending HTTP request during getting package");
            println!("{}", error.to_string());
            Err(error.into())
        }
    }
}
