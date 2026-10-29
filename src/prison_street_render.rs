//! Authored street pixels at one source pixel per native world pixel.
use crate::{prison_street as street, terminal_view::GridCamera};
use macroquad::prelude::*;
use project_rl::{game::WorldState, world::GridPos};
use std::cell::{Cell, RefCell};

struct Art {
    height: usize,
    background_file: String,
    background: Texture2D,
    #[cfg(debug_assertions)]
    review_background: Option<Texture2D>,
    car: Texture2D,
    flame: Texture2D,
    atmosphere: Option<crate::camp_atmosphere::Atmosphere>,
    roofs: Vec<Texture2D>,
    props: Vec<Texture2D>,
}
fn load(file: &str) -> Texture2D {
    let bytes = project_rl::data_paths::read(file).expect("accepted street texture");
    let texture = crate::asset_loading::texture(&bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn patch(texture: &Texture2D, origin: Rect, source: [f32; 4], at: [f32; 2]) {
    let scale = origin.w / 64.;
    draw_texture_ex(
        texture,
        origin.x + at[0] * scale,
        origin.y + at[1] * scale,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(source[0], source[1], source[2], source[3])),
            dest_size: Some(vec2(source[2], source[3]) * scale),
            ..Default::default()
        },
    );
}
thread_local! {
    static ART: RefCell<Option<Art>> = const { RefCell::new(None) };
    static REVIEW: Cell<bool> = const { Cell::new(false) };
    static LIGHTS: Cell<bool> = const { Cell::new(true) };
    #[cfg(debug_assertions)]
    static REPAIRS_REVIEW: Cell<bool> = const { Cell::new(false) };
    #[cfg(debug_assertions)]
    static REVIEW_AMBIENT: Cell<Option<[f32;3]>> = const {Cell::new(None)};
    #[cfg(debug_assertions)]
    static REVIEW_LEGACY_DOORS: Cell<bool> = const {Cell::new(false)};
    #[cfg(debug_assertions)]
    static REVIEW_GRIM: Cell<Option<bool>> = const {Cell::new(None)};
    #[cfg(debug_assertions)]
    static REVIEW_WHOLE_STYLE: Cell<Option<bool>> = const {Cell::new(None)};
}
#[cfg(debug_assertions)]
pub fn review_repairs(value: bool) {
    REPAIRS_REVIEW.set(value);
    ART.with(|art| *art.borrow_mut() = None);
}
fn scene_for_render(game: &WorldState) -> &'static street::Street {
    #[cfg(debug_assertions)]
    if let Some(scene) = crate::sewer_review::scene(game) {
        return scene;
    }
    if let Some(scene) = crate::sewer::scene(game) {
        return scene;
    }
    #[cfg(debug_assertions)]
    if crate::street_defense_review::enabled()
        && game.map().height() == street::expanded_scene().height
    {
        return crate::street_defense_review::scene();
    }
    #[cfg(debug_assertions)]
    if REPAIRS_REVIEW.get() && game.map().height() == street::expanded_scene().height {
        static SCENE: std::sync::OnceLock<street::Street> = std::sync::OnceLock::new();
        return SCENE.get_or_init(|| {
            serde_json::from_str(include_str!(
                "../assets/source-art/first-sector-review-01/u01-repairs-r09/street.json"
            ))
            .expect("U01 repair review manifest")
        });
    }
    street::scene_for_game(game)
}
#[cfg(debug_assertions)]
pub fn review_whole_style(value: bool) {
    REVIEW_WHOLE_STYLE.set(Some(value));
}
#[cfg(debug_assertions)]
pub fn review_grim(value: bool) {
    REVIEW_GRIM.set(Some(value));
}

#[cfg(debug_assertions)]
pub fn review_normal_style() {
    REVIEW_GRIM.set(None);
    REVIEW_WHOLE_STYLE.set(None);
}

fn draw_ground_prop(origin: Rect, art: &Art, texture: &Texture2D, at_px: [i32; 2]) {
    let scale = origin.w / 64.;
    crate::world_readability::restore();
    if let Some(atmosphere) = &art.atmosphere {
        atmosphere.begin_surface(texture.size());
    }
    draw_texture_ex(
        texture,
        origin.x + at_px[0] as f32 * scale,
        origin.y + at_px[1] as f32 * scale,
        WHITE,
        DrawTextureParams {
            dest_size: Some(texture.size() * scale),
            ..Default::default()
        },
    );
}

