There are two method to run the rust file:

here is the first one method,

To run individual Rust files like 'palindrome_check.rs', you should use rustc directly to compile and execute the file, as you did:

rustc palindrome_check.rs

Then, run the generated executable:

./palindrome_check


Second Method

If you want to run all the Rust files together by using 'cargo run', but you would need to create a 'main.rs'
file and call the functions from each of the individual files within the 'main.rs'. 

here is the example, 

mod palindrome_check;
mod first_occurrence;
mod shortest_word;
mod prime_check;
mod median;
mod longest_common_prefix;
mod kth_smallest;
mod max_depth;
mod reverse_string;
mod is_prime;
mod merge_arrays;
mod max_subarray_sum;

fn main() {
    palindrome_check::main();
    first_occurrence::main();
    shortest_word::main();
    prime_check::main();
    median::main();
    longest_common_prefix::main();
    kth_smallest::main();
    max_depth::main();
    reverse_string::main();
    is_prime::main();
    merge_arrays::main();
    max_subarray_sum::main();
}


