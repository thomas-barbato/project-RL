//! Isolated CITY-NORTH01 candidate. Does not read or write player saves.
//! cargo run --locked --example north_city_review
//! cargo run --locked --example north_city_review -- --capture artifacts/north-city-01
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
#[path = "surface_preview/assets.rs"]
mod assets;
#[path = "surface_preview/catalog.rs"]
mod catalog;
#[path = "north_city/decor.rs"]
mod decor;
#[path = "north_city/model.rs"]
mod model;
#[path = "surface_preview/paint.rs"]
mod paint;
#[path = "surface_preview/scene.rs"]
mod scene;
#[path = "north_city/street_skin.rs"]
mod street_skin;
#[path = "north_city/volumes.rs"]
mod volumes;

use macroquad::prelude::*;
use model::{Review, pos};
use project_rl::{
    game::{GameCommand, WorldState},
    world::{Direction, DoorState, GridPos, Terrain},
};
use std::{collections::BTreeSet, path::Path, time::Instant};

const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const STREET_ART: &str = "assets/maps/prison-street-expanded-01/street-cour-r03.png";
const REFERENCE: &str =
    "assets/source-art/first-sector-review-01/u01-style-r07/references/street-native-lighting.png";
const HOME_REFERENCE: &str =
    "assets/source-art/first-sector-review-01/u01-style-r07/references/home-native-lighting.png";
const CELL: f32 = 64.;

