use std::error::Error;
use std::fs;

pub struct Config {
    pub file_path: String, // Path to desired file
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments!"); // If there isn't a path argument, return an error
        }
        let file_path = args[1].clone(); // Get file path from arguments
        Ok(Config { file_path }) // Return new Config object with given file path
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents of file at the path in config then print
    let contents = fs::read_to_string(config.file_path)?;
    println!("Contents: \n{contents}");
    Ok(())
}