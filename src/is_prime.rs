use std::io;

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn main() {
    println!("Enter a number to check if it's prime:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input: u32 = input.trim().parse().expect("Please enter a valid integer");

    if is_prime(input) {
        println!("{} is a prime number!", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}
