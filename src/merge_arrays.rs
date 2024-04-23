use std::io;

pub fn merge_arrays(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut merged = vec![];
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        merged.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        merged.push(arr2[j]);
        j += 1;
    }

    merged
}

pub fn main() {
    println!("Enter sorted integers for array 1 separated by spaces:");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1)
        .expect("Failed to read line");

    let arr1: Vec<i32> = input1
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    println!("Enter sorted integers for array 2 separated by spaces:");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2)
        .expect("Failed to read line");

    let arr2: Vec<i32> = input2
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers"))
        .collect();

    let merged = merge_arrays(&arr1, &arr2);
    println!("Merged array: {:?}", merged);
}
