use std::env;

/// Backtracking N-Queens solver using bitmasks for columns and diagonals.
/// Returns the total solution count and the first solution found (as a
/// vector of column indices, one per row).
fn solve(n: u32) -> (u64, Option<Vec<u32>>) {
    let mut count = 0u64;
    let mut first: Option<Vec<u32>> = None;
    let mut placement = Vec::with_capacity(n as usize);
    let full: u32 = if n == 32 { u32::MAX } else { (1 << n) - 1 };

    fn backtrack(
        n: u32,
        full: u32,
        cols: u32,
        diag1: u32,
        diag2: u32,
        placement: &mut Vec<u32>,
        count: &mut u64,
        first: &mut Option<Vec<u32>>,
    ) {
        if cols == full {
            *count += 1;
            if first.is_none() {
                *first = Some(placement.clone());
            }
            return;
        }
        let mut available = full & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & available.wrapping_neg();
            available &= available - 1;
            placement.push(bit.trailing_zeros());
            backtrack(
                n,
                full,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                placement,
                count,
                first,
            );
            placement.pop();
        }
    }

    backtrack(n, full, 0, 0, 0, &mut placement, &mut count, &mut first);
    (count, first)
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

    // Columns/diagonals are tracked as bits in a u32, so 32 is the most
    // columns we can represent without overflowing the mask. Not that it
    // matters in practice: this brute-force backtracking search is already
    // hopelessly slow well before n reaches 32.
    if n == 0 || n > 32 {
        eprintln!("Please choose n between 1 and 32.");
        std::process::exit(1);
    }

    let (count, first) = solve(n);
    println!("N = {n}: {count} solution(s) found.");
    if let Some(placement) = first {
        println!("\nFirst solution:");
        print_board(n, &placement);
    }
}
