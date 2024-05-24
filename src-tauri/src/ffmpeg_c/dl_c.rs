use anyhow::Context;
use std::os::windows::process::CommandExt;
use std::{
    fs::{create_dir_all, read_dir, remove_dir_all, remove_file, rename},
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    // process::{Command, ExitStatus},
};

use ffmpeg_sidecar::{
    download::{
        // auto_download,
        // download_ffmpeg_package
        // check_latest_version,
        // ffmpeg_download_url,
        parse_linux_version,
        parse_macos_version,
        // unpack_ffmpeg,
        UNPACK_DIRNAME,
    },
    paths::{ffmpeg_path, sidecar_dir},
    // version::ffmpeg_version,
};

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
    if let Ok(installed) = ffmpeg_is_installed().await {
        if installed {
            println!("FFmpeg is already installed! 🎉");
            println!("For demo purposes, we'll re-download and unpack it anyway.");
            println!("TIP: Use `auto_download()` to skip manual customization.");
            return Ok(());
        }
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

    // // Use the freshly installed FFmpeg to check the version number
    // let version = ffmpeg_version()?;
    // println!("FFmpeg version: {}", version);

    println!("Done! 🏁");

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
                    // Column::Animation,
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

// /// Verify whether ffmpeg is installed on the system. This will return true if
// /// there is an ffmpeg binary in the PATH, or in the same directory as the Rust
// /// executable.
// /// https://learn.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
// /// https://doc.rust-lang.org/stable/std/os/windows/process/trait.CommandExt.html
// pub fn ffmpeg_is_installed() -> bool {
//     #[cfg(target_os = "windows")]
//     return Command::new(ffmpeg_path())
//         .arg("-version")
//         .stderr(Stdio::null())
//         .stdout(Stdio::null())
//         .creation_flags(0x08000000)
//         .status()
//         .map(|s| s.success())
//         .unwrap_or_else(|_| false);
//     #[cfg(not(target_os = "windows"))]
//     return Command::new(ffmpeg_path())
//         .arg("-version")
//         .stderr(Stdio::null())
//         .stdout(Stdio::null())
//         .status()
//         .map(|s| s.success())
//         .unwrap_or_else(|_| false);
// }

/// Verify whether ffmpeg is installed on the system. This will return true if
/// there is an ffmpeg binary in the PATH, or in the same directory as the Rust
/// executable.
pub async fn ffmpeg_is_installed() -> anyhow::Result<bool> {
    #[cfg(target_os = "windows")]
    let code = tokio::process::Command::new(ffmpeg_path())
        .arg("-version")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .creation_flags(0x08000000)
        .spawn()?
        .wait()
        .await?;
    #[cfg(not(target_os = "windows"))]
    let code = tokio::process::Command::new(ffmpeg_path())
        .arg("-version")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()?
        .wait()
        .await?;
    Ok(code.success())
}

/// URL of a manifest file containing the latest published build of FFmpeg. The
/// correct URL for the target platform is baked in at compile time.
pub fn ffmpeg_manifest_url() -> anyhow::Result<&'static str> {
    if cfg!(not(target_arch = "x86_64")) {
        anyhow::bail!("Downloads must be manually provided for non-x86_64 architectures");
    }

    if cfg!(target_os = "windows") {
        Ok("https://www.gyan.dev/ffmpeg/builds/release-version")
    } else if cfg!(target_os = "macos") {
        Ok("https://evermeet.cx/ffmpeg/info/ffmpeg/release")
    } else if cfg!(target_os = "linux") {
        Ok("https://johnvansickle.com/ffmpeg/release-readme.txt")
    } else {
        anyhow::bail!("Unsupported platform")
    }
}

/// URL for the latest published FFmpeg release. The correct URL for the target
/// platform is baked in at compile time.
pub fn ffmpeg_download_url() -> anyhow::Result<&'static str> {
    if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        Ok("https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip")
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        Ok("https://johnvansickle.com/ffmpeg/releases/ffmpeg-release-amd64-static.tar.xz")
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        Ok("https://evermeet.cx/ffmpeg/getrelease")
    } else if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        Ok("https://www.osxexperts.net/ffmpeg6arm.zip") // Mac M1
    } else {
        anyhow::bail!("Unsupported platform; you can provide your own URL instead and call download_ffmpeg_package directly.")
    }
}

