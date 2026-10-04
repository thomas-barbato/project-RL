//! Analytic presentation of the perception radius. Simulation visibility and
//! the remembered terrain catalogue remain the sole information gates.
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use std::cell::RefCell;

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 point;
varying lowp float remembered;
varying lowp float occluder;
varying lowp float memoryCell;
void main() {
    point = position.xy;
    remembered = color0.r / 255.0;
    occluder = color0.g / 255.0;
    memoryCell = color0.b / 255.0;
    gl_Position = Projection * Model * vec4(position, 1.0);
}"#;

const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
varying lowp float remembered;
varying lowp float occluder;
varying lowp float memoryCell;
uniform vec2 center;
uniform vec2 radii;
uniform vec3 memoryTint;
uniform sampler2D visibilityMask;
uniform vec2 maskOrigin;
uniform vec2 maskWorldSize;
uniform vec2 maskTexel;
void main() {
    vec2 uv = (point - maskOrigin) / maskWorldSize;
    if (memoryCell > 0.5) {
        float edge = 1.0 - smoothstep(0.75, 1.0, texture2D(visibilityMask, uv).g);
        gl_FragColor = vec4(0.0, 0.0, 0.0, edge);
        return;
    }
    float fog = smoothstep(radii.x, radii.y, distance(point, center));
    float field = texture2D(visibilityMask, uv).r;
    // Bilinear reconstruction produces curved and diagonal contours. A .75
    // threshold keeps the bright region strictly inside logically visible cells:
    // a hidden centre and its three visible neighbours can reach at most .75.
    float shadow = (1.0 - smoothstep(0.75, 1.0, field)) * (1.0 - occluder);
    float memory = remembered;
    if (shadow > fog) {
        vec2 gradient = vec2(
            texture2D(visibilityMask, uv - vec2(maskTexel.x, 0.0)).r
                - texture2D(visibilityMask, uv + vec2(maskTexel.x, 0.0)).r,
            texture2D(visibilityMask, uv - vec2(0.0, maskTexel.y)).r
                - texture2D(visibilityMask, uv + vec2(0.0, maskTexel.y)).r);
        vec2 direction = gradient / max(length(gradient), 0.001);
        // Only saved terrain determines whether a shadow meets memory or void.
        memory = smoothstep(0.99, 1.0, texture2D(visibilityMask, uv + direction * maskTexel).g);
        fog = shadow;
    }
    gl_FragColor = vec4(memoryTint * fog * memory, fog);
}"#;

thread_local! {
    static MASK: RefCell<Option<Result<Material, String>>> = const { RefCell::new(None) };
    static FIELD: RefCell<Option<VisibilityField>> = const { RefCell::new(None) };
}

struct VisibilityField {
    image: Image,
    texture: Texture2D,
}

fn field_image(
    game: &WorldState,
    camera: &GridCamera,
    focus: GridPos,
    known: &impl Fn(GridPos) -> bool,
) -> Image {
    let width = (camera.columns + 4) as u16;
    let height = (camera.rows + 4) as u16;
    let radius = i64::from(game.rules().player_field_of_view.radius);
    let mut image = Image::gen_image_color(width, height, BLACK);
    for y in 0..i32::from(height) {
        for x in 0..i32::from(width) {
            let at = GridPos::new(camera.first.x + x - 2, camera.first.y + y - 2);
            let dx = i64::from(at.x) - i64::from(focus.x);
            let dy = i64::from(at.y) - i64::from(focus.y);
            // The analytic circle handles range. Its outer cells must not be
            // mistaken for obstacle shadows and erode the circular edge twice.
            let lit =
                dx * dx + dy * dy > radius * radius || game.player_visibility().is_visible(at);
            let offset = (y as usize * usize::from(width) + x as usize) * 4;
            image.bytes[offset..offset + 4].copy_from_slice(&[
                if lit { 255 } else { 0 },
                if known(at) { 255 } else { 0 },
                0,
                255,
            ]);
        }
    }
    image
}

/// The circular edge lies inside the union of visible tile squares, including
/// at oblique angles; a one-cell feather removes the old stair-step silhouette.
fn radii(radius: u16, cell: f32) -> [f32; 2] {
    let outer = (f32::from(radius) - std::f32::consts::FRAC_1_SQRT_2).max(0.5) * cell;
    [(outer - cell * 0.8).max(0.0), outer]
}

