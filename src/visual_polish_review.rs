//! POLISH01: isolated native-size street study. Never enters a saved campaign.
//! The production scenery/actor/effect renderers supply both comparison columns.
use crate::{combat_visuals::CombatMotion, game_art, prison_street_render as street_render};
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use project_rl::{
    entity::{Actor, EntityId},
    game::{GameState, WorldState},
    social::PlayerRelation,
    world::{GridPos, Map},
};
use std::{io::Write, path::Path};

const W: u32 = 640;
const H: u32 = 576;
const BOARD_W: u32 = 1360;
const BOARD_H: u32 = 800;
const FPS: usize = 25;
const FRAMES: usize = 75;
const HERO: GridPos = GridPos::new(12, 54);
const VICTIM: GridPos = GridPos::new(13, 54);
const OTHER: GridPos = GridPos::new(15, 55);
const WALK_START: f64 = 0.15;
const WALK_END: f64 = 0.55;
const ATTACK: f64 = 1.05;

const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;
void main(){uv=texcoord;gl_Position=Projection*Model*vec4(position,1.0);}"#;
const COMPOSE: &str = r#"#version 100
precision highp float;varying highp vec2 uv;
uniform sampler2D Texture;uniform sampler2D Actors;uniform sampler2D Irradiance;
uniform vec2 Texel;uniform vec2 WorldOrigin;uniform vec2 WorldSize;
uniform float Proposed;uniform float Contact;
float lum(vec3 c){return dot(c,vec3(0.2126,0.7152,0.0722));}
void main(){
 vec3 ground=texture2D(Texture,uv).rgb;
 vec3 near1=texture2D(Texture,uv+vec2(Texel.x,0.0)).rgb;
 vec3 near2=texture2D(Texture,uv-vec2(Texel.x,0.0)).rgb;
 vec3 near3=texture2D(Texture,uv+vec2(0.0,Texel.y)).rgb;
 vec3 near4=texture2D(Texture,uv-vec2(0.0,Texel.y)).rgb;
 float variation=max(max(abs(lum(near1)-lum(ground)),abs(lum(near2)-lum(ground))),max(abs(lum(near3)-lum(ground)),abs(lum(near4)-lum(ground))));
 // Only low-contrast surface grain: preserve joints, cracks and object contours.
 float quiet=(1.0-smoothstep(0.025,0.065,variation))*0.26*Proposed;
 ground=mix(ground,(ground*4.0+near1+near2+near3+near4)/8.0,quiet);
 vec2 local=vec2(uv.x,1.0-uv.y)*vec2(640.0,576.0);
 vec2 world=(WorldOrigin+local)/WorldSize;
 vec3 light=texture2D(Irradiance,world).rgb*2.0;
 vec3 graded=max(vec3(0.0),(ground-0.17)*1.24+0.17);
 graded=mix(vec3(lum(graded)),graded,1.09);
 float shade=1.0-smoothstep(0.08,0.35,lum(ground));
 graded+=vec3(-0.006,0.003,0.017)*shade*smoothstep(0.005,0.08,lum(ground));
 // Strengthen the existing occluded lamp footprints, not a painted global glow.
 float warm=smoothstep(0.015,0.28,light.r-light.b);
 graded*=mix(vec3(1.0),vec3(1.17,1.09,1.015),warm);
 ground=mix(ground,graded,Proposed);
 vec4 actor=texture2D(Actors,uv);
 // The same approved irradiance lights the unmodified composed actors.
 vec3 body=actor.rgb*light;
 float bodyValue=lum(actor.rgb)/max(actor.a,0.001);
 float solid=smoothstep(0.04,0.15,bodyValue)*actor.a;
 float rim=1.0-min(texture2D(Actors,uv-vec2(Texel.x,0.0)).a,texture2D(Actors,uv+vec2(0.0,Texel.y)).a);
 // Preserve pigment ratios and dark outlines instead of adding a gray veil.
 body=mix(body,body*1.48,Proposed);
 vec3 pigment=actor.rgb/max(max(actor.r,max(actor.g,actor.b)),0.001);
 body+=Proposed*solid*rim*pigment*0.035;
 vec3 result=ground*(1.0-actor.a)+body;
 // Brief contact light, on the adjacent road only; no wall/room illumination.
 vec2 strike=local-vec2(399.0,296.0);
 float pool=exp(-dot(strike/vec2(66.0,29.0),strike/vec2(66.0,29.0)));
 result+=Proposed*Contact*pool*vec3(0.17,0.075,0.017);
 gl_FragColor=vec4(clamp(result,0.0,1.0),1.0);
}"#;

