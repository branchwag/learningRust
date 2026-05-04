// given 9 x 9 model
// loop through the 2D puzzle array. If you encounter a 0, check:
// ensure 1-9 across horizontallly. and if you pick a number, ensure that numebr is not in that
// array horizonallt
// check each column number and ensure we have 1-9 there and that the number is not in the column
// alreayd
// check 3x3 to ensure 1-9 there and that the number we are at is not already there
// hashmap to store values to ensure no dupes, keep looking them up there
// once no 0s remain, puzzle is solved
//
// good, but going with bitmask solution instead
//
fn sudoku(puzzle: &mut [[u8; 9]; 9]) {
    let mut rows = [0u16; 9];
    let mut cols = [0u16; 9];
    let mut boxes = [0u16; 9];

    // Initialize bitmasks
    for r in 0..9 {
        for c in 0..9 {
            let n = puzzle[r][c];
            if n != 0 {
                let bit = 1u16 << n;
                rows[r] |= bit;
                cols[c] |= bit;
                boxes[(r / 3) * 3 + (c / 3)] |= bit;
            }
        }
    }

    // Stack for backtracking: (row, col, remaining candidates)
    let mut stack: Vec<(usize, usize, u16)> = Vec::new();

    loop {
        let mut best: Option<(usize, usize, u16)> = None;

        // Find most constrained empty cell
        for r in 0..9 {
            for c in 0..9 {
                if puzzle[r][c] == 0 {
                    let b = (r / 3) * 3 + (c / 3);
                    let used = rows[r] | cols[c] | boxes[b];
                    let candidates = (!used) & 0b11_1111_1110;

                    if candidates == 0 {
                        best = None;
                        break;
                    }

                    if best.is_none_or(|(_, _, mask)| candidates.count_ones() < mask.count_ones()) {
                        best = Some((r, c, candidates));
                    }
                }
            }
        }

        // If no empty cells → solved
        if best.is_none() {
            return;
        }

        let (r, c, candidates) = best.unwrap();

        // Try next candidate
        if candidates != 0 {
            let bit = candidates & (!candidates + 1);
            let n = bit.trailing_zeros() as u8;
            let b = (r / 3) * 3 + (c / 3);

            // Save remaining candidates
            stack.push((r, c, candidates & (candidates - 1)));

            // Place number
            puzzle[r][c] = n;
            rows[r] |= bit;
            cols[c] |= bit;
            boxes[b] |= bit;
        } else {
            // Backtrack
            while let Some((pr, pc, pcands)) = stack.pop() {
                let b = (pr / 3) * 3 + (pc / 3);
                let prev = puzzle[pr][pc];
                let prev_bit = 1u16 << prev;

                // Undo previous placement
                puzzle[pr][pc] = 0;
                rows[pr] &= !prev_bit;
                cols[pc] &= !prev_bit;
                boxes[b] &= !prev_bit;

                if pcands != 0 {
                    let bit = pcands & (!pcands + 1);
                    let n = bit.trailing_zeros() as u8;

                    // Save updated remaining candidates
                    stack.push((pr, pc, pcands & (pcands - 1)));

                    // Place next candidate
                    puzzle[pr][pc] = n;
                    rows[pr] |= bit;
                    cols[pc] |= bit;
                    boxes[b] |= bit;

                    break;
                }
            }
        }
    }
}

fn main() {
    let mut puzzle = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    println!("Before:");
    print_board(&puzzle);

    sudoku(&mut puzzle);

    println!("\nAfter:");
    print_board(&puzzle);
}

fn print_board(board: &[[u8; 9]; 9]) {
    for row in board {
        for &val in row {
            print!("{} ", val);
        }
        println!();
    }
}
