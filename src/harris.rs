/// Compute Harris corner response
pub fn compute_corner_response(
    s_xx: &[f32],
    s_yy: &[f32],
    s_xy: &[f32],
    width: u32,
    height: u32,
    k: f32,
) -> Vec<f32> {
    let size = (width * height) as usize;
    let mut response = Vec::with_capacity(size);
    
    for i in 0..size {
        let det = s_xx[i] * s_yy[i] - s_xy[i] * s_xy[i];
        let trace = s_xx[i] + s_yy[i];
        let r = det - k * trace * trace;
        response.push(r);
    }
    
    response
}

/// Apply non-maximum suppression to find corner points
pub fn non_maximum_suppression(
    response: &[f32],
    width: u32,
    height: u32,
    threshold: f32,
) -> Vec<(u32, u32)> {
    let mut corners = Vec::new();
    
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let idx = (y * width + x) as usize;
            let val = response[idx];
            
            if val < threshold {
                continue;
            }
            
            let mut is_max = true;
            // Check 3x3 neighborhood
            for ny in (y - 1)..=(y + 1) {
                for nx in (x - 1)..=(x + 1) {
                    if nx == x && ny == y {
                        continue;
                    }
                    let n_idx = (ny * width + nx) as usize;
                    if response[n_idx] >= val {
                        is_max = false;
                        break;
                    }
                }
                if !is_max {
                    break;
                }
            }
            
            if is_max {
                corners.push((x, y));
            }
        }
    }
    
    corners
}

/// Draw corners on raw image data (RGB format)
pub fn draw_corners_on_data(
    image_data: &mut [u8],
    width: u32,
    height: u32,
    corners: &[(u32, u32)],
    square_size: u32,
) {
    let half = (square_size / 2) as i32;
    let red = [255u8, 0u8, 0u8]; // Red color for corners
    
    // Determine bytes per pixel (RGB=3, RGBA=4)
    let pixel_count = (width * height) as usize;
    let bytes_per_pixel = if image_data.len() == pixel_count * 4 { 4 } else { 3 };
    
    for &(x, y) in corners {
        let x = x as i32;
        let y = y as i32;
        
        for dy in -half..=half {
            for dx in -half..=half {
                let nx = x + dx;
                let ny = y + dy;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let idx = (ny as u32 * width + nx as u32) as usize * bytes_per_pixel;
                    
                    // Set RGB values to red
                    if idx + 2 < image_data.len() {
                        image_data[idx] = red[0];     // R
                        image_data[idx + 1] = red[1]; // G
                        image_data[idx + 2] = red[2]; // B
                        // Leave alpha channel unchanged if it exists
                    }
                }
            }
        }
    }
}

/// Draw corners with custom color
pub fn draw_corners_with_color(
    image_data: &mut [u8],
    width: u32,
    height: u32,
    corners: &[(u32, u32)],
    square_size: u32,
    color: [u8; 3],
) {
    let half = (square_size / 2) as i32;
    
    let pixel_count = (width * height) as usize;
    let bytes_per_pixel = if image_data.len() == pixel_count * 4 { 4 } else { 3 };
    
    for &(x, y) in corners {
        let x = x as i32;
        let y = y as i32;
        
        for dy in -half..=half {
            for dx in -half..=half {
                let nx = x + dx;
                let ny = y + dy;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let idx = (ny as u32 * width + nx as u32) as usize * bytes_per_pixel;
                    
                    if idx + 2 < image_data.len() {
                        image_data[idx] = color[0];     // R
                        image_data[idx + 1] = color[1]; // G
                        image_data[idx + 2] = color[2]; // B
                    }
                }
            }
        }
    }
}