use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <argument>", args[0]);
        return;
    }

    println!("{}e", args[1]);
}
