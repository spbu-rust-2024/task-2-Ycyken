mod manacher;

use std::io;
use manacher::longest_palindromic_substring;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let longest_palindrome = longest_palindromic_substring(&input);
    println!("{}", longest_palindrome);
}


