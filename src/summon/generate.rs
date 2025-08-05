use crate::summon::pool::summon_image_lu;
use crate::utils::driver::Error;

use std::io::Cursor;

use image::{DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use rayon::prelude::*;

pub fn combine_images(user_id: &str, image_names: Vec<String>) -> Result<(Vec<u8>, String), Error> {
    let lu = summon_image_lu();

    let images: Result<Vec<_>, _> = image_names
        .par_iter()
        .map(|name| {
            let (_, bytes) = lu
                .get(name.as_str())
                .ok_or_else(|| format!("Image not found: {}", name))?;
            image::load_from_memory(bytes)
                .map(|img| img.to_rgba8())
                .map_err(|e| Box::new(e) as Error)
        })
        .collect();

    let images = images?;
    if images.is_empty() {
        return Err("No images to combine".into());
    }

    let total_images = images.len();
    let cols = if total_images <= 10 { 5 } else { 10 };
    let rows = (total_images + cols - 1) / cols;

    let (width, height) = images[0].dimensions();
    let mut combined: RgbaImage = ImageBuffer::new(width * cols as u32, height * rows as u32);

    for (i, img) in images.iter().enumerate() {
        let x = (i % cols) as u32 * width;
        let y = (i / cols) as u32 * height;
        combined.copy_from(img, x, y)?;
    }

    let mut buffer = Cursor::new(Vec::new());
    let output_img = DynamicImage::ImageRgba8(combined);
    output_img.write_to(&mut buffer, image::ImageFormat::WebP)?;

    Ok((buffer.into_inner(), format!("{user_id}-summon.webp")))
}
