mod utils;

use std::str;
use wasm_bindgen::prelude::*;
use std::io::{Cursor, Seek, SeekFrom, Write, Read};

/// 解压
pub fn extract(x: &mut [u8]){
    let mut c = Cursor::new(Vec::new());
    c.write_all(x).unwrap();
    c.seek(SeekFrom::Start(0)).unwrap();
    let mut archive = zip::ZipArchive::new(c).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        if file.is_file() {
            let mut buffer = [0; 10];
            file.read_exact(&mut buffer).unwrap();
        }
    }

    // let mut file = match archive.by_name("manifest.yml") {
    //     Ok(file) => file,
    //     Err(..) => {
    //         println!("File test/lorem_ipsum.txt not found");
    //         return 2;
    //     }
    // };
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).unwrap();
    // log(&contents);
}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(message: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u8);
}

#[wasm_bindgen(catch)]
pub fn greet(x: &mut [u8]) {
    log("Hello from Rust!");
    extract(x);
}
