//caesar cipher
//  shifts each letter in the alphsbet by a fixed number
//  shift of 3
//  A B C D
//  B -> E
//  X -> A
//  encrypt and decrypt

use std::io::{self};
//str - collection of UTF-8 bytes
fn caesar(text: &str, shift: i8) -> String {
    text.chars() //turns string into iterator of characters
        .map(|c| {
            //for every character, run this:
            let base = match c {
                'a'..='z' => b'a', // byte literal a = 97, b = 98, c = 99 ...
                'A'..='Z' => b'A', // A = 65, B = 66 ...
                _ => return c,     //punctuation, spaces, numbers remain unchanged
            };

            let pos = c as i8 - base as i8;
            let shifted = (pos + shift).rem_euclid(26); //wrapping mod arithmetic for neg numbers

            (base + shifted as u8) as char //converts # back into char
        })
        .collect() //gathers into String
}

fn main() {
    println!("=== Caesar Cipher ===");

    println!("Choose mode:");
    println!("1) Encrypt");
    println!("2) Decrypt");

    let mut mode = String::new();
    io::stdin()
        .read_line(&mut mode)
        .expect("Failed to read input");

    println!("Enter shift amount:");
    let mut shift_input = String::new();
    io::stdin()
        .read_line(&mut shift_input)
        .expect("Failed to read input");

    let shift: i8 = shift_input
        .trim() //remove whitespace/newlines
        .parse() //converts string to int //returns Result
        .expect("Please enter a valid number"); //crashes program and prints this msg

    println!("Enter message:");

    let mut message = String::new();
    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read input");

    let message = message.trim();

    let result = match mode.trim() {
        "1" => caesar(message, shift),
        "2" => caesar(message, -shift),
        _ => {
            println!("Invalid option");
            return;
        }
    };

    println!("\nResult:");
    println!("{}", result);
}