fn config() -> Conf {
    Conf {
        window_title: "Project RL - CITY-NORTH01 - MAP07 en revue".into(),
        window_width: 1280,
        window_height: 864,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}
fn label(text: &str, x: f32, y: f32, size: f32, color: Color) {
    draw_text(text, x, y, size, color);
}
fn texture(path: &str, at: Vec2, size: Option<Vec2>, color: Color, source: Option<Rect>) {
    asset_preparation::with_texture(path, |p| {
        draw_texture_ex(
            &p.texture,
            at.x,
            at.y,
            color,
            DrawTextureParams {
                dest_size: size,
                source,
                ..Default::default()
            },
        );
    });
}
async fn prepare(paths: &BTreeSet<String>) -> Result<f64, String> {
    let start = Instant::now();
    loop {
        let mut complete = true;
        for path in paths {
            if !asset_preparation::ready(path)? {
                complete = false;
                let path = path.clone();
                let key = path.clone();
                let build = decor::builder(&path)
                    .or_else(|| street_skin::builder(&path))
                    .unwrap_or_else(|| {
                        Box::new(move || {
                            let image = asset_preparation::image(&path)?;
                            Ok(asset_preparation::PreparedImage {
                                image: (*image).clone(),
                                cell: [64, 64],
                                anchor: [32., 32.],
                            })
                        })
                    });
                asset_preparation::request(key, build);
            }
        }
        asset_preparation::pump();
        if complete {
            return Ok(start.elapsed().as_secs_f64() * 1000.);
        }
        if start.elapsed().as_secs() > 30 {
            return Err("Préparation du quartier trop longue.".into());
        }
        clear_background(Color::from_hex(0x10151c));
        label("Préparation du quartier…", 50., 80., 26., WHITE);
        next_frame().await;
    }
}
fn paths(review: &Review, world: &WorldState) -> BTreeSet<String> {
    let mut paths = BTreeSet::from([PLAYER.into(), HOME_REFERENCE.into()]);
    paths.extend(street_skin::keys().map(str::to_owned));
    if let Some(index) = model::zone_index(review, world) {
        paths.extend(decor::actor_keys(&review.zones[index], world));
        paths.extend(decor::keys(
            &review.zones[index],
            world.player_position().unwrap(),
        ));
    } else {
        paths.insert(STREET_ART.into());
    }
    paths
}

/// Native material fragments keep the outdoor ground continuous, without
/// decorative tile borders. Interiors retain their own editor materials.
fn ground(material: usize, at: GridPos, origin: Vec2, cell: f32) {
    ground_patch(material, at, origin, cell, Rect::new(0., 0., 64., 64.));
}
fn ground_patch(material: usize, at: GridPos, origin: Vec2, cell: f32, patch: Rect) {
    street_skin::ground(material, at, origin, cell, patch);
}

fn draw_world(
    review: &Review,
    scenes: &[scene::Scene],
    world: &WorldState,
    bounds: Rect,
    center: Vec2,
    cell: f32,
    roofs: bool,
) {
    let origin = (bounds.point() + bounds.size() / 2. - center * cell).floor();
    let Some(index) = model::zone_index(review, world) else {
        texture(STREET_ART, origin, Some(vec2(26., 66.) * cell), WHITE, None);
        if let Some(p) = world.player_position() {
            texture(
                PLAYER,
                origin + vec2(p.x as f32, p.y as f32) * cell,
                Some(vec2(cell, cell)),
                WHITE,
                None,
            );
        }
        return;
    };
    let zone = &review.zones[index];
    let scene = &scenes[index];
    let hidden = zone
        .buildings
        .iter()
        .filter(|b| roofs && !b.open(world))
        .collect::<Vec<_>>();
    let under_roof = |at: GridPos| hidden.iter().any(|b| b.contains(at));
    let min_x = ((bounds.x - origin.x) / cell).floor().max(0.) as i32;
    let min_y = ((bounds.y - origin.y) / cell).floor().max(0.) as i32;
    let max_x = (((bounds.right() - origin.x) / cell).ceil() as i32).min(scene.document.width);
    let max_y = (((bounds.bottom() - origin.y) / cell).ceil() as i32).min(scene.document.height);
    let sealed = zone.sealed.iter().copied().collect::<BTreeSet<_>>();
    let material_at = |p: GridPos| {
        hidden.iter().find(|b| b.contains(p)).map_or(
            if sealed.contains(&[p.x, p.y]) {
                99
            } else {
                scene.document.floors[(p.y * scene.document.width + p.x) as usize].unwrap_or(18)
            },
            |_| 99,
        )
    };
    decor::draw(zone, origin, bounds, cell);
    for y in min_y..max_y {
        for x in min_x..max_x {
            let p = GridPos::new(x, y);
            let material = material_at(p);
            // Curbs describe the actual street outline, never a tile grid.
            if material == 17 && !scene.is_structure(p) {
                let curb = Color::new(0.45, 0.46, 0.44, 0.9);
                let thickness = (cell * 0.035).max(0.4);
                for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                    let q = GridPos::new(x + dx, y + dy);
                    if !scene.game.map().contains(q)
                        || scene.is_structure(q)
                        || material_at(q) != 18
                    {
                        continue;
                    }
                    let at = origin + vec2(x as f32, y as f32) * cell;
                    if dx != 0 {
                        draw_rectangle(
                            at.x + if dx > 0 { cell - thickness } else { 0. },
                            at.y,
                            thickness,
                            cell,
                            curb,
                        );
                    } else {
                        draw_rectangle(
                            at.x,
                            at.y + if dy > 0 { cell - thickness } else { 0. },
                            cell,
                            thickness,
                            curb,
                        );
                    }
                }
            }
        }
    }
    // Aftermath decals are baked with ground occlusion by the shared worker.
    for prop in &scene.props {
        if !under_roof(prop.pos)
            && !decor::foot_hidden(zone, prop.pos, prop.size().1)
            && (min_x..max_x).contains(&prop.pos.x)
            && (min_y..max_y).contains(&prop.pos.y)
        {
            let at = origin + vec2(prop.pos.x as f32, prop.pos.y as f32) * cell;
            street_skin::draw_prop(prop, at, cell);
        }
    }
    // Door visuals read persistent engine terrain, including on return visits.
    for part in &scene.document.structures {
        if !(min_x..max_x).contains(&part.pos.x)
            || !(min_y..max_y).contains(&part.pos.y)
            || under_roof(part.pos)
            || sealed.contains(&[part.pos.x, part.pos.y])
        {
            continue;
        }
        if part.door.is_none() {
            continue;
        }
        if let Some(Terrain::Door(door)) = world.map().tile(part.pos).map(|t| t.terrain) {
            // draw_structure consults its isolated scene state. Select the
            // actual engine door texture directly instead.
            street_skin::draw_door(
                origin + vec2(part.pos.x as f32, part.pos.y as f32) * cell,
                cell,
                part.rotation % 2 == 1,
                door == DoorState::Open,
            );
        }
    }
    // Short white markings follow the wide streets actually authored in the
    // candidate. Narrow European alleys have no invented central lane stripe.
    for mark in &zone.road_markings {
        if !(min_x..max_x).contains(&mark.at[0]) || !(min_y..max_y).contains(&mark.at[1]) {
            continue;
        }
        let at = origin + vec2(mark.at[0] as f32 + 0.5, mark.at[1] as f32 + 0.5) * cell;
        let size = if mark.horizontal {
            vec2(0.5, 0.045)
        } else {
            vec2(0.045, 0.5)
        } * cell;
        draw_rectangle(
            at.x - size.x / 2.,
            at.y - size.y / 2.,
            size.x,
            size.y,
            Color::new(0.65, 0.67, 0.64, 0.65),
        );
    }
    for marker in &scene.document.markers {
        if under_roof(marker.pos) {
            continue;
        }
        let p = origin + vec2(marker.pos.x as f32, marker.pos.y as f32) * cell;
        draw_rectangle_lines(
            p.x + 3.,
            p.y + 3.,
            cell - 6.,
            cell - 6.,
            (cell / 32.).max(1.),
            Color::new(0.73, 0.61, 0.30, 0.8),
        );
    }
    if roofs {
        decor::draw_roofs(zone, world, origin, bounds, cell);
    }
    if let Some(p) = world.player_position() {
        texture(
            &decor::actor_key(zone, world, roofs),
            origin + vec2(p.x as f32, p.y as f32) * cell,
            Some(vec2(cell, cell)),
            WHITE,
            None,
        );
    }
}

