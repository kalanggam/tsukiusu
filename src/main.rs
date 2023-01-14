use std::env;
use std::process;

use tsukiusu::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // Retrieve & collect command line arguments
    // Parse command line arguments and store file path - print error and exit if failed
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // Print a welcome message
    println!("Reading vocabulary from file {}...", config.file_path);

    // Run with the given configuration, returning errors if any
    if let Err(e) = tsukiusu::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}