pub fn encode(s: &str) -> (String, usize) {
    if s.is_empty() {
        return (String::new(), 0);
    }

    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();

    let mut rotations: Vec<(Vec<char>, usize)> = (0..len)
        .map(|i| {
            let rotation = (0..len).map(|j| chars[(i + j) % len]).collect::<Vec<_>>();
            (rotation, i)
        })
        .collect();

    rotations.sort_by(|a, b| a.0.cmp(&b.0));

    let last_column: String = rotations.iter().map(|(row, _)| row[len - 1]).collect();

    let original_index = rotations.iter().position(|(_, start)| *start == 0).unwrap();

    (last_column, original_index)
}

pub fn decode(s: &str, n: usize) -> String {
    if s.is_empty() {
        return String::new();
    }

    use std::collections::HashMap;

    let last: Vec<char> = s.chars().collect();
    let len = last.len();

    let mut first = last.clone();
    first.sort_unstable();

    let mut counts = HashMap::new();
    let last_rank: Vec<(char, usize)> = last
        .iter()
        .map(|&c| {
            let rank = *counts.get(&c).unwrap_or(&0);
            counts.insert(c, rank + 1);
            (c, rank)
        })
        .collect();

    counts.clear();
    let mut first_pos = HashMap::new();
    for (i, &c) in first.iter().enumerate() {
        let rank = *counts.get(&c).unwrap_or(&0);
        first_pos.insert((c, rank), i);
        counts.insert(c, rank + 1);
    }

    let next: Vec<usize> = last_rank
        .iter()
        .map(|&(c, rank)| first_pos[&(c, rank)])
        .collect();

    let mut result = String::with_capacity(len);
    let mut row = n;

    for _ in 0..len {
        result.push(first[row]);
        row = next[row];
    }

    result
}

fn main() {
    let input = "bananabar";

    // Encode
    let (encoded, index) = encode(input);
    println!("Original: {}", input);
    println!("Encoded:  {}", encoded);
    println!("Index:    {}", index);

    // Decode
    let decoded = decode(&encoded, index);
    println!("Decoded:  {}", decoded);

    // Quick check
    assert_eq!(decoded, input);
}
