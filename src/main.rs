mod user_input;

use std::fs::File;
use std::io;
use std::io::Error;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    clearscreen::clear().expect("Failed to clear console");
    let file = get_file();
    let query = get_query();

    println!("Searching...");
    let reader = BufReader::new(file);
    let parsed_file = read_file(reader, &query)?;
    print_lines(parsed_file);

    Ok(())
}

fn print_lines(parsed_file: (Vec<(i32, String)>, i32)) {
    let lines_containing = parsed_file.0;
    let matches = parsed_file.1;
    println!("Found {matches} matches...");
    for line in lines_containing {
        println!("{}: {}", line.0, line.1);
    }
}

fn read_file(reader: BufReader<File>, query: &str) -> Result<(Vec<(i32, String)>, i32), Error> {
    let mut line_number = 0;
    let mut matches = 0;
    let mut lines_containing: Vec<(i32, String)> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.contains(&query) {
            lines_containing.push((line_number, line.to_string()));
            matches += 1;
        }
        line_number += 1;
    }
    Ok((lines_containing, matches))
}