/// Invoke cURL on the command line to download a file, returning it as a string.
pub fn curl(url: &str) -> anyhow::Result<String> {
    #[cfg(target_os = "windows")]
    let mut child = Command::new("curl")
        .args(["-L", url])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()?;
    #[cfg(not(target_os = "windows"))]
    let mut child = Command::new("curl")
        .args(["-L", url])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = child.stdout.take().context("Failed to get stdout")?;

    let mut string = String::new();
    std::io::BufReader::new(stdout).read_to_string(&mut string)?;
    Ok(string)
}

/// Makes an HTTP request to obtain the latest version available online,
/// automatically choosing the correct URL for the current platform.
pub fn check_latest_version() -> anyhow::Result<String> {
    let string = curl(ffmpeg_manifest_url()?)?;

    if cfg!(target_os = "windows") {
        Ok(string)
    } else if cfg!(target_os = "macos") {
        parse_macos_version(&string).context("failed to parse version number (macos variant)")
    } else if cfg!(target_os = "linux") {
        parse_linux_version(&string).context("failed to parse version number (linux variant)")
    } else {
        Err(anyhow::Error::msg("Unsupported platform"))
    }
}

/// After downloading, unpacks the archive to a folder, moves the binaries to
/// their final location, and deletes the archive and temporary folder.
pub fn unpack_ffmpeg(from_archive: &PathBuf, binary_folder: &Path) -> anyhow::Result<()> {
    let temp_dirname = UNPACK_DIRNAME;
    let temp_folder = binary_folder.join(temp_dirname);
    create_dir_all(&temp_folder)?;

    // Extract archive
    #[cfg(target_os = "windows")]
    Command::new("tar")
        .arg("-xf")
        .arg(from_archive)
        .creation_flags(0x08000000)
        .current_dir(&temp_folder)
        .status()?
        .success()
        .then_some(())
        .context("Failed to unpack ffmpeg")?;

    // Extract archive
    #[cfg(not(target_os = "windows"))]
    Command::new("tar")
        .arg("-xf")
        .arg(from_archive)
        .current_dir(&temp_folder)
        .status()?
        .success()
        .then_some(())
        .context("Failed to unpack ffmpeg")?;

    // Move binaries
    let (ffmpeg, ffplay, ffprobe) = if cfg!(target_os = "windows") {
        let inner_folder = read_dir(&temp_folder)?
            .next()
            .context("Failed to get inner folder")??;
        (
            inner_folder.path().join("bin/ffmpeg.exe"),
            inner_folder.path().join("bin/ffplay.exe"),
            inner_folder.path().join("bin/ffprobe.exe"),
        )
    } else if cfg!(target_os = "linux") {
        let inner_folder = read_dir(&temp_folder)?
            .next()
            .context("Failed to get inner folder")??;
        (
            inner_folder.path().join("./ffmpeg"),
            inner_folder.path().join("./ffplay"), // <- no ffplay on linux
            inner_folder.path().join("./ffprobe"),
        )
    } else if cfg!(target_os = "macos") {
        (
            temp_folder.join("ffmpeg"),
            temp_folder.join("ffplay"),  // <-- no ffplay on mac
            temp_folder.join("ffprobe"), // <-- no ffprobe on mac
        )
    } else {
        anyhow::bail!("Unsupported platform");
    };

    // Move binaries
    let move_bin = |path: &Path| {
        let file_name = binary_folder.join(path.file_name().with_context(|| {
            format!("Path {} does not have a file_name", path.to_string_lossy())
        })?);
        rename(path, file_name)?;
        anyhow::Ok(())
    };

    move_bin(&ffmpeg)?;

    if ffprobe.exists() {
        move_bin(&ffprobe)?;
    }

    if ffplay.exists() {
        move_bin(&ffplay)?;
    }

    // Delete archive and unpacked files
    if temp_folder.exists() && temp_folder.is_dir() {
        remove_dir_all(&temp_folder)?;
    }

    if from_archive.exists() {
        remove_file(from_archive)?;
    }

    Ok(())
}
