#[cfg(debug_assertions)]
mod agent_skill_lab_review;
mod animation_policy;
#[cfg(debug_assertions)]
mod animation_readability_review;
#[cfg(debug_assertions)]
mod art_quality_capture;
mod ascii_app;
mod asset_loading;
mod asset_preparation;
mod asset_smoke;
mod attack_preview_visuals;
mod camp_atmosphere;
mod camp_npc_review;
#[cfg(debug_assertions)]
mod class_armor_capture;
mod combat_visuals;
mod controls;
mod cryogenic_camp;
mod cryogenic_camp_render;
#[cfg(debug_assertions)]
mod dual_walk_capture;
mod effect_geometry;
#[cfg(debug_assertions)]
mod floor_surface_review;
mod game_art;
mod graphics;
mod guardian_taunt_visuals;
mod hub_layout;
mod melee_strike_visuals;
mod natural_effect;
mod pause_menu;
mod preview_markers;
mod prison_street;
mod prison_street_render;
mod sewer;
mod sewer_ambience;
#[cfg(debug_assertions)]
mod sewer_review;
mod sewer_water;
#[cfg(debug_assertions)]
mod sewer_water_review;
mod skill_learn_button;
#[cfg(debug_assertions)]
mod street_defense_review;
mod street_population;
mod surface_layout;
mod suspension;
mod target_outline;
mod terminal_chrome;
mod terminal_dialogue;
mod terminal_fire;
mod terminal_view;
mod test_expedition;
mod test_regional;
mod test_sector;
#[cfg(debug_assertions)]
mod ui_capture;
mod ui_skin;
mod ui_theme;
#[cfg(all(debug_assertions, test))]
mod vertical_walk_catalog18;
#[cfg(debug_assertions)]
mod vertical_walk_extension18;
#[cfg(debug_assertions)]
mod vertical_walk_integration18;
#[cfg(all(debug_assertions, test))]
mod vertical_walk_palette18;
#[cfg(debug_assertions)]
mod vertical_walk_review18;
mod visual_effects;
mod visual_polish;
#[cfg(debug_assertions)]
mod visual_polish_review;
#[cfg(debug_assertions)]
mod walk_lighting_capture;
#[cfg(debug_assertions)]
mod walk_review;
mod watcher_signals;
mod world_readability;

use ascii_app::{AsciiApp, TerminalDialogue};
use macroquad::prelude::*;

fn window_conf() -> Conf {
    let settings = graphics::startup().0;
    let polish_review = cfg!(debug_assertions)
        && matches!(
            std::env::args().nth(1).as_deref(),
            Some("--apercu-finition-visuelle" | "--ui-cold-visual-polish")
        );
    let water_review = cfg!(debug_assertions)
        && matches!(
            std::env::args().nth(1).as_deref(),
            Some(
                "--apercu-eau-egouts"
                    | "--ui-cold-sewer-water"
                    | "--apercu-canaux-egouts"
                    | "--ui-cold-sewer-canals"
            )
        );
    let reviewing = polish_review
        || water_review
        || (cfg!(debug_assertions)
            && std::env::args().nth(1).as_deref() == Some("--apercu-animations"));
    Conf {
        window_title: if water_review {
            "Project RL — EG-EAU01 · Revue des égouts"
        } else if polish_review {
            "Project RL — POLISH01 · Revue de finition"
        } else if reviewing {
            "Project RL — Revue des animations"
        } else {
            "Project RL — Terminal"
        }
        .to_owned(),
        window_width: if polish_review || water_review {
            1360
        } else {
            settings.windowed_size[0] as i32
        },
        window_height: if water_review {
            672
        } else if polish_review {
            800
        } else {
            settings.windowed_size[1] as i32
        },
        // Miniquad's desktop fullscreen is borderless, not an exclusive mode.
        fullscreen: !reviewing && settings.mode == graphics::WindowMode::Borderless,
        window_resizable: true,
        high_dpi: true,
        ..Default::default()
    }
}

fn main() {
    if std::env::args().nth(1).as_deref() == Some("--verify-assets") {
        match project_rl::data_paths::verify() {
            Ok(count) => println!(
                "Verified {count} packaged files and game content: {}",
                project_rl::data_paths::root().display()
            ),
            Err(error) => {
                eprintln!("[ASSETS] {error}");
                std::process::exit(1);
            }
        }
        return;
    }
    macroquad::Window::from_config(window_conf(), game_main());
}