fn center(world: &WorldState) -> Vec2 {
    let p = world.player_position().unwrap();
    vec2(p.x as f32 + 0.5, p.y as f32 + 0.5)
}
async fn capture(
    review: &Review,
    scenes: &[scene::Scene],
    world: &mut WorldState,
    output: &Path,
    beds_only: bool,
) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let target = render_target(1024, 704);
    target.texture.set_filter(FilterMode::Nearest);
    let mut timings = vec![];
    model::enter_city(world)?;
    let first = prepare(&paths(review, world)).await?;
    for index in 0..review.zones.len() {
        let zone = &review.zones[index];
        let bed_views;
        let views = if beds_only {
            bed_views = [(72, "02-logement"), (75, "04-pompiers"), (74, "cellules")]
                .into_iter()
                .filter_map(|(sprite, name)| {
                    scenes[index]
                        .props
                        .iter()
                        .find(|prop| prop.furniture && prop.sprite == sprite)
                        .map(|prop| model::View {
                            id: if sprite == 74 { format!("{name}-{index}") } else { name.into() },
                            at: [prop.pos.x, prop.pos.y],
                        })
                })
                .collect::<Vec<_>>();
            &bed_views
        } else {
            &zone.views
        };
        if views.is_empty() {
            continue;
        }
        model::travel_to(review, world, &zone.id)?;
        prepare(&paths(review, world)).await?;
        for view in views {
            let anchor = if beds_only {
                let [x, y] = view.at;
                [[x + 1, y + 1], [x - 1, y + 1], [x, y + 2], [x + 1, y], [x - 1, y]]
                    .into_iter()
                    .map(pos)
                    .find(|&near| model::route(world, near).is_ok())
                    .ok_or_else(|| format!("No accessible view of bed {}", view.id))?
            } else {
                pos(view.at)
            };
            model::walk_to(world, anchor)?;
            prepare(&paths(review, world)).await?;
            set_camera(&Camera2D {
                render_target: Some(target.clone()),
                ..Camera2D::from_display_rect(Rect::new(0., 0., 1024., 704.))
            });
            clear_background(Color::from_hex(0x111820));
            let start = Instant::now();
            draw_world(
                review,
                scenes,
                world,
                Rect::new(0., 0., 1024., 704.),
                center(world),
                CELL,
                !beds_only,
            );
            set_default_camera();
            timings.push(serde_json::json!({"id":view.id,"render_cpu_ms":start.elapsed().as_secs_f64()*1000.,"cell_px":64}));
            next_frame().await;
            let path = output.join(format!("{}.png", view.id));
            target
                .texture
                .get_texture_data()
                .export_png(path.to_str().ok_or("capture path")?);
        }
    }
    if beds_only {
        return Ok(());
    }
    model::travel_to(review, world, model::CITY)?;
    // The overview is labelled separately; only native crops judge the art.
    let overview_cell = 8.;
    let overview_width = scenes[0].document.width as f32 * overview_cell;
    let overview_height = scenes[0].document.height as f32 * overview_cell;
    let overview = render_target(overview_width as u32, overview_height as u32);
    set_camera(&Camera2D {
        render_target: Some(overview.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., overview_width, overview_height))
    });
    clear_background(BLACK);
    draw_world(
        review,
        scenes,
        world,
        Rect::new(0., 0., overview_width, overview_height),
        vec2(
            scenes[0].document.width as f32 / 2.,
            scenes[0].document.height as f32 / 2.,
        ),
        overview_cell,
        false,
    );
    set_default_camera();
    next_frame().await;
    overview
        .texture
        .get_texture_data()
        .export_png(output.join("plan.png").to_str().ok_or("plan path")?);
    // The same cache serves the return trip; capture before/after counters.
    let return_anchor = pos(review.street_links[0].arrival);
    model::walk_to(world, return_anchor)?;
    model::apply(
        world,
        GameCommand::Interact {
            target: return_anchor,
        },
    )?;
    let street = prepare(&paths(review, world)).await?;
    model::enter_city(world)?;
    let returned = prepare(&paths(review, world)).await?;
    let snapshot = world.recovery_snapshot_bytes()?;
    let mut restored = WorldState::from_recovery_snapshot_bytes(
        &snapshot,
        project_rl::game::GameRules::default(),
    )?;
    model::restore_obstructions(&mut restored, review, scenes)?;
    if restored.map() != world.map() {
        return Err("snapshot map mismatch".into());
    }
    let variants = capture_variants(output).await?;
    capture_style_comparison(review, scenes, world, output).await?;
    let house_walk = capture_house_walk(review, scenes, output).await?;
    capture_closed_block(review, scenes, output).await?;
    let mut checks = serde_json::json!({"review":review.review,"native_cell":64,"scenes":timings,
        "zone_count":review.zones.len(),"street_anchors":[[18,1],[18,2]],"campaign_integrated":false,
        "seed":review.game_seed,"basement_choices":review.zones.iter().filter_map(|z|
            z.selected_variant.map(|v|serde_json::json!({"zone":z.id,"variant":v,"file":z.file}))).collect::<Vec<_>>(),
        "variant_previews":variants,"overview":{"width":overview_width,"height":overview_height,"cell_px":overview_cell},
        "visual_revision":"CITY-NORTH01-MAP07-r01","style_sources":[street_skin::MATERIAL_SOURCE,street_skin::FURNITURE_SOURCE,street_skin::SERVICE_SOURCE],
        "visual_comparison":"comparaison-style-r03.png","corner_review":"raccords-r03.png",
        "house_walk":house_walk,"closed_block":"map07-ilot.png","architecture":"First-city references; 96px standard volumes, 138px complete B01 drawing, grounded on actual blocked footprints","outline_source":volumes::SOURCE,"block_source":volumes::BLOCK_SOURCE,
        "saves_touched":false,"optional_dungeon_round_trips":true,"snapshot_restored":true,
        "loading":{"first_city_prepare_ms":first,"street_prepare_ms":street,"city_return_prepare_ms":returned},
        "limits":["Exploration fixture; no population or loot balance yet.",
                  "CPU draw measurements, not GPU/present timings.","MAP07 awaits visual prevalidation; ambient lighting is a review fixture."]});
    #[cfg(debug_assertions)]
    {
        checks["resource_cache"] = asset_preparation::stats();
    }
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&checks).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

