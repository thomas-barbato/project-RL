//! Local scenery and perceived signals only. Neither changes the simulation.
use crate::test_sector::{Decor, SectorDecor};
use project_rl::ai::AiBehavior;
use project_rl::entity::Actor;
use project_rl::presentation::{VisualCue, VisualCueTarget};
use project_rl::world::{GridPos, Map, Terrain, has_line_of_sight};
use std::collections::BTreeSet;

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

/// Small, static cross-branched patches around initial homes; never redraw from
/// moving actors. Plain dry floors only, with no cables or interactive devices.
pub(crate) fn decorate_ground(decor: &mut SectorDecor, map: &Map, actors: &[Actor]) {
    let mut cells = BTreeSet::new();
    for actor in actors.iter().filter(|actor| {
        actor
            .ai()
            .is_some_and(|ai| ai.behavior == AiBehavior::Watcher)
    }) {
        let home = actor.position();
        for (dx, dy) in DIRECTIONS {
            for distance in 0..=2 {
                let at = GridPos::new(home.x + dx * distance, home.y + dy * distance);
                if map
                    .tile(at)
                    .is_some_and(|tile| tile.terrain == Terrain::Floor)
                    && !map.is_protected(at)
                    && has_line_of_sight(map, home, at, true)
                    && matches!(decor.cells.get(&at), Some(Decor::RuinFloor | Decor::Gravel))
                {
                    cells.insert(at);
                }
            }
        }
    }
    for &at in &cells {
        let mask = DIRECTIONS
            .iter()
            .enumerate()
            .fold(0, |mask, (bit, (dx, dy))| {
                mask | if cells.contains(&GridPos::new(at.x + dx, at.y + dy)) {
                    1 << bit
                } else {
                    0
                }
            });
        if mask != 0 {
            decor.cells.insert(at, Decor::NetworkTrace(mask));
        }
    }
}

/// Never infer edges from current actor positions or show a path to an unseen
/// recipient. A partially hidden transmission produces no world-space cue.
pub(crate) fn perceived_link(
    from: GridPos,
    to: GridPos,
    visible: impl Fn(GridPos) -> bool,
) -> Option<VisualCue> {
    if from == to || !visible(from) || !visible(to) {
        return None;
    }
    let cue = VisualCue::line("core:watcher_signal".parse().unwrap(), from, to).ok()?;
    let VisualCueTarget::World { cells, .. } = cue.target() else {
        return None;
    };
    cells
        .iter()
        .all(|cell| visible(cell.position))
        .then_some(cue)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visual_effects::{TerminalEffectFamily, VisualCuePlayer};
    use project_rl::ai::AiProfile;
    use project_rl::presentation::{
        TerminalCueStyle, TerminalEffectGlyph, VisualCueCatalog, VisualCueDefinition,
    };

    #[test]
    fn watcher_signals_hide_entire_links_with_unseen_endpoints_or_intermediate_cells() {
        let from = GridPos::new(2, 3);
        let to = GridPos::new(6, 3);
        assert!(perceived_link(from, to, |_| true).is_some());
        for hidden in [from, to, GridPos::new(4, 3)] {
            assert!(perceived_link(from, to, |at| at != hidden).is_none());
        }
        assert!(perceived_link(from, from, |_| true).is_none());
    }

    #[test]
    fn watcher_signals_are_directional_brief_and_respect_reduced_motion_and_visibility() {
        let id = "core:watcher_signal".parse().unwrap();
        let mut styles = VisualCueCatalog::default();
        styles
            .register(VisualCueDefinition::new(
                id,
                TerminalCueStyle::new(
                    vec![TerminalEffectGlyph::Signal],
                    [113, 167, 167, 220],
                    210,
                    65,
                    130,
                )
                .unwrap(),
            ))
            .unwrap();
        let mut player = VisualCuePlayer::with_catalog(styles);
        let from = GridPos::new(2, 3);
        let to = GridPos::new(6, 3);
        player.play(perceived_link(from, to, |_| true).unwrap(), 0.0);
        let at = GridPos::new(4, 3);
        assert!(player.sample_world(at, true, 0.02).is_none());
        let sample = player.sample_world(at, true, 0.18).unwrap();
        assert_eq!(sample.family, TerminalEffectFamily::Signal);
        assert_eq!(sample.direction, (1.0, 0.0));
        assert!(player.sample_world(at, false, 0.18).is_none());
        assert!(player.sample_world(at, true, 1.0).is_none());
        assert_eq!(
            player.sample_world_with_motion(at, true, 0.1, true),
            player.sample_world_with_motion(at, true, 0.2, true)
        );
        assert!(
            player
                .sample_world_with_motion(at, false, 0.1, true)
                .is_none()
        );
        player.clear_world();
        assert!(player.sample_world(at, true, 0.18).is_none());
    }

    #[test]
    fn watcher_ground_is_static_dry_noninteractive_and_save_compatible() {
        let mut map = Map::from_ascii(
            "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
        )
        .unwrap();
        let wall = GridPos::new(5, 3);
        let water = GridPos::new(4, 2);
        let safe = GridPos::new(2, 3);
        map.set_terrain(wall, Terrain::Wall).unwrap();
        map.set_terrain(water, Terrain::ShallowWater).unwrap();
        map.set_protected(safe, true).unwrap();
        let mut decor = SectorDecor::default();
        for y in 1..6 {
            for x in 1..8 {
                decor.cells.insert(GridPos::new(x, y), Decor::RuinFloor);
            }
        }
        let cache = GridPos::new(4, 4);
        decor.cells.insert(cache, Decor::SupplyCache);
        let mut ai = AiProfile::hunter(5, 0);
        ai.behavior = AiBehavior::Watcher;
        let actor = Actor::new(GridPos::new(4, 3), 8).unwrap().with_ai(ai);
        let map_before = map.clone();
        let actors = [actor];
        decorate_ground(&mut decor, &map, &actors);
        assert_eq!(map, map_before);
        assert_eq!(decor.cells[&cache], Decor::SupplyCache);
        for at in [wall, water, safe, GridPos::new(6, 3)] {
            assert!(!matches!(decor.cells[&at], Decor::NetworkTrace(_)));
        }
        assert!(
            decor
                .cells
                .values()
                .any(|kind| matches!(kind, Decor::NetworkTrace(_)))
        );
        for kind in decor
            .cells
            .values()
            .filter(|kind| matches!(kind, Decor::NetworkTrace(_)))
        {
            assert!(!kind.blocks());
            assert!(!kind.label().contains("interagir"));
        }
        let bytes = bincode::serialize(&decor).unwrap();
        let restored: SectorDecor = bincode::deserialize(&bytes).unwrap();
        assert_eq!(decor, restored);
        // Prior discriminants stay exactly where they were (Nest was last).
        let old = bincode::serialize(&Decor::Nest).unwrap();
        assert_eq!(u32::from_le_bytes(old.try_into().unwrap()), 64);
    }
}
