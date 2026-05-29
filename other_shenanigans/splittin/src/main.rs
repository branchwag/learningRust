fn main() {
    let text = "apple,banana,cherry";
    let parts: Vec<&str> = text.split(',').collect();
    println!("{:?}", parts);

    let text2 = "This will show splitting on whitespace";
    let words: Vec<&str> = text.split_whitespace().collect();
}