fn interior_visible(roof_id: Option<&str>, s: &street::Street, game: &WorldState) -> bool {
    roof_id.is_none_or(|id| {
        s.roofs
            .iter()
            .any(|roof| roof.id == id && roof.hidden(game))
    })
}

fn draw_scenery_props(camera: &GridCamera, s: &street::Street, art: &Art, game: &WorldState) {
    #[cfg(debug_assertions)]
    if REVIEW_GRIM.get() == Some(false) {
        return;
    }
    let origin = camera.rect(GridPos::new(0, 0));
    #[cfg(debug_assertions)]
    if crate::street_defense_review::enabled() {
        crate::street_defense_review::draw_cover_shadows(origin.point(), origin.w / 64.);
    }
    for (prop, texture) in s.scenery_props.iter().zip(&art.props) {
        if interior_visible(prop.interior_roof.as_deref(), s, game) {
            draw_ground_prop(origin, art, texture, prop.at_px);
        }
    }
    if let Some(atmosphere) = &art.atmosphere {
        atmosphere.begin_background(LIGHTS.get());
    }
}
#[cfg(debug_assertions)]
pub fn review_legacy_doors(value: bool) {
    REVIEW_LEGACY_DOORS.set(value);
}
#[cfg(debug_assertions)]
pub fn review_ambient(value: Option<[f32; 3]>) {
    REVIEW_AMBIENT.set(value);
}

#[cfg(debug_assertions)]
pub struct Review;
#[cfg(debug_assertions)]
pub fn review() -> Review {
    REVIEW.set(true);
    LIGHTS.set(true);
    ART.with(|art| *art.borrow_mut() = None);
    Review
}
#[cfg(debug_assertions)]
impl Drop for Review {
    fn drop(&mut self) {
        REVIEW.set(false);
        LIGHTS.set(true);
        ART.with(|art| *art.borrow_mut() = None);
    }
}
#[cfg(debug_assertions)]
pub fn review_lights(on: bool) {
    LIGHTS.set(on);
}

fn world_rect(camera: &GridCamera, s: &street::Street) -> Rect {
    let r = camera.rect(GridPos::new(0, 0));
    Rect::new(r.x, r.y, r.w * s.width as f32, r.h * s.height as f32)
}

pub fn active(game: &WorldState) -> bool {
    #[cfg(debug_assertions)]
    if crate::sewer_review::active(game) {
        return true;
    }
    game.current_zone().is_some_and(|z| {
        z.id.as_str() == street::ZONE_ID || crate::sewer::index_for_id(z.id.as_str()).is_some()
    })
}

