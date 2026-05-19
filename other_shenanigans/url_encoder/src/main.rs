fn encode(input: &str) -> String {
    let mut out = String::new();

    for b in input.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }

            b' ' => out.push_str("%20"),

            _ => out.push_str(&format!("%{:02X}", b)),
        }
    }

    out
}

fn param(key: &str, value: &str) -> String {
    let mut s = String::new();

    s.push_str(&encode(key));
    s.push('=');
    s.push_str(&encode(value));

    s
}

fn main() {
    let p = param("search", "hello world!");
    println!("{}", p);
}
