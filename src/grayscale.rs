use image::{RgbImage, Rgb, ImageReader}; 
use std::path::Path;

pub fn to_grayscale(input_path: &str, output_path: &str) -> Result<(), image::ImageError> {

    let img: image::ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(&Path::new(input_path))?.to_rgb8();
    let mut output: RgbImage = RgbImage::new(img.width(), img.height());

    let w_r = 0.299;
    let w_g = 0.587;
    let w_b = 0.114;

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b] = pixel.0;
        let gray = (w_r * r as f32 + w_g * g as f32 + w_b * b as f32).round() as u8;
        output.put_pixel(x, y, Rgb([gray, gray, gray]));
    }

    output.save(output_path)
}

pub fn to_grayscale_vec(input_path: &str) -> Result<(Vec<f32>, u32, u32), image::ImageError> {
    let img = image::open(&Path::new(input_path))?.to_rgb8();

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

    Ok((grayscale_vector, width, height))
}

//The code below is to test if the Sobel Operator works correctly, as the first test with an RGB file the image was dimmer.
//I suspect maybe that the parameters for the grayscale are different or my conversion is not correct.
//The test with an already grayscale image works fine, so I will leave it here for now.
//https://en.wikipedia.org/wiki/Sobel_operator

pub fn grayscale_image_to_vec(
    input_path: &str
) -> Result<(Vec<f32>, u32, u32), image::ImageError> {
    let gray_img = ImageReader::open(Path::new(input_path))?
        .with_guessed_format()?
        .decode()?
        .to_luma8();

    let (width, height) = gray_img.dimensions();

    let buffer = gray_img.into_raw();
    let vec = buffer
        .into_iter()
        .map(|v| v as f32 / 255.0)
        .collect();

    Ok((vec, width, height))
}

