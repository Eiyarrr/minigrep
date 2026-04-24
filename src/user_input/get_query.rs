use crate::io;

pub fn get_query() -> String {
    clearscreen::clear().expect("Failed to clear console");
    println!("Enter your query: ");
    let mut query = String::new();
    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read input");
    let query = query.trim();
    query.to_string()
}
