// logreview-rs - Analyze web server logs.
// Copyright (C) 2026 Michael McMahon
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use chrono::Local;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use toml;

#[derive(Deserialize)]
struct LogConfig {
  filenames: Vec<String>,
  number_of_frequencies: usize,
  column_index: usize,
}

// Temporariliy hardcoded config file location.
const CONFIG_FILE_PATH: &str = "config/logreview.toml";

fn load_config(filename: &str) -> Result<LogConfig, Box<dyn std::error::Error>> {
  let path = Path::new(filename);
  let contents = fs::read_to_string(path)?;
  let config: LogConfig = toml::from_str(&contents)?;
  Ok(config)
}

fn top_n_frequent_entries<P>(filename: P, q: usize, c: usize) -> io::Result<Vec<(String, usize)>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  let reader = io::BufReader::new(file);

  let mut frequency_map = HashMap::new();

  for line in reader.lines() {
    let line = line?;
    // Split the line by whitespace and collect the first column
    if let Some(first_column) = line.split_whitespace().next() {
      *frequency_map.entry(first_column.to_string()).or_insert(c) += 1;
    }
  }

  // Collecting results and sorting them based on frequency
  let mut entries: Vec<(String, usize)> = frequency_map.into_iter().collect();
  entries.sort_by(|a, b| b.1.cmp(&a.1)); // Sort in descending order by frequency

  // Return only the top N entries
  Ok(entries.into_iter().take(q).collect())
}

fn timeisthemaster() {
  let date = Local::now();
  println!(
    "\nCurrent date/time: {}",
    date.format("%Y-%m-%d %H:%M:%S")
  );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
  timeisthemaster();

  // Load configuration from file
  let config = load_config(CONFIG_FILE_PATH)?;

  println!("\nConfig file loaded successfully!");

  // Move the variables out of config so they can be copied/cloned as needed.
  let f = config.filenames;
  let q = config.number_of_frequencies;
  let c = config.column_index;

  // Print config.
  println!("Filenames: {:?}", f);
  println!("Number of frequencies: {}", q);
  println!("Column index: {}", c);

  // Temporarily manually set the log to scan.
  //let filename = "log/test1.log";
  // Parse IPs.
  //let top_entries = top_n_frequent_entries(filename, q, c)?;

  for file in f {
    // Parse IPs.
    let e = file.clone();
    let top_entries = top_n_frequent_entries(file, q, c)?;

    let d = c + 1;
    println!("\nTop {} most frequent IP addresses in column {} of {}:", q, d, e);
    for (entry, count) in top_entries {
      println!("{}: {}", count, entry);
    }
  }

  timeisthemaster();

  Ok(())
}
