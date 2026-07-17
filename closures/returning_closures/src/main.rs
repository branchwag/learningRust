fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn main() {
    let add_one = returns_closure();
    println!("{}", add_one(2));
}
