use image::imageops;
use reexport_dependent_types::{convert_image, DynamicImage};

fn main() {
    let src_image = DynamicImage::new_rgb8(800, 600);
    let dst_image = convert_image(src_image);
    imageops::flip_horizontal(&mut dst_image); // コンパイルエラー
}
