//! Combat presentation. No simulation RNG, movement, time or collision changes.
use std::collections::BTreeMap;

use macroquad::prelude::*;
use project_rl::entity::EntityId;
use project_rl::world::{GridPos, Map, VisibilityState};
#[path = "agent_device_visuals.rs"]
mod agent_devices;
#[path = "agent_ember_visuals.rs"]
pub(crate) mod agent_ember;
#[path = "agent_fire_visuals.rs"]
mod agent_fire;
#[path = "agent_magic_visuals.rs"]
mod agent_magic;
#[path = "agent_spell_visuals.rs"]
mod agent_spells;
#[path = "agent_support_visuals.rs"]
mod agent_support;
#[path = "agent_weapon_visuals.rs"]
mod agent_weapons;

/// An accepted compact picture replaces the generic timer badge for this status.
pub(crate) fn has_agent_status_icon(id: &str) -> bool {
    agent_support::has_status_icon(id) || id == "agent:provoked"
}

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
#[cfg(any(debug_assertions, test))]
pub(crate) fn hit_reaction_offset(at: Vec2, source: Vec2, age: f64, tile: f32) -> Vec2 {
    let direction = (source - at).normalize_or_zero();
    let direction = if direction == Vec2::ZERO {
        vec2(0.0, -1.0)
    } else {
        direction
    };
    motion_offset(direction, age, true) * tile
}

