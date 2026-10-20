//! One experimental fog pass for painted scenery, shared by camp and street.
//! It reads perception and memory only. Unknown ground remains opaque;
//! observed static sprites retain their complete alpha silhouette.
use crate::terminal_view::GridCamera;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use macroquad::prelude::*;
use project_rl::{
    game::WorldState,
    world::{DistanceMetric, GridPos},
};
use std::cell::{Cell, RefCell};

const VERTEX: &str = r#"#version 100
attribute vec3 position;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 point;
void main() {
    point=position.xy;
    gl_Position=Projection*Model*vec4(position,1.0);
}"#;

const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
uniform sampler2D field;
uniform vec2 origin;
uniform vec2 worldSize;
uniform vec2 texel;
uniform vec2 focus;
uniform vec2 radii;
uniform float circular;
uniform float sceneryCircle;
uniform sampler2D surfaces;
uniform sampler2D surfaceState;
uniform vec2 surfaceOrigin;
uniform vec2 surfaceSize;
void main() {
    vec2 uv=(point-origin)/worldSize;
    if(sceneryCircle>0.5) {
        float range=1.0-smoothstep(radii.x,radii.y,distance(point,focus));
        float memory=smoothstep(0.5,1.0,texture2D(field,uv).g)*0.45;
        gl_FragColor=vec4(0.0,0.0,0.0,1.0-max(range,memory));
        return;
    }
    vec2 cellCenter=(floor(uv/texel)+0.5)*texel;
    float knownCell=texture2D(field,cellCenter).g;
    vec2 surfaceUv=(point-surfaceOrigin)/surfaceSize;
    vec4 surface=vec4(0.0);
    if(surfaceUv.x>=0.0 && surfaceUv.y>=0.0 && surfaceUv.x<1.0 && surfaceUv.y<1.0)
        surface=texture2D(surfaces,surfaceUv);
    float surfaceId=floor(surface.r*255.0+0.5);
    vec2 objectState=texture2D(surfaceState,vec2((surfaceId+0.5)/256.0,0.5)).rg;
    float objectKnown=objectState.g*surface.a;
    if(knownCell<0.5 && objectKnown<0.001) {
        gl_FragColor=vec4(0.0,0.0,0.0,1.0);
        return;
    }
    vec2 data=texture2D(field,uv).rg;
    float sight=smoothstep(0.25,0.85,data.r);
    sight=mix(sight,objectState.r,objectKnown);
    float range=1.0-smoothstep(radii.x,radii.y,distance(point,focus));
    sight*=mix(1.0,range,circular);
    // Known terrain stays readable outside sight. Do not add another black
    // obstacle shadow or memory edge on top of this pass afterwards.
    float brightness=mix(0.45,1.0,sight);
    float discovery=max(smoothstep(0.5,1.0,data.g),objectKnown);
    gl_FragColor=vec4(0.0,0.0,0.0,1.0-brightness*discovery);
}"#;

thread_local! {
    static ENABLED: Cell<bool> = const { Cell::new(false) };
    static SCENERY_CIRCLE: Cell<bool> = const { Cell::new(false) };
    static MATERIAL: RefCell<Option<Material>> = const { RefCell::new(None) };
    static FIELD: RefCell<Option<(Image,Texture2D)>> = const { RefCell::new(None) };
    static SURFACES: RefCell<Vec<(String,Surfaces)>> = const { RefCell::new(Vec::new()) };
}

