use colored::Colorize;

pub fn colorize(text: &str, color: &str) -> String {
    match color.to_lowercase().as_str() {
        "red"     => text.red().to_string(),
        "green"   => text.green().to_string(),
        "blue"    => text.blue().to_string(),
        "yellow"  => text.yellow().to_string(),
        "cyan"    => text.cyan().to_string(),
        "magenta" => text.magenta().to_string(),
        _         => text.white().to_string(),
    }
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}