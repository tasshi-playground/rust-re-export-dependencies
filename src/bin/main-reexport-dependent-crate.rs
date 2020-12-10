use reexport_dependent_crate::convert_image;
use reexport_dependent_crate::image::{imageops, DynamicImage};

fn main() {
    let src_image = DynamicImage::new_rgb8(800, 600);
    let mut dst_image = convert_image(src_image);
    imageops::flip_horizontal(&mut dst_image);
}