pub fn draw(camera: &GridCamera, game: &WorldState, known: impl Fn(GridPos) -> bool, time: f32) {
    let s = scene_for_render(game);
    ART.with(|art| {
        let mut art = art.borrow_mut();
        if art
            .as_ref()
            .is_some_and(|art| art.height != s.height || art.background_file != s.background)
        {
            *art = None;
        }
        let art = art.get_or_insert_with(|| Art {
            height: s.height,
            background_file: s.background.clone(),
            background: load(if REVIEW.get() && s.height == 18 {
                "assets/source-art/first-sector-review-01/u01-style-r07/SCENE-U01-07-r01.png"
            } else {
                &s.background
            }),
            #[cfg(debug_assertions)]
            review_background: None,
            car: load(&s.car_file),
            flame: load("assets/animations/agent/skill-animation-review-06/flame-atlas.png"),
            roofs: s.roofs.iter().map(|roof| load(&roof.file)).collect(),
            props: s
                .scenery_props
                .iter()
                .map(|prop| load(&prop.file))
                .collect(),
            atmosphere: s.atmosphere.as_ref().map(|cfg| {
                crate::camp_atmosphere::Atmosphere::new(
                    cfg,
                    s.width as u32 * 64,
                    s.height as u32 * 64,
                )
            }),
        });
        if let Some(atmosphere) = &art.atmosphere {
            #[cfg(debug_assertions)]
            atmosphere.review_surface(REVIEW_WHOLE_STYLE.get());
            unsafe {
                macroquad::window::get_internal_gl().quad_gl.scissor(None);
            }
            atmosphere.begin(world_rect(camera, s));
            atmosphere.background_time(time);
            atmosphere.begin_background(LIGHTS.get());
        }
        let mut origin = camera.rect(street::pos(s.image_origin));
        origin.x += s.image_offset_px[0] as f32 * origin.w / 64.;
        origin.y += s.image_offset_px[1] as f32 * origin.h / 64.;
        #[cfg(debug_assertions)]
        let background = if let Some(path) = crate::floor_surface_review::street_path(&s.background)
        {
            art.review_background.get_or_insert_with(|| load(path))
        } else {
            &art.background
        };
        #[cfg(not(debug_assertions))]
        let background = &art.background;
        draw_texture_ex(
            background,
            origin.x,
            origin.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    origin.w * s.image_size[0] as f32,
                    origin.h * s.image_size[1] as f32,
                )),
                ..Default::default()
            },
        );
        #[cfg(debug_assertions)]
        crate::sewer_review::water(camera, game, time);
        if let Some(plan) = crate::sewer::current_plan(game) {
            crate::sewer_ambience::water(camera, plan, time);
        }
        // Approved static bodies belong below living actors.
        // The existing lighting material and visibility pass apply to both.
        draw_scenery_props(camera, s, art, game);
        #[cfg(debug_assertions)]
        crate::sewer_review::rats(camera, game, time);
        if let Some(plan) = crate::sewer::current_plan(game) {
            crate::sewer_ambience::rats(camera, plan, time);
        }
        #[cfg(debug_assertions)]
        if crate::street_defense_review::enabled() {
            if let Some(atmosphere) = &art.atmosphere {
                let r = camera.rect(GridPos::new(0, 0));
                crate::street_defense_review::draw(r.point(), r.w / 64., time, atmosphere);
                atmosphere.begin_background(LIGHTS.get());
            }
        }
        if let Some(doorway) = &s.doorway {
            // The old luminous lintel crossed the walkable threshold. Reuse
            // neutral floor pixels there and mount that same fixture on the
            // adjacent wall. Its UV emission mask stays attached to the lens;
            // the irradiance source follows the new destination separately.
            patch(
                &art.background,
                origin,
                doorway.floor_source,
                doorway.opening_at,
            );
            patch(
                &art.background,
                origin,
                doorway.lamp_source,
                doorway.lamp_at,
            );
        }
        for extra in &s.extra_doorways {
            let doorway = &extra.patch;
            patch(
                &art.background,
                origin,
                doorway.floor_source,
                doorway.opening_at,
            );
            patch(
                &art.background,
                origin,
                doorway.lamp_source,
                doorway.lamp_at,
            );
        }
        let scenery_doors = crate::terminal_view::scenery_circle_enabled();
        #[cfg(debug_assertions)]
        let scenery_doors = scenery_doors && !REVIEW_LEGACY_DOORS.get();
        for roof in &s.roofs {
            let Some(at) = roof.door.map(street::pos) else {
                continue;
            };
            // Static doors follow the same scenery presentation as their
            // facades. Actor LOS must not make a closed panel pop in and out.
            if (scenery_doors || game.player_visibility().is_visible(at))
                && game.map().tile(at).is_some_and(|t| {
                    matches!(t.terrain, project_rl::world::Terrain::Door(state)
                        if state != project_rl::world::DoorState::Open)
                })
            {
                let (source, at) = s
                    .extra_doorways
                    .iter()
                    .find(|d| street::pos(d.door) == at)
                    .map(|d| &d.patch)
                    .or(s.doorway.as_ref())
                    .map_or(([733., 305., 48., 30.], [166., 315.]), |doorway| {
                        (doorway.closed_source, doorway.closed_at)
                    });
                patch(&art.background, origin, source, at);
            }
        }
        if let Some(atmosphere) = &art.atmosphere {
            atmosphere.end_material();
            if let Some(ambience) = &s.ambience {
                let scale = origin.w / 64.;
                atmosphere.steam(
                    vec2(
                        origin.x + ambience.steam[0] * scale,
                        origin.y + ambience.steam[1] * scale,
                    ),
                    time,
                );
            }
        }
        let car = camera.rect(street::pos(s.car_at));
        if let Some(atmosphere) = &art.atmosphere {
            atmosphere.begin_surface(art.car.size());
        }
        draw_texture_ex(
            &art.car,
            car.x,
            car.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(
                    car.w * s.car_size[0] as f32,
                    car.h * s.car_size[1] as f32,
                )),
                ..Default::default()
            },
        );
        crate::world_readability::restore();
        let frame = (time.max(0.) * 8.) as u32 % 8;
        let _animated_color = crate::world_readability::color();
        // Same native flame placements as the reviewed two-cell car preview.
        for (x, y, w, h, phase) in [
            (91., 0., 32., 48., 0),
            (72., 8., 28., 42., 3),
            (104., 20., 20., 34., 5),
        ] {
            let scale = car.w / 64.;
            draw_texture_ex(
                &art.flame,
                car.x + x * scale,
                car.y + y * scale,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(
                        ((frame + phase) % 8) as f32 * 128.,
                        0.,
                        128.,
                        128.,
                    )),
                    dest_size: Some(vec2(w * scale, h * scale)),
                    ..Default::default()
                },
            );
        }
        if s.ambience.as_ref().is_some_and(|a| a.embers) {
            for i in 0..7 {
                let age = (time * 0.4 + i as f32 * 0.143).fract();
                let scale = car.w / 64.;
                let x = car.x + (91. + (i as f32 * 2.1 + age * 3.).sin() * 9.) * scale;
                let y = car.y + (28. - age * 28.) * scale;
                draw_circle(x, y, scale, Color::new(1., 0.43, 0.13, (1. - age) * 0.65));
            }
        }
    });
    if crate::terminal_view::scenery_circle_enabled() {
        return;
    }
    #[cfg(debug_assertions)]
    if crate::terminal_view::world_visibility_review::enabled() {
        if !crate::terminal_view::world_visibility_review::scenery_circle() {
            crate::terminal_view::world_visibility_review::draw(game, camera, known);
        }
        return;
    }
    // Mask even remembered walls: a painted background must not expose rooms
    // or roads through unexplored terrain. Keep the existing viewport scissor.
    for y in 0..camera.rows {
        for x in 0..camera.columns {
            let at = GridPos::new(camera.first.x + x, camera.first.y + y);
            let alpha = if !known(at) {
                1.
            } else if !game.player_visibility().is_visible(at) {
                0.72
            } else {
                0.
            };
            if alpha > 0. {
                let r = camera.rect(at);
                draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0., 0., 0., alpha));
            }
        }
    }
}

