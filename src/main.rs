use std::default::Default;
use image::GenericImageView;
use show_image::{create_window, ImageInfo, ImageView};

#[show_image::main]
fn main() {
    let source_image = image::open("tests/data/hoge.png")
        .expect("failed to read image")
        .to_luma8();
    let detection = edge_detection::canny(
        source_image,
        1.2,  // sigma
        0.2,  // strong threshold
        0.01, // weak threshold
    ).as_image();
    let image = ImageView::new(ImageInfo::rgb8(detection.width(), detection.height()), detection.as_bytes());
    let window = create_window("image", Default::default()).unwrap();
    window.set_image("detect", image).unwrap();
}