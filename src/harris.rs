use image::{RgbImage, Rgb};


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


pub fn draw_corners(
    img: &mut RgbImage,
    corners: &[(u32, u32)],
    square_size: u32,
    color: Rgb<u8>,
) {
    let (width, height) = img.dimensions();
    let half = (square_size / 2) as i32;

    for &(x, y) in corners {
        let x = x as i32;
        let y = y as i32;

        for dy in -half..=half {
            for dx in -half..=half {
                let nx = x + dx;
                let ny = y + dy;

                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    img.put_pixel(nx as u32, ny as u32, color);
                }
            }
        }
    }
}