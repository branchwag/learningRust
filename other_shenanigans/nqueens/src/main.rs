use std::env;

/// Constructs a valid N-Queens placement directly, with no search at all.
/// Based on the closed-form construction from Hoffman, Loessi & Moore
/// (1969): split 1..=n into evens and odds, concatenate them (evens first),
/// and column i gets a queen in the row given by the i-th number in that
/// list. When n mod 6 is 2 or 3 a small fixed rearrangement is needed to
/// avoid diagonal clashes near the start/end of the list. Runs in O(n) time
/// regardless of n, so n = 100 (or n = 100,000) is instant.
/// Returns the placement as a vector of column indices, one per row, or
/// None for n = 2 or n = 3, the only sizes with no valid arrangement.
fn solve(n: u32) -> Option<Vec<u32>> {
    if n == 0 {
        return Some(Vec::new());
    }
    if n == 2 || n == 3 {
        return None;
    }

    let mut evens: Vec<u32> = (1..=n / 2).map(|i| 2 * i).collect();
    let mut odds: Vec<u32> = (0..n.div_ceil(2)).map(|i| 2 * i + 1).collect();

    match n % 6 {
        2 => {
            // Swap the first two odds (1, 3) and move 5 to the end.
            odds.swap(0, 1);
            if let Some(pos) = odds.iter().position(|&v| v == 5) {
                let five = odds.remove(pos);
                odds.push(five);
            }
        }
        3 => {
            // Move 2 to the end of the evens, and 1, 3 to the end of the odds.
            if let Some(pos) = evens.iter().position(|&v| v == 2) {
                let two = evens.remove(pos);
                evens.push(two);
            }
            for target in [1u32, 3u32] {
                if let Some(pos) = odds.iter().position(|&v| v == target) {
                    let val = odds.remove(pos);
                    odds.push(val);
                }
            }
        }
        _ => {}
    }

    let mut columns_by_row = vec![0u32; n as usize];
    for (col, row) in evens.into_iter().chain(odds).enumerate() {
        columns_by_row[(row - 1) as usize] = col as u32;
    }

    Some(columns_by_row)
}

fn print_board(n: u32, placement: &[u32]) {
    for &col in placement {
        for c in 0..n {
            print!("{}", if c == col { "Q " } else { ". " });
        }
        println!();
    }
}

fn main() {
    let n: u32 = env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    match solve(n) {
        Some(placement) => {
            println!("Found a solution for N = {n}:");
            print_board(n, &placement);
        }
        None => println!("No solution exists for N = {n}."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// True if no two queens in `placement` share a row, column, or diagonal.
    fn is_valid(n: u32, placement: &[u32]) -> bool {
        use std::collections::HashSet;

        if placement.len() != n as usize {
            return false;
        }
        let mut cols = HashSet::with_capacity(placement.len());
        let mut diag1 = HashSet::with_capacity(placement.len());
        let mut diag2 = HashSet::with_capacity(placement.len());
        for (row, &col) in placement.iter().enumerate() {
            let row = row as i64;
            let col = col as i64;
            if !cols.insert(col) || !diag1.insert(row - col) || !diag2.insert(row + col) {
                return false;
            }
        }
        true
    }

    #[test]
    fn n_zero_is_trivially_solved() {
        assert_eq!(solve(0), Some(Vec::new()));
    }

    #[test]
    fn n_one_is_trivially_solved() {
        assert_eq!(solve(1), Some(vec![0]));
    }

    #[test]
    fn n_two_and_three_have_no_solution() {
        assert_eq!(solve(2), None);
        assert_eq!(solve(3), None);
    }

    #[test]
    fn every_size_up_to_500_is_valid_or_correctly_unsolvable() {
        for n in 1..=500u32 {
            match solve(n) {
                Some(placement) => assert!(
                    is_valid(n, &placement),
                    "solve({n}) returned an invalid placement: {placement:?}"
                ),
                None => assert!(
                    n == 2 || n == 3,
                    "solve({n}) unexpectedly found no solution"
                ),
            }
        }
    }

    #[test]
    fn matches_known_sequences_from_the_source_construction() {
        // Column i (0-indexed) holds a queen in row `sequence[i] - 1`.
        // These are the worked examples from the Hoffman/Loessi/Moore
        // construction (n mod 6 == 2, == 3, and the default case).
        let cases: &[(u32, &[u32])] = &[
            (14, &[2, 4, 6, 8, 10, 12, 14, 3, 1, 7, 9, 11, 13, 5]),
            (15, &[4, 6, 8, 10, 12, 14, 2, 5, 7, 9, 11, 13, 15, 1, 3]),
            (20, &[
                2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 3, 1, 7, 9, 11, 13, 15, 17, 19, 5,
            ]),
        ];

        for &(n, sequence) in cases {
            let mut expected = vec![0u32; n as usize];
            for (col, &row) in sequence.iter().enumerate() {
                expected[(row - 1) as usize] = col as u32;
            }
            assert_eq!(solve(n), Some(expected));
        }
    }

    #[test]
    fn large_n_is_still_instant_and_valid() {
        let n = 100_000;
        let placement = solve(n).expect("100,000 queens should have a solution");
        assert!(is_valid(n, &placement));
    }
}