pub(super) fn draw(game: &WorldState, camera: &GridCamera, known: impl Fn(GridPos) -> bool) {
    let rules = game.rules().player_field_of_view;
    if rules.distance_metric != project_rl::world::DistanceMetric::Euclidean || rules.radius == 0 {
        return;
    }
    let Some(focus) = game.player_position() else {
        return;
    };
    let center = camera.rect(focus).center();
    let radii = radii(rules.radius, camera.cell);
    let image = field_image(game, camera, focus, &known);
    let texture = FIELD.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot.as_ref().is_none_or(|field| {
            field.image.width != image.width || field.image.height != image.height
        }) {
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Linear);
            *slot = Some(VisibilityField {
                image: image.clone(),
                texture,
            });
        } else if let Some(field) = slot.as_mut() {
            if field.image.bytes != image.bytes {
                field.texture.update(&image);
                field.image = image.clone();
            }
        }
        slot.as_ref().unwrap().texture.clone()
    });
    // Project each shared fringe vertex beyond the perception radius, using
    // known memory only. Interpolation keeps the transition continuous where
    // explored ground meets unknown space; it introduces no per-cell seam.
    let remembered_at = |point: Vec2| {
        let outward = (point - center).normalize_or_zero() * (f32::from(rules.radius) + 1.0);
        let x = (focus.x as f32 + outward.x).floor() as i32;
        let y = (focus.y as f32 + outward.y).floor() as i32;
        // A corner ray can round beside a one-cell corridor. Sample the four
        // surrounding cell centres so its known continuation is retained.
        [
            GridPos::new(x, y),
            GridPos::new(x + 1, y),
            GridPos::new(x, y + 1),
            GridPos::new(x + 1, y + 1),
        ]
        .into_iter()
        .any(|position| known(position) && !game.player_visibility().is_visible(position))
    };
    // The destination-color blend fades to memory where it exists and to black
    // elsewhere. Only visible fringe cells are submitted; actors stay above it.
    MASK.with(|slot| {
        let mut slot = slot.borrow_mut();
        let material = slot.get_or_insert_with(|| {
            load_material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("center", UniformType::Float2),
                        UniformDesc::new("radii", UniformType::Float2),
                        UniformDesc::new("memoryTint", UniformType::Float3),
                        UniformDesc::new("maskOrigin", UniformType::Float2),
                        UniformDesc::new("maskWorldSize", UniformType::Float2),
                        UniformDesc::new("maskTexel", UniformType::Float2),
                    ],
                    textures: vec!["visibilityMask".to_owned()],
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::DestinationColor),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .map_err(|error| {
                eprintln!("[VISION] Masque circulaire indisponible : {error}");
                error.to_string()
            })
        });
        let Ok(material) = material else {
            return;
        };
        material.set_uniform("center", [center.x, center.y]);
        material.set_uniform("radii", radii);
        material.set_uniform("memoryTint", MEMORY_TINT);
        let origin = camera.rect(GridPos::new(camera.first.x - 2, camera.first.y - 2));
        material.set_uniform("maskOrigin", [origin.x, origin.y]);
        material.set_uniform(
            "maskWorldSize",
            [
                f32::from(image.width) * camera.cell,
                f32::from(image.height) * camera.cell,
            ],
        );
        material.set_uniform(
            "maskTexel",
            [1.0 / f32::from(image.width), 1.0 / f32::from(image.height)],
        );
        material.set_texture("visibilityMask", texture);
        gl_use_material(material);
        let mut mesh = Mesh {
            vertices: Vec::with_capacity(256),
            indices: Vec::with_capacity(384),
            texture: None,
        };
        for row in 0..camera.rows {
            for column in 0..camera.columns {
                let at = GridPos::new(camera.first.x + column, camera.first.y + row);
                let x = column + 2;
                let y = row + 2;
                let memory_cell = !game.player_visibility().is_visible(at);
                if memory_cell
                    && image.bytes[(y as usize * usize::from(image.width) + x as usize) * 4 + 1]
                        == 0
                {
                    continue;
                }
                let rect = camera.rect(at);
                let farthest = vec2(
                    (center.x - rect.x)
                        .abs()
                        .max((center.x - rect.right()).abs()),
                    (center.y - rect.y)
                        .abs()
                        .max((center.y - rect.bottom()).abs()),
                );
                let channel = usize::from(memory_cell);
                let shadow_edge = (-1..=1).any(|dy| {
                    (-1..=1).any(|dx| {
                        image.bytes[((y + dy) as usize * usize::from(image.width)
                            + (x + dx) as usize)
                            * 4
                            + channel]
                            == 0
                    })
                });
                if shadow_edge || (!memory_cell && farthest.length() >= radii[0]) {
                    if mesh.vertices.len() + 4 > usize::from(u16::MAX) {
                        draw_mesh(&mesh);
                        mesh.vertices.clear();
                        mesh.indices.clear();
                    }
                    let start = mesh.vertices.len() as u16;
                    // Visible obstacle tops stay legible; the reconstructed shadow
                    // covers the ground behind them, not the wall's own top face.
                    let occluder = !memory_cell && game.map().blocks_vision(at);
                    for point in [
                        vec2(rect.x, rect.y),
                        vec2(rect.right(), rect.y),
                        vec2(rect.right(), rect.bottom()),
                        vec2(rect.x, rect.bottom()),
                    ] {
                        mesh.vertices.push(Vertex::new(
                            point.x,
                            point.y,
                            0.0,
                            0.0,
                            0.0,
                            Color::new(
                                if !memory_cell && remembered_at(point) {
                                    1.0
                                } else {
                                    0.0
                                },
                                if occluder { 1.0 } else { 0.0 },
                                if memory_cell { 1.0 } else { 0.0 },
                                1.0,
                            ),
                        ));
                    }
                    mesh.indices.extend_from_slice(&[
                        start,
                        start + 1,
                        start + 2,
                        start,
                        start + 2,
                        start + 3,
                    ]);
                }
            }
        }
        if !mesh.indices.is_empty() {
            draw_mesh(&mesh);
        }
        gl_use_default_material();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_field_ignores_hidden_changes_and_refreshes_after_opening_a_door() {
        let focus = GridPos::new(10, 12);
        let camera = GridCamera {
            first: GridPos::new(4, 6),
            columns: 15,
            rows: 15,
            origin: Vec2::ZERO,
            cell: 32.0,
        };
        let world = |door: DoorState, hidden: Terrain| {
            let mut map = Map::filled(24, 24, Terrain::Floor).unwrap();
            for y in 0..24 {
                map.set_terrain(GridPos::new(12, y), Terrain::Wall).unwrap();
            }
            map.set_terrain(GridPos::new(12, 12), Terrain::Door(door))
                .unwrap();
            map.set_terrain(GridPos::new(14, 12), hidden).unwrap();
            WorldState::single(GameState::new(map, focus, 141).unwrap())
        };
        let closed = world(DoorState::Closed, Terrain::Floor);
        let changed = world(DoorState::Closed, Terrain::Wall);
        let before = closed.recovery_snapshot_bytes().unwrap();
        let image = |game: &WorldState| {
            field_image(game, &camera, focus, &|p| {
                game.player_visibility().is_visible(p)
            })
        };
        assert_eq!(image(&closed).bytes, image(&changed).bytes);
        assert_eq!(closed.recovery_snapshot_bytes().unwrap(), before);
        let open = world(DoorState::Open, Terrain::Floor);
        assert!(open.player_visibility().is_visible(GridPos::new(14, 12)));
        assert!(!closed.player_visibility().is_visible(GridPos::new(14, 12)));
        assert_ne!(image(&closed).bytes, image(&open).bytes);
    }
    #[test]
    fn radial_fringe_fits_inside_visible_tile_union_at_every_angle_and_zoom() {
        for cell in [8.0, 16.0, 32.0, 48.0, 64.0] {
            let [inner, outer] = radii(8, cell);
            assert!(inner < outer);
            for angle in 0..3600 {
                let a = angle as f32 * std::f32::consts::TAU / 3600.0;
                let x = outer / cell * a.cos();
                let y = outer / cell * a.sin();
                let tile_x = (x + 0.5).floor() as i32;
                let tile_y = (y + 0.5).floor() as i32;
                assert!(tile_x * tile_x + tile_y * tile_y <= 64);
            }
        }
    }
}
