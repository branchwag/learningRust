use std::collections::HashMap;

fn modes(numbers: &[i32]) -> Vec<i32> {
    let mut counts = HashMap::new();

    for &num in numbers {
        *counts.entry(num).or_insert(0) += 1;
    }

    let max_count = counts.values().copied().max().unwrap_or(0);

    counts
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(num, _)| num)
        .collect()
}

fn main() {
    let nums = vec![1, 2, 2, 2, 4, 2, 5, 3, 3];
    println!("{:?}", modes(&nums));
}
//handle ties better
