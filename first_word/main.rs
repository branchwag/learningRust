fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("The word var before clearing: {}", word);
    println!("The s var before clearing: {}", s);
    s.clear();
    println!("The word var after clearing: {}", word);
    println!("The s var after clearing: {}", s);
}