/// Empty control definitions still share the reviewed harmful-status reaction.
pub(crate) fn harmful_status(definition: &project_rl::status::StatusDefinition) -> bool {
    use project_rl::status::{StatusEffectPrimitive, StatusModifier, StatusTrigger};
    matches!(
        definition.id().as_str(),
        "agent:provoked"
            | "agent:frozen"
            | "agent:mine_frozen"
            | "agent:armor_breach"
            | "agent:silence"
            | "agent:regeneration_break"
            | "agent:disorientation"
            | "agent:interference"
            | "agent:domination"
            | "agent:stunned"
    ) || definition
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

/// Offset from the already committed destination, in world tiles. Never saved.
#[derive(Clone, Debug)]
struct TileGlide {
    offset: Vec2,
    destination: GridPos,
    started: f64,
}

impl TileGlide {
    const SECONDS: f64 = 0.080;

    fn offset(&self, now: f64) -> Vec2 {
        let remaining = (1.0 - (now - self.started) / Self::SECONDS).clamp(0.0, 1.0) as f32;
        self.offset * remaining * remaining
    }
}

#[derive(Clone, Debug)]
struct RuptureStrike {
    origin: GridPos,
    target: GridPos,
    started: f64,
}

#[derive(Default, Debug)]
pub(crate) struct CombatMotion {
    motions: BTreeMap<EntityId, Motion>,
    glides: BTreeMap<EntityId, TileGlide>,
    travels: BTreeMap<EntityId, (Vec2, f64)>,
    spells: Vec<agent_spells::SpellFlight>,
    blasts: Vec<agent_spells::SpellBlast>,
    fireballs: Vec<agent_fire::Fireball>,
    embers: Vec<agent_ember::Flight>,
    magic: Vec<agent_magic::MagicFlight>,
    projectile_impacts: BTreeMap<EntityId, f64>,
    strikes: Vec<RuptureStrike>,
    taunts: Vec<(GridPos, f64)>,
}

impl CombatMotion {
    pub fn glide(&mut self, entity: EntityId, from: GridPos, to: GridPos, now: f64) {
        let delta = vec2((from.x - to.x) as f32, (from.y - to.y) as f32);
        // Long transfers and skills with their own travel retain that presentation.
        if delta == Vec2::ZERO
            || delta.abs().max_element() > 1.0
            || self
                .travels
                .get(&entity)
                .is_some_and(|(_, started)| now - started < 0.18)
        {
            self.glides.remove(&entity);
            return;
        }
        let previous = self
            .glides
            .get(&entity)
            .filter(|glide| glide.destination == from)
            .map_or(Vec2::ZERO, |glide| glide.offset(now));
        self.glides.insert(
            entity,
            TileGlide {
                offset: delta + previous,
                destination: to,
                started: now,
            },
        );
    }

    pub fn stop_glide(&mut self, entity: EntityId) {
        self.glides.remove(&entity);
    }

    pub fn glide_offset(&self, entity: EntityId, now: f64, reduced_motion: bool) -> Vec2 {
        if reduced_motion {
            Vec2::ZERO
        } else {
            self.glides
                .get(&entity)
                .map_or(Vec2::ZERO, |glide| glide.offset(now))
        }
    }

    pub fn actor_rect(&self, entity: EntityId, rect: Rect, now: f64, reduced_motion: bool) -> Rect {
        let offset = self.offset(entity, now, reduced_motion) * rect.w;
        Rect::new(rect.x + offset.x, rect.y + offset.y, rect.w, rect.h)
    }

    pub fn play_agent_travel(
        &mut self,
        entity: EntityId,
        origin: GridPos,
        target: GridPos,
        now: f64,
    ) {
        self.glides.remove(&entity);
        self.travels.insert(
            entity,
            (
                vec2((origin.x - target.x) as f32, (origin.y - target.y) as f32),
                now,
            ),
        );
    }
    pub fn delay_agent_magic_impact(&mut self, target: EntityId, at: f64) {
        self.projectile_impacts.insert(target, at);
    }
    pub fn play_agent_magic(
        &mut self,
        skill: &str,
        origin: GridPos,
        target: GridPos,
        radius: u16,
        now: f64,
    ) {
        if self.magic.len() >= 128 {
            self.magic.remove(0);
        }
        self.magic.push(agent_magic::MagicFlight {
            skill: skill.into(),
            origin,
            target,
            radius,
            started: now,
        });
    }
    pub fn draw_agent_world(
        &self,
        game: &project_rl::game::GameState,
        now: f64,
        reduced: bool,
        background: bool,
        clip: Rect,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        agent_magic::draw(game, &self.magic, now, reduced, background, &rect);
        agent_devices::draw(game, now, reduced, background, &rect);
        let actor_rect = |entity, at| rect(at).map(|r| self.actor_rect(entity, r, now, reduced));
        agent_weapons::draw(game, now, reduced, background, &actor_rect);
        agent_support::draw(
            game,
            &self.magic,
            now,
            reduced,
            background,
            &rect,
            &actor_rect,
        );
        for flight in &self.magic {
            agent_weapons::burst(
                &flight.skill,
                flight.origin,
                flight.target,
                flight.radius,
                (now - flight.started) as f32,
                game.map(),
                background,
                &rect,
                clip,
            );
            if matches!(
                flight.skill.as_str(),
                "SG00" | "SG01" | "SG02" | "SM00" | "SM02" | "SM03" | "HY07" | "DR01" | "DP01"
            ) {
                agent_devices::burst(
                    &flight.skill,
                    flight.origin,
                    flight.target,
                    flight.radius,
                    (now - flight.started) as f32,
                    game.map(),
                    background,
                    &rect,
                );
            }
        }
    }
    pub fn draw_gel(
        &self,
        entity: EntityId,
        rect: Rect,
        now: f64,
        reduced: bool,
        background: bool,
    ) {
        agent_magic::draw_gel(rect, now, entity.get(), reduced, background);
    }
    pub fn play_ember(&mut self, victim: EntityId, origin: GridPos, target: GridPos, now: f64) {
        if self.embers.len() == 32 {
            self.embers.remove(0);
        }
        self.embers.push(agent_ember::Flight {
            origin,
            target,
            started: now,
        });
        self.projectile_impacts
            .insert(victim, now + agent_ember::FLIGHT_SECONDS);
    }
    pub fn play_fireball(&mut self, origin: GridPos, target: GridPos, radius: u16, now: f64) {
        if self.fireballs.len() == 32 {
            self.fireballs.remove(0);
        }
        self.fireballs.push(agent_fire::Fireball {
            origin,
            target,
            radius,
            started: now,
        });
    }
    pub fn begin_event_batch(&mut self) {
        self.projectile_impacts.clear();
    }
    pub fn impact_time(&self, target: EntityId, now: f64) -> f64 {
        self.projectile_impacts
            .get(&target)
            .copied()
            .unwrap_or(now)
            .max(now)
    }
    pub fn fireball_hit_after_flight(&mut self, target: EntityId, now: f64) {
        self.projectile_impacts.insert(target, now + 0.35);
    }
    pub fn draw_fireballs(
        &self,
        map: &Map,
        now: f64,
        reduced: bool,
        background: bool,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        agent_fire::draw(&self.fireballs, map, now, reduced, background, rect);
    }
    pub fn draw_burn(
        &self,
        entity: EntityId,
        rect: Rect,
        now: f64,
        reduced: bool,
        background: bool,
    ) {
        if reduced
            || self
                .projectile_impacts
                .get(&entity)
                .is_none_or(|at| now >= *at)
        {
            agent_fire::burn(rect, now, entity.get(), reduced, background);
        }
    }
    pub fn play_blast(&mut self, centre: GridPos, radius: u16, now: f64) {
        if self.blasts.len() == 32 {
            self.blasts.remove(0);
        }
        self.blasts.push(agent_spells::SpellBlast {
            centre,
            radius,
            started: now,
        });
    }
    pub fn draw_blasts(
        &self,
        map: &Map,
        now: f64,
        reduced: bool,
        background: bool,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        agent_spells::draw_blasts(&self.blasts, map, now, reduced, background, rect);
    }
    pub fn play_taunt(&mut self, origin: GridPos, now: f64) {
        if self.taunts.len() == 32 {
            self.taunts.remove(0);
        }
        self.taunts.push((origin, now));
    }
    pub fn draw_taunts(
        &self,
        map: &Map,
        now: f64,
        reduced: bool,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        if reduced {
            return;
        }
        for &(at, started) in &self.taunts {
            crate::guardian_taunt_visuals::pulse(map, at, (now - started) as f32, &rect);
        }
    }
    pub fn play_rupture(&mut self, origin: GridPos, target: GridPos, started: f64) {
        if self.strikes.len() == 32 {
            self.strikes.remove(0);
        }
        self.strikes.push(RuptureStrike {
            origin,
            target,
            started,
        });
    }
    pub fn play_spell(&mut self, victim: EntityId, origin: GridPos, target: GridPos, started: f64) {
        if self.spells.len() == 32 {
            self.spells.remove(0);
        }
        self.spells.push(agent_spells::SpellFlight {
            victim,
            origin,
            target,
            started,
        });
    }
    pub fn impact_time_for(&self, victim: EntityId, at: GridPos, now: f64) -> f64 {
        self.spells
            .iter()
            .rev()
            .find(|s| s.victim == victim && now - s.started <= 0.18)
            .map(|s| s.started + 0.18)
            .or_else(|| {
                self.strikes
                    .iter()
                    .rev()
                    .find(|s| s.target == at && now - s.started <= 0.22)
                    .map(|s| s.started + 0.22)
            })
            .unwrap_or(now)
    }
    pub fn draw_spells(&self, now: f64, reduced: bool, rect: impl Fn(GridPos) -> Option<Rect>) {
        agent_spells::draw(&self.spells, now, reduced, &rect);
        agent_ember::draw(&self.embers, now, reduced, &rect);
        for strike in &self.strikes {
            let age = (now - strike.started) as f32;
            if !(0.0..0.84).contains(&age) {
                continue;
            }
            let (Some(from), Some(to)) = (rect(strike.origin), rect(strike.target)) else {
                continue;
            };
            let d = (to.center() - from.center()).normalize_or_zero();
            let normal = vec2(-d.y, d.x);
            let tile = from.w;
            if !reduced && (0.05..0.57).contains(&age) {
                let t = ((age - 0.05) / 0.20).clamp(0., 1.);
                let phase = t * t * (3. - 2. * t);
                let start = from.center() + d * (tile * 0.34) - normal * (tile * 0.62);
                let end = to.center() + d * (tile * 0.36) + normal * (tile * 0.42);
                let life = (1. - (age - 0.22).max(0.) / 0.35).clamp(0., 1.);
                crate::melee_strike_visuals::cut(start, start.lerp(end, phase), tile * 0.28, life);
            }
            crate::melee_strike_visuals::impact(
                to.center(),
                if reduced { 0.10 } else { age - 0.22 },
                1.15,
                false,
                tile,
            );
        }
    }
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
        let now = if recoil {
            self.projectile_impacts
                .get(&entity)
                .copied()
                .unwrap_or(now)
                .max(now)
        } else {
            now
        };
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
        self.glide_offset(entity, now, false)
            + self.motions.get(&entity).map_or(Vec2::ZERO, |motion| {
                motion_offset(motion.direction, now - motion.started, motion.recoil)
            })
            + self
                .travels
                .get(&entity)
                .map_or(Vec2::ZERO, |(offset, started)| {
                    *offset * (1. - ((now - *started) / 0.18).clamp(0., 1.) as f32)
                })
    }

    pub fn prune(&mut self, now: f64) {
        self.glides
            .retain(|_, glide| now - glide.started < TileGlide::SECONDS);
        self.travels.retain(|_, (_, started)| now - *started < 0.18);
        self.fireballs.retain(|s| now - s.started < 0.90);
        self.magic.retain(|s| now - s.started < 1.2);
        self.embers
            .retain(|s| now - s.started < agent_ember::LIFETIME_SECONDS);
        self.projectile_impacts.retain(|_, at| now - *at < 0.30);
        self.taunts.retain(|(_, started)| now - started < 0.48);
        self.blasts.retain(|s| now - s.started < 0.55);
        self.strikes.retain(|s| now - s.started < 0.84);
        self.spells.retain(|s| now - s.started < 0.4);
        self.motions.retain(|_, motion| now - motion.started < 0.18);
    }
    pub fn clear(&mut self) {
        self.glides.clear();
        self.fireballs.clear();
        self.embers.clear();
        self.magic.clear();
        self.projectile_impacts.clear();
        self.taunts.clear();
        self.blasts.clear();
        self.strikes.clear();
        self.spells.clear();
        self.motions.clear();
        self.travels.clear();
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
    let native_corpse = cell
        .corpse_symbol
        .and_then(crate::game_art::campaign::decode_corpse);
    if let Some((number, variant)) = native_corpse.filter(|_| cell.blood == 0) {
        crate::game_art::campaign::draw_corpse(rect, number, variant, alpha);
        return;
    }
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
    if let Some((number, variant)) = native_corpse {
        crate::game_art::campaign::draw_corpse(rect, number, variant, alpha);
        return;
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
    fn tile_glide_moves_the_whole_actor_and_rests_after_eighty_milliseconds() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let from = GridPos::new(10, 10);
        for to in [
            GridPos::new(11, 10),
            GridPos::new(9, 10),
            GridPos::new(10, 9),
            GridPos::new(10, 11),
        ] {
            let mut motion = CombatMotion::default();
            motion.glide(id, from, to, 1.0);
            let delta = vec2((from.x - to.x) as f32, (from.y - to.y) as f32);
            assert_eq!(motion.offset(id, 1.0, false), delta);
            assert!((motion.offset(id, 1.04, false) - delta * 0.25).length() < 0.0001);
            for tile in [32., 64.] {
                let destination = Rect::new(to.x as f32 * tile, to.y as f32 * tile, tile, tile);
                let start = motion.actor_rect(id, destination, 1.0, false);
                assert_eq!(
                    start.point(),
                    vec2(from.x as f32 * tile, from.y as f32 * tile)
                );
                assert_eq!(motion.actor_rect(id, destination, 1.08, false), destination);
                assert_eq!(motion.actor_rect(id, destination, 1.02, true), destination);
            }
            motion.prune(1.08);
            assert!(motion.glides.is_empty());
        }
    }

    #[test]
    fn a_new_step_retargets_the_displayed_position_without_a_queue_or_a_snap() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        let a = GridPos::new(10, 10);
        let b = GridPos::new(11, 10);
        let c = GridPos::new(11, 9);
        motion.glide(id, a, b, 1.0);
        let shown = vec2(b.x as f32, b.y as f32) + motion.offset(id, 1.02, false);
        motion.glide(id, b, c, 1.02);
        assert!(
            (vec2(c.x as f32, c.y as f32) + motion.offset(id, 1.02, false) - shown).length()
                < 0.0001
        );
        assert_eq!(motion.offset(id, 1.11, false), Vec2::ZERO);
        assert_eq!(motion.glides.len(), 1);
        motion.play(id, c, b, true, 1.02);
        assert_ne!(
            motion.offset(id, 1.05, false),
            motion.glide_offset(id, 1.05, false)
        );
        motion.clear();
        assert_eq!(motion.offset(id, 1.05, false), Vec2::ZERO);
    }

    #[test]
    fn tile_glides_do_not_replace_skill_travel_or_teleports() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        let a = GridPos::new(10, 10);
        let b = GridPos::new(11, 10);
        motion.glide(id, a, b, 1.0);
        motion.play_agent_travel(id, a, b, 1.01);
        assert_eq!(motion.glide_offset(id, 1.02, false), Vec2::ZERO);
        motion.glide(id, a, b, 1.02);
        assert_eq!(motion.glide_offset(id, 1.03, false), Vec2::ZERO);
        motion.prune(2.0);
        motion.glide(id, b, GridPos::new(15, 10), 2.0);
        assert_eq!(motion.offset(id, 2.0, false), Vec2::ZERO);
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
