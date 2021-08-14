use std::str;
use wasm_bindgen::prelude::*;
use std::io::{Cursor, Seek, SeekFrom, Write, Read};
use zip::ZipArchive;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct ZipExtract {
    archive: ZipArchive<Cursor<Vec<u8>>>,
}

#[wasm_bindgen]
impl ZipExtract {
    #[wasm_bindgen(constructor)]
    pub fn new(u8: &mut [u8]) -> Self {
        log("Hello from Rust!");
        let mut c = Cursor::new(Vec::new());
        c.write_all(u8).unwrap();
        c.seek(SeekFrom::Start(0)).unwrap();
        let archive = zip::ZipArchive::new(c).unwrap();

        ZipExtract {
            archive
        }
    }

    pub fn extract(&mut self) {
        for i in 0..self.archive.len() {
            let mut file = self.archive.by_index(i).unwrap();
            if file.is_file() {
                let mut buffer = [0; 10];
                file.read_exact(&mut buffer).unwrap();
            }
        }
    }

}

#[wasm_bindgen]
extern {
    fn alert(message: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u8);
}
