//! Stable campaign appearance assignments, independent of combat/placement RNG.
//! Existing actor tags persist the selected native model and palette.
use crate::entity::Actor;
use crate::social::PlayerRelation;

const PREFIX: &str = "core:monster_art_";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtRole {
    CultistMelee,
    CultistRanged,
    PossessedMelee,
    PossessedRanged,
    DemonMelee,
    DemonRanged,
    Spirit,
    ReservedCaster,
}

#[derive(Clone, Copy, Debug)]
pub struct MonsterArt {
    pub number: u16,
    pub variant: u8,
    pub side: u16,
    pub index: u16,
    pub foot: [u16; 2],
    pub shadow: u16,
    pub role: ArtRole,
    pub name: &'static str,
}

include!("../assets/sprites/campaign-monsters/catalog.rs");

pub fn art(number: u16, variant: u8) -> Option<&'static MonsterArt> {
    MONSTER_ART
        .iter()
        .find(|e| e.number == number && e.variant == variant)
        .or_else(|| {
            crate::monster_palettes::palette(number, variant)
                .is_some()
                .then(|| {
                    MONSTER_ART
                        .iter()
                        .find(|e| e.number == number && e.variant == 0)
                })
                .flatten()
        })
}

pub fn appearance(actor: &Actor) -> Option<(u16, u8)> {
    actor.tags().iter().find_map(|tag| {
        let (number, variant) = tag.as_str().strip_prefix(PREFIX)?.split_once('_')?;
        let key = (number.parse().ok()?, variant.parse().ok()?);
        art(key.0, key.1).map(|_| key)
    })
}

pub fn display_name(actor: &Actor) -> Option<&'static str> {
    let (number, variant) = appearance(actor)?;
    art(number, variant).map(|e| e.name)
}

/// Generation 150 keeps its original choices exactly. Generation 151 adds the
/// reviewed warm/green palettes, generation 152 the blues. Sparse ids such as
/// 156: A/C/D remain stable; old campaigns keep their original choices.
pub fn variant_ids(number: u16, expanded: bool, blue: bool) -> Vec<u8> {
    let mut variants: Vec<_> = MONSTER_ART
        .iter()
        .filter(|e| e.number == number)
        .map(|e| e.variant)
        .collect();
    if matches!(number, 144 | 150)
        || (expanded && crate::monster_palettes::EXTENDED_PALETTES_APPROVED)
    {
        variants.extend(
            crate::monster_palettes::MATERIAL_PALETTES
                .iter()
                .filter(|p| p.number == number)
                .map(|p| p.variant),
        );
    }
    if blue && crate::monster_palettes::BLUE_PALETTES_APPROVED {
        variants.extend(
            crate::monster_palettes::BLUE_MATERIAL_PALETTES
                .iter()
                .filter(|p| p.number == number)
                .map(|p| p.variant),
        );
    }
    variants.sort_unstable();
    variants.dedup();
    variants
}

