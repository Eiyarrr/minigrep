use crate::io;
use std::fs::File;

pub fn get_path() -> Result<Vec<File>, String> {
    clearscreen::clear().expect("Failed to clear console");
    loop {
        println!("Enter path: ");
        let mut path = String::new();
        io::stdin()
            .read_line(&mut path)
            .expect("Failed to read input");
        let path = path.trim().to_string();
        if path[path.len()..path.len()] == *"/" {
            // get_dir(&path);
        }
        else {
            let x = get_file(&path);
            match x {
                Ok(file) => return Ok(vec![file]),
                Err(e) => return Err(e),
            };
        }
    }
}

fn get_file(path: &String) -> Result<File, String> {
    match File::open(&path) {
        Ok(file) => {
            return Ok(file);
        }
        Err(_) => {
            return Err(format!("Could not find file at {path}"));
        }
    };
}

// fn get_dir(str: &String) -> Result<Vec<File>, String> {}
