//! Combat presentation. No simulation RNG, movement, time or collision changes.
use std::collections::BTreeMap;

use macroquad::prelude::*;
use project_rl::entity::EntityId;
use project_rl::world::{GridPos, Map, VisibilityState};

/// Shared presentation curve. A reaction returns to its anchor after 180 ms.
fn motion_offset(direction: Vec2, age: f64, recoil: bool) -> Vec2 {
    let t = (age / 0.18) as f32;
    let pulse = if !(0.0..1.0).contains(&t) {
        0.0
    } else if t < 0.28 {
        t / 0.28
    } else {
        let remaining = (1.0 - t) / 0.72;
        remaining * remaining
    };
    direction * pulse * if recoil { -0.17 } else { 0.23 }
}

/// Same reaction for damage and harmful status applications, including lab views.
pub(crate) fn hit_reaction_offset(at: Vec2, source: Vec2, age: f64, tile: f32) -> Vec2 {
    let direction = (source - at).normalize_or_zero();
    let direction = if direction == Vec2::ZERO {
        vec2(0.0, -1.0)
    } else {
        direction
    };
    motion_offset(direction, age, true) * tile
}

/// Read existing effects, never a skill name, source faction or colour.
pub(crate) fn harmful_status(definition: &project_rl::status::StatusDefinition) -> bool {
    use project_rl::status::{StatusEffectPrimitive, StatusModifier, StatusTrigger};
    definition
        .modifiers()
        .iter()
        .any(|modifier| match modifier {
            StatusModifier::ArmorFragilization { .. }
            | StatusModifier::MovementTimeMinimum { .. } => true,
            StatusModifier::Accuracy { amount } | StatusModifier::Stability { amount } => {
                *amount < 0
            }
            StatusModifier::DamageGuard { .. } => false,
        })
        || [
            StatusTrigger::TurnStart,
            StatusTrigger::TurnEnd,
            StatusTrigger::DamageReceived,
            StatusTrigger::DamageDealt,
            StatusTrigger::Movement,
            StatusTrigger::Death,
        ]
        .into_iter()
        .any(|trigger| {
            definition.hooks_for(trigger).any(|hook| {
                hook.effects().iter().any(|effect| {
                    matches!(effect,
                        StatusEffectPrimitive::DealDamage { packet, .. } if packet.amount > 0
                    )
                })
            })
        })
}

#[derive(Clone, Debug)]
struct Motion {
    direction: Vec2,
    started: f64,
    recoil: bool,
}

#[derive(Default, Debug)]
pub(crate) struct CombatMotion {
    motions: BTreeMap<EntityId, Motion>,
}

impl CombatMotion {
    pub fn play(
        &mut self,
        entity: EntityId,
        from: GridPos,
        toward: GridPos,
        recoil: bool,
        now: f64,
    ) {
        let mut direction =
            vec2((toward.x - from.x) as f32, (toward.y - from.y) as f32).normalize_or_zero();
        if recoil && direction == Vec2::ZERO {
            direction = vec2(0.0, -1.0);
        }
        self.motions.insert(
            entity,
            Motion {
                direction,
                started: now,
                recoil,
            },
        );
    }

    pub fn offset(&self, entity: EntityId, now: f64, reduced_motion: bool) -> Vec2 {
        if reduced_motion {
            return Vec2::ZERO;
        }
        self.motions.get(&entity).map_or(Vec2::ZERO, |motion| {
            motion_offset(motion.direction, now - motion.started, motion.recoil)
        })
    }

