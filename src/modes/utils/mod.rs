use colored::Colorize;
use anyhow::Error;
// wraps a string in the ansi color codes for the terminal output
//# arguments
// * `s` the string to colour
// * `color` case-insensitive colour name



pub fn colorize(s: &str , color: &str) -> String{
    match color.to_lowercase().as_str(){
        "red" => s.red().bold().to_string(),
    }
}