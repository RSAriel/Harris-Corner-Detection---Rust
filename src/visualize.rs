use image::{GrayImage, Luma};

pub fn normalize_to_u8(data: &[f32]) -> Vec<u8> {
    let min = data.iter().cloned().fold(f32::INFINITY, f32::min);
    let max = data.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let range = max - min;
    data.iter()
        .map(|v| ((v - min) / range * 255.0).clamp(0.0, 255.0) as u8)
        .collect()
}

pub fn create_grayscale_image(data: &[u8], width: u32, height: u32) -> GrayImage {
    let mut img = GrayImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            img.put_pixel(x, y, Luma([data[i]]));
        }
    }
    img
}

pub fn create_normalized_f32_image(data: &[f32], width: u32, height: u32) -> GrayImage {
    let data_u8 = normalize_to_u8(data);
    create_grayscale_image(&data_u8, width, height)
}