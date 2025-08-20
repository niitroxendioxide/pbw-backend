use image::{ImageBuffer, Rgba};
use std::path::Path;

use crate::grid::{Grid};

pub fn convert_from_grid(grid: &Grid) {
    let width: u32 = 32;
    let height = 32;
    let mut img = ImageBuffer::new(width, height);


    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgba([255, 0, 0, 255])); // Red pixel
        }
    }

    let image_path = "output.png";
    let path = Path::new(image_path);
    image::save_buffer(
        path,
        &img.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ).unwrap();

    println!("Image saved to {}", image_path);
}