/// Only newly generated hostile bodies are assigned. AI, HP, equipment,
/// perception, loot and action timing remain exactly those of their role.
pub fn assign(
    actors: &mut [Actor],
    depth: u16,
    seed: u64,
    weapon_hands: impl Fn(&Actor) -> Option<bool>,
    expanded_palettes: bool,
    blue_palettes: bool,
) {
    for actor in actors {
        if appearance(actor).is_some()
            || actor.player_relation() != PlayerRelation::Hostile
            || actor.drone().is_some()
            || actor.ai().is_none()
            || actor
                .tags()
                .iter()
                .any(|tag| tag.as_str() == "core:sentinel_projector")
        {
            continue;
        }
        let ranged = actor.attacks().iter().any(|a| a.range() > 1);
        let role = match (depth, ranged) {
            (0..=1, false) => ArtRole::CultistMelee,
            (0..=1, true) => ArtRole::CultistRanged,
            (2..=3, false) => ArtRole::PossessedMelee,
            (2..=3, true) => ArtRole::PossessedRanged,
            (_, false) => ArtRole::DemonMelee,
            (_, true) => ArtRole::DemonRanged,
        };
        let hands = weapon_hands(actor);
        let pool: Vec<_> = MONSTER_ART
            .iter()
            .filter(|entry| {
                entry.variant == 0
                    && (hands.is_none()
                        || !matches!(role, ArtRole::CultistMelee | ArtRole::CultistRanged)
                        || if hands == Some(true) {
                            matches!(entry.number, 164 | 165)
                        } else {
                            !matches!(entry.number, 164 | 165)
                        })
                    && (entry.role == role
                        || (role == ArtRole::DemonMelee && entry.role == ArtRole::Spirit))
            })
            .collect();
        if pool.is_empty() {
            continue;
        }
        let at = actor.position();
        let mut value =
            seed ^ (u64::from(at.x as u32) << 32) ^ u64::from(at.y as u32) ^ 0x4d4f_4e53_5445_5231;
        value ^= value >> 30;
        value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value ^= value >> 27;
        let entry = pool[value as usize % pool.len()];
        let variants = variant_ids(entry.number, expanded_palettes, blue_palettes);
        let variant = variants[(value.rotate_left(19) % variants.len() as u64) as usize];
        let mut tags = vec![
            format!("{PREFIX}{}_{}", entry.number, variant)
                .parse()
                .unwrap(),
        ];
        if entry.role == ArtRole::Spirit {
            tags.push(
                crate::monster_modifiers::PHYSICAL_RESISTANCE_TAG
                    .parse()
                    .unwrap(),
            );
        }
        *actor = actor.clone().with_tags(tags);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{AiBehavior, AiProfile};
    use crate::combat::{AttackProfile, DamageType};
    use crate::world::GridPos;

    #[test]
    fn campaign_bank_excludes_refused_and_deferred_boss_models() {
        for number in [13, 18, 19, 23, 27, 91, 163] {
            assert!(art(number, 0).is_none());
        }
        assert_eq!(MONSTER_ART.iter().filter(|e| e.variant == 0).count(), 129);
        assert_eq!(art(156, 0).unwrap().side, 128);
        assert!(art(144, 4).is_none());
        for entry in MONSTER_ART {
            assert!(entry.foot[0] <= entry.side && entry.foot[1] <= entry.side);
        }
    }

    #[test]
    fn assignment_is_persistent_and_preserves_combat_and_placement() {
        let original = Actor::new(GridPos::new(4, 5), 50)
            .unwrap()
            .with_attack(AttackProfile::melee(DamageType::Kinetic, 20))
            .with_ai(AiProfile::new(AiBehavior::Hunter, 8, 0, 512, 0))
            .with_player_relation(PlayerRelation::Hostile);
        let mut actors = [original.clone()];
        assign(&mut actors, 0, 123, |_| Some(true), true, true);
        assert_eq!(appearance(&actors[0]), Some((165, 0)));
        assert_eq!(display_name(&actors[0]), Some("Cultiste"));
        assert_eq!(actors[0].attacks(), original.attacks());
        assert_eq!(actors[0].ai(), original.ai());
        assert_eq!(actors[0].maximum_integrity(), 50);
        let bytes = bincode::serialize(&actors[0]).unwrap();
        let mut restored = [bincode::deserialize(&bytes).unwrap()];
        assign(&mut restored, 7, 999, |_| None, true, true);
        assert_eq!(restored, actors);
        let mut unchanged = [original.with_player_relation(PlayerRelation::Neutral)];
        assign(&mut unchanged, 7, 123, |_| None, true, true);
        assert!(appearance(&unchanged[0]).is_none());
    }

    #[test]
    fn generation_150_keeps_frozen_appearances_and_new_palette_ids_persist() {
        let baseline: Vec<Vec<Option<(u16,u8)>>> = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07/generation-150-appearance-baseline.json"
        )).unwrap();
        let mut new_palettes = 0;
        let baseline151: Vec<Vec<Option<(u16,u8)>>> = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/monster-material-palettes-07/generation-151-appearance-baseline.json"
        )).unwrap();
        let mut blue_palettes = 0;
        for depth in 0..8 {
            let mut legacy: Vec<_> = (0..96)
                .map(|index| {
                    Actor::new(GridPos::new(index % 12, index / 12), 50)
                        .unwrap()
                        .with_ai(AiProfile::idle())
                        .with_player_relation(PlayerRelation::Hostile)
                        .with_attack(AttackProfile::melee(DamageType::Kinetic, 20))
                })
                .collect();
            let mut expanded = legacy.clone();
            let mut version151 = legacy.clone();
            let seed = 0xface_d00d + u64::from(depth);
            assign(&mut legacy, depth, seed, |_| None, false, false);
            assert_eq!(
                legacy.iter().map(appearance).collect::<Vec<_>>(),
                baseline[depth as usize]
            );
            assign(&mut version151, depth, seed, |_| None, true, false);
            assert_eq!(
                version151.iter().map(appearance).collect::<Vec<_>>(),
                baseline151[depth as usize]
            );
            assign(&mut expanded, depth, seed, |_| None, true, true);
            for (old, new) in legacy.iter().zip(&expanded) {
                let (old_number, old_variant) = appearance(old).unwrap();
                let (number, variant) = appearance(new).unwrap();
                blue_palettes += usize::from(variant == 4);
                assert_eq!(number, old_number);
                assert_eq!(old.position(), new.position());
                assert_eq!(old.attacks(), new.attacks());
                assert_eq!(old.ai(), new.ai());
                assert_eq!(old.maximum_integrity(), new.maximum_integrity());
                assert_eq!(
                    crate::monster_modifiers::visible_modifiers(old),
                    crate::monster_modifiers::visible_modifiers(new)
                );
                if variant >= 2 && !matches!(number, 144 | 150) {
                    assert!(old_variant < 2);
                    new_palettes += 1;
                    let bytes = bincode::serialize(new).unwrap();
                    let mut restored = [bincode::deserialize(&bytes).unwrap()];
                    assign(&mut restored, 7, 999, |_| None, false, false);
                    assert_eq!(&restored[0], new);
                }
            }
        }
        assert!(new_palettes > 100);
        assert!(blue_palettes > 20);
        assert_eq!(variant_ids(156, false, false), vec![0]);
        assert_eq!(variant_ids(156, true, true), vec![0, 2, 3]);
        assert!(art(156, 1).is_none());
    }
}
