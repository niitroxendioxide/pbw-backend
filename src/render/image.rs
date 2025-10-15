use image::{codecs::gif::{GifEncoder, Repeat}, Delay, Frame, ImageBuffer, RgbaImage};
use std::path::Path;
use std::fs::{File, create_dir_all};
use uuid::Uuid;

use crate::grid::{Grid};

pub const FRAMES_PER_SECOND: u32 = 24;
pub const OUTPUT_DIRECTORY: &str = "output/images";

pub fn grid_to_png(grid: &Grid) -> (String, String) {
    let width = grid.size as u32;
    let height = grid.size as u32;
    let first_frame = grid.get_frame(1);
    let mut img = ImageBuffer::new(width, height);


    for x in 0..width {
        for y in 0..height {
            img.put_pixel(x, y, first_frame.get_pixel_as_rgba(x, y));
        }
    }

    let image_id = Uuid::new_v4();
    
    // Create output directory if it doesn't exist
    if let Err(e) = create_dir_all(OUTPUT_DIRECTORY) {
        eprintln!("Failed to create output directory: {}", e);
    }
    
    let image_path = format!("{}/{}.png", OUTPUT_DIRECTORY, image_id);
    let path = Path::new(&image_path);

    if let Err(e) = image::save_buffer(
        path,
        &img.into_raw(),
        width,
        height,
        image::ColorType::Rgba8,
    ) {
        eprintln!("Failed to save PNG image: {}", e);
    }

    (image_path, image_id.to_string())
}

pub fn grid_to_gif(grid: &Grid) -> (String, String) {
    let width = grid.size as u32;
    let height = grid.size as u32;

    let image_id = Uuid::new_v4();
    
    // Create output directory if it doesn't exist
    if let Err(e) = create_dir_all(OUTPUT_DIRECTORY) {
        eprintln!("Failed to create output directory: {}", e);
    }
    
    let image_path = format!("{}/{}.gif", OUTPUT_DIRECTORY, image_id);
    let path = Path::new(&image_path);

    let file = match File::create(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create GIF file: {}", e);
            return (String::new(), String::new());
        }
    };
    
    let mut encoder = GifEncoder::new(file);
    let mut frames = Vec::new();

    if let Err(e) = encoder.set_repeat(Repeat::Infinite) {
        eprintln!("Failed to set GIF repeat: {}", e);
        return (String::new(), String::new());
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

    if let Err(e) = encoder.encode_frames(frames.into_iter()) {
        eprintln!("Failed to encode GIF frames: {}", e);
        return (String::new(), String::new());
    }

    (image_path, image_id.to_string())
}
