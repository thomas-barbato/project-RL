mod ascii_app;

use ascii_app::AsciiApp;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Project RL — ASCII Engine View".to_owned(),
        window_width: 1280,
        window_height: 800,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = match AsciiApp::new() {
        Ok(app) => app,
        Err(error) => {
            run_error_screen(&error).await;
            return;
        }
    };

    loop {
        app.update();
        app.draw();
        next_frame().await;
    }
}

async fn run_error_screen(error: &str) {
    eprintln!("[APP] Startup failed: {error}");
    loop {
        clear_background(Color::from_rgba(5, 8, 12, 255));
        draw_text("PROJECT RL — STARTUP ERROR", 32.0, 56.0, 30.0, RED);
        draw_text(error, 32.0, 96.0, 20.0, LIGHTGRAY);
        next_frame().await;
    }
}
