fn main() {
    let x: Option<i32> = Some(2);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };

    println!("{y:?}");
}
