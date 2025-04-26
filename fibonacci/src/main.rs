use std::io;

fn main() {
    println!("Input number:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return;
        }
    };

    println!("The {}th Fibonacci number is: {}", n, fibonacci(n));

}

fn fibonacci(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }

    let mut a = 0;
    let mut b = 1;

    for _ in 2..=n { //goes from 2 to n
        let temp = a + b;
        a = b;
        b = temp;
    }

    b //after loop ends, bholds the last fib number
}
