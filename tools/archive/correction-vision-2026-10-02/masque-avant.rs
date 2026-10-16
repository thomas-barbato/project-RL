//! Analytic presentation of the perception radius. Simulation visibility and
//! the remembered terrain catalogue remain the sole information gates.
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use std::cell::RefCell;

const VERTEX: &str = r#"#version 100
attribute vec3 position;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 point;
void main() {
    point = position.xy;
    gl_Position = Projection * Model * vec4(position, 1.0);
}"#;

const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
uniform vec2 center;
uniform vec2 radii;
void main() {
    float fog = smoothstep(radii.x, radii.y, distance(point, center));
    gl_FragColor = vec4(0.0, 0.0, 0.0, fog);
}"#;

thread_local! {
    static MASK: RefCell<Option<Result<Material, String>>> = const { RefCell::new(None) };
}

/// The circular edge lies inside the union of visible tile squares, including
/// at oblique angles; a one-cell feather removes the old stair-step silhouette.
fn radii(radius: u16, cell: f32) -> [f32; 2] {
    let outer = (f32::from(radius) - std::f32::consts::FRAC_1_SQRT_2).max(0.5) * cell;
    [(outer - cell * 0.8).max(0.0), outer]
}

pub(super) fn draw(game: &WorldState, camera: &GridCamera) {
    let rules = game.rules().player_field_of_view;
    if rules.distance_metric != project_rl::world::DistanceMetric::Euclidean || rules.radius == 0 {
        return;
    }
    let Some(focus) = game.player_position() else {
        return;
    };
    let center = camera.rect(focus).center();
    let radii = radii(rules.radius, camera.cell);
    // Only the radial fringe is painted: explored memory farther away keeps
    // its previous dim rendering. The mask is behind actors and every effect.
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
                    ],
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
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
        gl_use_material(material);
        for at in game
            .player_visibility()
            .visible_positions()
            .filter(|at| camera.contains(*at))
        {
            let rect = camera.rect(at);
            let farthest = vec2(
                (center.x - rect.x)
                    .abs()
                    .max((center.x - rect.right()).abs()),
                (center.y - rect.y)
                    .abs()
                    .max((center.y - rect.bottom()).abs()),
            );
            if farthest.length() >= radii[0] {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
            }
        }
        gl_use_default_material();
    });
}

#[cfg(test)]
mod tests {
    use super::*;
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