    pub fn prune(&mut self, now: f64) {
        self.motions.retain(|_, motion| now - motion.started < 0.18);
    }
    pub fn clear(&mut self) {
        self.motions.clear();
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct CombatResidue {
    pub cells: BTreeMap<GridPos, ResidueCell>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ResidueCell {
    pub blood: u8,
    pub wall: bool,
    pub seed: u32,
    pub corpse: Option<u8>,
    pub corpse_symbol: Option<char>,
}

impl CombatResidue {
    pub fn hit(
        &mut self,
        at: GridPos,
        amount: u16,
        seed: u64,
        map: &Map,
        visibility: &VisibilityState,
    ) {
        if amount == 0 || !visibility.is_visible(at) {
            return;
        }
        let cell = self.cells.entry(at).or_default();
        cell.blood = cell.blood.max(cell.blood.saturating_add(1).min(5));
        cell.seed = seed as u32;
        for next in at.cardinal_neighbors() {
            if visibility.is_visible(next)
                && let Some(tile) = map.tile(next)
            {
                // A projection stops on the adjacent surface, never across it.
                let cell = self.cells.entry(next).or_default();
                cell.wall = tile.terrain.blocks_movement();
                cell.blood = cell.blood.max(cell.blood.saturating_add(1).min(3));
                cell.seed = seed as u32 ^ next.x as u32 ^ (next.y as u32).rotate_left(13);
            }
        }
    }

    pub fn death(&mut self, at: GridPos, sprite: u8, seed: u64, visibility: &VisibilityState) {
        if !visibility.is_visible(at) {
            return;
        }
        let cell = self.cells.entry(at).or_default();
        cell.corpse = Some(sprite);
        cell.corpse_symbol = None;
        cell.blood = if crate::game_art::actor_bleeds(sprite) {
            6
        } else {
            0
        };
        cell.seed = seed as u32;
        cell.wall = false;
    }

    pub fn death_symbol(
        &mut self,
        at: GridPos,
        symbol: char,
        seed: u64,
        visibility: &VisibilityState,
    ) {
        if !visibility.is_visible(at) {
            return;
        }
        let cell = self.cells.entry(at).or_default();
        cell.corpse = None;
        cell.corpse_symbol = Some(symbol);
        cell.blood = 6;
        cell.seed = seed as u32;
        cell.wall = false;
    }
}

pub(crate) fn draw_residue(rect: Rect, cell: &ResidueCell, visible: bool) {
    let px = rect.w / 32.0;
    let alpha = if visible { 0.90 } else { 0.30 };
    // The approved corpse already contains its pool, or is a dry machine wreck.
    // Keep saved actor indices; select the authored drawing in the renderer.
    if let Some(sprite) = cell
        .corpse
        .filter(|sprite| crate::game_art::has_authored_corpse(*sprite))
    {
        crate::game_art::draw_corpse(rect, sprite, alpha);
        return;
    }
    if cell.blood == 0 && cell.corpse.is_none() && cell.corpse_symbol.is_none() {
        return;
    }
    let dark = Color::new(0.25, 0.025, 0.04, alpha);
    let red = Color::new(0.49, 0.045, 0.055, alpha);
    let mut seed = cell.seed.wrapping_add(17);
    let mut random = || {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        seed
    };
    if !cell.wall {
        let width = if cell.corpse.is_some() || cell.corpse_symbol.is_some() {
            24.0
        } else {
            6.0 + cell.blood as f32 * 2.0
        };
        for (y, w) in [(21.0, width - 6.0), (23.0, width), (26.0, width - 4.0)] {
            draw_rectangle(
                rect.x + (32.0 - w) * 0.5 * px,
                rect.y + y * px,
                w * px,
                3.0 * px,
                dark,
            );
        }
    }
    for _ in 0..(4 + cell.blood * 3) {
        let x = 3 + random() % 26;
        let y = if cell.wall {
            8 + random() % 17
        } else {
            16 + random() % 14
        };
        let width = 1 + random() % 3;
        draw_rectangle(
            rect.x + x as f32 * px,
            rect.y + y as f32 * px,
            width as f32 * px,
            if cell.wall {
                (2 + random() % 4) as f32 * px
            } else {
                px
            },
            red,
        );
    }
    if let Some(sprite) = cell.corpse {
        crate::game_art::draw_corpse(rect, sprite, alpha);
    } else if let Some(symbol) = cell.corpse_symbol {
        crate::terminal_view::draw_corpse_symbol(rect, symbol, alpha);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::Terrain;

    #[test]
    fn harmful_status_includes_non_damaging_penalties_but_excludes_guards_and_counters() {
        use project_rl::status::{StatusDefinition, StatusModifier, StatusStacking};
        let make = |modifiers: Vec<StatusModifier>| {
            StatusDefinition::new(
                "test:feedback".parse().unwrap(),
                Some(3),
                StatusStacking::KeepExisting,
                vec![],
            )
            .unwrap()
            .with_modifiers(modifiers)
            .unwrap()
        };
        for modifier in [
            StatusModifier::ArmorFragilization { amount: 4 },
            StatusModifier::MovementTimeMinimum { time_units: 2 },
            StatusModifier::Accuracy { amount: -20 },
            StatusModifier::Stability { amount: -3 },
        ] {
            assert!(harmful_status(&make(vec![modifier])));
        }
        for modifiers in [
            vec![],
            vec![StatusModifier::DamageGuard { amount: 15 }],
            vec![StatusModifier::Accuracy { amount: 20 }],
            vec![StatusModifier::Stability { amount: 3 }],
        ] {
            assert!(!harmful_status(&make(modifiers)));
        }
    }

    #[test]
    fn harmful_status_includes_self_damage_but_not_reflected_damage_on_someone_else() {
        use project_rl::status::{
            StatusDefinition, StatusEffectPrimitive, StatusHook, StatusStacking, StatusTrigger,
        };
        let packet =
            project_rl::combat::DamagePacket::new(3, project_rl::combat::DamageType::Chemical, 0);
        for (effect, expected) in [
            (
                StatusEffectPrimitive::DealDamage {
                    packet,
                    multiply_by_stacks: false,
                },
                true,
            ),
            (
                StatusEffectPrimitive::DealDamageToCounterpart {
                    packet,
                    multiply_by_stacks: false,
                },
                false,
            ),
        ] {
            let status = StatusDefinition::new(
                "test:feedback".parse().unwrap(),
                Some(3),
                StatusStacking::KeepExisting,
                vec![StatusHook::new(StatusTrigger::TurnStart, vec![effect])],
            )
            .unwrap();
            assert_eq!(harmful_status(&status), expected);
        }
    }

    #[test]
    fn shared_reaction_returns_to_rest_and_supports_self_inflicted_effects() {
        for source in [
            vec2(64.0, 0.0),
            vec2(-64.0, 0.0),
            vec2(0.0, 64.0),
            vec2(0.0, -64.0),
            Vec2::ZERO,
        ] {
            let peak = hit_reaction_offset(Vec2::ZERO, source, 0.0504, 64.0);
            assert!((peak.length() - 10.88).abs() < 0.01);
            assert_eq!(
                hit_reaction_offset(Vec2::ZERO, source, -0.01, 64.0),
                Vec2::ZERO
            );
            assert_eq!(
                hit_reaction_offset(Vec2::ZERO, source, 0.18, 64.0),
                Vec2::ZERO
            );
        }
    }

    #[test]
    fn motion_extends_and_returns_without_leaving_a_residual_offset() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        motion.play(id, GridPos::new(2, 2), GridPos::new(3, 2), false, 1.0);
        assert!(motion.offset(id, 1.05, false).x > 0.2);
        assert_eq!(motion.offset(id, 1.2, false), Vec2::ZERO);
        motion.play(id, GridPos::new(2, 2), GridPos::new(3, 2), true, 2.0);
        assert!(motion.offset(id, 2.05, false).x < -0.15);
        assert_eq!(motion.offset(id, 2.05, true), Vec2::ZERO);
    }

    #[test]
    fn blood_requires_positive_visible_damage_and_corpse_survives_serialization() {
        let mut map = Map::filled(7, 7, Terrain::Floor).unwrap();
        map.set_terrain(GridPos::new(4, 3), Terrain::Wall).unwrap();
        let mut visibility = VisibilityState::default();
        visibility.recompute(
            &map,
            GridPos::new(3, 3),
            project_rl::world::FieldOfViewRules {
                radius: 1,
                ..Default::default()
            },
        );
        let mut residue = CombatResidue::default();
        residue.hit(GridPos::new(1, 1), 5, 1, &map, &visibility);
        residue.hit(GridPos::new(3, 3), 0, 1, &map, &visibility);
        assert!(residue.cells.is_empty());
        residue.hit(GridPos::new(3, 3), 5, 1, &map, &visibility);
        assert_eq!(residue.cells.len(), 5);
        assert!(!residue.cells.contains_key(&GridPos::new(5, 3)));
        assert!(residue.cells[&GridPos::new(4, 3)].wall);
        residue.death(GridPos::new(3, 3), 4, 2, &visibility);
        residue.hit(GridPos::new(3, 3), 5, 3, &map, &visibility);
        assert_eq!(residue.cells[&GridPos::new(3, 3)].blood, 6);
        assert_eq!(
            residue,
            bincode::deserialize(&bincode::serialize(&residue).unwrap()).unwrap()
        );
    }
}
