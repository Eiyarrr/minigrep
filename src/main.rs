mod file_manager;
mod user_input;

use file_manager::read_file::read_file;

use user_input::get_file::get_file;
use user_input::get_query::get_query;

use std::io;
use std::io::Error;
use std::io::BufReader;

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
    for line in lines_containing {
        println!("{}: {}", line.0, line.1);
    }
    println!("Found {matches} matches...");
}
