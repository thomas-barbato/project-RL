//! Whole painted frames, clipped only by visible floor and physical occlusion.
use crate::{animation_policy as policy, effect_geometry};
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use project_rl::world::{GridPos, Map, has_line_of_sight};
use std::cell::RefCell;

struct Mask {
    cells: Vec<GridPos>,
    bounds: (i32, i32, u16, u16),
    texture: Texture2D,
}
struct Art {
    material: Material,
    masks: Vec<Mask>,
}
thread_local! { static OCCULT: std::cell::Cell<bool> = const {std::cell::Cell::new(false)}; static ART: RefCell<Option<Art>> = const { RefCell::new(None) }; }
impl Art {
    fn load() -> Self {
        let material = policy::material(ShaderSource::Glsl {
            vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
void main(){point=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
            fragment: r#"#version 100
precision highp float;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D floorMask;uniform vec2 gridOrigin;uniform vec2 gridSize;uniform vec2 tileSize;uniform float ice;uniform float occult;
void main(){vec2 p=(point-gridOrigin)/tileSize;
if(p.x<0.0||p.y<0.0||p.x>=gridSize.x||p.y>=gridSize.y)discard;
if(texture2D(floorMask,(floor(p)+vec2(0.5))/gridSize).r<0.5)discard;
vec4 c=texture2D(Texture,uv);
if(ice>0.5){float light=dot(c.rgb,vec3(0.2126,0.7152,0.0722));float frost=smoothstep(0.40,0.82,light);
c.rgb=mix(min(c.rgb*1.18,vec3(1.0)),vec3(0.80,0.94,1.0),frost*0.72);c.a*=mix(0.76,1.0,frost);}
if(occult>0.5){float violet=smoothstep(0.007,0.055,c.b-c.g)*step(c.r*0.82,c.b);float visible=smoothstep(0.022,0.12,max(c.r,c.b));float peak=smoothstep(0.35,0.75,max(c.r,c.b));c.rgb=mix(c.rgb,mix(vec3(0.65,0.15,1.0),vec3(0.80,0.30,1.0),peak),violet*visible*0.96);}
gl_FragColor=c*tint;}"#,
        }, MaterialParams { uniforms: vec![UniformDesc::new("gridOrigin",UniformType::Float2),UniformDesc::new("gridSize",UniformType::Float2),UniformDesc::new("tileSize",UniformType::Float2),UniformDesc::new("ice",UniformType::Float1),UniformDesc::new("occult",UniformType::Float1)], textures:vec!["floorMask".into()],pipeline_params:policy::blend(),..Default::default() }).expect("natural effect material");
        Self {
            material,
            masks: vec![],
        }
    }
}

pub fn anchor(
    center: GridPos,
    radius: u16,
    rect: &impl Fn(GridPos) -> Option<Rect>,
) -> Option<Rect> {
    let span = i32::from(radius) + 2;
    let (at, r) = std::iter::once(center)
        .chain(
            (center.y - span..=center.y + span)
                .flat_map(|y| (center.x - span..=center.x + span).map(move |x| GridPos::new(x, y))),
        )
        .find_map(|at| rect(at).map(|r| (at, r)))?;
    Some(Rect::new(
        r.x + (center.x - at.x) as f32 * r.w,
        r.y + (center.y - at.y) as f32 * r.h,
        r.w,
        r.h,
    ))
}

/// A stable nominal diameter: map edges and camera movement never resize a cast.
pub fn sheet(
    texture: &Texture2D,
    frame: usize,
    center: GridPos,
    radius: u16,
    opacity: f32,
    ice: bool,
    map: &Map,
    rect: &impl Fn(GridPos) -> Option<Rect>,
) {
    let Some(tile) = anchor(center, radius, rect) else {
        return;
    };
    let diameter = (f32::from(radius) * 2. + 1.) * tile.w * if ice { 1.10 } else { 1.12 };
    let (side, painted) = effect_geometry::atlas_bounds(texture);
    let scale = diameter / if ice { side } else { painted.w.max(painted.h) };
    let at = tile.center()
        - if ice {
            vec2(side, side) * 0.5
        } else {
            painted.center()
        } * scale;
    let dest = Rect::new(at.x, at.y, side * scale, side * scale);
    draw(
        texture,
        Rect::new(frame as f32 * side, 0., side, side),
        dest,
        0.,
        opacity,
        ice,
        center,
        tile,
        map,
        rect,
    );
}

pub fn draw(
    texture: &Texture2D,
    source: Rect,
    dest: Rect,
    rotation: f32,
    opacity: f32,
    ice: bool,
    origin: GridPos,
    tile: Rect,
    map: &Map,
    rect: &impl Fn(GridPos) -> Option<Rect>,
) {
    let extent = dest.size().length() * 0.5;
    let bounds = Rect::new(
        dest.center().x - extent,
        dest.center().y - extent,
        extent * 2.,
        extent * 2.,
    );
    on_floor(bounds, origin, tile, map, rect, ice, || {
        draw_texture_ex(
            texture,
            dest.x,
            dest.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(dest.size()),
                rotation,
                pivot: Some(dest.center()),
                ..Default::default()
            },
        )
    });
}

