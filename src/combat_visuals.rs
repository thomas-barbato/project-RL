//! Combat presentation. No simulation RNG, movement, time or collision changes.
use std::collections::{BTreeMap, BTreeSet};

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
    let t = (age / (0.18 * crate::animation_policy::duration_scale("reaction"))) as f32;
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
    seconds: f64,
    support: u8,
}

impl TileGlide {
    fn progress(&self, now: f64) -> f64 {
        if now >= self.started + self.seconds {
            1.
        } else {
            ((now - self.started) / self.seconds).clamp(0., 1.)
        }
    }

    fn offset(&self, now: f64) -> Vec2 {
        let remaining = (1.0 - self.progress(now)) as f32;
        self.offset * remaining
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
    walk_facings: BTreeMap<EntityId, i8>,
    walk_vertical_facings: BTreeMap<EntityId, i8>,
    walk_lateral_facings: BTreeMap<EntityId, i8>,
    walk_supports: BTreeMap<EntityId, u8>,
    continuing_walks: BTreeSet<EntityId>,
    travels: BTreeMap<EntityId, (Vec2, f64)>,
    spells: Vec<agent_spells::SpellFlight>,
    blasts: Vec<agent_spells::SpellBlast>,
    fireballs: Vec<agent_fire::Fireball>,
    embers: Vec<agent_ember::Flight>,
    prepared_cast: Option<crate::effect_geometry::PreparedCast>,
    magic: Vec<agent_magic::MagicFlight>,
    projectile_impacts: BTreeMap<EntityId, f64>,
    strikes: Vec<RuptureStrike>,
    taunts: Vec<(GridPos, f64)>,
}

impl CombatMotion {
    /// Keep the last moving pose across the input/render boundary between steps.
    /// This intent never advances an actor or starts an uncommitted step.
    pub fn continue_walk(&mut self, entity: EntityId, continuing: bool) {
        if continuing {
            self.continuing_walks.insert(entity);
        } else {
            self.continuing_walks.remove(&entity);
        }
    }
    /// Resource preparation precedes first display and must not consume a step's
    /// visible animation time. These timestamps are presentation-only.
    pub fn delay_glides(&mut self, seconds: f64) {
        if seconds > 0. {
            for glide in self.glides.values_mut() {
                glide.started += seconds;
            }
        }
    }
    #[cfg(any(debug_assertions, test))]
    pub fn glide(&mut self, entity: EntityId, from: GridPos, to: GridPos, now: f64) {
        self.glide_timed(entity, from, to, now, 120);
    }

    pub fn glide_timed(
        &mut self,
        entity: EntityId,
        from: GridPos,
        to: GridPos,
        now: f64,
        milliseconds: u16,
    ) {
        let delta = vec2((from.x - to.x) as f32, (from.y - to.y) as f32);
        // Long transfers and skills with their own travel retain that presentation.
        if delta == Vec2::ZERO
            || delta.abs().max_element() > 1.0
            || self.travels.get(&entity).is_some_and(|(_, started)| {
                crate::animation_policy::age("travel", now, *started) < 0.18
            })
        {
            self.glides.remove(&entity);
            self.walk_facings.remove(&entity);
            self.walk_vertical_facings.remove(&entity);
            self.walk_lateral_facings.remove(&entity);
            self.walk_supports.remove(&entity);
            self.continuing_walks.remove(&entity);
            return;
        }
        self.walk_facings
            .insert(entity, (to.x - from.x).signum() as i8);
        if to.x == from.x {
            self.walk_vertical_facings
                .insert(entity, (to.y - from.y).signum() as i8 * 2);
        } else {
            self.walk_vertical_facings.remove(&entity);
        }
        if to.x != from.x {
            self.walk_lateral_facings
                .insert(entity, (to.x - from.x).signum() as i8);
        }
        let requested_seconds = f64::from(if [120, 200, 400].contains(&milliseconds) {
            milliseconds
        } else {
            400
        }) / 1000.;
        // The review gives one committed cell its selected presentation time.
        // The legacy path still follows the previous input cadence.
        let seconds = if crate::game_art::walking::player_enabled() {
            requested_seconds
        } else {
            self.glides
                .get(&entity)
                .filter(|g| {
                    g.destination == from && now >= g.started && now - g.started < g.seconds
                })
                .map_or(requested_seconds, |g| {
                    requested_seconds.min((now - g.started).max(1. / 60.))
                })
        };
        let previous = self
            .glides
            .get(&entity)
            .filter(|glide| glide.destination == from)
            .map_or(Vec2::ZERO, |glide| glide.offset(now));
        self.glides.insert(
            entity,
            TileGlide {
                offset: if crate::game_art::walking::player_enabled() {
                    // Never carry unfinished distance into the next step.
                    delta
                } else {
                    delta + previous
                },
                destination: to,
                started: now,
                seconds,
                support: {
                    let next = self.walk_supports.entry(entity).or_insert(1);
                    *next ^= 1;
                    *next
                },
            },
        );
    }

