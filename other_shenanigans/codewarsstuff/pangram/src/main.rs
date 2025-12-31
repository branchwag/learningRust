fn is_pangram(s: &str) -> bool {
    //check string for each letter of the alphabet
    //ignore casing numbers and puntuation
    let letters: std::collections::HashSet<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect();

    letters.len() == 26
}

fn main() {
    println!("Is it a pangram?");
    let result = is_pangram("The quick brown fox jumps over the lazy dog");
    println!("{}", result);
}
