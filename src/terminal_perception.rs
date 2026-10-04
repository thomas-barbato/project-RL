//! Circular, visibility-clipped tactical ranges and bare perception warnings.
use super::*;

const SEGMENTS: usize = 96;

pub(super) fn draw_fields(
    game: &WorldState,
    camera: &GridCamera,
    vision: &[ActorObservationField],
    sounds: &[TerminalSoundField],
) {
    for field in vision {
        draw_field(
            game,
            camera,
            field.origin(),
            field.radius(),
            true,
            field.positions(),
            Color::from_rgba(55, 208, 174, 255),
        );
    }
    for field in sounds {
        draw_field(
            game,
            camera,
            field.origin,
            field.radius,
            field.circular,
            field.positions.iter().copied(),
            Color::from_rgba(240, 184, 79, 255),
        );
    }
}

fn display_radius(radius: u16, circular: bool) -> f32 {
    // Every genuinely reachable cell centre remains inside the graphic,
    // including radius-one footsteps. Legacy saves retain their sound rules.
    (f32::from(radius)
        * if circular {
            1.0
        } else {
            std::f32::consts::SQRT_2
        })
    .max(0.45)
}

fn circle_points(
    center: Vec2,
    radius: f32,
    boundary_centres: impl Iterator<Item = Vec2>,
) -> Vec<Vec2> {
    let mut angles: Vec<_> = (0..SEGMENTS)
        .map(|i| i as f32 * std::f32::consts::TAU / SEGMENTS as f32)
        .collect();
    let inner = radius * (std::f32::consts::PI / SEGMENTS as f32).cos();
    // Add exact grid boundary directions so an inscribed polygon cannot omit
    // a reachable centre (e.g. the 3/4/5 triangle). No radius inflation.
    for p in boundary_centres {
        let delta = p - center;
        if delta.length_squared() >= inner * inner && delta.length_squared() <= radius * radius {
            angles.push(delta.y.atan2(delta.x).rem_euclid(std::f32::consts::TAU));
        }
    }
    angles.sort_by(f32::total_cmp);
    angles.dedup();
    angles
        .into_iter()
        .map(|angle| center + vec2(angle.cos(), angle.sin()) * radius)
        .collect()
}

fn radial_rim_cell(at: GridPos, origin: GridPos, radius: u16, allowed: &BTreeSet<GridPos>) -> bool {
    let dx = i64::from(at.x) - i64::from(origin.x);
    let dy = i64::from(at.y) - i64::from(origin.y);
    dx * dx + dy * dy > i64::from(radius).pow(2)
        && at.cardinal_neighbors().iter().any(|p| allowed.contains(p))
}

fn draw_field(
    game: &WorldState,
    camera: &GridCamera,
    origin: GridPos,
    radius: u16,
    circular: bool,
    positions: impl Iterator<Item = GridPos>,
    color: Color,
) {
    let visible = game.player_visibility();
    if !visible.is_visible(origin) {
        return;
    }
    let allowed: BTreeSet<_> = positions
        .filter(|&at| visible.is_visible(at) && game.map().is_walkable(at))
        .collect();
    let center = camera.rect(origin).center();
    let grid_radius = radius;
    let radius = display_radius(radius, circular) * camera.cell;
    let circle = circle_points(
        center,
        radius,
        allowed.iter().map(|&at| camera.rect(at).center()),
    );
    let mut drawable = allowed.clone();
    if circular {
        // Continue only the curved sub-cell rim across a radial boundary.
        // These outer centres stay outside the exact circle. Cells blocked
        // inside its radius, walls and hidden terrain never enter this mask.
        drawable.extend(visible.visible_positions().filter(|&at| {
            game.map().is_walkable(at) && radial_rim_cell(at, origin, grid_radius, &allowed)
        }));
    }
    let fill = Color::new(color.r, color.g, color.b, 0.14);
    let mut polygon = Vec::with_capacity(SEGMENTS + 4);
    let mut scratch = Vec::with_capacity(SEGMENTS + 4);
    for &at in drawable.iter().filter(|&&at| camera.contains(at)) {
        let rect = camera.rect(at);
        let corners = [
            vec2(rect.x, rect.y),
            vec2(rect.right(), rect.y),
            vec2(rect.right(), rect.bottom()),
            vec2(rect.x, rect.bottom()),
        ];
        if corners
            .iter()
            .all(|&p| p.distance_squared(center) <= radius * radius)
        {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
            continue;
        }
        polygon.clone_from(&circle);
        clip_to_rect(&mut polygon, &mut scratch, rect);
        if polygon.len() >= 3 {
            for i in 1..polygon.len() - 1 {
                draw_triangle(polygon[0], polygon[i], polygon[i + 1], fill);
            }
        }
    }
    let at_point = |p: Vec2| {
        GridPos::new(
            camera.first.x + ((p.x - camera.origin.x) / camera.cell).floor() as i32,
            camera.first.y + ((p.y - camera.origin.y) / camera.cell).floor() as i32,
        )
    };
    for i in 0..circle.len() {
        let a = circle[i];
        let b = circle[(i + 1) % circle.len()];
        let at_a = at_point(a);
        let at_b = at_point(b);
        if drawable.contains(&at_a)
            && drawable.contains(&at_b)
            && camera.contains(at_a)
            && camera.contains(at_b)
        {
            draw_line(
                a.x,
                a.y,
                b.x,
                b.y,
                1.3,
                Color::new(color.r, color.g, color.b, 0.7),
            );
        }
    }
}

