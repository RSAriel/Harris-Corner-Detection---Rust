//Step 2
fn convolve(
    image: &[f32],
    width: u32,
    height: u32,
    kernel: &[[f32; 3]; 3]
) -> Vec<f32> {
    let mut output = vec![0.0; (width * height) as usize];
    
    let get_pixel = |x: i32, y: i32| -> f32 {
        if x < 0 || y < 0 || x >= width as i32 || y >= height as i32 {
            0.0
        } else {
            image[(y as u32 * width + x as u32) as usize]
        }
    };

    for y in 0..height as i32 {
        for x in 0..width as i32 {
            let mut acc = 0.0;

            for ky in 0..3 {
                for kx in 0..3 {
                    let px = x + kx - 1;
                    let py = y + ky - 1;
                    let pixel = get_pixel(px, py);
                    acc += pixel * kernel[ky as usize][kx as usize];
                }
            }

            output[(y as u32 * width + x as u32) as usize] = acc;
        }
    }

    output
}

//Step 2
pub fn compute_gradients(
    grayscale_vec: &[f32],
    width: u32,
    height: u32
) -> (Vec<f32>, Vec<f32>) {
    let kernel_x = [
        [-1.0, 0.0, 1.0],
        [-2.0, 0.0, 2.0],
        [-1.0, 0.0, 1.0]
    ];
    
    let kernel_y = [
        [1.0, 2.0, 1.0],
        [0.0, 0.0, 0.0],
        [-1.0, -2.0, -1.0]
    ];

    let grad_x = convolve(grayscale_vec, width, height, &kernel_x);
    let grad_y = convolve(grayscale_vec, width, height, &kernel_y);

    (grad_x, grad_y)
}

pub fn compute_gradient_magnitude(
    grad_x: &[f32],
    grad_y: &[f32]
) -> Vec<f32> {
    grad_x.iter()
        .zip(grad_y)
        .map(|(gx, gy)| (gx * gx + gy * gy).sqrt())
        .collect()
}

//Fazer uma função que forma o gradiente e já printa direto
//let grad_mag = compute_gradient_magnitude(&grad_x, &grad_y);
//visualize::save_normalized_f32_image(&grad_mag, width, height, "../images/grad_mag.png")?;



//********************************************** */
// Step 3
//********************************************** */

pub fn compute_gradient_products(
    grad_x: &[f32],
    grad_y: &[f32],
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let ixx = grad_x.iter().map(|gx| gx * gx).collect();
    let iyy = grad_y.iter().map(|gy| gy * gy).collect();
    let ixy = grad_x.iter().zip(grad_y).map(|(gx, gy)| gx * gy).collect();

    (ixx, iyy, ixy)
}

fn gaussian_kernel_3x3() -> [[f32; 3]; 3] {
    [
        [1.0, 2.0, 1.0],
        [2.0, 4.0, 2.0],
        [1.0, 2.0, 1.0],
    ].map(|row| row.map(|val| val / 16.0)) // normalize
}

pub fn smooth_gradient_products(
    ixx: &[f32],
    iyy: &[f32],
    ixy: &[f32],
    width: u32,
    height: u32,
) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let kernel = gaussian_kernel_3x3();

    let sxx = convolve(ixx, width, height, &kernel);
    let syy = convolve(iyy, width, height, &kernel);
    let sxy = convolve(ixy, width, height, &kernel);

    (sxx, syy, sxy)
}

