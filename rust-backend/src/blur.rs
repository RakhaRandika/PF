use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rayon::prelude::*;
use std::io::Cursor;

use crate::models::FaceBox;

pub fn process_image_with_blur(
    image_data: &[u8],
    faces: &[FaceBox],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let img = image::load_from_memory(image_data)?;
    let mut img = img.to_rgba8();

    let blur_regions: Vec<_> = faces
        .par_iter()
        .map(|face| {
            let x1 = face.x1.max(0) as u32;
            let y1 = face.y1.max(0) as u32;
            let x2 = face.x2.min(img.width() as i32) as u32;
            let y2 = face.y2.min(img.height() as i32) as u32;

            if x2 <= x1 || y2 <= y1 {
                return None;
            }

            let face_region = img.view(x1, y1, x2 - x1, y2 - y1).to_image();

            let blurred = apply_gaussian_blur(&face_region, 25.0);

            Some((x1, y1, blurred))
        })
        .collect();

    for region in blur_regions.into_iter().flatten() {
        let (x, y, blurred) = region;
        
        for (dx, dy, pixel) in blurred.enumerate_pixels() {
            img.put_pixel(x + dx, y + dy, *pixel);
        }
    }

    // Convert to JPEG
    let mut output = Vec::new();
    let cursor = Cursor::new(&mut output);
    
    let dynamic_img = DynamicImage::ImageRgba8(img);
    dynamic_img.write_to(
        &mut std::io::BufWriter::new(cursor),
        image::ImageOutputFormat::Jpeg(90),
    )?;

    Ok(output)
}

fn apply_gaussian_blur(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    sigma: f32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let (width, height) = img.dimensions();
    
    // Create kernel
    let kernel_size = ((sigma * 6.0).ceil() as usize) | 1; // Ensure odd size
    let kernel = create_gaussian_kernel(kernel_size, sigma);
    let half_kernel = kernel_size / 2;

    // Apply blur using parallel processing
    let pixels: Vec<_> = (0..height)
        .into_par_iter()
        .flat_map(|y| {
            (0..width)
                .map(|x| {
                    let mut r = 0.0;
                    let mut g = 0.0;
                    let mut b = 0.0;
                    let mut a = 0.0;
                    let mut weight_sum = 0.0;

                    for ky in 0..kernel_size {
                        for kx in 0..kernel_size {
                            let px = (x as i32 + kx as i32 - half_kernel as i32)
                                .max(0)
                                .min(width as i32 - 1) as u32;
                            let py = (y as i32 + ky as i32 - half_kernel as i32)
                                .max(0)
                                .min(height as i32 - 1) as u32;

                            let pixel = img.get_pixel(px, py);
                            let weight = kernel[ky][kx];

                            r += pixel[0] as f32 * weight;
                            g += pixel[1] as f32 * weight;
                            b += pixel[2] as f32 * weight;
                            a += pixel[3] as f32 * weight;
                            weight_sum += weight;
                        }
                    }

                    Rgba([
                        (r / weight_sum) as u8,
                        (g / weight_sum) as u8,
                        (b / weight_sum) as u8,
                        (a / weight_sum) as u8,
                    ])
                })
                .collect::<Vec<_>>()
        })
        .collect();

    ImageBuffer::from_vec(width, height, pixels.into_iter().flat_map(|p| p.0).collect())
        .unwrap()
}

fn create_gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let center = size / 2;
    let mut sum = 0.0;

    for i in 0..size {
        for j in 0..size {
            let x = i as f32 - center as f32;
            let y = j as f32 - center as f32;
            let value = (-(x * x + y * y) / (2.0 * sigma * sigma)).exp();
            kernel[i][j] = value;
            sum += value;
        }
    }

    // Normalize
    for i in 0..size {
        for j in 0..size {
            kernel[i][j] /= sum;
        }
    }

    kernel
}
