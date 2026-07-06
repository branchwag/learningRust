use std::env;

/// Backtracking N-Queens solver using bitmasks for columns and diagonals.
/// Stops as soon as it finds one valid placement and returns it (as a
/// vector of column indices, one per row), or None if no solution exists.
fn solve(n: u32) -> Option<Vec<u32>> {
    let mut placement = Vec::with_capacity(n as usize);
    let full: u32 = if n == 32 { u32::MAX } else { (1 << n) - 1 };

    fn backtrack(
        n: u32,
        full: u32,
        cols: u32,
        diag1: u32,
        diag2: u32,
        placement: &mut Vec<u32>,
    ) -> bool {
        if cols == full {
            return true;
        }
        let mut available = full & !(cols | diag1 | diag2);
        while available != 0 {
            let bit = available & available.wrapping_neg();
            available &= available - 1;
            placement.push(bit.trailing_zeros());
            if backtrack(
                n,
                full,
                cols | bit,
                (diag1 | bit) << 1,
                (diag2 | bit) >> 1,
                placement,
            ) {
                return true;
            }
            placement.pop();
        }
        false
    }

    if backtrack(n, full, 0, 0, 0, &mut placement) {
        Some(placement)
    } else {
        None
    }
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
