use kdam::term::Colorizer;
use reqwest::Client;
// use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct Releases {
    url: String,
    version: String,
}

// json lick this [{"url":"xxxx", version: "0.6.5"}, {"url":"xxxx", version: "0.6.4"}]

pub(super) async fn check_for_new_release(client: &Client) {
    if let Ok(response) = client
        .get("https://github.com/singcl/XmVideoPlayer/releases")
        .send().await
    {
        if let Ok(text) = response.text().await {
            if let Ok(releases) = serde_json::from_str::<Vec<Releases>>(&text) {
                if let Some(latest) = releases.first() {
                    if latest.version != env!("CARGO_PKG_VERSION") {
                        println!(
                            "     {} a new release of vsd is available {} -> {}\n            {}",
                            "Notice".colorize("bold cyan"),
                            env!("CARGO_PKG_VERSION").colorize("bold red"),
                            latest.version.colorize("bold green"),
                            latest.url
                        );
                    }
                }
            }
        }
    }
}
