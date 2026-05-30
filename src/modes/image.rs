use anyhow::{Context, Result};
use image::{imageops::FilterType, GenericImageView};
use std::fs;
use crate::cli::ImageArgs;

const RAMP_SIMPLE: &str = "@%#*+=-:. ";
const RAMP_DETAILED: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";

pub fn run(args: ImageArgs) -> Result<()> {
    let ascii = convert_to_ascii(&args.path, args.width, args.detailed, args.invert, args.center)
        .with_context(|| format!("Failed to process image: {}", args.path))?;

    match args.output {
        Some(path) => {
            fs::write(&path, &ascii).with_context(|| format!("Failed to write to {}", path))?;
            println!("Saved to {}", path);
        }
        None => print!("{}", ascii),
    }

    Ok(())
}

pub fn convert_to_ascii(path: &str, width: u32, detailed: bool, invert: bool, center: bool) -> Result<String> {
    let img = image::open(path)?;
    let (orig_w, orig_h) = img.dimensions();

    let height = ((orig_h as f64 / orig_w as f64) * width as f64 / 2.0) as u32;
    let height = height.max(1);

    let resized = img.resize_exact(width, height, FilterType::Lanczos3);
    let gray = resized.to_luma8();

    let ramp: Vec<char> = if detailed {
        RAMP_DETAILED.chars().collect()
    } else {
        RAMP_SIMPLE.chars().collect()
    };

    let ramp_len = ramp.len();
    let term_width = termsize::get().map(|s| s.cols as usize).unwrap_or(width as usize);
    let padding = if center && term_width > width as usize {
        (term_width - width as usize) / 2
    } else {
        0
    };
    let pad_str = " ".repeat(padding);

    let mut output = String::new();

    for y in 0..height {
        output.push_str(&pad_str);
        for x in 0..width {
            let luma = gray.get_pixel(x, y)[0] as usize;
            let idx = if invert {
                luma * (ramp_len - 1) / 255
            } else {
                (255 - luma) * (ramp_len - 1) / 255
            };
            output.push(ramp[idx]);
        }
        output.push('\n');
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GrayImage, Luma};

    fn make_test_image(r: u8, path: &str) {
        let mut img = GrayImage::new(10, 10);
        for pixel in img.pixels_mut() {
            *pixel = Luma([r]);
        }
        img.save(path).unwrap();
    }

    #[test]
    fn black_image_uses_dense_char() {
        make_test_image(0, "/tmp/black.png");
        let result = convert_to_ascii("/tmp/black.png", 10, false, false, false).unwrap();
        assert!(result.contains('@'));
    }

    #[test]
    fn white_image_uses_sparse_char() {
        make_test_image(255, "/tmp/white.png");
        let result = convert_to_ascii("/tmp/white.png", 10, false, false, false).unwrap();
        assert!(result.contains(' '));
    }

    #[test]
    fn invert_reverses_mapping() {
        make_test_image(0, "/tmp/inv.png");
        let normal = convert_to_ascii("/tmp/inv.png", 10, false, false, false).unwrap();
        let inverted = convert_to_ascii("/tmp/inv.png", 10, false, true, false).unwrap();
        assert_ne!(normal, inverted);
    }

    #[test]
    fn output_has_correct_line_count() {
        make_test_image(128, "/tmp/lines.png");
        let result = convert_to_ascii("/tmp/lines.png", 10, false, false, false).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 5);
    }

    #[test]
    fn detailed_ramp_produces_output() {
        make_test_image(100, "/tmp/detail.png");
        let result = convert_to_ascii("/tmp/detail.png", 10, true, false, false).unwrap();
        assert!(!result.is_empty());
    }
}