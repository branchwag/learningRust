use macroquad::prelude::*;
use std::collections::VecDeque;

const TILE: f32 = 28.0;
const COLS: usize = 19;
const ROWS: usize = 19;
const TOP: f32 = 40.0; // height of the score bar above the maze
const SPEED: f32 = 6.0; // tiles per second
const PACMAN_START: (i32, i32) = (9, 9);

// '#' = wall, '.' = pellet, ' ' = empty path.
// Left/right symmetric and fully connected (verified at startup).
const MAZE: [&str; ROWS] = [
    "###################",
    "#........#........#",
    "#.##.###.#.###.##.#",
    "#.................#",
    "#.##.#.#####.#.##.#",
    "#....#...#...#....#",
    "####.###.#.###.####",
    "#......#...#......#",
    "####.#.#####.#.####",
    "#....#.......#....#",
    "####.#.#####.#.####",
    "#......#...#......#",
    "####.###.#.###.####",
    "#....#...#...#....#",
    "#.##.#.#####.#.##.#",
    "#.................#",
    "#.##.###.#.###.##.#",
    "#........#........#",
    "###################",
];

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Wall,
    Pellet,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::None => (0, 0),
        }
    }

    fn opposite(self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
            Dir::None => Dir::None,
        }
    }

    fn angle(self) -> f32 {
        // screen-space angle (y points down)
        match self {
            Dir::Right => 0.0,
            Dir::Left => std::f32::consts::PI,
            Dir::Up => -std::f32::consts::FRAC_PI_2,
            Dir::Down => std::f32::consts::FRAC_PI_2,
            Dir::None => 0.0,
        }
    }
}

struct Game {
    grid: Vec<Vec<Cell>>,
    pos: Vec2,    // pacman position in tile coordinates (integers = tile centers)
    dir: Dir,     // current travel direction
    want: Dir,    // buffered direction from input
    facing: Dir,  // last non-None direction, for drawing the mouth
    score: i32,
    remaining: i32, // pellets left
    won: bool,
}

impl Game {
    fn new() -> Game {
        let mut grid = vec![vec![Cell::Empty; COLS]; ROWS];
        for (j, row) in MAZE.iter().enumerate() {
            for (i, ch) in row.chars().enumerate() {
                grid[j][i] = match ch {
                    '#' => Cell::Wall,
                    '.' => Cell::Pellet,
                    _ => Cell::Empty,
                };
            }
        }
        // clear the tile pacman spawns on so it isn't counted as a pellet
        grid[PACMAN_START.1 as usize][PACMAN_START.0 as usize] = Cell::Empty;

        let remaining = grid
            .iter()
            .flatten()
            .filter(|c| **c == Cell::Pellet)
            .count() as i32;

        Game {
            grid,
            pos: vec2(PACMAN_START.0 as f32, PACMAN_START.1 as f32),
            dir: Dir::None,
            want: Dir::None,
            facing: Dir::Right,
            score: 0,
            remaining,
            won: false,
        }
    }

    fn is_wall(&self, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= COLS as i32 || j >= ROWS as i32 {
            return true;
        }
        self.grid[j as usize][i as usize] == Cell::Wall
    }

