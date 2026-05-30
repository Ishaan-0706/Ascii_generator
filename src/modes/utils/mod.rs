use colored::Colorize;
use anyhow::Error;
// wraps a string in the ansi color codes for the terminal output
//# arguments
// * `s` the string to colour
// * `color` case-insensitive colour name



pub fn colorize(s: &str , color: &str) -> String{
    match color.to_lowercase().as_str(){
        "red" => s.red().bold().to_string(),
                "blue" => s.blue().bold().to_string(),
                "white" => s.white().bold().to_string(),
        "green" => s.green().bold().to_string(),
        "cyan" => s.cyan().bold().to_string(),
        "yellow" => s.yellow().bold().to_string(),

        _ => s.white().bold().to_string(),
    }
}

pub fn print_error(e: &Error){
    eprintln!("{}" "{}", "Error: ".red().bold() , e);

    for cause in e.chain().skip(1){
        eprintln!("{}" "{}", "Caused by: ".red().bold() , cause);
    }
}