//take string of words separated by spaces
//return first word
fn main() {
    println!("Hello, world!");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
