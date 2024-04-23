use std::io;

pub fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    let mut sorted = arr.to_vec();
    sorted.sort();
    Some(sorted[k - 1])
}

pub fn main() {
    println!("Enter integers separated by spaces:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Enter the value of k:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let k: usize = input.trim().parse().expect("Please enter a valid integer");

    if let Some(val) = kth_smallest(&arr, k) {
        println!("{}-th smallest element is {}", k, val);
    } else {
        println!("Invalid value of k.");
    }
}
