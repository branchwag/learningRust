use std::env;

fn double_if_even(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n * 2) } else { None }
}

fn main() {
    let result: i32 = env::args()
        .nth(1) //returns an Option<String>
        //enforce that it is a number
        .and_then(|s| s.parse::<i32>().ok())
        .and_then(double_if_even)
        .unwrap_or(0);

    println!("{}", result);
}
