// main.rs

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments are provided
    if args.len() < 3 {
        eprintln!("Usage: {} pattern filename", args[0]);
        std::process::exit(1);
    }

    // Extract pattern and filename from arguments
    let pattern = &args[1];
    let filename = &args[2];

    // Open the file
    let file = match File::open(Path::new(filename)) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    // Create a buffered reader
    let reader = BufReader::new(file);

    // Read lines and search for the pattern
    for (line_number, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue;
            }
        };

        if line.contains(pattern) {
            println!("{}: {}", line_number + 1, line);
        }
    }
}
```

This Rust code implements a basic `grep` clone that searches for a pattern in a given file and prints the lines containing the pattern along with their line numbers. The code includes error handling for both opening the file and reading lines.