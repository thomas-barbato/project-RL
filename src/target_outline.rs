//! HOVER-OUTLINE-01 r01. Review-only, generic composed-sprite silhouette.
use crate::game_art::{self, Visual};
use macroquad::miniquad as mq;
use macroquad::prelude::*;
use project_rl::{game::WorldState, social::PlayerRelation, world::GridPos};
use std::{cell::RefCell, collections::BTreeMap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Friendly,
    Hostile,
}
impl Tone {
    pub fn color(self) -> Color {
        match self {
            Self::Friendly => Color::from_rgba(70, 238, 114, 255),
            Self::Hostile => Color::from_rgba(250, 65, 53, 255),
        }
    }
}

thread_local! {
    static MASK: std::cell::Cell<bool> = const {std::cell::Cell::new(false)};
    static GPU: RefCell<Option<Outline>> = const {RefCell::new(None)};
}
#[cfg(any(debug_assertions, test))]
thread_local! {
    static REVIEW: std::cell::Cell<bool> = const {std::cell::Cell::new(false)};
    static POINTER: std::cell::Cell<Option<GridPos>> = const {std::cell::Cell::new(None)};
}
pub fn capturing_body_mask() -> bool {
    MASK.with(|m| m.get())
}
pub fn enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        REVIEW.with(|r| r.get())
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        false
    }
}
pub fn pointer(actual: Option<GridPos>) -> Option<GridPos> {
    #[cfg(any(debug_assertions, test))]
    {
        POINTER.with(|p| p.get()).or(actual)
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        actual
    }
}
#[cfg(any(debug_assertions, test))]
pub struct ReviewGuard(bool, Option<GridPos>);
#[cfg(any(debug_assertions, test))]
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        REVIEW.with(|r| r.set(self.0));
        POINTER.with(|p| p.set(self.1));
    }
}
#[cfg(any(debug_assertions, test))]
pub fn review(at: GridPos) -> ReviewGuard {
    ReviewGuard(
        REVIEW.with(|r| r.replace(true)),
        POINTER.with(|p| p.replace(Some(at))),
    )
}

pub fn tone_at(game: &WorldState, at: GridPos) -> Option<Tone> {
    if !game.player_visibility().is_visible(at) {
        return None;
    }
    let id = game.actors().entity_at(at)?;
    let actor = game.actors().get(id)?;
    if !actor.is_alive() {
        return None;
    }
    if id == game.player_id() || game.is_player_controlled_companion(id) {
        return Some(Tone::Friendly);
    }
    if actor.player_relation() == PlayerRelation::Hostile {
        return Some(Tone::Hostile);
    }
    (id == game.player_id()
        || actor.player_relation() == PlayerRelation::Allied
        || game.active_artisan(id)
        || game.active_merchant(id)
        || game.active_clinic(id)
        || game.active_quest_provider(id)
        || game.active_resident(id)
        || crate::cryogenic_camp::actor_index(actor).is_some()
        || actor
            .tags()
            .iter()
            .any(|t| t.as_str() == "core:conversation_contact"))
    .then_some(Tone::Friendly)
}

fn canvas(visual: Visual) -> (u32, Rect) {
    if let Visual::CampaignMonster { number, variant } = visual
        && let Some(a) = project_rl::monster_art::art(number, variant)
    {
        let side = (u32::from(a.side) + 8).next_power_of_two();
        return (
            side,
            Rect::new(
                a.foot[0] as f32 + 4. - 32.,
                a.foot[1] as f32 + 4. - 64.,
                64.,
                64.,
            ),
        );
    }
    (256, Rect::new(96., 128., 64., 64.))
}

