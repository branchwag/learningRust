use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Hello, raylib!")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_text("Hello from Rust + raylib!", 190, 200, 20, Color::WHITE);
    }
}
