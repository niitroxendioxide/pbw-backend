use image::{codecs::gif::{GifEncoder, Repeat}, Delay, Frame, ImageBuffer, RgbaImage};
use std::path::Path;
use std::fs::File;
use uuid::Uuid;

use crate::grid::{Grid};

pub const FRAMES_PER_SECOND: u32 = 24;
pub const OUTPUT_DIRECTORY: &str = "output";

pub fn grid_to_png(grid: &Grid) -> String {
    let width = grid.width as u32;
    let height = grid.height as u32;
    let first_frame = grid.get_frame(1);
    let mut img = ImageBuffer::new(width, height);


    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, first_frame.get_pixel_as_rgba(x, y));
        }
    }

    let image_id = Uuid::new_v4();
    let image_path = format!("{}/{}.png", OUTPUT_DIRECTORY, image_id);
    let path = Path::new(&image_path);

    image::save_buffer(
        path,
        &img.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ).unwrap();

    image_path
}

pub fn grid_to_gif(grid: &Grid) -> String {
    let width = grid.width as u32;
    let height = grid.height as u32;

    let image_id = Uuid::new_v4();
    let image_path = format!("{}/{}.gif", OUTPUT_DIRECTORY, image_id);
    let path = Path::new(&image_path);

    let file = File::create(path).unwrap();
    let mut encoder = GifEncoder::new(file);
    let mut frames = Vec::new();

    if let Err(e) = encoder.set_repeat(Repeat::Infinite) {
        eprintln!("Failed to set GIF repeat: {}", e);

        return String::new();
    }

    for frame in &grid.frames {
        let mut img: RgbaImage = ImageBuffer::new(width, height);
        for x in 0..width {
            for y in 0..height {
                img.put_pixel(x, y, frame.get_pixel_as_rgba(x, y));
            }
        }

        let frame = Frame::from_parts(img, 0, 0, Delay::from_numer_denom_ms(1, FRAMES_PER_SECOND));
        
        frames.push(frame);
    }

    encoder.encode_frames(frames.into_iter()).unwrap();

    image_path
}