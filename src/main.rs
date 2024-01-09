// Ignore warnings about unused code
#![allow(unused)]

// Import necessary modules for character conversion and random number generation
use std::char::from_u32;
use rand::{Rng, thread_rng};

fn main() {
    // Define the desired length of the random password
    let password_length = 15;

    // Initialize an empty string to store the random password
    let mut result = String::new();

    // Generate random characters and append them to the result string
    for _ in 0..password_length {
        // Generate a random ASCII value within the range of printable characters (48 to 122)
        let number = thread_rng().gen_range(48..=122);

        // Convert the ASCII value to a character and append it to the result string
        let ch = from_u32(number).unwrap();
        result.push(ch);
    }

    // Print the generated random password
    println!("Random Password: {}", result);
}
