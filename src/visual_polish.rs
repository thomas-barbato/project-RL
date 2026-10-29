//! POLISH01-02: approved world finish and B01 contact; presentation only.
use macroquad::{miniquad as mq, prelude::*};
use project_rl::{
    game::GameState,
    world::{GridPos, has_line_of_sight},
};
use std::cell::RefCell;

#[cfg(debug_assertions)]
thread_local! { static OVERRIDE: std::cell::Cell<Option<bool>> = const { std::cell::Cell::new(None) }; }
pub fn enabled() -> bool {
    #[cfg(debug_assertions)]
    if let Some(value) = OVERRIDE.get() {
        return value;
    }
    true
}
#[cfg(debug_assertions)]
pub struct ReviewGuard(Option<bool>);
#[cfg(debug_assertions)]
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        OVERRIDE.set(self.0);
        crate::world_readability::restore();
    }
}
#[cfg(debug_assertions)]
pub fn review(enabled: bool) -> ReviewGuard {
    ReviewGuard(OVERRIDE.replace(Some(enabled)))
}

// Shared by every scenery material. Lighting compensation applies the accepted
// curve to the lit surface before the ordinary final world-light composite.
pub const SURFACE_GLSL: &str = r#"
uniform float PolishOn;uniform float PolishLit;uniform vec2 PolishWorldSize;
uniform vec3 PolishAmbient;uniform float PolishLightOn;uniform vec2 PolishTexel;
uniform sampler2D PolishLight;
float polish_lum(vec3 c){return dot(c,vec3(0.2126,0.7152,0.0722));}
vec3 polish_surface(vec3 ink,vec2 p){
 vec3 light=vec3(1.0);
 if(PolishLit>0.5){
  vec2 w=vec2(gl_FragCoord.x/PolishWorldSize.x,1.0-gl_FragCoord.y/PolishWorldSize.y);
  vec3 irradiance=texture2D(PolishLight,w).rgb*2.0;
  float surface=smoothstep(0.07,0.17,max(ink.r,max(ink.g,ink.b)));
  light=mix(PolishAmbient,irradiance,PolishLightOn*surface);
  light=mix(light,vec3(polish_lum(light)),WorldGray);
 }
 vec3 lit=ink*light;
 if(PolishTexel.x>0.0 && PolishTexel.y>0.0){
  vec4 c=texture2D(Texture,p);
  vec4 n1=texture2D(Texture,p+vec2(PolishTexel.x,0.0));
  vec4 n2=texture2D(Texture,p-vec2(PolishTexel.x,0.0));
  vec4 n3=texture2D(Texture,p+vec2(0.0,PolishTexel.y));
  vec4 n4=texture2D(Texture,p-vec2(0.0,PolishTexel.y));
  float l=polish_lum(c.rgb);
  float v=max(max(abs(polish_lum(n1.rgb)-l),abs(polish_lum(n2.rgb)-l)),max(abs(polish_lum(n3.rgb)-l),abs(polish_lum(n4.rgb)-l)));
  float opaque=step(0.99,min(c.a,min(min(n1.a,n2.a),min(n3.a,n4.a))));
  float quiet=(1.0-smoothstep(0.025,0.065,v))*0.26*opaque;
  vec3 average=(c.rgb*4.0+n1.rgb+n2.rgb+n3.rgb+n4.rgb)/8.0;
  lit*=mix(vec3(1.0),average/max(c.rgb,vec3(0.001)),quiet);
 }
 // Generated floors are already dark and have no later illumination pass.
 // Keep their low-value detail instead of crushing it around the street pivot.
 float pivot=PolishLit>0.5?0.17:min(0.17,polish_lum(lit));
 vec3 graded=max(vec3(0.0),(lit-pivot)*1.24+pivot);
 graded=mix(vec3(polish_lum(graded)),graded,1.09);
 float shade=1.0-smoothstep(0.08,0.35,polish_lum(lit));
 graded+=vec3(-0.006,0.003,0.017)*shade*smoothstep(0.005,0.08,polish_lum(lit));
 float warm=smoothstep(0.015,0.28,light.r-light.b);
 graded*=mix(vec3(1.0),vec3(1.17,1.09,1.015),warm);
 return clamp(graded/max(light,vec3(0.001)),0.0,1.0);
}
"#;

struct ContactArt {
    texture: Texture2D,
    material: Material,
}
thread_local! { static CONTACT: RefCell<Option<ContactArt>> = const { RefCell::new(None) }; }

pub fn contact_age(now: f64, started: f64) -> f32 {
    (now - started - 0.22 * crate::animation_policy::duration_scale("B01")) as f32
}
fn lifetime(age: f32, reduced: bool) -> Option<(f32, f32)> {
    (!reduced && (0.0..0.24).contains(&age)).then(|| {
        let flash = if age < 0.16 {
            (1. - age / 0.16).powi(2)
        } else {
            0.
        };
        (flash, (1. - age / 0.24).powi(2))
    })
}
fn light_cell_visible(game: &GameState, origin: GridPos, at: GridPos) -> bool {
    game.map().is_walkable(at)
        && game.player_visibility().is_visible(at)
        && has_line_of_sight(game.map(), origin, at, true)
}