async fn capture_closed_block(
    review: &Review,
    scenes: &[scene::Scene],
    output: &Path,
) -> Result<(), String> {
    let mut outside = model::world(review, scenes)?;
    model::enter_city(&mut outside)?;
    model::walk_to(&mut outside, GridPos::new(28, 104))?;
    let building = &review.zones[0].buildings[0];
    if building.open(&outside) {
        return Err("Exterior capture unexpectedly opened B01".into());
    }
    prepare(&paths(review, &outside)).await?;
    let target = render_target(1536, 1088);
    target.texture.set_filter(FilterMode::Nearest);
    set_camera(&Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 1536., 1088.))
    });
    clear_background(BLACK);
    draw_world(
        review,
        scenes,
        &outside,
        Rect::new(0., 0., 1536., 1088.),
        vec2(22., 97.75),
        CELL,
        true,
    );
    draw_rectangle(0., 0., 1536., 32., Color::from_hex(0x111820));
    label(
        "MAP07 - B01 ferme - personnage et decor a 64 px/case",
        20.,
        22.,
        20.,
        Color::from_hex(0xe1c593),
    );
    set_default_camera();
    next_frame().await;
    target
        .texture
        .get_texture_data()
        .export_png(output.join("map07-ilot.png").to_str().ok_or("block path")?);
    Ok(())
}