fn clip_to_rect(polygon: &mut Vec<Vec2>, scratch: &mut Vec<Vec2>, rect: Rect) {
    for (axis, boundary, lower) in [
        (0, rect.x, true),
        (0, rect.right(), false),
        (1, rect.y, true),
        (1, rect.bottom(), false),
    ] {
        if polygon.is_empty() {
            return;
        }
        scratch.clear();
        let coordinate = |p: Vec2| if axis == 0 { p.x } else { p.y };
        let inside = |p: Vec2| {
            if lower {
                coordinate(p) >= boundary
            } else {
                coordinate(p) <= boundary
            }
        };
        let mut previous = *polygon.last().unwrap();
        for &current in polygon.iter() {
            if inside(previous) != inside(current) {
                let t = (boundary - coordinate(previous))
                    / (coordinate(current) - coordinate(previous));
                scratch.push(previous + (current - previous) * t);
            }
            if inside(current) {
                scratch.push(current);
            }
            previous = current;
        }
        std::mem::swap(polygon, scratch);
    }
}

pub(super) fn draw_signal(rect: Rect, seen: bool, heard: bool, badge_rows: usize) {
    if !seen && !heard {
        return;
    }
    let scale = (rect.w / 32.0).clamp(0.75, 1.25);
    let center = vec2(
        rect.x + rect.w * 0.5,
        rect.y - 13.0 * scale - badge_rows as f32 * 20.0,
    );
    let red = Color::from_rgba(255, 58, 73, 255);
    let stroke = 2.6 * scale;
    if seen {
        draw_ellipse_lines(
            center.x,
            center.y,
            12.0 * scale,
            7.0 * scale,
            0.0,
            stroke,
            red,
        );
        draw_circle(center.x, center.y, 3.5 * scale, red);
    } else {
        draw_ellipse_lines(
            center.x + scale,
            center.y - scale,
            7.0 * scale,
            10.0 * scale,
            0.0,
            stroke,
            red,
        );
        for (a, b) in [
            (vec2(2.0, -5.0), vec2(-2.0, 1.0)),
            (vec2(-2.0, 1.0), vec2(2.0, 5.0)),
            (vec2(-11.0, -4.0), vec2(-11.0, 4.0)),
        ] {
            let a = center + a * scale;
            let b = center + b * scale;
            draw_line(a.x, a.y, b.x, b.y, stroke, red);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn circular_ranges_cover_reachable_centres_and_clip_without_spilling() {
        for radius in 0..=8u16 {
            let r = display_radius(radius, true);
            let positions: BTreeSet<_> = (-i32::from(radius)..=i32::from(radius))
                .flat_map(|y| {
                    (-i32::from(radius)..=i32::from(radius)).filter_map(move |x| {
                        (x * x + y * y <= i32::from(radius).pow(2)).then_some(GridPos::new(x, y))
                    })
                })
                .collect();
            let circle = circle_points(
                Vec2::ZERO,
                r,
                positions.iter().map(|p| vec2(p.x as f32, p.y as f32)),
            );
            for y in -i32::from(radius)..=i32::from(radius) {
                for x in -i32::from(radius)..=i32::from(radius) {
                    if x * x + y * y > i32::from(radius).pow(2) {
                        continue;
                    }
                    let centre = vec2(x as f32, y as f32);
                    let rect = Rect::new(centre.x - 0.5, centre.y - 0.5, 1.0, 1.0);
                    let mut polygon = circle.clone();
                    clip_to_rect(&mut polygon, &mut Vec::new(), rect);
                    assert!(polygon.len() >= 3);
                    assert!(polygon.iter().all(|p| p.x >= rect.x - 0.00001
                        && p.x <= rect.right() + 0.00001
                        && p.y >= rect.y - 0.00001
                        && p.y <= rect.bottom() + 0.00001));
                    assert!(
                        polygon
                            .iter()
                            .zip(polygon.iter().cycle().skip(1))
                            .all(|(&a, &b)| (b - a).perp_dot(centre - a) >= -0.00001)
                    );
                }
            }
            for y in -i32::from(radius) - 1..=i32::from(radius) + 1 {
                for x in -i32::from(radius) - 1..=i32::from(radius) + 1 {
                    let at = GridPos::new(x, y);
                    if positions.contains(&at) {
                        continue;
                    }
                    let p = vec2(x as f32, y as f32);
                    assert!(
                        circle
                            .iter()
                            .zip(circle.iter().cycle().skip(1))
                            .any(|(&a, &b)| (b - a).perp_dot(p - a) < -0.00001),
                        "An unreachable centre was painted: {at:?} radius {radius}"
                    );
                }
            }
            let mut blocked = positions.clone();
            blocked.remove(&GridPos::new(0, 0));
            assert!(!radial_rim_cell(
                GridPos::new(0, 0),
                GridPos::new(0, 0),
                radius,
                &blocked
            ));
            assert!(display_radius(radius, false) >= f32::from(radius) * std::f32::consts::SQRT_2);
        }
    }
}
