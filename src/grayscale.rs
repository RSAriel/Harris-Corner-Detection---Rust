use image::RgbImage;

pub fn to_grayscale_vec(img: &RgbImage) -> (Vec<f32>, u32, u32) {
    let (width, height) = img.dimensions();
    let w_r = 0.299;
    let w_g = 0.587;
    let w_b = 0.114;
    
    let mut grayscale_vector = Vec::with_capacity((width * height) as usize);
    
    for (_x, _y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let gray_f = w_r * r as f32 + w_g * g as f32 + w_b * b as f32;
        grayscale_vector.push(gray_f / 255.0);
    }
    
    (grayscale_vector, width, height)
}