async fn capture_house_walk(
    review: &Review,
    scenes: &[scene::Scene],
    output: &Path,
) -> Result<serde_json::Value, String> {
    let mut world = model::world(review, scenes)?;
    model::enter_city(&mut world)?;
    model::walk_to(&mut world, pos([32, 97]))?;
    prepare(&paths(review, &world)).await?;
    let frame_dir = output.join("walk-frames");
    std::fs::create_dir_all(&frame_dir).map_err(|e| e.to_string())?;
    let target = render_target(1536, 960);
    target.texture.set_filter(FilterMode::Nearest);
    let mut frames = vec![];
    let mut steps = 0;
    let mut visited = vec![];
    let stops = [
        ("Entree", [28, 97]),
        ("Cuisine", [24, 95]),
        ("Chambre", [16, 95]),
        ("Sejour", [16, 100]),
        ("Salle d'eau", [24, 100]),
        ("Retour dehors", [32, 97]),
    ];
    let mut full_saved = false;
    for (title, destination) in stops {
        let directions = model::route(&world, pos(destination))?;
        let count = directions.len();
        for (index, direction) in directions.into_iter().enumerate() {
            let before = world.player_position().unwrap();
            model::step(&mut world, direction)?;
            if world.player_position() != Some(before.step(direction)) {
                return Err(
                    "House walkthrough did not advance by its actual engine command".into(),
                );
            }
            steps += 1;
            // Every frame follows one real adjacent engine movement command.
            prepare(&paths(review, &world)).await?;
            set_camera(&Camera2D {
                render_target: Some(target.clone()),
                ..Camera2D::from_display_rect(Rect::new(0., 0., 1536., 960.))
            });
            clear_background(Color::from_hex(0x111820));
            draw_world(
                review,
                scenes,
                &world,
                Rect::new(0., 48., 1536., 896.),
                vec2(22., 97.5),
                64.,
                false,
            );
            draw_rectangle(0., 0., 1536., 48., Color::from_hex(0x111820));
            label(
                &format!(
                    "MAP07 - B01 - {} - 64 px/case - deplacements reels du moteur",
                    title
                ),
                22.,
                31.,
                24.,
                Color::from_hex(0xdec69b),
            );
            set_default_camera();
            next_frame().await;
            let image = target.texture.get_texture_data();
            let frame = frames.len();
            image.export_png(
                frame_dir
                    .join(format!("frame-{frame:03}.png"))
                    .to_str()
                    .ok_or("walk frame path")?,
            );
            if title == "Cuisine" && index + 1 == count {
                image.export_png(
                    output
                        .join("decor04-logement.png")
                        .to_str()
                        .ok_or("house view path")?,
                );
                full_saved = true;
            }
            frames.push(serde_json::json!({"frame":frame,"title":title,"at":world.player_position().unwrap(),"pause":index+1==count}));
        }
        visited.push(serde_json::json!({"room":title,"at":destination}));
    }
    if !full_saved {
        return Err("Whole-house native view missing".into());
    }
    Ok(
        serde_json::json!({"native_image":"decor04-logement.png","animation":"decor04-parcours.webp","frame_directory":"walk-frames","width":1536,"height":960,"native_cell":64,"movement_commands":steps,"teleports":0,"visited":visited,"frames":frames,"final_position":world.player_position().unwrap(),"doors_opened_through_engine":true}),
    )
}

