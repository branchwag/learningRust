use std::env;

fn main() {
    let home = env::var_os("HOME");

    match home {
        Some(value) => println!("{:?}", value),
        None => println!("HOME is not set"),
    }
}
