use std::io;

pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

pub fn main() {
    println!("Enter a string to reverse:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    println!("Reversed string: {}", reverse_string(input));
}