struct Study {
    game: WorldState,
    target: EntityId,
    background: RenderTarget,
    actors: RenderTarget,
    columns: [RenderTarget; 2],
    material: Material,
    _irradiance: Texture2D,
    camera: crate::terminal_view::GridCamera,
}

fn target() -> RenderTarget {
    let rt = render_target(W, H);
    rt.texture.set_filter(FilterMode::Nearest);
    rt
}
fn camera(rt: &RenderTarget) {
    let mut cam = Camera2D::from_display_rect(Rect::new(0., 0., W as f32, H as f32));
    cam.render_target = Some(rt.clone());
    set_camera(&cam);
    unsafe {
        get_internal_gl().quad_gl.scissor(None);
    }
}
fn contact() -> f64 {
    ATTACK + 0.22 * crate::animation_policy::duration_scale("B01")
}

impl Study {
    fn new() -> Result<Self, String> {
        let mut map = Map::from_ascii(include_str!(
            "../assets/maps/prison-street-expanded-01/layout.txt"
        ))
        .map_err(|e| e.to_string())?;
        for &at in &crate::prison_street::expanded_scene().obstructions {
            map.set_obstruction(crate::prison_street::pos(at), true, false)
                .map_err(|e| e.to_string())?;
        }
        for at in [HERO, VICTIM, OTHER, GridPos::new(11, 54)] {
            if !map.is_walkable(at) {
                return Err(format!("Study actor is not on walkable ground: {at:?}"));
            }
        }
        let mut game = GameState::new(map, HERO, 2701).map_err(|e| e.to_string())?;
        let mut target_id = None;
        for (at, number) in [(VICTIM, 120), (OTHER, 14)] {
            if !game.player_visibility().is_visible(at) {
                return Err(format!("Study enemy is outside real visibility: {at:?}"));
            }
            let actor = Actor::new(at, 100)
                .map_err(|e| e.to_string())?
                .with_player_relation(PlayerRelation::Hostile)
                .with_tags([format!("core:monster_art_{number}_0").parse().unwrap()]);
            let id = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            if at == VICTIM {
                target_id = Some(id);
            }
        }
        game.drain_events();
        let bytes =
            project_rl::data_paths::read("assets/maps/prison-street-expanded-01/irradiance.png")?;
        let irradiance = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
        irradiance.set_filter(FilterMode::Linear);
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: COMPOSE,
            },
            MaterialParams {
                textures: vec!["Actors".into(), "Irradiance".into()],
                uniforms: vec![
                    UniformDesc::new("Texel", UniformType::Float2),
                    UniformDesc::new("WorldOrigin", UniformType::Float2),
                    UniformDesc::new("WorldSize", UniformType::Float2),
                    UniformDesc::new("Proposed", UniformType::Float1),
                    UniformDesc::new("Contact", UniformType::Float1),
                ],
                ..Default::default()
            },
        )
        .map_err(|e| e.to_string())?;
        material.set_texture("Irradiance", irradiance.clone());
        material.set_uniform("Texel", (1. / W as f32, 1. / H as f32));
        let grid =
            crate::terminal_view::GridCamera::new(Rect::new(0., 0., W as f32, H as f32), 64., HERO);
        material.set_uniform(
            "WorldOrigin",
            (grid.first.x as f32 * 64., grid.first.y as f32 * 64.),
        );
        material.set_uniform("WorldSize", (26.0_f32 * 64., 66.0_f32 * 64.));
        Ok(Self {
            game: WorldState::single(game),
            target: target_id.unwrap(),
            background: target(),
            actors: target(),
            columns: [target(), target()],
            material,
            _irradiance: irradiance,
            camera: grid,
        })
    }

    fn motion(&self) -> CombatMotion {
        let mut m = CombatMotion::default();
        m.glide_timed(
            self.game.player_id(),
            GridPos::new(11, 54),
            HERO,
            WALK_START,
            400,
        );
        m.play_rupture(HERO, VICTIM, ATTACK);
        m.play(self.game.player_id(), HERO, VICTIM, false, ATTACK);
        m.play(self.target, HERO, VICTIM, true, contact());
        m
    }

    fn render(&self, time: f64, reduced: bool) {
        // Preserve the exact approved pre-integration reference and proposal.
        let _legacy = crate::visual_polish::review(false);
        let _clock = crate::effect_geometry::frame_time(time);
        let motion = self.motion();
        let bounds = Rect::new(0., 0., W as f32, H as f32);
        camera(&self.background);
        clear_background(BLACK);
        {
            let _world = crate::world_readability::world(false);
            street_render::draw(
                &self.camera,
                &self.game,
                |at| self.game.player_visibility().is_visible(at),
                time as f32,
            );
            street_render::finish(&self.camera, bounds, &self.game);
            street_render::roofs(&self.camera, &self.game, |at| {
                self.game.player_visibility().is_visible(at)
            });
        }
        for column in 0..2 {
            let proposed = column == 1;
            camera(&self.actors);
            clear_background(BLANK);
            {
                let _world = crate::world_readability::world(false);
                for (at, visual, id) in [
                    (
                        HERO,
                        game_art::player(
                            Some("core:bastion"),
                            crate::terminal_view::PlayerWeaponPose::Melee,
                            Some("core:melee_hachette"),
                        ),
                        Some(self.game.player_id()),
                    ),
                    (
                        VICTIM,
                        game_art::Visual::CampaignMonster {
                            number: 120,
                            variant: 0,
                        },
                        Some(self.target),
                    ),
                    (
                        OTHER,
                        game_art::Visual::CampaignMonster {
                            number: 14,
                            variant: 0,
                        },
                        None,
                    ),
                ] {
                    let rect = self.camera.rect(at);
                    let walking = (WALK_START..WALK_END).contains(&time) && !reduced;
                    let rect = if let Some(id) = id {
                        motion.actor_rect(id, rect, time, reduced)
                    } else {
                        rect
                    };
                    let pose = if at == HERO && walking {
                        motion.player_walk_pose(self.game.player_id(), time, false)
                    } else {
                        None
                    };
                    game_art::walking::with_pose(pose, || {
                        game_art::draw_world_animated(rect, visual, 1., Some(time))
                    });
                }
            }
            camera(&self.columns[column]);
            clear_background(BLACK);
            self.material
                .set_texture("Actors", self.actors.texture.clone());
            self.material
                .set_uniform("Proposed", if proposed { 1.0_f32 } else { 0.0_f32 });
            let since = (time - contact()) as f32;
            let flash = if !reduced && (0.0..0.16).contains(&since) {
                (1. - since / 0.16).powi(2)
            } else {
                0.
            };
            self.material.set_uniform("Contact", flash);
            gl_use_material(&self.material);
            draw_texture_ex(
                &self.background.texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            );
            gl_use_default_material();
            let rect = |at: GridPos| Some(self.camera.rect(at));
            motion.draw_spells(self.game.active_game(), time, reduced, bounds, rect);
            if proposed && !reduced {
                self.contact_particles(since);
            }
        }
        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
        unsafe {
            get_internal_gl().quad_gl.scissor(None);
        }
        clear_background(Color::from_rgba(9, 13, 18, 255));
        let text = Color::from_rgba(217, 225, 232, 255);
        let muted = Color::from_rgba(139, 155, 168, 255);
        crate::ui_theme::draw_text_bold("POLISH01 · Finition de la rue", 24., 32., 25., text);
        crate::ui_theme::draw_text(
            "Comparaison de rendu · cases et silhouettes à taille native 64 px",
            24.,
            58.,
            17.,
            muted,
        );
        for (column, title) in ["01 · Référence actuelle", "02 · Finition proposée"]
            .iter()
            .enumerate()
        {
            let x = 24. + column as f32 * 672.;
            crate::ui_theme::draw_text_bold(
                title,
                x,
                87.,
                20.,
                if column == 0 {
                    muted
                } else {
                    Color::from_rgba(224, 183, 116, 255)
                },
            );
            draw_texture_ex(
                &self.columns[column].texture,
                x,
                100.,
                WHITE,
                DrawTextureParams {
                    flip_y: true,
                    ..Default::default()
                },
            );
            draw_rectangle_lines(
                x - 1.,
                99.,
                W as f32 + 2.,
                H as f32 + 2.,
                1.,
                Color::from_rgba(42, 53, 65, 255),
            );
        }
        crate::ui_theme::draw_text(
            "Même lieu · mêmes personnages · même arme · même animation de compétence",
            24.,
            705.,
            18.,
            text,
        );
        crate::ui_theme::draw_text(
            "02 : grain discret, volumes mieux détachés, liseré intérieur, lumière et éclats au contact",
            24.,
            732.,
            17.,
            muted,
        );
        crate::ui_theme::draw_text(
            "Espace : rejouer     M : mouvement réduit     Échap : fermer     Proposition séparée du jeu normal",
            24.,
            773.,
            16.,
            muted,
        );
    }

    fn contact_particles(&self, age: f32) {
        if !(0.0..0.24).contains(&age) {
            return;
        }
        let center = self.camera.rect(VICTIM).center() + vec2(-17., 8.);
        let life = (1. - age / 0.24).powi(2);
        for (i, angle) in [-2.35_f32, -1.88, -1.25, -0.54, 0.28, 0.83]
            .iter()
            .enumerate()
        {
            let speed = 52. + i as f32 * 8.;
            let d = vec2(angle.cos(), angle.sin());
            let tip = center + d * (4. + speed * age) + vec2(0., age * age * 70.);
            draw_line(
                tip.x,
                tip.y,
                tip.x - d.x * (2. + life * 4.),
                tip.y - d.y * (2. + life * 4.),
                1.,
                Color::new(1., 0.66, 0.27, life),
            );
        }
        if age < 0.07 {
            draw_line(
                center.x - 4.,
                center.y + 2.,
                center.x + 5.,
                center.y - 3.,
                2.,
                Color::new(1., 0.90, 0.65, life),
            );
        }
    }
}

