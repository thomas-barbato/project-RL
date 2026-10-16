//! ZONE-GROWTH02 / ZONE-VISION02: catalogue-driven range and FOV laboratory.
//! Accepted twelve-cell player vision with an eight-cell historical comparison.
use super::*;
use project_rl::skills::spatial::{Area, affected_cells, in_cast_range};
use project_rl::world::{FieldOfViewRules, Map, compute_visible_tiles, has_line_of_sight};
use serde_json::{Value, json};

const COLS: usize = 31;
const ROWS: usize = 25;
const TILE: f32 = 64.0;
const CANDIDATE_VISION: u16 = 12;
const CASES: [&str; 4] = [
    "Salle ouverte",
    "Obstacle isolé",
    "Cloison complète",
    "Couloir puis pièce",
];

fn catalogue() -> &'static Value {
    static DATA: std::sync::OnceLock<Value> = std::sync::OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../../assets/skills/agent/catalogue.json")).unwrap()
    })
}
fn reviewed() -> Vec<&'static Value> {
    catalogue()["skills"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|s| {
            s["parameters"]
                .as_object()
                .unwrap()
                .values()
                .any(|p| p["spatial_review"] == "ZONE-GROWTH02")
                || s["centred_damage"].is_object()
        })
        .collect()
}
fn val(skill: &Value, key: &str, rank: u16) -> u16 {
    if skill["parameters"][key].is_null() {
        0
    } else {
        let value = parameter_for(skill["id"].as_str().unwrap(), key, rank);
        if key == "zone_width_cells" && skill["affliction_area"]["decision"] == "AFFLICTIONS-ZONE01"
        {
            value.min(
                skill["parameters"][key]["maximum"]
                    .as_u64()
                    .unwrap_or(u16::MAX.into()) as u16,
            )
        } else {
            value
        }
    }
}
fn range(skill: &Value, rank: u16) -> u16 {
    if skill["id"] == "TR64" {
        return 0;
    } // This area's binding is Pulsation occulte.
    for key in [
        "range_cells",
        "base_throw_range_cells",
        "placement_range_cells",
        "center_range_cells",
        "command_range_cells",
        "maximum_drone_distance_cells",
        "maximum_target_distance_cells",
        "cone_range_cells",
        "maximum_wave_travel_cells",
        "bolt_range_cells",
        "movement_cells",
        "leap_cells",
    ] {
        if !skill["parameters"][key].is_null() {
            return val(skill, key, rank);
        }
    }
    if !skill["parameters"]["weapon_range_bonus_cells"].is_null() {
        let weapon = 6u16.saturating_add(val(skill, "weapon_range_bonus_cells", rank));
        let cap = val(skill, "range_skill_limit_cells", rank);
        return if cap > 0 { weapon.min(cap) } else { weapon };
    }
    0
}
fn shape(skill: &Value, rank: u16) -> Area {
    let id = skill["id"].as_str().unwrap();
    if ["FR02", "FR05", "FE05"].contains(&id) {
        return Area::Disc {
            radius: val(skill, "radius_cells", rank),
        };
    }
    if id == "HL03" {
        return Area::Disc {
            radius: val(skill, "aim_zone_radius_cells", rank),
        };
    }
    if ["FR06", "HY07"].contains(&id) {
        return Area::Diamond {
            radius: val(skill, "radius_cells", rank),
        };
    }
    if id == "HL02" {
        return Area::Diamond {
            radius: val(skill, "stun_radius_cells", rank),
        };
    }
    if id == "OC04" {
        return Area::Ring {
            radius: val(skill, "ritual_radius_cells", rank),
        };
    }
    for key in [
        "radius_cells",
        "aim_zone_radius_cells",
        "stun_radius_cells",
        "pulse_radius_cells",
    ] {
        if !skill["parameters"][key].is_null() {
            return Area::Square {
                radius: val(skill, key, rank),
            };
        }
    }
    if !skill["parameters"]["zone_width_cells"].is_null() {
        return Area::Square {
            radius: val(skill, "zone_width_cells", rank) / 2,
        };
    }
    if !skill["parameters"]["maximum_wall_length_cells"].is_null() {
        return Area::Wall {
            length: val(skill, "maximum_wall_length_cells", rank),
        };
    }
    if !skill["parameters"]["cone_range_cells"].is_null()
        || !skill["parameters"]["cone_angle_degrees"].is_null()
    {
        return Area::Cone {
            length: range(skill, rank),
        };
    }
    if !skill["parameters"]["wave_width_cells"].is_null() {
        return Area::Line {
            length: val(skill, "maximum_wave_travel_cells", rank),
            width: val(skill, "wave_width_cells", rank),
        };
    }
    Area::Point
}
fn area_json(area: Area) -> Value {
    match area {
        Area::Point => json!({"kind":"point"}),
        Area::Square { radius } => json!({"kind":"square","radius":radius}),
        Area::Disc { radius } => json!({"kind":"disc","radius":radius}),
        Area::Diamond { radius } => json!({"kind":"diamond","radius":radius}),
        Area::Ring { radius } => json!({"kind":"ring","radius":radius}),
        Area::Wall { length } => json!({"kind":"wall","length":length}),
        Area::Line { length, width } => json!({"kind":"line","length":length,"width":width}),
        Area::Cone { length } => json!({"kind":"cone","length":length}),
    }
}
fn map_source(case: usize) -> String {
    let mut rows = vec![vec!['.'; COLS]; ROWS];
    for y in 0..ROWS {
        for x in 0..COLS {
            if x == 0 || y == 0 || x == COLS - 1 || y == ROWS - 1 {
                rows[y][x] = '#';
            }
            if case == 1 && x == 18 && (10..=14).contains(&y) {
                rows[y][x] = '#';
            }
            if case == 2 && x == 18 {
                rows[y][x] = '#';
            }
            if case == 3 && x < 18 && !(11..=13).contains(&y) {
                rows[y][x] = '#';
            }
        }
    }
    rows.into_iter()
        .map(|r| r.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
fn coords(cells: &[GridPos]) -> Vec<[i32; 2]> {
    cells.iter().map(|p| [p.x, p.y]).collect()
}
struct State {
    skill: usize,
    learned: u16,
    bonus: u16,
    case: usize,
    vision: u16,
    player: GridPos,
    target: GridPos,
    direction: usize,
    camera: Vec2,
    confirmed: bool,
}
impl State {
    fn new() -> Self {
        Self {
            skill: reviewed().iter().position(|s| s["id"] == "A02").unwrap(),
            learned: 10,
            bonus: 3,
            case: 0,
            vision: CANDIDATE_VISION,
            player: GridPos::new(12, 12),
            target: GridPos::new(20, 12),
            direction: 0,
            camera: vec2(9.0, 7.0),
            confirmed: false,
        }
    }
    fn rank(&self) -> u16 {
        self.learned + self.bonus
    }
    fn map(&self) -> Map {
        Map::from_ascii(&map_source(self.case)).unwrap()
    }
    fn skill(&self) -> &'static Value {
        reviewed()[self.skill]
    }
    fn dir(&self) -> (i32, i32) {
        [(1, 0), (-1, 0), (0, -1), (0, 1)][self.direction]
    }
    fn visible(&self) -> std::collections::BTreeSet<GridPos> {
        compute_visible_tiles(
            &self.map(),
            self.player,
            FieldOfViewRules {
                radius: self.vision,
                ..Default::default()
            },
        )
    }
    fn origin(&self) -> GridPos {
        if matches!(shape(self.skill(), self.rank()), Area::Line { .. }) {
            return GridPos::new(self.player.x + self.dir().0, self.player.y + self.dir().1);
        }
        if matches!(shape(self.skill(), self.rank()), Area::Cone { .. })
            || range(self.skill(), self.rank()) == 0
        {
            self.player
        } else {
            self.target
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        affected_cells(
            &self.map(),
            self.origin(),
            shape(self.skill(), self.rank()),
            self.dir(),
        )
    }
    fn valid(&self) -> bool {
        let map = self.map();
        let origin = self.origin();
        map.is_walkable(origin)
            && (self.skill()["parameters"]["movement_cells"]["selection"] != "up_to_maximum"
                || origin != self.player)
            && in_cast_range(self.player, origin, range(self.skill(), self.rank()))
            && self.visible().contains(&origin)
            && has_line_of_sight(&map, self.player, origin, true)
    }
    fn record(&self) -> Value {
        let area = shape(self.skill(), self.rank());
        let retention = if self.skill()["centred_damage"].is_object() {
            self.cells()
                .into_iter()
                .filter_map(|p| {
                    project_rl::skills::area_damage::damage_at(100, self.origin(), p, area)
                        .map(|value| json!({"cell":[p.x,p.y],"percent":value}))
                })
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };
        json!({"id":self.skill()["id"],"rank":self.rank(),"case":self.case,"player":[self.player.x,self.player.y],
            "target":[self.target.x,self.target.y],"origin":[self.origin().x,self.origin().y],"range":range(self.skill(),self.rank()),
            "vision":self.vision,"direction":self.dir(),"shape":area_json(shape(self.skill(),self.rank())),
            "valid":self.valid(),"affected":coords(&self.cells()),"visible":coords(&self.visible().into_iter().collect::<Vec<_>>()),"direct_damage_retention":retention})
    }
}
fn tile_rect(p: GridPos, viewport: Rect, camera: Vec2) -> Rect {
    Rect::new(
        viewport.x + (p.x as f32 - camera.x) * TILE,
        viewport.y + (p.y as f32 - camera.y) * TILE,
        TILE,
        TILE,
    )
}
fn draw_scene(s: &State, viewport: Rect, clock: f32) {
    let map = s.map();
    let visible = s.visible();
    let cells = s.cells();
    let valid = s.valid();
    draw_rectangle(viewport.x, viewport.y, viewport.w, viewport.h, INK);
    for y in 0..ROWS {
        for x in 0..COLS {
            let at = GridPos::new(x as i32, y as i32);
            let r = tile_rect(at, viewport, s.camera);
            if r.x < viewport.x
                || r.y < viewport.y
                || r.right() > viewport.right()
                || r.bottom() > viewport.bottom()
                || !visible.contains(&at)
            {
                continue;
            }
            game_art::draw_world(
                r,
                Visual::Terrain {
                    kind: if map.blocks_vision(at) {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
                    position: GridPos::new(at.x % 4, at.y % 4),
                },
                1.0,
            );
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 0.7, Color::new(0.15, 0.18, 0.19, 0.75));
            if !s.confirmed && cells.contains(&at) && visible.contains(&s.origin()) {
                let alpha = 0.16 + 0.13 * (clock * 6.0).sin().abs();
                let color = if valid {
                    Color::new(0.04, 0.74, 0.94, alpha)
                } else {
                    Color::new(0.96, 0.26, 0.12, alpha)
                };
                draw_rectangle(r.x + 2.0, r.y + 2.0, 60.0, 60.0, color);
                draw_rectangle_lines(
                    r.x + 2.0,
                    r.y + 2.0,
                    60.0,
                    60.0,
                    2.0,
                    Color::new(color.r, color.g, color.b, 0.85),
                );
                if s.skill()["centred_damage"].is_object()
                    && let Some(percent) = project_rl::skills::area_damage::damage_at(
                        100,
                        s.origin(),
                        at,
                        shape(s.skill(), s.rank()),
                    )
                {
                    draw_text_bold(format!("{percent}%"), r.x + 16.0, r.y + 57.0, 14.0, TEXT);
                }
            }
        }
    }
    for (at, visual) in [
        (s.player, super::melee::player_visual()),
        (GridPos::new(17, 12), Visual::Actor(5)),
        (GridPos::new(20, 10), Visual::Actor(5)),
        (GridPos::new(23, 12), Visual::Actor(5)),
    ] {
        let r = tile_rect(at, viewport, s.camera);
        if visible.contains(&at)
            && map.is_walkable(at)
            && r.x >= viewport.x
            && r.y >= viewport.y
            && r.right() <= viewport.right()
            && r.bottom() <= viewport.bottom()
        {
            game_art::draw_world(r, visual, 1.0);
        }
    }
    let r = tile_rect(s.origin(), viewport, s.camera);
    if r.x >= viewport.x
        && r.y >= viewport.y
        && r.right() <= viewport.right()
        && r.bottom() <= viewport.bottom()
    {
        draw_rectangle_lines(
            r.x + 5.0,
            r.y + 5.0,
            54.0,
            54.0,
            3.0,
            if valid { YELLOW } else { RED },
        );
    }
}
fn hud(s: &State, x: f32, y: f32) {
    draw_text_bold("Portées, zones et vision", x, y, 22.0, TEXT);
    draw_text(
        s.skill()["title"].as_str().unwrap(),
        x,
        y + 34.0,
        20.0,
        TEXT,
    );
    for (i, line) in [
        format!("Rang appris {} + bonus {}", s.learned, s.bonus),
        format!("Portée : {} cases", range(s.skill(), s.rank())),
        format!("Zone : {:?}", shape(s.skill(), s.rank())),
        format!("Vision : {} cases", s.vision),
        CASES[s.case].into(),
        if s.valid() {
            "Cible accessible".into()
        } else {
            "Cible hors portée ou masquée".into()
        },
        "Tab : compétence suivante".into(),
        "Page haut/bas : rang appris".into(),
        "B : bonus d'équipement".into(),
        "V : vision 8 / 12".into(),
        "M : décor · R : direction".into(),
        "Clic : placer · Entrée : tester".into(),
        "ZQSD : déplacer le joueur".into(),
        "Flèches : déplacer la caméra".into(),
        "Vision 12 : validée".into(),
        "Test d'emprise, sans dégâts préparés".into(),
    ]
    .iter()
    .enumerate()
    {
        draw_text(
            line,
            x,
            y + 70.0 + i as f32 * 28.0,
            17.0,
            if i < 6 { TEXT } else { DIM },
        );
    }
}
pub(crate) async fn run() {
    game_art::prewarm();
    let mut s = State::new();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            s.skill = (s.skill + 1) % reviewed().len();
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::PageUp) {
            s.learned = (s.learned + 1).min(10);
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::PageDown) {
            s.learned = s.learned.saturating_sub(1).max(1);
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::B) {
            s.bonus = (s.bonus + 1) % 11;
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::V) {
            s.vision = if s.vision == 8 { CANDIDATE_VISION } else { 8 };
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::M) {
            s.case = (s.case + 1) % 4;
            s.confirmed = false;
        }
        if is_key_pressed(KeyCode::R) {
            s.direction = (s.direction + 1) % 4;
            s.confirmed = false;
        }
        for (key, dx, dy) in [
            (KeyCode::Right, 1.0, 0.0),
            (KeyCode::Left, -1.0, 0.0),
            (KeyCode::Down, 0.0, 1.0),
            (KeyCode::Up, 0.0, -1.0),
        ] {
            if is_key_pressed(key) {
                s.camera += vec2(dx, dy);
            }
        }
        for (key, dx, dy) in [
            (KeyCode::D, 1, 0),
            (KeyCode::Q, -1, 0),
            (KeyCode::S, 0, 1),
            (KeyCode::Z, 0, -1),
        ] {
            if is_key_pressed(key) {
                let next = GridPos::new(s.player.x + dx, s.player.y + dy);
                if s.map().is_walkable(next) {
                    s.player = next;
                    s.confirmed = false;
                }
            }
        }
        clear_background(INK);
        let viewport = Rect::new(
            16.0,
            76.0,
            ((screen_width() - 432.0) / 64.0).floor().max(1.0) * 64.0,
            ((screen_height() - 96.0) / 64.0).floor().max(1.0) * 64.0,
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if viewport.contains(vec2(mx, my)) {
                s.target = GridPos::new(
                    ((mx - viewport.x) / 64.0 + s.camera.x) as i32,
                    ((my - viewport.y) / 64.0 + s.camera.y) as i32,
                );
                s.confirmed = false;
            }
        }
        if is_key_pressed(KeyCode::Enter) && s.valid() {
            s.confirmed = true;
        }
        draw_text_bold(
            "ZONE-GROWTH02 · laboratoire en cases natives 64 px",
            16.0,
            38.0,
            22.0,
            TEXT,
        );
        draw_scene(&s, viewport, get_time() as f32);
        hud(&s, viewport.right() + 22.0, 90.0);
        next_frame().await;
    }
}
fn readback(target: &RenderTarget, path: &Path) -> Result<(), String> {
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    let im = target.texture.get_texture_data();
    let stride = usize::from(im.width) * 4;
    let bytes = im
        .bytes
        .chunks_exact(stride)
        .rev()
        .flatten()
        .copied()
        .collect::<Vec<_>>();
    std::fs::write(path, bytes).map_err(|e| e.to_string())
}