    pub fn stop_glide(&mut self, entity: EntityId) {
        self.continuing_walks.remove(&entity);
        self.glides.remove(&entity);
        self.walk_facings.remove(&entity);
        self.walk_vertical_facings.remove(&entity);
        self.walk_lateral_facings.remove(&entity);
        self.walk_supports.remove(&entity);
    }

    pub fn walk_pose(
        &self,
        entity: EntityId,
        now: f64,
        reduced: bool,
    ) -> Option<crate::game_art::walking::Pose> {
        if reduced {
            return None;
        }
        let facing = *self.walk_facings.get(&entity)?;
        let frame = self.glides.get(&entity).map_or(8, |g| {
            let progress = g.progress(now);
            if progress >= 1. {
                if self.continuing_walks.contains(&entity) {
                    g.support * 3 + 2
                } else {
                    8
                }
            } else {
                g.support * 3 + (progress * 3.) as u8
            }
        });
        Some(crate::game_art::walking::Pose { facing, frame })
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

    pub fn player_walk_pose(
        &self,
        entity: EntityId,
        now: f64,
        reduced: bool,
    ) -> Option<crate::game_art::walking::Pose> {
        // At arrival actors use their ordinary frontal equipment renderer.
        self.walk_pose(entity, now, reduced)
            .filter(|pose| pose.frame != 8)
    }

    /// Accepted VERT18: same phase/timing for player and NPC, true Y heading.
    pub fn vertical_walk_pose(
        &self,
        entity: EntityId,
        now: f64,
        reduced: bool,
    ) -> Option<crate::game_art::walking::Pose> {
        let pose = self
            .walk_pose(entity, now, reduced)
            .filter(|p| p.frame != 8)?;
        let facing = *self.walk_vertical_facings.get(&entity)?;
        matches!(facing, -2 | 2).then_some(crate::game_art::walking::Pose {
            facing,
            frame: pose.frame,
        })
    }

    pub fn creature_walk_pose(
        &self,
        entity: EntityId,
        now: f64,
        reduced: bool,
    ) -> Option<crate::game_art::walking::Pose> {
        let mut pose = self
            .walk_pose(entity, now, reduced)
            .filter(|pose| pose.frame != 8)?;
        if pose.facing == 0 {
            pose.facing = self.walk_lateral_facings.get(&entity).copied().unwrap_or(1);
        }
        Some(pose)
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
        self.continuing_walks.remove(&entity);
        self.glides.remove(&entity);
        self.walk_facings.remove(&entity);
        self.walk_vertical_facings.remove(&entity);
        self.walk_lateral_facings.remove(&entity);
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
            cells: self.cast_cells(skill, Some(origin), target),
        });
    }
    pub fn prepare_cast(&mut self, cast: Option<crate::effect_geometry::PreparedCast>) {
        self.prepared_cast = cast;
    }
    fn cast_cells(
        &self,
        skill: &str,
        origin: Option<GridPos>,
        target: GridPos,
    ) -> Option<Vec<GridPos>> {
        self.prepared_cast
            .as_ref()
            .and_then(|p| p.cells_for(skill, origin, target))
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
        let visible_rect = |at| {
            game.player_visibility()
                .is_visible(at)
                .then(|| rect(at))
                .flatten()
        };
        let rect = &visible_rect;
        crate::animation_policy::with_gain(1.18, || {
            agent_magic::draw(game, &self.magic, now, reduced, background, &rect);
            agent_devices::draw(game, now, reduced, background, &rect);
            let actor_rect =
                |entity, at| rect(at).map(|r| self.actor_rect(entity, r, now, reduced));
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
                    if reduced {
                        0.20
                    } else {
                        crate::animation_policy::age(&flight.skill, now, flight.started) as f32
                    },
                    game.map(),
                    background,
                    &rect,
                    clip,
                    flight.cells.as_deref(),
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
                        if reduced {
                            0.20
                        } else {
                            crate::animation_policy::age(&flight.skill, now, flight.started) as f32
                        },
                        game.map(),
                        background,
                        &rect,
                        flight.cells.as_deref(),
                    );
                }
            }
        });
    }
    pub fn draw_gel(
        &self,
        entity: EntityId,
        rect: Rect,
        now: f64,
        reduced: bool,
        background: bool,
    ) {
        crate::animation_policy::with_gain(1.18, || {
            agent_magic::draw_gel(rect, now, entity.get(), reduced, background)
        });
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
        self.projectile_impacts.insert(
            victim,
            now + agent_ember::FLIGHT_SECONDS * crate::animation_policy::duration_scale("FE01"),
        );
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
        self.projectile_impacts.insert(
            target,
            now + 0.35 * crate::animation_policy::duration_scale("FE05"),
        );
    }
    pub fn draw_fireballs(
        &self,
        game: &project_rl::game::GameState,
        now: f64,
        reduced: bool,
        background: bool,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        let visible_rect = |at| {
            game.player_visibility()
                .is_visible(at)
                .then(|| rect(at))
                .flatten()
        };
        crate::animation_policy::with_gain(crate::animation_policy::gain("FE05"), || {
            agent_fire::draw(
                &self.fireballs,
                game.map(),
                now,
                reduced,
                background,
                visible_rect,
            )
        });
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
            crate::animation_policy::with_gain(1.18, || {
                agent_fire::burn(rect, now, entity.get(), reduced, background)
            });
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
        crate::animation_policy::with_gain(crate::animation_policy::gain("A02"), || {
            agent_spells::draw_blasts(&self.blasts, map, now, reduced, background, rect)
        });
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
        let _color = crate::world_readability::color();
        if reduced {
            return;
        }
        for &(at, started) in &self.taunts {
            crate::guardian_taunt_visuals::pulse(
                map,
                at,
                crate::animation_policy::age("taunt", now, started) as f32,
                &rect,
            );
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
            .find(|s| {
                s.victim == victim && crate::animation_policy::age("A01", now, s.started) <= 0.18
            })
            .map(|s| s.started + 0.18 * crate::animation_policy::duration_scale("A01"))
            .or_else(|| {
                self.strikes
                    .iter()
                    .rev()
                    .find(|s| {
                        s.target == at
                            && crate::animation_policy::age("B01", now, s.started) <= 0.22
                    })
                    .map(|s| s.started + 0.22 * crate::animation_policy::duration_scale("B01"))
            })
            .unwrap_or(now)
    }
    pub fn draw_spells(
        &self,
        game: &project_rl::game::GameState,
        now: f64,
        reduced: bool,
        clip: Rect,
        rect: impl Fn(GridPos) -> Option<Rect>,
    ) {
        let visible_rect = |at| {
            game.player_visibility()
                .is_visible(at)
                .then(|| rect(at))
                .flatten()
        };
        let rect = &visible_rect;
        crate::animation_policy::with_gain(crate::animation_policy::gain("A01"), || {
            agent_spells::draw(&self.spells, game.map(), now, reduced, &rect)
        });
        crate::animation_policy::with_gain(crate::animation_policy::gain("FE01"), || {
            agent_ember::draw(&self.embers, game.map(), now, reduced, clip, &rect)
        });
        crate::animation_policy::with_gain(crate::animation_policy::gain("B01"), || {
            for strike in &self.strikes {
                let age = crate::animation_policy::age("B01", now, strike.started) as f32;
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
                    crate::melee_strike_visuals::cut(
                        start,
                        start.lerp(end, phase),
                        tile * 0.28,
                        life,
                    );
                }
                crate::melee_strike_visuals::impact(
                    to.center(),
                    if reduced { 0.10 } else { age - 0.22 },
                    1.15,
                    false,
                    tile,
                );
            }
        });
        for strike in &self.strikes {
            crate::visual_polish::draw_contact(
                game,
                strike.origin,
                strike.target,
                crate::visual_polish::contact_age(now, strike.started),
                reduced,
                rect,
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
                    *offset
                        * (1.
                            - (crate::animation_policy::age("travel", now, *started) / 0.18)
                                .clamp(0., 1.) as f32)
                })
    }

    pub fn prune(&mut self, now: f64) {
        self.glides.retain(|entity, glide| {
            now - glide.started < glide.seconds || self.continuing_walks.contains(entity)
        });
        self.travels
            .retain(|_, (_, started)| crate::animation_policy::age("travel", now, *started) < 0.18);
        self.fireballs
            .retain(|s| crate::animation_policy::age("FE05", now, s.started) < 0.90);
        self.magic
            .retain(|s| crate::animation_policy::age(&s.skill, now, s.started) < 2.2);
        self.embers.retain(|s| {
            crate::animation_policy::age("FE01", now, s.started) < agent_ember::LIFETIME_SECONDS
        });
        self.projectile_impacts.retain(|_, at| now - *at < 0.30);
        self.taunts
            .retain(|(_, started)| crate::animation_policy::age("taunt", now, *started) < 0.48);
        self.blasts
            .retain(|s| crate::animation_policy::age("A02", now, s.started) < 0.55);
        self.strikes
            .retain(|s| crate::animation_policy::age("B01", now, s.started) < 0.84);
        self.spells
            .retain(|s| crate::animation_policy::age("A01", now, s.started) < 0.4);
        self.motions.retain(|_, motion| {
            crate::animation_policy::age("reaction", now, motion.started) < 0.18
        });
    }
    pub fn clear(&mut self) {
        self.prepared_cast = None;
        self.glides.clear();
        self.walk_facings.clear();
        self.walk_vertical_facings.clear();
        self.walk_lateral_facings.clear();
        self.walk_supports.clear();
        self.continuing_walks.clear();
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
    #[cfg(debug_assertions)]
    pub fn review_age(&mut self, started: f64) {
        for e in &mut self.embers {
            e.started = started;
        }
        for e in &mut self.fireballs {
            e.started = started;
        }
        for e in &mut self.blasts {
            e.started = started;
        }
        for e in &mut self.spells {
            e.started = started;
        }
        for e in &mut self.magic {
            e.started = started;
        }
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
            let peak = hit_reaction_offset(Vec2::ZERO, source, 0.07056, 64.0);
            assert!((peak.length() - 10.88).abs() < 0.01);
            assert_eq!(
                hit_reaction_offset(Vec2::ZERO, source, -0.01, 64.0),
                Vec2::ZERO
            );
            assert_eq!(
                hit_reaction_offset(Vec2::ZERO, source, 0.252, 64.0),
                Vec2::ZERO
            );
        }
    }

    #[test]
    fn motion_extends_and_returns_without_leaving_a_residual_offset() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        motion.play(id, GridPos::new(2, 2), GridPos::new(3, 2), false, 1.0);
        assert!(motion.offset(id, 1.07, false).x > 0.2);
        assert_eq!(motion.offset(id, 1.3, false), Vec2::ZERO);
        motion.play(id, GridPos::new(2, 2), GridPos::new(3, 2), true, 2.0);
        assert!(motion.offset(id, 2.07, false).x < -0.15);
        assert_eq!(motion.offset(id, 2.05, true), Vec2::ZERO);
    }

    #[test]
    fn slower_projectiles_delay_reactions_and_survive_until_their_last_frame() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let a = GridPos::new(1, 2);
        let b = GridPos::new(4, 2);
        let mut motion = CombatMotion::default();
        motion.play_spell(id, a, b, 10.0);
        let impact = motion.impact_time_for(id, b, 10.0);
        assert!((impact - 10.27).abs() < 1e-9);
        motion.play(id, b, a, true, impact);
        assert_eq!(motion.offset(id, 10.269, false), Vec2::ZERO);
        assert_ne!(motion.offset(id, 10.34, false), Vec2::ZERO);
        motion.prune(10.59);
        assert_eq!(motion.spells.len(), 1);
        motion.prune(10.61);
        assert!(motion.spells.is_empty());
        motion.play_fireball(a, b, 1, 20.0);
        motion.fireball_hit_after_flight(id, 20.0);
        assert!((motion.impact_time(id, 20.0) - 20.455).abs() < 1e-9);
        motion.prune(21.16);
        assert_eq!(motion.fireballs.len(), 1);
        motion.prune(21.18);
        assert!(motion.fireballs.is_empty());
        motion.play_agent_magic("SG02", a, b, 1, 30.0);
        motion.prune(32.84);
        assert_eq!(motion.magic.len(), 1);
        motion.prune(33.31);
        assert!(motion.magic.is_empty());
    }

    #[test]
    fn tile_glide_moves_the_whole_actor_and_rests_after_one_hundred_twenty_milliseconds() {
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
            assert!((motion.offset(id, 1.06, false) - delta * 0.5).length() < 0.0001);
            for tile in [32., 64.] {
                let destination = Rect::new(to.x as f32 * tile, to.y as f32 * tile, tile, tile);
                let start = motion.actor_rect(id, destination, 1.0, false);
                assert_eq!(
                    start.point(),
                    vec2(from.x as f32 * tile, from.y as f32 * tile)
                );
                assert_eq!(
                    motion.actor_rect(id, destination, 1.121, false),
                    destination
                );
                assert_eq!(motion.actor_rect(id, destination, 1.02, true), destination);
            }
            motion.prune(1.121);
            assert!(motion.glides.is_empty());
        }
    }

    #[test]
    fn a_new_step_covers_only_its_committed_cell_without_a_queue() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        let a = GridPos::new(10, 10);
        let b = GridPos::new(11, 10);
        let c = GridPos::new(11, 9);
        motion.glide(id, a, b, 1.0);
        motion.glide(id, b, c, 1.02);
        assert_eq!(
            vec2(c.x as f32, c.y as f32) + motion.offset(id, 1.02, false),
            vec2(b.x as f32, b.y as f32)
        );
        assert_eq!(motion.offset(id, 1.141, false), Vec2::ZERO);
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
    fn walk_duration_facing_and_rest_follow_the_committed_step() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let start = GridPos::new(10, 10);
        for milliseconds in [120, 200, 400] {
            let mut motion = CombatMotion::default();
            let seconds = f64::from(milliseconds) / 1000.;
            for (to, facing) in [
                (GridPos::new(9, 10), -1),
                (GridPos::new(11, 11), 1),
                (GridPos::new(10, 9), 0),
                (GridPos::new(10, 11), 0),
            ] {
                motion.stop_glide(id);
                motion.glide_timed(id, start, to, 2., milliseconds);
                let half = motion.walk_pose(id, 2. + seconds * 0.51, false).unwrap();
                assert_eq!(half.facing, facing);
                assert!([1, 4].contains(&half.frame));
                assert!(motion.walk_pose(id, 2., true).is_none());
                assert_ne!(
                    motion.glide_offset(id, 2. + seconds * 0.99, false),
                    Vec2::ZERO
                );
                motion.prune(2. + seconds + 0.001);
                assert_eq!(motion.glide_offset(id, 3., false), Vec2::ZERO);
                assert_eq!(motion.walk_pose(id, 3., false).unwrap().frame, 8);
                assert_eq!(motion.walk_pose(id, 3., false).unwrap().facing, facing);
            }
        }
    }

    #[test]
    fn every_actor_returns_to_front_on_arrival_at_all_supported_speeds() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        for milliseconds in [120, 200, 400] {
            for direction in [-1, 1] {
                let mut motion = CombatMotion::default();
                let duration = f64::from(milliseconds) / 1000.;
                motion.glide_timed(
                    id,
                    GridPos::new(5, 5),
                    GridPos::new(5 + direction, 5),
                    1.,
                    milliseconds,
                );
                assert_eq!(
                    motion
                        .player_walk_pose(id, 1. + duration * 0.5, false)
                        .unwrap()
                        .facing,
                    direction as i8
                );
                assert_eq!(
                    motion
                        .creature_walk_pose(id, 1. + duration * 0.5, false)
                        .unwrap()
                        .facing,
                    direction as i8
                );
                assert!(
                    motion
                        .player_walk_pose(id, 1. + duration + 0.001, false)
                        .is_none()
                );
                assert!(
                    motion
                        .creature_walk_pose(id, 1. + duration + 0.001, false)
                        .is_none()
                );
                assert_eq!(
                    motion
                        .walk_pose(id, 1. + duration + 0.001, false)
                        .unwrap()
                        .facing,
                    direction as i8
                );
                motion.prune(2.);
                assert!(motion.player_walk_pose(id, 2., false).is_none());
                assert!(motion.creature_walk_pose(id, 2., false).is_none());
                assert_eq!(motion.glide_offset(id, 2., false), Vec2::ZERO);
            }
        }
    }

    #[test]
    fn vertical18_player_and_npc_face_the_route_then_return_to_front() {
        let ids: [EntityId; 2] = [
            serde_json::from_str("1").unwrap(),
            serde_json::from_str("2").unwrap(),
        ];
        for milliseconds in [120, 200, 400] {
            for dy in [-1, 1] {
                let mut motion = CombatMotion::default();
                let seconds = f64::from(milliseconds) / 1000.;
                for step in 0..2 {
                    let started = 1. + f64::from(step);
                    for id in ids {
                        motion.glide_timed(
                            id,
                            GridPos::new(4, 4 + step * dy),
                            GridPos::new(4, 4 + (step + 1) * dy),
                            started,
                            milliseconds,
                        );
                        let pose = motion
                            .vertical_walk_pose(id, started + seconds * 0.5, false)
                            .unwrap();
                        assert_eq!(pose.facing, dy as i8 * 2);
                        assert_eq!(pose.frame, step as u8 * 3 + 1);
                        assert!(
                            motion
                                .vertical_walk_pose(id, started + seconds + 0.001, false)
                                .is_none()
                        );
                        assert!(
                            motion
                                .vertical_walk_pose(id, started + seconds * 0.5, true)
                                .is_none()
                        );
                        // The normal renderer keeps its accepted route until
                        // an explicitly prepared vertical profile is enabled.
                        assert_eq!(
                            motion
                                .player_walk_pose(id, started + seconds * 0.5, false)
                                .unwrap()
                                .facing,
                            0
                        );
                    }
                }
                for id in ids {
                    motion.stop_glide(id);
                    assert!(motion.vertical_walk_pose(id, 3., false).is_none());
                    assert!(!motion.walk_vertical_facings.contains_key(&id));
                    motion.glide_timed(
                        id,
                        GridPos::new(4, 4),
                        GridPos::new(4, 3),
                        4.,
                        milliseconds,
                    );
                    motion.glide_timed(
                        id,
                        GridPos::new(4, 3),
                        GridPos::new(5, 3),
                        5.,
                        milliseconds,
                    );
                    assert!(
                        motion
                            .vertical_walk_pose(id, 5. + seconds * 0.5, false)
                            .is_none()
                    );
                }
                motion.clear();
                assert!(motion.walk_vertical_facings.is_empty());
            }
        }
    }

    #[test]
    fn held_walk_bridges_step_boundaries_without_front_frames_or_extra_distance() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        for milliseconds in [120, 200, 400] {
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let mut motion = CombatMotion::default();
                let seconds = f64::from(milliseconds) / 1000.;
                motion.continue_walk(id, true);
                for step in 0..4 {
                    let now = 1. + f64::from(step) * (seconds + 0.02);
                    motion.glide_timed(
                        id,
                        GridPos::new(5 + step * dx, 5 + step * dy),
                        GridPos::new(5 + (step + 1) * dx, 5 + (step + 1) * dy),
                        now,
                        milliseconds,
                    );
                    let before = motion
                        .player_walk_pose(id, now + seconds - 0.001, false)
                        .unwrap();
                    motion.prune(now + seconds + 0.016);
                    let after = motion
                        .player_walk_pose(id, now + seconds + 0.016, false)
                        .unwrap();
                    assert_eq!(before, after);
                    assert_eq!(after.frame, (step % 2) as u8 * 3 + 2);
                    assert_eq!(
                        motion.glide_offset(id, now + seconds + 0.016, false),
                        Vec2::ZERO
                    );
                    if dy != 0 {
                        assert_eq!(
                            motion
                                .vertical_walk_pose(id, now + seconds + 0.016, false)
                                .unwrap()
                                .facing,
                            dy as i8 * 2
                        );
                    }
                    assert!(
                        motion
                            .player_walk_pose(id, now + seconds + 0.016, true)
                            .is_none()
                    );
                }
                motion.continue_walk(id, false);
                assert!(motion.player_walk_pose(id, 9., false).is_none());
                assert!(motion.vertical_walk_pose(id, 9., false).is_none());
                motion.prune(9.);
                assert!(motion.glides.is_empty());
                motion.continue_walk(id, true);
                motion.stop_glide(id);
                assert!(!motion.continuing_walks.contains(&id));
            }
        }
    }

    #[test]
    fn vertical_creature_steps_keep_an_authored_cycle_and_last_lateral_orientation() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        motion.glide_timed(id, GridPos::new(5, 5), GridPos::new(5, 4), 1., 400);
        assert_eq!(motion.creature_walk_pose(id, 1.2, false).unwrap().facing, 1);
        assert_eq!(motion.player_walk_pose(id, 1.2, false).unwrap().facing, 0);
        motion.glide_timed(id, GridPos::new(5, 4), GridPos::new(4, 4), 2., 400);
        motion.glide_timed(id, GridPos::new(4, 4), GridPos::new(4, 5), 3., 400);
        let pose = motion.creature_walk_pose(id, 3.2, false).unwrap();
        assert_eq!(pose.facing, -1);
        assert_eq!(motion.player_walk_pose(id, 3.2, false).unwrap().facing, 0);
        for number in [22, 24, 120, 121] {
            assert!(
                crate::game_art::walking::creature_selection(
                    crate::game_art::Visual::CampaignMonster { number, variant: 0 },
                    pose
                )
                .is_some()
            );
        }
        assert!(motion.creature_walk_pose(id, 3.401, false).is_none());
        motion.stop_glide(id);
        assert!(motion.creature_walk_pose(id, 3.4, false).is_none());
    }

    #[test]
    fn vertical_monster_and_npc_steps_return_to_front_at_every_speed() {
        for code in ["2", "42"] {
            let id: EntityId = serde_json::from_str(code).unwrap();
            for milliseconds in [120, 200, 400] {
                for dy in [-1, 1] {
                    let mut motion = CombatMotion::default();
                    let duration = f64::from(milliseconds) / 1000.;
                    motion.glide_timed(
                        id,
                        GridPos::new(5, 5),
                        GridPos::new(5, 5 + dy),
                        1.,
                        milliseconds,
                    );
                    assert_ne!(
                        motion
                            .creature_walk_pose(id, 1. + duration * 0.5, false)
                            .unwrap()
                            .frame,
                        8
                    );
                    assert!(
                        motion
                            .creature_walk_pose(id, 1. + duration + 0.001, false)
                            .is_none()
                    );
                    assert_eq!(
                        motion.glide_offset(id, 1. + duration + 0.001, false),
                        Vec2::ZERO
                    );
                    motion.prune(2.);
                    assert!(motion.creature_walk_pose(id, 2., false).is_none());
                }
            }
        }
    }

    #[test]
    fn rapid_steps_follow_input_cadence_without_building_a_visual_queue() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        for x in 0..30 {
            let time = 1. + f64::from(x) * 0.08;
            motion.glide_timed(id, GridPos::new(x, 0), GridPos::new(x + 1, 0), time, 400);
            assert!(motion.glide_offset(id, time, false).length() <= 2.01);
        }
        assert_eq!(motion.glide_offset(id, 3.8, false), Vec2::ZERO);
    }
    #[test]
    fn reviewed_steps_cover_one_committed_cell_and_stop_on_arrival() {
        let _review = crate::game_art::walking::review_corrections();
        let id: EntityId = serde_json::from_str("1").unwrap();
        for milliseconds in [120, 200, 400] {
            for direction in [-1, 1] {
                let mut motion = CombatMotion::default();
                let seconds = f64::from(milliseconds) / 1000.;
                for x in 0..30 {
                    let now = 1. + f64::from(x) * 0.08;
                    motion.glide_timed(
                        id,
                        GridPos::new(x * direction, 0),
                        GridPos::new((x + 1) * direction, 0),
                        now,
                        milliseconds,
                    );
                    assert_eq!(motion.glides[&id].seconds, seconds);
                    assert_eq!(
                        motion.glide_offset(id, now, false),
                        vec2(-direction as f32, 0.)
                    );
                    assert!(
                        (motion.glide_offset(id, now + seconds * 0.5, false).x
                            + direction as f32 * 0.5)
                            .abs()
                            < 0.0001
                    );
                    assert_ne!(
                        motion
                            .walk_pose(id, now + seconds * 0.99, false)
                            .unwrap()
                            .frame,
                        8
                    );
                    assert_eq!(motion.glide_offset(id, now + seconds, false), Vec2::ZERO);
                    assert_eq!(motion.walk_pose(id, now + seconds, false).unwrap().frame, 8);
                }
                assert_eq!(motion.glide_offset(id, 4., false), Vec2::ZERO);
            }
        }
    }

    #[test]
    fn canvas02_plays_three_poses_per_cell_and_alternates_support() {
        let id: EntityId = serde_json::from_str("1").unwrap();
        let mut motion = CombatMotion::default();
        for step in 0..4 {
            let start = 2. + f64::from(step);
            motion.glide_timed(
                id,
                GridPos::new(step, 0),
                GridPos::new(step + 1, 0),
                start,
                400,
            );
            let frames: Vec<_> = [0.01, 0.15, 0.29]
                .map(|age| motion.walk_pose(id, start + age, false).unwrap().frame)
                .into();
            assert_eq!(
                frames,
                if step % 2 == 0 {
                    vec![0, 1, 2]
                } else {
                    vec![3, 4, 5]
                }
            );
            assert_eq!(motion.walk_pose(id, start + 0.401, false).unwrap().frame, 8);
            motion.prune(start + 0.5);
        }
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
