use std::error::Error;
use std::fs;

pub mod dictcc;

struct Entry {
    headword: String,
    definition: String,
}

pub struct Config {
    pub file_path: String, // Path to desired file
    pub source_lang: String,
    pub target_lang: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        match args.len() {
            1..=3 => return Err("not enough arguments"),
            4 => {
                let file_path = args[1].clone(); // Get file path from arguments
                let source_lang = args[2].clone();
                let target_lang = args[3].clone();
                Ok(Config { file_path, source_lang, target_lang }) // Return new Config object with given file path
            }
            _ => return Err("too many arguments"),
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read contents of file at the path in config then print
    let binding = fs::read_to_string(config.file_path)?;
    let contents = binding.lines();
    for term in contents {
        let result = dictcc::search(config.source_lang.clone(), config.target_lang.clone(), term.to_string())?;
        println!("{:#?}", result);
    }
    Ok(())
}