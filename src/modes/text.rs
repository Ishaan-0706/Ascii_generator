// text to ascii art

// ALGORITHM
//  ─────────
//  1. For every char in the input string, look up its 5×7 glyph
//     from font::FONT_5X7.

//  2. Build 7 output row-strings in parallel.
//     For each pixel row, scan every glyph and emit the block
//     char (lit pixel) or a space (dark pixel) based on the bitmask.

//  3. Apply optional horizontal scale (repeat each pixel N times).

//  4. Wrap every row in ANSI colour codes and print.

// TextArgs to run() to render_banner() to vec<String>
// to utlis::colorize() to println

use anyhow::Result;
use crate::cli::TextArgs;
use crate::font::FONT_5X7;
