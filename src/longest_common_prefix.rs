use std::io;

pub fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let first = &strs[0];
    let mut prefix = String::new();
    'outer: for (i, c) in first.chars().enumerate() {
        for s in &strs[1..] {
            if i >= s.len() || s.chars().nth(i) != Some(c) {
                break 'outer;
            }
        }
        prefix.push(c);
    }
    prefix
}

pub fn main() {
    println!("Enter a list of strings separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let strs: Vec<String> = input
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    println!("Longest common prefix is '{}'", longest_common_prefix(&strs));
}
