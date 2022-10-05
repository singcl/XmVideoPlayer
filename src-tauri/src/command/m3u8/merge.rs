use crate::utils;
use std::fs::File;
use std::path::PathBuf;

pub fn merge_ts(url_list_entity_hash: &Vec<u64>, temp_dir: &str, out_path: &str) {
    // let out_path = PathBuf::from(temp_dir);
    // let mut out_path = PathBuf::from(out_path.parent().unwrap());
    // out_path.push("output.ts");

    let mut f = File::create(&out_path).unwrap();
    for item in 0..url_list_entity_hash.len() {
        let id = url_list_entity_hash.get(item).unwrap();
        let mut ts_path = PathBuf::from(temp_dir);
        ts_path.push(format!("{}.ts", id));
        utils::read_file_buffer(&ts_path, &mut f);
    }
}
