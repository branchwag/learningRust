use macroquad::prelude::*;

#[macroquad::main("MyGame")]
async fn main() {
    loop {
        clear_background(BLUE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, YELLOW);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_triangle(
            vec2(100.0, 100.0),
            vec2(200.0, 100.0),
            vec2(150.0, 200.0),
            WHITE,
        );

        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