    fn read_input(&mut self) {
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            self.want = Dir::Up;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            self.want = Dir::Down;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            self.want = Dir::Left;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            self.want = Dir::Right;
        }
    }

    fn update(&mut self, dt: f32) {
        self.read_input();
        if self.won {
            return;
        }

        // allow instant reversal anywhere in a corridor (classic feel)
        if self.dir != Dir::None && self.want == self.dir.opposite() {
            self.dir = self.want;
        }

        let mut remaining = SPEED * dt;

        loop {
            let cx = self.pos.x.round();
            let cy = self.pos.y.round();
            let at_center =
                (self.pos.x - cx).abs() < 1e-4 && (self.pos.y - cy).abs() < 1e-4;

            if at_center {
                self.pos = vec2(cx, cy);
                let ci = cx as i32;
                let cj = cy as i32;

                self.eat(ci, cj);

                // turn toward the buffered direction if that path is open
                let (wx, wy) = self.want.delta();
                if self.want != Dir::None && !self.is_wall(ci + wx, cj + wy) {
                    self.dir = self.want;
                }
                // stop if a wall is dead ahead
                let (dx, dy) = self.dir.delta();
                if self.dir != Dir::None && self.is_wall(ci + dx, cj + dy) {
                    self.dir = Dir::None;
                }
                if self.dir == Dir::None {
                    break;
                }
                self.facing = self.dir;
            }

            if remaining <= 0.0 {
                break;
            }

            // move toward the next tile center in the current direction
            let target = match self.dir {
                Dir::Right => vec2(self.pos.x.floor() + 1.0, self.pos.y),
                Dir::Left => vec2(self.pos.x.ceil() - 1.0, self.pos.y),
                Dir::Down => vec2(self.pos.x, self.pos.y.floor() + 1.0),
                Dir::Up => vec2(self.pos.x, self.pos.y.ceil() - 1.0),
                Dir::None => break,
            };

            let to_target = (target - self.pos).length();
            if to_target <= 1e-6 {
                self.pos = target;
                continue;
            }
            let mv = remaining.min(to_target);
            let step = (target - self.pos) / to_target * mv;
            self.pos += step;
            remaining -= mv;

            if mv < to_target - 1e-6 {
                break; // ran out of distance before reaching the center
            }
            self.pos = target; // landed on a center; loop re-evaluates the turn
        }
    }

    fn eat(&mut self, i: i32, j: i32) {
        if self.grid[j as usize][i as usize] == Cell::Pellet {
            self.grid[j as usize][i as usize] = Cell::Empty;
            self.score += 10;
            self.remaining -= 1;
            if self.remaining == 0 {
                self.won = true;
            }
        }
    }

    fn draw(&self) {
        // maze
        for j in 0..ROWS {
            for i in 0..COLS {
                let px = i as f32 * TILE;
                let py = TOP + j as f32 * TILE;
                match self.grid[j][i] {
                    Cell::Wall => {
                        draw_rectangle(px + 1.0, py + 1.0, TILE - 2.0, TILE - 2.0, DARKBLUE);
                    }
                    Cell::Pellet => {
                        draw_circle(px + TILE / 2.0, py + TILE / 2.0, 2.5, GOLD);
                    }
                    Cell::Empty => {}
                }
            }
        }

        // pacman
        let px = self.pos.x * TILE + TILE / 2.0;
        let py = TOP + self.pos.y * TILE + TILE / 2.0;
        let r = TILE * 0.45;
        draw_circle(px, py, r, YELLOW);

        // animated mouth: a wedge of background color
        let base = self.facing.angle();
        let open = (get_time() as f32 * 12.0).sin().abs() * 0.55 + 0.05;
        let p1 = vec2(px, py);
        let p2 = vec2(px + r * 1.2 * (base - open).cos(), py + r * 1.2 * (base - open).sin());
        let p3 = vec2(px + r * 1.2 * (base + open).cos(), py + r * 1.2 * (base + open).sin());
        draw_triangle(p1, p2, p3, BLACK);

        // HUD
        draw_text(&format!("SCORE {}", self.score), 10.0, 28.0, 30.0, WHITE);

        if self.won {
            let msg = "YOU WIN!  press R";
            let size = 40.0;
            let dim = measure_text(msg, None, size as u16, 1.0);
            draw_text(
                msg,
                screen_width() / 2.0 - dim.width / 2.0,
                screen_height() / 2.0,
                size,
                YELLOW,
            );
        }
    }
}

// Flood fill from pacman's start; warn if any pellet is unreachable.
fn check_connectivity() {
    let mut open = vec![vec![false; COLS]; ROWS];
    let mut total = 0;
    for (j, row) in MAZE.iter().enumerate() {
        for (i, ch) in row.chars().enumerate() {
            if ch != '#' {
                open[j][i] = true;
                total += 1;
            }
        }
    }
    let mut seen = vec![vec![false; COLS]; ROWS];
    let mut q = VecDeque::new();
    let start = (PACMAN_START.0 as usize, PACMAN_START.1 as usize);
    seen[start.1][start.0] = true;
    q.push_back(start);
    let mut reached = 1;
    while let Some((x, y)) = q.pop_front() {
        for (dx, dy) in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || ny < 0 || nx >= COLS as i32 || ny >= ROWS as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if open[ny][nx] && !seen[ny][nx] {
                seen[ny][nx] = true;
                reached += 1;
                q.push_back((nx, ny));
            }
        }
    }
    if reached != total {
        eprintln!(
            "WARNING: maze not fully connected ({}/{} open tiles reachable from start) — some pellets are uneatable",
            reached, total
        );
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Pacman".to_owned(),
        window_width: (COLS as f32 * TILE) as i32,
        window_height: (ROWS as f32 * TILE + TOP) as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    check_connectivity();
    let mut game = Game::new();

    loop {
        if is_key_pressed(KeyCode::R) {
            game = Game::new();
        }
        let dt = get_frame_time().min(0.05); // clamp big frame gaps so we never skip a tile
        game.update(dt);

        clear_background(BLACK);
        game.draw();
        next_frame().await;
    }
}
