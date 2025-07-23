mod grayscale;
mod convolve;
mod visualize;
mod harris;

use grayscale::to_grayscale_vec;
use convolve::compute_gradients;
use harris::compute_corner_response;
use harris::non_maximum_suppression;
use harris::draw_corners;
use image::{RgbImage, Rgb};

pub fn detect_harris_corners(
    img: RgbImage,
    k: f32,
    threshold: f32,
    corner_radius: u32,
    corner_color: Rgb<u8>,
) -> RgbImage {
    let (grayscale_vec, width, height) = to_grayscale_vec(&img);
    let (grad_x, grad_y) = compute_gradients(&grayscale_vec, width, height);
    let (ixx, iyy, ixy) = convolve::compute_gradient_products(&grad_x, &grad_y);
    let (sxx, syy, sxy) = convolve::smooth_gradient_products(&ixx, &iyy, &ixy, width, height);
    
    let corner_response = compute_corner_response(&sxx, &syy, &sxy, width, height, k);
    let corners = non_maximum_suppression(&corner_response, width, height, threshold);
    
    let mut output_img = img;
    draw_corners(&mut output_img, &corners, corner_radius, corner_color);
    
    output_img
}

pub fn detect_harris_corners_default(img: RgbImage) -> RgbImage {
    let k = 0.04;
    let threshold = 0.1;
    let corner_radius = 5;
    let red = Rgb([255, 0, 0]);
    
    detect_harris_corners(img, k, threshold, corner_radius, red)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_harris_detector() {
        let img = image::open("../images/input.png").unwrap().to_rgb8();
        let result = detect_harris_corners_default(img);
        result.save("../images/test_output.png").unwrap();
    }
}