#[derive(serde::Deserialize)]
struct SurfaceObject {
    id: usize,
    witnesses: Vec<[i32; 2]>,
    covered_by: Option<usize>,
}
#[derive(serde::Deserialize)]
struct SurfaceManifest {
    texture: String,
    size: [f32; 2],
    objects: Vec<SurfaceObject>,
}
struct Surfaces {
    manifest: SurfaceManifest,
    texture: Texture2D,
    states: Image,
    state_texture: Texture2D,
}
fn surface_states(
    objects: &[SurfaceObject],
    visible: impl Fn(GridPos) -> bool,
    known: impl Fn(GridPos) -> bool,
) -> Image {
    let mut states = Image::gen_image_color(256, 1, BLACK);
    for object in objects {
        let witnessed = |sample: &dyn Fn(GridPos) -> bool| {
            object
                .witnesses
                .iter()
                .any(|&[x, y]| sample(GridPos::new(x, y)))
        };
        let current = witnessed(&visible);
        let seen = current || witnessed(&known);
        states.bytes[object.id * 4] = if current { 255 } else { 0 };
        states.bytes[object.id * 4 + 1] = if seen { 255 } else { 0 };
    }
    states
}
fn surface_field(
    game: &WorldState,
    known: &impl Fn(GridPos) -> bool,
) -> (Texture2D, Texture2D, [f32; 2]) {
    let name = if crate::cryogenic_camp_render::active(game) {
        "camp"
    } else {
        "street"
    };
    SURFACES.with(|cache| {
        let mut cache = cache.borrow_mut();
        let index = cache.iter().position(|(key, _)| key == name).unwrap_or_else(|| {
            let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
            let file = format!("assets/source-art/first-sector-review-01/u01-expansion-r06/visibility-surfaces-{name}-03.json");
            let manifest: SurfaceManifest = serde_json::from_slice(&std::fs::read(root.join(file)).unwrap()).unwrap();
            let texture = Texture2D::from_file_with_format(&std::fs::read(root.join(&manifest.texture)).unwrap(), Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            let states = Image::gen_image_color(256, 1, BLACK);
            let state_texture = Texture2D::from_image(&states);
            state_texture.set_filter(FilterMode::Nearest);
            cache.push((name.to_owned(), Surfaces { manifest, texture, states, state_texture }));
            cache.len()-1
        });
        let field = &mut cache[index].1;
        let mut states = surface_states(&field.manifest.objects, |at| game.player_visibility().is_visible(at), known);
        if name == "street" {
            for object in &field.manifest.objects {
                if object.covered_by.is_some_and(|index| crate::prison_street::scene_for_game(game).roofs[index].hidden(game)) {
                    states.bytes[object.id*4..object.id*4+2].fill(0);
                }
            }
        }
        if states.bytes != field.states.bytes {
            field.state_texture.update(&states);
            field.states = states;
        }
        (field.texture.clone(), field.state_texture.clone(), field.manifest.size)
    })
}

/// Observing the closed assembly authorizes its roof and facade together.
/// This never authorizes the terrain or actors in the room beneath it.
pub fn roof_exposure(index: usize) -> Option<f32> {
    SURFACES.with(|cache| {
        let cache = cache.borrow();
        let (_, field) = cache.iter().find(|(key, _)| key == "street")?;
        let object = field
            .manifest
            .objects
            .iter()
            .find(|o| o.covered_by == Some(index))?;
        let state = &field.states.bytes[object.id * 4..object.id * 4 + 2];
        if state[0] > 0 {
            Some(1.)
        } else if state[1] > 0 {
            Some(0.45)
        } else {
            None
        }
    })
}

pub fn set_enabled(value: bool) {
    ENABLED.set(value);
}
pub fn enabled() -> bool {
    ENABLED.get()
}
pub fn set_scenery_circle(value: bool) {
    SCENERY_CIRCLE.set(value);
}
pub fn scenery_circle() -> bool {
    enabled() && SCENERY_CIRCLE.get()
}

fn field_image(game: &WorldState, camera: &GridCamera, known: &impl Fn(GridPos) -> bool) -> Image {
    let w = (camera.columns + 4) as u16;
    let h = (camera.rows + 4) as u16;
    let mut image = Image::gen_image_color(w, h, BLACK);
    let focus = game
        .player_position()
        .unwrap_or(GridPos::new(i32::MIN / 2, i32::MIN / 2));
    let rules = game.rules().player_field_of_view;
    let radius = i64::from(rules.radius);
    for y in 0..i32::from(h) {
        for x in 0..i32::from(w) {
            let at = GridPos::new(camera.first.x + x - 2, camera.first.y + y - 2);
            let dx = i64::from(at.x) - i64::from(focus.x);
            let dy = i64::from(at.y) - i64::from(focus.y);
            let outside_circle = rules.radius > 0
                && rules.distance_metric == DistanceMetric::Euclidean
                && dx * dx + dy * dy > radius * radius;
            let index = (y as usize * usize::from(w) + x as usize) * 4;
            image.bytes[index] = if outside_circle || game.player_visibility().is_visible(at) {
                255
            } else {
                0
            };
            image.bytes[index + 1] = if known(at) { 255 } else { 0 };
        }
    }
    image
}

pub fn draw(game: &WorldState, camera: &GridCamera, known: impl Fn(GridPos) -> bool) {
    let Some(focus) = game.player_position() else {
        draw_rectangle(
            camera.viewport.x,
            camera.viewport.y,
            camera.viewport.w,
            camera.viewport.h,
            BLACK,
        );
        return;
    };
    let image = field_image(game, camera, &known);
    let texture = FIELD.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot
            .as_ref()
            .is_none_or(|(old, _)| old.width != image.width || old.height != image.height)
        {
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Linear);
            *slot = Some((image.clone(), texture));
        } else if let Some((old, texture)) = slot.as_mut() {
            if old.bytes != image.bytes {
                texture.update(&image);
                *old = image.clone();
            }
        }
        slot.as_ref().unwrap().1.clone()
    });
    let (surface_texture, surface_states, surface_size) = if scenery_circle() {
        (texture.clone(), texture.clone(), [1., 1.])
    } else {
        surface_field(game, &known)
    };
    MATERIAL.with(|slot| {
        let mut slot = slot.borrow_mut();
        let material = slot.get_or_insert_with(|| {
            load_material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("origin", UniformType::Float2),
                        UniformDesc::new("worldSize", UniformType::Float2),
                        UniformDesc::new("texel", UniformType::Float2),
                        UniformDesc::new("focus", UniformType::Float2),
                        UniformDesc::new("radii", UniformType::Float2),
                        UniformDesc::new("circular", UniformType::Float1),
                        UniformDesc::new("sceneryCircle", UniformType::Float1),
                        UniformDesc::new("surfaceOrigin", UniformType::Float2),
                        UniformDesc::new("surfaceSize", UniformType::Float2),
                    ],
                    textures: vec![
                        "field".to_owned(),
                        "surfaces".to_owned(),
                        "surfaceState".to_owned(),
                    ],
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
            .expect("visibility review shader")
        });
        let origin = camera.rect(GridPos::new(camera.first.x - 2, camera.first.y - 2));
        let rules = game.rules().player_field_of_view;
        let outer =
            (f32::from(rules.radius) - std::f32::consts::FRAC_1_SQRT_2).max(0.5) * camera.cell;
        let center = camera.rect(focus).center();
        material.set_uniform("origin", [origin.x, origin.y]);
        material.set_uniform(
            "worldSize",
            [
                f32::from(image.width) * camera.cell,
                f32::from(image.height) * camera.cell,
            ],
        );
        material.set_uniform(
            "texel",
            [1.0 / f32::from(image.width), 1.0 / f32::from(image.height)],
        );
        material.set_uniform("focus", [center.x, center.y]);
        material.set_uniform("sceneryCircle", if scenery_circle() { 1.0f32 } else { 0.0 });
        material.set_uniform("radii", [(outer - camera.cell * 1.2).max(0.0), outer]);
        material.set_uniform(
            "circular",
            if rules.distance_metric == DistanceMetric::Euclidean && rules.radius > 0 {
                1.0f32
            } else {
                0.0
            },
        );
        material.set_texture("field", texture);
        let world_origin = camera.rect(GridPos::new(0, 0));
        material.set_uniform("surfaceOrigin", [world_origin.x, world_origin.y]);
        material.set_uniform(
            "surfaceSize",
            [
                surface_size[0] * camera.cell / 64.,
                surface_size[1] * camera.cell / 64.,
            ],
        );
        material.set_texture("surfaces", surface_texture);
        material.set_texture("surfaceState", surface_states);
        gl_use_material(material);
        draw_rectangle(
            camera.viewport.x,
            camera.viewport.y,
            camera.viewport.w,
            camera.viewport.h,
            WHITE,
        );
        gl_use_default_material();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::{
        game::GameState,
        world::{Map, Terrain},
    };
    #[test]
    fn complete_scenery_is_authorized_only_by_its_own_observation() {
        let objects = [
            SurfaceObject {
                id: 1,
                witnesses: vec![[2, 2], [3, 2]],
                covered_by: None,
            },
            SurfaceObject {
                id: 2,
                witnesses: vec![[9, 9]],
                covered_by: None,
            },
        ];
        let at = GridPos::new(2, 2);
        let unknown = surface_states(&objects, |_| false, |_| false);
        assert_eq!(&unknown.bytes[4..12], &[0, 0, 0, 255, 0, 0, 0, 255]);
        let visible = surface_states(&objects, |p| p == at, |_| false);
        assert_eq!(&visible.bytes[4..12], &[255, 255, 0, 255, 0, 0, 0, 255]);
        let remembered = surface_states(&objects, |_| false, |p| p == at);
        assert_eq!(&remembered.bytes[4..12], &[0, 255, 0, 255, 0, 0, 0, 255]);
    }
    #[test]
    fn hidden_layout_does_not_change_the_fog_field_or_simulation() {
        let make = |hidden| {
            let mut map = Map::filled(24, 24, Terrain::Floor).unwrap();
            for y in 0..24 {
                map.set_terrain(GridPos::new(12, y), Terrain::Wall).unwrap();
            }
            map.set_terrain(GridPos::new(14, 12), hidden).unwrap();
            WorldState::single(GameState::new(map, GridPos::new(10, 12), 141).unwrap())
        };
        let a = make(Terrain::Floor);
        let b = make(Terrain::Wall);
        let camera = GridCamera {
            first: GridPos::new(4, 6),
            columns: 15,
            rows: 15,
            origin: Vec2::ZERO,
            cell: 64.,
            viewport: Rect::new(0., 0., 960., 960.),
        };
        let before = a.recovery_snapshot_bytes().unwrap();
        let sample =
            |g: &WorldState| field_image(g, &camera, &|at| g.player_visibility().is_visible(at));
        assert_eq!(sample(&a).bytes, sample(&b).bytes);
        assert_eq!(before, a.recovery_snapshot_bytes().unwrap());
    }
}
