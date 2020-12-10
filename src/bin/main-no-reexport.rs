use image::DynamicImage;
use no_reexport::convert_image;

fn main() {
    let src_image = DynamicImage::new_rgb8(800, 600);
    let dst_image = convert_image(src_image); // コンパイルエラー
}