async fn capture_style_comparison(
    review: &Review,
    scenes: &[scene::Scene],
    world: &mut WorldState,
    output: &Path,
) -> Result<(), String> {
    let target = render_target(1024, 352);
    model::travel_to(review, world, model::CITY)?;
    let view = review.zones[0]
        .views
        .iter()
        .find(|v| v.id == "02-logement")
        .ok_or("housing style view missing")?;
    model::walk_to(world, pos(view.at))?;
    prepare(&paths(review, world)).await?;
    set_camera(&Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 1024., 352.))
    });
    clear_background(Color::from_hex(0x10171e));
    texture(HOME_REFERENCE, vec2(8., 32.), None, WHITE, None);
    draw_world(
        review,
        scenes,
        world,
        Rect::new(512., 32., 512., 320.),
        center(world),
        64.,
        true,
    );
    // A single render target is shared; the native housing view cannot paint
    // over the reference half or the labels.
    texture(HOME_REFERENCE, vec2(8., 32.), None, WHITE, None);
    draw_rectangle(0., 0., 1024., 32., Color::from_hex(0x10171e));
    draw_rectangle(488., 32., 24., 320., Color::from_hex(0x10171e));
    label(
        "Rue validee - 64 px/case",
        12.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    label(
        "STYLE03 - quartier nord - 64 px/case",
        522.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    set_default_camera();
    next_frame().await;
    target.texture.get_texture_data().export_png(
        output
            .join("comparaison-style-r03.png")
            .to_str()
            .ok_or("style capture path")?,
    );

    let board = render_target(1024, 360);
    set_camera(&Camera2D {
        render_target: Some(board.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 1024., 360.))
    });
    clear_background(Color::from_hex(0x10171e));
    label(
        "STYLE03 - quatre angles relies aux murs droits - taille native 64 px",
        18.,
        28.,
        22.,
        Color::from_hex(0xdec69b),
    );
    for (i, mask) in [6u8, 12, 3, 9].into_iter().enumerate() {
        let base = vec2(i as f32 * 252. + 10., 66.);
        for y in 0..4 {
            for x in 0..4 {
                street_skin::ground(
                    18,
                    GridPos::new(x, y),
                    base + vec2(x as f32, y as f32) * 64.,
                    64.,
                    Rect::new(0., 0., 64., 64.),
                );
            }
        }
        let at = base + vec2(64., 64.);
        let mut pieces = vec![(at, mask)];
        for (bit, delta, line) in [
            (1, vec2(0., -64.), 5),
            (2, vec2(64., 0.), 10),
            (4, vec2(0., 64.), 5),
            (8, vec2(-64., 0.), 10),
        ] {
            if mask & bit != 0 {
                pieces.push((at + delta, line));
            }
        }
        for top in [false, true] {
            for (p, m) in &pieces {
                street_skin::draw_wall(*m, 1, *p, 64., top);
            }
        }
        label(
            &format!("03-{}", i + 1),
            base.x + 12.,
            base.y - 12.,
            20.,
            WHITE,
        );
    }
    set_default_camera();
    next_frame().await;
    board.texture.get_texture_data().export_png(
        output
            .join("raccords-r03.png")
            .to_str()
            .ok_or("corner capture path")?,
    );
    Ok(())
}