pub fn finish(camera: &GridCamera, clip: Rect, game: &WorldState) {
    let s = scene_for_render(game);
    ART.with(|art| {
        if let Some(atmosphere) = art
            .borrow()
            .as_ref()
            .and_then(|art| art.atmosphere.as_ref())
        {
            #[cfg(debug_assertions)]
            {
                let ambient = s.atmosphere.as_ref().unwrap().ambient_tint();
                atmosphere.review_ambient(
                    REVIEW_AMBIENT
                        .get()
                        .unwrap_or([ambient.r, ambient.g, ambient.b]),
                );
            }
            atmosphere.finish(world_rect(camera, s), clip, LIGHTS.get());
        }
    });
}

/// Draw opaque exterior surfaces after room illumination. Interior lamps must
/// not light the roof above them. Only observed exteriors can expose a roof;
/// rendering does not discover any ground cells beneath it.
pub fn roofs(camera: &GridCamera, game: &WorldState, known: impl Fn(GridPos) -> bool) {
    let s = scene_for_render(game);
    let r = camera.rect(street::pos(s.image_origin));
    let scale = r.w / 64.;
    let ambient = s
        .atmosphere
        .as_ref()
        .map_or(WHITE, |cfg| cfg.ambient_tint());
    #[cfg(debug_assertions)]
    let ambient = REVIEW_AMBIENT
        .get()
        .map_or(ambient, |[r, g, b]| Color::new(r, g, b, 1.));
    ART.with(|art| {
        let art = art.borrow();
        let Some(art) = art.as_ref() else { return };
        for (index, (roof, texture)) in s.roofs.iter().zip(&art.roofs).enumerate() {
            if roof.hidden(game) {
                continue;
            }
            let exposure = roof.exposure(|at| game.player_visibility().is_visible(at), &known);
            let exposure = if crate::terminal_view::scenery_circle_enabled() {
                Some(1.)
            } else {
                exposure
            };
            #[cfg(debug_assertions)]
            let exposure = if crate::terminal_view::world_visibility_review::enabled()
                && !crate::terminal_view::world_visibility_review::scenery_circle()
                && index >= 2
            {
                crate::terminal_view::world_visibility_review::roof_exposure(index)
            } else {
                exposure
            };
            #[cfg(not(debug_assertions))]
            let _ = index;
            let Some(exposure) = exposure else {
                continue;
            };
            if let Some(atmosphere) = &art.atmosphere {
                atmosphere.begin_surface(texture.size());
            }
            draw_texture_ex(
                texture,
                r.x + (s.image_offset_px[0] + roof.at_px[0]) as f32 * scale,
                r.y + (s.image_offset_px[1] + roof.at_px[1]) as f32 * scale,
                Color::new(
                    ambient.r * exposure,
                    ambient.g * exposure,
                    ambient.b * exposure,
                    1.,
                ),
                DrawTextureParams {
                    dest_size: Some(texture.size() * scale),
                    ..Default::default()
                },
            );
            crate::world_readability::restore();
        }
    });
}