pub fn draw_contact(
    game: &GameState,
    origin: GridPos,
    destination: GridPos,
    age: f32,
    reduced: bool,
    rect: &impl Fn(GridPos) -> Option<Rect>,
) {
    if !enabled() {
        return;
    }
    let Some((flash, life)) = lifetime(age, reduced) else {
        return;
    };
    if !game.player_visibility().is_visible(destination) {
        return;
    }
    let (Some(from), Some(to)) = (rect(origin), rect(destination)) else {
        return;
    };
    let direction = (to.center() - from.center()).normalize_or_zero();
    if direction == Vec2::ZERO {
        return;
    }
    let normal = vec2(-direction.y, direction.x);
    let scale = to.w / 64.;
    let center = to.center() - direction * 17. * scale + normal * 8. * scale;
    let _color = crate::world_readability::color();
    if flash > 0. {
        CONTACT.with_borrow_mut(|slot| {
            let art = slot.get_or_insert_with(|| {
                let mut image = Image::gen_image_color(128, 64, BLANK);
                for y in 0..64 {
                    for x in 0..128 {
                        let dx = (x as f32 + 0.5) / 128. * 6. - 3.;
                        let dy = (y as f32 + 0.5) / 64. * 6. - 3.;
                        let a = ((-dx * dx - dy * dy).exp() * 255.).round() as u8;
                        image.set_pixel(x, y, Color::from_rgba(255, 255, 255, a));
                    }
                }
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Linear);
                let material = load_material(
                    ShaderSource::Glsl {
                        vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
                        fragment: r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
void main(){gl_FragColor=texture2D(Texture,uv)*tint;}"#,
                    },
                    MaterialParams {
                        pipeline_params: mq::PipelineParams {
                            color_blend: Some(mq::BlendState::new(
                                mq::Equation::Add,
                                mq::BlendFactor::Value(mq::BlendValue::SourceAlpha),
                                mq::BlendFactor::One,
                            )),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                )
                .expect("approved contact-light shader");
                ContactArt { texture, material }
            });
            let size = vec2(396., 174.) * scale;
            let bounds = Rect::new(
                center.x - size.x / 2.,
                center.y - size.y / 2.,
                size.x,
                size.y,
            );
            gl_use_material(&art.material);
            // Clip the light to visible, reachable floor: never light a wall,
            // unexplored cell or the inside of an adjoining closed room.
            for dy in -2..=2 {
                for dx in -4..=4 {
                    let at = GridPos::new(destination.x + dx, destination.y + dy);
                    if !light_cell_visible(game, destination, at) {
                        continue;
                    }
                    let Some(cell) = rect(at) else {
                        continue;
                    };
                    let left = bounds.x.max(cell.x);
                    let top = bounds.y.max(cell.y);
                    let right = bounds.right().min(cell.right());
                    let bottom = bounds.bottom().min(cell.bottom());
                    if right <= left || bottom <= top {
                        continue;
                    }
                    draw_texture_ex(
                        &art.texture,
                        left,
                        top,
                        Color::new(0.17, 0.075, 0.017, flash),
                        DrawTextureParams {
                            source: Some(Rect::new(
                                (left - bounds.x) / size.x * 128.,
                                (top - bounds.y) / size.y * 64.,
                                (right - left) / size.x * 128.,
                                (bottom - top) / size.y * 64.,
                            )),
                            dest_size: Some(vec2(right - left, bottom - top)),
                            ..Default::default()
                        },
                    );
                }
            }
            gl_use_default_material();
        });
    }
    for (i, angle) in [-2.35_f32, -1.88, -1.25, -0.54, 0.28, 0.83]
        .iter()
        .enumerate()
    {
        let d = direction * angle.cos() + normal * angle.sin();
        let speed = 52. + i as f32 * 8.;
        let tip = center + d * (4. + speed * age) * scale + vec2(0., age * age * 70. * scale);
        let tail = tip - d * (2. + life * 4.) * scale;
        draw_line(
            tip.x,
            tip.y,
            tail.x,
            tail.y,
            scale,
            Color::new(1., 0.66, 0.27, life),
        );
    }
    if age < 0.07 {
        let a = center - direction * 4. * scale + normal * 2. * scale;
        let b = center + direction * 5. * scale - normal * 3. * scale;
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            2. * scale,
            Color::new(1., 0.90, 0.65, life),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn contact_uses_real_presentation_time() {
        let start = 3.;
        let contact = start + 0.22 * crate::animation_policy::duration_scale("B01");
        assert!(contact_age(contact, start).abs() < 0.00001);
        assert!(lifetime(contact_age(contact - 0.001, start), false).is_none());
        assert_eq!(lifetime(0., false), Some((1., 1.)));
        assert_eq!(lifetime(0.16, false).unwrap().0, 0.);
        assert!(lifetime(0.24, false).is_none());
    }
    #[test]
    fn reduced_motion_suppresses_every_new_contact_component() {
        for age in [0., 0.02, 0.1, 0.159, 0.2, 0.239] {
            assert!(lifetime(age, true).is_none());
        }
    }
    #[test]
    fn contact_light_cannot_reveal_a_wall_or_closed_room() {
        use project_rl::world::{Map, Terrain};
        let mut map = Map::filled(10, 7, Terrain::Floor).unwrap();
        map.set_terrain(GridPos::new(4, 3), Terrain::Wall).unwrap();
        let game = GameState::new(map, GridPos::new(2, 3), 2701).unwrap();
        let origin = GridPos::new(3, 3);
        assert!(light_cell_visible(&game, origin, origin));
        assert!(!light_cell_visible(&game, origin, GridPos::new(4, 3)));
        assert!(!light_cell_visible(&game, origin, GridPos::new(5, 3)));
        assert!(!light_cell_visible(&game, origin, GridPos::new(-1, 3)));
    }
}
