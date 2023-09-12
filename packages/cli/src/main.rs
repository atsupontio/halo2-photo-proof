use core::*;
use image::{io::Reader as ImageReader, EncodableLayout, RgbaImage};
use std::path::Path;
use std::{thread, time};

fn main() {
    let now = time::Instant::now();
    let path = Path::new("packages/cli/src/test100X61.png");
    let img = ImageReader::open(path).unwrap().decode().unwrap();
    let width = img.width();
    let height = img.height();
    let vec = img.as_bytes().to_vec();
    let (converted, _, _, _, _) = core::exec(vec.clone(), 2, width, height);
    let s_width = converted.width();
    let s_height = converted.height();
    let s_img = RgbaImage::from_vec(s_width, s_height, converted.as_bytes().to_vec()).unwrap();
    let s_vec = s_img.as_bytes().to_vec();

    let proof = core::create_img_proof(vec, width, height);
    let result = core::verify_img(proof, s_vec, s_width, s_height);

    println!("result: {}", result);
    assert_eq!(result, true);
    // 最初の時刻からの経過時間を表示
    println!("time: {:?}", now.elapsed());
}
