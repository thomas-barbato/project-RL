//! FX-FOOTPRINT-01: accepted animation geometry and preview transparency. No simulation mutations.
use macroquad::prelude::*;
use project_rl::{combat::AttackPreview, world::GridPos};
#[cfg(debug_assertions)]
use std::cell::Cell;
use std::rc::{Rc, Weak};
use std::{cell::RefCell, collections::HashMap};

type TextureKey = (&'static [u8], bool, bool);
thread_local! {
    // Keys compare the compressed content, not its address or file name.
    static SHARED_TEXTURES: RefCell<HashMap<TextureKey, Weak<Texture2D>>> = RefCell::new(HashMap::new());
}

/// Decode and upload only when this particular drawing is actually used.
/// Art banks can describe an entire skill family without loading every atlas.
pub(crate) struct LazyTexture {
    bytes: &'static [u8],
    filter: FilterMode,
    fitted: bool,
    value: std::cell::OnceCell<Rc<Texture2D>>,
}
impl LazyTexture {
    pub(crate) fn fitted(bytes: &'static [u8], filter: FilterMode) -> Self {
        Self::new(bytes, filter, true)
    }
    pub(crate) fn plain(bytes: &'static [u8], filter: FilterMode) -> Self {
        Self::new(bytes, filter, false)
    }
    fn new(bytes: &'static [u8], filter: FilterMode, fitted: bool) -> Self {
        let value = Self {
            bytes,
            filter,
            fitted,
            value: std::cell::OnceCell::new(),
        };
        #[cfg(debug_assertions)]
        if EAGER_ASSETS.get() {
            let _ = std::ops::Deref::deref(&value);
        }
        value
    }
}
impl std::ops::Deref for LazyTexture {
    type Target = Texture2D;
    fn deref(&self) -> &Texture2D {
        self.value.get_or_init(|| {
            let key = (self.bytes, self.filter == FilterMode::Linear, self.fitted);
            if let Some(texture) =
                SHARED_TEXTURES.with_borrow(|cache| cache.get(&key).and_then(Weak::upgrade))
            {
                return texture;
            }
            #[cfg(debug_assertions)]
            let started = std::time::Instant::now();
            let value = if self.fitted {
                texture(self.bytes, self.filter)
            } else {
                let value = crate::asset_loading::texture(self.bytes, Some(ImageFormat::Png));
                value.set_filter(self.filter);
                value
            };
            #[cfg(debug_assertions)]
            ASSET_LOADS.with_borrow_mut(|stats| {
                stats.0 += 1;
                stats.1 += self.bytes.len();
                stats.2 += started.elapsed().as_secs_f64() * 1000.;
            });
            let value = Rc::new(value);
            SHARED_TEXTURES.with_borrow_mut(|cache| {
                cache.retain(|_, texture| texture.strong_count() > 0);
                cache.insert(key, Rc::downgrade(&value));
            });
            value
        })
    }
}

#[cfg(debug_assertions)]
thread_local! {
    static EAGER_ASSETS: Cell<bool> = const { Cell::new(false) };
    static ASSET_LOADS: RefCell<(usize, usize, f64)> = const { RefCell::new((0, 0, 0.)) };
}
#[cfg(debug_assertions)]
pub(crate) fn diagnostic_asset_loading(eager: bool) {
    EAGER_ASSETS.set(eager);
}
#[cfg(debug_assertions)]
pub(crate) fn diagnostic_asset_stats() -> (usize, usize, f64) {
    ASSET_LOADS.with_borrow(|stats| *stats)
}

#[cfg(debug_assertions)]
thread_local! { static FRAME_TIME: Cell<Option<f64>> = const { Cell::new(None) }; }
pub fn presentation_time(now: f64) -> f64 {
    #[cfg(debug_assertions)]
    {
        return FRAME_TIME.with(Cell::get).unwrap_or(now);
    }
    #[cfg(not(debug_assertions))]
    {
        now
    }
}
#[cfg(debug_assertions)]
pub struct FrameGuard(Option<f64>);
#[cfg(debug_assertions)]
impl Drop for FrameGuard {
    fn drop(&mut self) {
        FRAME_TIME.with(|t| t.set(self.0));
    }
}
#[cfg(debug_assertions)]
pub fn frame_time(now: f64) -> FrameGuard {
    FrameGuard(FRAME_TIME.with(|t| t.replace(Some(now))))
}
pub fn enabled() -> bool {
    true
}

pub fn stroke(mut color: Color) -> Color {
    if enabled() {
        color.a *= 0.82;
    }
    color
}
pub fn underlay(mut color: Color) -> Color {
    if enabled() {
        color.a *= 0.45;
    }
    color
}

#[derive(Clone, Debug)]
pub struct PreparedCast {
    pub skill: String,
    pub origin: GridPos,
    pub target: GridPos,
    pub cells: Vec<GridPos>,
}
impl PreparedCast {
    pub fn new(skill: String, preview: AttackPreview) -> Self {
        Self {
            skill,
            origin: preview.origin(),
            target: preview.target(),
            cells: preview.cells().iter().map(|c| c.position).collect(),
        }
    }
    pub fn cells_for(
        &self,
        skill: &str,
        origin: Option<GridPos>,
        target: GridPos,
    ) -> Option<Vec<GridPos>> {
        (self
            .skill
            .eq_ignore_ascii_case(skill.trim_end_matches("-BLAST"))
            && origin.is_none_or(|origin| origin == self.origin)
            && self.target == target)
            .then(|| self.cells.clone())
    }
}

pub fn projectile(player: Vec2, target: Vec2, tile: f32) -> (Vec2, Vec2, Vec2) {
    let mut axis = (target - player).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1., 0.);
    }
    (player + axis * tile * 0.29, target, axis)
}