async fn capture_variants(output: &Path) -> Result<Vec<serde_json::Value>, String> {
    let target = render_target(720, 560);
    target.texture.set_filter(FilterMode::Nearest);
    let mut previews = vec![];
    for (floor, prefix) in [
        (1, "review:police_basement_layout"),
        (2, "review:police_deep_layout"),
    ] {
        for variant in 1..=3 {
            let (seed, review, scenes) = (1..=128)
                .find_map(|seed| {
                    let (review, scenes) = model::load(seed).ok()?;
                    review
                        .zones
                        .iter()
                        .any(|z| z.id == format!("{prefix}{variant:02}"))
                        .then_some((seed, review, scenes))
                })
                .ok_or("variant seed missing")?;
            decor::register(&review, &scenes);
            let mut world = model::world(&review, &scenes)?;
            model::enter_city(&mut world)?;
            let destination = format!("{prefix}{variant:02}");
            model::travel_to(&review, &mut world, &destination)?;
            prepare(&paths(&review, &world)).await?;
            set_camera(&Camera2D {
                render_target: Some(target.clone()),
                ..Camera2D::from_display_rect(Rect::new(0., 0., 720., 560.))
            });
            clear_background(BLACK);
            draw_world(
                &review,
                &scenes,
                &world,
                Rect::new(0., 0., 720., 560.),
                vec2(18., 14.),
                20.,
                false,
            );
            set_default_camera();
            next_frame().await;
            let name = format!("sous-sol-{floor}-variante-{variant}.png");
            target
                .texture
                .get_texture_data()
                .export_png(output.join(&name).to_str().ok_or("variant path")?);
            previews.push(
                serde_json::json!({"floor":floor,"variant":variant,"seed":seed,"image":name}),
            );
        }
    }
    Ok(previews)
}