#[cfg(test)]
mod movement_selection_tests {
    use super::*;

    #[test]
    fn affliction_area_growth_matches_the_catalogue_and_contagion_stays_capped() {
        let all = reviewed();
        for id in ["O02", "AF02", "AF03", "AF04", "AF05"] {
            let skill = all.iter().find(|s| s["id"] == id).unwrap();
            for (rank, width) in [(1, 3), (4, 5), (7, 7), (10, 9), (13, 11)] {
                assert_eq!(val(skill, "zone_width_cells", rank), width);
                assert!(
                    matches!(shape(skill, rank), Area::Square { radius } if radius == width / 2)
                );
            }
        }
        let skill = all.iter().find(|s| s["id"] == "AF06").unwrap();
        for (rank, width) in [(1, 5), (6, 7), (11, 9), (30, 9)] {
            assert_eq!(val(skill, "zone_width_cells", rank), width);
            assert!(matches!(shape(skill, rank), Area::Square { radius } if radius == width / 2));
        }
    }

    #[test]
    fn spectral_step_accepts_shorter_destinations_but_not_out_of_range_or_through_walls() {
        let mut s = State::new();
        s.skill = reviewed().iter().position(|s| s["id"] == "T02").unwrap();
        for (rank, maximum) in [(1, 2), (4, 4), (7, 6), (10, 8), (13, 10)] {
            s.learned = rank.min(10);
            s.bonus = rank.saturating_sub(10);
            assert_eq!(range(s.skill(), s.rank()), maximum);
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                for distance in 1..=i32::from(maximum) {
                    s.target = GridPos::new(12 + dx * distance, 12 + dy * distance);
                    assert!(s.valid(), "rank {rank}, selected distance {distance}");
                }
                let beyond = i32::from(maximum) + 1;
                s.target = GridPos::new(12 + dx * beyond, 12 + dy * beyond);
                assert!(!s.valid());
            }
            s.target = s.player;
            assert!(!s.valid());
        }
        s.learned = 10;
        s.bonus = 0;
        s.case = 2;
        s.target = GridPos::new(17, 12);
        assert!(s.valid());
        s.target = GridPos::new(18, 12);
        assert!(!s.valid());
        s.target = GridPos::new(19, 12);
        assert!(!s.valid());
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de capture existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    game_art::prewarm();
    let skills = reviewed();
    let mut fixtures = Vec::new();
    let mut s = State::new();
    for case in 0..4 {
        for i in 0..skills.len() {
            for rank in [1, 4, 7, 10, 13, 20] {
                s.case = case;
                s.skill = i;
                s.learned = rank.min(skills[i]["learned_cap"].as_u64().unwrap() as u16);
                s.bonus = rank.saturating_sub(s.learned);
                s.target = if case == 0 {
                    GridPos::new(20, 12)
                } else {
                    GridPos::new(16, 12)
                };
                fixtures.push(s.record());
            }
        }
    }
    let mut sprites = Vec::new();
    let small = render_target(64, 64);
    for y in 0..4 {
        for x in 0..4 {
            let name = format!("floor-{x}-{y}");
            set_camera(&Camera2D {
                render_target: Some(small.clone()),
                ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 64.0, 64.0))
            });
            clear_background(BLANK);
            game_art::draw_world(
                Rect::new(0.0, 0.0, 64.0, 64.0),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Deck,
                    position: GridPos::new(x, y),
                },
                1.0,
            );
            readback(&small, &output.join(format!("{name}.rgba")))?;
            sprites.push(json!({"name":name,"width":64,"height":64}));
        }
    }
    for (name, visual) in [
        (
            "wall",
            Visual::Terrain {
                kind: crate::test_sector::Decor::Wall,
                position: GridPos::new(0, 0),
            },
        ),
        ("player", super::melee::player_visual()),
        ("enemy", Visual::Actor(5)),
    ] {
        set_camera(&Camera2D {
            render_target: Some(small.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 64.0, 64.0))
        });
        clear_background(BLANK);
        game_art::draw_world(Rect::new(0.0, 0.0, 64.0, 64.0), visual, 1.0);
        readback(&small, &output.join(format!("{name}.rgba")))?;
        sprites.push(json!({"name":name,"width":64,"height":64}));
    }
    let target = render_target(2464, 1744);
    let mut captures = Vec::new();
    for (index, (case, vision, rank, id)) in [
        (0, 8, 13, "A02"),
        (0, 12, 13, "A02"),
        (1, 12, 13, "A02"),
        (2, 12, 13, "A02"),
        (3, 12, 13, "A02"),
        (0, 12, 1, "A02"),
        (0, 12, 13, "ME05"),
        (0, 12, 13, "HY04"),
    ]
    .into_iter()
    .enumerate()
    {
        s = State::new();
        s.case = case;
        s.vision = vision;
        s.learned = rank.min(10);
        s.bonus = rank.saturating_sub(10);
        s.skill = skills.iter().position(|s| s["id"] == id).unwrap();
        s.target = if case == 0 {
            GridPos::new(20, 12)
        } else {
            GridPos::new(16, 12)
        };
        s.camera = Vec2::ZERO;
        set_camera(&Camera2D {
            render_target: Some(target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 2464.0, 1744.0))
        });
        clear_background(INK);
        draw_text_bold(
            "Portées, zones et vision · revue native 64 px",
            32.0,
            38.0,
            26.0,
            TEXT,
        );
        draw_scene(
            &s,
            Rect::new(16.0, 80.0, COLS as f32 * 64.0, ROWS as f32 * 64.0),
            0.25,
        );
        hud(&s, 2024.0, 96.0);
        let name = format!("native-{index:02}");
        readback(&target, &output.join(format!("{name}.rgba")))?;
        captures.push(json!({"name":name,"width":2464,"height":1744,"state":s.record()}));
    }
    set_default_camera();
    std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&json!({"tile":64,"vision_candidate":12,"maps":(0..4).map(map_source).collect::<Vec<_>>(),"fixtures":fixtures,"captures":captures,"sprites":sprites})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