#[cfg(debug_assertions)]
pub fn export_review_world(game: &WorldState, path: &std::path::Path) -> Result<(), String> {
    let s = scene_for_render(game);
    ART.with(|art| {
        let art = art.borrow();
        let art = art.as_ref().ok_or("Street art not initialized")?;
        let atmosphere = art.atmosphere.as_ref().ok_or("Street atmosphere absent")?;
        let ambient = s.atmosphere.as_ref().unwrap().ambient_tint();
        atmosphere.export_review_world(path, || {
            for (prop, texture) in s.scenery_props.iter().zip(&art.props) {
                // The defense review's framebuffer already contains lit props
                // and guards in their correct layer order. Do not cover them
                // with a second unlit copy in the full-map export.
                if !crate::street_defense_review::enabled()
                    && interior_visible(prop.interior_roof.as_deref(), s, game)
                {
                    atmosphere.begin_surface(texture.size());
                    draw_texture(texture, prop.at_px[0] as f32, prop.at_px[1] as f32, WHITE);
                    crate::world_readability::restore();
                }
            }
            for (roof, texture) in s.roofs.iter().zip(&art.roofs) {
                if roof.hidden(game) {
                    continue;
                }
                atmosphere.begin_surface(texture.size());
                draw_texture(
                    texture,
                    (s.image_origin[0] * 64 + s.image_offset_px[0] + roof.at_px[0]) as f32,
                    (s.image_origin[1] * 64 + s.image_offset_px[1] + roof.at_px[1]) as f32,
                    ambient,
                );
                crate::world_readability::restore();
            }
            // The opt-in population proposal has an uncropped technical map;
            // its all-monster export does not affect normal game visibility.
            if path
                .file_stem()
                .is_some_and(|s| s.to_string_lossy().ends_with("-map"))
            {
                atmosphere.begin_surface(vec2(64., 64.));
                crate::sewer_review::draw_overview_population(game);
                crate::world_readability::restore();
            }
        })
    })
}

/// Only the side jambs can overlap the actor at the open threshold. The lamp
/// is fixed to the adjacent wall and never redrawn across the passage.
pub fn foreground(camera: &GridCamera, game: &WorldState) {
    let s = scene_for_render(game);
    ART.with(|art| {
        let art = art.borrow();
        let Some(art) = art.as_ref() else { return };
        if let Some(atmosphere) = &art.atmosphere {
            atmosphere.begin_background(LIGHTS.get());
        }
        let r = camera.rect(street::pos(s.image_origin));
        let scale = r.w / 64.;
        let shift = s.doorway.as_ref().map_or(0., |d| d.opening_at[0] - 160.);
        let lintel_y = s.doorway.as_ref().map_or(296., |d| d.opening_at[1]);
        let segments = s
            .doorway
            .as_ref()
            .map_or([(144., 240.), (240., 240.)], |doorway| {
                [
                    (144. + shift, doorway.opening_at[0]),
                    (
                        doorway.opening_at[0] + doorway.floor_source[2],
                        240. + shift,
                    ),
                ]
            });
        // Split on logical cells so the front layer cannot reveal a lintel
        // fragment in unknown or remembered terrain. Its fixed world placement
        // also preserves occlusion throughout the existing movement glide.
        for x in (s.home_entry[0] - 1)..=(s.home_entry[0] + 1) {
            let at = GridPos::new(x, s.home_entry[1]);
            if !game.player_visibility().is_visible(at) {
                continue;
            }
            let tile_left = ((x - s.image_origin[0]) * 64 - s.image_offset_px[0]) as f32;
            for (from, to) in segments {
                let left = tile_left.max(from);
                let right = (tile_left + 64.).min(to);
                if left >= right {
                    continue;
                }
                let source = Rect::new(left, lintel_y, right - left, 24.);
                draw_texture_ex(
                    &art.background,
                    r.x + (s.image_offset_px[0] as f32 + source.x) * scale,
                    r.y + (s.image_offset_px[1] as f32 + source.y) * scale,
                    WHITE,
                    DrawTextureParams {
                        source: Some(source),
                        dest_size: Some(source.size() * scale),
                        ..Default::default()
                    },
                );
            }
        }
        if let Some(atmosphere) = &art.atmosphere {
            atmosphere.end_material();
        }
    });
}
