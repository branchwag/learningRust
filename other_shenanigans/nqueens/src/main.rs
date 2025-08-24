fn solve_nqueens(n: usize) -> Vec<Vec<String>> {
    let mut solutions = Vec::new();
    let mut board = vec![vec!['.'; n]; n];

    backtrack(&mut board, 0, &mut solutions);
    solutions
}

fn backtrack(board: &mut Vec<Vec<char>>, row: usize, solutions: &mut Vec<Vec<String>>) {
    let n = board.len();
    
    if row == n {
        let solution: Vec<String> = board
            .iter()
            .map(|row| row.iter().collect())
            .collect();
        solutions.push(solution);
        return;
    }

    for col in 0..n {
        if is_safe(board, row, col) {
            board[row][col] = 'Q';
            backtrack(board, row + 1, solutions);
            board[row][col] = '.';
        }
    }
}

fn is_safe(board: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    let n = board.len();
    
    for i in 0..row {
        if board[i][col] == 'Q' {
            return false;
        }
    }

    let mut i = row as i32 - 1;
    let mut j = col as i32 - 1;
    while i >= 0 && j >= 0 {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j -= 1;
    }

    let mut i = row as i32 - 1;
    let mut j = col as i32 + 1;
    while i >= 0 && j < n as i32 {
        if board[i as usize][j as usize] == 'Q' {
            return false;
        }
        i -= 1;
        j += 1;
    }

    true
}

fn print_solutions(solutions: &Vec<Vec<String>>) {
    for (i, solution) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        for row in solution {
            println!("{}", row);
        }
        println!();
    }
}

fn main() {
    let n = 8;
    println!("Solving {}-Queens problem...", n);

    let solutions = solve_nqueens(n);
    println!("Found {} solutions:", solutions.len());
    print_solutions(&solutions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4queens() {
        let solutions = solve_nqueens(4);
        assert_eq!(solutions.len(), 2);
    }
    
    #[test]
    fn test_8queens() {
        let solutions = solve_nqueens(8);
        assert_eq!(solutions.len(), 92);
    }

    #[test]
    fn test_1queen() {
        let solutions = solve_nqueens(1);
        assert_eq!(solutions.len(), 1);
        assert_eq!(solutions[0], vec!["Q"]);
    }
}
