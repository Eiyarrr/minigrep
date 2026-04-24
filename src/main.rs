mod file_manager;
mod user_input;

use file_manager::read_file::read_file;

use user_input::get_file::get_path;
use user_input::get_query::get_query;

use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::{Error, ErrorKind};

fn main() -> Result<(), Error> {
    clearscreen::clear().expect("Failed to clear console");
    let file = get_path();
    let query = get_query();

    let f: Vec<File>;
    match file {
        Ok(a) => f = a,
        Err(e) => {
            return Err(Error::new(ErrorKind::Other, e));
        }
    };

    println!("Searching...");
    for a in f.iter() {
        let reader = BufReader::new(a);
        let parsed_file = read_file(reader, &query)?;
        print_lines(parsed_file);
    }

    Ok(())
}

fn print_lines(parsed_file: (Vec<(i32, String)>, i32)) {
    let lines_containing = parsed_file.0;
    let matches = parsed_file.1;
    for line in lines_containing {
        println!("{}: {}", line.0, line.1);
    }
    println!("Found {matches} matches...");
}
