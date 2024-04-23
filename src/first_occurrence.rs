use std::io;

pub fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

pub fn main() {
    println!("Enter sorted integers separated by spaces:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let arr: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Enter the target number:");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    
    let target: i32 = input.trim().parse().expect("Please enter a valid integer");

    if let Some(index) = first_occurrence(&arr, target) {
        println!("First occurrence of {} is at index {}", target, index);
    } else {
        println!("{} not found in array", target);
    }
}
