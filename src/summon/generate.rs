use crate::utils::driver::Error;

use std::{fs, path::Path, path::PathBuf};

use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, RgbaImage};
use walkdir::WalkDir;

pub static IMAGE_DIR: &str = "assets/summon_images";
pub static OUTPUT_DIR: &str = "assets/gen";

pub fn combine_images(user_id: &str, image_names: Vec<String>) -> Result<String, Error> {
    let image_paths = find_images(image_names, IMAGE_DIR)?;
    if image_paths.is_empty() {
        return Err("No images found".into());
    }

    let mut images: Vec<DynamicImage> = vec![];
    for path in image_paths {
        if let Ok(img) = image::open(&path) {
            images.push(img.to_rgba8().into());
        } else {
            return Err(format!("Failed to open image: {:?}", &path).into());
        }
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

    let output_filename = format!("{}-summon.png", user_id);
    let output_path = Path::new(OUTPUT_DIR).join(output_filename);
    fs::create_dir_all(OUTPUT_DIR)?;

    let output_img = DynamicImage::ImageRgba8(combined).to_rgba8();
    output_img.save(&output_path)?;

    output_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or(format!("Cannot return path: {:?}", output_path).into())
}

fn find_images(image_names: Vec<String>, folder_path: &str) -> Result<Vec<PathBuf>, Error> {
    let mut paths = Vec::new();

    for name in image_names {
        for entry in WalkDir::new(folder_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(filename) = entry.file_name().to_str() {
                if filename == name {
                    paths.push(entry.path().to_path_buf());
                    break;
                }
            } else {
                return Err(format!("Invalid filename: {:?}", entry.file_name()).into());
            }
        }
    }

    Ok(paths)
}
