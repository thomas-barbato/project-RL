mod ascii_app;
mod controls;
mod graphics;
mod pause_menu;
mod suspension;
mod terminal_view;
mod test_expedition;
mod test_regional;
mod test_sector;
#[cfg(debug_assertions)]
mod ui_capture;
mod ui_theme;
mod visual_effects;

use ascii_app::AsciiApp;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let settings = graphics::startup().0;
    Conf {
        window_title: "Project RL — Terminal".to_owned(),
        window_width: settings.windowed_size[0] as i32,
        window_height: settings.windowed_size[1] as i32,
        // Miniquad's desktop fullscreen is borderless, not an exclusive mode.
        fullscreen: settings.mode == graphics::WindowMode::Borderless,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    prevent_quit();
    if let Err(error) = ui_theme::initialize_fonts() {
        eprintln!("[UI] Police embarquée indisponible, police de secours utilisée : {error}");
    }
    #[cfg(debug_assertions)]
    if let Some(output) = graphics::ui_smoke_output() {
        let mode = std::env::args().nth(1).unwrap_or_default();
        let result = if mode.starts_with("--ui-cold") {
            AsciiApp::capture_cold_start(&output, mode.strip_prefix("--ui-cold-").unwrap_or("game"))
                .await
        } else {
            AsciiApp::capture_ui_checks(&output).await
        };
        if let Err(error) = result {
            eprintln!("[UI CHECK] {error}");
            std::process::exit(1);
        }
        return;
    }
    let mut app = match AsciiApp::new() {
        Ok(app) => app,
        Err(error) => {
            run_error_screen(&error).await;
            return;
        }
    };

    loop {
        if is_quit_requested() {
            app.request_quit();
        } else {
            // Do not let input from the close frame dismiss a save error or
            // change the run after its suspension has been written.
            app.update();
        }
        if app.should_quit() {
            break;
        }
        app.draw();
        next_frame().await;
    }
}

async fn run_error_screen(error: &str) {
    eprintln!("[APP] Startup failed: {error}");
    loop {
        if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
            return;
        }
        clear_background(Color::from_rgba(5, 8, 12, 255));
        ui_theme::draw_text_bold("PROJECT RL", 32.0, 56.0, 30.0, LIGHTGRAY);
        ui_theme::draw_text(
            "Impossible de lancer le jeu.",
            32.0,
            96.0,
            22.0,
            Color::from_rgba(255, 138, 118, 255),
        );
        ui_theme::draw_text(
            "Fermez la fenêtre puis réessayez. Échap permet de quitter.",
            32.0,
            128.0,
            18.0,
            LIGHTGRAY,
        );
        next_frame().await;
    }
}
