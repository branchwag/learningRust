fn main() {
    let s = "hello      \n\t";
    let trimmed = s.trim_end();

    println!("{}", trimmed);
}
