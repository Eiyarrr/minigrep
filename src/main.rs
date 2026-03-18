use std::fs::File;
use std::io;
use std::io::Error;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Error> {
    let path = get_path();
    let query = get_query();
    println!("Searching...");

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let parsed_file = read_file(reader, &query)?;
    let lines_containing = parsed_file.0;
    let matches = parsed_file.1;
    println!("Found {matches} matches...");
    for line in lines_containing {
        println!("{}: {}", line.0, line.1);
    }

    Ok(())
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

// Add input validation
fn get_path() -> String {
    println!("Enter your file path: ");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");
    let path = path.trim().to_string();
    path
}

// Add input validation
fn get_query() -> String {
    println!("Enter your query: ");
    let mut query = String::new();
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read input");
    let query = query.trim();
    query.to_string()
}
