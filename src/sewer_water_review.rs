//! EG-EAU01: native review of connected water and damp banks, Debug only.
//! Historical comparison of the accepted water and channel layout.
use crate::{prison_street_render, sewer::Plan, terminal_view::GridCamera};
use macroquad::prelude::*;
use project_rl::{
    game::{GameState, WorldState, ZoneInfo},
    world::{GridPos, Map, Terrain},
};
use std::{cell::Cell, io::Write, path::Path};

const W: u32 = 640;
const H: u32 = 448;
const BOARD_W: u32 = 1360;
const BOARD_H: u32 = 672;
const FPS: usize = 20;
const FRAMES: usize = 80;
thread_local! {
    static ENABLED: Cell<bool> = const {Cell::new(false)};
    static CANALS: Cell<bool> = const {Cell::new(false)};
    static NETWORK: Cell<bool> = const {Cell::new(false)};
}
pub fn enabled() -> bool {
    ENABLED.get()
}
pub fn active() -> bool {
    CAPTURING.get()
}

fn candidate(index: usize) -> &'static Plan {
    crate::sewer::canal_candidate(index)
}

pub fn water(camera: &GridCamera, plan: &Plan, time: f32) {
    let base = plan;
    let expanded = CANALS.get() && NETWORK.get();
    let plan = if expanded {
        candidate(
            (1..=3)
                .find(|&i| crate::sewer::plan(i).seed == plan.seed)
                .expect("Known sewer seed"),
        )
    } else {
        plan
    };
    crate::sewer_water::draw(camera, base, plan, time);
}

