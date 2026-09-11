use std::collections::BTreeMap;

use crate::controls::{self, Action, Controls, InputFrame, KeySemantics};
use crate::graphics::{self, GraphicsSettings, GraphicsState, WindowMode};
use crate::pause_menu::{ControlsLayout, MenuFocus, MenuLayout, MenuScreen, wheel_steps};
use crate::suspension::{self, RecordedCommand, Suspension};
use crate::terminal_view::{
    TerminalAlertKind, TerminalAttackPreview, TerminalDrawOptions, TerminalOverlay,
    TerminalStatusIcon, TerminalView,
};
use crate::test_sector::TestSector;

use macroquad::prelude::*;
use project_rl::ai::AiProfile;
use project_rl::combat::{AttackArea, AttackProfile, DamageType};
use project_rl::content::{ContentLoader, ExpeditionCatalog};
use project_rl::effects::{AbilityProfile, ApplyStatusEffect, EffectPrimitive};
use project_rl::entity::{Actor, EntityId, ItemInstanceId};
use project_rl::facility::{
    DoorLockdownPrevention, FacilityEvent, InstallationCapability, WorkerRole,
};
use project_rl::game::{
    CommandOutcome, CommandRejection, GameCommand, GameEvent, GameRules, GameState, RunStatus,
    StartingItemStack, WorldState,
};
use project_rl::item::{ItemEffect, ItemId, ItemKind};
use project_rl::localization::TextCatalog;
use project_rl::loot::LootCatalog;
use project_rl::presentation::{VisualCue, VisualCueCatalog, VisualCueCell, VisualCueId};
use project_rl::progression::DefeatReward;
use project_rl::skills::{DisciplineId, SystemFeatureSet, TechniqueAction, TechniqueId};
use project_rl::status::StatusId;
use project_rl::weapon::WeaponId;
use project_rl::world::{Direction, DistanceMetric, GridPos, Terrain};

use crate::visual_effects::VisualCuePlayer;

const INITIAL_SEED: u64 = 20_260_909;
const CURRENT_GENERATION_VERSION: u8 = 10;
const LOG_CAPACITY: usize = 6;
const DISPLAY_LOCALE: &str = "fr";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AttackAim {
    slot: u8,
    cursor: GridPos,
}

pub struct AsciiApp {
    game: WorldState,
    terminal: TerminalView,
    zone_views: BTreeMap<project_rl::content::ContentId, TerminalView>,
    zone_decor: BTreeMap<project_rl::content::ContentId, crate::test_sector::SectorDecor>,
    facing: Direction,
    rules: GameRules,
    texts: TextCatalog,
    loot: LootCatalog,
    expeditions: ExpeditionCatalog,
    generation_version: u8,
    seed: u64,
    actor_glyphs: BTreeMap<EntityId, char>,
    selected_target: Option<EntityId>,
    attack_aim: Option<AttackAim>,
    attack_aim_pointer: Option<(f32, f32)>,
    visual_cues: VisualCuePlayer,
    trace_cells: BTreeMap<GridPos, Direction>,
    traces_visible_until: f64,
    log: Vec<String>,
    active_weapon_slot: u8,
    inventory_open: bool,
    inventory_selection: usize,
    inventory_message: String,
    skills_open: bool,
    skill_discipline_selection: usize,
    skill_technique_selection: usize,
    skill_message: String,
    observation_report: Vec<String>,
    report_open: bool,
    report_scroll: usize,
    legend_open: bool,
    controls: Controls,
    controls_path: std::path::PathBuf,
    graphics: GraphicsState,
    menu: MenuScreen,
    menu_selection: usize,
    menu_message: String,
    menu_focus: MenuFocus,
    cursor_icon: miniquad::CursorIcon,
    quit_requested: bool,
    options_selection: usize,
    options_scroll: usize,
    rebinding: bool,
    options_message: String,
    history: Vec<RecordedCommand>,
    suspension_path: std::path::PathBuf,
    session_lock: Option<std::fs::File>,
}

impl AsciiApp {
    pub fn new() -> Result<Self, String> {
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        let (bindings, message) = Controls::load(&app.controls_path, controls::detect_layout());
        app.controls = bindings;
        app.options_message = message;
        let (settings, message) = graphics::startup();
        app.graphics.active = *settings;
        app.graphics.draft = *settings;
        app.graphics.message = message.clone();
        app.session_lock = Some(suspension::session_lock(
            &app.suspension_path.with_extension("lock"),
        )?);
        if app
            .suspension_path
            .try_exists()
            .map_err(|error| error.to_string())?
        {
            app.open_menu(MenuScreen::ResumeSuspension);
        }
        Ok(app)
    }

    pub fn update(&mut self) {
        if self.quit_requested {
            return;
        }
        let previous_cursor = self.cursor_icon;
        if !self.tick_graphics(get_time()) {
            let input = self.graphics.active.transform_input(InputFrame::capture());
            self.update_input(&input);
        }
        self.apply_graphics_window_change();
        let cursor_icon = if self.attack_aim.is_some() {
            miniquad::CursorIcon::Crosshair
        } else if self.menu_focus.hovered.is_some() && !self.rebinding {
            miniquad::CursorIcon::Pointer
        } else {
            miniquad::CursorIcon::Default
        };
        if cursor_icon != previous_cursor {
            miniquad::window::set_mouse_cursor(cursor_icon);
        }
        self.cursor_icon = cursor_icon;
    }

    fn apply_graphics_window_change(&mut self) {
        if let Some((previous, settings)) = self.graphics.pending.take() {
            if previous.mode != settings.mode {
                set_fullscreen(settings.mode == WindowMode::Borderless);
            }
            if settings.mode == WindowMode::Windowed
                && (previous.mode != settings.mode
                    || previous.windowed_size != settings.windowed_size)
            {
                request_new_screen_size(
                    settings.windowed_size[0] as f32,
                    settings.windowed_size[1] as f32,
                );
            }
        }
    }

