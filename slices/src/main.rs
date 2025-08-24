//take string of words separated by spaces
//return first word
fn main() {
    let s = String::from("hello world!");
    println!("{}", first_word(&s))
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
