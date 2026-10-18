#[cfg(debug_assertions)]
mod agent_skill_lab_review;
mod ascii_app;
mod attack_preview_visuals;
mod camp_atmosphere;
mod camp_npc_review;
mod combat_visuals;
mod controls;
mod cryogenic_camp;
mod cryogenic_camp_render;
mod effect_geometry;
mod game_art;
mod graphics;
mod guardian_taunt_visuals;
mod hub_layout;
mod melee_strike_visuals;
mod pause_menu;
mod preview_markers;
mod skill_learn_button;
mod surface_layout;
mod suspension;
mod target_outline;
mod terminal_chrome;
mod terminal_dialogue;
mod terminal_view;
mod test_expedition;
mod test_regional;
mod test_sector;
#[cfg(debug_assertions)]
mod ui_capture;
mod ui_skin;
mod ui_theme;
mod visual_effects;
mod watcher_signals;

use ascii_app::{AsciiApp, TerminalDialogue};
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
    terminal_chrome::prepare_text();
    terminal_view::initialize_player_art();
    game_art::prewarm();
    #[cfg(debug_assertions)]
    if let Some(output) = graphics::ui_smoke_output() {
        let mode = std::env::args().nth(1).unwrap_or_default();
        let result = if matches!(
            mode.as_str(),
            "--ui-cold-aim-styles" | "--ui-cold-aim-choice"
        ) {
            AsciiApp::capture_aim_styles(&output, mode == "--ui-cold-aim-choice").await
        } else if mode == "--ui-cold-marker-pulse" {
            AsciiApp::capture_marker_pulse(&output).await
        } else if mode == "--ui-cold-skill-learn-buttons" {
            AsciiApp::capture_skill_learn_buttons(&output).await
        } else if mode == "--ui-cold-weapon-sets" {
            AsciiApp::capture_weapon_sets_review(&output).await
        } else if mode == "--ui-cold-skill-demo" {
            AsciiApp::capture_skill_demo(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-effect-footprints"
                | "--ui-cold-effect-footprints-fire"
                | "--ui-cold-souffle-corridor"
        ) {
            AsciiApp::capture_effect_footprints(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-walk-destinations" | "--ui-cold-walk-preview"
        ) {
            AsciiApp::capture_walk_preview(&output).await
        } else if mode == "--ui-cold-dialogue-terminal" {
            AsciiApp::capture_terminal_dialogue(&output).await
        } else if mode == "--ui-cold-hover-outline" {
            AsciiApp::capture_target_outline(&output).await
        } else if mode == "--ui-cold-camp-npc-review" {
            AsciiApp::capture_camp_npc_review(&output).await
        } else if mode == "--ui-cold-cryogenic-camp" {
            AsciiApp::capture_cryogenic_camp(&output).await
        } else if mode == "--ui-cold-campaign-bestiary" {
            AsciiApp::capture_campaign_monsters(&output, false, false).await
        } else if mode == "--ui-cold-monster-palettes" {
            AsciiApp::capture_campaign_monsters(&output, true, false).await
        } else if mode == "--ui-cold-monster-blue-palettes" {
            AsciiApp::capture_campaign_monsters(&output, true, true).await
        } else if mode == "--ui-cold-inspection" {
            AsciiApp::capture_inspection(&output).await
        } else if mode == "--ui-cold-native-hud" {
            AsciiApp::capture_native_hud(&output).await
        } else if mode == "--ui-cold-agent-catalogue-playable" {
            AsciiApp::capture_agent_catalogue_playable(&output).await
        } else if mode == "--ui-cold-agent-skill-animations" {
            agent_skill_lab_review::capture(&output).await
        } else if mode == "--ui-cold-agent-spell-playable" {
            AsciiApp::capture_agent_spell_playable(&output).await
        } else if mode == "--ui-cold-mixed-weapons" {
            AsciiApp::capture_mixed_weapon_trial(&output).await
        } else if mode == "--ui-cold-agent-fireball-playable" {
            AsciiApp::capture_agent_fireball_playable(&output).await
        } else if mode == "--ui-cold-agent-blast-playable" {
            AsciiApp::capture_agent_blast_playable(&output).await
        } else if mode == "--ui-cold-agent-melee-playable" {
            AsciiApp::capture_agent_melee_playable(&output).await
        } else if mode == "--ui-cold-agent-drone-playable" {
            AsciiApp::capture_agent_drone_playable(&output).await
        } else if mode == "--ui-cold-tactical-reading" {
            AsciiApp::capture_tactical_reading_integration(&output).await
        } else if mode == "--ui-cold-tactical-listening" {
            AsciiApp::capture_tactical_reading_proposal(&output).await
        } else if mode == "--ui-cold-installations" {
            AsciiApp::capture_installation_interactions(&output).await
        } else if mode == "--ui-cold-performance-actions" {
            AsciiApp::capture_action_performance(&output)
        } else if mode == "--ui-cold-performance-turns" {
            AsciiApp::capture_turn_performance(&output)
        } else if mode == "--ui-cold-performance-recovery" {
            AsciiApp::capture_recovery_performance(&output).await
        } else if mode == "--ui-cold-performance-lights" {
            AsciiApp::capture_lighting_performance(&output).await
        } else if mode == "--ui-cold-performance" {
            AsciiApp::capture_performance(&output).await
        } else if mode.starts_with("--ui-cold") {
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
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-demonstrations-competences") {
        if let Err(error) = AsciiApp::run_skill_demo_review().await {
            run_error_screen(&error).await;
        }
        return;
    }
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-boutons-competences") {
        AsciiApp::run_skill_learn_buttons_review().await;
        return;
    }
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-deplacement-souris") {
        if let Err(error) = AsciiApp::run_walk_preview_review().await {
            run_error_screen(&error).await;
        }
        return;
    }
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-laboratoire-competences") {
        agent_skill_lab_review::run().await;
        return;
    }
    #[cfg(debug_assertions)]
    if std::env::args().any(|arg| arg == "--apercu-lecture-tactique") {
        if let Err(error) = AsciiApp::run_tactical_reading_proposal().await {
            run_error_screen(&error).await;
        }
        return;
    }
    let trial = std::env::args().any(|arg| arg == "--essai-expedition");
    let installation_trial = std::env::args().any(|arg| arg == "--essai-interactions");
    let mut app = match (|| {
        if std::env::args().any(|arg| arg == "--laboratoire-competences") {
            return AsciiApp::new_agent_skill_laboratory();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-sort-agent") {
            return AsciiApp::new_agent_weapon_switch_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-armes-mixtes") {
            return AsciiApp::new_mixed_weapon_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-braise") {
            return AsciiApp::new_agent_ember_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-boule-de-feu") {
            return AsciiApp::new_agent_fireball_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-deflagration-occulte") {
            return AsciiApp::new_agent_blast_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-frappe-rupture") {
            return AsciiApp::new_agent_melee_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-drones-agent") {
            return AsciiApp::new_agent_drone_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-lecture-tactique") {
            return AsciiApp::new_tactical_reading_trial();
        }
        if installation_trial {
            AsciiApp::new_installation_trial()
        } else if trial {
            AsciiApp::new_playable_trial()
        } else {
            AsciiApp::new()
        }
    })() {
        Ok(app) => app,
        Err(error) => {
            run_error_screen(&error).await;
            return;
        }
    };

    let mut dialogue = match TerminalDialogue::load().await {
        Ok(dialogue) => dialogue,
        Err(error) => {
            eprintln!("[DIALOGUE AUDIO] {error}");
            TerminalDialogue::silent()
        }
    };
    loop {
        if is_quit_requested() {
            app.request_quit();
        } else if TerminalDialogue::officer(&app).is_some() {
            let mut input = controls::InputFrame::capture();
            dialogue.dismiss(&mut app, &mut input, get_time());
        } else {
            // Do not let input from the close frame dismiss a save error or
            // change the run after its suspension has been written.
            app.update();
        }
        if app.should_quit() {
            break;
        }
        if let Err(error) = dialogue.draw(&mut app, get_time()) {
            eprintln!("[DIALOGUE] {error}");
            dialogue.stop();
            app.draw();
        }
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
