use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use rand::Rng;
use std::collections::VecDeque;
use std::io::{self, Write};
use std::time::{Duration, Instant};

const WIDTH: u16 = 40;
const HEIGHT: u16 = 20;
const TICK_RATE: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: u16,
    y: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    snake: VecDeque<Position>,
    direction: Direction,
    food: Position,
    score: u32,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            snake: VecDeque::new(),
            direction: Direction::Right,
            food: Position { x: 0, y: 0 },
            score: 0,
            game_over: false,
        };

        // Start snake in the middle
        let start_x = WIDTH / 2;
        let start_y = HEIGHT / 2;
        game.snake.push_back(Position {
            x: start_x,
            y: start_y,
        });

        game.spawn_food();
        game
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::rng();
        loop {
            let pos = Position {
                x: rng.gen_range(1..WIDTH - 1),
                y: rng.gen_range(1..HEIGHT - 1),
            };

            // Make sure food doesn't spawn on the snake
            if !self.snake.contains(&pos) {
                self.food = pos;
                break;
            }
        }
    }

    fn update(&mut self) {
        if self.game_over {
            return;
        }

        let head = self.snake.front().unwrap();
        let new_head = match self.direction {
            Direction::Up => Position {
                x: head.x,
                y: head.y.saturating_sub(1),
            },
            Direction::Down => Position {
                x: head.x,
                y: head.y + 1,
            },
            Direction::Left => Position {
                x: head.x.saturating_sub(1),
                y: head.y,
            },
            Direction::Right => Position {
                x: head.x + 1,
                y: head.y,
            },
        };

        // Check wall collision
        if new_head.x == 0 || new_head.x >= WIDTH - 1 || new_head.y == 0 || new_head.y >= HEIGHT - 1
        {
            self.game_over = true;
            return;
        }

        // Check self collision
        if self.snake.contains(&new_head) {
            self.game_over = true;
            return;
        }

        self.snake.push_front(new_head);

        // Check if ate food
        if new_head == self.food {
            self.score += 1;
            self.spawn_food();
        } else {
            self.snake.pop_back();
        }
    }

    fn change_direction(&mut self, new_direction: Direction) {
        // Prevent 180-degree turns
        let valid = match (self.direction, new_direction) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => false,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => false,
            _ => true,
        };

        if valid {
            self.direction = new_direction;
        }
    }
    fn render(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        // Clear screen and move to top
        execute!(
            stdout,
            terminal::Clear(ClearType::Purge),
            cursor::MoveTo(0, 0)
        )?;

        // Draw top border
        println!("{}", "═".repeat(WIDTH as usize));

        // Draw game area
        for y in 1..HEIGHT - 1 {
            print!("║");
            for x in 1..WIDTH - 1 {
                let pos = Position { x, y };
                if self.snake.front() == Some(&pos) {
                    print!("●"); // Snake head
                } else if self.snake.contains(&pos) {
                    print!("○"); // Snake body
                } else if self.food == pos {
                    print!("◆"); // Food
                } else {
                    print!(" ");
                }
            }
            println!("║");
        }

        // Draw bottom border
        println!("{}", "═".repeat(WIDTH as usize));
        println!(
            "Score: {} | Use arrow keys to move | Press 'q' to quit",
            self.score
        );

        if self.game_over {
            println!("GAME OVER! Press 'r' to restart or 'q' to quit");
        }

        stdout.flush()?;
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Enter raw mode and hide cursor
    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut game = Game::new();
    let mut last_tick = Instant::now();

    // Initial render
    game.render()?;

    loop {
        // Handle input (non-blocking)
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('r') if game.game_over => {
                        game = Game::new();
                        last_tick = Instant::now();
                        game.render()?; // Render after restart
                    }
                    KeyCode::Up => game.change_direction(Direction::Up),
                    KeyCode::Down => game.change_direction(Direction::Down),
                    KeyCode::Left => game.change_direction(Direction::Left),
                    KeyCode::Right => game.change_direction(Direction::Right),
                    _ => {}
                }
            }
        }

        // Update game state at fixed tick rate
        if last_tick.elapsed() >= TICK_RATE {
            game.update();
            game.render()?; // Only render after update
            last_tick = Instant::now();
        }
    }

    // Cleanup: restore terminal
    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