#[derive(Clone, Copy, Debug)]
pub struct Placement {
    pub center: Vec2,
    pub size: Vec2,
    pub angle: f32,
}
pub fn placement(
    cells: &[GridPos],
    rect: &impl Fn(GridPos) -> Option<Rect>,
    angle: f32,
) -> Option<Placement> {
    let (anchor, first) = cells.iter().find_map(|p| rect(*p).map(|r| (*p, r)))?;
    let axis = vec2(angle.cos(), angle.sin());
    let side = vec2(-axis.y, axis.x);
    let mut low = vec2(f32::INFINITY, f32::INFINITY);
    let mut high = vec2(f32::NEG_INFINITY, f32::NEG_INFINITY);
    for at in cells {
        // Include offscreen parts of the captured footprint in the layout.
        // Panning the camera must not resize an already cast animation.
        let r = Rect::new(
            first.x + (at.x - anchor.x) as f32 * first.w,
            first.y + (at.y - anchor.y) as f32 * first.h,
            first.w,
            first.h,
        );
        for p in [
            vec2(r.x, r.y),
            vec2(r.right(), r.y),
            vec2(r.x, r.bottom()),
            vec2(r.right(), r.bottom()),
        ] {
            let p = vec2(p.dot(axis), p.dot(side));
            low = low.min(p);
            high = high.max(p);
        }
    }
    let middle = (low + high) * 0.5;
    Some(Placement {
        center: axis * middle.x + side * middle.y,
        size: high - low,
        angle,
    })
}

#[derive(Clone, Copy)]
struct Atlas {
    side: f32,
    painted: Rect,
}
thread_local! { static ATLASES:RefCell<HashMap<macroquad::miniquad::TextureId,Atlas>>=RefCell::new(HashMap::new()); }
// Decode once on the CPU, upload once; fitting never reads a texture back mid-frame.
pub fn texture(bytes: &[u8], filter: FilterMode) -> Texture2D {
    let image =
        crate::asset_loading::image(bytes, Some(ImageFormat::Png)).expect("packaged animation PNG");
    let side = usize::from(image.height);
    let bounds_loading =
        project_rl::loading_profile::Span::caller("prepare_alpha_bounds", image.bytes.len());
    let mut low = (side, side);
    let mut high = (0, 0);
    let mut any = false;
    for y in 0..side {
        for x in 0..usize::from(image.width) {
            if image.bytes[(y * usize::from(image.width) + x) * 4 + 3] >= 16 {
                let x = x % side;
                low = (low.0.min(x), low.1.min(y));
                high = (high.0.max(x), high.1.max(y));
                any = true;
            }
        }
    }
    let painted = if any {
        Rect::new(
            low.0 as f32,
            low.1 as f32,
            (high.0 - low.0 + 1) as f32,
            (high.1 - low.1 + 1) as f32,
        )
    } else {
        Rect::new(0., 0., side as f32, side as f32)
    };
    drop(bounds_loading);
    let texture = crate::asset_loading::from_image(&image);
    texture.set_filter(filter);
    ATLASES.with_borrow_mut(|cache| {
        cache.insert(
            texture.raw_miniquad_id(),
            Atlas {
                side: side as f32,
                painted,
            },
        )
    });
    texture
}
fn atlas(texture: &Texture2D) -> Atlas {
    ATLASES.with_borrow(|cache| {
        *cache
            .get(&texture.raw_miniquad_id())
            .expect("registered animation atlas")
    })
}
pub(crate) fn atlas_bounds(texture: &Texture2D) -> (f32, Rect) {
    let atlas = atlas(texture);
    (atlas.side, atlas.painted)
}
pub fn fitted_sheet(
    texture: &Texture2D,
    frame: usize,
    cells: &[GridPos],
    rect: &impl Fn(GridPos) -> Option<Rect>,
    angle: f32,
    opacity: f32,
) {
    let Some(place) = placement(cells, rect, angle) else {
        return;
    };
    let atlas = atlas(texture);
    let scale = place.size.x.max(place.size.y) * 1.12 / atlas.painted.w.max(atlas.painted.h);
    let origin = place.center - atlas.painted.center() * scale;
    draw_texture_ex(
        texture,
        origin.x,
        origin.y,
        Color::new(1., 1., 1., opacity),
        DrawTextureParams {
            source: Some(Rect::new(
                frame as f32 * atlas.side,
                0.,
                atlas.side,
                atlas.side,
            )),
            dest_size: Some(vec2(atlas.side, atlas.side) * scale),
            rotation: place.angle,
            pivot: Some(place.center),
            ..Default::default()
        },
    );
}

