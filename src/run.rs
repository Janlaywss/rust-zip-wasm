use std::fs;
use std::fs::File;
use std::io::{copy, Read, Seek, Write};
use std::path::Path;
use std::str;

use zip::write::FileOptions;

pub fn main() {
    extract(Path::new("./tmp.zip"), Path::new("./tmp1"));
}

/// 解压
pub fn extract(test: &Path, mut target: &Path) {
    let zipfile = std::fs::File::open(&test).unwrap();
    let mut zip = zip::ZipArchive::new(zipfile).unwrap();

    if !target.exists() {
        fs::create_dir_all(target).map_err(|e| {
            println!("{}", e);
        });
    }
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        println!("Filename: {} {:?}", file.name(), file.sanitized_name());
    }
}
