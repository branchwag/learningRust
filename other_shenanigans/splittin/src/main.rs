fn main() {
    let text = "apple,banana,cherry";
    let parts: Vec<&str> = text.split(',').collect();
    println!("{:?}", parts);
}
