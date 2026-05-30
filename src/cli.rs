use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ascii_gen", about = "ASCII art generator", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Text(TextArgs),
    Image(ImageArgs),
}

#[derive(Parser)]
pub struct TextArgs {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long, default_value = "white")]
    pub color: String,

    #[arg(short, long, default_value = "█")]
    pub block: String,

    #[arg(short, long, default_value_t = 1)]
    pub scale: usize,
}

#[derive(Parser)]
pub struct ImageArgs {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long, default_value_t = 100)]
    pub width: u32,

    #[arg(short, long)]
    pub detailed: bool,

    #[arg(short, long)]
    pub invert: bool,

    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short = 'C', long)]
    pub center: bool,
}