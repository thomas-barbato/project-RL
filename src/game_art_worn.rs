//! Native fixed outfit + sleeves + actual weapon family + fingers.
//! Mirroring exchanges held layers only; helmet and clothing stay stable.
use super::*;
use std::collections::BTreeMap;
#[path = "game_art_agent_rigs.rs"]
mod agent_rigs;
#[path = "game_art_finish_worn.rs"]
mod finish_worn;
#[path = "game_art_flat.rs"]
mod flat_agents;
#[path = "game_art_held_geometry.rs"]
mod held_geometry;
#[cfg(all(debug_assertions, test))]
pub(super) fn vertical_review_weapon(
    model: u16,
    left: bool,
    color: EquipmentPalette,
) -> Option<(Image, [f32; 2], Option<[f32; 2]>)> {
    finish_worn::vertical_review_weapon(model, left, color)
}

#[cfg(debug_assertions)]
pub(super) fn vertical_review_composition(
    source: &Image,
    visual: Visual,
    palms: [[f32; 2]; 2],
    back: bool,
) -> Option<Image> {
    if let Visual::Player {
        weapon,
        hands,
        palettes,
        ..
    } = visual
    {
        finish_worn::vertical_review_composition(
            source,
            palms,
            back,
            selection(weapon, hands),
            palettes,
        )
    } else {
        Some(source.clone())
    }
}
#[cfg(debug_assertions)]
pub(super) fn vertical_review_standing(visual: Visual) -> Option<Image> {
    if let Visual::Player {
        body,
        equipment,
        weapon,
        hands,
        palettes,
    } = visual
    {
        crate::game_art::walking::with_pose(None, || {
            finish_worn::compose(
                (
                    outfit_index(body, equipment),
                    selection(weapon, hands),
                    palettes,
                ),
                0,
            )
        })
    } else {
        None
    }
}
#[cfg(debug_assertions)]
pub(super) fn vertical_review_standing_head(
    visual: Visual,
    apply: impl FnOnce(&mut Image),
) -> Option<Image> {
    let Visual::Player {
        body,
        equipment,
        weapon,
        hands,
        palettes,
    } = visual
    else {
        return None;
    };
    crate::game_art::walking::with_pose(None, || {
        finish_worn::vertical_review_standing_head(
            (
                outfit_index(body, equipment),
                selection(weapon, hands),
                palettes,
            ),
            apply,
        )
    })
}
pub(super) fn approved_walking_image(
    body: u8,
    equipment: [Option<u16>; 4],
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
    pose: crate::game_art::walking::Pose,
) -> Option<Image> {
    if !(3..=7).contains(&body)
        || ![-1, 1].contains(&pose.facing)
        || ![0, 1, 2, 3, 4, 5, 8].contains(&pose.frame)
    {
        return None;
    }
    let key = (
        outfit_index(body, equipment),
        selection(weapon, hands),
        colors,
    );
    let layers = finish_worn::approved_walking_layers(key, pose)?;
    let mut image = Image::gen_image_color(128, 128, BLANK);
    for layer in layers {
        held_geometry::over(&mut image, &layer);
    }
    Some(image)
}
pub(super) fn outfit_index(body: u8, equipment: [Option<u16>; 4]) -> u16 {
    if let Some(outfit) = finish_worn::class_outfit(body, equipment) {
        return outfit;
    }
    if let Some(outfit) = equipment[0].and_then(palettes::reviewed_outfit) {
        return outfit;
    }
    equipment[0]
        .filter(|id| (101..=114).contains(id))
        .map(|id| 3 + id - 101)
        .unwrap_or(if (3..=7).contains(&body) {
            200 + u16::from(body - 3)
        } else {
            u16::from(body.min(2))
        })
}
struct Sources {
    flat: flat_agents::FlatAgents,
    agents: agent_rigs::AgentRigs,
    bodies: palettes::LazyImage,
    body_maps: palettes::LazyImage,
    arms: palettes::LazyImage,
    arm_maps: palettes::LazyImage,
    fingers: palettes::LazyImage,
    weapons: palettes::LazyImage,
    weapon_maps: palettes::LazyImage,
}
impl Sources {
    fn load() -> Self {
        Self {
            flat: flat_agents::FlatAgents::load(),
            agents: agent_rigs::AgentRigs::load(),
            bodies: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-bodies-32.png"
            )),
            body_maps: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-body-materials-32.png"
            )),
            arms: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-arms-48.png"
            )),
            arm_maps: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-arm-materials-48.png"
            )),
            fingers: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-fingers-48.png"
            )),
            weapons: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/families-48.png"
            )),
            weapon_maps: palettes::LazyImage::new(include_bytes!(
                "../assets/sprites/cyberpunk/crawl-weapon-materials-48.png"
            )),
        }
    }
    #[cfg(test)]
    fn fixed_body(&self, body: u16) -> Image {
        self.agents
            .body(body)
            .unwrap_or_else(|| palettes::cell(&self.bodies, body, 32))
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HeldLayer {
    model: u16,
    family: u8,
    left: bool,
    pose: u8,
}
type Key = (u16, [Option<HeldLayer>; 2], [EquipmentPalette; 3]);
thread_local! {
    #[cfg(debug_assertions)]
    static CREATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static SOURCES: Sources=Sources::load();
    static CACHE: std::cell::RefCell<BTreeMap<(Key,u8,u16),(portrait::Sheet,f64)>>=std::cell::RefCell::new(BTreeMap::new());
}
fn layer(weapon: u16, left: bool) -> Option<HeldLayer> {
    let family = *WEAPON_FAMILIES.get(usize::from(weapon))?;
    debug_assert!(u16::from(family) < WEAPON_FAMILY_COUNT);
    let pose = match weapon_hold(weapon) {
        WeaponHold::Side => 0,
        WeaponHold::Raised => 1,
        WeaponHold::TwoHanded => 2,
        WeaponHold::MeleeTwoHanded => 3,
    };
    Some(HeldLayer {
        model: weapon,
        family,
        left,
        pose,
    })
}
fn selection(weapon: Option<u16>, hands: Option<[Option<u16>; 2]>) -> [Option<HeldLayer>; 2] {
    match hands {
        Some([left, right]) => [
            left.and_then(|w| layer(w, true)),
            right.and_then(|w| layer(w, false)),
        ],
        None => [None, weapon.and_then(|w| layer(w, false))],
    }
}
fn hand_cut(layers: [Option<HeldLayer>; 2], x: usize, y: usize) -> bool {
    if !(17..23).contains(&y) {
        return false;
    }
    let left = (3..10).contains(&x);
    let right = (22..30).contains(&x);
    layers.into_iter().flatten().any(|l| {
        if l.pose >= 2 {
            left || right
        } else {
            // A raised wrist needs its original lowered elbow/upper forearm.
            // Side grips only replace the old fist and the very end of its cuff.
            y >= if l.pose == 1 { 19 } else { 18 } && if l.left { left } else { right }
        }
    })
}
fn paste(result: &mut Image, source: &Image, dx: usize, dy: usize, flip: bool) {
    let w = usize::from(source.width);
    let h = usize::from(source.height);
    for y in 0..h.min(usize::from(result.height).saturating_sub(dy)) {
        for x in 0..w.min(usize::from(result.width).saturating_sub(dx)) {
            let sx = if flip { w - 1 - x } else { x };
            let a = (y * w + sx) * 4;
            if source.bytes[a + 3] == 0 {
                continue;
            }
            let b = ((dy + y) * usize::from(result.width) + dx + x) * 4;
            result.bytes[b..b + 4].copy_from_slice(&source.bytes[a..a + 4]);
        }
    }
}
// Two held poses, 1.5 seconds each: the reviewed one-pixel breath. None is
// used for portraits, reduced motion and non-world callers.
fn idle_offset(time: Option<f64>) -> u8 {
    time.filter(|t| t.is_finite() && *t >= 0.0)
        .map_or(0, |t| ((t / 1.5).floor() % 2.0) as u8)
}
#[cfg(test)]
fn compose(source: &Sources, key: Key) -> Image {
    compose_pose(source, key, 0)
}
#[cfg(test)]
fn compose_pose(source: &Sources, key: Key, breath: u8) -> Image {
    compose_pose_at_resolution(source, key, breath, 32)
}
fn compose_pose_at_resolution(source: &Sources, key: Key, breath: u8, resolution: u16) -> Image {
    if resolution == 64
        && let Some(image) = finish_worn::compose(key, breath)
    {
        return image;
    }
    if let Some(image) = source.flat.compose_pose(key, resolution, breath) {
        return image;
    }
    if let Some(image) = source.agents.compose(key, breath) {
        return image;
    }
    let (body, layers, colors) = key;
    let mut fixed = palettes::cell(&source.bodies, body, 32);
    palettes::recolor(
        &mut fixed,
        &palettes::cell(&source.body_maps, body, 32),
        colors[0],
    );
    for y in 0..32 {
        for x in 0..32 {
            if hand_cut(layers, x, y) {
                fixed.bytes[(y * 32 + x) * 4..(y * 32 + x) * 4 + 4].fill(0);
            }
        }
    }
    let mut result = Image::gen_image_color(48, 48, BLANK);
    // Draw the upper body first, then the fixed lower body over the waist.
    // Moving the full character would make its feet slide on the map.
    for (start, end, offset) in [(0, 23, breath as usize), (23, 32, 0)] {
        for y in start..end {
            for x in 0..32 {
                let a = (y * 32 + x) * 4;
                if fixed.bytes[a + 3] != 0 {
                    let b = ((8 + y + offset) * 48 + 8 + x) * 4;
                    result.bytes[b..b + 4].copy_from_slice(&fixed.bytes[a..a + 4]);
                }
            }
        }
    }
    for l in layers.into_iter().flatten() {
        let mut sleeve = palettes::cell(&source.arms, u16::from(l.family), 48);
        palettes::recolor(
            &mut sleeve,
            &palettes::cell(&source.arm_maps, u16::from(l.family), 48),
            colors[0],
        );
        paste(&mut result, &sleeve, 0, breath as usize, l.left);
        let mut weapon = palettes::cell(&source.weapons, u16::from(l.family), 48);
        palettes::recolor(
            &mut weapon,
            &palettes::cell(&source.weapon_maps, u16::from(l.family), 48),
            colors[if l.left { 2 } else { 1 }],
        );
        paste(&mut result, &weapon, 0, breath as usize, l.left);
        paste(
            &mut result,
            &palettes::cell(&source.fingers, u16::from(l.family), 48),
            0,
            breath as usize,
            l.left,
        );
    }
    result
}
pub(super) fn prewarm() {
    SOURCES.with(|_| {});
}
fn source_resolution(key: Key, rect: Rect, side: u16) -> u16 {
    if finish_worn::supports(key) {
        return 64;
    }
    if (200..200 + palettes::FLAT_OUTFIT_COUNT).contains(&key.0)
        && (side == 128 || rect.w.max(rect.h) > 32.0)
    {
        64
    } else {
        32
    }
}
fn draw_key(rect: Rect, key: Key, side: u16, opacity: f32, idle_time: Option<f64>) {
    if finish_worn::draw(rect, key, side, opacity, idle_offset(idle_time)) {
        return;
    }
    let resolution = source_resolution(key, rect, side);
    let cache_key = (key, idle_offset(idle_time), resolution);
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        if !cache.contains_key(&cache_key) {
            let _loading = project_rl::loading_profile::Span::caller("prepare_player_sprite", 0);
            let image = SOURCES
                .with(|source| compose_pose_at_resolution(source, key, cache_key.1, resolution));
            let sheet = portrait::Sheet::from_image(&image, image.width);
            palettes::evict(&mut cache, 192);
            cache.insert(cache_key, (sheet, get_time()));
            #[cfg(debug_assertions)]
            CREATIONS.with(|count| count.set(count.get() + 1));
        }
        let (sheet, time) = cache.get_mut(&cache_key).unwrap();
        *time = get_time();
        if side == 128 {
            sheet.draw(rect, 0, opacity);
            return;
        }
        let texture = &sheet.texture;
        let scale = texture.width() / f32::from(resolution);
        let destination = Rect::new(
            rect.x - rect.w * (scale - 1.0) * 0.5,
            rect.y - rect.h * (scale - 1.0) * 0.5,
            rect.w * scale,
            rect.h * scale,
        );
        crate::world_readability::texture(
            texture,
            destination.x,
            destination.y,
            Color::new(1.0, 1.0, 1.0, opacity),
            DrawTextureParams {
                dest_size: Some(destination.size()),
                ..Default::default()
            },
        );
    });
}

