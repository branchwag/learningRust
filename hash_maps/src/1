fn to_pig_latin(input: &str) -> String {
    input
        .split_whitespace()
        .map(pig_latin_word)
        .collect::<Vec<_>>()
        .join(" ")
}

fn pig_latin_word(word: &str) -> String {
    let mut chars = word.chars();

    let first = match chars.next() {
        Some(c) => c,
        None => return String::new(),
    };

    if is_vowel(first) {
        format!("{}-hay", word)
    } else {
        let rest: String = chars.collect();
        format!("{}-{}ay", rest, first)
    }
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

fn main() {
    let s = "first apple hello 世界";
    println!("{}", to_pig_latin(s));
}
