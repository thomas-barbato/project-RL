use macroquad::prelude::*;

#[macroquad::main("Project-rl")]
async fn main() {
    loop {
        clear_background(RED);
        next_frame().await
    }
}
