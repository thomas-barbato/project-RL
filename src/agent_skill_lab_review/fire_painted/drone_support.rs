//! ANIM-S44: drone deployment and one prepared Mule pickup, never campaign AI.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["D56", "D57"];
const TITLES: [&str; 2] = ["Invocation de l’Éclaireur", "Invocation de la Mule"];
const PLAYER: GridPos = GridPos::new(4, 6);
const FRAMES: usize = 112;
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#..###..#\n#..#.#..#\n#..###..#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#########",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Deploy,
    Quiet,
    Blocked,
    Occupied,
    Energy,
    Existing,
    Collect,
    Hidden,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Deploy => "DEPLOIEMENT",
            Self::Quiet => "ATTENTE",
            Self::Blocked => "MUR",
            Self::Occupied => "OCCUPEE",
            Self::Energy => "ENERGIE",
            Self::Existing => "EXISTANT",
            Self::Collect => "COLLECTE",
            Self::Hidden => "CACHE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Deploy => "Déploiement",
            Self::Quiet => "Attente sans tour",
            Self::Blocked => "Aucune case libre",
            Self::Occupied => "Case choisie occupée",
            Self::Energy => "Énergie insuffisante",
            Self::Existing => "Modèle déjà déployé",
            Self::Collect => "Ramassage par la Mule",
            Self::Hidden => "Loot hors de votre vue",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    moved: Option<f32>,
    pickup: Option<f32>,
    turn: u16,
    energy: u16,
    cooldown: u16,
    deployed: bool,
    deployments: u16,
    spawn: GridPos,
    drone: GridPos,
    loot_ground: bool,
    cargo: bool,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        let spawn = if case == Case::Hidden {
            GridPos::new(3, 5)
        } else {
            GridPos::new(4, 5)
        };
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            started: None,
            moved: None,
            pickup: None,
            turn: 0,
            energy: if case == Case::Energy { 9 } else { 100 },
            cooldown: 0,
            deployed: case == Case::Existing,
            deployments: 0,
            spawn,
            drone: spawn,
            loot_ground: matches!(case, Case::Collect | Case::Hidden),
            cargo: false,
        }
    }
    fn map_index(&self) -> usize {
        if self.case == Case::Blocked {
            1
        } else if self.case == Case::Hidden {
            2
        } else {
            0
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn loot(&self) -> GridPos {
        if self.case == Case::Hidden {
            GridPos::new(3, 3)
        } else {
            GridPos::new(4, 4)
        }
    }
    fn visible_loot(&self) -> bool {
        let map = self.map();
        let dx = self.drone.x - self.loot().x;
        let dy = self.drone.y - self.loot().y;
        let radius = i32::from(parameter_for("D56", "vision_radius_cells", 1));
        has_line_of_sight(&map, PLAYER, self.loot(), true)
            || (self.skill == 0
                && self.deployed
                && dx * dx + dy * dy <= radius * radius
                && has_line_of_sight(&map, self.drone, self.loot(), true))
    }
    fn reason(&self) -> Option<&'static str> {
        let dx = (self.spawn.x - PLAYER.x).abs();
        let dy = (self.spawn.y - PLAYER.y).abs();
        if self.deployed {
            Some("Ce modèle est déjà déployé.")
        } else if self.energy < parameter_for(IDS[self.skill], "energy", 1) {
            Some("Énergie insuffisante.")
        } else if !self.map().is_walkable(self.spawn)
            || dx.max(dy) != 1
            || !has_line_of_sight(&self.map(), PLAYER, self.spawn, true)
        {
            Some("Aucune arrivée possible sur cette case.")
        } else if self.case == Case::Occupied {
            Some("Case occupée.")
        } else {
            None
        }
    }
    fn deploy(&mut self) {
        if self.reason().is_some() {
            return;
        }
        self.energy -= parameter_for(IDS[self.skill], "energy", 1);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 1);
        self.deployed = true;
        self.deployments += 1;
        self.started = Some(self.clock);
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        // Prepared collection order, issued after deployment. This does not
        // define the pending command scheduler or postpone newborn drone AI.
        if self.skill == 1 && self.case == Case::Collect && self.visible_loot() {
            if self.turn == 1 {
                self.drone = self.loot();
                self.moved = Some(self.clock);
            }
            if self.turn == 2 && self.drone == self.loot() && self.loot_ground && !self.cargo {
                self.loot_ground = false;
                self.cargo = true;
                self.pickup = Some(self.clock);
            }
        }
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,"launched":self.started.is_some(),"deployed":self.deployed,"deployments":self.deployments,"actor_identity":if self.deployed {Some(if self.case==Case::Existing {"existing-drone"}else{"new-drone"})}else{None},"spawn":[self.spawn.x,self.spawn.y],"drone":[self.drone.x,self.drone.y],"preview":self.started.is_none()&&self.reason().is_none(),"refusal":if self.started.is_none(){self.reason()}else{None},"ground_item":self.loot_ground,"loot_visible":self.visible_loot(),"cargo":self.cargo,"item_identity":"review:camp-knife-01","item_name":"Couteau de camp","pickup_events":u8::from(self.pickup.is_some()),"enemy_hp":100,"drone_attacks":0})
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let at = position(at, direction);
    vec2(
        rect.x + (at.x as f32 + 0.5) * 64.,
        rect.y + (at.y as f32 + 0.5) * 64.,
    )
}
fn player_visual() -> Visual {
    game_art::wear(
        game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_technique".into())],
    )
}
fn arrival(p: Vec2, age: f32) {
    if !(0.0..0.34).contains(&age) {
        return;
    }
    let t = age / 0.34;
    let opacity = (1. - t).powf(0.6);
    let light = Color::new(0.82, 0.96, 1., opacity);
    let halo = Color::new(0.27, 0.67, 0.81, opacity * 0.24);
    for offset in [-12., 0., 12.] {
        let y = p.y + 20. - (1. - t) * offset;
        draw_ellipse(p.x, y, 14. + t * 21., 5. + t * 5., 0., halo);
        draw_ellipse_lines(p.x, y, 14. + t * 21., 5. + t * 5., 0., 2.4, light);
    }
    for n in 0..8 {
        let a = n as f32 * std::f32::consts::TAU / 8. + t * 0.5;
        let d = vec2(a.cos(), a.sin() * 0.65);
        let from = p + d * (16. + t * 20.) - vec2(0., (1. - t) * 18.);
        let to = from + d * (4. + (1. - t) * 9.);
        draw_line(from.x, from.y, to.x, to.y, 6., halo);
        draw_line(from.x, from.y, to.x, to.y, 2.5, light);
    }
}
fn scene(clip: &Clip, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    if s.started.is_none() && s.reason().is_none() {
        preview(&[s.spawn], s.direction, rect, s.clock);
    }
    if s.loot_ground && s.visible_loot() {
        let p = point(s.loot(), s.direction, rect);
        game_art::draw_world(
            Rect::new(p.x - 18., p.y - 18., 36., 36.),
            Visual::ColoredItem(
                game_art::item("core:melee_couteau_de_camp").unwrap(),
                project_rl::entity::EquipmentPalette::Original,
            ),
            1.,
        );
    }
    actor(point(PLAYER, s.direction, rect), player_visual());
    let hostile = if s.case == Case::Occupied {
        s.spawn
    } else {
        GridPos::new(6, 2)
    };
    enemy(point(hostile, s.direction, rect));
    if s.deployed {
        let mut p = point(s.drone, s.direction, rect);
        if let Some(time) = s.moved {
            p = point(s.spawn, s.direction, rect).lerp(p, ((s.clock - time) / 0.16).clamp(0., 1.));
        }
        let opacity = s
            .started
            .map_or(1., |time| ((s.clock - time) / 0.16).clamp(0., 1.));
        game_art::draw_world(
            Rect::new(p.x - 32., p.y - 32., 64., 64.),
            Visual::Bestiary {
                number: 56 + s.skill as u8,
                corpse: false,
            },
            opacity,
        );
        if let Some(time) = s.started {
            clip.apply_for(s.map_index(), s.direction, rect);
            arrival(p, s.clock - time);
            gl_use_default_material();
        }
        if let Some(age) = s.pickup.map(|time| s.clock - time).filter(|age| *age < 1.2) {
            let text = "Couteau de camp";
            let width = measure_text(text, None, 20, 1.).width;
            let at = p + vec2(-width / 2., -55. - age * 11.);
            draw_text_bold(text, at.x, at.y, 20., alpha(TEXT, (1.2 - age).min(1.)));
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    vec![
        (Case::Deploy, 0, "DROITE"),
        (Case::Deploy, 1, "GAUCHE"),
        (Case::Deploy, 2, "HAUT"),
        (Case::Deploy, 3, "BAS"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Blocked, 2, "MUR"),
        (Case::Occupied, 2, "OCCUPEE"),
        (Case::Energy, 2, "ENERGIE"),
        (Case::Existing, 2, "EXISTANT"),
        (Case::Collect, 2, "COLLECTE"),
        (Case::Hidden, 2, "CACHE"),
    ]
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 850.);
    for _ in 0..4 {
        next_frame().await;
    }
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, Case::Deploy, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Drones de soutien · 97–98", 20., 35., 24., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 65. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    155. + (i / 2) as f32 * 37.,
                    156.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, case) in [
            Case::Deploy,
            Case::Quiet,
            Case::Blocked,
            Case::Occupied,
            Case::Energy,
            Case::Existing,
            Case::Collect,
            Case::Hidden,
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 245. + i as f32 * 35., 330., 31.),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        let rect = Rect::new(400., 88., 576., 576.);
        if s.started.is_none() && is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            if rect.contains(vec2(x, y)) {
                let p = GridPos::new(((x - rect.x) / 64.) as i32, ((y - rect.y) / 64.) as i32);
                s.spawn = position(
                    p,
                    match s.direction {
                        0 => 1,
                        1 => 0,
                        d => d,
                    },
                );
                s.drone = s.spawn;
            }
        }
        if button(
            Rect::new(20., 540., 330., 34.),
            "Déployer",
            s.reason().is_none(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.deploy();
        }
        if button(
            Rect::new(20., 582., 330., 34.),
            "Passer un tour",
            s.started.is_some(),
        ) {
            s.advance();
        }
        if button(Rect::new(20., 624., 158., 34.), "Rejouer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(186., 624., 164., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&clip, &s, rect);
        draw_text_bold(
            format!(
                "Énergie {} / 100 · recharge {} tours · tour {}",
                s.energy, s.cooldown, s.turn
            ),
            400.,
            704.,
            18.,
            TEXT,
        );
        draw_text(
            format!(
                "Drone présent : {} · objet dans la Mule : {}",
                s.deployed, s.cargo
            ),
            400.,
            734.,
            17.,
            TEXT,
        );
        draw_text(
            s.reason()
                .filter(|_| s.started.is_none())
                .unwrap_or("Les deux modèles restent présents et n’attaquent pas."),
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
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let only = std::env::args().find_map(|a| a.strip_prefix("--review-case=").map(str::to_owned));
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases() {
            if only.as_deref().is_some_and(|wanted| wanted != label) {
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
                    s.deploy();
                }
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&clip, &s, Rect::new(0., 0., 576., 576.));
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
            entries.push(serde_json::json!({"id":IDS[skill],"number":97+skill,"title":TITLES[skill],"variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
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
    fn deployment_is_atomic_on_refusal_and_keeps_one_persistent_noncombat_drone() {
        for skill in 0..2 {
            for case in [Case::Blocked, Case::Occupied, Case::Energy, Case::Existing] {
                let mut s = State::new(skill, case, 2);
                let before = s.snapshot();
                s.deploy();
                s.advance();
                assert_eq!(s.snapshot(), before);
            }
            let mut s = State::new(skill, Case::Deploy, 2);
            s.deploy();
            assert_eq!((s.energy, s.cooldown, s.deployments), (90, 1, 1));
            let before = s.snapshot();
            s.deploy();
            assert_eq!(s.snapshot(), before);
            s.clock = 100.;
            assert!(s.deployed);
            assert_eq!(s.cooldown, 1);
            for _ in 0..9 {
                s.advance();
            }
            assert!(s.deployed);
            assert_eq!(s.deployments, 1);
            assert_eq!(s.cooldown, 0);
        }
    }
    #[test]
    fn mule_moves_then_transfers_one_existing_item_and_ignores_hidden_loot() {
        let mut s = State::new(1, Case::Collect, 2);
        s.deploy();
        assert!(s.loot_ground && !s.cargo);
        s.advance();
        assert_eq!(s.drone, s.loot());
        assert!(s.loot_ground && !s.cargo);
        s.advance();
        assert!(!s.loot_ground && s.cargo);
        assert!(s.pickup.is_some());
        let event = s.pickup;
        s.advance();
        assert_eq!(s.pickup, event);
        let mut scout = State::new(0, Case::Collect, 2);
        scout.deploy();
        for _ in 0..3 {
            scout.advance();
        }
        assert!(scout.loot_ground && !scout.cargo && scout.pickup.is_none());
        let mut hidden = State::new(1, Case::Hidden, 2);
        assert!(!hidden.visible_loot());
        assert!(has_line_of_sight(
            &hidden.map(),
            hidden.spawn,
            hidden.loot(),
            true
        ));
        hidden.deploy();
        for _ in 0..3 {
            hidden.advance();
        }
        assert!(hidden.loot_ground && !hidden.cargo && hidden.pickup.is_none());
        assert_eq!(hidden.drone, hidden.spawn);
        let mut scout = State::new(0, Case::Hidden, 2);
        assert!(!scout.visible_loot());
        scout.deploy();
        assert!(scout.deployed && scout.visible_loot());
        assert!(scout.loot_ground && !scout.cargo);
    }
}
