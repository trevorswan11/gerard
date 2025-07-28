use crate::utils::driver::Error;

use std::{collections::HashMap, io::Cursor, path::PathBuf};

use image::{DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use rayon::prelude::*;
use walkdir::WalkDir;

pub static IMAGE_DIR: &str = "assets/summon_images";
pub static OUTPUT_DIR: &str = "assets/gen";

pub fn combine_images(user_id: &str, image_names: Vec<String>) -> Result<(Vec<u8>, String), Error> {
    let image_paths = find_images(image_names, IMAGE_DIR)?;
    if image_paths.is_empty() {
        return Err("No images found".into());
    }

    let images: Result<Vec<_>, _> = image_paths
        .par_iter()
        .map(|path| {
            image::open(path)
                .map(|img| img.to_rgba8())
                .map_err(|e| Box::new(e) as Error)
        })
        .collect();
    let images = images?;

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

fn find_images(image_names: Vec<String>, folder_path: &str) -> Result<Vec<PathBuf>, Error> {
    let file_map: HashMap<String, PathBuf> = WalkDir::new(folder_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let name = e.file_name().to_str()?.to_owned();
            Some((name, e.into_path()))
        })
        .collect();

    image_names
        .iter()
        .map(|name| {
            file_map
                .get(name)
                .cloned()
                .ok_or_else(|| format!("Image not found: {name}").into())
        })
        .collect()
}
