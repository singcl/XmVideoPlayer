use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub fn merge_ts(temp_dir: &str) {
    // 读取目录下所有文件路径
    // 可能包含其他非ts文件 如.DS_Store
    let mut paths = std::fs::read_dir(temp_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    paths.sort();

    let mut buffer = Vec::new();
    let out_path = PathBuf::from(temp_dir);
    let mut out_path = PathBuf::from(out_path.parent().unwrap());
    out_path.push("output.ts");

    for id in 0..paths.len() {
        let mut ts_path = PathBuf::from(temp_dir);
        ts_path.push(format!("{}.ts", id));
        if let Ok(mut f) = File::open(&ts_path) {
            f.read_to_end(&mut buffer).unwrap();
            std::fs::remove_file(&ts_path).unwrap();
        }
    }

    let mut f = File::create(&out_path).unwrap();
    f.write_all(&buffer).unwrap();
}
