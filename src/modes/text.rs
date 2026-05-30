use anyhow::Result;
use crate::cli::TextArgs;
use crate::font::FONT_5X7;
use crate::utils::colorize;

pub fn run(args: TextArgs) -> Result<()> {
    let scale = args.scale.clamp(1, 4);
    let block: char = args.block.chars().next().unwrap_or('█');
    let rows = render_banner(&args.input, block, scale);

    println!();
    for row in &rows {
        let line = colorize(row, &args.color);
        println!("{}", line);
    }
    println!();

    Ok(())
}

pub fn render_banner(text: &str, block: char, scale: usize) -> Vec<String> {
    let mut rows: Vec<String> = vec![String::new(); 7];

    for ch in text.chars() {
        let code = ch as usize;

        if code < 32 || (code - 32) >= FONT_5X7.len() {
            for row in rows.iter_mut() {
                let blank = " ".repeat(5 * scale + scale);
                row.push_str(&blank);
            }
            continue;
        }

        let glyph: &[u8; 7] = &FONT_5X7[code - 32];

        for (row_idx, row_str) in rows.iter_mut().enumerate() {
            let bits: u8 = glyph[row_idx];
            for col in (0..5).rev() {
                let pixel = if (bits >> col) & 1 == 1 { block } else { ' ' };
                for _ in 0..scale {
                    row_str.push(pixel);
                }
            }
            for _ in 0..scale {
                row_str.push(' ');
            }
        }
    } 

    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_has_seven_rows() {
        let rows = render_banner("Hello", '█', 1);
        assert_eq!(rows.len(), 7);
    }

    #[test]
    fn banner_has_same_width() {
        let rows = render_banner("Hello", '█', 1);
        let width: Vec<usize> = rows.iter().map(|s| s.len()).collect();
        // ✅ Bug 3 fixed: assert! not assert_eq!
        assert!(width.windows(2).all(|w| w[0] == w[1]),
            "Row widths differ: {:?}", width);
    }

    #[test]
    fn scale_doubles_width() {
        let r1 = render_banner("HI", '█', 1);
        let r2 = render_banner("HI", '█', 2);
        assert_eq!(r2[0].len(), r1[0].len() * 2);
    }
}