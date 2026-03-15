use std::io;

fn main() {
    let path = get_path();
    let query = get_query();

    println!("path: {path}\nquery: {query}");
}

// Add input validation
fn get_path() -> String {
    println!("Enter your file path: ");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read input");
    let path = path.trim();
    path.to_string()
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
