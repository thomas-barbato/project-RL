//! Walking presentation context; accepted WALK06 and disabled legacy WALK04.
//! Presentation state never enters the simulation or save.
use super::*;
use project_rl::asset_cache::ByteCache;
use serde::Deserialize;
use std::{cell::Cell, collections::BTreeMap, sync::OnceLock};
#[path = "game_art_walk_approved.rs"]
mod approved;
#[path = "game_art_walk_creatures.rs"]
mod creatures;
#[path = "game_art_walk_extras.rs"]
mod extras;
#[path = "game_art_walk_rifle.rs"]
mod rifle;

// WALK12 common gait, WALK11 proportions and rigid grips accepted on25 October2026.
// The separate WALK08 forms/drone proposals keep their own review context.
const PLAYER_WALK_ACCEPTED: bool = true;
const PROPORTIONS_ACCEPTED: bool = true;
const GAIT12_ACCEPTED: bool = true;
// DUAL15-r01 accepted on 28 October 2026. The scoped override retains ARM14
// solely for historical comparisons; normal Debug and Release enable DUAL15.
const DUAL15_ACCEPTED: bool = true;
pub(crate) fn dual17_enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        return DUAL17.get();
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        false
    }
}
#[cfg(any(debug_assertions, test))]
thread_local! { static DUAL17: Cell<bool> = const { Cell::new(false) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct Dual17ReviewGuard(bool);
#[cfg(any(debug_assertions, test))]
impl Drop for Dual17ReviewGuard {
    fn drop(&mut self) {
        DUAL17.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_dual17(enabled: bool) -> Dual17ReviewGuard {
    Dual17ReviewGuard(DUAL17.replace(enabled))
}
pub(crate) fn dual15_enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        return DUAL15.get().unwrap_or(DUAL15_ACCEPTED);
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        DUAL15_ACCEPTED
    }
}
#[cfg(any(debug_assertions, test))]
thread_local! { static DUAL15: Cell<Option<bool>> = const { Cell::new(None) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct DualReviewGuard(Option<bool>);
#[cfg(any(debug_assertions, test))]
impl Drop for DualReviewGuard {
    fn drop(&mut self) {
        DUAL15.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_dual15(enabled: bool) -> DualReviewGuard {
    DualReviewGuard(DUAL15.replace(Some(enabled)))
}
pub(crate) fn player_enabled() -> bool {
    PLAYER_WALK_ACCEPTED || corrections_enabled()
}

pub(crate) fn corrections_enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        return REVIEW08.get();
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        false
    }
}

#[cfg(all(debug_assertions, test))]
pub(super) fn vertical_palette(number: u16, variant: u8, image: &mut Image) -> Option<()> {
    creatures::vertical_palette(number, variant, image)
}
#[cfg(any(debug_assertions, test))]
thread_local! { static REVIEW08: Cell<bool> = const { Cell::new(false) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct ReviewGuard(bool);
#[cfg(any(debug_assertions, test))]
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        REVIEW08.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_corrections() -> ReviewGuard {
    ReviewGuard(REVIEW08.replace(true))
}

pub(crate) fn common_cycle_enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        if corrections_enabled() {
            return COMMON09.get();
        }
    }
    PLAYER_WALK_ACCEPTED
}
#[cfg(any(debug_assertions, test))]
thread_local! { static COMMON09: Cell<bool> = const { Cell::new(false) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct CommonReviewGuard(bool);
#[cfg(any(debug_assertions, test))]
impl Drop for CommonReviewGuard {
    fn drop(&mut self) {
        COMMON09.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_common(enabled: bool) -> CommonReviewGuard {
    CommonReviewGuard(COMMON09.replace(enabled))
}

// Normal play uses the accepted bank. The catalogue can still select WALK09
// explicitly for its historical comparison, without changing the live default.
pub(crate) fn proportions_enabled() -> bool {
    #[cfg(any(debug_assertions, test))]
    {
        return PROPORTIONS11.get().unwrap_or(PROPORTIONS_ACCEPTED);
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        PROPORTIONS_ACCEPTED
    }
}
#[cfg(any(debug_assertions, test))]
thread_local! { static PROPORTIONS11: Cell<Option<bool>> = const { Cell::new(None) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct ProportionsReviewGuard(Option<bool>);
#[cfg(any(debug_assertions, test))]
impl Drop for ProportionsReviewGuard {
    fn drop(&mut self) {
        PROPORTIONS11.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_proportions(enabled: bool) -> ProportionsReviewGuard {
    ProportionsReviewGuard(PROPORTIONS11.replace(Some(enabled)))
}

// Normal Debug and Release play use the accepted WALK12 figures. A scoped
// catalogue override retains the previous WALK11 comparison without changing it.
pub(crate) fn gait12_enabled() -> bool {
    if !proportions_enabled() {
        return false;
    }
    #[cfg(any(debug_assertions, test))]
    {
        return GAIT12_OVERRIDE.get().unwrap_or(GAIT12_ACCEPTED);
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        GAIT12_ACCEPTED
    }
}
#[cfg(any(debug_assertions, test))]
thread_local! { static GAIT12_OVERRIDE: Cell<Option<bool>> = const { Cell::new(None) }; }
#[cfg(any(debug_assertions, test))]
pub(crate) struct GaitReviewGuard(Option<bool>);
#[cfg(any(debug_assertions, test))]
impl Drop for GaitReviewGuard {
    fn drop(&mut self) {
        GAIT12_OVERRIDE.set(self.0);
    }
}
#[cfg(any(debug_assertions, test))]
pub(crate) fn review_gait12(enabled: bool) -> GaitReviewGuard {
    GaitReviewGuard(GAIT12_OVERRIDE.replace(Some(enabled)))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Pose {
    /// -1 left, 0 legacy front, +1 right; -2 true back and +2 true front.
    pub facing: i8,
    /// Six movement poses, three per cell; marker8 selects the separate idle.
    pub frame: u8,
}
thread_local! {
    static POSE: Cell<Option<Pose>> = const {Cell::new(None)};
    static CACHE: std::cell::RefCell<ByteCache<&'static str, Option<Texture2D>>> =
        const {std::cell::RefCell::new(ByteCache::new(64 * 1024 * 1024))};
}
// The rejected WALK04 bank/rigs stay disabled. WALK06 is dispatched separately
// below and must not activate these old experiments as a side effect.
const REVIEW_ACCEPTED: bool = false;
pub(crate) fn with_pose<T>(pose: Option<Pose>, draw: impl FnOnce() -> T) -> T {
    struct Guard(Option<Pose>);
    impl Drop for Guard {
        fn drop(&mut self) {
            POSE.set(self.0);
        }
    }
    let _guard = Guard(POSE.replace(pose));
    draw()
}
pub(super) fn active() -> bool {
    REVIEW_ACCEPTED && POSE.get().is_some()
}
pub(crate) fn pose() -> Option<Pose> {
    if REVIEW_ACCEPTED { POSE.get() } else { None }
}

pub(super) fn static_cell(cell: Rect, visual: Visual) -> Rect {
    creatures::static_cell(cell, visual)
}

#[cfg(debug_assertions)]
pub(crate) fn creature_selection(visual: Visual, pose: Pose) -> Option<(&'static str, Rect)> {
    creatures::diagnostic_selection(visual, pose)
}

#[cfg(debug_assertions)]
pub(crate) fn creature_audit() -> Vec<serde_json::Value> {
    creatures::diagnostic_audit()
}

#[derive(Deserialize)]
struct Entry {
    id: String,
    file: String,
    side: u16,
    foot: [u16; 2],
    configurations: u8,
    shadow: f32,
}
#[derive(Deserialize)]
struct Manifest {
    format_version: u8,
    native_tile: u16,
    frames: u8,
    entries: Vec<Entry>,
}
fn manifest() -> &'static BTreeMap<String, Entry> {
    static MANIFEST: OnceLock<BTreeMap<String, Entry>> = OnceLock::new();
    MANIFEST.get_or_init(|| {
        let parsed: Manifest =
            serde_json::from_str(include_str!("../assets/animations/walk-04/manifest.json"))
                .expect("validated WALK04 manifest");
        assert_eq!(
            (parsed.format_version, parsed.native_tile, parsed.frames),
            (1, 64, 9)
        );
        parsed
            .entries
            .into_iter()
            .map(|e| (e.id.clone(), e))
            .collect()
    })
}
fn binding(visual: Visual) -> Option<(String, u8)> {
    let (id, configuration) = match visual {
        Visual::Player { .. } => return None,
        // Service NPCs use the live modular clothing renderer, rather than the
        // older raw actor tile with the same numeric index.
        Visual::Actor(20..=25) => return None,
        Visual::Actor(index) => (format!("ACTOR-{index:02}"), 0),
        Visual::Bestiary {
            number,
            corpse: false,
        } => (format!("BANK-{number:03}"), 0),
        Visual::CampaignMonster { number, variant } => (format!("MON-{number:03}-{variant}"), 0),
        Visual::CampNpc(index) => (format!("NPC-{index:02}"), 0),
        Visual::DroneScout => ("DRONE-SCOUT".into(), 0),
        Visual::DroneTracker => ("DRONE-TRACKER".into(), 0),
        Visual::DroneGunner => ("DRONE-GUNNER".into(), 0),
        Visual::DroneGuardian => ("DRONE-GUARDIAN".into(), 0),
        Visual::AgentForm(index) => (
            format!(
                "FORM-{}",
                [
                    "PREDATOR",
                    "COLOSSUS",
                    "ARMOR",
                    "POSSESSED",
                    "ABYSS",
                    "CLOSED"
                ]
                .get(usize::from(index))?
            ),
            0,
        ),
        _ => return None,
    };
    Some((id, configuration))
}
pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32) -> bool {
    #[cfg(debug_assertions)]
    if crate::vertical_walk_review18::draw(cell, visual, opacity, POSE.get()) {
        return true;
    }
    if super::vertical::draw(cell, visual, opacity, POSE.get()) {
        return true;
    }
    if extras::draw(cell, visual, opacity, POSE.get()) {
        return true;
    }
    if approved::draw(cell, visual, opacity, POSE.get()) {
        return true;
    }
    if creatures::draw(cell, visual, opacity, POSE.get()) {
        return true;
    }
    if !REVIEW_ACCEPTED {
        return false;
    }
    let Some(pose) = POSE.get() else {
        return false;
    };
    let Some((id, config)) = binding(visual) else {
        return false;
    };
    let Some(entry) = manifest().get(&id) else {
        return false;
    };
    if config >= entry.configurations {
        return false;
    }
    let scale = cell.w / 64.;
    let side = f32::from(entry.side);
    let destination = Rect::new(
        cell.center().x - f32::from(entry.foot[0]) * scale,
        cell.bottom() - f32::from(entry.foot[1]) * scale,
        side * scale,
        side * scale,
    );
    CACHE.with_borrow_mut(|cache| {
        let key = entry.id.as_str();
        if cache.get(&key).is_none() {
            let loaded = project_rl::data_paths::read(&entry.file)
                .ok()
                .and_then(|bytes| crate::asset_loading::image(&bytes, Some(ImageFormat::Png)).ok());
            let bytes = loaded.as_ref().map_or(0, |i| i.bytes.len());
            let texture = loaded.map(|i| {
                assert_eq!(
                    (i.width, i.height),
                    (
                        entry.side * 9,
                        entry.side * 3 * u16::from(entry.configurations)
                    )
                );
                let texture = crate::asset_loading::from_image(&i);
                texture.set_filter(FilterMode::Nearest);
                texture
            });
            if texture.is_none() {
                eprintln!("[WALK04] Animation indisponible : {}", entry.file);
            }
            cache.insert(key, texture, bytes);
        }
        let Some(texture) = cache.get(&key).and_then(Option::as_ref) else {
            return false;
        };
        super::draw_contact_shadow(
            destination,
            [
                entry.foot[0] as u8,
                entry.foot[1].saturating_sub(1) as u8,
                entry.shadow as u8,
            ],
            entry.side,
            opacity,
        );
        crate::world_readability::texture(
            texture,
            destination.x,
            destination.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    f32::from(pose.frame.min(8)) * side,
                    f32::from(u16::from(config) * 3 + (pose.facing.clamp(-1, 1) + 1) as u16) * side,
                    side,
                    side,
                )),
                dest_size: Some(destination.size()),
                ..Default::default()
            },
        );
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejected_experiments_are_inactive_in_the_playable_renderer() {
        with_pose(
            Some(Pose {
                facing: 1,
                frame: 2,
            }),
            || {
                assert!(!active());
                assert_eq!(pose(), None);
                assert!(!draw(Rect::new(0., 0., 64., 64.), Visual::Actor(1), 1.));
            },
        );
    }
    #[test]
    fn runtime_inventory_is_complete_and_uses_only_installed_assets() {
        let entries = manifest();
        assert_eq!(entries.len(), 659);
        for entry in entries.values() {
            assert!(entry.file.starts_with("assets/animations/walk-04/"));
            let bytes =
                std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(&entry.file))
                    .unwrap();
            assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
            let size = (
                u32::from_be_bytes(bytes[16..20].try_into().unwrap()),
                u32::from_be_bytes(bytes[20..24].try_into().unwrap()),
            );
            assert_eq!(
                size,
                (
                    u32::from(entry.side) * 9,
                    u32::from(entry.side) * 3 * u32::from(entry.configurations)
                ),
                "{}",
                entry.id
            );
        }
    }
    #[test]
    fn unsupported_weapon_and_helmet_cannot_be_replaced_with_a_prototype_weapon() {
        let base = Visual::Player {
            body: 3,
            equipment: [None; 4],
            palettes: [EquipmentPalette::Original; 3],
            weapon: Some(66),
            hands: None,
        };
        // Every player loadout now uses its actual attached layers, including
        // the models that used to fall back to a motionless frontal body.
        assert_eq!(binding(base), None);
        if let Visual::Player {
            mut equipment,
            body,
            palettes,
            weapon,
            hands,
        } = base
        {
            equipment[1] = Some(115);
            assert!(
                binding(Visual::Player {
                    body,
                    equipment,
                    palettes,
                    weapon,
                    hands
                })
                .is_none()
            );
        }
        assert!(
            binding(Visual::Bestiary {
                number: 1,
                corpse: true
            })
            .is_none()
        );
    }
}