#[cfg(debug_assertions)]
pub(super) fn cache_stats() -> (usize, usize) {
    (
        CACHE.with(|c| c.borrow().len()) + finish_worn::cache_len(),
        CREATIONS.with(|c| c.get()),
    )
}
pub(super) fn draw(
    rect: Rect,
    body: u8,
    equipment: [Option<u16>; 4],
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
    side: u16,
    opacity: f32,
    idle_time: Option<f64>,
) {
    draw_key(
        rect,
        (
            outfit_index(body, equipment),
            selection(weapon, hands),
            colors,
        ),
        side,
        opacity,
        idle_time,
    );
}
pub(super) fn draw_npc(
    rect: Rect,
    body: u8,
    weapon: Option<u16>,
    side: u16,
    opacity: f32,
    idle_time: Option<f64>,
) {
    if (20..=25).contains(&body) {
        draw_key(
            rect,
            (
                17 + u16::from(body - 20),
                selection(weapon, None),
                [EquipmentPalette::Original; 3],
            ),
            side,
            opacity,
            idle_time,
        );
    }
}
#[cfg(debug_assertions)]
pub(super) fn draw_pistol_reference(rect: Rect, body: u8, equipment: [Option<u16>; 4], side: u16) {
    draw(
        rect,
        body,
        equipment,
        held_weapon("core:modele_cogneur_p01"),
        None,
        [EquipmentPalette::Original; 3],
        side,
        1.0,
        None,
    );
}
#[cfg(debug_assertions)]
pub(super) fn draw_grip_review(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
) {
    draw_grip_review_at_time(rect, body, weapon, hands, None);
}
#[cfg(debug_assertions)]
pub(super) fn draw_finish_review(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
) {
    draw_key(rect, (body, selection(weapon, hands), colors), 32, 1., None);
}
#[cfg(debug_assertions)]
pub(super) fn draw_grip_review_at_time(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    time: Option<f64>,
) {
    draw_key(
        rect,
        (
            body,
            selection(weapon, hands),
            [EquipmentPalette::Original; 3],
        ),
        32,
        1.0,
        time,
    );
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn breath_timing_is_discrete_and_can_be_disabled() {
        for t in [0.0, 0.75, 1.499, 3.0, 4.49] {
            assert_eq!(idle_offset(Some(t)), 0);
        }
        for t in [1.5, 2.999, 4.5, 5.99] {
            assert_eq!(idle_offset(Some(t)), 1);
        }
        for time in [None, Some(-1.0), Some(f64::NAN), Some(f64::INFINITY)] {
            assert_eq!(idle_offset(time), 0);
        }
    }
    #[test]
    fn breathing_keeps_feet_fixed_and_moves_every_held_part_together() {
        let source = Sources::load();
        for body in 0..23 {
            for weapon in std::iter::once(None).chain((0..HELD_WEAPON_COUNT).map(Some)) {
                let key = (
                    body,
                    selection(weapon, None),
                    [EquipmentPalette::Original; 3],
                );
                let rest = compose_pose(&source, key, 0);
                let breath = compose_pose(&source, key, 1);
                // Everything above the fixed waist, including weapon and fingers,
                // is translated exactly one native pixel, with no resampling.
                let origin = (u32::from(rest.width) - 32) / 2;
                for y in 0..origin + 22 {
                    for x in 0..u32::from(rest.width) {
                        assert_eq!(
                            rest.get_pixel(x, y),
                            breath.get_pixel(x, y + 1),
                            "body {body}, weapon {weapon:?}, {x},{y}"
                        );
                    }
                }
                // On an unarmed figure the entire lower body remains identical.
                if weapon.is_none() {
                    for y in origin + 24..u32::from(rest.height) {
                        for x in 0..u32::from(rest.width) {
                            assert_eq!(rest.get_pixel(x, y), breath.get_pixel(x, y));
                        }
                    }
                }
                // No weapon leaves the extended canvas during the breath.
                for x in 0..u32::from(rest.width) {
                    assert_eq!(rest.get_pixel(x, u32::from(rest.height) - 1).a, 0.0);
                }
            }
        }
    }
    #[test]
    fn torso_selects_whole_outfit_regardless_of_other_equipment_or_class() {
        for torso in 101..=114 {
            for body in 0..3 {
                let expected = match torso {
                    101 => 212,
                    103 => 213,
                    105 => 214,
                    106 => 215,
                    107 => 216,
                    108 => 208,
                    109 => 217,
                    110 => 209,
                    111 => 210,
                    112 => 211,
                    114 => 218,
                    _ => 3 + torso - 101,
                };
                assert_eq!(
                    outfit_index(body, [Some(torso), Some(120), Some(126), Some(132)]),
                    expected
                );
            }
        }
    }
    #[test]
    fn all_reviewed_worn_outfits_select_native64_for_world_and_portraits() {
        for body in 200..200 + palettes::FLAT_OUTFIT_COUNT {
            let key = (body, [None, None], [EquipmentPalette::Original; 3]);
            assert_eq!(
                source_resolution(key, Rect::new(0.0, 0.0, 64.0, 64.0), 32),
                64
            );
            assert_eq!(
                source_resolution(key, Rect::new(0.0, 0.0, 56.0, 56.0), 32),
                64
            );
            assert_eq!(
                source_resolution(key, Rect::new(0.0, 0.0, 128.0, 128.0), 128),
                64
            );
            assert_eq!(
                source_resolution(key, Rect::new(0.0, 0.0, 32.0, 32.0), 32),
                if finish_worn::supports(key) { 64 } else { 32 }
            );
        }
    }
    #[test]
    fn every_catalogued_weapon_has_a_nonempty_held_family() {
        let source = Sources::load();
        for weapon in 0..HELD_WEAPON_COUNT {
            let l = layer(weapon, false).unwrap();
            assert!(u16::from(l.family) < WEAPON_FAMILY_COUNT);
            assert!(
                palettes::cell(&source.weapons, u16::from(l.family), 48)
                    .bytes
                    .chunks_exact(4)
                    .any(|p| p[3] > 0)
            );
        }
        assert!(layer(HELD_WEAPON_COUNT, false).is_none());
        assert_ne!(
            layer(held_weapon("core:couteau_ceramique").unwrap(), false),
            layer(held_weapon("core:marteau_de_guerre").unwrap(), false)
        );
    }
    fn components(image: &Image) -> Vec<Vec<(u32, u32)>> {
        let mut remaining: std::collections::BTreeSet<_> = (0..u32::from(image.height))
            .flat_map(|y| (0..u32::from(image.width)).map(move |x| (x, y)))
            .filter(|&(x, y)| image.get_pixel(x, y).a > 0.0)
            .collect();
        let mut groups = Vec::new();
        while let Some(seed) = remaining.pop_first() {
            let mut group = vec![seed];
            let mut stack = vec![seed];
            while let Some((x, y)) = stack.pop() {
                for yy in y.saturating_sub(1)..=(y + 1).min(u32::from(image.height) - 1) {
                    for xx in x.saturating_sub(1)..=(x + 1).min(u32::from(image.width) - 1) {
                        if remaining.remove(&(xx, yy)) {
                            group.push((xx, yy));
                            stack.push((xx, yy));
                        }
                    }
                }
            }
            groups.push(group);
        }
        groups.sort_by_key(|g| std::cmp::Reverse(g.len()));
        groups
    }
    #[test]
    fn weapons_have_no_detached_debris_and_every_hand_covers_its_grip() {
        let source = Sources::load();
        for family in 0..WEAPON_FAMILY_COUNT {
            let weapon = palettes::cell(&source.weapons, family, 48);
            let fingers = palettes::cell(&source.fingers, family, 48);
            assert_eq!(
                components(&weapon).len(),
                1,
                "family {family}: floating weapon fragment"
            );
            for &(x, y) in WEAPON_GRIPS[usize::from(family)] {
                let contact = (u32::from(y) - 2..=u32::from(y) + 2)
                    .flat_map(|yy| (u32::from(x) - 2..=u32::from(x) + 2).map(move |xx| (xx, yy)))
                    .filter(|&(xx, yy)| {
                        weapon.get_pixel(xx, yy).a > 0.0 && fingers.get_pixel(xx, yy).a > 0.0
                    })
                    .count();
                assert!(
                    contact >= 2,
                    "family {family}, grip {x},{y}: fingers must wrap the handle ({contact})"
                );
            }
        }
    }
    #[test]
    fn every_body_and_hand_combination_has_no_new_floating_pixels() {
        let source = Sources::load();
        let one = [0, 1, 4, 5, 6, 7, 16];
        let representative =
            |family: u8| WEAPON_FAMILIES.iter().position(|f| *f == family).unwrap() as u16;
        for body in 0..23 {
            let fixed = source.fixed_body(body);
            let check = |layers| {
                let image = compose(&source, (body, layers, [EquipmentPalette::Original; 3]));
                let origin = (u32::from(image.width) - 32) / 2;
                for group in components(&image).into_iter().skip(1) {
                    assert!(
                        group.iter().all(|&(x, y)| {
                            (origin..origin + 32).contains(&x)
                                && (origin..origin + 32).contains(&y)
                                && fixed.get_pixel(x - origin, y - origin).a > 0.0
                        }),
                        "body {body}, layers {layers:?}: new floating part {group:?}"
                    );
                }
            };
            for family in 0..WEAPON_FAMILY_COUNT as u8 {
                for left in [false, true] {
                    if left && !one.contains(&family) {
                        continue;
                    }
                    check([None, layer(representative(family), left)]);
                }
            }
            for left in one {
                for right in one {
                    check(selection(
                        None,
                        Some([Some(representative(left)), Some(representative(right))]),
                    ));
                }
            }
        }
    }
    #[test]
    fn knife_and_greatsword_cutting_edges_face_outward() {
        let source = Sources::load();
        for (family, rows) in [(0, 15..25), (2, 6..21)] {
            let weapon = palettes::cell(&source.weapons, family, 48);
            let mut tested = 0;
            for y in rows {
                let steel: Vec<_> = (0..48)
                    .filter(|&x| {
                        let p = weapon.get_pixel(x, y);
                        p.a > 0.0 && p.r > 0.35 && (p.r - p.b).abs() < 0.18
                    })
                    .collect();
                if steel.len() < 2 {
                    continue;
                }
                let brightest = *steel
                    .iter()
                    .max_by(|&&a, &&b| {
                        weapon
                            .get_pixel(a, y)
                            .r
                            .total_cmp(&weapon.get_pixel(b, y).r)
                    })
                    .unwrap();
                assert!(
                    brightest >= (steel[0] + steel[steel.len() - 1]) / 2,
                    "family {family}, row {y}: sharpened edge must face screen right"
                );
                tested += 1;
            }
            assert!(tested >= 5);
        }
    }
    #[test]
    fn both_hands_keep_their_family_without_mirroring_the_outfit() {
        let pistol = held_weapon("core:modele_cogneur_p01").unwrap();
        let smg = held_weapon("core:modele_crache_fer_pm01").unwrap();
        let a = selection(None, Some([Some(pistol), Some(smg)]));
        let b = selection(None, Some([Some(smg), Some(pistol)]));
        assert_eq!(a[0].unwrap().family, b[1].unwrap().family);
        assert!(a[0].unwrap().left);
        assert!(!a[1].unwrap().left);
        let source = Sources::load();
        let left = compose(&source, (4, a, [EquipmentPalette::Original; 3]));
        let right = compose(&source, (4, b, [EquipmentPalette::Original; 3]));
        for y in (8..18).chain(33..40) {
            for x in 22..26 {
                let n = (y * 48 + x) * 4;
                assert_eq!(&left.bytes[n..n + 4], &right.bytes[n..n + 4]);
            }
        }
    }
    #[test]
    fn all_bodies_are_thirty_pixels_tall_and_stable_outside_foreground_and_hands() {
        let source = Sources::load();
        for body in 0..23 {
            // Agent rigs have independent masks and are checked byte-for-byte
            // against all approved family/hand compositions in agent_rigs tests.
            if agent_rigs::AgentRigs::profile(body).is_some() {
                continue;
            }
            let fixed = palettes::cell(&source.bodies, body, 32);
            let rows: Vec<_> = (0..32)
                .filter(|y| (0..32).any(|x| fixed.get_pixel(x, *y).a > 0.0))
                .collect();
            assert_eq!(rows.last().unwrap() - rows[0] + 1, 30);
            for weapon in 0..HELD_WEAPON_COUNT {
                let layers = selection(Some(weapon), None);
                let image = compose(&source, (body, layers, [EquipmentPalette::Original; 3]));
                let mut occlusion = Image::gen_image_color(48, 48, BLANK);
                for l in layers.into_iter().flatten() {
                    for (bank, id) in [
                        (&source.arms, u16::from(l.family)),
                        (&source.weapons, u16::from(l.family)),
                        (&source.fingers, u16::from(l.family)),
                    ] {
                        paste(&mut occlusion, &palettes::cell(bank, id, 48), 0, 0, l.left);
                    }
                }
                for y in 0..32 {
                    for x in 0..32 {
                        if !hand_cut(layers, x, y)
                            && occlusion.get_pixel(x as u32 + 8, y as u32 + 8).a == 0.0
                        {
                            assert_eq!(
                                fixed.get_pixel(x as u32, y as u32),
                                image.get_pixel(x as u32 + 8, y as u32 + 8)
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn separate_equipment_palettes_preserve_geometry() {
        let source = Sources::load();
        let pistol = held_weapon("core:modele_cogneur_p01").unwrap();
        let knife = held_weapon("core:couteau_ceramique").unwrap();
        let layers = selection(None, Some([Some(pistol), Some(knife)]));
        let original = compose(&source, (9, layers, [EquipmentPalette::Original; 3]));
        for colors in [
            [
                EquipmentPalette::Blue,
                EquipmentPalette::Original,
                EquipmentPalette::Original,
            ],
            [
                EquipmentPalette::Original,
                EquipmentPalette::Red,
                EquipmentPalette::Gray,
            ],
        ] {
            let colored = compose(&source, (9, layers, colors));
            assert_ne!(original.bytes, colored.bytes);
            for (a, b) in original
                .bytes
                .chunks_exact(4)
                .zip(colored.bytes.chunks_exact(4))
            {
                assert_eq!(a[3], b[3]);
            }
        }
    }

    #[test]
    fn axe_blade_faces_outward_in_each_hand() {
        let source = Sources::load();
        let weapon = palettes::cell(&source.weapons, 3, 48);
        let count = |left: bool| {
            (0..18)
                .flat_map(|y| (0..48).map(move |x| (x, y)))
                .filter(|&(x, y)| {
                    let p = weapon.get_pixel(x, y);
                    p.a > 0.0 && p.r > 0.6 && p.g > 0.5 && p.b > 0.4 && (x < 27) == left
                })
                .count()
        };
        assert!(
            count(false) > count(true) * 2,
            "blade must project to the right of its shaft"
        );
        let mut mirrored = Image::gen_image_color(48, 48, BLANK);
        paste(&mut mirrored, &weapon, 0, 0, true);
        for y in 0..48 {
            for x in 0..48 {
                assert_eq!(weapon.get_pixel(x, y), mirrored.get_pixel(47 - x, y));
            }
        }
    }
}
