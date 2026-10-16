//! ANIM-S33: fragmentation throw and its permanent range passive, native64 review.
//! Prepared responses only; neither campaign commands nor save state are changed.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use project_rl::skills::spatial::{Area, affected_cells, in_cast_range};
use project_rl::world::{FieldOfViewRules, Map, compute_visible_tiles, has_line_of_sight};

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const CLOSE: GridPos = GridPos::new(4, 3);
const FAR: GridPos = GridPos::new(4, 1);
const FLIGHT: f32 = 0.18;
const BLAST: f32 = 0.42;
const IDS: [&str; 2] = ["SG00", "SG03"];
const NUMBERS: [u16; 2] = [74, 77];
const TITLES: [&str; 2] = ["Lancer de grenade", "Portée de lancer"];
const CASES: [&str; 4] = [
    "Deux groupes à des distances différentes",
    "Explosion au bord d’une cloison",
    "Trajectoire bloquée",
    "Explosion près du joueur",
];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 4] = [
    ".........\n.........\n.........\n.........\n.........\n.........\n.........\n.........\n.........",
    ".....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...",
    ".........\n.........\n.........\n.........\n#########\n.........\n.........\n.........\n.........",
    ".........\n.........\n.........\n.........\n.........\n.........\n.........\n.........\n.........",
];

