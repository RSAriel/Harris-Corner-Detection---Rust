mod grayscale;
mod convolve;
mod visualize;
mod harris;


use grayscale::to_grayscale_vec;
use convolve::compute_gradients;
use harris:: compute_corner_response;
use harris::non_maximum_suppression;
use harris::draw_corners;

use image::{Rgb, ImageReader}; 

const INPUT_PATH: &str = "../images/input.png";


//  Step 1:     Color to grayscale
//  Step 2:     Spatial derivative calculation
//  Step 3:     Structure tensor setup
//  Step 4:     Harris response calculation
//  Step 5:     Non-maximum suppression
//  Step 6:     Visualization


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (grayscale_vec, width, height) = to_grayscale_vec(INPUT_PATH)?;

    let (grad_x, grad_y) = compute_gradients(&grayscale_vec, width, height);
    let (ixx, iyy, ixy) = convolve::compute_gradient_products(&grad_x, &grad_y);
    let (sxx, syy, sxy) = convolve::smooth_gradient_products(&ixx, &iyy, &ixy, width, height);

    let k = 0.04; 
    let corner_response = compute_corner_response(&sxx, &syy, &sxy, width, height, k);
    let threshold = 0.1; 

    let mut img = ImageReader::open(INPUT_PATH)?.decode()?.to_rgb8();
    let corners = non_maximum_suppression(&corner_response, width, height, threshold);
    let red = Rgb([255, 0, 0]);

    draw_corners(&mut img, &corners, 5, red);
    img.save("../images/output_with_corners.png")?;

    Ok(())
}
