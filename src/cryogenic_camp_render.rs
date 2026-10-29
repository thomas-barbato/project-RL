//! Reviewed scenery in the real camera, with the real visibility and actors.
use crate::camp_atmosphere::Atmosphere;
use crate::cryogenic_camp::{self as camp, Sprite};
use crate::terminal_view::GridCamera;
use macroquad::prelude::*;
use project_rl::game::WorldState;
use project_rl::world::GridPos;
use std::cell::RefCell;

struct Scene {
    floor: Texture2D,
    #[cfg(debug_assertions)]
    review_floor: Option<(&'static str, Texture2D)>,
    structure: Texture2D,
    #[cfg(debug_assertions)]
    review_structure: Option<(&'static str, Texture2D)>,
    props: Vec<Texture2D>,
    npcs: Vec<Texture2D>,
    order: Vec<usize>,
    next_prop: usize,
    atmosphere: Atmosphere,
}
fn load(file: &str) -> Texture2D {
    let bytes = project_rl::data_paths::read(file).expect("accepted camp texture");
    let t = crate::asset_loading::texture(&bytes, Some(ImageFormat::Png));
    t.set_filter(FilterMode::Nearest);
    t
}
impl Scene {
    fn new() -> Self {
        let c = camp::scene();
        let mut order: Vec<_> = (0..c.props.len()).collect();
        order.sort_by_key(|&i| (c.props[i].at[1] + c.props[i].height, c.props[i].at[0]));
        Self {
            floor: load(&c.floor),
            #[cfg(debug_assertions)]
            review_floor: None,
            structure: load(&c.structure),
            #[cfg(debug_assertions)]
            review_structure: None,
            props: c.props.iter().map(|p| load(&p.file)).collect(),
            npcs: c.actors[1..].iter().map(|a| load(&a.file)).collect(),
            order,
            next_prop: 0,
            atmosphere: Atmosphere::new(&c.atmosphere, c.width as u32 * 64, c.height as u32 * 64),
        }
    }
}
thread_local! {static SCENE:RefCell<Option<Scene>>=const{RefCell::new(None)};}
fn with_scene<R>(draw: impl FnOnce(&mut Scene) -> R) -> R {
    SCENE.with(|s| {
        let mut s = s.borrow_mut();
        draw(s.get_or_insert_with(Scene::new))
    })
}
pub fn active(game: &WorldState) -> bool {
    game.current_zone()
        .is_some_and(|z| z.id.as_str() == camp::ZONE_ID)
}
fn world_rect(camera: &GridCamera) -> Rect {
    let origin = camera.rect(GridPos::new(0, 0));
    let c = camp::scene();
    Rect::new(
        origin.x,
        origin.y,
        origin.w * c.width as f32,
        origin.h * c.height as f32,
    )
}
fn sprite_rect(camera: &GridCamera, s: &Sprite) -> Rect {
    let cell = camera.rect(camp::pos(s.at));
    let scale = cell.w / 64.;
    Rect::new(
        cell.x + (s.width as f32 * 32. - 64.) * scale,
        cell.y + (s.height as f32 * 64. - 96.) * scale,
        128. * scale,
        128. * scale,
    )
}
fn draw(t: &Texture2D, r: Rect, opacity: f32) {
    draw_texture_ex(
        t,
        r.x,
        r.y,
        Color::new(1., 1., 1., opacity),
        DrawTextureParams {
            dest_size: Some(r.size()),
            ..Default::default()
        },
    );
}
pub fn begin(camera: &GridCamera) {
    unsafe {
        macroquad::window::get_internal_gl().quad_gl.scissor(None);
    }
    with_scene(|s| {
        let r = world_rect(camera);
        s.atmosphere.begin(r);
        s.atmosphere.begin_background(true);
        #[cfg(debug_assertions)]
        let floor = if let Some(path) = crate::floor_surface_review::camp_path() {
            if s.review_floor
                .as_ref()
                .is_none_or(|(current, _)| *current != path)
            {
                s.review_floor = Some((path, load(path)));
            }
            &s.review_floor.as_ref().expect("review floor loaded").1
        } else {
            &s.floor
        };
        #[cfg(not(debug_assertions))]
        let floor = &s.floor;
        draw(floor, r, 1.);
        #[cfg(debug_assertions)]
        let structure = if let Some(path) = crate::floor_surface_review::structure_path() {
            if s.review_structure
                .as_ref()
                .is_none_or(|(current, _)| *current != path)
            {
                s.review_structure = Some((path, load(path)));
            }
            &s.review_structure
                .as_ref()
                .expect("review structure loaded")
                .1
        } else {
            &s.structure
        };
        #[cfg(not(debug_assertions))]
        let structure = &s.structure;
        draw(structure, r, 1.);
        s.atmosphere.end_material();
        s.next_prop = 0;
    });
}
pub fn props_before(camera: &GridCamera, key: (i32, i32), time: f32, game: &WorldState) {
    with_scene(|s| {
        let c = camp::scene();
        while let Some(&i) = s.order.get(s.next_prop) {
            let p = &c.props[i];
            if (p.at[1] + p.height, p.at[0]) > key {
                break;
            }
            s.next_prop += 1;
            // Same contact shadow and native foot anchor as the reviewed prototype.
            let r = sprite_rect(camera, p);
            let scale = r.w / 128.;
            draw_rectangle(
                r.x + 48. * scale,
                r.y + 93. * scale,
                32. * scale,
                5. * scale,
                Color::from_rgba(8, 12, 14, 76),
            );
            s.atmosphere
                .begin_prop(&p.id, time, p.at[0] as f32 * 0.71 + i as f32 * 0.43);
            draw(&s.props[i], r, 1.);
            s.atmosphere.end_material();
            if p.id == "DEC-CAMP-16" && game.player_visibility().is_visible(camp::pos(p.at)) {
                let cell = camera.rect(camp::pos(p.at));
                s.atmosphere
                    .steam(vec2(cell.center().x, cell.bottom()), time);
            }
        }
    });
}
pub fn finish(
    camera: &GridCamera,
    clip: Rect,
    game: &WorldState,
    known: impl Fn(GridPos) -> bool,
    time: f32,
) {
    props_before(camera, (i32::MAX, i32::MAX), time, game);
    if crate::terminal_view::scenery_circle_enabled() {
        with_scene(|s| s.atmosphere.finish(world_rect(camera), clip, true));
        return;
    }
    #[cfg(debug_assertions)]
    if crate::terminal_view::world_visibility_review::enabled() {
        if !crate::terminal_view::world_visibility_review::scenery_circle() {
            crate::terminal_view::world_visibility_review::draw(game, camera, known);
        }
        with_scene(|s| s.atmosphere.finish(world_rect(camera), clip, true));
        return;
    }
    let c = camp::scene();
    for y in 0..c.height as i32 {
        for x in 0..c.width as i32 {
            let at = GridPos::new(x, y);
            let opacity = if !known(at) {
                1.
            } else if !game.player_visibility().is_visible(at) {
                0.72
            } else {
                0.
            };
            if opacity > 0. {
                let r = camera.rect(at);
                draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0., 0., 0., opacity));
            }
        }
    }
    with_scene(|s| s.atmosphere.finish(world_rect(camera), clip, true));
}
pub fn draw_npc(cell: Rect, index: u8, opacity: f32, world: bool, idle_time: Option<f64>) {
    with_scene(|s| {
        let t = &s.npcs[index as usize - 1];
        let r = if world {
            Rect::new(
                cell.x - cell.w / 2.,
                cell.bottom() - cell.w * 1.5,
                cell.w * 2.,
                cell.w * 2.,
            )
        } else {
            cell
        };
        if world && !crate::target_outline::capturing_body_mask() {
            let scale = r.w / 128.;
            draw_rectangle(
                r.x + 48. * scale,
                r.y + 93. * scale,
                32. * scale,
                5. * scale,
                Color::new(8. / 255., 12. / 255., 14. / 255., 76. / 255. * opacity),
            );
        }
        let breath = if world {
            crate::camp_npc_review::breath_offset(crate::camp_npc_review::animation_time(idle_time))
        } else {
            0.
        };
        if !world {
            draw(t, r, opacity);
        } else {
            // The 64 px body starts at (32,32) in its transparent 128 px
            // canvas. Native row 46 is the accepted humanoid waist seam.
            // The fixed lower strip covers the overlap; feet and shadow stay put.
            let scale = r.w / 128.;
            // Every world pose must bind the same actor material. A direct
            // draw at breath=0 inherited the scenery material and changed
            // exposure when breathing started or movement suppressed it.
            let strips = if breath == 0. {
                [(0., 128., 0.), (0., 0., 0.)]
            } else {
                [(0., 78., breath), (78., 50., 0.)]
            };
            for (y, height, offset) in strips {
                if height == 0. {
                    continue;
                }
                crate::world_readability::texture(
                    t,
                    r.x,
                    r.y + (y + offset) * scale,
                    Color::new(1., 1., 1., opacity),
                    DrawTextureParams {
                        source: Some(Rect::new(0., y, 128., height)),
                        dest_size: Some(vec2(r.w, height * scale)),
                        ..Default::default()
                    },
                );
            }
        }
    });
}
