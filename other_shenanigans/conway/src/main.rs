use eframe::egui;

const GRID_SIZE: usize = 25;
const CELL_SIZE: f32 = 15.0;

struct GameOfLife {
    grid: [[bool; GRID_SIZE]; GRID_SIZE],
    is_running: bool,
    last_update: std::time::Instant,
}

impl GameOfLife {
    fn new() -> Self {
        Self {
            grid: [[false; GRID_SIZE]; GRID_SIZE],
            is_running: false,
            last_update: std::time::Instant::now(),
        }
    }

    fn start_simulation(&mut self) {
        self.is_running = true;
        self.last_update = std::time::Instant::now();
    }

    fn stop_simulation(&mut self) {
        self.is_running = false;
    }

    fn clear_grid(&mut self) {
        self.grid = [[false; GRID_SIZE]; GRID_SIZE];
    }

    fn compute_next_generation(&mut self) {
        let mut next_grid = [[false; GRID_SIZE]; GRID_SIZE];

        for row in 0..GRID_SIZE {
            for col in 0..GRID_SIZE {
                let live_neighbors = self.count_live_neighbors(row, col);

                if self.grid[row][col] {
                    next_grid[row][col] = live_neighbors == 2 || live_neighbors == 3;
                } else {
                    next_grid[row][col] = live_neighbors == 3;
                }
            }
        }

        self.grid = next_grid;
    }

    fn count_live_neighbors(&self, row: usize, col: usize) -> i32 {
        let mut live_neighbors = 0;
        for i in -1i32..=1 {
            for j in -1i32..=1 {
                if i == 0 && j == 0 {
                    continue;
                }

                let new_row = row as i32 + i;
                let new_col = col as i32 + j;

                if new_row >= 0 && new_row < GRID_SIZE as i32 && new_col >= 0 && new_col < GRID_SIZE as i32 {
                    if self.grid[new_row as usize][new_col as usize] {
                        live_neighbors += 1;
                    }
                }
            }
        }
        live_neighbors
    }
}

impl eframe::App for GameOfLife {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.is_running {
            let now = std::time::Instant::now();
            if now.duration_since(self.last_update).as_millis() >= 200 {
                self.compute_next_generation();
                self.last_update = now;
            }
        }

        let ctx = ui.ctx().clone();

        egui::Panel::bottom("controls").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                if ui.button(if self.is_running { "Stop" } else { "Start" }).clicked() {
                    if self.is_running {
                        self.stop_simulation();
                    } else {
                        self.start_simulation();
                    }
                }

                if ui.button("Clear").clicked() {
                    self.stop_simulation();
                    self.clear_grid();
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let grid_size_pixels = GRID_SIZE as f32 * CELL_SIZE;
            let (response, painter) = ui.allocate_painter(
                egui::Vec2::new(grid_size_pixels, grid_size_pixels),
                egui::Sense::click(),
            );

            for row in 0..GRID_SIZE {
                for col in 0..GRID_SIZE {
                    let rect = egui::Rect::from_min_size(
                        egui::Pos2::new(
                            response.rect.left() + col as f32 * CELL_SIZE,
                            response.rect.top() + row as f32 * CELL_SIZE,
                        ),
                        egui::Vec2::new(CELL_SIZE - 1.0, CELL_SIZE - 1.0),
                    );

                    let color = if self.grid[row][col] {
                        egui::Color32::from_rgb(0, 255, 0)
                    } else {
                        egui::Color32::from_rgb(40, 40, 40)
                    };

                    painter.rect_filled(rect, 0.0, color);
                }
            }

            if response.clicked() {
                if let Some(pos) = response.interact_pointer_pos() {
                    let local_x = pos.x - response.rect.left();
                    let local_y = pos.y - response.rect.top();
                    if local_x >= 0.0 && local_y >= 0.0 {
                        let col = (local_x / CELL_SIZE) as usize;
                        let row = (local_y / CELL_SIZE) as usize;
                        if row < GRID_SIZE && col < GRID_SIZE {
                            self.grid[row][col] = !self.grid[row][col];
                        }
                    }
                }
            }
        });

        if self.is_running {
            ctx.request_repaint();
        }
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Conway's Game of Life",
        options,
        Box::new(|_cc| Ok(Box::new(GameOfLife::new()))),
    )
}
