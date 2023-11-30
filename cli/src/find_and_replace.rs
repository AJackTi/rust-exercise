use std::env;

use text_colorizer::*;

#[allow(dead_code)]
#[derive(Debug)]
struct Arguments {
    pattern: String,
    replace: String,
    input_file: String,
    output_file: String,
}

fn print_help() {
    eprintln!("{} - replace a string with a new string", "Find and Replace".green());
    eprintln!("Usage: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>");
}

pub fn run() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_help();
        eprintln!(
            "{} wrong number of arguments give. Expected 4, got {}",
            "Error".red().bold(),
            args.len()
        );
        std::process::exit(1)
    }
}
