use image::{EncodableLayout, ImageBuffer, Rgba};

pub fn exec(buf: Vec<u8>, scale_factor: u32, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {

    let mut img: ImageBuffer<Rgba<u8>, Vec<_>> = ImageBuffer::from_raw(width, height, buf).unwrap();
    let block_size = scale_factor;

    let new_width = width / block_size;
    let new_height = height / block_size;

    let mut new_img = ImageBuffer::new(new_width, new_height);

    for y in 0..new_height {
        for x in 0..new_width {
            let mut r_sum: u32 = 0;
            let mut g_sum: u32 = 0;
            let mut b_sum: u32 = 0;
            let mut a_sum: u32 = 0;

            for dy in 0..block_size {
                for dx in 0..block_size {
                    // 元の画像の座標
                    // x, y は縮小後の画像(ブロック)の座標
                    // dx, dy はブロック内の座標
                    let pixel_x = x * block_size + dx;
                    let pixel_y = y * block_size + dy;

                    // 元の画像の座標が画像の範囲内であれば、そのピクセルの値を加算する
                    // 例外：ブロックの端のピクセルは、ブロックの範囲外になる場合がある
                    if pixel_x < width && pixel_y < height {
                        let pixel = img.get_pixel(pixel_x, pixel_y);
                        r_sum += pixel[0] as u32;
                        g_sum += pixel[1] as u32;
                        b_sum += pixel[2] as u32;
                        a_sum += pixel[3] as u32;
                    }
                }
            }

            // ブロック内のピクセル数
            let pixel_count = block_size * block_size;
            // ブロック内のピクセルの平均値
            let avg_pixel = Rgba([
                (r_sum / pixel_count) as u8,
                (g_sum / pixel_count) as u8,
                (b_sum / pixel_count) as u8,
                (a_sum / pixel_count) as u8,
            ]);

            new_img.put_pixel(x, y, avg_pixel);
        }
    }

    new_img


    // for h in (0..height).step_by(block_size as usize) {
    //     for w in (0..width).step_by(block_size as usize) {
    //         let mut r_sum: u32 = 0;
    //         let mut g_sum: u32 = 0;
    //         let mut b_sum: u32 = 0;
    //         let mut a_sum: u32 = 0;
    //         let mut safe_area_x = 0;
    //         let mut safe_area_y = 0;
    //         for y in 0..block_size as u32 {
    //             if height <= h + y as u32 {
    //                 break;
    //             }
    //             safe_area_y = y as u32 + 1;
    //             for x in 0..block_size as u32 {
    //                 if width <= w + x as u32 {
    //                     break;
    //                 }
    //                 r_sum += img.get_pixel(w + x, h + y).0[0] as u32;
    //                 g_sum += img.get_pixel(w + x, h + y).0[1] as u32;
    //                 b_sum += img.get_pixel(w + x, h + y).0[2] as u32;
    //                 a_sum += img.get_pixel(w + x, h + y).0[3] as u32;
    //                 safe_area_x = x + 1;
    //             }
    //         }

    //         for y in 0..safe_area_y {
    //             for x in 0..safe_area_x {
    //                 img.put_pixel(
    //                     w + x,
    //                     h + y,
    //                     Rgba([
    //                         (r_sum / (block_size * block_size)) as u8,
    //                         (g_sum / (block_size * block_size)) as u8,
    //                         (b_sum / (block_size * block_size)) as u8,
    //                         (a_sum / (block_size * block_size)) as u8,
    //                     ]),
    //                 )
    //             }
    //         }
    //     }
    // }
    // img.as_bytes().to_vec()
}