pub fn on_floor(
    bounds: Rect,
    origin: GridPos,
    tile: Rect,
    map: &Map,
    rect: &impl Fn(GridPos) -> Option<Rect>,
    ice: bool,
    draw: impl FnOnce(),
) {
    let min_x = (origin.x + ((bounds.x - tile.x) / tile.w).floor() as i32 - 1).max(0);
    let min_y = (origin.y + ((bounds.y - tile.y) / tile.h).floor() as i32 - 1).max(0);
    let max_x = (origin.x + ((bounds.right() - tile.x) / tile.w).ceil() as i32 + 1)
        .min(map.width() as i32 - 1);
    let max_y = (origin.y + ((bounds.bottom() - tile.y) / tile.h).ceil() as i32 + 1)
        .min(map.height() as i32 - 1);
    if max_x < min_x || max_y < min_y {
        return;
    }
    let bounds = (
        min_x,
        min_y,
        (max_x - min_x + 1) as u16,
        (max_y - min_y + 1) as u16,
    );
    let cells: Vec<_> = (min_y..=max_y)
        .flat_map(|y| (min_x..=max_x).map(move |x| GridPos::new(x, y)))
        .filter(|at| {
            rect(*at).is_some() && map.is_walkable(*at) && has_line_of_sight(map, origin, *at, true)
        })
        .collect();
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        let index = if let Some(i) = art
            .masks
            .iter()
            .position(|m| m.bounds == bounds && m.cells == cells)
        {
            i
        } else {
            let mut image = Image::gen_image_color(bounds.2, bounds.3, BLACK);
            for p in &cells {
                image.set_pixel((p.x - min_x) as u32, (p.y - min_y) as u32, WHITE);
            }
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            if art.masks.len() >= 16 {
                art.masks.remove(0);
            }
            art.masks.push(Mask {
                cells,
                bounds,
                texture,
            });
            art.masks.len() - 1
        };
        art.material
            .set_texture("floorMask", art.masks[index].texture.clone());
        art.material.set_uniform(
            "gridOrigin",
            (
                tile.x + (min_x - origin.x) as f32 * tile.w,
                tile.y + (min_y - origin.y) as f32 * tile.h,
            ),
        );
        art.material
            .set_uniform("gridSize", (bounds.2 as f32, bounds.3 as f32));
        art.material.set_uniform("tileSize", (tile.w, tile.h));
        art.material
            .set_uniform("ice", if ice { 1f32 } else { 0f32 });
        art.material
            .set_uniform("occult", if OCCULT.get() { 1f32 } else { 0f32 });
        policy::use_material(&art.material);
        draw();
        policy::restore_material();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_partly_offscreen_effect_keeps_its_center_and_scale() {
        let center = GridPos::new(4, 2);
        let full = |p: GridPos| {
            Some(Rect::new(
                19. + p.x as f32 * 64.,
                37. + p.y as f32 * 64.,
                64.,
                64.,
            ))
        };
        let clipped = |p: GridPos| if p.x < 4 { full(p) } else { None };
        assert_eq!(anchor(center, 2, &full), anchor(center, 2, &clipped));
    }
}

pub fn occult<T>(draw: impl FnOnce() -> T) -> T {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            OCCULT.set(self.0);
        }
    }
    let _guard = Guard(OCCULT.replace(true));
    draw()
}
