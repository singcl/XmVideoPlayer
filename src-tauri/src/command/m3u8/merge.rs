use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub fn merge_ts(id_range: usize, temp_dir: &str) {
    let mut buffer = Vec::new();
    let out_path = PathBuf::from(temp_dir);
    let mut out_path = PathBuf::from(out_path.parent().unwrap());
    out_path.push("output.ts");

    for id in 0..id_range {
        let mut ts_path = PathBuf::from(temp_dir);
        ts_path.push(format!("{}.ts", id));
        let mut f = File::open(&ts_path).unwrap();
        f.read_to_end(&mut buffer).unwrap();
        std::fs::remove_file(&ts_path).unwrap();
    }

    let mut f = File::create(&out_path).unwrap();
    f.write_all(&buffer).unwrap();
}