#[macroquad::main(config)]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("CITY-NORTH01: {error}");
        std::process::exit(1);
    }
}
async fn run() -> Result<(), String> {
    asset_preparation::initialize_graphics_thread();
    let args = std::env::args().collect::<Vec<_>>();
    let seed = if let Some(at) = args.iter().position(|a| a == "--seed") {
        args.get(at + 1)
            .ok_or("seed missing")?
            .parse::<u64>()
            .map_err(|_| "invalid seed")?
    } else {
        20261105
    };
    let (review, scenes) = model::load(seed)?;
    decor::register(&review, &scenes);
    let mut world = model::world(&review, &scenes)?;
    // Validate coverage before displaying anything: no generic-kit fallback.
    for scene in &scenes {
        for prop in &scene.props {
            if !prop.furniture || !street_skin::supports(prop.sprite) {
                return Err(format!("Missing STYLE03 artwork for prop {}", prop.sprite));
            }
        }
    }
    if let Some(at) = args.iter().position(|a| a == "--capture") {
        return capture(
            &review,
            &scenes,
            &mut world,
            Path::new(args.get(at + 1).ok_or("capture output missing")?),
            args.iter().any(|arg| arg == "--capture-lits"),
        )
        .await;
    }
    model::enter_city(&mut world)?;
    if args.iter().any(|a| a == "--logement") {
        model::walk_to(&mut world, pos([32, 97]))?;
    }
    prepare(&paths(&review, &world)).await?;
    let mut overview = false;
    let mut roofs = true;
    let mut message = String::from("MAP07 : quartier en reprise, style de la première ville.");
    let mut next_step = 0.;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Tab) {
            overview = !overview;
        }
        if is_key_pressed(KeyCode::R) {
            roofs = !roofs;
        }
        for (keys, direction) in [
            ([KeyCode::Up, KeyCode::W, KeyCode::Z], Direction::North),
            ([KeyCode::Right, KeyCode::D, KeyCode::D], Direction::East),
            ([KeyCode::Down, KeyCode::S, KeyCode::S], Direction::South),
            ([KeyCode::Left, KeyCode::A, KeyCode::Q], Direction::West),
        ] {
            if !overview && get_time() >= next_step && keys.iter().any(|&k| is_key_down(k)) {
                if let Err(error) = model::step(&mut world, direction) {
                    message = error;
                }
                next_step = get_time() + 0.11;
                break;
            }
        }
        if is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::Enter) {
            let p = world.player_position().unwrap();
            let passage = [
                p,
                GridPos::new(p.x, p.y - 1),
                GridPos::new(p.x + 1, p.y),
                GridPos::new(p.x, p.y + 1),
                GridPos::new(p.x - 1, p.y),
            ]
            .into_iter()
            .find(|&p| world.passage(p).is_some());
            if let Some(target) = passage {
                model::apply(&mut world, GameCommand::Interact { target })?;
            } else if model::zone_index(&review, &world) == Some(0)
                && (p.x - review.future_exit.at[0]).abs() + (p.y - review.future_exit.at[1]).abs()
                    <= 1
            {
                message = "Caserne militaire : sortie prévue ; intérieur à construire.".into();
            }
        }
        prepare(&paths(&review, &world)).await?;
        clear_background(Color::from_hex(0x111820));
        let bounds = Rect::new(0., 58., screen_width(), screen_height() - 134.);
        let c = if overview {
            vec2(
                world.map().width() as f32 / 2.,
                world.map().height() as f32 / 2.,
            )
        } else {
            center(&world)
        };
        let cell = if overview {
            (bounds.w / world.map().width() as f32).min(bounds.h / world.map().height() as f32)
        } else {
            CELL
        };
        draw_world(
            &review,
            &scenes,
            &world,
            bounds,
            c,
            cell,
            roofs && !overview,
        );
        // Click on an overview destination walks there through real engine
        // commands, then returns to native size. No teleport or direct edits.
        if overview && is_mouse_button_pressed(MouseButton::Left) {
            let origin = bounds.point() + bounds.size() / 2. - c * cell;
            let mouse = vec2(mouse_position().0, mouse_position().1);
            if bounds.contains(mouse) {
                let at = ((mouse - origin) / cell).floor();
                match model::walk_to(&mut world, GridPos::new(at.x as i32, at.y as i32)) {
                    Ok(()) => {
                        overview = false;
                        message.clear();
                    }
                    Err(error) => message = error,
                }
            }
        }
        draw_rectangle(0., 0., screen_width(), 58., Color::from_hex(0x131b24));
        draw_rectangle(
            0.,
            screen_height() - 76.,
            screen_width(),
            76.,
            Color::from_hex(0x131b24),
        );
        label(
            &format!("01 / {}", world.current_zone().unwrap().name),
            22.,
            37.,
            24.,
            Color::from_hex(0xdec69b),
        );
        let p = world.player_position().unwrap();
        label(
            &format!(
                "X {} ; Y {}  ·  {}",
                p.x,
                p.y,
                if overview { "PLAN" } else { "64 px / case" }
            ),
            screen_width() - 310.,
            35.,
            19.,
            LIGHTGRAY,
        );
        label(
            "Flèches / ZQSD : marcher · E : passage · Tab : plan + clic · R : toits · Échap : fermer",
            22.,
            screen_height() - 45.,
            19.,
            LIGHTGRAY,
        );
        label(
            &message,
            22.,
            screen_height() - 17.,
            18.,
            Color::from_hex(0xb3bec9),
        );
        next_frame().await;
    }
    Ok(())
}
