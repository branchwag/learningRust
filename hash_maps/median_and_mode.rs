use std::collections::HashMap;
use std::io;

fn median_and_mode(nums: &[i32]) -> Option<(i32, i32)> {
    if nums.is_empty() {
        return None;
    }

    //median
    let mut sorted = nums.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;
    let median = sorted[mid];

    //mode
    let mut counts: HashMap<i32, usize> = HashMap::new();
    for &n in nums {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mode = counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap();

    Some((median, mode))
}

fn main() {
    println!("Enter a list of integers separated by spaces:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    //let numbers = vec![1, 2, 2, 3, 4];
    let numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<_, _>>()
        .expect("Please enter valid integers");

    match median_and_mode(&numbers) {
        Some((median, mode)) => {
            println!("Median: {}", median);
            println!("Mode: {}", mode);
        }
        None => {
            println!("No numbers provided");
        }
    }
}
