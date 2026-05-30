# ascii_gen

A command-line ASCII art generator written in Rust. Converts text strings into large bitmap banners and images into full ASCII art rendered in your terminal.

---

## Project Structure

```
ascii_gen/
├── Cargo.toml
├── .gitignore
└── src/
    ├── main.rs
    ├── cli.rs
    ├── font/
    │   └── mod.rs
    ├── modes/
    │   ├── mod.rs
    │   ├── text.rs
    │   └── image.rs
    └── utils/
        └── mod.rs
```

---

## Dependencies

| Crate      | Purpose                              |
|------------|--------------------------------------|
| `clap`     | CLI argument parsing with derive macros |
| `image`    | Image loading, resizing, pixel access |
| `colored`  | ANSI color codes in terminal output  |
| `anyhow`   | Ergonomic error handling             |
| `termsize` | Detect terminal width for centering  |

---

## Installation

Install Rust if not already installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone and build:

```bash
git clone https://github.com/YOUR_USERNAME/ascii-gen.git
cd ascii-gen
cargo build --release
```

The binary is placed at `./target/release/ascii_gen`.

---

## Usage

```
ascii_gen <COMMAND> [OPTIONS]

Commands:
  text    Render a string as a large ASCII banner
  image   Convert an image file to ASCII art
```

---

## Text Mode

Renders any ASCII string as a large 5x7 bitmap font banner.

```bash
./target/release/ascii_gen text --input "HELLO"
```

### Options

| Flag        | Short | Default | Description                          |
|-------------|-------|---------|--------------------------------------|
| `--input`   | `-i`  | required | Text string to render               |
| `--color`   | `-c`  | white   | Output color                         |
| `--block`   | `-b`  | █       | Character used for lit pixels        |
| `--scale`   | `-s`  | 1       | Scale factor, repeats each pixel 1-4 times |

### Available Colors

`red` `green` `blue` `yellow` `cyan` `magenta` `white`

### Examples

```bash
# Basic banner
./target/release/ascii_gen text --input "RUST"

# Colored output
./target/release/ascii_gen text --input "HELLO" --color cyan

# Scaled up 3x
./target/release/ascii_gen text --input "HI" --scale 3

# Custom block character
./target/release/ascii_gen text --input "IIT" --block "#" --scale 2

# Large colored banner
./target/release/ascii_gen text --input "GOAT" --color yellow --scale 2
```

---

## Image Mode

Converts PNG, JPEG, BMP, or GIF images to ASCII art using a luminance-mapped character ramp.

```bash
./target/release/ascii_gen image --path photo.jpg
```

### Options

| Flag         | Short | Default | Description                                      |
|--------------|-------|---------|--------------------------------------------------|
| `--path`     | `-p`  | required | Path to the image file                         |
| `--width`    | `-w`  | 100     | Output width in characters                       |
| `--detailed` | `-d`  | false   | Use 70-character ramp for finer tonal gradation  |
| `--invert`   | `-i`  | false   | Invert mapping, use on light-background terminals|
| `--center`   | `-C`  | false   | Center the output in the terminal                |
| `--output`   | `-o`  | none    | Save output to a file instead of printing        |

### Character Ramps

| Mode       | Ramp |
|------------|------|
| Simple     | `@%#*+=-:. ` |
| Detailed   | `$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\|()1{}[]?-_+~<>i!lI;:,"^\`'. ` |

Dark pixels map to dense characters. Bright pixels map to sparse characters.

### Examples

```bash
# Basic conversion
./target/release/ascii_gen image --path photo.jpg

# High detail output
./target/release/ascii_gen image --path photo.jpg --width 120 --detailed

# Centered in terminal
./target/release/ascii_gen image --path photo.jpg --width 700 --detailed --center

# Invert for light background terminals
./target/release/ascii_gen image --path photo.jpg --width 120 --detailed --invert

# Save to file
./target/release/ascii_gen image --path photo.jpg --width 120 --detailed --output art.txt

# View saved file without line wrapping
less -S art.txt
```

---

## Getting the Best Output

### Preprocessing with ImageMagick

Install ImageMagick:

```bash
sudo dnf install imagemagick   # Fedora
sudo apt install imagemagick   # Ubuntu/Debian
```

Convert to grayscale with contrast and sharpness boost:

```bash
magick photo.jpeg \
  -colorspace Gray \
  -contrast-stretch 3%x3% \
  -sharpen 0x1 \
  photo_best.png
```

### Fitting the Terminal Exactly

Get your terminal dimensions:

```bash
tput cols && tput lines
```

Resize image to match — height is doubled because ASCII characters are taller than wide:

```bash
magick photo.jpeg -resize 954x330 \
  -colorspace Gray \
  -contrast-stretch 3%x3% \
  -sharpen 0x1 \
  photo_best.png
```

One-liner that auto-detects terminal size, preprocesses, and renders:

```bash
magick photo.jpeg \
  -resize $(tput cols)x$(($(tput lines) * 2)) \
  -colorspace Gray \
  -contrast-stretch 3%x3% \
  -sharpen 0x1 \
  photo_best.png && \
./target/release/ascii_gen image \
  --path photo_best.png \
  --width $(tput cols) \
  --detailed \
  --center
```

### What Makes a Good Source Image

- High contrast between subject and background
- Simple or plain background
- Portrait or close-up shots
- Grayscale or black and white images
- High resolution source files

---

## Running Tests

```bash
# Run all tests
cargo test

# Text module only
cargo test --lib modes::text

# Image module only
cargo test --lib modes::image
```

---

## Built-in Help

```bash
./target/release/ascii_gen --help
./target/release/ascii_gen text --help
./target/release/ascii_gen image --help
```

---

## Pushing to GitHub

```bash
cd ascii_gen
git init
git add .
git commit -m "feat: initial ascii art generator"
git remote add origin https://github.com/YOUR_USERNAME/ascii-gen.git
git branch -M main
git push -u origin main
```

---

