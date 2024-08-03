use std::str::SplitWhitespace;

fn main() {
    let binding = "Manchester Bogota Paris Dallas Chicago".to_string();
    let cities: SplitWhitespace = binding.split_whitespace();
    
    for city in cities {
        println!("City {}", city);
    }
}