pub async fn run(output: Option<&Path>) -> Result<(), String> {
    request_new_screen_size(BOARD_W as f32, BOARD_H as f32);
    for _ in 0..4 {
        next_frame().await;
    }
    let study = Study::new()?;
    let before = study.game.recovery_snapshot_bytes()?;
    if let Some(output) = output {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let path = output.join("sequence.rgba");
        if path.exists() {
            return Err("La capture existe déjà ; choisir un nouveau dossier ou réutiliser l'aperçu publié.".into());
        }
        let mut raw = std::fs::File::create(&path).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            study.render(frame as f64 / FPS as f64, false);
            let image = crate::ui_capture::framebuffer()?;
            if image.width as u32 != BOARD_W || image.height as u32 != BOARD_H {
                return Err("La fenêtre ne conserve pas les dimensions natives attendues.".into());
            }
            raw.write_all(&image.bytes).map_err(|e| e.to_string())?;
            if [20, 34, 39].contains(&frame) {
                image.export_png(
                    output
                        .join(format!("comparison-{frame:02}.png"))
                        .to_str()
                        .ok_or("capture path")?,
                );
            }
            next_frame().await;
        }
        study.render(contact() + 0.02, true);
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("reduced-motion.png")
                .to_str()
                .ok_or("capture path")?,
        );
        if study.game.recovery_snapshot_bytes()? != before {
            return Err("La présentation a modifié la simulation.".into());
        }
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"POLISH01-r01","status":"awaiting_visual_approval","native_cell":64,
            "width":BOARD_W,"height":BOARD_H,"panel_width":W,"panel_height":H,"frames":FRAMES,"fps":FPS,
            "panel_origins":[[24,100],[696,100]],"game_state_unchanged":true,"normal_game_activation":false,
            "production_street_renderer":true,"production_actor_compositor":true,"production_skill_animation":"B01",
            "source_art_unchanged":true,"same_actors_and_source_frames":true,"walk_seconds":0.4,
            "presentation_only_contact_seconds":contact(),"proposed_contact_flash_seconds":0.16,"proposed_particle_seconds":0.24,
            "saved_campaign_access":false,"settings_written":false,"rendering_scope":"Controlled isolated study on the real street map; comparison reference is not a full normal-HUD screenshot"
        })).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        return Ok(());
    }
    let mut started = get_time();
    let mut reduced = false;
    loop {
        if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            started = get_time();
        }
        if is_key_pressed(KeyCode::M) {
            reduced = !reduced;
        }
        study.render((get_time() - started) % 3., reduced);
        next_frame().await;
    }
    if study.game.recovery_snapshot_bytes()? != before {
        return Err("La présentation a modifié la simulation.".into());
    }
    Ok(())
}
