use core::exec;
use image::{io::Reader as ImageReader, RgbaImage};
use std::path::Path;

fn main() {
    let path = Path::new("packages/cli/src/test.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    let vec = img.as_bytes().to_vec();
    let scale_factor = 2; // 縮小率

    let result = exec(vec, scale_factor, width, height);

    println!("Result width: {}, height: {}", result.width(), result.height());

    // 修正：縮小した画像を保存する
    let _ = result.save("packages/cli/src/result2.png");
}