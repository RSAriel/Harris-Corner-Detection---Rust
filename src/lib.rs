mod grayscale;
mod convolve;
mod harris;

use wasm_bindgen::prelude::*;
use grayscale::rgb_to_grayscale_vec;
use convolve::compute_gradients;
use harris::{compute_corner_response, non_maximum_suppression, draw_corners_on_data};

// Initialize panic handler for better error messages
#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub struct HarrisResult {
    image_data: Vec<u8>,
    corners: Vec<u32>, // Flattened array of (x, y) coordinates
}

#[wasm_bindgen]
impl HarrisResult {
    #[wasm_bindgen(getter)]
    pub fn image_data(&self) -> Vec<u8> {
        self.image_data.clone()
    }
    
    #[wasm_bindgen(getter)]
    pub fn corners(&self) -> Vec<u32> {
        self.corners.clone()
    }
}

#[wasm_bindgen]
pub fn harris_corner_detection(
    image_data: &[u8],
    width: u32,
    height: u32,
    k: f32,
    threshold: f32,
    corner_size: u32,
) -> HarrisResult {
    // Convert RGB image data to grayscale vector
    let grayscale_vec = rgb_to_grayscale_vec(image_data, width, height);
    
    // Step 2: Compute gradients using Sobel operators
    let (grad_x, grad_y) = compute_gradients(&grayscale_vec, width, height);
    
    // Step 3: Compute gradient products (structure tensor elements)
    let (ixx, iyy, ixy) = convolve::compute_gradient_products(&grad_x, &grad_y);
    
    // Step 4: Smooth gradient products with Gaussian kernel
    let (sxx, syy, sxy) = convolve::smooth_gradient_products(&ixx, &iyy, &ixy, width, height);
    
    // Step 5: Compute Harris corner response
    let corner_response = compute_corner_response(&sxx, &syy, &sxy, width, height, k);
    
    // Step 6: Apply non-maximum suppression to find corner points
    let corners = non_maximum_suppression(&corner_response, width, height, threshold);
    
    // Step 7: Draw corners on the original image
    let mut result_image = image_data.to_vec();
    draw_corners_on_data(&mut result_image, width, height, &corners, corner_size);
    
    // Flatten corners for JavaScript (x1, y1, x2, y2, ...)
    let flattened_corners: Vec<u32> = corners.iter()
        .flat_map(|&(x, y)| vec![x, y])
        .collect();
    
    HarrisResult {
        image_data: result_image,
        corners: flattened_corners,
    }
}

#[wasm_bindgen]
pub fn get_corner_coordinates(
    image_data: &[u8],
    width: u32,
    height: u32,
    k: f32,
    threshold: f32,
) -> Vec<u32> {
    let grayscale_vec = rgb_to_grayscale_vec(image_data, width, height);
    let (grad_x, grad_y) = compute_gradients(&grayscale_vec, width, height);
    let (ixx, iyy, ixy) = convolve::compute_gradient_products(&grad_x, &grad_y);
    let (sxx, syy, sxy) = convolve::smooth_gradient_products(&ixx, &iyy, &ixy, width, height);
    let corner_response = compute_corner_response(&sxx, &syy, &sxy, width, height, k);
    let corners = non_maximum_suppression(&corner_response, width, height, threshold);
    
    // Return flattened coordinates
    corners.iter()
        .flat_map(|&(x, y)| vec![x, y])
        .collect()
}