//! Area-curse previews using the ANIM-S41-REV03 approved marker presentation.
//! Prepared resistance outcomes, isolated from campaign commands and saves.
use super::super::melee::{actor, blood_spray, player_visual};
use super::zones::{position, preview, terrain};
use super::*;
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use project_rl::skills::spatial::{Area, affected_cells, in_cast_range};
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 6] = ["O02", "AF02", "AF03", "AF04", "AF05", "AF06"];
const TITLES: [&str; 6] = [
    "Interférence",
    "Faille d’armure",
    "Silence",
    "Rupture de régénération",
    "Désorientation",
    "Contagion",
];
const OPEN: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const WALL: &str = "#########\n#.......#\n#.......#\n#....#..#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########";
const FRAMES: usize = 112;
const CENTRE: GridPos = GridPos::new(4, 3);
const MARKER_SIZE: f32 = 24.;

fn first_skill() -> usize {
    if std::env::args().any(|arg| arg == "--lot-desorientation-contagion") {
        4
    } else if std::env::args().any(|arg| arg == "--lot-silence-regeneration") {
        2
    } else {
        0
    }
}

/// The approved PNGs stay byte-identical. Background keying belongs only to
/// the floating marker renderer, with a thin contour for readability on stone.
struct StatusArt {
    icons: [Texture2D; 6],
    material: Material,
}
impl StatusArt {
    fn load() -> Self {
        let image = |bytes: &[u8]| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        };
        let material = load_material(
            ShaderSource::Glsl {
                vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;
varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
                fragment: r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;
float shape(vec2 p){vec4 c=texture2D(Texture,p);return c.a*smoothstep(0.035,0.075,max(c.r,max(c.g,c.b)));}
void main(){
    vec4 c=texture2D(Texture,uv)*tint;
    float a=shape(uv)*tint.a;
    vec2 step=vec2(1.0/24.0);
    float rim=max(max(shape(uv+vec2(step.x,0.0)),shape(uv-vec2(step.x,0.0))),max(shape(uv+vec2(0.0,step.y)),shape(uv-vec2(0.0,step.y))))*0.82*tint.a;
    float opacity=a+rim*(1.0-a);
    vec3 rgb=(c.rgb*a+vec3(0.018,0.022,0.027)*rim*(1.0-a))/max(opacity,0.0001);
    gl_FragColor=vec4(rgb,opacity);
}"#,
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("transparent curse-marker material");
        Self {
            icons: [
                image(include_bytes!(
                    "../../../assets/skills/agent/O02-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/AF02-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/AF03-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/AF04-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/AF05-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/AF06-compact.png"
                )),
            ],
            material,
        }
    }
    fn icon(&self, skill: usize, p: Vec2) {
        gl_use_material(&self.material);
        draw_texture_ex(
            &self.icons[skill],
            p.x - MARKER_SIZE / 2.,
            p.y - MARKER_SIZE / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(MARKER_SIZE, MARKER_SIZE)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Group,
    Player,
    Quiet,
    Wall,
    Resist,
    Move,
    Damage,
    Chain,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Group => "GROUPE",
            Self::Player => "JOUEUR",
            Self::Quiet => "ATTENTE",
            Self::Wall => "MUR",
            Self::Resist => "RESISTE",
            Self::Move => "DEPLACEMENT",
            Self::Damage => "DEGATS",
            Self::Chain => "CHAINE",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    centre: GridPos,
    clock: f32,
    cast_at: Option<f32>,
    turn: u16,
    cooldown: u16,
    energy: u16,
    applied: [bool; 5],
    remaining: [u16; 5],
    positions: [GridPos; 5],
    left_at: Option<f32>,
    entered_at: Option<f32>,
    damaged_at: Option<f32>,
    dead: [bool; 5],
    death_procs: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        let mut positions = [
            GridPos::new(3, 2),
            GridPos::new(5, 2),
            GridPos::new(3, 4),
            GridPos::new(6, 4),
            GridPos::new(5, 4),
        ];
        if case == Case::Wall {
            positions[1] = GridPos::new(5, 4);
            positions[4] = GridPos::new(3, 6);
        }
        if skill == 5 && case == Case::Move {
            positions[3] = GridPos::new(7, 4);
        }
        if case == Case::Chain {
            positions[3] = GridPos::new(1, 2);
        }
        Self {
            skill,
            case,
            direction,
            centre: CENTRE,
            clock: 0.,
            cast_at: None,
            turn: 0,
            cooldown: 0,
            energy: 100,
            applied: [false; 5],
            remaining: [0; 5],
            positions,
            left_at: None,
            entered_at: None,
            damaged_at: None,
            dead: [false; 5],
            death_procs: 0,
        }
    }
    fn parameter(&self, key: &str) -> u16 {
        if self.skill == 5 {
            return match key {
                "energy" | "cooldown_turns" => 0,
                // Prepared source: a directly applied Rupture with six turns
                // left (rank-seven baseline). Retain half at Contagion rank one.
                "duration_rank_turns" => {
                    parameter_for("AF04", key, 7)
                        * parameter_for("AF06", "transmitted_potency_retained_percent", 1)
                        / 100
                }
                "range_cells" => parameter_for("AF06", "range_from_victim_cells", 1),
                _ => parameter_for("AF06", key, 1),
            };
        }
        parameter_for(IDS[self.skill], key, 1)
    }
    fn map(&self) -> Map {
        Map::from_ascii(if self.case == Case::Wall { WALL } else { OPEN }).unwrap()
    }
    fn caster(&self) -> GridPos {
        if self.case == Case::Wall {
            GridPos::new(2, 5)
        } else {
            GridPos::new(4, 6)
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        affected_cells(
            &self.map(),
            self.centre,
            Area::Square {
                radius: self.parameter("zone_width_cells") / 2,
            },
            (0, -1),
        )
    }
    fn hostile(&self, i: usize) -> bool {
        if self.case == Case::Player {
            i == 2
        } else {
            i < 4
        }
    }
    fn can_cast(&self) -> bool {
        let map = self.map();
        if self.skill == 5 {
            return self.cast_at.is_none() && map.is_walkable(self.centre);
        }
        self.cooldown == 0
            && self.energy >= self.parameter("energy")
            && map.is_walkable(self.centre)
            && in_cast_range(self.caster(), self.centre, self.parameter("range_cells"))
            && has_line_of_sight(&map, self.caster(), self.centre, true)
    }
    fn cast(&mut self) {
        if !self.can_cast() {
            return;
        }
        let cells = self.cells();
        self.energy -= self.parameter("energy");
        self.cooldown = self.parameter("cooldown_turns");
        self.cast_at = Some(self.clock);
        if self.skill == 5 {
            self.spread(self.centre, false);
        } else {
            self.apply(&cells);
        }
    }
    fn spread(&mut self, origin: GridPos, received: bool) {
        if received {
            return;
        }
        self.death_procs += 1;
        let cells = affected_cells(
            &self.map(),
            origin,
            Area::Square {
                radius: self.parameter("zone_width_cells") / 2,
            },
            (0, -1),
        );
        self.apply(&cells);
    }
    fn apply(&mut self, cells: &[GridPos]) {
        for i in 0..5 {
            self.applied[i] = self.hostile(i)
                && !self.dead[i]
                && cells.contains(&self.positions[i])
                && !(self.case == Case::Resist && i == 1);
            self.remaining[i] = if self.applied[i] {
                self.parameter("duration_rank_turns")
            } else {
                0
            };
        }
    }
    fn advance(&mut self) {
        if self.cast_at.is_none() {
            return;
        }
        self.turn += 1;
        for n in &mut self.remaining {
            *n = n.saturating_sub(1);
        }
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.case == Case::Move {
            if self.turn == 1 {
                self.positions[0] = GridPos::new(if self.skill == 5 { 1 } else { 2 }, 2);
                self.left_at = Some(self.clock);
            }
            if self.turn == 2 {
                self.positions[3] = GridPos::new(5, 3);
                self.entered_at = Some(self.clock);
            }
        }
        if self.turn == 1 && self.case == Case::Damage {
            self.damaged_at = Some(self.clock);
            if self.skill == 4 {
                self.remaining[0] = 0;
            }
        }
        if self.turn == 1 && self.case == Case::Chain {
            self.dead[0] = true;
            self.remaining[0] = 0;
            if self.skill == 5 {
                self.spread(self.positions[0], true);
            }
        }
    }
    fn snapshot(&self) -> serde_json::Value {
        let cells = self.cells();
        let victims:Vec<_>=(0..5).map(|i|serde_json::json!({"index":i,"position":[self.positions[i].x,self.positions[i].y],"hostile":self.hostile(i),"inside_area_now":cells.contains(&self.positions[i]),"applied":self.applied[i],"remaining":self.remaining[i],"icon_visible":self.remaining[i]>0,"hp":if self.dead[i]{0}else if i==0&&self.damaged_at.is_some(){90}else{100},"damage":if i==0&&self.damaged_at.is_some(){10}else{0},"dead":self.dead[i],"received_by_contagion":self.skill==5&&self.applied[i],"player":self.case==Case::Player&&i==2})).collect();
        serde_json::json!({"turn":self.turn,"cooldown":self.cooldown,"energy":self.energy,"cast":self.cast_at.is_some(),"preview":self.skill!=5&&self.cast_at.is_none()&&self.can_cast(),"area_cells":cells.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"area_width":self.parameter("zone_width_cells"),"centre":[self.centre.x,self.centre.y],"applied_count":self.applied.iter().filter(|x|**x).count(),"active_count":self.remaining.iter().filter(|x|**x>0).count(),"passive":self.skill==5,"death_procs":self.death_procs,"source_alive":self.skill==5&&self.cast_at.is_none(),"inherited_status":if self.skill==5{Some("AF04")}else{None},"victims":victims})
    }
}
fn point(p: GridPos, dir: usize, rect: Rect) -> Vec2 {
    let p = position(p, dir);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn target_point(s: &State, i: usize, rect: Rect) -> Vec2 {
    let target = point(s.positions[i], s.direction, rect);
    let (movement, from) = if i == 0 {
        (s.left_at, GridPos::new(3, 2))
    } else if i == 3 {
        (
            s.entered_at,
            GridPos::new(if s.skill == 5 { 7 } else { 6 }, 4),
        )
    } else {
        (None, s.positions[i])
    };
    let moving = movement.map_or(target, |t| {
        point(from, s.direction, rect).lerp(target, ((s.clock - t) / 0.18).clamp(0., 1.))
    });
    let damaged = if i == 0 && !s.dead[i] {
        s.damaged_at.map_or(Vec2::ZERO, |t| {
            crate::combat_visuals::hit_reaction_offset(
                moving,
                point(s.caster(), s.direction, rect),
                f64::from(s.clock - t),
                64.,
            )
        })
    } else {
        Vec2::ZERO
    };
    moving
        + damaged
        + if s.applied[i] {
            s.cast_at.map_or(Vec2::ZERO, |t| {
                crate::combat_visuals::hit_reaction_offset(
                    moving,
                    point(s.caster(), s.direction, rect),
                    f64::from(s.clock - t),
                    64.,
                )
            })
        } else {
            Vec2::ZERO
        }
}
fn scene(art: &StatusArt, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    if s.skill != 5 && s.cast_at.is_none() && s.can_cast() {
        preview(&s.cells(), s.direction, rect, s.clock);
    }
    if s.skill == 5 {
        let source = point(s.centre, s.direction, rect);
        actor(
            source,
            Visual::Bestiary {
                number: 12,
                corpse: s.cast_at.is_some(),
            },
        );
        if let Some(age) = s.cast_at.map(|t| s.clock - t) {
            if age < 0.30 {
                art.icon(5, source + vec2(0., -48.));
            }
        } else {
            art.icon(3, source + vec2(0., -48.));
            draw_text_bold(
                parameter_for("AF04", "duration_rank_turns", 7).to_string(),
                source.x + 14.,
                source.y - 41.,
                14.,
                TEXT,
            );
        }
    }
    let caster = point(s.caster(), s.direction, rect);
    if s.case == Case::Player {
        enemy(caster);
    } else {
        actor(caster, player_visual());
    }
    for i in 0..5 {
        let p = target_point(s, i, rect);
        if s.dead[i] {
            actor(
                p,
                Visual::Bestiary {
                    number: 12,
                    corpse: true,
                },
            );
        } else if (s.case == Case::Player && i == 2) || (s.case != Case::Player && i == 4) {
            actor(p, player_visual());
        } else {
            enemy(p);
        }
        if i == 0 {
            if let Some(age) = s.damaged_at.map(|t| s.clock - t) {
                blood_spray(p, age, 91, 0.7);
                if age < 0.75 {
                    draw_text_bold("10", p.x - 10., p.y - 78. - age * 15., 22., TEXT);
                }
            }
        }
        if !s.hostile(i) {
            draw_text(
                "Allié",
                p.x - 16.,
                p.y + 40.,
                13.,
                Color::new(0.65, 0.85, 0.68, 1.),
            );
        }
    }
    for i in 0..5 {
        let p = target_point(s, i, rect);
        if s.remaining[i] > 0 {
            let mut badge = p + vec2(0., -48.);
            if s.skill == 5 {
                let t = ((s.clock - s.cast_at.unwrap()) / 0.20).clamp(0., 1.);
                badge = (point(s.centre, s.direction, rect) + vec2(0., -48.)).lerp(badge, t)
                    + vec2(0., -12. * (t * std::f32::consts::PI).sin());
            }
            art.icon(if s.skill == 5 { 3 } else { s.skill }, badge);
            draw_text_bold(
                s.remaining[i].to_string(),
                badge.x + 14.,
                badge.y + 7.,
                14.,
                TEXT,
            );
        } else if s.case == Case::Resist && i == 1 {
            if let Some(a) = s.cast_at.map(|t| s.clock - t).filter(|a| *a < 0.8) {
                draw_text_bold(
                    if s.skill == 4 {
                        "Immunisé"
                    } else {
                        "Résiste"
                    },
                    p.x - 28.,
                    p.y - 48. - a * 10.,
                    18.,
                    TEXT,
                );
            }
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    let mut cases = vec![
        (Case::Group, 0, "DROITE"),
        (Case::Group, 1, "GAUCHE"),
        (Case::Group, 2, "HAUT"),
        (Case::Group, 3, "BAS"),
        (Case::Player, 2, "JOUEUR"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Wall, 2, "MUR"),
        (Case::Resist, 2, "RESISTE"),
        (Case::Move, 2, "DEPLACEMENT"),
    ];
    if first_skill() == 4 {
        cases.extend([(Case::Damage, 2, "DEGATS"), (Case::Chain, 2, "CHAINE")]);
    }
    cases
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 850.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = StatusArt::load();
    let first = first_skill();
    let mut s = State::new(first, Case::Group, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold(
            format!("Malédictions de zone · {}–{}", 91 + first, 92 + first),
            20.,
            35.,
            24.,
            TEXT,
        );
        for (i, title) in TITLES[first..first + 2].iter().enumerate() {
            if button(Rect::new(20., 70. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(first + i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    170. + (i / 2) as f32 * 38.,
                    156.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, (case, title)) in [
            (Case::Group, "Groupe et allié épargné"),
            (Case::Player, "Joueur touché"),
            (Case::Quiet, "Attente sans nouveau tour"),
            (Case::Wall, "Obstacles et coin fermé"),
            (Case::Resist, "Résistance individuelle"),
            (Case::Move, "Sortie et entrée dans la zone"),
            (Case::Damage, "Dégâts et interruption"),
            (Case::Chain, "Mort sans retransmission"),
        ]
        .iter()
        .enumerate()
        .take(if first == 4 { 8 } else { 6 })
        {
            if button(
                Rect::new(20., 260. + i as f32 * 36., 330., 32.),
                title,
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        let rect = Rect::new(400., 88., 576., 576.);
        if is_mouse_button_pressed(MouseButton::Left) && s.cast_at.is_none() && s.skill != 5 {
            let (x, y) = mouse_position();
            if rect.contains(vec2(x, y)) {
                let displayed =
                    GridPos::new(((x - rect.x) / 64.) as i32, ((y - rect.y) / 64.) as i32);
                s.centre = position(
                    displayed,
                    match s.direction {
                        0 => 1,
                        1 => 0,
                        d => d,
                    },
                );
            }
        }
        let offset = if first == 4 { 72. } else { 0. };
        if button(
            Rect::new(20., 495. + offset, 330., 34.),
            if s.skill == 5 {
                "Déclencher la mort"
            } else {
                "Lancer"
            },
            s.can_cast(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.cast();
        }
        if button(
            Rect::new(20., 537. + offset, 330., 34.),
            "Passer un tour",
            s.cast_at.is_some(),
        ) {
            s.advance();
        }
        if button(Rect::new(20., 579. + offset, 158., 34.), "Rejouer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(186., 579. + offset, 164., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, rect);
        draw_text_bold(
            format!(
                "Énergie {} / 100 · recharge {} tours · tour {}",
                s.energy, s.cooldown, s.turn
            ),
            400.,
            704.,
            19.,
            TEXT,
        );
        draw_text(
            format!(
                "{} victimes affectées · {} statuts actifs",
                s.applied.iter().filter(|x| **x).count(),
                s.remaining.iter().filter(|x| **x > 0).count()
            ),
            400.,
            733.,
            18.,
            TEXT,
        );
        draw_text(
            if s.skill == 5 {
                "Passif : la mort de la victime déclenche la transmission."
            } else {
                "Avant de lancer : cliquer une case pour choisir le centre."
            },
            400.,
            764.,
            16.,
            DIM,
        );
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists()
        || output
            .parent()
            .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Utiliser une revue neuve non validée.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = StatusArt::load();
    let first = first_skill();
    let marker_target = render_target(128, 64);
    set_camera(&Camera2D {
        render_target: Some(marker_target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 128., 64.))
    });
    clear_background(Color::new(0., 0., 0., 0.));
    for i in 0..2 {
        art.icon(first + i, vec2(32. + i as f32 * 64., 32.));
    }
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    let marker_image = marker_target.texture.get_texture_data();
    let mut transparency = Vec::new();
    for (skill, id) in IDS[first..first + 2].iter().enumerate() {
        let mut transparent = 0;
        let mut visible = 0;
        for y in 20..44 {
            for x in 20 + skill * 64..44 + skill * 64 {
                let alpha = marker_image.bytes[(y * 128 + x) * 4 + 3];
                transparent += usize::from(alpha == 0);
                visible += usize::from(alpha > 127);
            }
        }
        if transparent < 32 || visible < 32 {
            return Err(format!("Marqueur {id} : transparence ou forme absente."));
        }
        transparency.push(serde_json::json!({"id":id,"size":24,"fully_transparent_pixels":transparent,"visible_pixels":visible}));
    }
    marker_image.export_png(output.join("marker-proof.png").to_str().unwrap());
    std::fs::write(
        output.join("marker-transparency.json"),
        serde_json::to_vec_pretty(&transparency).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    set_default_camera();
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    let only_case =
        std::env::args().find_map(|arg| arg.strip_prefix("--review-case=").map(str::to_owned));
    for skill in first..first + 2 {
        for (case, direction, label) in cases() {
            if only_case.as_deref().is_some_and(|wanted| wanted != label) {
                continue;
            }
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            let mut indices = Vec::new();
            let mut known = BTreeMap::new();
            let mut images = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.;
                if frame == 12 {
                    s.cast();
                }
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&art, &s, Rect::new(0., 0., 576., 576.));
                unsafe {
                    macroquad::window::get_internal_gl().flush();
                }
                let mut im = target.texture.get_texture_data();
                for p in im.bytes.as_chunks_mut::<4>().0 {
                    p[3] = 255;
                }
                let mut hash = DefaultHasher::new();
                im.bytes.hash(&mut hash);
                let h = hash.finish();
                let n = if let Some(n) = known.get(&h) {
                    *n
                } else {
                    let n = images.len();
                    images.push(im);
                    known.insert(h, n);
                    n
                };
                indices.push(n);
                set_default_camera();
                if frame % 8 == 0 {
                    next_frame().await;
                }
            }
            let columns = 4;
            let rows = images.len().div_ceil(columns);
            let mut atlas =
                Image::gen_image_color((576 * columns) as u16, (576 * rows) as u16, INK);
            for (i, im) in images.into_iter().enumerate() {
                for y in 0..576 {
                    let k = (((i / columns) * 576 + y) * 576 * columns + (i % columns) * 576) * 4;
                    atlas.bytes[k..k + 576 * 4]
                        .copy_from_slice(&im.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
                }
            }
            atlas.export_png(dir.join("atlas.png").to_str().unwrap());
            let revision = if first == 0 { "A03" } else { "A01" };
            entries.push(serde_json::json!({"id":IDS[skill],"number":91+skill,"title":TITLES[skill],"variant":format!("{}-{revision}-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contagion_spreads_reduced_actor_bound_status_and_never_retransmits() {
        let mut s = State::new(5, Case::Group, 2);
        assert_eq!(s.cells().len(), 25);
        assert_eq!(s.parameter("duration_rank_turns"), 3);
        s.cast();
        assert_eq!(s.applied, [true, true, true, true, false]);
        assert_eq!(s.remaining, [3, 3, 3, 3, 0]);
        assert_eq!((s.energy, s.cooldown, s.death_procs), (100, 0, 1));
        s.clock = 100.;
        s.cast();
        assert_eq!(s.death_procs, 1);
        assert_eq!(s.remaining[0], 3);
        let mut chain = State::new(5, Case::Chain, 2);
        assert!(!chain.cells().contains(&chain.positions[3]));
        let secondary_area = affected_cells(
            &chain.map(),
            chain.positions[0],
            Area::Square { radius: 2 },
            (0, -1),
        );
        assert!(secondary_area.contains(&chain.positions[3]));
        chain.cast();
        chain.advance();
        assert!(chain.dead[0]);
        assert_eq!(chain.remaining[0], 0);
        assert_eq!(chain.death_procs, 1);
        assert!(!chain.applied[3]);
        let mut moving = State::new(5, Case::Move, 2);
        moving.cast();
        moving.advance();
        assert!(!moving.cells().contains(&moving.positions[0]));
        assert_eq!(moving.remaining[0], 2);
        moving.advance();
        assert!(moving.cells().contains(&moving.positions[3]));
        assert!(!moving.applied[3]);
        for case in [Case::Wall, Case::Resist, Case::Player] {
            let mut s = State::new(5, case, 2);
            s.cast();
            assert!(!s.applied[4]);
            if case == Case::Player {
                assert_eq!(s.applied, [false, false, true, false, false]);
            } else {
                assert!(!s.applied[1]);
            }
        }
    }
    #[test]
    fn disorientation_ends_on_damage_but_inherited_regeneration_rupture_does_not() {
        for skill in [4, 5] {
            let mut s = State::new(skill, Case::Damage, 2);
            s.cast();
            s.advance();
            assert!(s.damaged_at.is_some());
            assert_eq!(s.remaining[0], if skill == 4 { 0 } else { 2 });
            assert_eq!(s.remaining[1], if skill == 4 { 1 } else { 2 });
        }
    }
    #[test]
    fn area_curses_filter_each_victim_and_keep_status_on_moving_actors() {
        for skill in 0..5 {
            let mut s = State::new(skill, Case::Group, 2);
            let duration = s.parameter("duration_rank_turns");
            let cooldown = s.parameter("cooldown_turns");
            assert_eq!(s.cells().len(), 9);
            s.cast();
            assert_eq!(s.applied, [true, true, true, false, false]);
            let energy = s.energy;
            s.clock = 100.;
            assert_eq!(s.remaining, [duration, duration, duration, 0, 0]);
            s.cast();
            assert_eq!(s.energy, energy);
            for _ in 0..duration {
                s.advance();
            }
            assert_eq!(s.remaining, [0; 5]);
            assert_eq!(s.cooldown, cooldown.saturating_sub(duration));
            let mut m = State::new(skill, Case::Move, 2);
            m.cast();
            m.advance();
            assert!(!m.cells().contains(&m.positions[0]));
            assert_eq!(m.remaining[0], duration - 1);
            m.advance();
            assert!(m.cells().contains(&m.positions[3]));
            assert_eq!(m.remaining[3], 0);
            let mut r = State::new(skill, Case::Resist, 2);
            r.cast();
            assert_eq!(r.applied, [true, false, true, false, false]);
            let mut w = State::new(skill, Case::Wall, 2);
            assert!(w.can_cast());
            w.cast();
            assert!(w.applied[0]);
            assert!(!w.applied[1]);
            assert!(!w.remaining.iter().all(|x| *x == 0));
            let mut p = State::new(skill, Case::Player, 2);
            p.cast();
            assert_eq!(p.applied, [false, false, true, false, false]);
            let mut blocked = State::new(skill, Case::Wall, 2);
            blocked.centre = GridPos::new(5, 3);
            blocked.cast();
            blocked.advance();
            assert_eq!(blocked.energy, 100);
            assert_eq!(blocked.turn, 0);
            let mut far = State::new(skill, Case::Group, 2);
            far.centre = GridPos::new(1, 1);
            far.cast();
            assert_eq!(far.energy, 100);
            let mut empty = State::new(skill, Case::Group, 2);
            empty.centre = GridPos::new(1, 5);
            empty.cast();
            assert_eq!(empty.applied, [false; 5]);
        }
    }
    #[test]
    fn accepted_affliction_widths_include_bonus_ranks_and_contagion_cap() {
        for id in ["O02", "AF02", "AF03", "AF04", "AF05"] {
            for (rank, width) in [(1, 3), (4, 5), (7, 7), (10, 9), (13, 11)] {
                assert_eq!(parameter_for(id, "zone_width_cells", rank), width);
            }
        }
        // The display evaluator does not apply general maxima itself; callers
        // retain the explicitly accepted Contagion cap when computing its area.
        for (rank, width) in [(1, 5), (6, 7), (11, 9), (30, 9)] {
            assert_eq!(
                parameter_for("AF06", "zone_width_cells", rank).min(9),
                width
            );
        }
    }
}
