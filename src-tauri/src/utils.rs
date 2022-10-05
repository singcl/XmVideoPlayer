use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{Read, Write};
use std::path::PathBuf;

pub fn hash_str(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    // println!("Hash is {:x}!", &m3_hash);
    hasher.finish()
}

// 带缓冲区的读写
pub fn read_file_buffer(filepath: &PathBuf, out_file: &mut File) {
    const BUFFER_LEN: usize = 512;
    let mut buffer = [0u8; BUFFER_LEN];
    if let Ok(mut f) = File::open(&filepath) {
        loop {
            let read_count = f.read(&mut buffer).unwrap();
            // println!("read_size: {}", read_count);
            // let write_count =
            out_file.write(&mut buffer[0..read_count]).unwrap();
            // println!("write_size: {}", write_count);
            if read_count != BUFFER_LEN {
                std::fs::remove_file(&filepath).unwrap();
                break;
            }
        }
    }
}

// 读取目录下所有文件路径
// 可能包含其他非ts文件 如.DS_Store
// TODO:过滤掉无关的文件
pub fn read_dir_all(temp_dir: &str) {
    let mut paths = std::fs::read_dir(temp_dir)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, std::io::Error>>()
        .unwrap();
    // 非数字顺序
    paths.sort();
}
