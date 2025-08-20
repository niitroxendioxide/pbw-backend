use image::{ImageBuffer, Rgba};
use std::path::Path;

use crate::grid::{grid::DIMENSION, Grid};

pub fn grid_to_png(grid: &Grid) -> String {
    let width = DIMENSION as u32;
    let height = DIMENSION as u32;
    let mut img = ImageBuffer::new(width, height);


    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, Rgba(grid.get_pixel(x, y)));
        }
    }

    let image_id = "hi";
    let image_path = format!("output/{}.png", image_id);
    let path = Path::new(&image_path);

    image::save_buffer(
        path,
        &img.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ).unwrap();

    println!("Image saved to {}", image_path);

    image_path
}