struct State {
    falloff: bool,
    skill: usize,
    case: usize,
    direction: usize,
    clock: f32,
    launched: Option<f32>,
    resolved: bool,
    turn: u16,
    energy: u16,
    cooldown: u16,
    hp: [u16; 4],
    contacts: [u16; 4],
    hit_at: [Option<f32>; 4],
}
impl State {
    fn new(skill: usize, case: usize, direction: usize) -> Self {
        Self {
            falloff: false,
            skill,
            case,
            direction,
            clock: 0.0,
            launched: None,
            resolved: false,
            turn: 0,
            energy: 100,
            cooldown: 0,
            hp: [100; 4],
            contacts: [0; 4],
            hit_at: [None; 4],
        }
    }
    fn for_review(skill: usize, case: usize, direction: usize) -> Self {
        let mut s = Self::new(skill, case, direction);
        s.falloff = !std::env::args().any(|arg| arg == "--historique-grenades-uniformes");
        s
    }
    fn rank(&self) -> u16 {
        if self.skill == 0 { 1 } else { 11 }
    }
    fn range(&self) -> u16 {
        parameter_for("SG00", "base_throw_range_cells", 1)
            + if self.skill == 1 {
                parameter_for("SG03", "throw_range_bonus_cells", self.rank())
            } else {
                0
            }
    }
    fn target(&self) -> GridPos {
        if self.case == 3 {
            PLAYER
        } else if self.case == 1 || self.skill == 0 {
            CLOSE
        } else {
            FAR
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.case]).unwrap()
    }
    fn visible(&self, at: GridPos) -> bool {
        compute_visible_tiles(
            &self.map(),
            PLAYER,
            FieldOfViewRules {
                radius: 12,
                ..Default::default()
            },
        )
        .contains(&at)
    }
    fn targets(&self) -> [GridPos; 4] {
        if self.case == 3 {
            [
                GridPos::new(4, 5),
                GridPos::new(3, 6),
                GridPos::new(6, 6),
                GridPos::new(7, 2),
            ]
        } else if self.falloff && self.case == 0 {
            [CLOSE, GridPos::new(3, 4), FAR, GridPos::new(3, 1)]
        } else {
            [
                CLOSE,
                GridPos::new(3, 3),
                FAR,
                if self.case == 1 {
                    GridPos::new(6, 3)
                } else {
                    GridPos::new(5, 1)
                },
            ]
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        affected_cells(
            &self.map(),
            self.target(),
            if self.falloff {
                Area::Square {
                    radius: parameter_for("SG00", "radius_cells", 1),
                }
            } else {
                Area::Disc {
                    radius: parameter_for("SG00", "radius_cells", 1),
                }
            },
            (1, 0),
        )
    }
    fn damage(&self, at: GridPos) -> u16 {
        if self.falloff {
            super::super::area_falloff::damage(24, self.target(), at, 1).unwrap_or(0)
        } else {
            24
        }
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0
            && self.energy >= 8
            && self.map().is_walkable(self.target())
            && self.visible(self.target())
            && in_cast_range(PLAYER, self.target(), self.range())
            && has_line_of_sight(&self.map(), PLAYER, self.target(), true)
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.resolved = false;
        self.energy -= 8;
        self.cooldown = parameter_for("SG00", "cooldown_turns", 1);
        self.turn += 1;
    }
    fn refresh(&mut self) {
        if self.resolved || self.launched.is_none_or(|t| self.clock - t < FLIGHT) {
            return;
        }
        let cells = self.cells();
        for (i, at) in self.targets().into_iter().enumerate() {
            if cells.contains(&at) {
                self.hp[i] = self.hp[i].saturating_sub(self.damage(at));
                self.contacts[i] += 1;
                self.hit_at[i] = self.launched.map(|t| t + FLIGHT);
            }
        }
        self.resolved = true;
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|t| (self.clock - t).max(0.0))
    }
    fn record(&self) -> serde_json::Value {
        serde_json::json!({"clock":self.clock,"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,"hp":self.hp,"contacts":self.contacts,"damage":self.targets().map(|p|if self.cells().contains(&p){self.damage(p)}else{0}),"falloff":self.falloff,"player_hp":100,"valid":self.can_launch(),"range":self.range(),"preview":self.launched.is_none(),"projectile":self.age().is_some_and(|a|a<FLIGHT),"explosion":self.age().is_some_and(|a|(FLIGHT..FLIGHT+BLAST).contains(&a))})
    }
}

fn point(at: GridPos, direction: usize, r: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn load(bytes: &[u8]) -> Texture2D {
    let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    t.set_filter(FilterMode::Linear);
    t
}
struct Art {
    grenade: Texture2D,
    burst: Texture2D,
    blood: Texture2D,
    clip: Material,
    masks: Vec<Texture2D>,
}
impl Art {
    fn load() -> Self {
        let clip=load_material(ShaderSource::Glsl{vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
void main(){p=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
fragment:r#"#version 100
precision highp float;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;uniform sampler2D mask;uniform vec2 origin;uniform float direction;
void main(){vec2 q=p-origin;if(direction<0.5)q=vec2(q.y,576.0-q.x);else if(direction<1.5)q=vec2(576.0-q.y,q.x);else if(direction>2.5)q=vec2(576.0)-q;if(q.x<0.0||q.y<0.0||q.x>=576.0||q.y>=576.0)discard;if(texture2D(mask,(floor(q/64.0)+vec2(0.5))/9.0).r<0.5)discard;gl_FragColor=texture2D(Texture,uv)*tint;}"#},MaterialParams{uniforms:vec![UniformDesc::new("origin",UniformType::Float2),UniformDesc::new("direction",UniformType::Float1)],textures:vec!["mask".into()],pipeline_params:PipelineParams{color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},..Default::default()}).unwrap();
        let masks = (0..8)
            .map(|i| {
                let s = State::new(i / 4, i % 4, 2);
                let mut im = Image::gen_image_color(9, 9, BLACK);
                for p in affected_cells(&s.map(), s.target(), Area::Square { radius: 12 }, (1, 0)) {
                    im.set_pixel(p.x as u32, p.y as u32, WHITE);
                }
                let t = Texture2D::from_image(&im);
                t.set_filter(FilterMode::Nearest);
                t
            })
            .collect();
        Self {
            grenade: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-72/grenade-source.png"
            )),
            burst: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-72/fragment-burst-source.png"
            )),
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
            clip,
            masks,
        }
    }
    fn mask(&self, s: &State, r: Rect) {
        self.clip.set_uniform("origin", [r.x, r.y]);
        self.clip.set_uniform("direction", s.direction as f32);
        self.clip
            .set_texture("mask", self.masks[s.skill * 4 + s.case].clone());
        gl_use_material(&self.clip);
    }
    fn effects(&self, s: &State, r: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let from = point(PLAYER, s.direction, r) + vec2(12.0, -8.0);
        let to = point(s.target(), s.direction, r);
        if age < FLIGHT {
            let t = age / FLIGHT;
            let p = from.lerp(to, t) + vec2(0.0, -(t * std::f32::consts::PI).sin() * 30.0);
            draw_ellipse(p.x, p.y + 18.0, 7.0, 3.0, 0.0, alpha(BLACK, 0.30));
            draw_texture_ex(
                &self.grenade,
                p.x - 16.0,
                p.y - 16.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    rotation: t * 2.7,
                    ..Default::default()
                },
            );
        }
        let a = age - FLIGHT;
        if (0.0..BLAST).contains(&a) {
            let frame = ((a / BLAST * 6.0) as u32).min(5);
            let size = if s.falloff { 220.0 } else { 180.0 };
            self.mask(s, r);
            draw_texture_ex(
                &self.burst,
                to.x - size / 2.0,
                to.y - size / 2.0,
                alpha(WHITE, 0.94),
                DrawTextureParams {
                    source: Some(Rect::new(
                        (frame % 3) as f32 * 512.0,
                        (frame / 3) as f32 * 512.0,
                        512.0,
                        512.0,
                    )),
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
            gl_use_default_material();
        }
    }
    fn blood(&self, s: &State, r: Rect) {
        self.mask(s, r);
        for (i, at) in s.targets().into_iter().enumerate() {
            if let Some(t) = s.hit_at[i] {
                let age = s.clock - t;
                if (0.0..0.30).contains(&age) {
                    let p = point(at, s.direction, r);
                    let frame = ((age / 0.30 * 6.0) as u32).min(5);
                    draw_texture_ex(
                        &self.blood,
                        p.x - 31.0,
                        p.y - 25.0,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                            dest_size: Some(vec2(62.0, 62.0)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        gl_use_default_material();
    }
}
fn draw(s: &State, art: &Art, r: Rect, mode: u8) {
    terrain(&s.map(), s.direction, r, false);
    if mode == 0 && s.launched.is_none() {
        preview(&s.cells(), s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    if mode == 0 {
        art.blood(s, r);
    }
    if mode < 2 {
        for (i, at) in s.targets().into_iter().enumerate() {
            if s.visible(at) && s.map().is_walkable(at) {
                let p = point(at, s.direction, r);
                let offset = if mode == 0 {
                    if let Some(t) = s.hit_at[i] {
                        crate::combat_visuals::hit_reaction_offset(
                            p,
                            point(s.target(), s.direction, r),
                            f64::from(s.clock - t),
                            64.0,
                        )
                    } else {
                        Vec2::ZERO
                    }
                } else {
                    Vec2::ZERO
                };
                actor(p + offset, Visual::Actor(5));
            }
        }
        actor(
            point(PLAYER, s.direction, r),
            super::super::melee::player_visual(),
        );
    }
    if mode == 0 || mode == 2 {
        art.effects(s, r);
    }
    if mode == 0 {
        if let Some(a) = s.age() {
            if a < 0.70 {
                let p = point(PLAYER, s.direction, r);
                draw_text_bold(
                    "Lancer de grenade",
                    p.x - 70.0,
                    p.y - 38.0 - a * 8.0,
                    16.0,
                    alpha(TEXT, 1.0 - a / 0.70),
                );
            }
        }
        for (i, at) in s.targets().into_iter().enumerate() {
            if let Some(t) = s.hit_at[i] {
                let a = s.clock - t;
                if (0.0..0.65).contains(&a) {
                    let p = point(at, s.direction, r);
                    let c = alpha(TEXT, 1.0 - a / 0.65);
                    draw_text_bold(
                        format!("{}", s.damage(at)),
                        p.x - 14.0,
                        p.y - 34.0 - a * 14.0,
                        22.0,
                        c,
                    );
                    draw_text("(", p.x - 21.0, p.y - 18.0 - a * 14.0, 15.0, c);
                    draw_line(
                        p.x - 11.0,
                        p.y - 21.0 - a * 14.0,
                        p.x - 4.0,
                        p.y - 15.0 - a * 14.0,
                        2.4,
                        c,
                    );
                    draw_text(
                        format!("{})", s.damage(at)),
                        p.x + 2.0,
                        p.y - 18.0 - a * 14.0,
                        16.0,
                        c,
                    );
                }
            }
        }
    }
}
fn hud(s: &State, r: Rect) {
    draw_text_bold(
        format!("{} · {}", NUMBERS[s.skill], TITLES[s.skill]),
        r.x,
        r.y - 37.0,
        20.0,
        TEXT,
    );
    draw_text(
        format!(
            "Portée {} cases · {}",
            s.range(),
            if s.skill == 0 {
                "sans passif"
            } else {
                "passif au rang effectif 11"
            }
        ),
        r.x,
        r.y - 15.0,
        14.0,
        DIM,
    );
    draw_text(
        format!(
            "Tour {} · énergie {} · recharge {} · PV joueur 100",
            s.turn, s.energy, s.cooldown
        ),
        r.x + 8.0,
        r.bottom() + 24.0,
        15.0,
        TEXT,
    );
    draw_text(
        if s.launched.is_some() {
            "Projectile puis impact ; l’attente ne passe aucun tour."
        } else if !s.can_launch() {
            "Lancement refusé : cible masquée ou trajet bloqué."
        } else {
            "Choix de zone avant confirmation."
        },
        r.x + 8.0,
        r.bottom() + 46.0,
        14.0,
        DIM,
    );
}
pub(crate) async fn run() {
    let art = Art::load();
    let mut states = [State::for_review(0, 0, 2), State::for_review(1, 0, 2)];
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for s in &mut states {
            s.clock += get_frame_time();
            s.refresh();
        }
        if is_key_pressed(KeyCode::Space) {
            for s in &mut states {
                s.launch();
            }
        }
        if is_key_pressed(KeyCode::N) {
            for s in &mut states {
                s.advance();
            }
        }
        if is_key_pressed(KeyCode::R) {
            states = states.map(|s| State::for_review(s.skill, s.case, s.direction));
        }
        if is_key_pressed(KeyCode::W) {
            states = states.map(|s| State::for_review(s.skill, (s.case + 1) % 4, s.direction));
        }
        if is_key_pressed(KeyCode::D) {
            states = states.map(|s| State::for_review(s.skill, s.case, (s.direction + 1) % 4));
        }
        clear_background(INK);
        draw_text(
            "Grenades · Espace : lancer · N : tour · W : décor · D : direction · R : reset",
            32.0,
            28.0,
            17.0,
            TEXT,
        );
        for (i, s) in states.iter().enumerate() {
            let r = Rect::new(32.0 + i as f32 * 616.0, 88.0, SIZE, SIZE);
            draw(s, &art, r, 0);
            hud(s, r);
        }
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Dossier de capture existant.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    let target = render_target(640, 720);
    let mut entries = Vec::new();
    for skill in 0..2 {
        for case in 0..4 {
            for direction in 0..4 {
                let mut s = State::for_review(skill, case, direction);
                let revision = if s.falloff { "A02" } else { "A01" };
                let folder = format!("{}-{}-C{}-{}", IDS[skill], revision, case, DIRS[direction]);
                let dir = output.join(&folder);
                std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                let mut timeline = Vec::new();
                for n in 0..104 {
                    s.clock = n as f32 / 24.0;
                    if n == 18 {
                        s.launch();
                    }
                    s.refresh();
                    if (n == 64 || n == 88) && s.launched.is_some() {
                        s.advance();
                    }
                    timeline.push(s.record());
                    let r = Rect::new(32.0, 88.0, SIZE, SIZE);
                    for (mode, name) in [
                        (0, format!("{n:03}.rgba")),
                        (1, format!("control-{n:03}.rgba")),
                        (2, format!("effect-proof-{n:03}.rgba")),
                        (3, format!("terrain-proof-{n:03}.rgba")),
                    ] {
                        if mode > 0 && n != 27 {
                            continue;
                        }
                        set_camera(&Camera2D {
                            render_target: Some(target.clone()),
                            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
                        });
                        clear_background(INK);
                        draw(&s, &art, r, mode);
                        if mode < 2 {
                            hud(&s, r);
                        }
                        super::occult::export(&target, &dir.join(name));
                        set_default_camera();
                    }
                }
                std::fs::write(
                    dir.join("timeline.json"),
                    serde_json::to_vec_pretty(&timeline).unwrap(),
                )
                .map_err(|e| e.to_string())?;
                entries.push(serde_json::json!({"id":IDS[skill],"number":NUMBERS[skill],"title":TITLES[skill],"folder":folder,"variant":folder,"case":case,"case_title":CASES[case],"direction":direction,"tile":64,"frames":104,"fps":24,"launch_frame":18,"turn_frames":[64,88],"proof_frame":27,"map":MAPS[case],"target":[s.target().x,s.target().y],"player":[PLAYER.x,PLAYER.y],"mechanical_cells":s.cells().iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"range":s.range(),"falloff":s.falloff,"target_cells":s.targets().map(|p|[p.x,p.y])}));
            }
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn proposed_square_hits_corners_with_half_damage_and_preserves_self_immunity() {
        for skill in 0..2 {
            let mut s = State::new(skill, 0, 2);
            s.falloff = true;
            assert_eq!(s.cells().len(), 9);
            s.launch();
            s.clock = FLIGHT;
            s.refresh();
            assert_eq!(
                s.hp,
                if skill == 0 {
                    [76, 88, 100, 100]
                } else {
                    [100, 100, 76, 88]
                }
            );
            let hp = s.hp;
            s.clock = 600.0;
            s.refresh();
            assert_eq!(s.hp, hp);
            assert_eq!((s.turn, s.energy, s.cooldown), (1, 92, 1));
            let mut own = State::new(skill, 3, 2);
            own.falloff = true;
            own.launch();
            own.clock = FLIGHT;
            own.refresh();
            assert_eq!(own.hp, [88, 88, 100, 100]);
            assert_eq!(own.record()["player_hp"], 100);
        }
    }
    #[test]
    fn throw_resolves_once_and_waiting_does_not_consume_turns() {
        for skill in 0..2 {
            let mut s = State::new(skill, 0, 2);
            s.launch();
            assert_eq!((s.turn, s.energy, s.cooldown), (1, 92, 1));
            s.clock = FLIGHT;
            s.refresh();
            assert_eq!(s.hp.iter().filter(|&&hp| hp == 76).count(), 2);
            let hp = s.hp;
            s.clock = 600.0;
            s.refresh();
            assert_eq!(s.hp, hp);
            assert_eq!((s.turn, s.energy, s.cooldown), (1, 92, 1));
            assert!(!s.age().is_some_and(|a| a < FLIGHT + BLAST));
        }
    }
    #[test]
    fn wall_refusal_is_atomic_and_self_cast_keeps_player_immune() {
        for skill in 0..2 {
            let mut blocked = State::new(skill, 2, 2);
            assert!(!blocked.can_launch());
            blocked.launch();
            blocked.clock = 10.0;
            blocked.refresh();
            assert!(blocked.launched.is_none());
            assert_eq!(
                (blocked.turn, blocked.energy, blocked.cooldown),
                (0, 100, 0)
            );
            assert_eq!(blocked.hp, [100; 4]);
            let mut own = State::new(skill, 3, 2);
            own.launch();
            own.clock = FLIGHT;
            own.refresh();
            assert_eq!(own.hp, [76, 76, 100, 100]);
            assert_eq!(own.record()["player_hp"], 100);
        }
    }
    #[test]
    fn passive_extends_range_without_damage_or_area_change() {
        let base = State::new(0, 0, 2);
        let extended = State::new(1, 0, 2);
        assert_eq!((base.range(), extended.range()), (4, 8));
        assert!(!in_cast_range(PLAYER, FAR, base.range()));
        assert!(in_cast_range(PLAYER, FAR, extended.range()));
        assert_eq!(base.cells().len(), 5);
        assert_eq!(extended.cells().len(), 5);
        for d in 0..4 {
            assert_eq!(
                position(PLAYER, d).x.abs().max(position(PLAYER, d).y.abs()) <= 8,
                true
            );
        }
    }
}