pub fn clipped(bounds: Rect, parent: Rect, draw: impl FnOnce()) {
    let Some(r) = bounds.intersect(parent) else {
        return;
    };
    if r.w <= 0. || r.h <= 0. {
        return;
    }
    unsafe {
        macroquad::window::get_internal_gl().quad_gl.scissor(Some((
            r.x.ceil() as i32,
            r.y.ceil() as i32,
            r.w.floor() as i32,
            r.h.floor() as i32,
        )));
    }
    draw();
    unsafe {
        macroquad::window::get_internal_gl().quad_gl.scissor(Some((
            parent.x as i32,
            parent.y as i32,
            parent.w as i32,
            parent.h as i32,
        )));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn projectile_launch_and_heading_follow_all_eight_targets() {
        for (x, y) in [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, 1),
            (1, -1),
            (-1, -1),
        ] {
            let p = vec2(256., 256.);
            let target = p + vec2(x as f32, y as f32) * 128.;
            let (from, hit, axis) = projectile(p, target, 64.);
            assert_eq!(hit, target);
            assert!((from - p).dot(axis) > 0.);
            assert!((target - from).normalize().distance(axis) < 0.0001);
            let degree_angle = axis.y.atan2(axis.x).to_degrees();
            assert!(
                vec2(
                    degree_angle.to_radians().cos(),
                    degree_angle.to_radians().sin()
                )
                .distance(axis)
                    < 0.0001
            );
        }
    }
    #[test]
    fn fitting_keeps_the_cast_size_when_the_camera_crops_a_larger_area() {
        let cells: Vec<_> = (0..7)
            .flat_map(|y| (0..7).map(move |x| GridPos::new(x, y)))
            .collect();
        let rect = |p: GridPos| Some(Rect::new(p.x as f32 * 64., p.y as f32 * 64., 64., 64.));
        let all = placement(&cells, &rect, 0.).unwrap();
        let cropped = placement(&cells, &|p| if p.x < 2 { rect(p) } else { None }, 0.).unwrap();
        assert_eq!(all.size, vec2(448., 448.));
        assert_eq!(all.size, cropped.size);
        assert_eq!(all.center, cropped.center);
    }
    #[test]
    fn a_vertical_wave_uses_the_same_cells_with_swapped_local_axes() {
        let cells = (0..5).map(|y| GridPos::new(2, y)).collect::<Vec<_>>();
        let p = placement(
            &cells,
            &|p| Some(Rect::new(p.x as f32 * 64., p.y as f32 * 64., 64., 64.)),
            std::f32::consts::FRAC_PI_2,
        )
        .unwrap();
        assert!(p.size.distance(vec2(320., 64.)) < 0.001);
        assert!(p.center.distance(vec2(160., 160.)) < 0.001);
    }
    #[test]
    fn captured_cells_cannot_leak_to_a_different_cast_or_another_shatter_source() {
        let cast = PreparedCast {
            skill: "fe05".into(),
            origin: GridPos::new(2, 3),
            target: GridPos::new(5, 3),
            cells: vec![GridPos::new(4, 3), GridPos::new(5, 3)],
        };
        assert_eq!(
            cast.cells_for("FE05", Some(cast.origin), cast.target),
            Some(cast.cells.clone())
        );
        assert!(cast.cells_for("FR02", None, cast.target).is_none());
        assert!(
            cast.cells_for("FE05", Some(GridPos::new(1, 1)), cast.target)
                .is_none()
        );
        assert!(cast.cells_for("FE05", None, GridPos::new(4, 3)).is_none());
    }
}
