use crate::io;
use std::fs::File;

pub fn get_file() -> File {
    clearscreen::clear().expect("Failed to clear console");
    loop {
        println!("Enter your file path: ");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read input");
        let path = path.trim().to_string();
        match File::open(&path) {
            Ok(file) => {
                return file;
            }
            Err(_) => {
                println!("Could not find file at {path}");
                continue;
            }
        };
    }
}
