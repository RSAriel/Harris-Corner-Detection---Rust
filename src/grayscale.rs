/// Convert RGB image data to grayscale vector
/// image_data: RGBA or RGB bytes from canvas/image
/// width, height: image dimensions
/// Returns: normalized grayscale values (0.0 to 1.0)
pub fn rgb_to_grayscale_vec(image_data: &[u8], width: u32, height: u32) -> Vec<f32> {
    let w_r = 0.299;
    let w_g = 0.587;
    let w_b = 0.114;
    
    let pixel_count = (width * height) as usize;
    let mut grayscale_vector = Vec::with_capacity(pixel_count);
    
    // Handle both RGB (3 bytes) and RGBA (4 bytes) formats
    let bytes_per_pixel = if image_data.len() == pixel_count * 4 { 4 } else { 3 };
    
    for i in 0..pixel_count {
        let base_idx = i * bytes_per_pixel;
        let r = image_data[base_idx] as f32;
        let g = image_data[base_idx + 1] as f32;
        let b = image_data[base_idx + 2] as f32;
        
        let gray_f = w_r * r + w_g * g + w_b * b;
        grayscale_vector.push(gray_f / 255.0); // Normalize to 0.0-1.0
    }
    
    grayscale_vector
}

/// Convert grayscale float vector back to RGB image data
pub fn grayscale_to_rgb(grayscale: &[f32], width: u32, height: u32) -> Vec<u8> {
    let mut rgb_data = Vec::with_capacity((width * height * 3) as usize);
    
    for &gray_val in grayscale {
        let gray_u8 = (gray_val * 255.0).clamp(0.0, 255.0) as u8;
        rgb_data.push(gray_u8); // R
        rgb_data.push(gray_u8); // G
        rgb_data.push(gray_u8); // B
    }
    
    rgb_data
}