use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize)]
struct LogConfig {
    filenames: Vec<String>,
    number_of_frequencies: u32,
    column_index: u32,
}

const CONFIG_FILE_PATH: &str = "config/logreview.toml";

fn load_config(filename: &str) -> Result<LogConfig, Box<dyn std::error::Error>> {
    let path = Path::new(filename);
    let contents = fs::read_to_string(path)?;
    let config: LogConfig = toml::from_str(&contents)?;
    Ok(config)
}

fn display_results(config: LogConfig) {
    println!("Filenames: {:?}", config.filenames);
    println!("Number of frequencies: {}", config.number_of_frequencies);
    println!("Column index: {}", config.column_index);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from file
    let config = load_config(CONFIG_FILE_PATH)?;

    // Display results
    display_results(config);

    Ok(())
}
