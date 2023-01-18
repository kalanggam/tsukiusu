use std::error::Error;
use std::fs;

pub mod dictcc;

pub struct Config {
    pub file_path: String, // Path to desired file
}

struct Entry {
    headword: String,
    definition: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments"); // If there isn't a path argument, return an error
        }
        let file_path = args[1].clone(); // Get file path from arguments
        Ok(Config { file_path }) // Return new Config object with given file path
    }
}

fn lookup(word: String) -> Result<String, &'static str> {
    let definition: String = String::from(";");
    Ok(definition)
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents of file at the path in config then print
    let contents = fs::read_to_string(config.file_path)?;
    println!("Contents: \n{contents}");
    Ok(())
}