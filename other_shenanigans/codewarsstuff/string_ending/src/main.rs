use std::io;

fn solution(word: &str, ending: &str) -> bool {
    word.ends_with(ending)
}

fn main() {
    
    println!("Enter a string to check:");
    let mut string_tocheck = String::new();

    io::stdin()
        .read_line(&mut string_tocheck)
        .expect("Failed to read line");

    println!("Enter the ending you are looking for:");
    let mut ending = String::new();

    io::stdin()
        .read_line(&mut ending)
        .expect("Failed to read ending");

    let string_tocheck = string_tocheck.trim();
    let ending = ending.trim();

    let result = solution(string_tocheck, ending);

    if result {
        println!("'{}' ends with '{}'", string_tocheck, ending);
    } else {
        println!("'{}' does not end with '{}'", string_tocheck, ending);
    }

}


