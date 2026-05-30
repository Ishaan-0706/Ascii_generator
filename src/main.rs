mod cli;
mod font;
mod modes;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    if let Err(e) = run() {
        utils::print_error(&e.to_string());
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Text(args) => modes::text::run(args),
        Commands::Image(args) => modes::image::run(args),
    }
}