use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

pub fn read_file(reader: BufReader<File>, query: &str) -> Result<(Vec<(i32, String)>, i32), Error> {
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