struct Study {
    game: WorldState,
    proposed_game: Option<WorldState>,
    camera: GridCamera,
    panels: [RenderTarget; 2],
    index: usize,
}
impl Study {
    fn new(index: usize) -> Result<Self, String> {
        let plan = crate::sewer::plan(index);
        let mut map =
            Map::filled(plan.width, plan.height, Terrain::Wall).map_err(|e| e.to_string())?;
        for (y, row) in plan.layout.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                let at = GridPos::new(x as i32, y as i32);
                if c != b'#' {
                    map.set_terrain(at, Terrain::Floor)
                        .map_err(|e| e.to_string())?;
                }
                if c == b'~' {
                    map.set_obstruction(at, true, false)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        for at in &plan.blocked {
            map.set_obstruction(GridPos::new(at[0], at[1]), true, false)
                .map_err(|e| e.to_string())?;
        }
        let focus = if CANALS.get() {
            if index == 3 {
                GridPos::new(13, 35)
            } else {
                GridPos::new(22, 19)
            }
        } else if index == 2 {
            GridPos::new(31, 17)
        } else {
            let view = plan
                .views
                .iter()
                .find(|v| v.id == "collector")
                .ok_or("Collector view missing")?;
            GridPos::new(view.at[0], view.at[1])
        };
        let game = GameState::new(map, focus, 206027).map_err(|e| e.to_string())?;
        let mut game = WorldState::single(game);
        game.enable(ZoneInfo {
            id: format!("core:prison_sewer_0{index}").parse().unwrap(),
            name: "Égouts".into(),
            kind: "core:dungeon".parse().unwrap(),
            depth: 1,
        })?;
        let proposed_game = if CANALS.get() {
            let map = crate::sewer::canal_candidate_map(index)?;
            for at in plan
                .groups
                .iter()
                .flat_map(|g| g.members.iter().map(|m| m.at))
                .chain(plan.views.iter().map(|v| v.at))
                .chain([plan.spawn])
            {
                if !map.is_walkable(GridPos::new(at[0], at[1])) {
                    return Err("Candidate canal blocks an actor, viewpoint or arrival".into());
                }
            }
            let mut sample =
                WorldState::single(GameState::new(map, focus, 206027).map_err(|e| e.to_string())?);
            sample.enable(game.current_zone().unwrap().clone())?;
            Some(sample)
        } else {
            None
        };
        let panels = std::array::from_fn(|_| {
            let rt = render_target(W, H);
            rt.texture.set_filter(FilterMode::Nearest);
            rt
        });
        Ok(Self {
            game,
            proposed_game,
            camera: GridCamera::new(Rect::new(0., 0., W as f32, H as f32), 64., focus),
            panels,
            index,
        })
    }
    fn panel(&self, proposed: bool, time: f32, reduced: bool) {
        let rt = &self.panels[usize::from(proposed)];
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., W as f32, H as f32));
        camera.render_target = Some(rt.clone());
        set_camera(&camera);
        clear_background(BLACK);
        unsafe {
            get_internal_gl().quad_gl.scissor(None);
        }
        let previous = ENABLED.replace(proposed || CANALS.get());
        let network = NETWORK.replace(proposed);
        let game = if proposed {
            self.proposed_game.as_ref().unwrap_or(&self.game)
        } else {
            &self.game
        };
        {
            let _world = crate::world_readability::world(false);
            // Only water's clock changes in this comparison, to isolate it
            // from lamp flicker, decorative rats and actor breathing.
            let before_time = if reduced { 0. } else { time };
            WATER_CLOCK.set(before_time);
            prison_street_render::draw(&self.camera, game, |_| true, 1.);
            let at = game.player_position().unwrap();
            crate::game_art::draw_world(
                self.camera.rect(at),
                crate::game_art::player(
                    Some("core:bastion"),
                    crate::terminal_view::PlayerWeaponPose::Melee,
                    Some("core:melee_hachette"),
                ),
                1.,
            );
            prison_street_render::finish(&self.camera, Rect::new(0., 0., W as f32, H as f32), game);
            prison_street_render::roofs(&self.camera, game, |_| true);
        }
        ENABLED.set(previous);
        NETWORK.set(network);
    }
    fn render(&self, time: f32, reduced: bool) {
        self.panel(false, time, reduced);
        self.panel(true, time, reduced);
        set_default_camera();
        unsafe {
            get_internal_gl().quad_gl.scissor(None);
        }
        clear_background(Color::from_rgba(9, 13, 18, 255));
        let ink = Color::from_rgba(218, 228, 234, 255);
        let muted = Color::from_rgba(153, 174, 186, 255);
        crate::ui_theme::draw_text(
            if CANALS.get() {
                "EG-CAN02 · Égouts — réseau de canaux"
            } else {
                "EG-EAU01 · Égouts — eau et berges"
            },
            24.,
            32.,
            25.,
            ink,
        );
        let labels = if CANALS.get() {
            [
                "01 · Anciens canaux, nouvelle eau",
                "02 · Canaux reliés et nouvelles traversées",
            ]
        } else {
            ["01 · Rendu actuel", "02 · Courant lent et canaux continus"]
        };
        for (i, label) in labels.iter().enumerate() {
            let x = 24. + i as f32 * 672.;
            crate::ui_theme::draw_text(label, x, 70., 20., if i == 0 { muted } else { ink });
            draw_texture_ex(
                &self.panels[i].texture,
                x,
                90.,
                WHITE,
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            );
        }
        crate::ui_theme::draw_text(
            &format!(
                "Variante {} · cases de 64 px · mêmes décors, acteur et éclairage",
                self.index
            ),
            24.,
            568.,
            18.,
            ink,
        );
        crate::ui_theme::draw_text(
            if CANALS.get() {
                "02 : davantage de galeries d'eau, ponts sur grilles et passages secs reliés"
            } else {
                "02 : filet continu, petites rides lentes, eau assombrie contre les berges"
            },
            24.,
            596.,
            18.,
            muted,
        );
        crate::ui_theme::draw_text(
            "1 / 2 / 3 : carte   ·   M : mouvement réduit   ·   Échap : fermer   ·   Aperçu avant intégration",
            24.,
            640.,
            17.,
            muted,
        );
    }
}

// Fixed lamp/rat clocks in this controlled study; ordinary rendering never
// reads this override. Its absence preserves the caller's real ambient clock.
thread_local! {static WATER_CLOCK:Cell<f32>=const{Cell::new(0.)};static CAPTURING:Cell<bool>=const{Cell::new(false)};}
pub fn time(time: f32) -> f32 {
    if CAPTURING.get() {
        WATER_CLOCK.get()
    } else {
        time
    }
}

