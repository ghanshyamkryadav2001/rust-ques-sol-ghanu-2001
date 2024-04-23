use std::io;

pub fn shortest_word(s: &str) -> Option<&str> {
    s.split_whitespace().min_by_key(|&word| word.len())
}

pub fn main() {
    println!("Enter a string of words:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove newline character

    if let Some(shortest) = shortest_word(input) {
        println!("Shortest word is '{}'", shortest);
    } else {
        println!("No words entered.");
    }
}