async fn game_main() {
    #[cfg(debug_assertions)]
    let loading_start =
        if std::env::args().nth(1).as_deref() == Some("--ui-cold-performance-loading") {
            if !std::env::args().any(|arg| arg == "--no-loading-trace") {
                project_rl::loading_profile::start();
            }
            Some(std::time::Instant::now())
        } else {
            None
        };
    show_mouse(true);
    miniquad::window::set_mouse_cursor(miniquad::CursorIcon::Default);
    prevent_quit();
    asset_preparation::initialize_graphics_thread();
    if let Err(error) = ui_theme::initialize_fonts() {
        eprintln!("[UI] Police embarquée indisponible, police de secours utilisée : {error}");
    }
    terminal_chrome::prepare_text();
    terminal_view::initialize_player_art();
    game_art::prewarm();
    #[cfg(debug_assertions)]
    if let Some(started) = loading_start {
        let output = graphics::ui_smoke_output().expect("loading diagnostic output");
        if let Err(error) =
            AsciiApp::capture_loading_performance(&output, started.elapsed().as_secs_f64() * 1000.)
                .await
        {
            eprintln!("[LOADING] {error}");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(debug_assertions)]
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some(
            "--apercu-eau-egouts"
                | "--ui-cold-sewer-water"
                | "--apercu-canaux-egouts"
                | "--ui-cold-sewer-canals"
        )
    ) {
        let output = matches!(
            std::env::args().nth(1).as_deref(),
            Some("--ui-cold-sewer-water" | "--ui-cold-sewer-canals")
        )
        .then(|| std::env::args_os().nth(2).map(std::path::PathBuf::from))
        .flatten();
        if let Err(error) = sewer_water_review::run(output.as_deref()).await {
            eprintln!("[EG-EAU01] {error}");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(debug_assertions)]
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some("--apercu-finition-visuelle" | "--ui-cold-visual-polish")
    ) {
        let capture = std::env::args().nth(1).as_deref() == Some("--ui-cold-visual-polish");
        let output = capture
            .then(|| std::env::args_os().nth(2).map(std::path::PathBuf::from))
            .flatten();
        if capture && output.is_none() {
            eprintln!("[POLISH01] --ui-cold-visual-polish OUTPUT");
            std::process::exit(1);
        }
        if let Err(error) = visual_polish_review::run(output.as_deref()).await {
            eprintln!("[POLISH01] {error}");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(debug_assertions)]
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some("--apercu-animations" | "--ui-cold-walk-catalogue")
    ) {
        let output = std::env::args_os().nth(2).map(std::path::PathBuf::from);
        if let Err(error) = walk_review::run(output.as_deref()).await {
            eprintln!("[WALK08 REVIEW] {error}");
        }
        return;
    }
    if std::env::args().nth(1).as_deref() == Some("--smoke-assets") {
        let output = std::env::args_os()
            .nth(2)
            .map(std::path::PathBuf::from)
            .expect("--smoke-assets OUTPUT");
        if let Err(error) = asset_smoke::run(&output).await {
            eprintln!("[ASSETS] {error}");
            std::process::exit(1);
        }
        return;
    }
    #[cfg(debug_assertions)]
    if let Some(output) = graphics::ui_smoke_output() {
        let mode = std::env::args().nth(1).unwrap_or_default();
        let result = if mode == "--ui-cold-vertical-integration" {
            vertical_walk_integration18::capture(&output).await
        } else if mode == "--ui-cold-vertical-extension" {
            vertical_walk_extension18::capture(&output).await
        } else if mode == "--ui-cold-vertical-size" {
            vertical_walk_review18::capture_sizes(&output).await
        } else if mode == "--ui-cold-vertical-headgear" {
            vertical_walk_review18::capture_heads(&output).await
        } else if mode == "--ui-cold-vertical-equipment" {
            vertical_walk_review18::capture_equipment(&output).await
        } else if mode == "--ui-cold-vertical-roster" {
            vertical_walk_review18::capture_roster(&output).await
        } else if mode == "--ui-cold-vertical-walk" {
            vertical_walk_review18::capture(&output).await
        } else if mode == "--ui-cold-dual-texture" || mode == "--ui-cold-dual-texture-candidate" {
            let _review = (mode == "--ui-cold-dual-texture-candidate")
                .then(|| game_art::walking::review_dual17(true));
            match std::env::var_os("RL_WALK_SAVED_RUN") {
                Some(path) => {
                    AsciiApp::capture_saved_walk_live(&std::path::PathBuf::from(path), &output)
                        .await
                }
                None => Err("RL_WALK_SAVED_RUN required for isolated texture diagnostic".into()),
            }
        } else if matches!(
            mode.as_str(),
            "--ui-cold-dual-walk" | "--ui-cold-dual-walk-integration"
        ) {
            dual_walk_capture::capture(&output).await
        } else if mode == "--ui-cold-corridor-wear-integration" {
            AsciiApp::capture_corridor_wear_integration(&output).await
        } else if mode == "--ui-cold-corridor-wear" {
            AsciiApp::capture_corridor_wear(&output).await
        } else if mode == "--ui-cold-corridor-grate" {
            AsciiApp::capture_corridor_grate(&output).await
        } else if mode == "--ui-cold-future-floor" {
            AsciiApp::capture_future_floor(&output).await
        } else if mode == "--ui-cold-camp-floor-03" {
            AsciiApp::capture_camp_floor_03(&output).await
        } else if mode == "--ui-cold-exterior-floor-integration" {
            AsciiApp::capture_exterior_floor_integration(&output).await
        } else if mode == "--ui-cold-continuous-floor" {
            AsciiApp::capture_continuous_floor(&output).await
        } else if mode == "--ui-cold-sewer-canals-integration" {
            AsciiApp::capture_sewer_canals_integration(&output).await
        } else if mode == "--ui-cold-walk-lighting" {
            walk_lighting_capture::capture(&output).await
        } else if mode == "--ui-cold-polish-integration" {
            AsciiApp::capture_polish_integration(&output).await
        } else if mode == "--ui-cold-proportions-npcs" {
            AsciiApp::capture_npc_proportions(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-walk-integration" | "--ui-cold-walk08-integration"
        ) {
            let _review = (mode == "--ui-cold-walk08-integration")
                .then(game_art::walking::review_corrections);
            AsciiApp::capture_walk_integration(&output).await
        } else if mode == "--ui-cold-actor-readability" {
            AsciiApp::capture_actor_readability(&output).await
        } else if mode == "--ui-cold-class-armor-integration" {
            class_armor_capture::capture(&output).await
        } else if mode == "--ui-cold-animation-readability" {
            animation_readability_review::capture(&output).await
        } else if mode == "--ui-cold-animation-shape" {
            animation_readability_review::shape::capture(&output).await
        } else if mode == "--ui-cold-blizzard-readability" {
            animation_readability_review::blizzard::capture(&output).await
        } else if mode == "--ui-cold-combat-fixes-review" {
            AsciiApp::capture_combat_fixes_review(&output).await
        } else if mode == "--ui-cold-sewer-integration" {
            AsciiApp::capture_sewer_integration(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-sewer-review"
                | "--ui-cold-sewer-decor-review"
                | "--ui-cold-sewer-habitat-review"
                | "--ui-cold-sewer-composition-review"
        ) {
            sewer_review::configure_review(match mode.as_str() {
                "--ui-cold-sewer-composition-review" => 4,
                "--ui-cold-sewer-habitat-review" => 3,
                "--ui-cold-sewer-decor-review" => 2,
                _ => 1,
            });
            AsciiApp::capture_sewer_review(&output).await
        } else if mode == "--ui-cold-street-population" {
            AsciiApp::capture_street_population(&output).await
        } else if mode == "--ui-cold-street-defense-10" {
            AsciiApp::capture_street_defense(&output).await
        } else if mode == "--ui-cold-street-repairs-09" {
            AsciiApp::capture_street_repairs(&output).await
        } else if mode == "--ui-cold-torso-icons-integration" {
            AsciiApp::capture_torso_icons(&output).await
        } else if mode == "--ui-cold-art-finish-completion" {
            AsciiApp::capture_art_finish_completion(&output).await
        } else if mode == "--ui-cold-art-finish-05-review" {
            AsciiApp::capture_art_finish_05(&output).await
        } else if mode == "--ui-cold-art-finish-05-integration" {
            AsciiApp::capture_art_finish_05_integration(&output).await
        } else if mode == "--ui-cold-art-finish-04-review" {
            AsciiApp::capture_art_finish_04(&output).await
        } else if mode == "--ui-cold-art-finish-04-integration" {
            AsciiApp::capture_art_finish_04_integration(&output).await
        } else if mode == "--ui-cold-art-finish-03-review" {
            AsciiApp::capture_art_finish_03(&output).await
        } else if mode == "--ui-cold-art-finish-03-integration" {
            AsciiApp::capture_art_finish_03_integration(&output).await
        } else if mode == "--ui-cold-art-finish-02-review" {
            AsciiApp::capture_art_finish_02(&output).await
        } else if mode == "--ui-cold-art-finish-02-integration" {
            AsciiApp::capture_art_finish_02_integration(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-art-finish-review" | "--ui-cold-art-finish-integration"
        ) {
            AsciiApp::capture_art_finish(&output).await
        } else if mode == "--ui-cold-art-quality-audit" {
            art_quality_capture::capture(&output).await
        } else if mode == "--ui-cold-street-civilians-integration" {
            AsciiApp::capture_street_civilians(&output).await
        } else if matches!(
            mode.as_str(),
            "--ui-cold-street-style"
                | "--ui-cold-street-style-integration"
                | "--ui-cold-street-paving-integration"
        ) {
            AsciiApp::capture_street_style(&output).await
        } else if mode == "--ui-cold-world-fixes" {
            AsciiApp::capture_world_fixes(&output).await
        } else if matches!(
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
        } else if matches!(
            mode.as_str(),
            "--ui-cold-prison-street"
                | "--ui-cold-prison-street-lighting"
                | "--ui-cold-street-roofs"
                | "--ui-cold-street-roofs-integration"
                | "--ui-cold-street-expansion"
                | "--ui-cold-street-integration"
                | "--ui-cold-street-defects"
        ) {
            AsciiApp::capture_prison_street(&output).await
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
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-egouts") {
            return AsciiApp::new_sewer_trial();
        }
        #[cfg(debug_assertions)]
        if std::env::args().any(|arg| arg == "--essai-population-prison") {
            return AsciiApp::new_street_population_trial();
        }
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
        if let Err(error) = app.prepare_presentation() {
            run_error_screen(&error).await;
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
