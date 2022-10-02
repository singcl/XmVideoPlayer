use m3u8_rs::{ Playlist};

pub fn parse_m3u8_list(list_string: &str, m3u8_url: &str) -> Vec<String> {
    let mut url_iter = m3u8_url.split('/');
    url_iter.next_back();
    let url_base = url_iter.collect::<Vec<_>>().join("/");
    let mut url_list = Vec::<String>::new();

    match m3u8_rs::parse_playlist_res(list_string.as_bytes()) {
        Ok(Playlist::MasterPlaylist(_)) => {
            // println!("Master playlist:\n{:?}", pl)
        }
        Ok(Playlist::MediaPlaylist(pl)) => {
            // println!("Media playlist:\n{:?}", pl)
            for seg in pl.segments {
                url_list.push(format!("{}/{}", url_base, seg.uri));
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
    url_list
}