pub async fn run(output: Option<&Path>) -> Result<(), String> {
    CANALS.set(matches!(
        std::env::args().nth(1).as_deref(),
        Some("--apercu-canaux-egouts" | "--ui-cold-sewer-canals")
    ));
    request_new_screen_size(BOARD_W as f32, BOARD_H as f32);
    for _ in 0..4 {
        next_frame().await;
    }
    CAPTURING.set(true);
    if let Some(output) = output {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut checks = Vec::new();
        for index in 1..=3 {
            let study = Study::new(index)?;
            let before = study.game.recovery_snapshot_bytes()?;
            let proposed_before = study
                .proposed_game
                .as_ref()
                .map(WorldState::recovery_snapshot_bytes)
                .transpose()?;
            study.render(1., false);
            crate::ui_capture::framebuffer()?.export_png(
                output
                    .join(format!("variant-{index}-native64.png"))
                    .to_str()
                    .ok_or("Capture path")?,
            );
            next_frame().await;
            if CANALS.get() {
                for proposed in [false, true] {
                    study.panel(proposed, 1., false);
                    prison_street_render::export_review_world(
                        if proposed {
                            study.proposed_game.as_ref().unwrap()
                        } else {
                            &study.game
                        },
                        &output.join(format!(
                            "variant-{index}-{}-world-native64.png",
                            if proposed { "proposed" } else { "before" }
                        )),
                    )?;
                }
            }
            if before != study.game.recovery_snapshot_bytes()? {
                return Err("Rendering changed simulation".into());
            }
            if proposed_before
                != study
                    .proposed_game
                    .as_ref()
                    .map(WorldState::recovery_snapshot_bytes)
                    .transpose()?
            {
                return Err("Drawing changed candidate state".into());
            }
            checks.push(serde_json::json!({"variant":index,"state_unchanged":true}));
        }
        let study = Study::new(2)?;
        let before = study.game.recovery_snapshot_bytes()?;
        let raw_path = output.join("sequence.rgba");
        let mut raw = std::fs::File::create(&raw_path).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            study.render(frame as f32 / FPS as f32, false);
            let image = crate::ui_capture::framebuffer()?;
            if image.width as u32 != BOARD_W || image.height as u32 != BOARD_H {
                return Err("Native capture dimensions changed".into());
            }
            raw.write_all(&image.bytes).map_err(|e| e.to_string())?;
            next_frame().await;
        }
        study.render(1., true);
        let still_a = crate::ui_capture::framebuffer()?;
        study.render(3., true);
        let still_b = crate::ui_capture::framebuffer()?;
        if still_a.bytes != still_b.bytes {
            return Err("Reduced motion is not static".into());
        }
        still_a.export_png(
            output
                .join("reduced-motion.png")
                .to_str()
                .ok_or("Capture path")?,
        );
        if before != study.game.recovery_snapshot_bytes()? {
            return Err("Animation changed simulation".into());
        }
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":if CANALS.get(){"EG-CAN02-r01"}else{"EG-EAU01-r01"},"status":"awaiting_visual_approval","normal_game_activation":false,"native_cell":64,"width":BOARD_W,"height":BOARD_H,"panel_width":W,"panel_height":H,"panel_origins":[[24,90],[696,90]],"frames":FRAMES,"fps":FPS,"variants":checks,"game_state_unchanged":true,"reduced_motion_static":true,"source_art_unchanged":true,"saved_campaign_access":false,"settings_written":false,"scope":"Controlled native crop of production sewer renderer; fixed lamps, rats and actor; no full HUD or combat simulation"})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    } else {
        let mut study = Study::new(2)?;
        let started = get_time();
        let mut reduced = false;
        loop {
            if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
                break;
            }
            for (key, index) in [(KeyCode::Key1, 1), (KeyCode::Key2, 2), (KeyCode::Key3, 3)] {
                if is_key_pressed(key) {
                    study = Study::new(index)?;
                }
            }
            if is_key_pressed(KeyCode::M) {
                reduced = !reduced;
            }
            study.render((get_time() - started) as f32, reduced);
            next_frame().await;
        }
    }
    CAPTURING.set(false);
    Ok(())
}
