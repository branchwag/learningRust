use std::collections::HashMap;

fn mode(numbers: &[i32]) -> Option<i32> {
    let mut counts = HashMap::new();

    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(num, _)| num)
}

fn main() {
    let nums = vec![1, 2, 2, 2, 4, 2, 5, 3, 3];
    match mode(&nums) {
        Some(m) => println!("Mode: {}", m),
        None => println!("No mode found"),
    }
}
