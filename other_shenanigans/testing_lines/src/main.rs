fn main() {
    let text = "apple\nbananas\ncherry";

    for line in text.lines() {
        println!("{}", line);
    }
}