struct Outline {
    material: Material,
    frames: BTreeMap<u32, RenderTarget>,
}
impl Outline {
    fn new() -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    mq::UniformDesc::new("pixel", mq::UniformType::Float2),
                    mq::UniformDesc::new("edge_color", mq::UniformType::Float4),
                ],
                pipeline_params: mq::PipelineParams {
                    color_blend: Some(mq::BlendState::new(
                        mq::Equation::Add,
                        mq::BlendFactor::Value(mq::BlendValue::SourceAlpha),
                        mq::BlendFactor::OneMinusValue(mq::BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("silhouette review shader");
        Self {
            material,
            frames: BTreeMap::new(),
        }
    }
}

/// Render only the hovered actor into one reusable native mask, including its
/// held layers and current pose. Draw the exterior stroke at that actor's depth.
pub fn draw(cell: Rect, visual: Visual, time: Option<f64>, tone: Tone, clip: Option<Rect>) {
    if !enabled() {
        return;
    }
    GPU.with(|gpu| {
        let mut gpu = gpu.borrow_mut();
        let gpu = gpu.get_or_insert_with(Outline::new);
        let (side, local) = canvas(visual);
        let frame = gpu
            .frames
            .entry(side)
            .or_insert_with(|| {
                let t = render_target(side, side);
                t.texture.set_filter(FilterMode::Nearest);
                t
            })
            .clone();
        push_camera_state();
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., side as f32, side as f32));
        camera.render_target = Some(frame.clone());
        set_camera(&camera);
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        clear_background(Color::new(0., 0., 0., 0.));
        MASK.with(|m| m.set(true));
        game_art::draw_world_animated(local, visual, 1., time);
        MASK.with(|m| m.set(false));
        gl_use_default_material();
        pop_camera_state();
        unsafe {
            macroquad::window::get_internal_gl()
                .quad_gl
                .scissor(clip.map(|r| (r.x as i32, r.y as i32, r.w as i32, r.h as i32)));
        }
        let color = tone.color();
        gpu.material
            .set_uniform("pixel", (1. / side as f32, 1. / side as f32));
        gpu.material
            .set_uniform("edge_color", (color.r, color.g, color.b, color.a));
        gl_use_material(&gpu.material);
        let scale = cell.w / 64.;
        draw_texture_ex(
            &frame.texture,
            cell.x - local.x * scale,
            cell.y - local.y * scale,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(side as f32 * scale, side as f32 * scale)),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    });
}

#[cfg(debug_assertions)]
pub fn export_mask(visual: Visual, cell: Rect, path: &std::path::Path) -> [f32; 3] {
    let (side, local) = canvas(visual);
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    GPU.with(|g| {
        g.borrow().as_ref().unwrap().frames[&side]
            .texture
            .get_texture_data()
            .export_png(path.to_str().unwrap())
    });
    [cell.x - local.x, cell.y - local.y, side as f32]
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;
void main(){uv=texcoord;gl_Position=Projection*Model*vec4(position,1.0);}"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;uniform sampler2D Texture;uniform vec2 pixel;uniform vec4 edge_color;varying highp vec2 uv;
void main(){
 if(texture2D(Texture,uv).a>0.1) discard;
 float a=0.0;
 for(int x=-2;x<=2;x++){for(int y=-2;y<=2;y++){
   a=max(a,texture2D(Texture,uv+vec2(float(x),float(y))*pixel).a);
 }}
 gl_FragColor=vec4(edge_color.rgb,edge_color.a*step(0.1,a));
}"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn campaign_canvases_keep_the_whole_native_body_and_padding() {
        for a in project_rl::monster_art::MONSTER_ART {
            let (side, cell) = canvas(Visual::CampaignMonster {
                number: a.number,
                variant: a.variant,
            });
            let left = cell.center().x - a.foot[0] as f32;
            let top = cell.bottom() - a.foot[1] as f32;
            assert_eq!((left, top), (4., 4.));
            assert!(left + a.side as f32 + 4. <= side as f32);
        }
    }
    #[test]
    fn hover_color_obeys_relation_services_visibility_and_empty_ground() {
        use project_rl::{
            entity::Actor,
            game::{GameState, WorldState},
            world::Map,
        };
        let map = Map::from_ascii("#########\n#..#....#\n#..#....#\n#########").unwrap();
        let state = GameState::new(map, GridPos::new(1, 1), 17).unwrap();
        let mut game = WorldState::single(state);
        for (at, relation) in [
            (GridPos::new(2, 1), PlayerRelation::Allied),
            (GridPos::new(2, 2), PlayerRelation::Hostile),
            (GridPos::new(5, 1), PlayerRelation::Hostile),
        ] {
            game.spawn_actor(Actor::new(at, 20).unwrap().with_player_relation(relation))
                .unwrap();
        }
        assert_eq!(tone_at(&game, GridPos::new(2, 1)), Some(Tone::Friendly));
        assert_eq!(tone_at(&game, GridPos::new(2, 2)), Some(Tone::Hostile));
        assert_eq!(tone_at(&game, GridPos::new(5, 1)), None);
        assert_eq!(tone_at(&game, GridPos::new(1, 2)), None);
        game.spawn_actor(
            Actor::new(GridPos::new(1, 2), 20)
                .unwrap()
                .with_tags(vec!["core:conversation_contact".parse().unwrap()]),
        )
        .unwrap();
        assert_eq!(tone_at(&game, GridPos::new(1, 2)), Some(Tone::Friendly));
        assert!(!enabled());
        assert_eq!(pointer(Some(GridPos::new(2, 1))), Some(GridPos::new(2, 1)));
        {
            let _preview = review(GridPos::new(2, 2));
            assert!(enabled());
            assert_eq!(pointer(None), Some(GridPos::new(2, 2)));
        }
        assert!(!enabled());
        assert_eq!(pointer(None), None);
    }
}