    /// Native render smoke check, debug builds only. Never loads or consumes the
    /// user's run/configuration and never writes settings. Images contain only
    /// this deterministic fixture's framebuffer, not the user's desktop.
    #[cfg(debug_assertions)]
    pub async fn capture_cold_start(output: &std::path::Path, scene: &str) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.graphics.active.mode = WindowMode::Windowed;
        for _ in 0..60 {
            app.draw();
            next_frame().await;
        }
        match scene {
            "game" => {}
            "pause" => app.open_menu(MenuScreen::Pause),
            "options" => app.open_menu(MenuScreen::Options),
            "graphics" => app.open_menu(MenuScreen::Graphics),
            "controls" => app.open_menu(MenuScreen::Controls),
            "resume" => app.open_menu(MenuScreen::ResumeSuspension),
            "legend" => app.legend_open = true,
            "maintenance" => {
                // Observe the checkpoint without occupying a worker route or
                // an interaction cell around the relay.
                app.walk_fixture_to(GridPos::new(45, 27))?;
                for _ in 0..96 {
                    if app.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                        return Err("Maintenance diagnostic wait rejected".into());
                    }
                    app.capture_events_at(Some(0.0));
                }
                let order = "core:restore_checkpoint_power"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                if app
                    .game
                    .active_facility()
                    .and_then(|facility| facility.repair_status(&order))
                    != Some(project_rl::facility::RepairStatus::Completed)
                {
                    return Err("Maintenance diagnostic did not complete".into());
                }
            }
            "maintenance-delivery" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                app.walk_fixture_to(GridPos::new(27, 31))?;
                app.facing = Direction::East;
                if app.execute_command(GameCommand::Interact {
                    target: GridPos::new(28, 31),
                }) != CommandOutcome::Applied
                {
                    return Err("Maintenance material delivery rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if app
                    .game
                    .player_inventory()
                    .iter()
                    .any(|entry| entry.item() == &regulator)
                    || !app.log.iter().any(|line| line.contains("vous livrez"))
                {
                    return Err("Maintenance material delivery was not committed".into());
                }
            }
            "maintenance-alert" => {
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if app.visible_local_alert_summary().is_none()
                    || !app.log.iter().any(|line| line.contains("ALERTE LOCALE"))
                {
                    return Err("Visible maintenance alert was not presented".into());
                }
            }
            "maintenance-security-alarm" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                let owner = "core:maintenance_collective"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(45, 27))?;
                for _ in 0..96 {
                    if app.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                        return Err("Security alarm diagnostic wait rejected".into());
                    }
                    app.capture_events_at(Some(0.0));
                }
                // Open the checkpoint door, then step back outside so its
                // configured lockdown can engage without enclosing the player.
                app.walk_fixture_to(GridPos::new(52, 29))?;
                app.walk_fixture_to(GridPos::new(52, 27))?;
                app.game
                    .spawn_ground_item_with_owner(GridPos::new(52, 27), regulator, 1, Some(owner))
                    .map_err(|error| error.to_string())?;
                app.game.drain_events();
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Security alarm diagnostic pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if app.visible_security_alarm_summary().is_none()
                    || !app
                        .log
                        .iter()
                        .any(|line| line.contains("ALARME DE SÉCURITÉ"))
                {
                    return Err("Visible installed security alarm was not presented".into());
                }
            }
            "maintenance-inventory" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                app.inventory_selection = app
                    .game
                    .player_inventory()
                    .iter()
                    .position(|entry| entry.item() == &regulator)
                    .ok_or("Maintenance material missing from inventory")?;
                app.inventory_open = true;
            }
            "expedition" => {
                app.walk_expedition_fixture(0)?;
            }
            "expedition-return" => {
                app.walk_expedition_fixture(1)?;
            }
            "expedition-revisit" => {
                app.walk_expedition_fixture(2)?;
            }
            "attack-preview" => {
                app.walk_fixture_to(GridPos::new(65, 21))?;
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                app.begin_attack_aim(None);
                let mut aim = app.attack_aim.ok_or("Area preview did not open")?;
                aim.cursor = aim.cursor.step(Direction::West);
                if app.game.actors().entity_at(aim.cursor).is_some() {
                    return Err("Area preview diagnostic cursor is not on empty ground".into());
                }
                if app
                    .game
                    .player_attack_preview(aim.slot, aim.cursor)
                    .is_err()
                {
                    return Err("Area preview is invalid in diagnostic scene".into());
                }
                app.attack_aim = Some(aim);
            }
            "attack-preview-protected" => {
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                app.begin_attack_aim(None);
                let aim = app
                    .attack_aim
                    .ok_or("Protected area preview did not open")?;
                if app.game.player_attack_preview(aim.slot, aim.cursor)
                    != Err(CommandRejection::ProtectedZone)
                {
                    return Err("Protected area preview did not expose its rejection".into());
                }
                if app
                    .game
                    .player_attack_footprint(aim.slot, aim.cursor)
                    .map_err(|error| format!("Protected area footprint failed: {error:?}"))?
                    .cells()
                    .len()
                    <= 1
                {
                    return Err("Protected area preview collapsed to its cursor".into());
                }
            }
            "empty-area-attack" => {
                let map = project_rl::world::Map::from_ascii(
                    "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
                )
                .map_err(|error| error.to_string())?;
                let origin = GridPos::new(1, 3);
                let target = GridPos::new(6, 3);
                let game = GameState::new_with_rules(map, origin, INITIAL_SEED, app.rules.clone())
                    .map_err(|error| error.to_string())?;
                app.terminal = TerminalView::new(
                    crate::test_sector::SectorDecor::default(),
                    game.map(),
                    game.player_visibility(),
                );
                app.game = WorldState::single(game);
                app.actor_glyphs.clear();
                app.active_weapon_slot = 2;
                app.attack_aim = Some(AttackAim {
                    slot: 2,
                    cursor: target,
                });
                let turn = app.game.turn();
                app.update_input(&InputFrame {
                    pressed: [controls::Binding::key("F")].into(),
                    ..Default::default()
                });
                if app.attack_aim.is_some()
                    || app.game.turn() != turn + 1
                    || app.game.ground_effects().is_empty()
                    || app.visual_cues.active_count() == 0
                    || !app
                        .log
                        .iter()
                        .any(|line| line.contains("ATTAQUE CONFIRMÉE"))
                {
                    return Err("Empty area attack did not complete its visible live path".into());
                }
            }
            "visual-effects" => {
                let origin = app
                    .game
                    .player_position()
                    .ok_or("Visual effect diagnostic has no player")?;
                let id = visual_cue_id("core:flamethrower");
                let attack = app
                    .game
                    .rules()
                    .weapons
                    .get(&id)
                    .ok_or("Visual effect diagnostic has no flamethrower")?
                    .attack();
                let cells = attack
                    .affected_cells(app.game.map(), origin, GridPos::new(origin.x + 5, origin.y))
                    .into_iter()
                    .map(|cell| VisualCueCell::new(cell.position, cell.step));
                let cue = VisualCue::world(id, origin, cells).map_err(|error| error.to_string())?;
                // Capture the overlap where the pressurized jet has reached
                // its long tip while the narrow base is still burning.
                app.visual_cues.play(cue, get_time() - 0.34);
            }
            "burning-status" => {
                let map = project_rl::world::Map::from_ascii(
                    "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
                )
                .map_err(|error| error.to_string())?;
                let mut game = GameState::new_with_rules(
                    map,
                    GridPos::new(1, 3),
                    INITIAL_SEED,
                    app.rules.clone(),
                )
                .map_err(|error| error.to_string())?;
                let target = game
                    .spawn_actor(
                        Actor::new(GridPos::new(8, 3), 30)
                            .map_err(|error| error.to_string())?
                            .with_ai(AiProfile::skirmisher(12, 0, 9)),
                    )
                    .map_err(|error| error.to_string())?;
                game.drain_events();
                app.terminal = TerminalView::new(
                    crate::test_sector::SectorDecor::default(),
                    game.map(),
                    game.player_visibility(),
                );
                app.game = WorldState::single(game);
                app.actor_glyphs.clear();
                app.actor_glyphs.insert(target, 'd');
                app.selected_target = Some(target);
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                if app.execute_command(GameCommand::Attack { slot: 2, target })
                    != CommandOutcome::Applied
                {
                    return Err("Burning status diagnostic attack rejected".into());
                }
                // The persistent mechanical state remains after its transient
                // attack/status cues have finished.
                app.capture_events_at(Some(get_time() - 5.0));
                if app.game.actors().get(target).map(Actor::position) != Some(GridPos::new(9, 3))
                    || app.burning_status_icon_at(GridPos::new(9, 3))
                        != Some(TerminalStatusIcon::Burning)
                {
                    return Err("Burning status diagnostic has no status icon".into());
                }
            }
            "resume-error" => {
                app.suspension_path = output.join("rejected-run.json");
                let mut saved = app.suspension()?;
                saved.state ^= 1;
                saved.write(&app.suspension_path)?;
                app.open_menu(MenuScreen::ResumeSuspension);
                let rect = MenuLayout::new(app.ui_width(), app.ui_height(), 2).buttons[0];
                app.update_input(&InputFrame {
                    pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
                    viewport: Some((app.ui_width(), app.ui_height())),
                    pressed: [controls::Binding::MouseLeft].into(),
                    ..Default::default()
                });
                if app.menu_message.is_empty() || !app.suspension_path.exists() {
                    return Err(
                        "L'échec de reprise doit être visible et préserver le fichier.".to_owned(),
                    );
                }
            }
            "inventory" => app.inventory_open = true,
            "skills" => app.skills_open = true,
            _ => return Err(format!("Scène de diagnostic inconnue : {scene}")),
        }
        for _ in 0..3 {
            app.draw();
            next_frame().await;
        }
        app.draw();
        let path = output.join("cold-start.png");
        if path.exists() {
            return Err("Capture déjà présente".to_owned());
        }
        // Capture once, at the end: neither a warm-up nor a previous framebuffer
        // readback may change the texture bindings before the reproduction.
        let screenshot = crate::ui_capture::framebuffer()?;
        screenshot.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
        let mut probes = if app.menu == MenuScreen::Controls {
            vec![Rect::new(18.0, 24.0, 300.0, 38.0)]
        } else if app.menu != MenuScreen::Hidden {
            MenuLayout::new(app.ui_width(), app.ui_height(), app.menu_labels().len())
                .buttons
                .into_iter()
                .enumerate()
                .filter_map(|(index, rect)| app.menu_row_enabled(index).then_some(rect))
                .collect()
        } else if app.inventory_open || app.skills_open {
            vec![Rect::new(38.0, 37.0, 380.0, 38.0)]
        } else {
            vec![Rect::new(6.0, 10.0, 300.0, 30.0)]
        };
        if scene == "resume-error" {
            let area = MenuLayout::new(app.ui_width(), app.ui_height(), 2).resume_error();
            probes.push(Rect::new(area.x, area.y + 30.0, area.w, area.h - 30.0));
        }
        for probe in probes {
            if !crate::ui_capture::text_is_visible(
                &screenshot,
                probe,
                app.ui_scale() * screen_dpi_scale(),
            ) {
                return Err(format!(
                    "{scene} : texte illisible au démarrage à froid ({probe:?})."
                ));
            }
        }
        eprintln!("[COLD UI] {scene} : contraste OK, {}", path.display());
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub async fn capture_ui_checks(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.graphics.active.mode = WindowMode::Windowed;
        let before = suspension::fingerprint(&app.game);
        // Start exactly like gameplay. A special text warm-up used to hide a
        // texture-cache bug when menus expanded the font atlas after the world.
        for _ in 0..60 {
            app.draw();
            next_frame().await;
        }
        for (name, size, percent, menu, row, mode) in [
            (
                "pause-hover",
                [1280, 800],
                100,
                MenuScreen::Pause,
                2,
                WindowMode::Windowed,
            ),
            (
                "abandon-confirmation",
                [1280, 800],
                100,
                MenuScreen::ConfirmAbandon,
                0,
                WindowMode::Windowed,
            ),
            (
                "options-hover",
                [1280, 800],
                100,
                MenuScreen::Options,
                1,
                WindowMode::Windowed,
            ),
            (
                "graphics-small",
                [960, 540],
                150,
                MenuScreen::Graphics,
                2,
                WindowMode::Windowed,
            ),
            (
                "controls-scrolled",
                [960, 540],
                200,
                MenuScreen::Controls,
                1,
                WindowMode::Windowed,
            ),
            (
                "borderless",
                [1280, 800],
                100,
                MenuScreen::Graphics,
                0,
                WindowMode::Borderless,
            ),
            (
                "restored-window",
                [1280, 800],
                100,
                MenuScreen::Graphics,
                0,
                WindowMode::Windowed,
            ),
        ] {
            if is_quit_requested() {
                return Err("Diagnostic interrompu.".to_owned());
            }
            let previous = app.graphics.active;
            app.graphics.active.mode = mode;
            app.graphics.active.windowed_size = size;
            app.graphics.active.ui_scale_percent = percent;
            app.graphics.pending = Some((previous, app.graphics.active));
            app.apply_graphics_window_change();
            // Resize requests are asynchronous. Require a usable, stable viewport
            // instead of mistaking a transient/minimized 1x1 buffer for success.
            let deadline = get_time() + 3.0;
            let mut stable_frames = 0;
            loop {
                clear_background(BLACK);
                next_frame().await;
                let ready = screen_width() >= 640.0
                    && screen_height() >= 480.0
                    && (mode == WindowMode::Borderless
                        || ((screen_width() - size[0] as f32).abs() < 1.0
                            && (screen_height() - size[1] as f32).abs() < 1.0));
                stable_frames = if ready { stable_frames + 1 } else { 0 };
                if stable_frames >= 3 {
                    break;
                }
                if is_quit_requested() || get_time() >= deadline {
                    return Err(format!(
                        "{name} : fenêtre non disponible ou taille non appliquée ({} × {}).",
                        screen_width(),
                        screen_height()
                    ));
                }
            }
            app.open_menu(menu);
            let rect = if menu == MenuScreen::Controls {
                ControlsLayout::new(app.ui_width(), app.ui_height(), 0, Action::ALL.len() + 1)
                    .row(row)
                    .unwrap()
            } else {
                MenuLayout::new(app.ui_width(), app.ui_height(), menu.buttons().len()).buttons[row]
            };
            app.update_input(&InputFrame {
                pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
                viewport: Some((app.ui_width(), app.ui_height())),
                wheel_y: if menu == MenuScreen::Controls {
                    -10.0
                } else {
                    0.0
                },
                ..Default::default()
            });
            // New labels/sizes can grow Macroquad's glyph atlas on the first
            // draw. Capture the settled frame, as seen after normal frame updates.
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let screenshot = crate::ui_capture::framebuffer()?;
            if !crate::ui_capture::text_is_visible(
                &screenshot,
                rect,
                app.ui_scale() * screen_dpi_scale(),
            ) {
                return Err(format!(
                    "{name} : le texte du bouton n'est pas lisible dans la capture."
                ));
            }
            let path = output.join(format!("{name}.png"));
            if path.exists() {
                return Err(format!("Capture déjà présente : {}", path.display()));
            }
            screenshot.export_png(path.to_str().ok_or("Chemin de capture non UTF-8")?);
            eprintln!(
                "[UI CHECK] {name}: window {}x{}, GUI {:.0} %, frame {}x{}",
                screen_width(),
                screen_height(),
                app.ui_scale() * 100.0,
                screenshot.width,
                screenshot.height
            );
            next_frame().await;
        }
        if suspension::fingerprint(&app.game) != before {
            return Err("La simulation a été modifiée.".to_owned());
        }
        app.open_menu(MenuScreen::Hidden);
        for (name, destination, interaction) in [
            ("ville-place", GridPos::new(27, 23), None),
            ("ville-porte-fermee", GridPos::new(32, 16), None),
            (
                "ville-porte-ouverte",
                GridPos::new(32, 16),
                Some(GridPos::new(32, 15)),
            ),
            ("ville-archives-verrouillees", GridPos::new(52, 16), None),
            (
                "ville-console",
                GridPos::new(50, 18),
                Some(TestSector::CONTROL),
            ),
            (
                "ville-archives-ouvertes",
                GridPos::new(52, 16),
                Some(TestSector::LOCKED_DOOR),
            ),
            ("ville-sortie", GridPos::new(61, 21), None),
            ("exterieur", GridPos::new(65, 21), None),
        ] {
            app.walk_fixture_to(destination)?;
            if let Some(target) = interaction {
                if let CommandOutcome::Rejected(error) =
                    app.execute_command(GameCommand::Interact { target })
                {
                    return Err(format!("Interaction de contrôle refusée : {error:?}"));
                }
                app.capture_events_at(Some(0.0));
            }
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let path = output.join(format!("{name}.png"));
            if path.exists() {
                return Err("Capture déjà présente".to_owned());
            }
            crate::ui_capture::framebuffer()?.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
            eprintln!(
                "[WORLD CHECK] {name}: {:?}, tour {}",
                app.game.player_position(),
                app.game.turn()
            );
            next_frame().await;
        }
        for _ in 0..2 {
            app.terminal.draw_debug_overview(&app.game);
            next_frame().await;
        }
        app.terminal.draw_debug_overview(&app.game);
        let path = output.join("plan-diagnostic-ville-exterieur.png");
        if path.exists() {
            return Err("Capture déjà présente".to_owned());
        }
        crate::ui_capture::framebuffer()?.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
        eprintln!(
            "[UI CHECK] OK: 7 menus sans mutation + 8 étapes jouées + 1 plan de diagnostic. Aucun réglage ni suspension utilisateur touché."
        );
        Ok(())
    }

    fn tick_graphics(&mut self, now: f64) -> bool {
        let expired = self.graphics.tick(now);
        if expired {
            self.open_menu(MenuScreen::Graphics);
        }
        expired
    }

    /// Diagnostic/test traversal uses real commands and opens ordinary doors;
    /// it never teleports, reveals the map or touches a user's files.
    #[cfg(any(debug_assertions, test))]
    fn walk_expedition_fixture(&mut self, stage: u8) -> Result<(), String> {
        self.walk_fixture_to(GridPos::new(66, 21))?;
        let source = GridPos::new(67, 21);
        let link = self
            .game
            .passage(source)
            .cloned()
            .ok_or("Missing expedition passage")?;
        if self.execute_command(GameCommand::Interact { target: source }) != CommandOutcome::Applied
        {
            return Err("Outbound travel rejected".into());
        }
        self.capture_events_at(Some(0.0));
        let loot = self
            .game
            .ground_items()
            .iter()
            .next()
            .map(|(_, stack)| stack.position())
            .ok_or("Missing destination loot")?;
        self.walk_fixture_to(loot)?;
        if self.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
            return Err("Pickup failed".into());
        }
        self.capture_events_at(Some(0.0));
        if stage >= 1 {
            self.walk_fixture_to(link.arrival)?;
            let outcome = self.execute_command(GameCommand::Interact {
                target: link.arrival,
            });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Return failed: {outcome:?}"));
            }
            self.capture_events_at(Some(0.0));
        }
        if stage >= 2 {
            let outcome = self.execute_command(GameCommand::Interact { target: source });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Revisit failed: {outcome:?}"));
            }
            self.capture_events_at(Some(0.0));
            if self.game.ground_items().item_at(loot).is_some() {
                return Err("Loot respawned on revisit".into());
            }
        }
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    fn walk_fixture_to(&mut self, goal: GridPos) -> Result<(), String> {
        for _ in 0..300 {
            let origin = self.game.player_position().ok_or("Joueur absent")?;
            if origin == goal {
                return Ok(());
            }
            let mut navigation = self.game.map().clone();
            for y in 0..navigation.height() as i32 {
                for x in 0..navigation.width() as i32 {
                    let p = GridPos::new(x, y);
                    if navigation.tile(p).is_some_and(|t| {
                        t.terrain == Terrain::Door(project_rl::world::DoorState::Closed)
                    }) {
                        navigation
                            .set_terrain(p, Terrain::Floor)
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
            if project_rl::world::find_path(&navigation, origin, goal, 15000, |p| {
                Some(p) != self.game.exit()
            })
            .is_none()
            {
                return Err("Trajet de contrôle introuvable".to_owned());
            }
            let Some(path) = project_rl::world::find_path(&navigation, origin, goal, 15000, |p| {
                self.game.actors().entity_at(p).is_none() && Some(p) != self.game.exit()
            }) else {
                // A worker can temporarily occupy the only doorway. Diagnostic
                // traversal waits through the real turn loop instead of
                // teleporting or declaring the static route invalid.
                if self.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                    return Err("Attente de contrôle refusée".to_owned());
                }
                self.capture_events_at(Some(0.0));
                continue;
            };
            let next = *path.get(1).ok_or("Trajet de contrôle vide")?;
            let command = if self.game.map().is_walkable(next) {
                GameCommand::Move(
                    Direction::from_delta(next.x - origin.x, next.y - origin.y).unwrap(),
                )
            } else {
                GameCommand::Interact { target: next }
            };
            if let CommandOutcome::Rejected(error) = self.execute_command(command) {
                return Err(format!("Trajet refusé : {error:?}"));
            }
            self.capture_events_at(Some(0.0));
        }
        Err("Trajet de contrôle trop long".to_owned())
    }

    fn update_input(&mut self, input: &InputFrame) {
        // A successful save freezes the snapshot until the application exits.
        // In particular, input captured alongside an OS close cannot take a turn.
        if self.quit_requested {
            return;
        }
        self.menu_focus.begin_frame(input.pointer);
        if input.pause {
            if self.legend_open {
                self.legend_open = false;
            } else if self.attack_aim.take().is_some() {
                self.attack_aim_pointer = None;
                self.push_log("VISÉE ANNULÉE".to_owned());
            } else if self.rebinding {
                self.rebinding = false;
                self.options_message = "Réattribution annulée.".to_owned();
            } else {
                self.open_menu(self.menu.back());
            }
            return;
        }
        if self.menu == MenuScreen::Controls {
            self.update_options(input);
            return;
        }
        if self.menu != MenuScreen::Hidden {
            self.update_menu(input);
            return;
        }
        if self.controls.pressed(Action::Legend, input) {
            self.legend_open = !self.legend_open;
            if self.legend_open {
                self.attack_aim = None;
                self.attack_aim_pointer = None;
                self.inventory_open = false;
                self.skills_open = false;
                self.report_open = false;
            }
            return;
        }
        if self.legend_open {
            return;
        }
        if self.report_open {
            if self.controls.pressed(Action::Report, input) {
                self.report_open = false;
            } else if self.controls.pressed(Action::MenuDown, input) {
                self.report_scroll =
                    (self.report_scroll + 1).min(self.observation_report.len().saturating_sub(1));
            } else if self.controls.pressed(Action::MenuUp, input) {
                self.report_scroll = self.report_scroll.saturating_sub(1);
            } else if input.wheel_y > 0.0 {
                self.report_scroll = self
                    .report_scroll
                    .saturating_sub(wheel_steps(input.wheel_y));
            } else if input.wheel_y < 0.0 {
                self.report_scroll = self
                    .report_scroll
                    .saturating_add(wheel_steps(input.wheel_y))
                    .min(self.observation_report.len().saturating_sub(1));
            }
            return;
        }
        if self.controls.pressed(Action::Report, input) {
            if self.observation_report.is_empty() {
                self.push_log(
                    "Aucun relevé disponible : utiliser d'abord une technique apprise.".to_owned(),
                );
            } else {
                self.attack_aim = None;
                self.attack_aim_pointer = None;
                self.report_open = true;
                self.report_scroll = 0;
            }
            return;
        }
        if self.controls.pressed(Action::Skills, input) {
            self.skills_open = !self.skills_open;
            if self.skills_open {
                self.attack_aim = None;
                self.attack_aim_pointer = None;
            }
            self.inventory_open = false;
            self.clamp_skill_selection();
            if self.skills_open {
                self.skill_message =
                    "Sélectionner une technique pour lire sa description.".to_owned();
            }
            return;
        }
        if self.controls.pressed(Action::Inventory, input) {
            self.inventory_open = !self.inventory_open;
            if self.inventory_open {
                self.attack_aim = None;
                self.attack_aim_pointer = None;
            }
            self.skills_open = false;
            self.clamp_inventory_selection();
            if self.inventory_open {
                self.inventory_message =
                    "Sélectionner un objet pour l'équiper ou l'utiliser.".to_owned();
            }
            return;
        }

        if self.inventory_open {
            self.update_inventory(input);
            return;
        }
        if self.skills_open {
            self.update_skills(input);
            return;
        }

        if self.controls.pressed(Action::Restart, input) {
            let next_seed = self.seed.wrapping_add(1);
            match Self::from_seed(
                next_seed,
                self.rules.clone(),
                self.texts.clone(),
                self.loot.clone(),
                self.expeditions.clone(),
            ) {
                Ok(mut next_run) => {
                    next_run.controls = self.controls.clone();
                    next_run.controls_path = self.controls_path.clone();
                    next_run.options_message = self.options_message.clone();
                    next_run.graphics = self.graphics.clone();
                    next_run.suspension_path = self.suspension_path.clone();
                    next_run.session_lock = self.session_lock.take();
                    *self = next_run;
                }
                Err(error) => self.push_log(format!("RESTART ERROR: {error}")),
            }
            return;
        }

        if self.game.status() != RunStatus::Active {
            return;
        }

        if self.attack_aim.is_some() {
            self.update_attack_aim(input);
            return;
        }

        if input.pressed.contains(&controls::Binding::MouseLeft)
            && let Some(target) = self
                .attack_pointer_cell(input)
                .filter(|position| self.game.player_visibility().is_visible(*position))
            && self.begin_pointer_attack_aim(target, input.pointer)
        {
            return;
        }

        if let Some(slot) = pressed_weapon_slot(&self.controls, input) {
            self.select_weapon_slot(slot);
            return;
        }

        if self.controls.pressed(Action::CycleTarget, input) {
            self.cycle_target();
            return;
        }

        for (action, direction) in [
            (Action::MoveNorth, Direction::North),
            (Action::MoveEast, Direction::East),
            (Action::MoveSouth, Direction::South),
            (Action::MoveWest, Direction::West),
        ] {
            if self.controls.pressed(action, input) {
                self.facing = direction;
                break;
            }
        }
        let command = if self.controls.pressed(Action::Interact, input) {
            self.interaction_command()
        } else if self.controls.pressed(Action::PickUp, input) {
            Some(GameCommand::PickUp)
        } else if self.controls.pressed(Action::Analyze, input) {
            self.target_analysis_command()
        } else if self.controls.pressed(Action::Traces, input) {
            self.trace_reading_command()
        } else if self.controls.pressed(Action::Walls, input) {
            self.wall_analysis_command()
        } else if self.controls.pressed(Action::Threat, input) {
            self.threat_analysis_command()
        } else if self.controls.pressed(Action::Multiple, input) {
            technique_id("core:rec_09").and_then(|id| self.technique_command(id))
        } else if self.controls.pressed(Action::Corrosion, input) {
            self.corrosion_command()
        } else if self.controls.pressed(Action::Pulse, input) {
            self.ability_command()
        } else if self.controls.pressed(Action::Attack, input) {
            self.weapon_command(input.pointer)
        } else {
            read_movement_command(&self.game, self.active_weapon_slot, &self.controls, input)
        };
        if let Some(command) = command {
            let outcome = self.execute_command(command);
            if let CommandOutcome::Rejected(reason) = outcome {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            self.capture_events();
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(5, 8, 12, 255));

        let resolved_attack_preview = self
            .attack_aim
            .map(|aim| self.game.player_attack_preview(aim.slot, aim.cursor));
        let resolved_attack_footprint = self
            .attack_aim
            .map(|aim| self.game.player_attack_footprint(aim.slot, aim.cursor));
        let attack_preview = self.attack_aim.map(|aim| TerminalAttackPreview {
            cells: resolved_attack_footprint
                .as_ref()
                .and_then(|preview| preview.as_ref().ok())
                .map_or(&[], |preview| preview.cells()),
            cursor: aim.cursor,
            valid: resolved_attack_preview.as_ref().is_some_and(Result::is_ok),
        });
        self.terminal.draw(
            &self.game,
            TerminalDrawOptions {
                bounds: self.terminal_bounds(),
                cell_size: self.graphics.active.world_cell_px,
                interact_label: &self.controls.label(Action::Interact),
                legend_label: &self.controls.label(Action::Legend),
                legend_open: self.legend_open,
                attack_preview,
            },
            |position| {
                self.glyph_at(position)
                    .map(|(glyph, color, accent_color, highlight_color)| {
                        let alert = self.visible_alert_at(position).map(|(kind, _)| kind);
                        TerminalOverlay {
                            symbol: glyph,
                            color: if alert.is_some() {
                                Color::from_rgba(255, 175, 83, 255)
                            } else {
                                color
                            },
                            accent_color,
                            highlight_color,
                            selected: self.is_selected_target_at(position),
                            alert,
                            status_icon: self.burning_status_icon_at(position),
                        }
                    })
            },
        );

        set_camera(&graphics::ui_camera(self.ui_width(), self.ui_height()));
        self.draw_header();
        self.draw_footer();
        self.draw_end_message();
        if self.inventory_open {
            self.draw_inventory();
        }
        if self.skills_open {
            self.draw_skills();
        }
        if self.report_open {
            self.draw_observation_report();
        }
        if self.menu == MenuScreen::Controls {
            self.draw_options();
        } else if self.menu != MenuScreen::Hidden {
            self.draw_menu();
        }
        set_default_camera();
    }

    fn ui_scale(&self) -> f32 {
        self.graphics
            .active
            .ui_scale(screen_width(), screen_height())
    }

    fn interaction_command(&mut self) -> Option<GameCommand> {
        let origin = self.game.player_position()?;
        if self.game.passage(origin).is_some() {
            return Some(GameCommand::Interact { target: origin });
        }
        let candidates: Vec<_> = origin
            .cardinal_neighbors()
            .into_iter()
            .filter(|p| {
                self.game.player_visibility().is_visible(*p)
                    && (self.game.passage(*p).is_some()
                        || self
                            .game
                            .active_facility()
                            .is_some_and(|facility| facility.is_depot_at(*p))
                        || self
                            .game
                            .map()
                            .tile(*p)
                            .is_some_and(|t| t.terrain.is_interactive()))
            })
            .collect();
        let target = if candidates.contains(&origin.step(self.facing)) {
            Some(origin.step(self.facing))
        } else if candidates.len() == 1 {
            candidates.first().copied()
        } else {
            None
        };
        if let Some(target) = target {
            Some(GameCommand::Interact { target })
        } else {
            self.push_log(
                if candidates.is_empty() {
                    "Approchez-vous d'une porte, d'une console, d'une installation ou d'un passage."
                } else {
                    "Plusieurs interactions : faites face à celle souhaitée avec une direction."
                }
                .to_owned(),
            );
            None
        }
    }

    fn ui_width(&self) -> f32 {
        screen_width() / self.ui_scale()
    }
    fn ui_height(&self) -> f32 {
        screen_height() / self.ui_scale()
    }

    fn terminal_bounds(&self) -> Rect {
        let ui_scale = self.ui_scale();
        Rect::new(
            20.0,
            76.0 * ui_scale,
            screen_width() - 40.0,
            (screen_height() - 180.0 * ui_scale).max(100.0),
        )
    }

    fn from_seed(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
    ) -> Result<Self, String> {
        Self::from_seed_version(
            seed,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )
    }

    fn from_seed_version(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
        version: u8,
    ) -> Result<Self, String> {
        let rules = rules_for_generation_version(rules, version);
        let mut app = if version >= 5 {
            Self::from_seed_base(seed, rules, texts, loot, expeditions)?
        } else {
            Self::from_seed_legacy(seed, rules, texts, loot, expeditions)?
        };
        app.generation_version = version;
        app.enable_expedition()?;
        if version >= 5 {
            app.enable_maintenance_fixture()?;
        }
        Ok(app)
    }

    fn enable_expedition(&mut self) -> Result<(), String> {
        let expedition_id = "core:starter_expedition"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let definition = self
            .expeditions
            .get(&expedition_id)
            .ok_or("Définition d'expédition de départ absente.")?
            .clone();
        if self.generation_version >= 7 {
            for owner in definition.player_property_take_authorizations() {
                self.game
                    .grant_player_property_take_authorization(owner.clone());
            }
        }
        let presentation = crate::test_expedition::attach(
            &mut self.game,
            self.seed,
            (self.generation_version >= 3).then_some(&self.loot),
            &definition,
        )?;
        self.terminal.decor.cells.insert(
            presentation.source_passage,
            crate::test_sector::Decor::Passage,
        );
        self.zone_decor = presentation.decor;
        self.refresh_zone_title();
        self.terminal
            .observe(self.game.map(), self.game.player_visibility());
        Ok(())
    }

    fn enable_maintenance_fixture(&mut self) -> Result<(), String> {
        let expedition_id = "core:starter_expedition"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let definition = self
            .expeditions
            .get(&expedition_id)
            .ok_or("Définition d'expédition de départ absente.")?;
        let mut facility = definition
            .hub_facility
            .clone()
            .ok_or("Circuit de maintenance de départ absent.")?;
        if self.generation_version < 6 {
            facility.blueprint.owner = None;
            for worker in &mut facility.blueprint.workers {
                worker.affiliation = None;
                worker.witness_profile = None;
                worker.local_alert_profile = None;
            }
            for material in &mut facility.materials {
                material.owner = None;
            }
        } else if self.generation_version < 7 {
            for worker in &mut facility.blueprint.workers {
                worker.local_alert_profile = None;
            }
        }
        if self.generation_version < 8 {
            for installation in &mut facility.blueprint.installations {
                installation.security_alarm_profile = None;
            }
        } else if self.generation_version < 9 {
            for installation in &mut facility.blueprint.installations {
                installation.security_alarm_profile = installation
                    .security_alarm_profile
                    .as_ref()
                    .map(|profile| profile.without_responses());
            }
        }
        for worker in &facility.blueprint.workers {
            let mut actor = Actor::new(worker.actor_position, worker.maximum_integrity)
                .map_err(|error| error.to_string())?
                .with_ai(AiProfile::idle());
            if let Some(affiliation) = &worker.affiliation {
                actor = actor.with_affiliation(affiliation.clone());
            }
            if let Some(profile) = worker.witness_profile {
                actor = actor.with_witness_profile(profile);
            }
            if let Some(profile) = worker.local_alert_profile {
                actor = actor.with_local_alert_profile(profile);
            }
            self.game
                .spawn_actor(actor)
                .map_err(|error| error.to_string())?;
        }
        for material in &facility.materials {
            self.game
                .spawn_ground_item_with_owner(
                    material.position,
                    material.item.clone(),
                    material.quantity,
                    material.owner.clone(),
                )
                .map_err(|error| error.to_string())?;
        }
        self.game
            .register_facility(definition.hub.id.clone(), facility.blueprint)?;
        self.game.drain_events();
        self.sync_facility_presentation();
        self.terminal
            .observe(self.game.map(), self.game.player_visibility());
        Ok(())
    }

    fn sync_facility_presentation(&mut self) {
        let Some(facility) = self.game.active_facility() else {
            return;
        };
        for (id, installation) in facility.installations() {
            let operational = facility.is_operational(id);
            let decor =
                if installation
                    .capabilities()
                    .contains(&InstallationCapability::Storage)
                {
                    crate::test_sector::Decor::Depot
                } else if installation
                    .capabilities()
                    .contains(&InstallationCapability::PowerRelay)
                {
                    if operational {
                        crate::test_sector::Decor::RelayOnline
                    } else {
                        crate::test_sector::Decor::RelayOffline
                    }
                } else if installation
                    .capabilities()
                    .contains(&InstallationCapability::SecuritySensor)
                {
                    if operational {
                        crate::test_sector::Decor::SensorOnline
                    } else {
                        crate::test_sector::Decor::SensorOffline
                    }
                } else if installation.capabilities().iter().any(|capability| {
                    matches!(capability, InstallationCapability::DoorActuator { .. })
                }) {
                    if operational {
                        crate::test_sector::Decor::ActuatorOnline
                    } else {
                        crate::test_sector::Decor::ActuatorOffline
                    }
                } else {
                    continue;
                };
            self.terminal
                .decor
                .cells
                .insert(installation.position(), decor);
        }
    }

    fn refresh_zone_title(&mut self) {
        if let Some(zone) = self.game.current_zone() {
            self.terminal.title = format!("{} · profondeur {}", zone.name, zone.depth);
        }
    }

    fn from_seed_legacy(
        seed: u64,
        mut rules: GameRules,
        texts: TextCatalog,
        loot_catalog: LootCatalog,
        expedition_catalog: ExpeditionCatalog,
    ) -> Result<Self, String> {
        rules.items = rules.items.without_kind(ItemKind::Material);
        Self::from_seed_base(seed, rules, texts, loot_catalog, expedition_catalog)
    }

    fn from_seed_base(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot_catalog: LootCatalog,
        expedition_catalog: ExpeditionCatalog,
    ) -> Result<Self, String> {
        let sector = TestSector::build(seed)?;
        let exit = sector.level.exit();
        let mut game = GameState::from_generated(sector.level, seed, rules.clone())
            .map_err(|error| error.to_string())?;
        let mut actor_glyphs = BTreeMap::new();

        for (index, position) in sector.enemies.iter().copied().enumerate() {
            if position == exit {
                continue;
            }
            let integrity = 5 + index as u16 * 2;
            let (attack, ai, glyph, reward) = match index % 3 {
                0 => (
                    AttackProfile::melee(DamageType::Kinetic, 3),
                    AiProfile::hunter(10, 0),
                    'd',
                    DefeatReward::persistent(5, 1),
                ),
                1 => (
                    AttackProfile::new(
                        6,
                        DistanceMetric::Euclidean,
                        true,
                        DamageType::Electrical,
                        2,
                        0,
                    ),
                    AiProfile::sentry(8, 0),
                    't',
                    DefeatReward::persistent(7, 2),
                ),
                _ => (
                    AttackProfile::new(
                        5,
                        DistanceMetric::Euclidean,
                        true,
                        DamageType::Piercing,
                        2,
                        1,
                    ),
                    AiProfile::skirmisher(9, 0, 3),
                    'r',
                    DefeatReward::persistent(8, 2),
                ),
            };
            let enemy = Actor::new(position, integrity)
                .map_err(|error| error.to_string())?
                .with_attack(attack)
                .with_ai(ai)
                .with_defeat_reward(reward);
            let id = game.spawn_actor(enemy).map_err(|error| error.to_string())?;
            actor_glyphs.insert(id, glyph);
        }

        let player_position = game.player_position();
        let loot_positions = sector
            .loot
            .iter()
            .copied()
            .filter(|position| {
                Some(*position) != player_position
                    && *position != exit
                    && game.actors().entity_at(*position).is_none()
            })
            .collect::<Vec<_>>();
        let mut loot = game
            .rules()
            .weapons
            .iter()
            .find(|(id, _)| id.namespace().as_str() != "core")
            .map(|(id, _)| (id.clone(), 1))
            .into_iter()
            .collect::<Vec<_>>();
        loot.extend(
            game.rules()
                .items
                .iter()
                .find(|(id, _)| id.as_str() == "core:repair_patch")
                .map(|(id, _)| (id.clone(), 1)),
        );
        for ((item, quantity), position) in loot.into_iter().zip(loot_positions) {
            game.spawn_ground_item(position, item, quantity)
                .map_err(|error| error.to_string())?;
        }
        game.drain_events();
        let terminal = TerminalView::new(sector.decor, game.map(), game.player_visibility());

        Ok(Self {
            game: WorldState::single(game),
            terminal,
            zone_views: BTreeMap::new(),
            zone_decor: BTreeMap::new(),
            facing: Direction::North,
            rules,
            texts,
            loot: loot_catalog,
            expeditions: expedition_catalog,
            generation_version: 1,
            seed,
            actor_glyphs,
            selected_target: None,
            attack_aim: None,
            attack_aim_pointer: None,
            visual_cues: VisualCuePlayer::with_catalog(ascii_visual_cue_catalog()?),
            trace_cells: BTreeMap::new(),
            traces_visible_until: 0.0,
            log: vec![
                "Ville de départ · extérieur généré · survolez les cases pour inspecter."
                    .to_owned(),
            ],
            active_weapon_slot: 0,
            inventory_open: false,
            inventory_selection: 0,
            inventory_message: String::new(),
            skills_open: false,
            skill_discipline_selection: 0,
            skill_technique_selection: 0,
            skill_message: String::new(),
            observation_report: Vec::new(),
            report_open: false,
            report_scroll: 0,
            legend_open: false,
            controls: Controls::preset(controls::Layout::Qwerty, KeySemantics::native()),
            controls_path: controls::config_path(),
            graphics: GraphicsState::default(),
            menu: MenuScreen::Hidden,
            menu_selection: 0,
            menu_message: String::new(),
            menu_focus: MenuFocus::default(),
            cursor_icon: miniquad::CursorIcon::Default,
            quit_requested: false,
            options_selection: 0,
            options_scroll: 0,
            rebinding: false,
            options_message: String::new(),
            history: Vec::new(),
            suspension_path: controls::config_path().with_file_name("city-test-run.json"),
            session_lock: None,
        })
    }

    fn glyph_at(&self, position: GridPos) -> Option<(char, Color, Option<Color>, Option<Color>)> {
        let visibility = self.game.player_visibility();
        if !visibility.is_explored(position) {
            return None;
        }

        if let Some(sample) =
            self.visual_cues
                .sample_world(position, visibility.is_visible(position), get_time())
        {
            return Some((
                sample.symbol,
                sample.color,
                sample.accent_color,
                sample.highlight_color,
            ));
        }

        if visibility.is_visible(position) {
            if self.game.player_position() == Some(position) {
                return Some(('@', Color::from_rgba(99, 242, 210, 255), None, None));
            }
            if self.security_alarm_remaining_at(position).is_some() {
                return Some(('s', Color::from_rgba(255, 175, 83, 255), None, None));
            }
            if let Some(entity) = self.game.actors().entity_at(position) {
                let role = self.game.active_worker_role(entity);
                let glyph = role.map_or_else(
                    || {
                        self.actor_glyphs.get(&entity).copied().unwrap_or_else(|| {
                            match self
                                .game
                                .actors()
                                .get(entity)
                                .and_then(Actor::ai)
                                .map(|ai| ai.behavior)
                            {
                                Some(project_rl::ai::AiBehavior::Sentry) => 't',
                                Some(project_rl::ai::AiBehavior::Skirmisher) => 'r',
                                _ => 'd',
                            }
                        })
                    },
                    |role| match role {
                        WorkerRole::Retriever => 'c',
                        WorkerRole::Technician => 'm',
                    },
                );
                let burning = self.game.actors().get(entity).is_some_and(|actor| {
                    actor
                        .statuses()
                        .any(|status| status.definition.as_str() == "core:burning")
                });
                let has_status = self
                    .game
                    .actors()
                    .get(entity)
                    .is_some_and(|actor| actor.statuses().next().is_some());
                let color = if role.is_some() {
                    Color::from_rgba(112, 207, 190, 255)
                } else if burning {
                    Color::from_rgba(255, 137, 48, 255)
                } else if has_status {
                    Color::from_rgba(143, 221, 107, 255)
                } else {
                    Color::from_rgba(244, 105, 90, 255)
                };
                return Some((glyph, color, None, None));
            }
            if self.game.exit() == Some(position) {
                return Some(('>', Color::from_rgba(255, 211, 92, 255), None, None));
            }
            if let Some(ground_item) = self.game.ground_items().item_at(position)
                && let Some(stack) = self.game.ground_items().get(ground_item)
            {
                if self.game.rules().weapons.get(stack.item()).is_some() {
                    return Some((')', Color::from_rgba(255, 211, 92, 255), None, None));
                }
                if self
                    .game
                    .rules()
                    .items
                    .get(stack.item())
                    .is_some_and(|item| item.kind() == ItemKind::Material)
                {
                    return Some(('=', Color::from_rgba(118, 202, 207, 255), None, None));
                }
                return Some(('!', Color::from_rgba(111, 224, 143, 255), None, None));
            }
            if let Some(effect) = self.game.ground_effects().at(position).next() {
                let palette = self.visual_cues.palette_for(effect.definition());
                return Some((
                    '^',
                    palette.color,
                    palette.accent_color,
                    palette.highlight_color,
                ));
            }
        }

        if get_time() <= self.traces_visible_until
            && visibility.is_visible(position)
            && let Some(direction) = self.trace_cells.get(&position)
        {
            let glyph = match direction {
                Direction::North => '↑',
                Direction::East => '→',
                Direction::South => '↓',
                Direction::West => '←',
            };
            return Some((glyph, Color::from_rgba(196, 158, 98, 255), None, None));
        }

        None
    }

    fn burning_status_icon_at(&self, position: GridPos) -> Option<TerminalStatusIcon> {
        self.game
            .actors()
            .entity_at(position)
            .and_then(|entity| self.game.actors().get(entity))
            .is_some_and(|actor| {
                actor
                    .statuses()
                    .any(|status| status.definition.as_str() == "core:burning")
            })
            .then_some(TerminalStatusIcon::Burning)
    }

    fn local_alert_remaining_at(&self, position: GridPos) -> Option<u64> {
        if !self.game.player_visibility().is_visible(position) {
            return None;
        }
        let entity = self.game.actors().entity_at(position)?;
        self.game
            .actors()
            .get(entity)?
            .local_alert()
            .filter(|alert| alert.is_active(self.game.turn()))
            .map(|alert| alert.remaining_turns(self.game.turn()))
    }

    fn security_alarm_remaining_at(&self, position: GridPos) -> Option<u64> {
        if !self.game.player_visibility().is_visible(position) {
            return None;
        }
        self.game
            .active_facility()?
            .security_alarm_at(position, self.game.turn())
            .map(|(_, alarm)| alarm.remaining_turns(self.game.turn()))
    }

    fn visible_alert_at(&self, position: GridPos) -> Option<(TerminalAlertKind, u64)> {
        self.security_alarm_remaining_at(position)
            .map(|remaining| (TerminalAlertKind::SecuritySystem, remaining))
            .or_else(|| {
                self.local_alert_remaining_at(position)
                    .map(|remaining| (TerminalAlertKind::LocalWitness, remaining))
            })
    }

    fn visible_local_alert_summary(&self) -> Option<(usize, u64)> {
        let mut count = 0;
        let mut longest_remaining = 0;
        self.game
            .actors()
            .iter()
            .filter(|(_, actor)| self.game.player_visibility().is_visible(actor.position()))
            .filter_map(|(_, actor)| actor.local_alert())
            .filter(|alert| alert.is_active(self.game.turn()))
            .for_each(|alert| {
                count += 1;
                longest_remaining = longest_remaining.max(alert.remaining_turns(self.game.turn()));
            });
        (count > 0).then_some((count, longest_remaining))
    }

    fn visible_security_alarm_summary(&self) -> Option<(usize, u64)> {
        let mut count = 0;
        let mut longest_remaining = 0;
        if let Some(facility) = self.game.active_facility() {
            facility
                .active_security_alarms(self.game.turn())
                .filter(|(source, _)| {
                    self.game.player_visibility().is_visible(source.position())
                        || facility
                            .active_security_door_lockdowns(self.game.turn())
                            .any(|(door, lockdown)| {
                                lockdown.installation == *source.id()
                                    && self.game.player_visibility().is_visible(door)
                            })
                })
                .for_each(|(_, alarm)| {
                    count += 1;
                    longest_remaining =
                        longest_remaining.max(alarm.remaining_turns(self.game.turn()));
                });
        }
        (count > 0).then_some((count, longest_remaining))
    }

    fn visible_alert_summary(&self) -> Option<(usize, usize, u64)> {
        let (local, local_remaining) = self.visible_local_alert_summary().unwrap_or((0, 0));
        let (security, security_remaining) =
            self.visible_security_alarm_summary().unwrap_or((0, 0));
        (local + security > 0).then_some((local, security, local_remaining.max(security_remaining)))
    }

    fn execute_command(&mut self, command: GameCommand) -> CommandOutcome {
        let recorded = RecordedCommand::record(&command);
        let previous_zone = self.game.current_zone().map(|zone| zone.id.clone());
        let outcome = self.game.process_player_command(command);
        if !matches!(outcome, CommandOutcome::Rejected(_)) {
            self.history.push(recorded);
            let current_zone = self.game.current_zone().map(|zone| zone.id.clone());
            if previous_zone != current_zone
                && let (Some(previous), Some(current)) = (previous_zone, current_zone)
            {
                let next = self.zone_views.remove(&current).unwrap_or_else(|| {
                    TerminalView::new(
                        self.zone_decor.remove(&current).unwrap_or_default(),
                        self.game.map(),
                        self.game.player_visibility(),
                    )
                });
                let old = std::mem::replace(&mut self.terminal, next);
                self.zone_views.insert(previous, old);
                self.refresh_zone_title();
                self.selected_target = None;
                self.visual_cues.clear_world();
                self.trace_cells.clear();
                self.observation_report.clear();
                self.report_open = false;
            }
            self.sync_facility_presentation();
            self.terminal
                .observe(self.game.map(), self.game.player_visibility());
        }
        outcome
    }

    fn capture_events(&mut self) {
        #[cfg(test)]
        self.capture_events_at(Some(0.0));
        #[cfg(not(test))]
        self.capture_events_at(None);
    }

    fn capture_events_at(&mut self, replay_time: Option<f64>) {
        let mut player_attack_confirmation = None;
        for event in self.game.drain_events() {
            match event {
                GameEvent::ZoneChanged { .. } => {
                    if let Some(zone) = self.game.current_zone() {
                        self.push_log(format!(
                            "{} · profondeur {}. Les autres zones continuent d'évoluer.",
                            zone.name, zone.depth
                        ));
                    }
                }
                GameEvent::Facility(event) => match event {
                    FacilityEvent::WorkerMoved { .. } => {}
                    FacilityEvent::DoorOpened { .. } => {
                        self.push_log("Un agent de maintenance ouvre une porte.".to_owned())
                    }
                    FacilityEvent::MaterialCollected { item, quantity, .. } => {
                        self.push_log(format!(
                            "Récupérateur : {} récupéré x{quantity}.",
                            self.item_name(&item)
                        ))
                    }
                    FacilityEvent::MaterialDelivered { item, quantity, .. } => self.push_log(
                        format!("Dépôt : {} livré x{quantity}.", self.item_name(&item)),
                    ),
                    FacilityEvent::PlayerMaterialDeposited { item, quantity, .. } => self.push_log(
                        format!("Dépôt : vous livrez {} x{quantity}.", self.item_name(&item)),
                    ),
                    FacilityEvent::RepairAssigned { .. } => {
                        self.push_log("Technicien : pièce réservée au dépôt.".to_owned())
                    }
                    FacilityEvent::RepairStarted { turns, .. } => self.push_log(format!(
                        "Technicien : remise en service commencée ({turns} tours)."
                    )),
                    FacilityEvent::InstallationRepaired { .. } => self.push_log(
                        "Relais réparé : porte et capteur de sécurité rétablis.".to_owned(),
                    ),
                    FacilityEvent::SecurityAlarmRaised {
                        at, duration_turns, ..
                    } => {
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:security_alarm"), at),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "ALARME DE SÉCURITÉ VISIBLE · CAPTEUR · {duration_turns} TOURS"
                        ));
                    }
                    FacilityEvent::DoorLockdownStarted {
                        door,
                        duration_turns,
                        ..
                    } => {
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:door_lockdown"), door),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "VERROUILLAGE DE SÉCURITÉ ACTIF · {duration_turns} TOURS"
                        ));
                    }
                    FacilityEvent::DoorLockdownPrevented { reason, .. } => self.push_log(
                        match reason {
                            DoorLockdownPrevention::ActuatorUnavailable => {
                                "Verrouillage de sécurité impossible : commande indisponible."
                            }
                            DoorLockdownPrevention::DoorAlreadyLocked => {
                                "Verrouillage de sécurité déjà actif sur cet accès."
                            }
                            DoorLockdownPrevention::DoorObstructed => {
                                "Verrouillage de sécurité suspendu : accès encombré."
                            }
                            DoorLockdownPrevention::NoSafeEgress => {
                                "Verrouillage de sécurité suspendu : aucune issue sûre."
                            }
                        }
                        .to_owned(),
                    ),
                    FacilityEvent::DoorLockdownEnded { door, .. } => {
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:door_lockdown"), door),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log("Verrouillage de sécurité levé.".to_owned())
                    }
                    FacilityEvent::MaterialSpilled { item, quantity, .. } => self.push_log(
                        format!("{} x{quantity} abandonné au sol.", self.item_name(&item)),
                    ),
                    FacilityEvent::WorkInterrupted { .. } => {
                        self.push_log("Travail de maintenance interrompu.".to_owned())
                    }
                    FacilityEvent::SimulationFault(error) => {
                        self.push_log(format!("Maintenance suspendue : {error}"))
                    }
                },
                GameEvent::TerrainInteracted { terrain, .. } => {
                    self.push_log(
                        match terrain {
                            Terrain::Door(project_rl::world::DoorState::Open) => "Porte ouverte.",
                            Terrain::Door(_) => "Porte fermée.",
                            Terrain::ControlPanel { .. } => {
                                "Console utilisée : accès déverrouillé."
                            }
                            _ => "Interaction effectuée.",
                        }
                        .to_owned(),
                    );
                }
                GameEvent::EntityMoved { entity, to, .. } if entity == self.game.player_id() => {
                    if let Some(ground_item) = self.game.ground_items().item_at(to)
                        && let Some(stack) = self.game.ground_items().get(ground_item)
                    {
                        self.push_log(format!(
                            "{} DÉTECTÉ{} — {} : RAMASSER",
                            display_content_name(stack.item()),
                            stack.owner().map_or("", |owner| {
                                if self.game.player_may_take_property_of(owner) {
                                    " · MATÉRIEL ATTRIBUÉ · PRISE AUTORISÉE"
                                } else {
                                    " · MATÉRIEL ATTRIBUÉ · PRISE SIGNALÉE SI OBSERVÉE"
                                }
                            }),
                            self.controls.label(Action::PickUp)
                        ));
                    }
                }
                GameEvent::AttackPerformed {
                    attacker,
                    origin,
                    target_at,
                    weapon,
                    affected_cells,
                    ..
                } => {
                    let weapon_name = weapon
                        .as_ref()
                        .map(|id| self.item_name(id))
                        .unwrap_or_else(|| "Attaque".to_owned());
                    let visibility = self.game.player_visibility();
                    let id = weapon.unwrap_or_else(|| visual_cue_id("core:generic_attack"));
                    let cue = if affected_cells.len() > 1 {
                        VisualCue::world(
                            id,
                            origin,
                            affected_cells
                                .into_iter()
                                .filter(|cell| visibility.is_visible(cell.position))
                                .map(|cell| VisualCueCell::new(cell.position, cell.step)),
                        )
                    } else if visibility.is_visible(origin) && visibility.is_visible(target_at) {
                        VisualCue::line(id, origin, target_at)
                    } else {
                        continue;
                    };
                    if let Ok(cue) = cue {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    if attacker == self.game.player_id() {
                        player_attack_confirmation = Some(format!(
                            "ATTAQUE CONFIRMÉE · {weapon_name} · ZONE {},{}",
                            target_at.x, target_at.y
                        ));
                    }
                }
                GameEvent::GroundEffectCreated { effect, at, .. }
                | GameEvent::GroundEffectTriggered { effect, at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.visual_cues.play(
                            VisualCue::point(effect, at),
                            replay_time.unwrap_or_else(get_time),
                        );
                    }
                }
                GameEvent::DamageApplied {
                    target,
                    amount,
                    damage_type,
                    ..
                } if target == self.game.player_id() => {
                    self.push_log(format!(
                        "PV -{amount}  {} DAMAGE",
                        format!("{damage_type:?}").to_uppercase()
                    ));
                }
                GameEvent::DamageApplied { amount, .. } => {
                    self.push_log(format!("TARGET DAMAGED -{amount}"));
                }
                GameEvent::EntityDied { entity } if entity != self.game.player_id() => {
                    self.actor_glyphs.remove(&entity);
                    if self.selected_target == Some(entity) {
                        self.selected_target = None;
                    }
                    self.push_log("TARGET DESTROYED".to_owned());
                }
                GameEvent::ExperienceAwarded { amount, total, .. } => {
                    self.push_log(format!("EXPERIENCE +{amount}  TOTAL {total}"));
                }
                GameEvent::LevelGained {
                    level,
                    skill_points_awarded,
                } => {
                    self.push_log(format!(
                        "CORE LEVEL {level}  SKILL POINTS +{skill_points_awarded}"
                    ));
                }
                GameEvent::TechniqueLearned {
                    technique,
                    rank,
                    skill_points_remaining,
                    ..
                } => {
                    self.push_log(format!(
                        "{} LEARNED  RANK {rank}  {skill_points_remaining} POINTS LEFT",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::TechniqueUsed {
                    technique,
                    observed_on_turn,
                    ..
                } => {
                    self.observation_report = vec![format!(
                        "{} — relevé du tour {observed_on_turn}",
                        self.technique_name(&technique)
                    )];
                    self.report_scroll = 0;
                    self.push_log(format!(
                        "Nouveau relevé disponible — {} : rapport.",
                        self.controls.label(Action::Report)
                    ));
                }
                GameEvent::EnergySpent {
                    amount, remaining, ..
                } => {
                    self.observation_report
                        .push(format!("Énergie : -{amount} E ; réserve {remaining} E."));
                }
                GameEvent::TargetAnalyzed {
                    at,
                    integrity,
                    maximum_integrity,
                    resistances,
                    ..
                } => {
                    let resistance_summary = [
                        ("KIN", DamageType::Kinetic),
                        ("PIR", DamageType::Piercing),
                        ("EXP", DamageType::Explosive),
                        ("THR", DamageType::Thermal),
                        ("ELE", DamageType::Electrical),
                        ("CHM", DamageType::Chemical),
                        ("RAD", DamageType::Radiation),
                        ("COR", DamageType::Corruption),
                    ]
                    .into_iter()
                    .filter_map(|(label, damage_type)| {
                        let value = resistances.get(damage_type);
                        (value != 0).then_some(format!("{label} {value:+}%"))
                    })
                    .collect::<Vec<_>>();
                    let resistance_summary = if resistance_summary.is_empty() {
                        "NO IDENTIFIABLE RESISTANCE".to_owned()
                    } else {
                        resistance_summary.join("  ")
                    };
                    let message = format!(
                        "Cible observée en ({}, {}) : PV {integrity}/{maximum_integrity} — {resistance_summary}",
                        at.x, at.y
                    );
                    self.observation_report.push(message.clone());
                    self.push_log(message);
                }
                GameEvent::MovementTracesRead { traces, .. } => {
                    if traces.is_empty() {
                        self.observation_report
                            .push("Aucune trace accessible dans la zone examinée.".to_owned());
                    }
                    for trace in &traces {
                        let direction = match trace.direction {
                            Direction::North => "nord",
                            Direction::East => "est",
                            Direction::South => "sud",
                            Direction::West => "ouest",
                        };
                        self.observation_report.push(format!(
                            "({}, {}) : vers le {direction}, âge au relevé : {} tours.",
                            trace.position.x, trace.position.y, trace.age_turns
                        ));
                    }
                    self.trace_cells = traces
                        .iter()
                        .map(|trace| (trace.position, trace.direction))
                        .collect();
                    self.traces_visible_until = replay_time.unwrap_or_else(get_time) + 1.5;
                    let tile_count = self.trace_cells.len();
                    let oldest = traces
                        .iter()
                        .map(|trace| trace.age_turns)
                        .max()
                        .unwrap_or(0);
                    self.push_log(format!(
                        "TRACES: {} INDICES ON {tile_count} TILES  OLDEST {oldest}T",
                        traces.len()
                    ));
                }
                GameEvent::TerrainAnalyzed { tiles, .. } => {
                    if tiles.is_empty() {
                        self.observation_report
                            .push("Aucune paroi accessible dans la zone examinée.".to_owned());
                    }
                    for tile in &tiles {
                        self.observation_report.push(format!(
                            "Paroi ({}, {}) : passage {}, visibilité {}.",
                            tile.position.x,
                            tile.position.y,
                            if tile.blocks_movement {
                                "bloqué"
                            } else {
                                "libre"
                            },
                            if tile.blocks_vision {
                                "bloquée"
                            } else {
                                "libre"
                            }
                        ));
                    }
                    self.observation_report.push(
                        "Prototype : matériau et durabilité des parois ne sont pas encore simulés."
                            .to_owned(),
                    );
                    if let Ok(cue) = VisualCue::world(
                        visual_cue_id("core:structure_scan"),
                        self.game.player_position().unwrap_or(GridPos::new(0, 0)),
                        tiles
                            .iter()
                            .map(|tile| VisualCueCell::new(tile.position, 0)),
                    ) {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    self.push_log(format!(
                        "STRUCTURE: {} CONNECTED WALL TILES — SOLID / OPAQUE",
                        tiles.len()
                    ));
                }
                GameEvent::ThreatAnalyzed { attacks, .. } => {
                    if attacks.is_empty() {
                        self.observation_report
                            .push("Aucune attaque renseignée dans ce profil.".to_owned());
                    }
                    for attack in &attacks {
                        self.observation_report.push(format!("Attaque : portée {}, dégâts {} {:?}, pénétration {} ; ligne de vue {}.", attack.range(), attack.damage().amount, attack.damage().damage_type, attack.damage().penetration, if attack.requires_line_of_sight() { "requise" } else { "non requise" }));
                    }
                    let profile = attacks.first().map_or_else(
                        || "NO OBSERVABLE ATTACK".to_owned(),
                        |attack| {
                            let damage = attack.damage();
                            format!(
                                "RANGE {}  DAMAGE {} {:?}  PEN {}",
                                attack.range(),
                                damage.amount,
                                damage.damage_type,
                                damage.penetration
                            )
                            .to_uppercase()
                        },
                    );
                    self.push_log(format!("THREAT: {profile}"));
                }
                GameEvent::StatusApplied {
                    target,
                    status,
                    stacks,
                    remaining_turns,
                    ..
                } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    let duration = remaining_turns
                        .map(|turns| format!("{turns} TURNS"))
                        .unwrap_or_else(|| "PERMANENT".to_owned());
                    self.push_log(format!(
                        "{} ON {subject} x{stacks} ({duration})",
                        display_content_name(&status),
                    ));
                    if let Some(position) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(position)
                    {
                        self.visual_cues.play(
                            VisualCue::point(status, position),
                            replay_time.unwrap_or_else(get_time),
                        );
                    }
                }
                GameEvent::StatusTriggered { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    self.push_log(format!(
                        "{} TRIGGERS ON {subject}",
                        display_content_name(&status)
                    ));
                }
                GameEvent::StatusRemoved { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    self.push_log(format!(
                        "{} EXPIRED ON {subject}",
                        display_content_name(&status)
                    ));
                }
                GameEvent::WeaponEquipped {
                    slot,
                    weapon,
                    displaced,
                    ..
                } => {
                    let displaced = displaced
                        .map(|_| " / PREVIOUS WEAPON STORED".to_owned())
                        .unwrap_or_default();
                    self.push_log(format!(
                        "CHANNEL {} <- {}{}",
                        slot + 1,
                        display_content_name(&weapon),
                        displaced
                    ));
                }
                GameEvent::ItemUsed { definition, .. } => {
                    self.push_log(format!("{} USED", display_content_name(&definition)));
                }
                GameEvent::ItemPickedUp {
                    definition,
                    quantity,
                    ..
                } => {
                    self.push_log(format!(
                        "{} ACQUIRED x{quantity}",
                        display_content_name(&definition)
                    ));
                }
                GameEvent::PropertyTakeWitnessed {
                    witness,
                    definition,
                    quantity,
                    ..
                } => {
                    let witness = match self.game.active_worker_role(witness) {
                        Some(WorkerRole::Retriever) => "Le récupérateur",
                        Some(WorkerRole::Technician) => "Le technicien",
                        None => "Un témoin",
                    };
                    self.push_log(format!(
                        "{witness} vous voit prendre {} x{quantity}, matériel attribué.",
                        self.item_name(&definition)
                    ));
                }
                GameEvent::LocalAlertRaised {
                    source,
                    at,
                    duration_turns,
                    ..
                } => {
                    self.visual_cues.play(
                        VisualCue::point(visual_cue_id("core:local_alert"), at),
                        replay_time.unwrap_or_else(get_time),
                    );
                    let source = match self.game.active_worker_role(source) {
                        Some(WorkerRole::Retriever) => "RÉCUPÉRATEUR",
                        Some(WorkerRole::Technician) => "TECHNICIEN",
                        None => "TÉMOIN",
                    };
                    self.push_log(format!(
                        "ALERTE LOCALE VISIBLE · {source} · {duration_turns} TOURS"
                    ));
                }
                GameEvent::ItemDropped {
                    definition,
                    quantity,
                    ..
                } => {
                    self.push_log(format!(
                        "{} DROPPED x{quantity}",
                        display_content_name(&definition)
                    ));
                }
                GameEvent::IntegrityRestored { amount, .. } => {
                    self.push_log(format!("PV +{amount}"));
                }
                GameEvent::ExitReached { .. } => {
                    self.push_log("EXIT REACHED — SIMULATION LAYER CLEARED".to_owned());
                }
                GameEvent::PropagationResolved { origin, cells, .. } => {
                    if let Ok(cue) = VisualCue::from_propagation(
                        visual_cue_id("core:radial_damage"),
                        origin,
                        &cells,
                    ) {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    self.push_log("RADIAL PULSE RELEASED".to_owned());
                }
                _ => {}
            }
        }
        if let Some(message) = player_attack_confirmation {
            self.push_log(message);
        }
    }

    fn is_selected_target_at(&self, position: GridPos) -> bool {
        self.selected_target.is_some_and(|target| {
            self.game
                .actors()
                .get(target)
                .is_some_and(|actor| actor.position() == position)
        })
    }

    fn push_log(&mut self, message: String) {
        self.log.push(message);
        if self.log.len() > LOG_CAPACITY {
            self.log.remove(0);
        }
    }

    fn cycle_target(&mut self) {
        let targets = self.visible_targets();
        if targets.is_empty() {
            self.selected_target = None;
            self.push_log("NO VISIBLE TARGET".to_owned());
            return;
        }

        let next_index = self
            .selected_target
            .and_then(|selected| targets.iter().position(|target| *target == selected))
            .map_or(0, |index| (index + 1) % targets.len());
        self.selected_target = targets.get(next_index).copied();
        if self.selected_target.is_some() {
            self.push_log("TARGET LOCKED".to_owned());
        }
    }

    fn select_weapon_slot(&mut self, slot: u8) {
        let Some(weapon) = self.game.equipped_player_weapon(slot) else {
            self.push_log(format!("CHANNEL {} IS EMPTY", slot + 1));
            return;
        };
        let name = self.item_name(weapon.id());
        self.active_weapon_slot = slot;
        self.push_log(format!("ACTIVE [{}] {name}", slot + 1));
    }

    fn weapon_command(&mut self, pointer: Option<(f32, f32)>) -> Option<GameCommand> {
        let Some(weapon) = self.game.equipped_player_weapon(self.active_weapon_slot) else {
            self.push_log(format!(
                "ACTIVE CHANNEL {} IS EMPTY",
                self.active_weapon_slot + 1
            ));
            return None;
        };
        if !matches!(weapon.attack().area(), AttackArea::Single) {
            self.begin_attack_aim(pointer);
            return None;
        }
        let target = self.ensure_visible_target()?;
        Some(GameCommand::Attack {
            slot: self.active_weapon_slot,
            target,
        })
    }

    fn begin_attack_aim(&mut self, pointer: Option<(f32, f32)>) {
        self.begin_attack_aim_at(None, pointer);
    }

    fn begin_pointer_attack_aim(&mut self, target: GridPos, pointer: Option<(f32, f32)>) -> bool {
        let Some(weapon) = self.game.equipped_player_weapon(self.active_weapon_slot) else {
            return false;
        };
        if matches!(weapon.attack().area(), AttackArea::Single) {
            return false;
        }
        self.begin_attack_aim_at(Some(target), pointer);
        true
    }

    fn begin_attack_aim_at(&mut self, requested: Option<GridPos>, pointer: Option<(f32, f32)>) {
        let Some(origin) = self.game.player_position() else {
            return;
        };
        let slot = self.active_weapon_slot;
        let selected = self.selected_target.and_then(|target| {
            self.game
                .actors()
                .get(target)
                .map(|actor| actor.position())
                .filter(|position| self.game.player_visibility().is_visible(*position))
        });
        let range = self
            .game
            .equipped_player_weapon(slot)
            .map_or(1, |weapon| weapon.attack().range());
        let requested = requested.filter(|position| {
            *position != origin && self.game.player_visibility().is_visible(*position)
        });
        let cursor = requested
            .or_else(|| {
                selected.filter(|position| self.game.player_attack_preview(slot, *position).is_ok())
            })
            .or_else(|| {
                (1..=range).rev().find_map(|distance| {
                    let mut position = origin;
                    for _ in 0..distance {
                        position = position.step(self.facing);
                    }
                    (self.game.player_visibility().is_visible(position)
                        && self.game.player_attack_preview(slot, position).is_ok())
                    .then_some(position)
                })
            })
            .unwrap_or_else(|| origin.step(self.facing));
        self.attack_aim = Some(AttackAim { slot, cursor });
        self.attack_aim_pointer = pointer;
        self.push_log(
            "VISÉE DE ZONE · DÉPLACEMENT/CURSEUR · ATTAQUER POUR CONFIRMER · ÉCHAP POUR ANNULER"
                .to_owned(),
        );
    }

    fn update_attack_aim(&mut self, input: &InputFrame) {
        let Some(mut aim) = self.attack_aim else {
            return;
        };
        if input.pressed.contains(&controls::Binding::MouseRight) {
            self.attack_aim = None;
            self.attack_aim_pointer = None;
            self.push_log("VISÉE ANNULÉE".to_owned());
            return;
        }
        if self.controls.pressed(Action::CycleTarget, input) {
            self.cycle_target();
            if let Some(position) = self
                .selected_target
                .and_then(|target| self.game.actors().get(target).map(|actor| actor.position()))
            {
                aim.cursor = position;
            }
            self.attack_aim = Some(aim);
            self.attack_aim_pointer = input.pointer;
            return;
        }

        let mut keyboard_moved = false;
        for (action, direction) in [
            (Action::MoveNorth, Direction::North),
            (Action::MoveEast, Direction::East),
            (Action::MoveSouth, Direction::South),
            (Action::MoveWest, Direction::West),
        ] {
            if self.controls.pressed(action, input) {
                let candidate = aim.cursor.step(direction);
                if self.game.player_visibility().is_visible(candidate) {
                    aim.cursor = candidate;
                    self.facing = direction;
                }
                keyboard_moved = true;
                break;
            }
        }

        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let pointer_moved = input.pointer.is_some() && input.pointer != self.attack_aim_pointer;
        let hovered = if pointer_moved || clicked {
            self.attack_pointer_cell(input)
                .filter(|position| self.game.player_visibility().is_visible(*position))
        } else {
            None
        };
        self.attack_aim_pointer = input.pointer;
        if !keyboard_moved && let Some(position) = hovered {
            aim.cursor = position;
        }
        self.attack_aim = Some(aim);

        let keyboard_confirmed = self.controls.pressed(Action::Attack, input) && !clicked;
        let mouse_confirmed = clicked && hovered.is_some();
        if !keyboard_confirmed && !mouse_confirmed {
            return;
        }
        if let Err(reason) = self.game.player_attack_preview(aim.slot, aim.cursor) {
            self.push_log(command_rejection_message(reason).to_owned());
            return;
        }
        let outcome = self.execute_command(GameCommand::AttackAt {
            slot: aim.slot,
            target: aim.cursor,
        });
        match outcome {
            CommandOutcome::Rejected(reason) => {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.attack_aim = None;
                self.attack_aim_pointer = None;
            }
        }
        self.capture_events();
    }

    fn attack_pointer_cell(&self, input: &InputFrame) -> Option<GridPos> {
        let pointer = input.pointer?;
        let scale = self.ui_scale();
        self.terminal.hit_test(
            &self.game,
            self.terminal_bounds(),
            self.graphics.active.world_cell_px,
            (pointer.0 * scale, pointer.1 * scale),
        )
    }

    fn ability_command(&mut self) -> Option<GameCommand> {
        let target = self.ensure_visible_target()?;
        let Some(position) = self.game.actors().get(target).map(|actor| actor.position()) else {
            self.selected_target = None;
            return None;
        };
        Some(GameCommand::UseAbility {
            slot: 0,
            target: position,
        })
    }

    fn corrosion_command(&mut self) -> Option<GameCommand> {
        let target = self.ensure_visible_target()?;
        let Some(position) = self.game.actors().get(target).map(|actor| actor.position()) else {
            self.selected_target = None;
            return None;
        };
        Some(GameCommand::UseAbility {
            slot: 1,
            target: position,
        })
    }

    fn ensure_visible_target(&mut self) -> Option<EntityId> {
        let targets = self.visible_targets();
        let selected_is_visible = self
            .selected_target
            .is_some_and(|selected| targets.contains(&selected));
        if !selected_is_visible {
            self.selected_target = targets.first().copied();
        }

        let Some(target) = self.selected_target else {
            self.push_log("NO VISIBLE TARGET".to_owned());
            return None;
        };
        Some(target)
    }

    fn target_analysis_command(&mut self) -> Option<GameCommand> {
        self.technique_command(technique_id("core:rec_01")?)
    }

    fn trace_reading_command(&self) -> Option<GameCommand> {
        Some(GameCommand::UseTechnique {
            technique: technique_id("core:rec_02")?,
            targets: Vec::new(),
        })
    }

    fn wall_analysis_command(&self) -> Option<GameCommand> {
        Some(GameCommand::UseTechnique {
            technique: technique_id("core:rec_04")?,
            targets: Vec::new(),
        })
    }

    fn threat_analysis_command(&mut self) -> Option<GameCommand> {
        self.technique_command(technique_id("core:rec_05")?)
    }

    fn technique_command(&mut self, technique: TechniqueId) -> Option<GameCommand> {
        let action = self.game.rules().skills.technique(&technique)?.action()?;
        let maximum = match action {
            TechniqueAction::AnalyzeTarget { .. } | TechniqueAction::AnalyzeThreat { .. } => 1,
            TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets, ..
            } => usize::from(maximum_targets),
            _ => 0,
        };
        let targets = if maximum > 0 {
            let candidates = self.game.player_technique_targets(&technique);
            let Some(selected) = self
                .selected_target
                .filter(|id| candidates.contains(id))
                .or_else(|| candidates.first().copied())
            else {
                self.push_log("Aucune cible visible à portée de cette technique.".to_owned());
                return None;
            };
            self.selected_target = Some(selected);
            std::iter::once(selected)
                .chain(candidates.into_iter().filter(|id| *id != selected))
                .take(maximum)
                .collect()
        } else {
            Vec::new()
        };
        Some(GameCommand::UseTechnique { technique, targets })
    }

    fn visible_targets(&self) -> Vec<EntityId> {
        let Some(player_position) = self.game.player_position() else {
            return Vec::new();
        };
        let mut targets: Vec<(u32, EntityId)> = self
            .game
            .actors()
            .iter()
            .filter_map(|(entity, actor)| {
                (entity != self.game.player_id()
                    && self.game.active_worker_role(entity).is_none()
                    && self.game.player_visibility().is_visible(actor.position()))
                .then_some((grid_distance(player_position, actor.position()), entity))
            })
            .collect();
        targets.sort_unstable();
        targets.into_iter().map(|(_, entity)| entity).collect()
    }

    fn combat_channels_label(&self) -> String {
        self.game
            .rules()
            .player_weapon_slots
            .iter()
            .enumerate()
            .take(3)
            .map(|(slot, _)| {
                let weapon = self
                    .game
                    .equipped_player_weapon(slot as u8)
                    .map(|weapon| self.item_name(weapon.id()))
                    .unwrap_or_else(|| "VIDE".to_owned());
                format!(
                    "{}[{}] {weapon}",
                    if slot as u8 == self.active_weapon_slot {
                        ">"
                    } else {
                        ""
                    },
                    slot + 1
                )
            })
            .collect::<Vec<_>>()
            .join("  ·  ")
    }

    fn draw_header(&self) {
        let player_pv = self
            .game
            .actors()
            .get(self.game.player_id())
            .map(|actor| format!("{}/{}", actor.integrity(), actor.maximum_integrity()))
            .unwrap_or_else(|| "0/--".to_owned());
        let target = self.attack_aim.map_or_else(
            || {
                self.selected_target.map_or_else(
                    || "CIBLE --".to_owned(),
                    |entity| {
                        let label = match self.actor_glyphs.get(&entity) {
                            Some('d') => "HUNTER",
                            Some('t') => "SENTRY",
                            Some('r') => "SKIRMISHER",
                            _ => "HOSTILE",
                        };
                        let status = self
                            .game
                            .actors()
                            .get(entity)
                            .and_then(|actor| actor.statuses().next())
                            .map(|status| {
                                let duration = status
                                    .remaining_turns
                                    .map(|turns| format!("{turns}T"))
                                    .unwrap_or_else(|| "PERMANENT".to_owned());
                                format!(
                                    " / {} x{} {duration}",
                                    display_content_name(&status.definition),
                                    status.stacks
                                )
                            })
                            .unwrap_or_default();
                        format!("CIBLE {label}{status}")
                    },
                )
            },
            |aim| {
                let result = self.game.player_attack_preview(aim.slot, aim.cursor);
                let footprint = self.game.player_attack_footprint(aim.slot, aim.cursor);
                let affected = footprint.as_ref().map_or(0, |preview| {
                    preview
                        .cells()
                        .iter()
                        .filter(|cell| {
                            !self.game.map().is_protected(cell.position)
                                && self
                                    .game
                                    .actors()
                                    .entity_at(cell.position)
                                    .is_some_and(|entity| entity != self.game.player_id())
                        })
                        .count()
                });
                let status = match result.as_ref() {
                    Ok(_) => "VALIDE".to_owned(),
                    Err(reason) => {
                        format!("INVALIDE : {}", attack_preview_rejection_label(reason))
                    }
                };
                format!(
                    "VISÉE {},{} · {} · {} CIBLE{}",
                    aim.cursor.x,
                    aim.cursor.y,
                    status,
                    affected,
                    if affected > 1 { "S" } else { "" }
                )
            },
        );
        let progression = self.game.player_progression();
        let header = format!(
            "PROJECT RL  |  SEED {}  |  TURN {}  |  CORE L{} XP {} SP {}  |  PV {}  |  {}",
            self.seed,
            self.game.turn(),
            progression.level(),
            progression.experience(),
            progression.unspent_skill_points(),
            player_pv,
            target,
        );
        draw_text(
            &header,
            20.0,
            34.0,
            22.0,
            Color::from_rgba(99, 242, 210, 255),
        );
        let active_weapon = self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .map(|weapon| self.item_name(weapon.id()))
            .unwrap_or_else(|| "EMPTY".to_owned());
        if let Some((local_alerts, security_alarms, remaining_turns)) = self.visible_alert_summary()
        {
            let pulse = ((get_time() * 4.0).sin() * 0.5 + 0.5) as f32;
            draw_rectangle(
                12.0,
                40.0,
                self.ui_width() - 24.0,
                29.0,
                Color::from_rgba(104, 31, 17, 255),
            );
            draw_rectangle_lines(
                12.0,
                40.0,
                self.ui_width() - 24.0,
                29.0,
                2.0 + pulse,
                Color::new(1.0, 0.48 + pulse * 0.18, 0.17, 1.0),
            );
            let warning = match (local_alerts, security_alarms) {
                (local, 0) => {
                    format!("ALERTE LOCALE  ·  {local} SOURCE(S) VISIBLE(S)")
                }
                (0, security) => {
                    format!("ALARME DE SÉCURITÉ  ·  {security} SYSTÈME(S) ACTIF(S)")
                }
                (local, security) => format!(
                    "ALERTE + ALARME  ·  {local} TÉMOIN(S)  ·  {security} SYSTÈME(S) ACTIF(S)"
                ),
            };
            let alert_text = format!(
                "!  {warning}  ·  {remaining_turns}T  |  CANAL [{}] {}  ·  ÉNERGIE {}/{}",
                self.active_weapon_slot + 1,
                active_weapon,
                self.game.player_energy().available(),
                self.game.player_energy().capacity(),
            );
            let alert_text_size: u16 =
                if measure_text(&alert_text, None, 18, 1.0).width > self.ui_width() - 40.0 {
                    15
                } else {
                    18
                };
            draw_text(
                &alert_text,
                20.0,
                61.0,
                f32::from(alert_text_size),
                Color::from_rgba(255, 237, 199, 255),
            );
        } else {
            let channels = self.combat_channels_label();
            let equipment = format!(
                "TERMINAL À GLYPHES  ·  {channels}  ·  ÉNERGIE {}/{}",
                self.game.player_energy().available(),
                self.game.player_energy().capacity(),
            );
            let equipment_size =
                if measure_text(&equipment, None, 17, 1.0).width > self.ui_width() - 40.0 {
                    14.0
                } else {
                    17.0
                };
            draw_text(
                &equipment,
                20.0,
                58.0,
                equipment_size,
                Color::from_rgba(94, 126, 137, 255),
            );
        }
    }

    fn draw_footer(&self) {
        if let Some(aim) = self.attack_aim {
            let confirmation = match self.game.player_attack_preview(aim.slot, aim.cursor) {
                Ok(_) => format!(
                    "{} ou clic gauche : attaquer",
                    self.controls.label(Action::Attack)
                ),
                Err(reason) => format!(
                    "confirmation bloquée : {}",
                    attack_preview_rejection_label(&reason).to_lowercase()
                ),
            };
            let controls = format!(
                "VISÉE DE ZONE · {} {} {} {} : déplacer · souris : orienter · {confirmation} · {} : cible suivante · Échap / clic droit : annuler",
                self.controls.label(Action::MoveNorth),
                self.controls.label(Action::MoveWest),
                self.controls.label(Action::MoveSouth),
                self.controls.label(Action::MoveEast),
                self.controls.label(Action::CycleTarget),
            );
            draw_wrapped_text(
                &controls,
                20.0,
                self.ui_height() - 84.0,
                self.ui_width() - 40.0,
                2,
                16,
                Color::from_rgba(255, 190, 91, 255),
            );
            return;
        }
        let controls = format!(
            "{} · Déplacement {} {} {} {} · Canaux {} / {} / {} · {} : interagir · {} : attaquer · {} : cible · {} : inventaire · {} : compétences · {} : rapport{} · Échap : menu",
            self.controls.layout.name(),
            self.controls.label(Action::MoveNorth),
            self.controls.label(Action::MoveWest),
            self.controls.label(Action::MoveSouth),
            self.controls.label(Action::MoveEast),
            self.controls.label(Action::Slot1),
            self.controls.label(Action::Slot2),
            self.controls.label(Action::Slot3),
            self.controls.label(Action::Interact),
            self.controls.label(Action::Attack),
            self.controls.label(Action::CycleTarget),
            self.controls.label(Action::Inventory),
            self.controls.label(Action::Skills),
            self.controls.label(Action::Report),
            if self.observation_report.is_empty() {
                " (vide)"
            } else {
                " disponible"
            }
        );
        draw_wrapped_text(
            &controls,
            20.0,
            self.ui_height() - 84.0,
            self.ui_width() - 40.0,
            2,
            16,
            Color::from_rgba(156, 178, 184, 255),
        );

        for (index, message) in self.log.iter().rev().take(2).rev().enumerate() {
            draw_text(
                message,
                20.0,
                self.ui_height() - 42.0 + index as f32 * 20.0,
                16.0,
                Color::from_rgba(107, 137, 145, 255),
            );
        }
    }

    fn draw_end_message(&self) {
        let message = if self.game.status() == RunStatus::PlayerDestroyed {
            Some(format!(
                "NOYAU DÉTRUIT — {} : RECOMMENCER",
                self.controls.label(Action::Restart)
            ))
        } else if self.game.status() == RunStatus::Escaped {
            Some(format!(
                "SORTIE ATTEINTE — {} : NOUVELLE PARTIE",
                self.controls.label(Action::Restart)
            ))
        } else {
            None
        };

        if let Some(message) = message {
            let metrics = measure_text(&message, None, 28, 1.0);
            let x = (self.ui_width() - metrics.width) * 0.5;
            let y = self.ui_height() * 0.5;
            draw_rectangle(
                x - 18.0,
                y - 34.0,
                metrics.width + 36.0,
                52.0,
                Color::from_rgba(4, 8, 12, 235),
            );
            draw_text(&message, x, y, 28.0, Color::from_rgba(255, 211, 92, 255));
        }
    }

    fn clamp_inventory_selection(&mut self) {
        let count = self.game.player_inventory().len();
        self.inventory_selection = self.inventory_selection.min(count.saturating_sub(1));
    }

    fn skill_disciplines(&self) -> Vec<DisciplineId> {
        self.game
            .rules()
            .skills
            .disciplines()
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn discipline_name(&self, id: &DisciplineId) -> String {
        self.game
            .rules()
            .skills
            .discipline(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn technique_name(&self, id: &TechniqueId) -> String {
        self.game
            .rules()
            .skills
            .technique(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn item_name(&self, id: &ItemId) -> String {
        self.game
            .rules()
            .items
            .get(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .or_else(|| {
                self.game.rules().weapons.get(id).and_then(|definition| {
                    self.texts.resolve(DISPLAY_LOCALE, definition.name_key())
                })
            })
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn selected_skill_techniques(&self) -> Vec<TechniqueId> {
        let disciplines = self.skill_disciplines();
        let Some(discipline) = disciplines.get(self.skill_discipline_selection) else {
            return Vec::new();
        };
        self.game
            .rules()
            .skills
            .techniques()
            .filter(|(_, technique)| technique.discipline() == discipline)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn clamp_skill_selection(&mut self) {
        let discipline_count = self.game.rules().skills.disciplines().count();
        self.skill_discipline_selection = self
            .skill_discipline_selection
            .min(discipline_count.saturating_sub(1));
        let technique_count = self.selected_skill_techniques().len();
        self.skill_technique_selection = self
            .skill_technique_selection
            .min(technique_count.saturating_sub(1));
    }

    fn update_skills(&mut self, input: &InputFrame) {
        let discipline_count = self.game.rules().skills.disciplines().count();
        if discipline_count == 0 {
            return;
        }
        if self.controls.pressed(Action::MenuLeft, input) {
            self.skill_discipline_selection = self.skill_discipline_selection.saturating_sub(1);
            self.skill_technique_selection = 0;
            return;
        }
        if self.controls.pressed(Action::MenuRight, input) {
            self.skill_discipline_selection =
                (self.skill_discipline_selection + 1).min(discipline_count - 1);
            self.skill_technique_selection = 0;
            return;
        }

        let techniques = self.selected_skill_techniques();
        if techniques.is_empty() {
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            self.skill_technique_selection = self.skill_technique_selection.saturating_sub(1);
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.skill_technique_selection =
                (self.skill_technique_selection + 1).min(techniques.len() - 1);
            return;
        }
        if self.controls.pressed(Action::Use, input) {
            let id = techniques[self.skill_technique_selection].clone();
            if !self.game.player_skills().has_learned(&id) {
                self.skill_message = "Cette technique doit d'abord être apprise.".to_owned();
                return;
            }
            if let Some(command) = self.technique_command(id) {
                match self.execute_command(command) {
                    CommandOutcome::Applied => self.skills_open = false,
                    CommandOutcome::Rejected(reason) => {
                        self.skill_message = command_rejection_message(reason).to_owned()
                    }
                    CommandOutcome::AppliedWithoutTime => {}
                }
                self.capture_events();
            } else {
                self.skill_message = "Aucune cible visible pour cette technique.".to_owned();
            }
            return;
        }
        if !self.controls.pressed(Action::Learn, input) {
            return;
        }

        let technique = techniques[self.skill_technique_selection].clone();
        let Some(definition) = self.game.rules().skills.technique(&technique) else {
            self.skill_message = "TECHNIQUE DATA IS UNAVAILABLE".to_owned();
            return;
        };
        let discipline = definition.discipline().clone();
        let Ok(availability) = self.game.discipline_availability(&discipline) else {
            self.skill_message = "DISCIPLINE DATA IS INVALID".to_owned();
            return;
        };
        if !availability.is_open() {
            self.skill_message =
                "DISCIPLINE LOCKED — REQUIRED GAMEPLAY SYSTEMS ARE NOT READY".to_owned();
            return;
        }

        let outcome = self.execute_command(GameCommand::LearnTechnique {
            technique: technique.clone(),
        });
        match outcome {
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.skill_message = format!("{} LEARNED", self.technique_name(&technique));
            }
            CommandOutcome::Rejected(reason) => {
                self.skill_message = command_rejection_message(reason).to_owned();
            }
        }
        self.capture_events();
    }

    fn update_inventory(&mut self, input: &InputFrame) {
        let count = self.game.player_inventory().len();
        if count == 0 {
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            self.inventory_selection = self.inventory_selection.saturating_sub(1);
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.inventory_selection = (self.inventory_selection + 1).min(count - 1);
            return;
        }

        let Some(item) = self
            .game
            .player_inventory()
            .iter()
            .nth(self.inventory_selection)
            .map(|entry| entry.instance())
        else {
            return;
        };
        if self.controls.pressed(Action::Drop, input) {
            let item_name = self
                .game
                .player_inventory()
                .get(item)
                .map(|entry| display_content_name(entry.item()))
                .unwrap_or_else(|| "UNKNOWN ITEM".to_owned());
            let outcome = self.execute_command(GameCommand::DropItem { item });
            match outcome {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    self.inventory_message = format!("{item_name} DROPPED");
                }
                CommandOutcome::Rejected(reason) => {
                    self.inventory_message = command_rejection_message(reason).to_owned();
                }
            }
            self.capture_events();
            self.clamp_inventory_selection();
            return;
        }
        if self.controls.pressed(Action::Use, input) {
            let item_name = self
                .game
                .player_inventory()
                .get(item)
                .map(|entry| display_content_name(entry.item()))
                .unwrap_or_else(|| "UNKNOWN ITEM".to_owned());
            let outcome = self.execute_command(GameCommand::UseItem { item });
            match outcome {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    self.inventory_message = format!("{item_name} USED");
                }
                CommandOutcome::Rejected(reason) => {
                    self.inventory_message = command_rejection_message(reason).to_owned();
                }
            }
            self.capture_events();
            self.clamp_inventory_selection();
            return;
        }

        let Some(slot) = pressed_weapon_slot(&self.controls, input) else {
            return;
        };
        if self
            .game
            .rules()
            .player_weapon_slots
            .get(usize::from(slot))
            .and_then(|slot_id| self.game.player_equipment().equipped(slot_id))
            == Some(item)
        {
            self.active_weapon_slot = slot;
            self.inventory_message = format!("CHANNEL {} IS NOW ACTIVE", slot + 1);
            return;
        }
        let item_name = self
            .game
            .player_inventory()
            .get(item)
            .map(|entry| display_content_name(entry.item()))
            .unwrap_or_else(|| "UNKNOWN ITEM".to_owned());
        let outcome = self.execute_command(GameCommand::EquipWeapon { slot, item });
        match outcome {
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.active_weapon_slot = slot;
                self.inventory_message =
                    format!("{item_name} EQUIPPED + ACTIVE ON CHANNEL {}", slot + 1);
            }
            CommandOutcome::Rejected(reason) => {
                self.inventory_message = command_rejection_message(reason).to_owned();
                self.push_log(self.inventory_message.clone());
            }
        }
        self.capture_events();
    }

    fn draw_inventory(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (self.ui_width() - margin * 2.0).max(620.0);
        let height = (self.ui_height() - top * 2.0).max(400.0);
        let left_width = width * 0.43;
        let panel = Color::from_rgba(5, 12, 17, 248);
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 210),
        );
        draw_rectangle(margin, top, width, height, panel);
        draw_rectangle_lines(margin, top, width, height, 2.0, cyan);
        draw_line(
            margin + left_width,
            top + 58.0,
            margin + left_width,
            top + height - 52.0,
            1.0,
            muted,
        );
        draw_line(margin, top + 58.0, margin + width, top + 58.0, 1.0, muted);
        draw_line(
            margin,
            top + height - 52.0,
            margin + width,
            top + height - 52.0,
            1.0,
            muted,
        );

        draw_text(
            "INVENTORY // LOADOUT",
            margin + 20.0,
            top + 37.0,
            25.0,
            cyan,
        );
        let inventory = self.game.player_inventory();
        let capacity = format!("SLOTS {:02}/{:02}", inventory.len(), inventory.capacity());
        let capacity_width = measure_text(&capacity, None, 18, 1.0).width;
        draw_text(
            &capacity,
            margin + width - capacity_width - 20.0,
            top + 35.0,
            18.0,
            muted,
        );

        draw_text("CARGO", margin + 20.0, top + 88.0, 17.0, muted);
        let row_height = 48.0;
        let visible_rows = ((height - 220.0) / row_height).floor().max(1.0) as usize;
        let first_visible = self
            .inventory_selection
            .saturating_sub(visible_rows.saturating_sub(1));
        for (visible_index, (index, entry)) in inventory
            .iter()
            .enumerate()
            .skip(first_visible)
            .take(visible_rows)
            .enumerate()
        {
            let row_y = top + 135.0 + visible_index as f32 * row_height;
            if index == self.inventory_selection {
                draw_rectangle(
                    margin + 10.0,
                    row_y - 25.0,
                    left_width - 20.0,
                    39.0,
                    Color::from_rgba(18, 58, 63, 230),
                );
                draw_rectangle(margin + 10.0, row_y - 25.0, 3.0, 39.0, amber);
            }
            let classification = if self.game.rules().weapons.get(entry.item()).is_some() {
                format!("WEAPON  {}", self.equipped_channel_label(entry.instance()))
            } else if let Some(definition) = self.game.rules().items.get(entry.item()) {
                format!(
                    "{}{}  x{}",
                    match definition.kind() {
                        ItemKind::Consumable => "CONSUMABLE",
                        ItemKind::Material => "MATERIAL",
                    },
                    entry.owner().map_or("", |owner| {
                        if self.game.player_may_take_property_of(owner) {
                            " · ATTRIBUÉ · AUTORISÉ"
                        } else {
                            " · ATTRIBUÉ · NON AUTORISÉ"
                        }
                    }),
                    entry.quantity()
                )
            } else {
                "UNCLASSIFIED".to_owned()
            };
            let prefix = if index == self.inventory_selection {
                ">"
            } else {
                " "
            };
            draw_text(
                format!("{prefix} {}", self.item_name(entry.item())),
                margin + 19.0,
                row_y,
                20.0,
                if index == self.inventory_selection {
                    amber
                } else {
                    text
                },
            );
            draw_text(&classification, margin + 40.0, row_y + 17.0, 14.0, muted);
        }
        if first_visible > 0 {
            draw_text(
                "^ MORE",
                margin + left_width - 76.0,
                top + 88.0,
                14.0,
                amber,
            );
        }
        if first_visible + visible_rows < inventory.len() {
            draw_text(
                "v MORE",
                margin + left_width - 76.0,
                top + height - 64.0,
                14.0,
                amber,
            );
        }

        let detail_x = margin + left_width + 24.0;
        let selected = inventory.iter().nth(self.inventory_selection);
        if let Some(entry) = selected {
            let weapon = self.game.rules().weapons.get(entry.item());
            let item_definition = self.game.rules().items.get(entry.item());
            draw_text(
                self.item_name(entry.item()),
                detail_x,
                top + 94.0,
                27.0,
                amber,
            );
            if let Some(weapon) = weapon {
                draw_text(
                    self.equipped_channel_label(entry.instance()),
                    detail_x,
                    top + 120.0,
                    15.0,
                    cyan,
                );
                draw_text("COMBAT PROFILE", detail_x, top + 151.0, 16.0, muted);
                let attack = weapon.attack();
                let damage = attack.damage();
                draw_stat_line(
                    detail_x,
                    top + 188.0,
                    "DAMAGE",
                    &damage.amount.to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 218.0,
                    "TYPE",
                    &format!("{:?}", damage.damage_type).to_uppercase(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 248.0,
                    "PENETRATION",
                    &damage.penetration.to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 278.0,
                    "RANGE",
                    &attack.range().to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 308.0,
                    "LINE OF SIGHT",
                    if attack.requires_line_of_sight() {
                        "REQUIRED"
                    } else {
                        "NO"
                    },
                    text,
                    muted,
                );
                let area = match attack.area() {
                    AttackArea::Single => "SINGLE TARGET".to_owned(),
                    AttackArea::Cone(cone) => format!(
                        "LONG CONE · {}-{} WIDE",
                        1,
                        cone.maximum_half_width()
                            .saturating_mul(2)
                            .saturating_add(1)
                    ),
                };
                draw_stat_line(detail_x, top + 338.0, "AREA", &area, text, muted);
                draw_text(
                    "COMBAT CHANNELS",
                    detail_x,
                    top + height - 142.0,
                    15.0,
                    muted,
                );
                for slot in 0..self.game.rules().player_weapon_slots.len().min(3) {
                    let y = top + height - 116.0 + slot as f32 * 21.0;
                    let equipped = self
                        .game
                        .equipped_player_weapon(slot as u8)
                        .map(|weapon| self.item_name(weapon.id()))
                        .unwrap_or_else(|| "-- EMPTY --".to_owned());
                    draw_text(
                        format!(
                            "{} [{}] {equipped}",
                            if slot as u8 == self.active_weapon_slot {
                                ">"
                            } else {
                                " "
                            },
                            slot + 1
                        ),
                        detail_x,
                        y,
                        16.0,
                        if slot as u8 == self.active_weapon_slot {
                            amber
                        } else {
                            text
                        },
                    );
                }
            } else if let Some(item_definition) = item_definition {
                draw_text(
                    format!(
                        "{}{} x{}",
                        match item_definition.kind() {
                            ItemKind::Consumable => "CONSUMABLE",
                            ItemKind::Material => "MATERIAL",
                        },
                        entry.owner().map_or("", |owner| {
                            if self.game.player_may_take_property_of(owner) {
                                " · ATTRIBUÉ · AUTORISÉ"
                            } else {
                                " · ATTRIBUÉ · NON AUTORISÉ"
                            }
                        }),
                        entry.quantity()
                    ),
                    detail_x,
                    top + 120.0,
                    15.0,
                    cyan,
                );
                draw_text(
                    match item_definition.kind() {
                        ItemKind::Consumable => "CONSUMABLE",
                        ItemKind::Material => "CRAFTING MATERIAL",
                    },
                    detail_x,
                    top + 151.0,
                    16.0,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 188.0,
                    "QUANTITY",
                    &entry.quantity().to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 218.0,
                    "MAX STACK",
                    &item_definition.maximum_stack().to_string(),
                    text,
                    muted,
                );
                for (index, effect) in item_definition.effects().iter().enumerate() {
                    let ItemEffect::RestoreIntegrity { amount } = effect;
                    draw_stat_line(
                        detail_x,
                        top + 260.0 + index as f32 * 30.0,
                        "RESTORES",
                        &format!("{amount} PV"),
                        text,
                        muted,
                    );
                }
            }
        }

        draw_text(
            &self.inventory_message,
            margin + 20.0,
            top + height - 34.0,
            14.0,
            amber,
        );
        let action_hint = selected.map_or(String::new(), |entry| {
            if self.game.rules().weapons.get(entry.item()).is_some() {
                format!(
                    "{} / {} / {} : ÉQUIPER (1 TOUR)",
                    self.controls.label(Action::Slot1),
                    self.controls.label(Action::Slot2),
                    self.controls.label(Action::Slot3)
                )
            } else if self
                .game
                .rules()
                .items
                .get(entry.item())
                .is_some_and(|definition| definition.kind() == ItemKind::Material)
            {
                format!(
                    "MATÉRIAU · {} : LIVRER AU DÉPÔT DEPUIS LE MONDE",
                    self.controls.label(Action::Interact)
                )
            } else {
                format!("{} : UTILISER (1 TOUR)", self.controls.label(Action::Use))
            }
        });
        draw_wrapped_text(
            &format!(
                "{} / {} : CHOIX   {action_hint}   {} : DÉPOSER   {} : FERMER",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Drop),
                self.controls.label(Action::Inventory)
            ),
            margin + 20.0,
            top + height - 14.0,
            width - 40.0,
            1,
            14,
            cyan,
        );
    }

    fn draw_skills(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (self.ui_width() - margin * 2.0).max(620.0);
        let height = (self.ui_height() - top * 2.0).max(400.0);
        let left_width = width * 0.31;
        let panel = Color::from_rgba(5, 12, 17, 248);
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);
        let locked = Color::from_rgba(154, 98, 92, 255);

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 210),
        );
        draw_rectangle(margin, top, width, height, panel);
        draw_rectangle_lines(margin, top, width, height, 2.0, cyan);
        draw_line(margin, top + 58.0, margin + width, top + 58.0, 1.0, muted);
        draw_line(
            margin + left_width,
            top + 58.0,
            margin + left_width,
            top + height - 52.0,
            1.0,
            muted,
        );
        draw_line(
            margin,
            top + height - 52.0,
            margin + width,
            top + height - 52.0,
            1.0,
            muted,
        );

        draw_text("COMPÉTENCES", margin + 20.0, top + 37.0, 25.0, cyan);
        let points = format!(
            "{} POINTS DISPONIBLES",
            self.game.player_progression().unspent_skill_points()
        );
        let points_width = measure_text(&points, None, 18, 1.0).width;
        draw_text(
            &points,
            margin + width - points_width - 20.0,
            top + 35.0,
            18.0,
            amber,
        );

        draw_text("DISCIPLINES", margin + 20.0, top + 88.0, 17.0, muted);
        let disciplines = self.skill_disciplines();
        for (index, discipline) in disciplines.iter().enumerate() {
            let row_y = top + 132.0 + index as f32 * 58.0;
            let selected = index == self.skill_discipline_selection;
            if selected {
                draw_rectangle(
                    margin + 10.0,
                    row_y - 26.0,
                    left_width - 20.0,
                    47.0,
                    Color::from_rgba(18, 58, 63, 230),
                );
                draw_rectangle(margin + 10.0, row_y - 26.0, 3.0, 47.0, amber);
            }
            let availability = self.game.discipline_availability(discipline).ok();
            let is_open = availability.as_ref().is_some_and(|state| state.is_open());
            let rank = self.game.player_skills().rank(discipline);
            draw_text(
                format!(
                    "{} {}",
                    if selected { ">" } else { " " },
                    self.discipline_name(discipline)
                ),
                margin + 19.0,
                row_y,
                19.0,
                if selected { amber } else { text },
            );
            draw_text(
                format!(
                    "RANK {rank}/{}  {}",
                    self.game.rules().skill_progression.maximum_rank(),
                    if is_open { "OUVERTE" } else { "INDISPONIBLE" }
                ),
                margin + 40.0,
                row_y + 17.0,
                13.0,
                if is_open { cyan } else { locked },
            );
        }

        let detail_x = margin + left_width + 24.0;
        let Some(discipline) = disciplines.get(self.skill_discipline_selection) else {
            draw_text(
                "NO SKILL DISCIPLINE LOADED",
                detail_x,
                top + 105.0,
                20.0,
                locked,
            );
            return;
        };
        let availability = self.game.discipline_availability(discipline).ok();
        let discipline_open = availability.as_ref().is_some_and(|state| state.is_open());
        draw_text(
            self.discipline_name(discipline),
            detail_x,
            top + 94.0,
            27.0,
            amber,
        );
        draw_wrapped_text(
            &if discipline_open {
                format!(
                    "{} : APPRENDRE    {} : UTILISER LA SÉLECTION",
                    self.controls.label(Action::Learn),
                    self.controls.label(Action::Use)
                )
            } else {
                "DISCIPLINE INDISPONIBLE DANS CETTE VERSION".to_owned()
            },
            detail_x,
            top + 120.0,
            width - left_width - 48.0,
            1,
            14,
            if discipline_open { cyan } else { locked },
        );

        let techniques = self.selected_skill_techniques();
        if let Some(id) = techniques.get(self.skill_technique_selection)
            && let Some(definition) = self.game.rules().skills.technique(id)
        {
            let x = margin + 20.0;
            let available_width = left_width - 40.0;
            let mut y = top + 223.0;
            y = draw_wrapped_text(
                &self.technique_name(id),
                x,
                y,
                available_width,
                3,
                19,
                amber,
            );
            let description = self
                .texts
                .resolve(DISPLAY_LOCALE, definition.description_key())
                .unwrap_or("Description indisponible.");
            let max_lines = (((height - 355.0) / 21.0).floor() as usize).clamp(2, 8);
            y = draw_wrapped_text(
                description,
                x,
                y + 10.0,
                available_width,
                max_lines,
                16,
                text,
            );
            let usage = match definition.action() {
                Some(TechniqueAction::AnalyzeTarget { range }) => format!(
                    "1 tour / 0 E. Une cible visible, portée {range}. Prototype : PV et résistances uniquement."
                ),
                Some(TechniqueAction::AnalyzeMultipleTargets {
                    maximum_targets,
                    energy_cost,
                }) => format!(
                    "1 tour / {energy_cost} E. Jusqu'à {maximum_targets} cibles visibles : sélection d'abord, puis les plus proches. Même analyse que le prérequis."
                ),
                Some(TechniqueAction::ReadMovementTraces { radius }) => format!(
                    "1 tour / 0 E. Rayon {radius}. Indices datés ; aucun suivi de leur auteur."
                ),
                Some(TechniqueAction::AnalyzeNearbyWalls {
                    maximum_tiles,
                    radius,
                }) => format!(
                    "1 tour / 0 E. Jusqu'à {maximum_tiles} parois liées, rayon {radius}. Prototype : passage et visibilité ; matériau et durabilité à venir."
                ),
                Some(TechniqueAction::AnalyzeThreat { range }) => format!(
                    "1 tour / 0 E. Cible visible, portée {range}. Prototype : attaques du profil uniquement ; pas de prédiction de l'IA."
                ),
                None => "Fonction indisponible dans cette version.".to_owned(),
            };
            let remaining_lines = (((top + height - 72.0 - y) / 20.0).floor().max(0.0)) as usize;
            draw_wrapped_text(
                &usage,
                x,
                y + 10.0,
                available_width,
                remaining_lines,
                14,
                muted,
            );
        }
        let row_height = 43.0;
        let visible_rows = ((height - 250.0) / row_height).floor().max(1.0) as usize;
        let first_visible = self
            .skill_technique_selection
            .saturating_sub(visible_rows.saturating_sub(1));
        for (visible_index, (index, technique_id)) in techniques
            .iter()
            .enumerate()
            .skip(first_visible)
            .take(visible_rows)
            .enumerate()
        {
            let row_y = top + 169.0 + visible_index as f32 * row_height;
            let selected = index == self.skill_technique_selection;
            if selected {
                draw_rectangle(
                    detail_x - 8.0,
                    row_y - 25.0,
                    width - left_width - 36.0,
                    37.0,
                    Color::from_rgba(18, 58, 63, 190),
                );
            }
            let Some(technique) = self.game.rules().skills.technique(technique_id) else {
                continue;
            };
            let learned = self.game.player_skills().has_learned(technique_id);
            let available_in_version = availability
                .as_ref()
                .is_some_and(|state| state.available.contains(technique_id));
            let next_rank = self.game.player_skills().rank(discipline).saturating_add(1);
            let status = if learned {
                format!(
                    "APPRISE — {} POUR UTILISER",
                    self.controls.label(Action::Use)
                )
            } else if !discipline_open {
                "DISCIPLINE VERROUILLÉE".to_owned()
            } else if !available_in_version {
                "INDISPONIBLE DANS CETTE VERSION".to_owned()
            } else if technique.minimum_rank() > next_rank {
                format!("RANG {} REQUIS", technique.minimum_rank())
            } else if let Some(prerequisite) = technique.prerequisite()
                && !self.game.player_skills().has_learned(prerequisite)
            {
                format!("PRÉREQUIS : {}", self.technique_name(prerequisite))
            } else {
                self.game
                    .rules()
                    .skill_progression
                    .cost_for_rank(next_rank)
                    .map(|cost| {
                        if self.game.player_progression().unspent_skill_points() < u32::from(cost) {
                            format!("COÛT {cost} — POINTS INSUFFISANTS")
                        } else {
                            format!("DISPONIBLE — COÛT {cost}")
                        }
                    })
                    .unwrap_or_else(|| "RANG MAXIMUM".to_owned())
            };
            draw_text(
                format!(
                    "{} {}",
                    if selected { ">" } else { " " },
                    self.technique_name(technique_id)
                ),
                detail_x,
                row_y,
                18.0,
                if selected { amber } else { text },
            );
            draw_text(
                format!(
                    "{}  R{}  {:?}  {status}",
                    technical_reference(technique_id),
                    technique.minimum_rank(),
                    technique.kind()
                )
                .to_uppercase(),
                detail_x + 24.0,
                row_y + 16.0,
                12.0,
                if learned || (discipline_open && available_in_version) {
                    cyan
                } else {
                    locked
                },
            );
        }

        draw_text(
            &self.skill_message,
            margin + 20.0,
            top + height - 34.0,
            14.0,
            amber,
        );
        draw_wrapped_text(
            &format!(
                "{} / {} : DISCIPLINE   {} / {} : CHOIX   {} : APPRENDRE   {} : UTILISER   {} : FERMER",
                self.controls.label(Action::MenuLeft),
                self.controls.label(Action::MenuRight),
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Learn),
                self.controls.label(Action::Use),
                self.controls.label(Action::Skills)
            ),
            margin + 20.0,
            top + height - 14.0,
            width - 40.0,
            1,
            14,
            cyan,
        );
    }

    fn equipped_channel_label(&self, item: ItemInstanceId) -> String {
        let channels = self
            .game
            .rules()
            .player_weapon_slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                (self.game.player_equipment().equipped(slot) == Some(item))
                    .then_some((index + 1).to_string())
            })
            .collect::<Vec<_>>()
            .join("/");
        if channels.is_empty() {
            "STORED".to_owned()
        } else if channels == (usize::from(self.active_weapon_slot) + 1).to_string() {
            format!("ACTIVE CHANNEL {channels}")
        } else {
            format!("CHANNEL {channels}")
        }
    }

    pub const fn should_quit(&self) -> bool {
        self.quit_requested
    }

    pub fn request_quit(&mut self) {
        if self.quit_requested {
            return;
        }
        self.graphics.revert();
        if self.menu == MenuScreen::ResumeSuspension || self.game.status() != RunStatus::Active {
            // Preserve a pending suspension, and never resurrect a finished run.
            self.quit_requested = true;
            return;
        }
        self.open_menu(MenuScreen::Pause);
        match self.suspend_run() {
            Ok(()) => self.quit_requested = true,
            Err(error) => {
                self.menu_message =
                    format!("Sauvegarde impossible : {error} La partie reste ouverte.");
            }
        }
    }

    fn open_menu(&mut self, menu: MenuScreen) {
        if menu != MenuScreen::ConfirmGraphics {
            self.graphics.revert();
        }
        if menu == MenuScreen::Graphics {
            self.graphics.draft = self.graphics.active;
        }
        self.menu = menu;
        self.menu_selection = 0;
        self.menu_message.clear();
        self.rebinding = false;
        self.menu_focus.reset();
    }

    fn update_menu(&mut self, input: &InputFrame) {
        let count = self.menu.buttons().len();
        if count == 0 {
            return;
        }
        let hovered = input
            .pointer
            .zip(input.viewport)
            .and_then(|(pointer, (width, height))| {
                MenuLayout::new(width, height, count).hit(pointer)
            })
            .filter(|index| self.menu_row_enabled(*index));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let up = self.controls.pressed(Action::MenuUp, input);
        let down = self.controls.pressed(Action::MenuDown, input);
        let activate = self.controls.pressed(Action::Learn, input) && !clicked;
        if up && !clicked {
            if let Some(index) = (0..self.menu_selection)
                .rev()
                .find(|index| self.menu_row_enabled(*index))
            {
                self.menu_selection = index;
            }
        } else if down
            && !clicked
            && let Some(index) =
                (self.menu_selection + 1..count).find(|index| self.menu_row_enabled(*index))
        {
            self.menu_selection = index;
        }
        let left = self.controls.pressed(Action::MenuLeft, input);
        let right = self.controls.pressed(Action::MenuRight, input);
        self.menu_focus.update(
            hovered,
            &mut self.menu_selection,
            clicked,
            up || down || activate || left || right,
        );
        if self.menu == MenuScreen::Graphics && (left || right) && !clicked {
            self.graphics.draft.cycle(self.menu_selection, right);
            return;
        }
        if ((clicked && hovered.is_some()) || activate)
            && self.menu_row_enabled(self.menu_selection)
        {
            match (self.menu, self.menu_selection) {
                (MenuScreen::Pause, 0) => self.open_menu(MenuScreen::Hidden),
                (MenuScreen::Pause, 1) => self.open_menu(MenuScreen::Options),
                (MenuScreen::Pause, 2) => self.request_quit(),
                (MenuScreen::Pause, 3) => self.open_menu(MenuScreen::ConfirmAbandon),
                (MenuScreen::Options, 0) => self.open_menu(MenuScreen::Controls),
                (MenuScreen::Options, 1) => self.open_menu(MenuScreen::Graphics),
                (MenuScreen::Options, 2) | (MenuScreen::ConfirmAbandon, 0) => {
                    self.open_menu(MenuScreen::Pause)
                }
                (MenuScreen::Graphics, row @ 0..=2) => self.graphics.draft.cycle(row, true),
                (MenuScreen::Graphics, 4) => self.graphics.draft.cycle(4, true),
                (MenuScreen::Graphics, 5) => self.graphics.draft = GraphicsSettings::default(),
                (MenuScreen::Graphics, 6) => {
                    if self.graphics.begin_preview() {
                        self.open_menu(MenuScreen::ConfirmGraphics);
                    }
                }
                (MenuScreen::Graphics, 7) => self.open_menu(MenuScreen::Options),
                (MenuScreen::ConfirmGraphics, 0) => self.open_menu(MenuScreen::Graphics),
                (MenuScreen::ConfirmGraphics, 1) => {
                    self.graphics.confirm();
                    self.open_menu(MenuScreen::Graphics);
                }
                (MenuScreen::ConfirmAbandon, 1) => self.quit_requested = true,
                (MenuScreen::ResumeSuspension, 0) => {
                    if let Err(error) = self.resume_run() {
                        self.menu_message = format!("Reprise impossible : {error}");
                    }
                }
                (MenuScreen::ResumeSuspension, 1) => self.quit_requested = true,
                _ => {}
            }
        }
    }

    fn menu_row_enabled(&self, index: usize) -> bool {
        if self.menu == MenuScreen::Pause && index == 3 && self.game.status() != RunStatus::Active {
            return false;
        }
        self.menu != MenuScreen::Graphics
            || (index != 3 && (index != 1 || self.graphics.draft.mode == WindowMode::Windowed))
    }

    fn menu_labels(&self) -> Vec<String> {
        if self.menu == MenuScreen::ResumeSuspension && !self.menu_message.is_empty() {
            vec!["Réessayer la reprise".to_owned(), "Quitter".to_owned()]
        } else if self.menu == MenuScreen::Pause && self.game.status() != RunStatus::Active {
            vec![
                "Reprendre".to_owned(),
                "Options".to_owned(),
                "Quitter".to_owned(),
                "Abandonner la partie (terminée)".to_owned(),
            ]
        } else if self.menu == MenuScreen::Graphics {
            let settings = self.graphics.draft;
            vec![
                format!("Mode : {}", settings.mode.label()),
                if settings.mode == WindowMode::Windowed {
                    format!(
                        "Résolution : {} × {}",
                        settings.windowed_size[0], settings.windowed_size[1]
                    )
                } else {
                    "Résolution : bureau (automatique)".to_owned()
                },
                format!("Taille de l'interface : {} %", settings.ui_scale_percent),
                "Rendu : Terminal à glyphes (textures à venir)".to_owned(),
                format!("Taille des cases : {} px", settings.world_cell_px),
                "Valeurs par défaut".to_owned(),
                "Appliquer les modifications".to_owned(),
                "Retour sans appliquer".to_owned(),
            ]
        } else {
            self.menu
                .buttons()
                .iter()
                .map(|label| (*label).to_owned())
                .collect()
        }
    }

    fn draw_menu(&self) {
        let buttons = self.menu_labels();
        let layout = MenuLayout::new(self.ui_width(), self.ui_height(), buttons.len());
        let panel = layout.panel;
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 3, 6, 210),
        );
        draw_rectangle(
            panel.x,
            panel.y,
            panel.w,
            panel.h,
            Color::from_rgba(5, 14, 20, 255),
        );
        draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 2.0, SKYBLUE);
        draw_wrapped_text(
            self.menu.title(),
            panel.x + 24.0,
            panel.y + 45.0,
            panel.w - 48.0,
            1,
            28,
            WHITE,
        );
        for (index, (label, rect)) in buttons.iter().zip(&layout.buttons).enumerate() {
            let enabled = self.menu_row_enabled(index);
            let selected = enabled && self.menu_focus.highlighted(index, self.menu_selection);
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if selected {
                    Color::from_rgba(24, 67, 75, 255)
                } else {
                    Color::from_rgba(13, 31, 40, 255)
                },
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if selected { 2.0 } else { 1.0 },
                if selected { YELLOW } else { GRAY },
            );
            if selected {
                draw_rectangle(
                    rect.x + 4.0,
                    rect.y + 7.0,
                    3.0,
                    (rect.h - 14.0).max(1.0),
                    YELLOW,
                );
            }
            draw_wrapped_text(
                label,
                rect.x + 14.0,
                rect.y + rect.h * 0.65,
                rect.w - 28.0,
                1,
                21,
                if !enabled {
                    GRAY
                } else if selected {
                    YELLOW
                } else {
                    WHITE
                },
            );
        }
        let graphics_note = if self.menu == MenuScreen::ConfirmGraphics {
            format!(
                "Retour automatique dans {} s sans confirmation. Échap rétablit les anciens réglages.",
                self.graphics.remaining_seconds()
            )
        } else if self.graphics.draft != self.graphics.active {
            "Modifications non appliquées. L'interface s'adapte si la fenêtre est trop petite."
                .to_owned()
        } else if self.ui_scale() * 100.0 + 0.1 < self.graphics.active.ui_scale_percent as f32 {
            format!(
                "{} Échelle adaptée à {:.0} % pour garder les menus accessibles.",
                self.graphics.message,
                self.ui_scale() * 100.0
            )
        } else {
            self.graphics.message.clone()
        };
        let resume_error =
            self.menu == MenuScreen::ResumeSuspension && !self.menu_message.is_empty();
        if resume_error {
            let area = layout.resume_error();
            draw_rectangle(
                area.x,
                area.y,
                area.w,
                area.h,
                Color::from_rgba(44, 24, 24, 255),
            );
            draw_rectangle_lines(
                area.x,
                area.y,
                area.w,
                area.h,
                1.0,
                Color::from_rgba(255, 180, 130, 255),
            );
            draw_text(
                "REPRISE IMPOSSIBLE",
                area.x + 14.0,
                area.y + 26.0,
                20.0,
                Color::from_rgba(255, 205, 160, 255),
            );
            draw_wrapped_text(
                self.menu_message
                    .strip_prefix("Reprise impossible : ")
                    .unwrap_or(&self.menu_message),
                area.x + 14.0,
                area.y + 53.0,
                area.w - 28.0,
                4,
                16,
                WHITE,
            );
        }
        let note = if resume_error {
            "Ta partie suspendue est conservée. Tu peux réessayer ou quitter."
        } else if !self.menu_message.is_empty() {
            self.menu_message.as_str()
        } else if matches!(
            self.menu,
            MenuScreen::Graphics | MenuScreen::ConfirmGraphics
        ) {
            graphics_note.as_str()
        } else if self.menu == MenuScreen::ConfirmAbandon {
            "La partie sera perdue, sans sauvegarde ni possibilité de reprise."
        } else if self.menu == MenuScreen::ResumeSuspension {
            "Reprise unique : la suspension sera consommée après vérification. Aucun retour à un ancien état."
        } else if self.menu == MenuScreen::Options {
            "Réglages personnels. La partie reste en pause."
        } else if self.game.status() != RunStatus::Active {
            "Partie terminée : aucune sauvegarde de reprise ne sera créée."
        } else {
            "Sauvegarder et quitter permet une reprise unique. Aucun tour ne s'écoule dans le menu."
        };
        draw_wrapped_text(
            note,
            panel.x + 24.0,
            panel.y + panel.h - 70.0,
            panel.w - 48.0,
            2,
            15,
            LIGHTGRAY,
        );
        let hint = if self.menu == MenuScreen::Graphics {
            format!(
                "{} / {} : valeur · {} ou clic : choisir · Échap : retour",
                self.controls.label(Action::MenuLeft),
                self.controls.label(Action::MenuRight),
                self.controls.label(Action::Learn)
            )
        } else {
            format!(
                "{} / {} · {} ou clic : choisir · Échap : retour",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Learn)
            )
        };
        draw_wrapped_text(
            &hint,
            panel.x + 24.0,
            panel.y + panel.h - 22.0,
            panel.w - 48.0,
            1,
            14,
            SKYBLUE,
        );
    }

    fn suspension(&self) -> Result<Suspension, String> {
        if self.game.status() != RunStatus::Active {
            return Err("Une partie terminée ne peut pas être suspendue.".to_owned());
        }
        let saved = Suspension {
            version: self.generation_version,
            build: env!("PROJECT_RL_BUILD_FINGERPRINT").to_owned(),
            rules: rules_fingerprint_for_version(&self.rules, self.generation_version),
            loot_rules: (self.generation_version >= 3).then(|| suspension::fingerprint(&self.loot)),
            world_rules: (self.generation_version >= 4).then(|| {
                if self.generation_version == 4 {
                    suspension::fingerprint(&self.expeditions.without_facilities())
                } else if self.generation_version == 5 {
                    suspension::fingerprint(&self.expeditions.without_social_metadata())
                } else if self.generation_version == 6 {
                    suspension::fingerprint(&self.expeditions.without_local_alert_metadata())
                } else if self.generation_version == 7 {
                    suspension::fingerprint(&self.expeditions.without_security_alarm_metadata())
                } else if self.generation_version == 8 {
                    suspension::fingerprint(
                        &self.expeditions.without_security_alarm_response_metadata(),
                    )
                } else {
                    suspension::fingerprint(&self.expeditions)
                }
            }),
            seed: self.seed,
            commands: self.history.clone(),
            state: if self.generation_version == 1 {
                suspension::fingerprint(self.game.active_game())
            } else {
                suspension::fingerprint(&self.game)
            },
            active_weapon_slot: self.active_weapon_slot,
            selected_target: self.selected_target.map(|id| id.get()),
            report: self.observation_report.clone(),
            log: self.log.clone(),
        };
        saved.validate()?;
        Ok(saved)
    }

    fn restore_suspension(
        saved: &Suspension,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
    ) -> Result<Self, String> {
        // Build provenance is deliberately not a gate. Even after recompiling,
        // a run is installed only when rules AND the entire replay match.
        saved.validate()?;
        if saved.rules != rules_fingerprint_for_version(&rules, saved.version) {
            return Err("Les règles ou les mods ont changé ; suspension conservée.".to_owned());
        }
        if saved.version >= 3 && saved.loot_rules != Some(suspension::fingerprint(&loot)) {
            return Err("Les tables de butin ont changé ; suspension conservée.".to_owned());
        }
        let expected_world_rules = match saved.version {
            4 => suspension::fingerprint(&expeditions.without_facilities()),
            5 => suspension::fingerprint(&expeditions.without_social_metadata()),
            6 => suspension::fingerprint(&expeditions.without_local_alert_metadata()),
            7 => suspension::fingerprint(&expeditions.without_security_alarm_metadata()),
            8 => suspension::fingerprint(&expeditions.without_security_alarm_response_metadata()),
            _ => suspension::fingerprint(&expeditions),
        };
        if saved.version >= 4 && saved.world_rules != Some(expected_world_rules) {
            return Err("Les définitions du monde ont changé ; suspension conservée.".to_owned());
        }
        let mut restored = if saved.version == 1 {
            Self::from_seed_legacy(
                saved.seed,
                rules_for_generation_version(rules, saved.version),
                texts,
                loot,
                expeditions,
            )?
        } else {
            Self::from_seed_version(saved.seed, rules, texts, loot, expeditions, saved.version)?
        };
        for (index, recorded) in saved.commands.iter().enumerate() {
            let command = recorded.command(&restored.game)?;
            if let CommandOutcome::Rejected(error) = restored.execute_command(command) {
                return Err(format!("Rejeu divergent à la commande {index} : {error:?}"));
            }
            restored.capture_events_at(Some(0.0));
        }
        let state = if saved.version == 1 {
            suspension::fingerprint(restored.game.active_game())
        } else {
            suspension::fingerprint(&restored.game)
        };
        if restored.game.status() != RunStatus::Active || state != saved.state {
            return Err(
                "L'état reconstruit ne correspond pas à la suspension ; fichier conservé."
                    .to_owned(),
            );
        }
        if usize::from(saved.active_weapon_slot) >= restored.game.rules().player_weapon_slots.len()
        {
            return Err("Canal d'équipement enregistré invalide.".to_owned());
        }
        restored.active_weapon_slot = saved.active_weapon_slot;
        if let Some(target) = saved.selected_target {
            restored.selected_target = Some(
                restored
                    .game
                    .actors()
                    .iter()
                    .map(|(id, _)| id)
                    .find(|id| id.get() == target)
                    .ok_or("Cible sélectionnée absente de l'état reconstruit.")?,
            );
        }
        restored.observation_report = saved.report.clone();
        restored.log = saved.log.clone();
        restored.trace_cells.clear();
        restored.visual_cues.clear_world();
        restored.traces_visible_until = 0.0;
        if saved.version == 1 {
            // Verify the exact old state first. Enabling zone travel changes no
            // inventory, actors or accepted history and never rewrites the file.
            restored.generation_version = 2;
            restored.enable_expedition()?;
        }
        Ok(restored)
    }

    fn suspend_run(&self) -> Result<(), String> {
        let saved = self.suspension()?;
        // Prove that the entire current state is recoverable before writing or quitting.
        Self::restore_suspension(
            &saved,
            self.rules.clone(),
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        saved.write(&self.suspension_path)
    }

    fn resume_run(&mut self) -> Result<(), String> {
        let saved = Suspension::read(&self.suspension_path)?;
        let mut restored = Self::restore_suspension(
            &saved,
            self.rules.clone(),
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        // No state is installed and nothing is consumed until every check succeeds.
        std::fs::remove_file(&self.suspension_path)
            .map_err(|error| format!("Impossible de consommer la suspension : {error}"))?;
        restored.controls = self.controls.clone();
        restored.controls_path = self.controls_path.clone();
        restored.options_message = self.options_message.clone();
        restored.graphics = self.graphics.clone();
        restored.suspension_path = self.suspension_path.clone();
        restored.session_lock = self.session_lock.take();
        restored.push_log("Partie reprise ; suspension consommée.".to_owned());
        *self = restored;
        Ok(())
    }

    fn update_options(&mut self, input: &InputFrame) {
        if self.rebinding {
            if input.pressed.len() == 1 {
                let action = Action::ALL[self.options_selection - 1];
                let mut next = self.controls.clone();
                match next.rebind(action, input.pressed.first().unwrap().clone()) {
                    Ok(()) => {
                        self.save_controls(next);
                        self.rebinding = false;
                    }
                    Err(error) => self.options_message = error,
                }
            } else if !input.pressed.is_empty() {
                self.options_message =
                    "Appuyer sur une seule touche ou un seul bouton de souris.".to_owned();
            }
            return;
        }
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let up = self.controls.pressed(Action::MenuUp, input);
        let down = self.controls.pressed(Action::MenuDown, input);
        let activate = self.controls.pressed(Action::Learn, input) && !clicked;
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let mut layout =
            ControlsLayout::new(width, height, self.options_scroll, Action::ALL.len() + 1);
        let scrolling = wheel_steps(input.wheel_y) > 0
            && input
                .pointer
                .is_some_and(|pointer| layout.bounds.contains(pointer.into()));
        if scrolling {
            layout.scroll(input.wheel_y);
            self.menu_focus.scroll();
        }
        if up && !clicked {
            self.options_selection = self.options_selection.saturating_sub(1);
        } else if down && !clicked {
            self.options_selection = (self.options_selection + 1).min(Action::ALL.len() + 1);
        }
        if up || down || activate || (self.menu_focus.keyboard_mode() && !scrolling) {
            layout.reveal(self.options_selection.min(Action::ALL.len()));
        }
        self.options_scroll = layout.first;
        let hovered = input.pointer.and_then(|pointer| {
            if layout.back.contains(pointer.into()) {
                Some(Action::ALL.len() + 1)
            } else {
                layout.hit(pointer)
            }
        });
        self.menu_focus.update(
            hovered,
            &mut self.options_selection,
            clicked,
            up || down || activate,
        );
        if (clicked && hovered.is_some()) || activate {
            if self.options_selection > Action::ALL.len() {
                self.open_menu(MenuScreen::Options);
            } else if self.options_selection == 0 {
                let mut next = self.controls.clone();
                match next.change_layout(next.layout.other()) {
                    Ok(()) => self.save_controls(next),
                    Err(error) => self.options_message = error,
                }
            } else {
                self.rebinding = true;
                self.menu_focus.reset();
                self.options_message =
                    "Nouvelle touche ou bouton de souris ; Échap pour annuler.".to_owned();
            }
        }
    }

    fn save_controls(&mut self, next: Controls) {
        match next.save(&self.controls_path) {
            Ok(()) => {
                self.controls = next;
                self.options_message = "Réglages enregistrés. Une seule touche par action ; personnalisations conservées.".to_owned();
            }
            Err(error) => {
                self.options_message =
                    format!("Enregistrement impossible, réglages inchangés : {error}")
            }
        }
    }

    fn draw_options(&self) {
        let layout = ControlsLayout::new(
            self.ui_width(),
            self.ui_height(),
            self.options_scroll,
            Action::ALL.len() + 1,
        );
        let margin = layout.bounds.x;
        let width = layout.bounds.w;
        let bottom = self.ui_height() - 125.0;
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(3, 9, 13, 255),
        );
        draw_text("OPTIONS", margin, 55.0, 30.0, WHITE);
        let back_selected = self
            .menu_focus
            .highlighted(Action::ALL.len() + 1, self.options_selection);
        let back = layout.back;
        draw_rectangle(
            back.x,
            back.y,
            back.w,
            back.h,
            if back_selected {
                Color::from_rgba(24, 67, 75, 255)
            } else {
                Color::from_rgba(13, 31, 40, 255)
            },
        );
        draw_rectangle_lines(
            back.x,
            back.y,
            back.w,
            back.h,
            if back_selected { 2.0 } else { 1.0 },
            if back_selected { YELLOW } else { GRAY },
        );
        draw_text(
            "Retour",
            back.x + 18.0,
            back.y + 22.0,
            20.0,
            if back_selected { YELLOW } else { WHITE },
        );
        draw_text("COMMANDES", margin, 87.0, 19.0, SKYBLUE);
        draw_wrapped_text(
            "Disposition et raccourcis personnels. La partie est en pause. Échap : retour aux options.",
            margin,
            115.0,
            width,
            2,
            16,
            GRAY,
        );
        for index in layout.first..layout.first + layout.visible {
            let rect = layout.row(index).expect("visible controls row");
            let y = rect.y + 21.0;
            let selected = self.menu_focus.highlighted(index, self.options_selection);
            let (name, binding) = if index == 0 {
                (
                    "Disposition du clavier",
                    self.controls.layout.name().to_owned(),
                )
            } else {
                let action = Action::ALL[index - 1];
                (action.name(), self.controls.label(action))
            };
            if selected {
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    Color::from_rgba(18, 58, 63, 255),
                );
                draw_rectangle(rect.x, rect.y + 4.0, 3.0, rect.h - 8.0, YELLOW);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, SKYBLUE);
            }
            draw_wrapped_text(
                name,
                margin + 10.0,
                y,
                width * 0.68 - 20.0,
                1,
                17,
                if selected { YELLOW } else { LIGHTGRAY },
            );
            draw_wrapped_text(
                &binding,
                margin + width * 0.7,
                y,
                width * 0.3 - 10.0,
                1,
                17,
                SKYBLUE,
            );
        }
        if layout.count > layout.visible {
            let height = layout.bounds.h;
            let thumb = (height * layout.visible as f32 / layout.count as f32).max(12.0);
            let y = layout.bounds.y
                + (height - thumb) * layout.first as f32 / (layout.count - layout.visible) as f32;
            draw_rectangle(
                layout.bounds.x + layout.bounds.w - 5.0,
                layout.bounds.y,
                4.0,
                height,
                DARKGRAY,
            );
            draw_rectangle(
                layout.bounds.x + layout.bounds.w - 5.0,
                y,
                4.0,
                thumb,
                SKYBLUE,
            );
        }
        draw_wrapped_text(
            &self.options_message,
            margin,
            bottom + 35.0,
            width,
            2,
            15,
            YELLOW,
        );
        let hint = format!(
            "{} / {} : sélectionner · molette : défiler · {} ou clic : {} · Échap : retour",
            self.controls.label(Action::MenuUp),
            self.controls.label(Action::MenuDown),
            self.controls.label(Action::Learn),
            if self.options_selection > Action::ALL.len() {
                "revenir"
            } else if self.options_selection == 0 {
                "changer de disposition"
            } else {
                "réattribuer"
            }
        );
        draw_wrapped_text(
            &hint,
            margin,
            self.ui_height() - 36.0,
            width,
            2,
            15,
            SKYBLUE,
        );
    }

    fn draw_observation_report(&self) {
        let x = 40.0;
        let width = (self.ui_width() - 80.0).max(100.0);
        let height = (self.ui_height() - 80.0).max(150.0);
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 225),
        );
        draw_rectangle(x, 40.0, width, height, Color::from_rgba(5, 12, 17, 255));
        draw_rectangle_lines(
            x,
            40.0,
            width,
            height,
            1.0,
            Color::from_rgba(99, 242, 210, 255),
        );
        draw_text("RAPPORT D'OBSERVATION", x + 20.0, 77.0, 24.0, WHITE);
        draw_wrapped_text(
            "Données datées : ce relevé ne suit pas les cibles après l'observation.",
            x + 20.0,
            106.0,
            width - 40.0,
            2,
            16,
            GRAY,
        );
        let bottom = 40.0 + height - 65.0;
        let mut y = 151.0;
        for line in self.observation_report.iter().skip(self.report_scroll) {
            let max_lines = ((bottom - y) / 24.0).floor().max(0.0) as usize;
            if max_lines == 0 {
                break;
            }
            y = draw_wrapped_text(line, x + 20.0, y, width - 40.0, max_lines, 19, LIGHTGRAY) + 10.0;
        }
        draw_wrapped_text(
            &format!(
                "{} / {} OU MOLETTE : DÉFILER   {} : FERMER / REVOIR LE RAPPORT",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Report)
            ),
            x + 20.0,
            40.0 + height - 25.0,
            width - 40.0,
            2,
            14,
            Color::from_rgba(99, 242, 210, 255),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_wrapped_text(
    text: &str,
    x: f32,
    mut y: f32,
    width: f32,
    maximum_lines: usize,
    size: u16,
    color: Color,
) -> f32 {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure_text(&candidate, None, size, 1.0).width > width {
            lines.push(std::mem::take(&mut line));
            line = word.to_owned();
        } else {
            line = candidate;
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    for (index, line) in lines.iter().take(maximum_lines).enumerate() {
        let line = if index + 1 == maximum_lines && lines.len() > maximum_lines {
            format!("{line}…")
        } else {
            line.clone()
        };
        let measured = measure_text(&line, None, size, 1.0).width;
        let font_size = f32::from(size) * (width / measured.max(1.0)).min(1.0);
        draw_text(&line, x, y, font_size, color);
        y += f32::from(size) + 5.0;
    }
    y
}

fn rules_fingerprint_for_version(rules: &GameRules, version: u8) -> u64 {
    let mut legacy = rules_for_generation_version(rules.clone(), version);
    if version >= 5 {
        suspension::fingerprint(&legacy)
    } else {
        // ItemKind::Material and its definitions entered the ruleset together
        // with generation v5. Older replays must retain their exact catalogue.
        legacy.items = legacy.items.without_kind(ItemKind::Material);
        suspension::fingerprint(&legacy)
    }
}

fn rules_for_generation_version(mut rules: GameRules, version: u8) -> GameRules {
    if version < CURRENT_GENERATION_VERSION {
        let flamethrower: WeaponId = "core:flamethrower"
            .parse()
            .expect("built-in flamethrower ID must remain valid");
        let burning: StatusId = "core:burning"
            .parse()
            .expect("built-in burning status ID must remain valid");
        if let Some(index) = rules
            .player_starting_weapons
            .iter()
            .position(|weapon| weapon == &flamethrower)
        {
            rules.player_starting_weapons.remove(index);
            if index < rules.player_base_attacks.len() {
                rules.player_base_attacks.remove(index);
            }
        }
        rules
            .player_starting_equipment
            .retain(|weapon| weapon.as_ref() != Some(&flamethrower));
        rules.weapons = rules.weapons.without_id(&flamethrower);
        rules.statuses = rules.statuses.without_id(&burning);
    }
    rules
}

fn ascii_game_content() -> Result<(GameRules, TextCatalog, LootCatalog, ExpeditionCatalog), String>
{
    let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let loaded = ContentLoader::load(
        &[project_root.join("content"), project_root.join("mods")],
        &semver::Version::new(0, 1, 0),
    )
    .map_err(|error| error.to_string())?;
    let corrosion_id: StatusId = "core:corroded"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let melee_id: WeaponId = "core:integrity_blade"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let ranged_id: WeaponId = "core:needle_launcher"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let flamethrower_id: WeaponId = "core:flamethrower"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let repair_id: ItemId = "core:repair_patch"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let player_starting_equipment = vec![
        Some(melee_id.clone()),
        Some(ranged_id.clone()),
        Some(flamethrower_id.clone()),
    ];
    let player_starting_weapons = vec![melee_id, ranged_id, flamethrower_id];
    let player_base_attacks = player_starting_weapons
        .iter()
        .map(|id| {
            loaded
                .weapons()
                .get(id)
                .ok_or_else(|| format!("missing starting weapon '{id}'"))
                .map(|weapon| weapon.attack())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let player_weapon_slots = [
        "core:close_combat_channel",
        "core:ranged_combat_channel",
        "core:auxiliary_combat_channel",
    ]
    .into_iter()
    .map(str::parse)
    .collect::<Result<Vec<_>, project_rl::content::ContentIdError>>()
    .map_err(|error| error.to_string())?;
    let texts = loaded.texts().clone();
    let loot = loaded.loot().clone();
    let expeditions = loaded.expeditions().clone();
    let (statuses, weapons, items, skills) = loaded.into_registries();
    let mut rules = GameRules {
        player_base_attacks,
        player_weapon_slots,
        player_starting_weapons,
        player_starting_items: vec![StartingItemStack::new(repair_id, 2)],
        player_starting_equipment,
        statuses,
        weapons,
        items,
        skills,
        enabled_system_features: SystemFeatureSet::new(["core:traces"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?]),
        ..GameRules::default()
    };
    rules.player_base_abilities.push(AbilityProfile::new(
        6,
        DistanceMetric::Euclidean,
        true,
        true,
        vec![EffectPrimitive::ApplyStatus(
            ApplyStatusEffect::new(corrosion_id, 1).map_err(|error| error.to_string())?,
        )],
    ));
    Ok((rules, texts, loot, expeditions))
}

fn ascii_visual_cue_catalog() -> Result<VisualCueCatalog, String> {
    static CATALOG: std::sync::OnceLock<Result<VisualCueCatalog, String>> =
        std::sync::OnceLock::new();
    CATALOG
        .get_or_init(|| {
            let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ContentLoader::load(
                &[project_root.join("content"), project_root.join("mods")],
                &semver::Version::new(0, 1, 0),
            )
            .map(|loaded| loaded.visual_cues().clone())
            .map_err(|error| error.to_string())
        })
        .clone()
}

fn display_content_name(id: &project_rl::content::ContentId) -> String {
    id.name().replace(['_', '-'], " ").to_uppercase()
}

fn technical_reference(id: &project_rl::content::ContentId) -> String {
    id.name().replace('_', "-").to_uppercase()
}

fn draw_stat_line(
    x: f32,
    y: f32,
    label: &str,
    value: &str,
    value_color: Color,
    label_color: Color,
) {
    draw_text(label, x, y, 16.0, label_color);
    draw_text(value, x + 148.0, y, 18.0, value_color);
}

fn command_rejection_message(reason: CommandRejection) -> &'static str {
    match reason {
        CommandRejection::PassageUnavailable => "Passage indisponible.",
        CommandRejection::PassageObstructed => "Arrivée encombrée : attendez avant de réessayer.",
        CommandRejection::InteractionOutOfReach => {
            "Interaction hors de portée : placez-vous juste à côté."
        }
        CommandRejection::NothingToInteract => "Aucune interaction ici.",
        CommandRejection::DoorLocked => "Porte verrouillée : cherchez la console de commande.",
        CommandRejection::DoorUnpowered => {
            "Porte sans alimentation : le relais associé doit être remis en service."
        }
        CommandRejection::DoorObstructed => "Porte encombrée : fermeture impossible.",
        CommandRejection::ControlUnavailable => "Cette console n'a plus d'action disponible.",
        CommandRejection::NoMaterialForDepot => {
            "Vous ne transportez aucune pièce actuellement demandée par ce dépôt."
        }
        CommandRejection::FacilityUnavailable => {
            "L'installation ne peut pas traiter cette livraison pour le moment."
        }
        CommandRejection::ProtectedZone => {
            "Zone protégée : aucune attaque depuis ou vers la ville."
        }
        CommandRejection::RunEnded => "THE RUN HAS ENDED",
        CommandRejection::NotPlayersTurn => "WAIT FOR YOUR TURN",
        CommandRejection::MissingPlayer => "CORE CONNECTION LOST",
        CommandRejection::BlockedByTerrain(_) => "PATH BLOCKED",
        CommandRejection::Occupied(_) => "SPACE OCCUPIED",
        CommandRejection::UnknownTarget(_) => "TARGET LOST",
        CommandRejection::MissingAttackSlot(_) => "NO WEAPON EQUIPPED IN THIS CHANNEL",
        CommandRejection::TargetOutOfRange(_) => "TARGET OUT OF RANGE",
        CommandRejection::NoLineOfSight(_) => "NO LINE OF SIGHT",
        CommandRejection::AttackTargetOutsideMap(_) => "INVALID TARGET AREA",
        CommandRejection::AttackTargetIsOrigin => "AIM AWAY FROM YOUR POSITION",
        CommandRejection::AttackTargetOutOfRange(_) => "TARGET AREA OUT OF RANGE",
        CommandRejection::AttackNoLineOfSight(_) => "NO LINE OF SIGHT TO TARGET AREA",
        CommandRejection::FreeAimRequiresAreaWeapon => "THIS WEAPON REQUIRES AN ENTITY TARGET",
        CommandRejection::MissingEquipmentSlot(_) => "UNKNOWN EQUIPMENT CHANNEL",
        CommandRejection::UnknownInventoryItem(_) => "ITEM IS NO LONGER AVAILABLE",
        CommandRejection::ItemIsNotWeapon(_) => "THIS ITEM IS NOT A WEAPON",
        CommandRejection::ItemAlreadyEquippedInSlot { .. } => "WEAPON ALREADY EQUIPPED",
        CommandRejection::ItemIsNotUsable(_) => "Cet objet ne s'utilise pas directement.",
        CommandRejection::ItemHasNoUsefulEffect(_) => "PV ARE ALREADY FULL",
        CommandRejection::InventoryChanged(_) => "INVENTORY CHANGED; TRY AGAIN",
        CommandRejection::NoItemToPickUp => "NO ITEM HERE",
        CommandRejection::InventoryCannotFitItem => "INVENTORY CANNOT HOLD THIS STACK",
        CommandRejection::UnknownGroundItemDefinition => "LOOT DATA IS UNAVAILABLE",
        CommandRejection::CannotDropItemHere => "MOVE OFF THE CURRENT LOOT BEFORE DROPPING",
        CommandRejection::MissingAbilitySlot(_) => "ABILITY UNAVAILABLE",
        CommandRejection::AbilityTargetOutsideMap(_) => "INVALID TARGET AREA",
        CommandRejection::AbilityTargetBlocked(_) => "TARGET AREA BLOCKED",
        CommandRejection::AbilityTargetOutOfRange(_) => "ABILITY TARGET OUT OF RANGE",
        CommandRejection::AbilityNoLineOfSight(_) => "ABILITY HAS NO LINE OF SIGHT",
        CommandRejection::AbilityTargetHasNoActor(_) => "NO ACTOR AT TARGET",
        CommandRejection::AbilityUnknownStatusDefinition => "ABILITY DATA IS UNAVAILABLE",
        CommandRejection::TechniqueLearning(_) => "TECHNIQUE CANNOT BE LEARNED",
        CommandRejection::InsufficientSkillPoints { .. } => "NOT ENOUGH SKILL POINTS",
        CommandRejection::UnknownTechnique(_) => "TECHNIQUE DATA IS UNAVAILABLE",
        CommandRejection::TechniqueNotLearned(_) => "LEARN THIS TECHNIQUE IN THE SKILLS SCREEN",
        CommandRejection::TechniqueHasNoActiveAction(_) => "THIS TECHNIQUE IS NOT AN ACTION",
        CommandRejection::TechniqueMissingTarget => "SELECT A VISIBLE TARGET",
        CommandRejection::TechniqueUnexpectedTarget => "THIS TECHNIQUE NEEDS NO TARGET",
        CommandRejection::TechniqueTargetNotVisible(_) => "TARGET IS NOT VISIBLE",
        CommandRejection::TechniqueTargetOutOfRange(_) => "TARGET IS OUT OF SENSOR RANGE",
        CommandRejection::TechniqueTooManyTargets { .. } => "TROP DE CIBLES POUR CETTE TECHNIQUE",
        CommandRejection::TechniqueDuplicateTarget(_) => {
            "UNE MÊME CIBLE EST SÉLECTIONNÉE PLUSIEURS FOIS"
        }
        CommandRejection::InsufficientEnergy { .. } => "ÉNERGIE INSUFFISANTE",
    }
}

fn attack_preview_rejection_label(reason: &CommandRejection) -> &'static str {
    match reason {
        CommandRejection::ProtectedZone => "ZONE PROTÉGÉE",
        CommandRejection::AttackTargetOutsideMap(_) => "HORS CARTE",
        CommandRejection::AttackTargetIsOrigin => "CHOISISSEZ UNE DIRECTION",
        CommandRejection::AttackTargetOutOfRange(_) => "HORS DE PORTÉE",
        CommandRejection::AttackNoLineOfSight(_) => "LIGNE DE VUE BLOQUÉE",
        CommandRejection::FreeAimRequiresAreaWeapon => "ARME SANS ZONE",
        CommandRejection::MissingAttackSlot(_) => "CANAL VIDE",
        _ => "VISÉE INDISPONIBLE",
    }
}

fn technique_id(value: &str) -> Option<TechniqueId> {
    value.parse().ok()
}

fn visual_cue_id(value: &str) -> VisualCueId {
    value
        .parse()
        .expect("built-in visual cue IDs must be canonical")
}

fn read_movement_command(
    game: &WorldState,
    attack_slot: u8,
    controls: &Controls,
    input: &InputFrame,
) -> Option<GameCommand> {
    let direction = if controls.pressed(Action::MoveNorth, input) {
        Some(Direction::North)
    } else if controls.pressed(Action::MoveEast, input) {
        Some(Direction::East)
    } else if controls.pressed(Action::MoveSouth, input) {
        Some(Direction::South)
    } else if controls.pressed(Action::MoveWest, input) {
        Some(Direction::West)
    } else {
        None
    };

    if let Some(direction) = direction {
        let player_position = game.player_position()?;
        let destination = player_position.step(direction);
        if let Some(target) = game.actors().entity_at(destination)
            && target != game.player_id()
            && game.active_worker_role(target).is_none()
        {
            return Some(GameCommand::Attack {
                slot: attack_slot,
                target,
            });
        }
        return Some(GameCommand::Move(direction));
    }

    controls
        .pressed(Action::Wait, input)
        .then_some(GameCommand::Wait)
}

fn pressed_weapon_slot(controls: &Controls, input: &InputFrame) -> Option<u8> {
    if controls.pressed(Action::Slot1, input) {
        Some(0)
    } else if controls.pressed(Action::Slot2, input) {
        Some(1)
    } else if controls.pressed(Action::Slot3, input) {
        Some(2)
    } else {
        None
    }
}

fn grid_distance(first: GridPos, second: GridPos) -> u32 {
    let delta_x = (i64::from(first.x) - i64::from(second.x)).unsigned_abs();
    let delta_y = (i64::from(first.y) - i64::from(second.y)).unsigned_abs();
    delta_x.max(delta_y).min(u64::from(u32::MAX)) as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{Binding, Layout};
    use project_rl::world::Map;

    fn input(key: &str) -> InputFrame {
        InputFrame {
            pressed: if key == "Escape" {
                Default::default()
            } else {
                [Binding::key(key)].into()
            },
            pause: key == "Escape",
            ..Default::default()
        }
    }

    fn app_with_test_controls() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.controls = Controls::preset(Layout::Azerty, KeySemantics::Physical);
        app.suspension_path = temporary_folder("test-run").join("suspended-run.json");
        app
    }

    fn temporary_folder(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "project-rl-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn apply(app: &mut AsciiApp, command: GameCommand) {
        let outcome = app.execute_command(command);
        assert!(
            !matches!(outcome, CommandOutcome::Rejected(_)),
            "{outcome:?}"
        );
        app.capture_events_at(Some(0.0));
    }

    fn menu_pointer(menu: MenuScreen, index: usize, clicked: bool) -> InputFrame {
        let layout = MenuLayout::new(1280.0, 800.0, menu.buttons().len());
        let rect = layout.buttons[index];
        InputFrame {
            pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
            viewport: Some((1280.0, 800.0)),
            pressed: if clicked {
                [Binding::MouseLeft].into()
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn menu_hover_does_not_need_a_click_and_keyboard_keeps_focus_until_mouse_moves() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Pause);
        app.update_input(&menu_pointer(MenuScreen::Pause, 1, false));
        assert_eq!(app.menu, MenuScreen::Pause);
        assert_eq!(app.menu_selection, 1);
        assert!(app.menu_focus.highlighted(1, app.menu_selection));
        let mut keyboard = menu_pointer(MenuScreen::Pause, 1, false);
        keyboard.pressed.insert(Binding::key("Down"));
        app.update_input(&keyboard);
        assert_eq!(app.menu_selection, 2);
        app.update_input(&menu_pointer(MenuScreen::Pause, 1, false));
        assert_eq!(app.menu_selection, 2);
        assert!(app.menu_focus.highlighted(2, app.menu_selection));
        app.update_input(&InputFrame {
            pointer: Some((1.0, 1.0)),
            ..Default::default()
        });
        assert!(!app.menu_focus.highlighted(2, app.menu_selection));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
        assert!(!app.should_quit());
    }

    #[test]
    fn clicking_outside_menu_never_activates_a_mouse_bound_confirmation() {
        let mut app = app_with_test_controls();
        app.controls
            .rebind(Action::Learn, Binding::MouseLeft)
            .unwrap();
        app.open_menu(MenuScreen::ConfirmAbandon);
        app.menu_selection = 1;
        app.update_input(&InputFrame {
            pointer: Some((1.0, 1.0)),
            viewport: Some((1280.0, 800.0)),
            pressed: [Binding::MouseLeft].into(),
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::ConfirmAbandon);
        assert!(!app.should_quit());
        app.update_input(&menu_pointer(MenuScreen::ConfirmAbandon, 0, true));
        assert_eq!(app.menu, MenuScreen::Pause);
    }

    #[test]
    fn controls_support_hover_click_wheel_and_clickable_back_without_advancing_time() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Controls);
        let mut pointer = InputFrame {
            pointer: Some((50.0, 180.0)),
            viewport: Some((700.0, 480.0)),
            ..Default::default()
        };
        app.update_input(&pointer);
        assert_eq!(app.options_selection, 1);
        assert!(!app.rebinding);
        pointer.pressed.insert(Binding::MouseLeft);
        app.update_input(&pointer);
        assert!(app.rebinding);
        assert_eq!(app.controls.binding(Action::MoveNorth), &Binding::key("W"));
        app.update_input(&input("Escape"));
        assert!(!app.rebinding);
        pointer.pressed.clear();
        pointer.wheel_y = -3.0;
        app.update_input(&pointer);
        assert_eq!(app.options_scroll, 3);
        assert_eq!(app.options_selection, 4);
        let layout = ControlsLayout::new(700.0, 480.0, 3, Action::ALL.len() + 1);
        pointer.pointer = Some((layout.back.x + 5.0, layout.back.y + 5.0));
        pointer.wheel_y = 0.0;
        pointer.pressed.insert(Binding::MouseLeft);
        app.update_input(&pointer);
        assert_eq!(app.menu, MenuScreen::Options);
        app.open_menu(MenuScreen::Hidden);
        app.observation_report = (0..20).map(|index| format!("Ligne {index}")).collect();
        app.report_open = true;
        app.update_input(&InputFrame {
            wheel_y: -3.0,
            ..Default::default()
        });
        assert_eq!(app.report_scroll, 3);
        app.update_input(&InputFrame {
            wheel_y: 2.0,
            ..Default::default()
        });
        assert_eq!(app.report_scroll, 1);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn graphics_menu_applies_confirms_persists_and_preserves_run_and_controls() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("graphics-menu");
        app.graphics.path = folder.join("graphics.json");
        app.controls_path = folder.join("controls.json");
        app.controls.save(&app.controls_path).unwrap();
        let bindings = std::fs::read(&app.controls_path).unwrap();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Options);
        app.update_input(&menu_pointer(MenuScreen::Options, 1, true));
        assert_eq!(app.menu, MenuScreen::Graphics);
        // The automatic desktop resolution and future renderer cannot be selected.
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 2);
        app.update_input(&input("Right"));
        assert_eq!(app.graphics.draft.ui_scale_percent, 125);
        assert_eq!(app.graphics.active.ui_scale_percent, 100);
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 4);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 3, true));
        assert_eq!(app.menu_selection, 4);
        assert_eq!(app.menu_focus.hovered, None);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 0, true));
        assert_eq!(app.graphics.draft.mode, WindowMode::Windowed);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 1, true));
        assert_eq!(app.graphics.draft.windowed_size, [1600, 900]);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 6, true));
        assert_eq!(app.menu, MenuScreen::ConfirmGraphics);
        assert!(app.graphics.previewing());
        assert!(!app.graphics.path.exists());
        // Click the confirmation using physical coordinates at the new UI scale.
        let scale = app.graphics.active.ui_scale(1600.0, 900.0);
        let layout = MenuLayout::new(1600.0 / scale, 900.0 / scale, 2);
        let keep = layout.buttons[1];
        let pointer = app.graphics.active.transform_input(InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((
                (keep.x + keep.w / 2.0) * scale,
                (keep.y + keep.h / 2.0) * scale,
            )),
            viewport: Some((1600.0, 900.0)),
            ..Default::default()
        });
        app.update_input(&pointer);
        assert_eq!(app.menu, MenuScreen::Graphics);
        assert!(!app.graphics.previewing());
        assert_eq!(
            GraphicsSettings::load(&app.graphics.path).0,
            app.graphics.active
        );
        assert_eq!(std::fs::read(&app.controls_path).unwrap(), bindings);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
        // New runs retain display choices, just like bindings.
        let settings = app.graphics.active;
        app.open_menu(MenuScreen::Hidden);
        app.update_input(&input("R"));
        assert_eq!(app.graphics.active, settings);
        assert_eq!(app.graphics.path, folder.join("graphics.json"));
        std::fs::remove_file(app.graphics.path).unwrap();
        std::fs::remove_file(app.controls_path).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn graphics_escape_and_timeout_restore_settings_without_consuming_turns() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        for exit in ["Escape", "timeout"] {
            app.open_menu(MenuScreen::Graphics);
            app.update_input(&input("Enter")); // Borderless -> windowed draft.
            assert_eq!(app.graphics.active.mode, WindowMode::Borderless);
            app.update_input(&menu_pointer(MenuScreen::Graphics, 6, true));
            assert_eq!(app.menu, MenuScreen::ConfirmGraphics);
            for key in ["W", "Space", "F", "R"] {
                app.update_input(&input(key));
            }
            match exit {
                "Escape" => app.update_input(&input("Escape")),
                "timeout" => {
                    assert!(app.tick_graphics(16.0));
                }
                _ => unreachable!(),
            }
            assert_eq!(app.graphics.active.mode, WindowMode::Borderless);
            assert!(!app.graphics.previewing());
            assert!(!app.should_quit());
        }
        app.open_menu(MenuScreen::Graphics);
        app.update_input(&input("Enter"));
        app.update_input(&input("Escape")); // Discard an unapplied draft too.
        app.open_menu(MenuScreen::Graphics);
        assert_eq!(app.graphics.draft, app.graphics.active);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn normal_exit_saves_once_and_no_input_can_change_the_saved_snapshot() {
        for exit in ["keyboard", "mouse", "window"] {
            let mut app = app_with_test_controls();
            apply(&mut app, GameCommand::Wait);
            let before = suspension::fingerprint(&app.game);
            let history = app.history.clone();
            app.open_menu(MenuScreen::Pause);
            assert_eq!(app.menu_labels()[2], "Sauvegarder et quitter");
            match exit {
                "keyboard" => {
                    app.menu_selection = 2;
                    app.update_input(&input("Enter"));
                }
                "mouse" => app.update_input(&menu_pointer(MenuScreen::Pause, 2, true)),
                _ => {
                    // Closing even from the abandonment dialog uses the safe exit.
                    app.open_menu(MenuScreen::ConfirmAbandon);
                    app.request_quit();
                }
            }
            assert!(app.should_quit(), "{}", app.menu_message);
            let saved = Suspension::read(&app.suspension_path).unwrap();
            assert_eq!(saved.state, before);
            assert_eq!(saved.commands, history);
            let bytes = std::fs::read(&app.suspension_path).unwrap();
            app.request_quit(); // Repeated close events cannot overwrite or fail the save.
            for key in ["Escape", "Space", "R", "W", "Enter"] {
                app.update_input(&input(key));
            }
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            app.resume_run().unwrap();
            assert!(!app.should_quit());
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert!(!app.suspension_path.exists()); // Still a single-use suspension.
            std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn window_close_reverts_unconfirmed_graphics_and_saves_the_active_run() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        let display = app.graphics.active;
        app.open_menu(MenuScreen::Graphics);
        app.update_input(&input("Enter"));
        app.update_input(&menu_pointer(MenuScreen::Graphics, 6, true));
        assert!(app.graphics.previewing());
        app.request_quit();
        assert!(app.should_quit());
        assert!(!app.graphics.previewing());
        assert_eq!(app.graphics.active, display);
        assert_eq!(
            Suspension::read(&app.suspension_path).unwrap().state,
            before
        );
        app.resume_run().unwrap();
        assert_eq!(app.graphics.active, display);
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn failed_window_close_keeps_run_open_and_can_retry_without_losing_progress() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("close-failure");
        std::fs::create_dir(&folder).unwrap();
        let blocker = folder.join("not-a-directory");
        std::fs::write(&blocker, "fixture").unwrap();
        app.suspension_path = blocker.join("run.json");
        apply(&mut app, GameCommand::Wait);
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Controls);
        app.rebinding = true;
        app.request_quit();
        assert!(!app.should_quit());
        assert_eq!(app.menu, MenuScreen::Pause);
        assert!(!app.rebinding);
        assert!(app.menu_message.contains("Sauvegarde impossible"));
        assert!(app.menu_message.contains("reste ouverte"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(std::fs::read_to_string(&blocker).unwrap(), "fixture");
        app.suspension_path = folder.join("run.json");
        app.request_quit();
        assert!(app.should_quit());
        app.resume_run().unwrap();
        assert_eq!(suspension::fingerprint(&app.game), before);
        std::fs::remove_file(blocker).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn closing_resume_prompt_or_abandoning_never_removes_an_existing_suspension() {
        for exit in ["window", "button", "abandon"] {
            let mut app = app_with_test_controls();
            let folder = app.suspension_path.parent().unwrap().to_path_buf();
            std::fs::create_dir(&folder).unwrap();
            std::fs::write(
                &app.suspension_path,
                "existing incompatible suspension fixture",
            )
            .unwrap();
            let bytes = std::fs::read(&app.suspension_path).unwrap();
            if exit == "abandon" {
                app.request_quit();
                assert!(!app.should_quit()); // Normal exit refuses to overwrite.
                assert!(app.menu_message.contains("Sauvegarde impossible"));
                app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
                app.update_input(&menu_pointer(MenuScreen::ConfirmAbandon, 1, true));
            } else {
                app.open_menu(MenuScreen::ResumeSuspension);
                if exit == "window" {
                    app.request_quit();
                } else {
                    app.update_input(&menu_pointer(MenuScreen::ResumeSuspension, 1, true));
                }
            }
            assert!(app.should_quit());
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            std::fs::remove_file(&app.suspension_path).unwrap();
            std::fs::remove_dir(folder).unwrap();
        }
    }

    #[test]
    fn pause_buttons_options_and_abandon_confirmation_are_modal_and_clickable() {
        let mut app = app_with_test_controls();
        assert_eq!(
            MenuScreen::ConfirmAbandon.buttons(),
            &["Annuler", "Confirmer"]
        );
        let before = suspension::fingerprint(&app.game);
        app.update_input(&input("F1"));
        assert!(app.legend_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        app.update_input(&input("W"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&input("Escape"));
        assert!(!app.legend_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        for key in ["W", "F", "C", "Space", "R"] {
            app.update_input(&input(key));
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
        let layout = MenuLayout::new(1280.0, 800.0, 4);
        let button = layout.buttons[1];
        app.update_input(&InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((button.x + 10.0, button.y + 10.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Controls);
        app.update_input(&input("Escape"));
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        app.menu_selection = 3;
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::ConfirmAbandon);
        assert!(!app.should_quit());
        assert_eq!(app.menu_selection, 0);
        app.update_input(&input("Enter")); // default = cancel
        assert_eq!(app.menu, MenuScreen::Pause);
        app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
        app.update_input(&input("Escape"));
        assert!(!app.should_quit());
        app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        assert!(app.should_quit());
        assert!(!app.suspension_path.exists());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn suspension_replays_the_complete_run_and_is_consumed_only_once() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        rules.progression.starting_skill_points = 9;
        rules.player_maximum_integrity = 500;
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        let folder = temporary_folder("suspension");
        app.suspension_path = folder.join("suspended-run.json");
        app.session_lock = Some(suspension::session_lock(&folder.join("session.lock")).unwrap());
        for id in [
            "core:rec_01",
            "core:rec_02",
            "core:rec_09",
            "core:rec_04",
            "core:rec_05",
        ] {
            apply(
                &mut app,
                GameCommand::LearnTechnique {
                    technique: technique_id(id).unwrap(),
                },
            );
        }
        let weapon = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:needle_launcher")
            .unwrap()
            .instance();
        apply(
            &mut app,
            GameCommand::EquipWeapon {
                slot: 2,
                item: weapon,
            },
        );
        app.active_weapon_slot = 2;
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        for _ in 0..150 {
            let origin = app.game.player_position().unwrap();
            if app.visible_targets().iter().any(|target| {
                grid_distance(origin, app.game.actors().get(*target).unwrap().position()) <= 4
            }) {
                break;
            }
            let goal = app
                .game
                .actors()
                .iter()
                .filter(|(id, _)| *id != app.game.player_id())
                .min_by_key(|(_, actor)| grid_distance(origin, actor.position()))
                .unwrap()
                .1
                .position();
            let path = project_rl::world::find_path(app.game.map(), origin, goal, 5000, |pos| {
                Some(pos) != app.game.exit()
            })
            .unwrap();
            let next = path[1];
            apply(
                &mut app,
                GameCommand::Move(
                    Direction::from_delta(next.x - origin.x, next.y - origin.y).unwrap(),
                ),
            );
        }
        let target = app.visible_targets()[0];
        app.selected_target = Some(target);
        apply(
            &mut app,
            GameCommand::UseTechnique {
                technique: technique_id("core:rec_09").unwrap(),
                targets: vec![target],
            },
        );
        assert_eq!(app.game.player_energy().available(), 98);
        apply(
            &mut app,
            GameCommand::UseTechnique {
                technique: technique_id("core:rec_02").unwrap(),
                targets: vec![],
            },
        );
        let repair = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:repair_patch")
            .unwrap()
            .instance();
        apply(&mut app, GameCommand::DropItem { item: repair });
        apply(&mut app, GameCommand::PickUp);
        let target = app.visible_targets()[0];
        let at = app.game.actors().get(target).unwrap().position();
        apply(
            &mut app,
            GameCommand::UseAbility {
                slot: 1,
                target: at,
            },
        );
        assert!(!app.observation_report.is_empty());
        let before = suspension::fingerprint(&app.game);
        let report = app.observation_report.clone();
        let history = app.history.clone();
        app.open_menu(MenuScreen::Pause);
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(app.should_quit(), "{}", app.menu_message);
        assert!(app.suspension_path.exists());
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.suspend_run().is_err()); // no overwrite of an existing suspended run
        let mut next = AsciiApp::from_seed(
            1,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        next.suspension_path = app.suspension_path.clone();
        next.session_lock = app.session_lock.take();
        next.open_menu(MenuScreen::ResumeSuspension);
        next.update_input(&input("Enter"));
        assert_eq!(next.menu, MenuScreen::Hidden, "{}", next.menu_message);
        assert_eq!(suspension::fingerprint(&next.game), before);
        assert_eq!(next.history, history);
        assert_eq!(next.observation_report, report);
        assert_eq!(next.active_weapon_slot, 2);
        assert!(!next.suspension_path.exists());
        assert!(next.resume_run().is_err());
        assert_eq!(suspension::fingerprint(&next.game), before);
        apply(&mut next, GameCommand::Wait);
        next.suspend_run().unwrap(); // a new suspension continues the same journal
        next.resume_run().unwrap();
        assert_eq!(next.game.player_energy().available(), 98);
        drop(next);
        std::fs::remove_file(folder.join("session.lock")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn a_click_resumes_an_older_build_only_after_an_identical_replay() {
        for key in [false, true] {
            let mut app = app_with_test_controls();
            app.walk_fixture_to(GridPos::new(32, 16)).unwrap();
            apply(
                &mut app,
                GameCommand::Interact {
                    target: GridPos::new(32, 15),
                },
            );
            let mut saved = app.suspension().unwrap();
            saved.build = "617e393cfa1b13c7".to_owned();
            assert_ne!(saved.build, env!("PROJECT_RL_BUILD_FINGERPRINT"));
            saved.write(&app.suspension_path).unwrap();
            let mut next = app_with_test_controls();
            next.suspension_path = app.suspension_path.clone();
            next.open_menu(MenuScreen::ResumeSuspension);
            next.update_input(&if key {
                input("Enter")
            } else {
                menu_pointer(MenuScreen::ResumeSuspension, 0, true)
            });
            assert_eq!(next.menu, MenuScreen::Hidden, "{}", next.menu_message);
            assert_eq!(suspension::fingerprint(&next.game), saved.state);
            assert_eq!(next.history, saved.commands);
            assert_eq!(
                next.game.map().tile(GridPos::new(32, 15)),
                app.game.map().tile(GridPos::new(32, 15))
            );
            assert!(!next.suspension_path.exists());
            assert!(next.resume_run().is_err());
            next.suspend_run().unwrap();
            assert_eq!(
                Suspension::read(&next.suspension_path).unwrap().build,
                env!("PROJECT_RL_BUILD_FINGERPRINT")
            );
            std::fs::remove_file(&next.suspension_path).unwrap();
            std::fs::remove_dir(next.suspension_path.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn cross_build_replay_rejects_incompatible_data_and_preserves_file_and_game() {
        for fault in [
            "version", "build", "rules", "loot", "world", "state", "command", "slot", "target",
        ] {
            let mut app = app_with_test_controls();
            let mut saved = app.suspension().unwrap();
            saved.build = "617e393cfa1b13c7".to_owned();
            match fault {
                "version" => saved.version = 255,
                "build" => saved.build = "invalid".to_owned(),
                "rules" => saved.rules ^= 1,
                "loot" => saved.loot_rules = saved.loot_rules.map(|fingerprint| fingerprint ^ 1),
                "world" => saved.world_rules = saved.world_rules.map(|fingerprint| fingerprint ^ 1),
                "state" => saved.state ^= 1,
                "command" => saved
                    .commands
                    .push(RecordedCommand::Move { direction: 255 }),
                "slot" => saved.active_weapon_slot = 255,
                "target" => saved.selected_target = Some(u64::MAX),
                _ => unreachable!(),
            }
            let bytes = serde_json::to_vec(&saved).unwrap();
            std::fs::create_dir_all(app.suspension_path.parent().unwrap()).unwrap();
            std::fs::write(&app.suspension_path, &bytes).unwrap();
            let before = suspension::fingerprint(&app.game);
            app.open_menu(MenuScreen::ResumeSuspension);
            app.update_input(&menu_pointer(MenuScreen::ResumeSuspension, 0, true));
            assert_eq!(app.menu, MenuScreen::ResumeSuspension, "{fault}");
            assert!(!app.should_quit());
            assert!(
                app.menu_message.starts_with("Reprise impossible : "),
                "{fault}"
            );
            assert_eq!(app.menu_labels()[0], "Réessayer la reprise");
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            app.request_quit();
            assert!(app.should_quit());
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            std::fs::remove_file(&app.suspension_path).unwrap();
            std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
        }
    }

    /// Explicit local diagnostic, never run by the normal suite. Read-only:
    /// verifies a chosen real suspension without consuming it or creating a run.
    #[test]
    #[ignore = "Set PROJECT_RL_CHECK_SUSPENSION to a file to verify without consuming it"]
    fn check_suspension_file_without_consuming() {
        let path = std::path::PathBuf::from(
            std::env::var_os("PROJECT_RL_CHECK_SUSPENSION")
                .expect("explicit suspension path required"),
        );
        let before = std::fs::read(&path).unwrap();
        let saved = Suspension::read(&path).unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), before);
        let resaved = restored.suspension().unwrap();
        let checked = AsciiApp::restore_suspension(
            &resaved,
            restored.rules.clone(),
            restored.texts.clone(),
            restored.loot.clone(),
            restored.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&checked.game), resaved.state);
        println!(
            "Suspension vérifiée sans consommation : {} commandes, état {:016x}, position {:?}.",
            saved.commands.len(),
            saved.state,
            restored.game.player_position()
        );
    }

    #[test]
    fn failed_suspension_or_incompatible_resume_never_quits_or_consumes_data() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("bad-suspension");
        std::fs::create_dir(&folder).unwrap();
        app.suspension_path = folder.join("suspended-run.json");
        let mut saved = app.suspension().unwrap();
        saved.state ^= 1;
        saved.write(&app.suspension_path).unwrap();
        let bytes = std::fs::read(&app.suspension_path).unwrap();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::ResumeSuspension);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::ResumeSuspension);
        assert!(!app.should_quit());
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
        assert_eq!(suspension::fingerprint(&app.game), before);
        saved.build = "another build".to_owned();
        assert!(
            AsciiApp::restore_suspension(
                &saved,
                app.rules.clone(),
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone()
            )
            .is_err()
        );
        let mut altered_rules = app.rules.clone();
        altered_rules.player_starting_energy -= 1;
        assert!(
            AsciiApp::restore_suspension(
                &app.suspension().unwrap(),
                altered_rules,
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone()
            )
            .is_err()
        );
        app.open_menu(MenuScreen::Pause);
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(!app.should_quit());
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::write(folder.join("not-a-directory"), "block").unwrap();
        app.suspension_path = folder.join("not-a-directory/suspend.json");
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(!app.should_quit());
        assert!(app.menu_message.contains("Sauvegarde impossible"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        std::fs::remove_file(folder.join("not-a-directory")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn an_unrecorded_mutation_cannot_be_saved_and_session_lock_prevents_double_resume() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("session");
        let path = folder.join("session.lock");
        let guard = suspension::session_lock(&path).unwrap();
        assert!(suspension::session_lock(&path).is_err());
        drop(guard);
        let guard = suspension::session_lock(&path).unwrap();
        app.suspension_path = folder.join("suspended-run.json");
        app.game.process_player_command(GameCommand::Wait);
        app.game.drain_events();
        assert!(app.suspend_run().is_err());
        assert!(!app.suspension_path.exists());
        drop(guard);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn a_finished_run_cannot_create_a_resurrection_checkpoint() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        let player = app.game.player_id();
        for _ in 0..100 {
            if app.game.status() != RunStatus::Active {
                break;
            }
            apply(
                &mut app,
                GameCommand::Attack {
                    slot: 0,
                    target: player,
                },
            );
        }
        assert_eq!(app.game.status(), RunStatus::PlayerDestroyed);
        assert!(app.suspension().is_err());
        app.open_menu(MenuScreen::Pause);
        assert_eq!(app.menu_labels()[2], "Quitter");
        assert!(!app.menu_row_enabled(3));
        app.menu_selection = 2;
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 2); // Skip the unavailable last row.
        app.update_input(&input("Enter"));
        assert!(app.should_quit());
        assert!(!app.suspension_path.exists());
    }

    #[test]
    fn interactions_use_the_binding_and_replay_door_console_and_remembered_states() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(32, 16)).unwrap();
        let door = GridPos::new(32, 15);
        let turn = app.game.turn();
        app.update_input(&input("V"));
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert_eq!(app.game.turn(), turn + 1);
        app.update_input(&input("V"));
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Closed)
        );
        app.walk_fixture_to(GridPos::new(50, 18)).unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: TestSector::CONTROL,
            },
        );
        app.walk_fixture_to(GridPos::new(52, 16)).unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: TestSector::LOCKED_DOOR,
            },
        );
        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
        for p in app.game.player_visibility().explored_positions() {
            assert_eq!(app.terminal.known(p), restored.terminal.known(p));
        }
    }

    #[test]
    fn expedition_round_trip_replays_every_zone_and_its_visual_memory() {
        let mut app = app_with_test_controls();
        app.walk_expedition_fixture(2).unwrap();
        assert_eq!(app.game.visited_zone_count(), 2);
        assert_eq!(app.game.current_zone().unwrap().depth, 1);
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 10);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.zone_views.len(), 1);
        assert_eq!(restored.history, app.history);
        for p in app.game.player_visibility().explored_positions() {
            assert_eq!(app.terminal.known(p), restored.terminal.known(p));
        }
        for (id, view) in &app.zone_views {
            let other = &restored.zone_views[id];
            for y in 0..68 {
                for x in 0..110 {
                    let p = GridPos::new(x, y);
                    assert_eq!(view.known(p), other.known(p));
                }
            }
        }
        let before = suspension::fingerprint(&app.game);
        app.update_input(&input("Escape"));
        for _ in 0..5 {
            app.update_input(&input("Space"));
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn maintenance_circuit_moves_one_real_part_and_restores_linked_functions() {
        let mut app = app_with_test_controls();
        let relay = "core:maintenance_relay".parse().unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        let order = "core:restore_checkpoint_power".parse().unwrap();
        let regulator = "core:power_regulator".parse().unwrap();
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Unpowered)
        );
        let initial = app.game.active_facility().unwrap();
        assert!(!initial.is_operational(&relay));
        assert!(!initial.is_operational(&sensor));
        assert_eq!(
            initial.repair_status(&order),
            Some(project_rl::facility::RepairStatus::WaitingForMaterial)
        );

        for _ in 0..96 {
            apply(&mut app, GameCommand::Wait);
        }

        let repaired = app.game.active_facility().unwrap();
        assert!(repaired.is_operational(&relay));
        assert!(repaired.is_operational(&sensor));
        assert_eq!(
            app.terminal.decor.cells.get(&GridPos::new(46, 32)),
            Some(&crate::test_sector::Decor::RelayOnline)
        );
        assert_eq!(
            repaired.repair_status(&order),
            Some(project_rl::facility::RepairStatus::Completed)
        );
        assert_eq!(repaired.depot_stock(&regulator), 0);
        assert_eq!(
            app.game.map().tile(GridPos::new(32, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert!(
            app.game
                .ground_items()
                .iter()
                .all(|(_, stack)| stack.item() != &regulator)
        );
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Closed)
        );
    }

    #[test]
    fn installed_security_alarm_is_visible_and_survives_replay() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let regulator: ItemId = "core:power_regulator".parse().unwrap();
        let owner: project_rl::social::SocialGroupId =
            "core:maintenance_collective".parse().unwrap();
        let mut alarm_expeditions = ExpeditionCatalog::default();
        for (id, definition) in expeditions.iter() {
            let mut definition = definition.clone();
            if id.as_str() == "core:starter_expedition" {
                definition.hub_facility.as_mut().unwrap().materials.push(
                    project_rl::content::FacilityMaterialSpawn {
                        position: GridPos::new(52, 27),
                        item: regulator.clone(),
                        quantity: 1,
                        owner: Some(owner.clone()),
                    },
                );
            }
            alarm_expeditions.register(definition).unwrap();
        }
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, alarm_expeditions).unwrap();
        for _ in 0..96 {
            apply(&mut app, GameCommand::Wait);
        }
        app.walk_fixture_to(GridPos::new(52, 29)).unwrap();
        app.walk_fixture_to(GridPos::new(52, 27)).unwrap();
        apply(&mut app, GameCommand::PickUp);

        assert_eq!(app.visible_security_alarm_summary(), Some((1, 8)));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("VERROUILLAGE DE SÉCURITÉ ACTIF"))
        );
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Locked)
        );
        assert_eq!(
            app.game
                .active_facility()
                .and_then(|facility| {
                    facility.security_door_lockdown_at(GridPos::new(52, 28), app.game.turn())
                })
                .map(|lockdown| lockdown.remaining_turns(app.game.turn())),
            Some(8)
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 10);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.visible_security_alarm_summary(), Some((1, 8)));
        assert!(
            restored
                .game
                .active_facility()
                .and_then(|facility| facility
                    .security_door_lockdown_at(GridPos::new(52, 28), restored.game.turn()))
                .is_some()
        );
    }

    #[test]
    fn player_can_deliver_the_requested_material_and_replay_the_intervention() {
        let mut app = app_with_test_controls();
        let regulator: ItemId = "core:power_regulator".parse().unwrap();
        let order = "core:restore_checkpoint_power".parse().unwrap();
        app.walk_fixture_to(GridPos::new(16, 23)).unwrap();
        apply(&mut app, GameCommand::PickUp);
        assert!(
            app.game
                .player_inventory()
                .iter()
                .any(|entry| entry.item() == &regulator)
        );
        assert!(
            app.game
                .player_inventory()
                .iter()
                .find(|entry| entry.item() == &regulator)
                .and_then(|entry| entry.owner())
                .is_some()
        );
        assert!(
            app.game
                .actors()
                .iter()
                .any(|(_, actor)| !actor.observed_property_takes().is_empty())
        );
        assert!(app.game.actors().iter().any(|(_, actor)| {
            actor
                .local_alert()
                .is_some_and(|alert| alert.is_active(app.game.turn()))
        }));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("vous voit prendre"))
        );
        assert!(app.log.iter().any(|line| line.contains("ALERTE LOCALE")));
        assert_eq!(app.visible_local_alert_summary(), Some((1, 8)));
        app.walk_fixture_to(GridPos::new(27, 31)).unwrap();
        app.facing = Direction::East;
        assert_eq!(
            app.interaction_command(),
            Some(GameCommand::Interact {
                target: GridPos::new(28, 31),
            })
        );

        let before_turn = app.game.turn();
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: GridPos::new(28, 31),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(app.game.turn(), before_turn + 1);
        app.capture_events_at(Some(0.0));
        assert!(app.log.iter().any(|line| line.contains("vous livrez")));
        assert!(
            app.game
                .player_inventory()
                .iter()
                .all(|entry| entry.item() != &regulator)
        );
        let after_delivery = suspension::fingerprint(&app.game);
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: GridPos::new(28, 31),
            }),
            CommandOutcome::Rejected(CommandRejection::NoMaterialForDepot)
        );
        assert_eq!(suspension::fingerprint(&app.game), after_delivery);

        let saved = app.suspension().unwrap();
        let mut restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        for _ in 0..48 {
            apply(&mut restored, GameCommand::Wait);
        }
        assert_eq!(
            restored
                .game
                .active_facility()
                .unwrap()
                .repair_status(&order),
            Some(project_rl::facility::RepairStatus::Completed)
        );
    }

    #[test]
    fn version_five_suspensions_keep_the_pre_social_facility_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 5).unwrap();
        assert!(
            app.game
                .actors()
                .iter()
                .all(|(_, actor)| actor.affiliation().is_none()
                    && actor.witness_profile().is_none()
                    && actor.observed_property_takes().is_empty())
        );
        assert!(
            app.game
                .ground_items()
                .iter()
                .all(|(_, item)| item.owner().is_none())
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 5);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(
                &app.expeditions.without_social_metadata()
            ))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_six_suspensions_keep_the_pre_alert_social_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 6).unwrap();
        assert!(app.game.actors().iter().any(|(_, actor)| {
            actor.affiliation().is_some()
                && actor.witness_profile().is_some()
                && actor.local_alert_profile().is_none()
        }));
        app.walk_fixture_to(GridPos::new(16, 23)).unwrap();
        apply(&mut app, GameCommand::PickUp);
        assert!(app.game.actors().iter().any(|(_, actor)| {
            !actor.observed_property_takes().is_empty() && actor.local_alert().is_none()
        }));
        assert!(!app.log.iter().any(|line| line.contains("ALERTE LOCALE")));

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 6);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(
                &app.expeditions.without_local_alert_metadata()
            ))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_seven_suspensions_keep_the_pre_installed_alarm_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 7).unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        assert!(
            app.game
                .active_facility()
                .and_then(|facility| facility.installation(&sensor))
                .is_some_and(|installation| installation.security_alarm_profile().is_none())
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 7);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(
                &app.expeditions.without_security_alarm_metadata()
            ))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_eight_suspensions_keep_alarms_without_lockdown_responses() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 8).unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        let profile = app
            .game
            .active_facility()
            .and_then(|facility| facility.installation(&sensor))
            .and_then(|installation| installation.security_alarm_profile())
            .unwrap();
        assert!(profile.responses().is_empty());
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 8);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(
                &app.expeditions.without_security_alarm_response_metadata()
            ))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn legacy_suspension_is_verified_then_upgraded_and_can_be_resuspended() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut legacy = AsciiApp::from_seed_legacy(
            INITIAL_SEED,
            rules_for_generation_version(rules, 1),
            texts,
            loot,
            expeditions,
        )
        .unwrap();
        legacy.walk_fixture_to(GridPos::new(32, 16)).unwrap();
        apply(
            &mut legacy,
            GameCommand::Interact {
                target: GridPos::new(32, 15),
            },
        );
        let mut saved = legacy.suspension().unwrap();
        saved.version = 1;
        saved.state = suspension::fingerprint(legacy.game.active_game());
        let mut upgraded = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(
            upgraded.game.player_position(),
            legacy.game.player_position()
        );
        assert_eq!(upgraded.history, legacy.history);
        assert_eq!(upgraded.game.current_zone().unwrap().depth, 0);
        upgraded.walk_expedition_fixture(2).unwrap();
        let next = upgraded.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &next,
            upgraded.rules.clone(),
            upgraded.texts.clone(),
            upgraded.loot.clone(),
            upgraded.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), next.state);
        saved.state ^= 1;
        assert!(
            AsciiApp::restore_suspension(
                &saved,
                legacy.rules,
                legacy.texts,
                legacy.loot,
                legacy.expeditions
            )
            .is_err()
        );
    }

    #[test]
    fn expedition_generation_is_valid_and_reproducible_across_seeds() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        for seed in 0..64 {
            // Construction validates the finished map, both passage anchors,
            // actor placements and loot before the first accepted command.
            let a = AsciiApp::from_seed(
                seed,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
            )
            .unwrap();
            let b = AsciiApp::from_seed(
                seed,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
            )
            .unwrap();
            assert_eq!(
                suspension::fingerprint(&a.game),
                suspension::fingerprint(&b.game)
            );
            assert_eq!(a.game.visited_zone_count(), 1);
            assert!(a.game.passage(GridPos::new(67, 21)).is_some());
        }
    }

    fn blade_loot_catalog() -> LootCatalog {
        use project_rl::loot::{LootEntry, LootTable};
        let mut catalog = LootCatalog::default();
        catalog
            .register(
                LootTable::new(
                    "core:industrial_floor".parse().unwrap(),
                    vec![LootEntry {
                        item: "core:integrity_blade".parse().unwrap(),
                        weight: 1,
                        minimum_depth: 0,
                        maximum_depth: None,
                        map_kinds: vec![],
                        sources: vec![],
                        minimum_quantity: 1,
                        maximum_quantity: 1,
                    }],
                )
                .unwrap(),
            )
            .unwrap();
        catalog
    }

    #[test]
    fn weighted_loot_does_not_change_the_terrain_or_enemy_random_stream() {
        let (rules, texts, _, expeditions) = ascii_game_content().unwrap();
        let mut weighted = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            blade_loot_catalog(),
            expeditions.clone(),
            3,
        )
        .unwrap();
        let mut legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            blade_loot_catalog(),
            expeditions,
            2,
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(weighted.game.active_game()),
            suspension::fingerprint(legacy.game.active_game())
        );
        for app in [&mut weighted, &mut legacy] {
            app.walk_fixture_to(GridPos::new(66, 21)).unwrap();
            apply(
                app,
                GameCommand::Interact {
                    target: GridPos::new(67, 21),
                },
            );
        }
        assert_eq!(
            suspension::fingerprint(weighted.game.map()),
            suspension::fingerprint(legacy.game.map())
        );
        assert_eq!(
            suspension::fingerprint(weighted.game.actors()),
            suspension::fingerprint(legacy.game.actors())
        );
        assert_eq!(weighted.game.rng_state(), legacy.game.rng_state());
        assert!(
            weighted
                .game
                .ground_items()
                .iter()
                .all(|(_, stack)| stack.item().as_str() == "core:integrity_blade")
        );
        assert!(
            legacy
                .game
                .ground_items()
                .iter()
                .any(|(_, stack)| stack.item().as_str() == "core:repair_patch")
        );
    }

    #[test]
    fn version_two_suspensions_keep_fixed_loot_and_do_not_depend_on_new_tables() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut old =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 2).unwrap();
        old.walk_expedition_fixture(2).unwrap();
        let saved = old.suspension().unwrap();
        assert_eq!(saved.version, 2);
        assert_eq!(saved.loot_rules, None);
        let restored = AsciiApp::restore_suspension(
            &saved,
            old.rules.clone(),
            old.texts.clone(),
            LootCatalog::default(),
            old.expeditions.clone(),
        )
        .unwrap();
        let next = restored.suspension().unwrap();
        assert_eq!(next.version, 2);
        assert_eq!(next.loot_rules, None);
        assert_eq!(next.state, saved.state);
        assert_eq!(next.commands, saved.commands);
    }

    #[test]
    fn changed_loot_tables_preserve_a_version_three_suspension_and_active_run() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 3).unwrap();
        app.suspension_path = temporary_folder("loot-v3").join("suspended-run.json");
        app.walk_expedition_fixture(2).unwrap();
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 3);
        assert_eq!(saved.loot_rules, Some(suspension::fingerprint(&app.loot)));
        saved.write(&app.suspension_path).unwrap();
        let before = std::fs::read(&app.suspension_path).unwrap();
        let state = suspension::fingerprint(&app.game);
        app.loot = blade_loot_catalog();
        let error = app.resume_run().unwrap_err();
        assert!(error.contains("tables de butin"), "{error}");
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), before);
        assert_eq!(suspension::fingerprint(&app.game), state);
        let mut malformed = saved.clone();
        malformed.loot_rules = None;
        assert!(malformed.validate().is_err());
        malformed = saved;
        malformed.version = 2;
        assert!(malformed.validate().is_err());
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn changed_world_definitions_preserve_a_version_four_suspension_and_active_run() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 4).unwrap();
        app.suspension_path = temporary_folder("world-v4").join("suspended-run.json");
        app.walk_expedition_fixture(2).unwrap();
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 4);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(
                &app.expeditions.without_facilities()
            ))
        );
        saved.write(&app.suspension_path).unwrap();
        let before = std::fs::read(&app.suspension_path).unwrap();
        let state = suspension::fingerprint(&app.game);

        let mut changed = ExpeditionCatalog::default();
        for (id, definition) in app.expeditions.iter() {
            if id.as_str() == "core:starter_expedition" {
                let mut destination = definition.destination.clone();
                destination.seed_salt ^= 1;
                changed
                    .register(
                        project_rl::content::ExpeditionDefinition::new(
                            id.clone(),
                            definition.hub.clone(),
                            destination,
                            definition.hub_passage,
                        )
                        .unwrap(),
                    )
                    .unwrap();
            } else {
                changed.register(definition.clone()).unwrap();
            }
        }
        app.expeditions = changed;

        let error = app.resume_run().unwrap_err();
        assert!(error.contains("définitions du monde"), "{error}");
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), before);
        assert_eq!(suspension::fingerprint(&app.game), state);

        let mut malformed = saved.clone();
        malformed.world_rules = None;
        assert!(malformed.validate().is_err());
        malformed = saved;
        malformed.version = 3;
        assert!(malformed.validate().is_err());
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn report_has_exactly_one_toggle_and_never_advances_simulation() {
        let mut app = app_with_test_controls();
        let before = (
            app.game.turn(),
            app.game.rng_state(),
            app.game.player_energy(),
        );
        app.update_input(&input("O"));
        assert!(!app.report_open); // No report has been produced yet.
        app.observation_report.push("Relevé daté".to_owned());
        app.update_input(&input("O"));
        assert!(app.report_open);
        for key in ["Enter", "KpEnter", "C", "L", "B", "P", "Semicolon", "Space"] {
            app.update_input(&input(key));
            assert!(app.report_open);
        }
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        assert!(app.report_open);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert!(app.report_open); // Pause overlays the report; it does not toggle it.
        app.update_input(&input("O"));
        assert!(!app.report_open);
        app.controls
            .rebind(Action::Report, Binding::key("F3"))
            .unwrap();
        app.update_input(&input("O"));
        assert!(!app.report_open);
        app.update_input(&input("F3"));
        assert!(app.report_open);
        app.update_input(&input("O"));
        assert!(app.report_open);
        app.update_input(&input("F3"));
        assert!(!app.report_open);
        assert_eq!(
            (
                app.game.turn(),
                app.game.rng_state(),
                app.game.player_energy()
            ),
            before
        );
    }

    #[test]
    fn azerty_movement_uses_one_physical_position_and_no_qwerty_aliases() {
        let mut app = app_with_test_controls();
        app.game = WorldState::single(
            GameState::new(
                Map::from_ascii("#####\n#...#\n#...#\n#...#\n#####").unwrap(),
                GridPos::new(2, 2),
                7,
            )
            .unwrap(),
        );
        assert_eq!(
            read_movement_command(&app.game, 0, &app.controls, &input("W")),
            Some(GameCommand::Move(Direction::North))
        );
        assert_eq!(
            read_movement_command(&app.game, 0, &app.controls, &input("A")),
            Some(GameCommand::Move(Direction::West))
        );
        for key in ["Z", "Q", "Up", "Left", "Period"] {
            assert_eq!(
                read_movement_command(&app.game, 0, &app.controls, &input(key)),
                None
            );
        }
        assert!(app.controls.pressed(Action::Multiple, &input("Semicolon")));
        assert!(!app.controls.pressed(Action::Multiple, &input("M")));
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Key1")), Some(0));
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Kp1")), None);
        app.controls
            .rebind(Action::Slot1, Binding::key("Kp1"))
            .unwrap();
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Key1")), None);
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Kp1")), Some(0));
    }

    #[test]
    fn a_new_run_equips_and_selects_the_flamethrower_on_channel_three() {
        let mut app = app_with_test_controls();

        assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);
        assert_eq!(
            app.game
                .equipped_player_weapon(2)
                .map(|weapon| weapon.id().as_str()),
            Some("core:flamethrower")
        );
        let channels = app.combat_channels_label();
        assert!(channels.contains("[3] Lance-flammes industriel"));
        assert!(channels.starts_with(">[1]"));

        app.update_input(&input("Key3"));
        assert_eq!(app.active_weapon_slot, 2);
        assert!(
            app.combat_channels_label()
                .contains(">[3] Lance-flammes industriel")
        );
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("Lance-flammes industriel"))
        );
    }

    #[test]
    fn area_aiming_moves_a_preview_without_time_and_commits_an_empty_tile() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        app.active_weapon_slot = 2;
        app.facing = Direction::East;
        let turn_before_aiming = app.game.turn();
        let history_before_aiming = app.history.len();

        app.update_input(&input("F"));

        let initial = app.attack_aim.expect("area weapon should enter aim mode");
        assert_eq!(initial.slot, 2);
        assert!(
            app.game
                .player_attack_preview(initial.slot, initial.cursor)
                .is_ok()
        );
        assert_eq!(app.game.turn(), turn_before_aiming);
        assert_eq!(app.history.len(), history_before_aiming);

        app.update_input(&input("A"));
        let moved = app.attack_aim.expect("movement must keep aim mode open");
        assert_eq!(moved.cursor, initial.cursor.step(Direction::West));
        assert_eq!(app.game.actors().entity_at(moved.cursor), None);
        assert_eq!(app.game.turn(), turn_before_aiming);
        assert_eq!(app.history.len(), history_before_aiming);

        let mut empty_cursor = None;
        'rows: for y in 0..app.game.map().height() as i32 {
            for x in 0..app.game.map().width() as i32 {
                let candidate = GridPos::new(x, y);
                if !app.game.player_visibility().is_visible(candidate) {
                    continue;
                }
                let Ok(preview) = app.game.player_attack_preview(2, candidate) else {
                    continue;
                };
                if preview
                    .cells()
                    .iter()
                    .all(|cell| app.game.actors().entity_at(cell.position).is_none())
                {
                    empty_cursor = Some(candidate);
                    break 'rows;
                }
            }
        }
        let empty_cursor = empty_cursor.expect("fixture needs a valid cone without any actor");
        app.attack_aim = Some(AttackAim {
            slot: 2,
            cursor: empty_cursor,
        });

        // A real key must be released before Macroquad emits its next pressed
        // edge. Keep this empty frame in the regression test so confirmation
        // exercises the same sequence as live input.
        app.update_input(&InputFrame::default());
        app.update_input(&input("F"));

        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn_before_aiming + 1);
        assert!(app.visual_cues.active_count() > 0);
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("ATTAQUE CONFIRMÉE") && line.contains("Lance-flammes")),
            "successful empty attack log missing from {:?}",
            app.log
        );
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::AttackAt { slot: 2, x, y })
                if *x == empty_cursor.x && *y == empty_cursor.y
        ));
        assert_eq!(
            app.history.last().unwrap().command(&app.game),
            Ok(GameCommand::AttackAt {
                slot: 2,
                target: empty_cursor,
            })
        );
    }

    #[test]
    fn pointer_attack_starts_area_aim_on_the_clicked_empty_cell_without_time() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        app.active_weapon_slot = 2;
        let turn_before = app.game.turn();
        let history_before = app.history.len();
        let clicked = GridPos::new(70, 20);
        assert!(app.game.player_visibility().is_visible(clicked));
        assert_eq!(app.game.actors().entity_at(clicked), None);

        assert!(app.begin_pointer_attack_aim(clicked, Some((400.0, 300.0))));

        assert_eq!(
            app.attack_aim,
            Some(AttackAim {
                slot: 2,
                cursor: clicked,
            })
        );
        assert_eq!(app.game.turn(), turn_before);
        assert_eq!(app.history.len(), history_before);
    }

    #[test]
    fn escape_cancels_area_aiming_before_opening_the_pause_menu() {
        let mut app = app_with_test_controls();
        app.active_weapon_slot = 2;
        let before = suspension::fingerprint(&app.game);

        app.update_input(&input("F"));
        assert!(app.attack_aim.is_some());
        app.update_input(&input("Escape"));

        assert!(app.attack_aim.is_none());
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn options_change_layout_and_rebind_without_touching_the_run() {
        let mut app = app_with_test_controls();
        // Use native backend semantics for the persisted fixture.
        app.controls = Controls::preset(Layout::Azerty, KeySemantics::native());
        let folder = std::env::temp_dir().join(format!(
            "project-rl-options-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        app.controls_path = folder.join("controls.json");
        let before = (
            app.game.turn(),
            app.game.rng_state(),
            app.game.player_energy(),
            app.game.export_player_progression().unwrap(),
        );
        let options = InputFrame {
            pause: true,
            ..Default::default()
        };
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Pause);
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Controls);
        app.update_input(&input("Enter"));
        assert_eq!(app.controls.layout, Layout::Qwerty);
        app.options_selection = Action::ALL
            .iter()
            .position(|action| *action == Action::Report)
            .unwrap()
            + 1;
        app.update_input(&input("Enter"));
        assert!(app.rebinding);
        app.update_input(&InputFrame {
            pressed: [Binding::MouseRight].into(),
            ..Default::default()
        });
        assert!(!app.rebinding);
        assert_eq!(app.controls.binding(Action::Report), &Binding::MouseRight);
        assert_eq!(
            Controls::load(&app.controls_path, Some(Layout::Azerty)).0,
            app.controls
        );
        app.update_input(&input("Space"));
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&options);
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(
            (
                app.game.turn(),
                app.game.rng_state(),
                app.game.player_energy(),
                app.game.export_player_progression().unwrap()
            ),
            before
        );
        let saved = app.controls.clone();
        app.update_input(&input("R"));
        assert_eq!(app.controls, saved);
        std::fs::remove_file(&app.controls_path).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }

    #[test]
    fn ascii_content_targets_and_dated_reports_work_without_a_graphics_context() {
        let mut app = app_with_test_controls();
        assert_eq!(app.game.player_progression().unspent_skill_points(), 2);
        assert_eq!(app.game.player_energy().available(), 100);
        let multiple = technique_id("core:rec_09").unwrap();
        assert_eq!(app.technique_name(&multiple), "Analyse multiple");
        for (_, definition) in app.game.rules().skills.techniques() {
            assert!(
                app.texts
                    .resolve(DISPLAY_LOCALE, definition.description_key())
                    .is_some()
            );
        }

        let mut rules = app.rules.clone();
        rules.progression.starting_skill_points = 9;
        rules.player_field_of_view.radius = 4;
        app.game = WorldState::single(
            GameState::new_with_rules(
                Map::from_ascii("#########\n#.......#\n#.......#\n#########").unwrap(),
                GridPos::new(1, 1),
                7,
                rules,
            )
            .unwrap(),
        );
        for id in [
            "core:rec_01",
            "core:rec_02",
            "core:rec_09",
            "core:rec_04",
            "core:rec_05",
        ] {
            assert_eq!(
                app.game
                    .process_player_command(GameCommand::LearnTechnique {
                        technique: technique_id(id).unwrap(),
                    }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        let targets: Vec<_> = [2, 3, 4, 7]
            .into_iter()
            .map(|x| {
                app.game
                    .spawn_actor(Actor::new(GridPos::new(x, 1), 12).unwrap())
                    .unwrap()
            })
            .collect();
        app.game.drain_events();
        app.selected_target = Some(targets[2]);
        let command = app.technique_command(multiple.clone()).unwrap();
        assert_eq!(
            command,
            GameCommand::UseTechnique {
                technique: multiple.clone(),
                targets: vec![targets[2], targets[0], targets[1]],
            }
        );
        assert_eq!(
            app.game.process_player_command(command),
            CommandOutcome::Applied
        );
        app.capture_events();
        assert!(!app.report_open);
        assert_eq!(app.observation_report.len(), 5);
        assert!(app.observation_report[0].contains("Analyse multiple — relevé du tour 0"));
        assert!(app.observation_report[1].contains("-2 E ; réserve 98 E"));
        assert!(app.observation_report[2].contains("(4, 1)"));
        assert!(
            !app.observation_report
                .iter()
                .any(|line| line.contains("(7, 1)"))
        );
        let snapshot = app.observation_report.clone();
        app.game.process_player_command(GameCommand::Wait);
        app.capture_events();
        assert_eq!(app.observation_report, snapshot);

        // A stale selection cannot add a hidden target to a new command.
        app.selected_target = Some(targets[3]);
        assert_eq!(
            app.technique_command(multiple.clone()),
            Some(GameCommand::UseTechnique {
                technique: multiple,
                targets: targets[..3].to_vec(),
            })
        );
        let command = app.target_analysis_command().unwrap();
        assert_eq!(
            app.game.process_player_command(command),
            CommandOutcome::Applied
        );
        app.capture_events();
        assert_eq!(app.observation_report.len(), 2);
        assert!(app.observation_report[0].contains("Analyse de cible — relevé du tour 2"));
    }
}
