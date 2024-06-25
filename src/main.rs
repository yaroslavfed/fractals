use image::{Rgb, RgbImage};
use num::complex::Complex;
use human_panic::setup_panic;

fn main() {
    setup_panic!();

    let resolution = 2048;
    let target_x = -0.6582034218739634;
    let target_y = 0.44967917993930356;
    let max_iters = 5000;

    for scale_coefficient in 0..10 {
        let scale = 500_000.0 * scale_coefficient as f64;

        let x_min = target_x - (1.0 / scale);
        let x_max = target_x + (1.0 / scale);
        let y_min = target_y - (1.0 / scale);
        let y_max = target_y + (1.0 / scale);

        generate_set(
            format!("fractal_{}.png", scale).to_string(),
            max_iters,
            x_min,
            y_min,
            x_max,
            y_max,
            vec!["#1C448E", "#6F8695", "#CEC288", "#FFE381", "#DBFE87"],
            resolution,
        );
    }
}

fn num_iters(cx: f64, cy: f64, max_iters: u32) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(cx, cy);

    for iter in 0..max_iters {
        if z.norm() > 2.0 {
            return iter;
        }
        z = z * z + c;
    }

    return max_iters;
}

fn generate_set(
    file_name: String,
    max_iters: u32,
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
    colors: Vec<&str>,
    resolution: u32,
) {
    let mut buffer = RgbImage::new(resolution, resolution);
    let gradient = get_gradient(colors, max_iters);

    for x in 0..resolution {
        for y in 0..resolution {
            let x_percent = x as f64 / resolution as f64;
            let y_percent = y as f64 / resolution as f64;

            let cx = x_min + (x_max - x_min) * x_percent;
            let cy = y_min + (y_max - y_min) * y_percent;

            let iters = num_iters(cx, cy, max_iters);
            let pixel = buffer.get_pixel_mut(x, y);

            let color = gradient.get(iters as usize).unwrap_or(&[0, 0, 0]);
            *pixel = Rgb(*color);
        }
    }

    buffer.save(&file_name).unwrap();
}

fn get_gradient(gradient_colors: Vec<&str>, max_iters: u32) -> Vec<[u8; 3]> {
    let mut colors = vec![];
    let mut gradient_colors_rgb = vec![];

    for color in &gradient_colors {
        let rgb = hex_to_rgb(color).unwrap();
        gradient_colors_rgb.push([rgb[0], rgb[1], rgb[2]]);
    }

    for iter in 0..max_iters {
        let color_index = (iter as usize * (gradient_colors.len() - 1)) / max_iters as usize;
        let color_value = (iter as f64 * (gradient_colors.len() as f64 - 1f64)) / max_iters as f64;

        let value = color_value % 1f64;

        colors.push(lerp_color(
            &gradient_colors_rgb[color_index],
            &gradient_colors_rgb[color_index + 1],
            value));
    }

    return colors;
}

fn lerp_color(first_color: &[u8; 3], second_color: &[u8; 3], value: f64) -> [u8; 3] {
    return [
        (first_color[0] as f64 + (second_color[0] as f64 - first_color[0] as f64) * value) as u8,
        (first_color[1] as f64 + (second_color[1] as f64 - first_color[1] as f64) * value) as u8,
        (first_color[2] as f64 + (second_color[2] as f64 - first_color[2] as f64) * value) as u8
    ];
}

fn hex_to_rgb(hex: &&str) -> Result<Vec<u8>, String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err("Invalid HEX color length".to_string());
    }

    let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid HEX color")?;
    let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid HEX color")?;
    let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid HEX color")?;

    return Ok(vec![r, g, b]);
}