use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn path_finder(area: &[Vec<u32>]) -> u32 {
    let n = area.len();
    if n == 0 {
        return 0;
    }

    let mut dist = vec![vec![u32::MAX; n]; n];
    let mut heap = BinaryHeap::new();

    dist[0][0] = 0;
    heap.push((Reverse(0u32), 0usize, 0usize));

    let directions = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];

    while let Some((Reverse(cost), x, y)) = heap.pop() {
        if x == n - 1 && y == n - 1 {
            return cost;
        }

        if cost > dist[x][y] {
            continue;
        }

        for &(dx, dy) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < n as isize && ny < n as isize {
                let nx = nx as usize;
                let ny = ny as usize;

                let move_cost = area[x][y].abs_diff(area[nx][ny]);
                let next_cost = cost + move_cost;

                if next_cost < dist[nx][ny] {
                    dist[nx][ny] = next_cost;
                    heap.push((Reverse(next_cost), nx, ny));
                }
            }
        }
    }

    0
}

fn main() {
    // Example grid
    let area = vec![vec![0, 1, 2], vec![1, 2, 3], vec![2, 3, 4]];

    let result = path_finder(&area);

    println!("Minimum climb rounds: {}", result);
}
