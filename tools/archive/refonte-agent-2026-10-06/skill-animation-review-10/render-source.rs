//! ANIM-OVER01: complete effect, covered only by solid walls and frontal actors.
//! The bright blinking footprint alone defines the unchanged affected targets.
use super::*;
const DIRECTIONS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const LABELS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const CASES: [&str; 3] = ["COULOIR", "CLOISON", "OBSTACLE"];
const PANELS: [[f32; 2]; 4] = [[24.0, 48.0], [512.0, 48.0], [24.0, 536.0], [512.0, 536.0]];

fn variant(case: usize, direction: usize) -> String {
    format!("FE02-A06-{}-{}", CASES[case], DIRECTIONS[direction])
}
fn position(at: GridPos, direction: usize) -> GridPos {
    match direction {
        1 => GridPos::new(6 - at.x, 6 - at.y),
        2 => GridPos::new(at.y, 6 - at.x),
        3 => GridPos::new(6 - at.y, at.x),
        _ => at,
    }
}
fn point(at: Vec2, direction: usize) -> Vec2 {
    match direction {
        1 => vec2(448.0 - at.x, 448.0 - at.y),
        2 => vec2(at.y, 448.0 - at.x),
        3 => vec2(448.0 - at.y, at.x),
        _ => at,
    }
}
fn blink(time: f32) -> f32 {
    // Alternating brightness at one cycle/second; the contour never disappears.
    if (time.max(0.0) * 2.0) as u32 % 2 == 0 {
        1.0
    } else {
        0.42
    }
}
fn terrain(fixture: &Fixture, direction: usize, rect: Rect) {
    for y in 0..7 {
        for x in 0..7 {
            let source = GridPos::new(x, y);
            let at = position(source, direction);
            let p = cell(rect, at.x as f32, at.y as f32);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: if fixture.map.blocks_vision(source) {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
                    position: source,
                },
                1.0,
            );
            draw_rectangle_lines(
                p.x,
                p.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
}
fn cover_walls(fixture: &Fixture, direction: usize, rect: Rect) {
    for y in 0..7 {
        for x in 0..7 {
            let source = GridPos::new(x, y);
            if !fixture.map.blocks_vision(source) {
                continue;
            }
            let at = position(source, direction);
            let p = cell(rect, at.x as f32, at.y as f32);
            // The solid cell is physical terrain, not the gameplay effect footprint.
            // Repaint it above the effect; free cells receive no clipping at all.
            draw_rectangle(p.x, p.y, TILE, TILE, INK);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Wall,
                    position: source,
                },
                1.0,
            );
            draw_rectangle_lines(
                p.x,
                p.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
}
fn torrent(fire: &DrawnFire, direction: usize, rect: Rect, age: f32) {
    let frame = if age < 0.083 {
        0
    } else {
        1 + (age * 12.0) as u32 % 7
    };
    // Rotate a complete 384px drawing around its centre, including its emitter.
    let center = point(vec2(282.0, 219.0), direction) + vec2(rect.x, rect.y);
    let rotation = match direction {
        1 => std::f32::consts::PI,
        2 => -std::f32::consts::FRAC_PI_2,
        3 => std::f32::consts::FRAC_PI_2,
        _ => 0.0,
    };
    draw_texture_ex(
        &fire.cone,
        center.x - 192.0,
        center.y - 192.0,
        alpha(WHITE, 0.95),
        DrawTextureParams {
            source: Some(Rect::new(frame as f32 * 384.0, 0.0, 384.0, 384.0)),
            dest_size: Some(vec2(384.0, 384.0)),
            rotation,
            ..Default::default()
        },
    );
}
fn footprint(
    fixture: &Fixture,
    direction: usize,
    rect: Rect,
    time: f32,
    fill: bool,
    casting: bool,
) {
    let strength = blink(time);
    for &source in &fixture.affected {
        let at = position(source, direction);
        let p = cell(rect, at.x as f32, at.y as f32);
        if fill {
            draw_rectangle(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                Color::new(
                    0.03,
                    0.80,
                    0.98,
                    if casting {
                        0.035 + 0.055 * strength
                    } else {
                        0.12 + 0.20 * strength
                    },
                ),
            );
        } else {
            draw_rectangle_lines(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                7.0,
                Color::new(0.01, 0.025, 0.04, 0.96),
            );
            draw_rectangle_lines(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                4.0,
                Color::new(0.10, 0.95, 1.0, strength),
            );
            draw_rectangle_lines(
                p.x + 4.0,
                p.y + 4.0,
                56.0,
                56.0,
                1.0,
                Color::new(0.78, 1.0, 1.0, strength),
            );
        }
    }
}
fn scene(
    fire: &DrawnFire,
    fixture: &Fixture,
    case: usize,
    direction: usize,
    rect: Rect,
    clock: f32,
    cast_age: Option<f32>,
    show_effect: bool,
) {
    terrain(fixture, direction, rect);
    if let Some(age) = cast_age {
        if show_effect {
            torrent(fire, direction, rect, age);
        }
    }
    footprint(fixture, direction, rect, clock, true, cast_age.is_some());
    // Both the wide spell and the approved small burn flames are behind bodies.
    if let Some(age) = cast_age {
        if show_effect {
            for (index, &source) in fixture.targets.iter().enumerate() {
                if fixture.affected.contains(&source) {
                    let at = position(source, direction);
                    fire.burn(
                        cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5),
                        age,
                        index as u32 * 2,
                        0.9,
                    );
                }
            }
        }
    }
    cover_walls(fixture, direction, rect);
    for &source in &fixture.targets {
        let at = position(source, direction);
        enemy(cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5));
    }
    let origin = position(ORIGIN, direction);
    let player = cell(rect, origin.x as f32 + 0.5, origin.y as f32 + 0.5);
    actor(player, caster());
    // Targeting stays above the visual effect; actor textures are never tinted.
    footprint(fixture, direction, rect, clock, false, cast_age.is_some());
    for (index, &source) in fixture.targets.iter().enumerate() {
        let at = position(source, direction);
        let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
        let letter = char::from(b'A' + index as u8).to_string();
        draw_text_bold(&letter, p.x - 3.0, p.y - 27.0, 13.0, INK);
        draw_text_bold(letter, p.x - 4.0, p.y - 28.0, 13.0, TEXT);
    }
    if let Some(age) = cast_age {
        name(0, player, age);
    }
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 28.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(
        if cast_age.is_none() {
            "Cases touchées · aperçu clignotant"
        } else {
            SCENARIOS[case].2
        },
        rect.x + 14.0,
        rect.y + rect.h - 12.0,
        14.0,
        TEXT,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1060.0, 670.0);
    let fire = DrawnFire::load();
    let fixtures: Vec<_> = (0..3).map(Fixture::new).collect();
    let mut case = 0;
    let mut direction = 0;
    let mut clock = 0.0;
    let mut cast_start = None;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / SOUFFLE SUPERPOSÉ", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-OVER01 · quatre directions · personnages devant les flammes",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (index, (_, title, _)) in SCENARIOS.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + index as f32 * 46.0, 270.0, 35.0),
                title,
                true,
            ) {
                case = index;
                cast_start = None;
            }
        }
        for (index, label) in LABELS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (index % 2) as f32 * 138.0,
                    252.0 + (index / 2) as f32 * 42.0,
                    132.0,
                    34.0,
                ),
                label,
                true,
            ) {
                direction = index;
                cast_start = None;
            }
        }
        scene(
            &fire,
            &fixtures[case],
            case,
            direction,
            Rect::new(320.0, 96.0, 448.0, 448.0),
            clock,
            cast_start.map(|start| clock - start),
            true,
        );
        draw_text(variant(case, direction), 320.0, 567.0, 14.0, DIM);
        if button(Rect::new(20.0, 596.0, 115.0, 34.0), "Lancer", true)
            || is_key_pressed(KeyCode::Enter)
        {
            cast_start = Some(clock);
            paused = false;
        }
        if button(Rect::new(145.0, 596.0, 115.0, 34.0), "Viser", true) {
            cast_start = None;
        }
        if button(
            Rect::new(270.0, 596.0, 115.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(395.0, 596.0, 135.0, 34.0),
            if speed == 1.0 {
                "Vitesse ×1"
            } else {
                "Vitesse ×0,5"
            },
            true,
        ) {
            speed = if speed == 1.0 { 0.5 } else { 1.0 };
        }
        draw_text(
            "Les murs masquent leurs surfaces · les contours indiquent les cases touchées",
            22.0,
            655.0,
            14.0,
            DIM,
        );
        if !paused {
            clock += get_frame_time() * speed;
        }
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    request_new_screen_size(1024.0, 1024.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let fire = DrawnFire::load();
    let mut entries = Vec::new();
    for case in 0..3 {
        let fixture = Fixture::new(case);
        let folder = output.join(CASES[case]);
        std::fs::create_dir(&folder).map_err(|error| error.to_string())?;
        for frame in 0..90 {
            clear_background(INK);
            let clock = frame as f32 / 30.0;
            let cast_age = (clock >= 0.7).then_some(clock - 0.7);
            for direction in 0..4 {
                let [x, y] = PANELS[direction];
                draw_text_bold(LABELS[direction], x, y - 12.0, 18.0, TEXT);
                scene(
                    &fire,
                    &fixture,
                    case,
                    direction,
                    Rect::new(x, y, 448.0, 448.0),
                    clock,
                    cast_age,
                    true,
                );
            }
            crate::ui_capture::framebuffer()?
                .export_png(folder.join(format!("{frame:03}.png")).to_str().unwrap());
            if frame == 30 {
                clear_background(INK);
                for direction in 0..4 {
                    let [x, y] = PANELS[direction];
                    draw_text_bold(LABELS[direction], x, y - 12.0, 18.0, TEXT);
                    scene(
                        &fire,
                        &fixture,
                        case,
                        direction,
                        Rect::new(x, y, 448.0, 448.0),
                        clock,
                        cast_age,
                        false,
                    );
                }
                crate::ui_capture::framebuffer()?
                    .export_png(folder.join("control.png").to_str().unwrap());
            }
            next_frame().await;
        }
        for direction in 0..4 {
            let mut rows = vec![vec!['.'; 7]; 7];
            for y in 0..7 {
                for x in 0..7 {
                    let source = GridPos::new(x, y);
                    let at = position(source, direction);
                    if fixture.map.blocks_vision(source) {
                        rows[at.y as usize][at.x as usize] = '#';
                    }
                }
            }
            let coords = |at: GridPos| {
                let at = position(at, direction);
                [at.x, at.y]
            };
            entries.push(serde_json::json!({"id":"FE02","number":10,"variant":variant(case,direction),
                "folder":CASES[case],"title":SCENARIOS[case].1,"direction":DIRECTIONS[direction],
                "tile":64,"frames":90,"fps":30,"framebuffer_size":[1024,1024],
                "crop":[PANELS[direction][0] as u32,PANELS[direction][1] as u32,448,448],
                "map":rows.into_iter().map(|r| r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),
                "origin":coords(ORIGIN),"affected_cells":fixture.affected.iter().copied().map(coords).collect::<Vec<_>>(),
                "targets":fixture.targets.iter().enumerate().map(|(i,&at)| serde_json::json!({
                    "label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"affected":fixture.affected.contains(&at)})).collect::<Vec<_>>(),
                "control_frame":30,"blink_period_seconds":1.0,"lowest_preview_opacity":0.42,
                "layers":"terrain; complete flame overlay; targeting fill; burn flames; solid wall cover; frontal actors; targeting contours; labels"}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cloison_protects_targets_in_all_four_aiming_directions() {
        let fixture = Fixture::new(1);
        for (direction, protected) in [
            (0, GridPos::new(5, 3)),
            (1, GridPos::new(1, 3)),
            (2, GridPos::new(3, 1)),
            (3, GridPos::new(3, 5)),
        ] {
            let affected: Vec<_> = fixture
                .affected
                .iter()
                .copied()
                .map(|p| position(p, direction))
                .collect();
            assert!(affected.contains(&GridPos::new(3, 3)));
            assert!(!affected.contains(&protected));
            assert_eq!(position(fixture.targets[1], direction), protected);
        }
    }
}
