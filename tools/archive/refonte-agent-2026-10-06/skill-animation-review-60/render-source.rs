//! ANIM-S30: mine placement, arming and capacity, independent prepared fixtures.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const MINE: GridPos = GridPos::new(4, 5);
const IDS: [&str; 2] = ["SM00", "SM01"];
const TITLES: [&str; 2] = ["Pose de mine", "Réseau de mines"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const FRAMES: usize = 160;
const ACTIONS: [usize; 3] = [64, 88, 112];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Capacity,
    Rank12,
    AtArming,
    Layer,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Armement puis passage ennemi",
            Self::Quiet => "Attente sans passage de tour",
            Self::Wall => "Explosion contre une cloison",
            Self::Capacity => "Remplacement de la plus ancienne",
            Self::Rank12 => "Rang effectif 12 : cinq mines",
            Self::AtArming => "Ennemi arrivé avant l’armement",
            Self::Layer => "Changement de couche",
        }
    }
}
#[derive(Clone)]
struct Mine {
    at: GridPos,
    armed: bool,
    serial: u16,
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    step: u16,
    turn: u16,
    energy: u16,
    cooldown: u16,
    mines: Vec<Mine>,
    removed_oldest: u16,
    primary: GridPos,
    from: GridPos,
    moved: Option<f32>,
    blast: Option<f32>,
    hp: [u16; 3],
    hits: [bool; 3],
    explosions: u16,
    layer_changed: bool,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        let mut s = Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            step: 0,
            turn: 0,
            energy: 100,
            cooldown: 0,
            mines: vec![],
            removed_oldest: 0,
            primary: if case == Case::AtArming {
                GridPos::new(4, 4)
            } else {
                GridPos::new(4, 3)
            },
            from: GridPos::new(4, 3),
            moved: None,
            blast: None,
            hp: [100; 3],
            hits: [false; 3],
            explosions: 0,
            layer_changed: false,
        };
        if case == Case::Capacity {
            let old = [
                GridPos::new(3, 6),
                GridPos::new(5, 6),
                GridPos::new(3, 7),
                GridPos::new(5, 7),
                GridPos::new(4, 7),
            ];
            s.mines = old
                .into_iter()
                .take(s.capacity())
                .enumerate()
                .map(|(i, at)| Mine {
                    at,
                    armed: true,
                    serial: i as u16 + 1,
                })
                .collect();
        }
        s
    }
    fn capacity(&self) -> usize {
        if self.skill == 0 { 1 } else { 5 }
    }
    fn max_steps(&self) -> u16 {
        match self.case {
            Case::Quiet => 0,
            Case::AtArming => 1,
            Case::Layer => 2,
            _ => 3,
        }
    }
    fn rank(&self) -> u16 {
        if self.skill == 0 {
            1
        } else if self.case == Case::Rank12 {
            12
        } else {
            11
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn variant(&self) -> String {
        format!("{}-A01", IDS[self.skill])
    }
    fn targets(&self) -> [GridPos; 3] {
        [
            self.primary,
            GridPos::new(3, 5),
            GridPos::new(if self.case == Case::Wall { 6 } else { 5 }, 5),
        ]
    }
    fn area(&self) -> Vec<GridPos> {
        // Cardinal radius-one fixture; final diagonal/large-footprint geometry is separate.
        [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(x, y)| GridPos::new(MINE.x + x, MINE.y + y))
            .filter(|&p| self.map().is_walkable(p))
            .collect()
    }
    fn led_on(&self) -> bool {
        (self.clock * 24.0).round() as u32 % 18 < 7
    }
    fn begin(&mut self) {
        if self.begun.is_some() {
            return;
        }
        self.begun = Some(self.clock);
        self.turn = 1;
        self.energy -= 10;
        self.cooldown = 1;
        let serial = self.mines.last().map_or(1, |m| m.serial + 1);
        if self.mines.len() == self.capacity() {
            self.mines.remove(0);
            self.removed_oldest += 1;
        }
        self.mines.push(Mine {
            at: MINE,
            armed: false,
            serial,
        });
    }
    fn move_primary(&mut self, at: GridPos) {
        self.from = self.primary;
        self.primary = at;
        self.moved = Some(self.clock);
    }
    fn detonate(&mut self) {
        if let Some(index) = self
            .mines
            .iter()
            .position(|m| m.at == self.primary && m.armed)
        {
            self.mines.remove(index);
            self.explosions += 1;
            self.blast = Some(self.clock);
            let area = self.area();
            for (i, at) in self.targets().into_iter().enumerate() {
                self.hits[i] = area.contains(&at);
                if self.hits[i] {
                    self.hp[i] -= 13;
                }
            }
        }
    }
    fn tick(&mut self) {
        if self.case == Case::AtArming
            && self.step == 0
            && self.begun.is_some_and(|t| self.clock - t >= 0.18)
            && self.primary != MINE
        {
            self.move_primary(MINE);
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() || self.step >= self.max_steps() {
            return;
        }
        self.step += 1;
        self.turn += 1;
        self.cooldown = 0;
        if self.case == Case::Layer && self.step == 2 {
            self.mines.clear();
            self.layer_changed = true;
            return;
        }
        if self.layer_changed {
            return;
        }
        match self.step {
            1 => {
                for m in &mut self.mines {
                    m.armed = true;
                }
                self.detonate();
            }
            2 if self.case != Case::AtArming => self.move_primary(GridPos::new(4, 4)),
            3 if self.case != Case::AtArming => {
                self.move_primary(MINE);
                self.detonate();
            }
            _ => (),
        }
    }
}
fn point(at: GridPos, direction: usize, r: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
struct Art {
    mine: Texture2D,
    blast: Texture2D,
    icons: [Texture2D; 2],
    fire: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            mine: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-60/mine-sprite.png"
            )),
            blast: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-20/burst-atlas.png"
            )),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SM00-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SM01-compact.png"
                )),
            ],
            fire: load(include_bytes!(
                "../../../assets/skills/agent/FE01-compact.png"
            )),
        }
    }
    fn mine(&self, m: &Mine, s: &State, r: Rect) {
        let p = point(m.at, s.direction, r);
        draw_ellipse(p.x, p.y + 29.0, 22.0, 4.0, 0.0, alpha(BLACK, 0.48));
        draw_texture_ex(
            &self.mine,
            p.x - 41.3,
            p.y - 16.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(72.0, 72.0)),
                ..Default::default()
            },
        );
        // The generated lens centre is (147,104) in the packed256 sprite. Blink starts at placement.
        if s.led_on() {
            let a = if m.armed { 1.0 } else { 0.76 };
            draw_ellipse(
                p.x,
                p.y + 13.25,
                5.2,
                2.3,
                0.0,
                alpha(Color::new(0.95, 0.06, 0.04, 1.0), a),
            );
            draw_ellipse(
                p.x - 1.0,
                p.y + 12.6,
                2.2,
                0.9,
                0.0,
                alpha(Color::new(1.0, 0.63, 0.47, 1.0), a),
            );
        }
    }
    fn effect(&self, s: &State, r: Rect) {
        let Some(a) = s
            .blast
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.42).contains(a))
        else {
            return;
        };
        let p = point(MINE, s.direction, r);
        let f = ((a / 0.42 * 6.0) as usize).min(5);
        draw_texture_ex(
            &self.blast,
            p.x - 82.0,
            p.y - 82.0,
            alpha(WHITE, 0.90),
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(164.0, 164.0)),
                ..Default::default()
            },
        );
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool) {
        for (i, at) in s.targets().into_iter().enumerate() {
            let mut p = point(at, s.direction, r);
            if i == 0 {
                if let Some(t) = s.moved {
                    p = point(s.from, s.direction, r)
                        .lerp(p, ((s.clock - t) / 0.12).clamp(0.0, 1.0));
                }
            }
            if effects && s.hits[i] {
                if let Some(t) = s.blast {
                    p += crate::combat_visuals::hit_reaction_offset(
                        p,
                        point(MINE, s.direction, r),
                        f64::from(s.clock - t),
                        64.0,
                    );
                }
            }
            actor(
                p,
                Visual::Bestiary {
                    number: 12 + i as u8,
                    corpse: false,
                },
            );
        }
        actor(
            point(PLAYER, s.direction, r),
            game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        );
    }
    fn hud(&self, s: &State, r: Rect, hover: bool) {
        draw_rectangle(r.x + 10.0, r.y + 10.0, 325.0, 79.0, alpha(INK, 0.96));
        draw_texture_ex(
            &self.icons[s.skill],
            r.x + 18.0,
            r.y + 24.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_text(
            format!(
                "{} · rang {}",
                if s.skill == 0 { "Actif" } else { "Passif" },
                s.rank()
            ),
            r.x + 60.0,
            r.y + 32.0,
            14.0,
            if s.skill == 1 {
                Color::new(0.38, 0.77, 1.0, 1.0)
            } else {
                DIM
            },
        );
        draw_text(
            format!("Mines simultanées : {} / {}", s.mines.len(), s.capacity()),
            r.x + 60.0,
            r.y + 54.0,
            17.0,
            TEXT,
        );
        draw_text(
            "Pose : 10 énergie · rayon : 1 case",
            r.x + 60.0,
            r.y + 75.0,
            13.0,
            DIM,
        );
        if let Some(t) = s.begun {
            if s.clock - t < 0.4 {
                let p = point(PLAYER, s.direction, r);
                draw_text_bold(
                    "Pose de mine",
                    p.x - 42.0,
                    p.y - 40.0 - (s.clock - t) * 8.0,
                    15.0,
                    alpha(TEXT, 1.0 - (s.clock - t) / 0.4),
                );
            }
        }
        if hover {
            draw_rectangle(r.x + 290.0, r.y + 100.0, 268.0, 108.0, alpha(INK, 0.97));
            draw_text_bold("Mine de feu", r.x + 302.0, r.y + 122.0, 17.0, TEXT);
            draw_text(
                if s.mines.iter().any(|m| m.at == MINE && m.armed) {
                    "Armée"
                } else {
                    "Armement : 1 tour"
                },
                r.x + 302.0,
                r.y + 146.0,
                15.0,
                TEXT,
            );
            draw_text(
                "13 dégâts de feu · rayon 1 case",
                r.x + 302.0,
                r.y + 170.0,
                13.0,
                TEXT,
            );
            draw_text(
                "Le joueur est protégé.",
                r.x + 302.0,
                r.y + 194.0,
                13.0,
                DIM,
            );
        }
        if let Some(a) = s
            .blast
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.65).contains(a))
        {
            for (i, at) in s.targets().into_iter().enumerate() {
                if s.hits[i] {
                    let p = point(at, s.direction, r);
                    let y = p.y - 38.0 - a * 16.0;
                    let c = alpha(Color::new(1.0, 0.67, 0.25, 1.0), 1.0 - a / 0.65);
                    draw_text_bold("13", p.x - 12.0, y, 21.0, c);
                    draw_text("(", p.x - 22.0, y + 14.0, 12.0, c);
                    draw_texture_ex(
                        &self.fire,
                        p.x - 17.0,
                        y + 3.0,
                        alpha(WHITE, 1.0 - a / 0.65),
                        DrawTextureParams {
                            dest_size: Some(vec2(12.0, 12.0)),
                            ..Default::default()
                        },
                    );
                    draw_text("13)", p.x - 1.0, y + 14.0, 12.0, c);
                }
            }
        }
        draw_rectangle(r.x + 5.0, r.y + SIZE - 62.0, 566.0, 57.0, alpha(INK, 0.96));
        draw_text(
            format!(
                "Tour {} · énergie {} · recharge {} · explosions {}",
                s.turn, s.energy, s.cooldown, s.explosions
            ),
            r.x + 12.0,
            r.y + SIZE - 45.0,
            14.0,
            TEXT,
        );
        draw_text(
            if s.layer_changed {
                "Changement de couche : mines retirées."
            } else if s.begun.is_none() {
                "Confirmez la pose · voyant central rouge clignotant"
            } else if s.mines.iter().any(|m| m.at == MINE && !m.armed) {
                "Mine posée · armement au passage du tour"
            } else if s.mines.iter().any(|m| m.at == MINE) {
                "Mine armée · aucun déclenchement sans ennemi"
            } else {
                "Mine consommée · aucun résidu dans cet exemple"
            },
            r.x + 12.0,
            r.y + SIZE - 26.0,
            12.0,
            DIM,
        );
        draw_text(
            format!(
                "PV ennemis {}/{}/{} · joueur 100/100 · remplacées {}",
                s.hp[0], s.hp[1], s.hp[2], s.removed_oldest
            ),
            r.x + 12.0,
            r.y + SIZE - 9.0,
            12.0,
            TEXT,
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool, hover: bool) {
    terrain(&s.map(), s.direction, r, false);
    if s.begun.is_none() {
        preview(&s.area(), s.direction, r, s.clock);
    }
    if hover && s.mines.iter().any(|m| m.at == MINE) {
        preview(&s.area(), s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    clip.apply_for(s.map_index(), s.direction, r);
    for m in &s.mines {
        art.mine(m, s, r);
    }
    if effects {
        art.effect(s, r);
    }
    art.bodies(s, r, effects);
    gl_use_default_material();
    art.hud(s, r, hover);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Capacity,
        Case::Rank12,
        Case::AtArming,
        Case::Layer,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Mines · ANIM-S30", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 370.0, 34.0),
                t,
                true,
            ) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, t) in ["DROITE", "GAUCHE", "HAUT", "BAS"].iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 174.0,
                    179.0 + (i / 2) as f32 * 36.0,
                    166.0,
                    30.0,
                ),
                t,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, c) in cases.iter().enumerate() {
            if button(
                Rect::new(20.0, 270.0 + i as f32 * 35.0, 370.0, 30.0),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 535.0, 350.0, 34.0),
            "Poser la mine",
            s.begun.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20.0, 577.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.step < s.max_steps(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 619.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 619.0, 160.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
            s.tick();
        }
        let r = Rect::new(410.0, 88.0, SIZE, SIZE);
        let p = point(MINE, s.direction, r);
        let (x, y) = mouse_position();
        let hover = s.mines.iter().any(|m| m.at == MINE)
            && (x - p.x).abs() < 32.0
            && (y - p.y).abs() < 32.0;
        scene(&art, &clip, &s, r, true, hover);
        draw_text(
            "Scènes préparées · survolez la mine pour voir son rayon",
            410.0,
            704.0,
            13.0,
            DIM,
        );
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 44, 60, 64, 66, 68, 72, 76, 80, 84, 88, 108, 112, 114, 116, 120, 124, 128, 132,
        136, 148, 158,
    ];
    let hover_frames = [44, 54, 66, 76, 96, 104];
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in [
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Capacity, 2, "CAPACITE"),
            (Case::Rank12, 2, "RANG-12"),
            (Case::AtArming, 2, "ENNEMI-ARMEMENT"),
            (Case::Layer, 2, "COUCHE"),
        ] {
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", s.variant());
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            let mut hovered = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                s.tick();
                if ACTIONS.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"step":s.step,"turn":s.turn,"energy":s.energy,"cooldown":s.cooldown,"rank":s.rank(),"capacity":s.capacity(),"count":s.mines.len(),"mines":s.mines.iter().map(|m|serde_json::json!({"at":[m.at.x,m.at.y],"armed":m.armed,"serial":m.serial})).collect::<Vec<_>>(),"led_on":s.led_on(),"removed_oldest":s.removed_oldest,"primary":[s.primary.x,s.primary.y],"hp":s.hp,"hits":s.hits,"explosions":s.explosions,"layer_changed":s.layer_changed,"player_hp":100,"preview_visible":s.begun.is_none()}));
                let variants = 1
                    + usize::from(controls.contains(&frame))
                    + usize::from(
                        hover_frames.contains(&frame) && s.mines.iter().any(|m| m.at == MINE),
                    );
                for v in 0..variants {
                    let control = v == 1 && controls.contains(&frame);
                    let hover = v > 0 && !control;
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 68 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        22.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{} · {label} · étape {}", s.variant(), s.step),
                        32.0,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &art,
                        &clip,
                        &s,
                        Rect::new(32.0, 88.0, SIZE, SIZE),
                        !control,
                        hover,
                    );
                    draw_text(
                        "Voyant clignotant dès la pose · armement après un tour",
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "Explosion brève · pas de Brûlure ni de résidu dans cette scène",
                        32.0,
                        707.0,
                        12.0,
                        DIM,
                    );
                    let name = if control {
                        format!("control-{frame:03}.rgba")
                    } else if hover {
                        hovered.push(frame);
                        format!("hover-{frame:03}.rgba")
                    } else {
                        format!("{frame:03}.rgba")
                    };
                    super::occult::export(&target, &dir.join(name));
                }
                set_default_camera();
                next_frame().await;
            }
            std::fs::write(
                dir.join("timeline.json"),
                serde_json::to_vec_pretty(&timeline).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    if s.map().blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), direction);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            let p = position(MINE, direction);
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":68+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":true,"quiet":case==Case::Quiet,"control_frames":controls,"hover_frames":hovered,"mine_cell":[p.x,p.y],"action_frames":ACTIONS[..s.max_steps() as usize].to_vec(),"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
