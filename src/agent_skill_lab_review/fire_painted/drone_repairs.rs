//! ANIM-S48: blue repair feedback, isolated from campaign combat and saves.
//! HP totals, positions and refusal boundaries are explicit visual fixtures.
//! Maintenance uses its accepted rank-11 example, not a new progression formula.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

const IDS: [&str; 2] = ["DP01", "DR01"];
const TITLES: [&str; 2] = ["Maintenance des drones", "Réparation ciblée"];
const PLAYER: GridPos = GridPos::new(4, 7);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#########",
];
const FRAMES: usize = 168;
const FPS: usize = 24;
const START: usize = 12;
const TURNS: [usize; 6] = [30, 48, 66, 90, 114, 138];
const MAX_HP: u16 = 200;
const REPAIR_TICK: u16 = 14; // Accepted 7% example, with an exact integer result.
const ACTIVE_HEAL: u16 = 46;
const BLUE: Color = Color::new(0.12, 0.48, 0.96, 1.);
const PALE: Color = Color::new(0.72, 0.88, 1., 1.);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Repair,
    Interrupted,
    Cap,
    Wall,
    Recalled,
    Destroyed,
    Energy,
    Quiet,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Repair => "REPARATION",
            Self::Interrupted => "HOSTILE",
            Self::Cap => "PLAFOND",
            Self::Wall => "CLOISON",
            Self::Recalled => "RAPPELE",
            Self::Destroyed => "DETRUIT",
            Self::Energy => "ENERGIE",
            Self::Quiet => "ATTENTE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Repair => "Réparations",
            Self::Interrupted => "Hostile repéré",
            Self::Cap => "Un seul PV manquant",
            Self::Wall => "Cloison",
            Self::Recalled => "Drones rappelés",
            Self::Destroyed => "Drones détruits",
            Self::Energy => "Énergie insuffisante",
            Self::Quiet => "Attente sans nouveau tour",
        }
    }
}
struct Repair {
    at: f32,
    target: usize,
    amount: u16,
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    turn: u16,
    calm: u16,
    hp: [u16; 3],
    energy: u16,
    cooldown: u16,
    events: Vec<Repair>,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            started: None,
            turn: 0,
            calm: 0,
            hp: if case == Case::Destroyed {
                [0; 3]
            } else if case == Case::Cap {
                [199; 3]
            } else {
                [70, 110, 40]
            },
            energy: if case == Case::Energy { 11 } else { 100 },
            cooldown: 0,
            events: Vec::new(),
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[usize::from(self.case == Case::Wall)]).unwrap()
    }
    fn positions(&self) -> [GridPos; 3] {
        [GridPos::new(4, 4), GridPos::new(2, 5), GridPos::new(6, 5)]
    }
    fn deployed(&self) -> bool {
        self.case != Case::Recalled
    }
    fn hostile(&self) -> bool {
        self.case == Case::Interrupted && (self.skill == 1 || self.turn >= 5)
    }
    fn reason(&self) -> Option<&'static str> {
        if self.skill == 0 {
            return None;
        }
        if !self.deployed() {
            Some("Robot rappelé.")
        } else if self.hp[0] == 0 {
            Some("Robot détruit.")
        } else if self.energy < 12 {
            Some("Énergie insuffisante.")
        } else if !has_line_of_sight(&self.map(), PLAYER, self.positions()[0], true) {
            Some("La cloison bloque le soin ciblé.")
        } else {
            None
        }
    }
    fn restore(&mut self, target: usize, amount: u16, at: f32) {
        if !self.deployed() || self.hp[target] == 0 {
            return;
        }
        let gain = amount.min(MAX_HP - self.hp[target]);
        if gain == 0 {
            return;
        }
        self.hp[target] += gain;
        self.events.push(Repair {
            at,
            target,
            amount: gain,
        });
    }
    fn start(&mut self) {
        if self.started.is_some() || self.reason().is_some() {
            return;
        }
        self.started = Some(self.clock);
        if self.skill == 1 {
            self.energy -= 12;
            self.cooldown = 4;
            // HP change is immediate; the 100ms travel is presentation only.
            self.restore(0, ACTIVE_HEAL, self.clock + 0.10);
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.skill == 0 {
            if self.hostile() {
                self.calm = 0;
                return;
            }
            self.calm += 1;
            if self.calm >= 4 {
                for target in 0..3 {
                    self.restore(target, REPAIR_TICK, self.clock);
                }
            }
        }
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"started":self.started.is_some(),"hp":self.hp,"maximum_hp":MAX_HP,"energy":self.energy,"cooldown":self.cooldown,"quiet_turns":self.calm,"hostile_visible":self.hostile(),"deployed":self.deployed(),"events":self.events.len(),"repaired_total":self.events.iter().map(|e|u32::from(e.amount)).sum::<u32>(),"preview":self.skill==1&&self.started.is_none()&&self.reason().is_none(),"refusal":if self.started.is_none(){self.reason()}else{None},"blood_events":0})
    }
}
fn point(at: GridPos, dir: usize, rect: Rect) -> Vec2 {
    let p = position(at, dir);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn player_visual() -> Visual {
    game_art::wear(
        game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_technique".into())],
    )
}
struct Art {
    drones: [Texture2D; 3],
}
impl Art {
    fn load() -> Self {
        fn decode(data: &[u8]) -> Texture2D {
            let t = Texture2D::from_file_with_format(data, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Nearest);
            t
        }
        Self {
            drones: [
                decode(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
                )),
                decode(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
                )),
                decode(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-93/traqueur-palette02/D58-PAL02-native64.png"
                )),
            ],
        }
    }
    fn drone(&self, i: usize, p: Vec2) {
        draw_ellipse(
            p.x,
            p.y + 30.,
            if i == 0 { 30. } else { 23. },
            4.,
            0.,
            alpha(BLACK, 0.3),
        );
        let half = if i == 0 { 64. } else { 32. };
        draw_texture(&self.drones[i], p.x - half, p.y - half, WHITE);
    }
}
/// Hard-edged illustrated light, with the body unobscured and no full-tile fill.
fn repair_back(p: Vec2, age: f32, active: bool) {
    let duration = if active { 0.62 } else { 0.46 };
    if !(0.0..duration).contains(&age) {
        return;
    }
    let t = age / duration;
    let f = (1. - t).sqrt();
    let radius = if active { 30. } else { 24. };
    draw_ellipse(
        p.x,
        p.y + 27.,
        radius + t * 10.,
        6. + t * 3.,
        0.,
        alpha(BLUE, 0.20 * f),
    );
    draw_ellipse_lines(
        p.x,
        p.y + 27.,
        radius + t * 10.,
        6. + t * 3.,
        0.,
        2.4,
        alpha(PALE, 0.65 * f),
    );
    for side in [-1., 1.] {
        let x = p.x + side * (20. + t * 10.);
        let pts = [
            vec2(x, p.y + 23.),
            vec2(x + side * 6., p.y + 8. - t * 16.),
            vec2(x + side * 2., p.y - 13. - t * 18.),
        ];
        for line in pts.windows(2) {
            draw_line(
                line[0].x,
                line[0].y,
                line[1].x,
                line[1].y,
                7.,
                alpha(INK, f * 0.7),
            );
            draw_line(
                line[0].x,
                line[0].y,
                line[1].x,
                line[1].y,
                4.,
                alpha(BLUE, f),
            );
            draw_line(
                line[0].x,
                line[0].y,
                line[1].x,
                line[1].y,
                1.3,
                alpha(PALE, f),
            );
        }
    }
}
fn repair_front(p: Vec2, age: f32, active: bool) {
    let duration = if active { 0.62 } else { 0.46 };
    if !(0.0..duration).contains(&age) {
        return;
    }
    let t = age / duration;
    let f = (1. - t).sqrt();
    for i in 0..if active { 8 } else { 5 } {
        let a = i as f32 * 2.399 + 0.2;
        let d = vec2(a.cos(), a.sin() * 0.8);
        let q = p + vec2(0., -3.) + d * (8. + t * 23.) - vec2(0., t * 9.);
        let end = q + d * (3. + t * 2.);
        draw_line(q.x, q.y, end.x, end.y, 4., alpha(INK, f));
        draw_line(q.x, q.y, end.x, end.y, 2.3, alpha(PALE, f));
        draw_circle(q.x, q.y, 1.5, alpha(BLUE, f));
    }
    let y = p.y + 19. - t * 42.;
    draw_line(p.x - 11., y, p.x + 11., y, 4., alpha(INK, f * 0.7));
    draw_line(p.x - 10., y, p.x + 10., y, 2., alpha(PALE, f * 0.85));
}
fn impulse(from: Vec2, to: Vec2, age: f32) {
    if !(0.0..0.10).contains(&age) {
        return;
    }
    let p = from.lerp(to, age / 0.10);
    let d = (to - from).normalize_or_zero();
    draw_line(
        p.x - d.x * 22.,
        p.y - d.y * 22.,
        p.x,
        p.y,
        8.,
        alpha(INK, 0.8),
    );
    draw_line(p.x - d.x * 20., p.y - d.y * 20., p.x, p.y, 5., BLUE);
    draw_line(p.x - d.x * 13., p.y - d.y * 13., p.x, p.y, 2., PALE);
    draw_circle(p.x, p.y, 4., PALE);
}
fn number(p: Vec2, amount: u16, age: f32) {
    if !(0.0..0.75).contains(&age) {
        return;
    }
    let text = format!("+{amount}");
    let w = measure_text(&text, None, 23, 1.).width;
    draw_text_bold(
        text,
        p.x - w / 2.,
        p.y - 42. - age * 22.,
        23.,
        alpha(PALE, ((0.75 - age) * 4.).min(1.)),
    );
}
fn scene(art: &Art, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    let positions = s.positions();
    if s.skill == 1 && s.started.is_none() && s.reason().is_none() {
        preview(&[positions[0]], s.direction, rect, s.clock);
    }
    for e in &s.events {
        repair_back(
            point(positions[e.target], s.direction, rect),
            s.clock - e.at,
            s.skill == 1,
        );
    }
    let player = point(PLAYER, s.direction, rect);
    actor(player, player_visual());
    if s.hostile() {
        actor(
            point(GridPos::new(4, 2), s.direction, rect),
            Visual::Bestiary {
                number: 12,
                corpse: false,
            },
        );
    }
    let mut order: Vec<_> = (0..3).collect();
    order.sort_by(|&a, &b| {
        point(positions[a], s.direction, rect)
            .y
            .total_cmp(&point(positions[b], s.direction, rect).y)
    });
    for i in order {
        if !s.deployed() || s.hp[i] == 0 {
            continue;
        }
        let p = point(positions[i], s.direction, rect);
        art.drone(i, p);
        draw_rectangle(p.x - 23., p.y + 34., 46., 5., INK);
        draw_rectangle(
            p.x - 22.,
            p.y + 35.,
            44. * f32::from(s.hp[i]) / f32::from(MAX_HP),
            3.,
            Color::new(0.28, 0.58, 0.34, 1.),
        );
        for e in s.events.iter().filter(|e| e.target == i) {
            repair_front(p, s.clock - e.at, s.skill == 1);
            number(p, e.amount, s.clock - e.at);
        }
    }
    if let Some(start) = s.started {
        if s.skill == 1 {
            impulse(
                player - vec2(0., 7.),
                point(positions[0], s.direction, rect),
                s.clock - start,
            );
        }
    }
    if let Some(e) = s.events.last() {
        let age = s.clock - e.at;
        if (0.0..0.75).contains(&age) {
            draw_text_bold(
                TITLES[s.skill],
                player.x - 110.,
                player.y - 64.,
                18.,
                alpha(PALE, ((0.75 - age) * 4.).min(1.)),
            );
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    vec![
        (Case::Repair, 0, "DROITE"),
        (Case::Repair, 1, "GAUCHE"),
        (Case::Repair, 2, "HAUT"),
        (Case::Repair, 3, "BAS"),
        (Case::Interrupted, 2, "HOSTILE"),
        (Case::Cap, 2, "PLAFOND"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Recalled, 2, "RAPPELE"),
        (Case::Destroyed, 2, "DETRUIT"),
        (Case::Energy, 2, "ENERGIE"),
        (Case::Quiet, 2, "ATTENTE"),
    ]
}
pub(crate) async fn run() {
    request_new_screen_size(1100., 850.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut s = State::new(0, Case::Repair, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("105–106 · Réparation des drones", 20., 34., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 58. + i as f32 * 38., 330., 33.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, (case, _, _)) in cases()
            .into_iter()
            .filter(|(_, dir, _)| *dir == 2)
            .enumerate()
        {
            if button(
                Rect::new(20., 150. + i as f32 * 29., 330., 26.),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 167.,
                    430. + (i / 2) as f32 * 37.,
                    157.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        if button(
            Rect::new(20., 520., 330., 34.),
            "Démarrer l’exemple",
            s.started.is_none() && s.reason().is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.start();
        }
        if button(
            Rect::new(20., 564., 330., 34.),
            "Passer un tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20., 608., 155., 34.), "Rejouer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(185., 608., 165., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, Rect::new(400., 72., 576., 576.));
        draw_text_bold(
            format!(
                "Tour {} · drones : {} / {} / {} PV",
                s.turn, s.hp[0], s.hp[1], s.hp[2]
            ),
            400.,
            695.,
            20.,
            TEXT,
        );
        draw_text(
            format!(
                "Énergie {} / 100 · recharge {} tours · calme {}",
                s.energy, s.cooldown, s.calm
            ),
            400.,
            730.,
            18.,
            TEXT,
        );
        draw_text(
            s.reason()
                .unwrap_or("Profil d’essai ; effets de campagne non raccordés."),
            400.,
            765.,
            17.,
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
    let art = Art::load();
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases() {
            let mut s = State::new(skill, case, direction);
            let mut known = BTreeMap::new();
            let mut images = Vec::new();
            let mut indices = Vec::new();
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / FPS as f32;
                if frame == START {
                    s.start();
                }
                if case != Case::Quiet && TURNS.contains(&frame) {
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
                let index = if let Some(&n) = known.get(&h) {
                    n
                } else {
                    let n = images.len();
                    known.insert(h, n);
                    images.push(im);
                    n
                };
                indices.push(index);
                set_default_camera();
                if frame % 8 == 0 {
                    next_frame().await;
                }
            }
            let folder = format!("{}-{label}", IDS[skill]);
            let path = output.join(&folder);
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            let cols = 6;
            let rows = images.len().div_ceil(cols);
            let mut atlas = Image::gen_image_color((576 * cols) as u16, (576 * rows) as u16, INK);
            for (i, im) in images.into_iter().enumerate() {
                for y in 0..576 {
                    let k = (((i / cols) * 576 + y) * 576 * cols + (i % cols) * 576) * 4;
                    atlas.bytes[k..k + 576 * 4]
                        .copy_from_slice(&im.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
                }
            }
            let mut file = std::io::BufWriter::new(
                std::fs::File::create(path.join("atlas.ppm")).map_err(|e| e.to_string())?,
            );
            write!(file, "P6\n{} {}\n255\n", atlas.width, atlas.height)
                .map_err(|e| e.to_string())?;
            let mut row = Vec::with_capacity(usize::from(atlas.width) * 3);
            for rgba in atlas.bytes.chunks_exact(usize::from(atlas.width) * 4).rev() {
                row.clear();
                for p in rgba.chunks_exact(4) {
                    row.extend_from_slice(&p[..3]);
                }
                file.write_all(&row).map_err(|e| e.to_string())?;
            }
            file.flush().map_err(|e| e.to_string())?;
            entries.push(serde_json::json!({"id":IDS[skill],"number":105+skill,"variant":format!("{}-A01-{label}",IDS[skill]),"title":TITLES[skill],"case":case.label(),"label":label,"folder":folder,"direction":direction,"frames":FRAMES,"fps":FPS,"start_frame":START,"turn_frames":TURNS,"columns":cols,"unique_images":known.len(),"indices":indices,"timeline":timeline}));
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
    fn drone_repairs_maintenance_waits_and_interrupts_without_wall_clock_actions() {
        let mut s = State::new(0, Case::Interrupted, 2);
        s.start();
        for _ in 0..3 {
            s.advance();
            assert_eq!(s.hp, [70, 110, 40]);
        }
        s.advance();
        assert_eq!(s.hp, [84, 124, 54]);
        let before = s.snapshot();
        s.clock = 999.;
        assert_eq!(s.snapshot(), before);
        s.advance();
        assert_eq!(s.hp, [84, 124, 54]);
        assert_eq!(s.calm, 0);
        s.advance();
        assert_eq!(s.hp, [84, 124, 54]);
        assert_eq!(s.energy, 100);
    }
    #[test]
    fn drone_repairs_active_is_single_target_immediate_capped_and_costed_once() {
        let mut s = State::new(1, Case::Repair, 2);
        s.start();
        assert_eq!(s.hp, [116, 110, 40]);
        assert_eq!(s.energy, 88);
        assert_eq!(s.cooldown, 4);
        s.start();
        s.clock = 400.;
        assert_eq!(s.hp, [116, 110, 40]);
        assert_eq!(s.energy, 88);
        let mut cap = State::new(1, Case::Cap, 2);
        cap.start();
        assert_eq!(cap.hp, [200, 199, 199]);
        assert_eq!(cap.events[0].amount, 1);
    }
    #[test]
    fn drone_repairs_unavailable_targets_never_revive_or_receive_maintenance() {
        for case in [Case::Recalled, Case::Destroyed] {
            for skill in 0..2 {
                let mut s = State::new(skill, case, 2);
                let hp = s.hp;
                s.start();
                for _ in 0..6 {
                    s.advance();
                }
                assert_eq!(s.hp, hp);
                assert!(s.events.is_empty());
                assert_eq!(s.energy, 100);
            }
        }
        for case in [Case::Energy, Case::Wall] {
            let mut s = State::new(1, case, 2);
            let before = s.snapshot();
            s.start();
            assert_eq!(s.snapshot(), before);
        }
    }
}
