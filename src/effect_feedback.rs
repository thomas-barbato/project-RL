//! Read-only combat feedback. No presentation state is serialized in a run.
use super::*;
use crate::terminal_view::TerminalEffectBadge;
use project_rl::status::{StatusEffectPrimitive, StatusTrigger};
use project_rl::weapon::{WeaponDefinition, WeaponEffectKind, WeaponEffectOrigin};

pub(super) fn weapon_bonus_label(bonus: project_rl::entity::MagicItemModifiers) -> String {
    let mut labels = Vec::new();
    for attribute in PrimaryAttribute::ALL {
        let value = bonus.attribute_bonus(attribute);
        if value > 0 {
            labels.push(format!(
                "{} +{value}",
                match attribute {
                    PrimaryAttribute::Power => "Puissance",
                    PrimaryAttribute::Coordination => "Coordination",
                    PrimaryAttribute::Resilience => "Résilience",
                    PrimaryAttribute::Perception => "Perception",
                    PrimaryAttribute::Processing => "Traitement",
                }
            ));
        }
    }
    if bonus.accuracy_bonus() > 0 {
        labels.push(format!("Précision +{}", bonus.accuracy_bonus()));
    }
    if bonus.armor_penetration_bonus() > 0 {
        labels.push(format!("Pénétration +{}", bonus.armor_penetration_bonus()));
    }
    if bonus.mass_reduction_percent() > 0 {
        labels.push(format!("Poids −{} %", bonus.mass_reduction_percent()));
    }
    if bonus.maximum_hit_points_bonus() > 0 {
        labels.push(format!("PV max +{}", bonus.maximum_hit_points_bonus()));
    }
    if bonus.energy_capacity_bonus() > 0 {
        labels.push(format!("Énergie max +{}", bonus.energy_capacity_bonus()));
    }
    if bonus.heat_dissipation_bonus() > 0 {
        labels.push(format!(
            "Dissipation +{}/tour",
            bonus.heat_dissipation_bonus()
        ));
    }
    labels.join(" · ")
}

fn radial_effect_visual(kind: Option<&WeaponEffectKind>) -> &'static str {
    if matches!(kind, Some(WeaponEffectKind::AccumulatedFracture { effect, .. })
        if effect.damage.damage_type == DamageType::Kinetic)
    {
        return "core:weapon_fracture";
    }
    if let Some(
        WeaponEffectKind::RadialDamage { effect, .. }
        | WeaponEffectKind::Catalysis { effect, .. }
        | WeaponEffectKind::AccumulatedFracture { effect, .. },
    ) = kind
    {
        match effect.damage.damage_type {
            DamageType::Thermal => return "core:weapon_catalysis",
            DamageType::Chemical => return "core:corroded",
            DamageType::Electrical => {}
            // Never paint unknown/non-elemental damage as fire or lightning.
            _ => return "core:radial_damage",
        }
    }
    match kind {
        Some(WeaponEffectKind::Ricochet { .. }) => "core:weapon_ricochet",
        Some(WeaponEffectKind::CatalyticCone { .. }) => "core:weapon_catalysis",
        Some(WeaponEffectKind::Alternation { .. }) => "core:weapon_alternation",
        Some(WeaponEffectKind::PiercingLine { .. }) => "core:weapon_piercing",
        Some(WeaponEffectKind::RadialDamage {
            origin: WeaponEffectOrigin::Bearer,
            ..
        }) => "core:weapon_bearer_wave",
        Some(
            WeaponEffectKind::RadialDamage { effect, .. }
            | WeaponEffectKind::Catalysis { effect, .. }
            | WeaponEffectKind::AccumulatedFracture { effect, .. },
        ) if effect.propagation_policy.uses_distinct_water_costs() => "core:weapon_conduction",
        Some(
            WeaponEffectKind::RadialDamage { .. }
            | WeaponEffectKind::Catalysis { .. }
            | WeaponEffectKind::AccumulatedFracture { .. },
        ) => "core:weapon_impact_wave",
        _ => "core:radial_damage",
    }
}

/// Presentation-only spacing for nearby victims' totals; no new message or
/// recipient is created. Keep horizontal alignment with the affected actor.
pub(super) fn separated_damage_label(preferred: Rect, occupied: &[Rect], bounds: Rect) -> Rect {
    let stride = preferred.h + 5.0;
    for distance in 0..=8 {
        for sign in [-1.0, 1.0] {
            let candidate = Rect::new(
                preferred.x,
                preferred.y + sign * distance as f32 * stride,
                preferred.w,
                preferred.h,
            );
            if candidate.y < bounds.y || candidate.bottom() > bounds.bottom() {
                continue;
            }
            let padded = Rect::new(
                candidate.x - 3.0,
                candidate.y - 2.0,
                candidate.w + 6.0,
                candidate.h + 4.0,
            );
            if occupied.iter().all(|other| !padded.overlaps(other)) {
                return candidate;
            }
        }
    }
    preferred
}

#[cfg(test)]
mod placement_tests {
    use super::*;

    #[test]
    fn elemental_presentation_follows_damage_not_just_the_effect_name() {
        use project_rl::combat::DamagePacket;
        use project_rl::effects::{DamageFalloff, RadialDamageEffect};
        use project_rl::world::{NeighborMode, TerrainPropagationPolicy};
        for (damage, expected) in [
            (DamageType::Thermal, "core:weapon_catalysis"),
            (DamageType::Chemical, "core:corroded"),
            (DamageType::Electrical, "core:weapon_impact_wave"),
            (DamageType::Kinetic, "core:radial_damage"),
        ] {
            let effect = RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(4, damage, 0),
                falloff: DamageFalloff::None,
            };
            for kind in [
                WeaponEffectKind::RadialDamage {
                    effect: effect.clone(),
                    origin: WeaponEffectOrigin::Impact,
                    affects_source: false,
                },
                WeaponEffectKind::Catalysis {
                    effect,
                    required_status: "core:burning".parse().unwrap(),
                    affects_source: false,
                },
            ] {
                assert_eq!(radial_effect_visual(Some(&kind)), expected);
            }
        }
    }

    #[test]
    fn adjacent_damage_labels_keep_their_columns_without_touching() {
        let bounds = Rect::new(0.0, 0.0, 640.0, 480.0);
        let first = Rect::new(290.0, 200.0, 30.0, 16.0);
        let adjacent = Rect::new(316.0, 200.0, 20.0, 16.0);
        let placed = separated_damage_label(adjacent, &[first], bounds);
        assert_eq!(placed.x, adjacent.x);
        assert!(!placed.overlaps(&first));
        assert!(placed.bottom() + 4.0 <= first.y);
        let top = Rect::new(290.0, 0.0, 30.0, 16.0);
        let second = separated_damage_label(top, &[top], bounds);
        assert!(second.y > top.bottom());
        assert!(second.bottom() <= bounds.bottom());
        assert_eq!(separated_damage_label(first, &[], bounds), first);
    }
}

impl AsciiApp {
    pub(super) fn fracture_threshold(&self, status: &ContentId) -> Option<u16> {
        self.game
            .rules()
            .weapons
            .effect_sets()
            .flat_map(|(_, effects)| effects)
            .find_map(|effect| match effect.kind() {
                WeaponEffectKind::AccumulatedFracture {
                    mark_status,
                    threshold,
                    ..
                } if mark_status == status => Some(*threshold),
                _ => None,
            })
    }

    pub(super) fn fracture_readiness(&self) -> Option<String> {
        let weapon = self
            .game
            .resolved_equipped_player_weapon(self.active_weapon_slot)?;
        let (status, threshold) =
            weapon
                .effects()
                .iter()
                .find_map(|effect| match effect.kind() {
                    WeaponEffectKind::AccumulatedFracture {
                        mark_status,
                        threshold,
                        ..
                    } => Some((mark_status, threshold)),
                    _ => None,
                })?;
        let actor = self
            .selected_target
            .and_then(|id| self.game.actors().get(id))
            .filter(|actor| self.game.player_visibility().is_visible(actor.position()));
        Some(actor.map_or_else(
            || "MARQUAGE : CHOISIR UNE CIBLE".into(),
            |actor| {
                let charges = actor.status(status).map_or(0, |mark| mark.stacks);
                format!("MARQUAGE : {charges}/{threshold}")
            },
        ))
    }

    /// Authoritative presence comes from the current world on every frame.
    /// Distinct fields/statuses coexist; none is hidden by a one-effect queue.
    pub(super) fn sustained_effects_at(
        &self,
        at: GridPos,
        now: f64,
        reduced: bool,
    ) -> Vec<crate::visual_effects::TerminalEffectSample> {
        if !self.game.player_visibility().is_visible(at) {
            return Vec::new();
        }
        let mut effects =
            self.visual_cues
                .sample_ground(self.game.ground_effects(), at, true, now, reduced);
        if self
            .game
            .pending_weapon_echoes()
            .any(|(position, _)| position == at)
        {
            effects.push(self.visual_cues.sample_sustained(
                &visual_cue_id("core:weapon_echo"),
                at,
                now,
                reduced,
            ));
        }
        if let Some(actor) = self
            .game
            .actors()
            .entity_at(at)
            .and_then(|id| self.game.actors().get(id))
        {
            effects.extend(actor.statuses().filter_map(|status| {
                self.visual_cues
                    .sustains_status(&status.definition)
                    .then(|| {
                        self.visual_cues
                            .sample_sustained(&status.definition, at, now, reduced)
                    })
            }));
        }
        effects
    }

    pub(super) fn active_effect_readiness(&self) -> Option<String> {
        self.percussion_readiness()
            .or_else(|| self.catalysis_readiness())
            .or_else(|| self.fracture_readiness())
            .or_else(|| self.echo_readiness())
    }

    pub(super) fn echo_readiness(&self) -> Option<String> {
        for effect in self
            .game
            .resolved_equipped_player_weapon(self.active_weapon_slot)?
            .effects()
        {
            match effect.kind() {
                WeaponEffectKind::DelayedEcho { delay_turns, .. } => {
                    return Some(format!("ÉCHO : {delay_turns} TOURS"));
                }
                WeaponEffectKind::Alternation { range, .. } => {
                    let player = self.game.player_id();
                    let previous = self
                        .game
                        .alternation_previous(player)
                        .filter(|(target, _)| {
                            self.game.actors().get(*target).is_some_and(|actor| {
                                self.game.player_visibility().is_visible(actor.position())
                            })
                        });
                    let text = match previous {
                        None => "TOUCHEZ UNE CIBLE",
                        Some((previous, _)) => match self.selected_target.filter(|target| {
                            self.game.actors().get(*target).is_some_and(|actor| {
                                self.game.player_visibility().is_visible(actor.position())
                            })
                        }) {
                            Some(target) if target == previous => "CHANGEZ DE CIBLE",
                            Some(target)
                                if self.game.alternation_can_link(player, target, *range) =>
                            {
                                "LIEN POSSIBLE"
                            }
                            Some(_) => "LIEN IMPOSSIBLE",
                            None => "CHOISISSEZ UNE CIBLE",
                        },
                    };
                    return Some(text.into());
                }
                _ => {}
            }
        }
        None
    }

    pub(super) fn catalysis_readiness(&self) -> Option<String> {
        self.game
            .resolved_equipped_player_weapon(self.active_weapon_slot)?
            .effects()
            .iter()
            .any(|effect| matches!(effect.kind(), WeaponEffectKind::CatalyticCone { .. }))
            .then(|| "CÔNE DE FLAMMES".into())
    }

    /// Essential limits belong beside the description, not in its prose.
    pub(super) fn weapon_effect_limits(&self, weapon: &WeaponDefinition) -> String {
        weapon
            .effects()
            .iter()
            .filter_map(|effect| match effect.kind() {
                WeaponEffectKind::LifeSteal {
                    maximum_per_attack, ..
                } => Some(format!(
                    "Soin : {maximum_per_attack} PV maximum par attaque"
                )),
                WeaponEffectKind::ApplyBearerStatus(status) => {
                    let definition = self.game.rules().statuses.get(status.status())?;
                    definition
                        .modifiers()
                        .iter()
                        .any(|modifier| {
                            matches!(
                                modifier,
                                project_rl::status::StatusModifier::DamageGuard { .. }
                            )
                        })
                        .then(|| {
                            format!(
                                "Durée : {} tours · un seul coup absorbé",
                                definition.duration_turns().unwrap_or(1).saturating_sub(1)
                            )
                        })
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" · ")
    }

    pub(super) fn percussion_readiness(&self) -> Option<String> {
        let weapon = self
            .game
            .resolved_equipped_player_weapon(self.active_weapon_slot)?;
        let recovery = weapon
            .effects()
            .iter()
            .find_map(|effect| match effect.kind() {
                WeaponEffectKind::Percussion {
                    recovery_status, ..
                } => Some(recovery_status),
                _ => None,
            })?;
        let turns = self
            .game
            .actors()
            .get(self.game.player_id())?
            .status(recovery)
            .and_then(|status| status.remaining_turns);
        Some(turns.map_or_else(
            || "PERCUSSION PRÊTE".into(),
            |turns| format!("RECHARGE : {turns} TOUR(S)"),
        ))
    }

    pub(super) fn radial_visual_id(&self, source: Option<&(WeaponId, usize)>) -> ContentId {
        let kind = source
            .and_then(|(id, index)| self.game.rules().weapons.effect_at_source(id, *index))
            .map(|effect| effect.kind());
        visual_cue_id(radial_effect_visual(kind))
    }

    /// One badge per effect identity, not a cap on simultaneous gameplay effects.
    /// Ground exposure is distinct from a status attached to the actor.
    pub(super) fn effect_badges_at(&self, at: GridPos) -> Vec<TerminalEffectBadge> {
        if !self.game.player_visibility().is_visible(at) {
            return Vec::new();
        }
        let Some(actor) = self
            .game
            .actors()
            .entity_at(at)
            .and_then(|id| self.game.actors().get(id))
        else {
            return Vec::new();
        };
        let mut badges = Vec::new();
        if self
            .game
            .alternation_previous(self.game.player_id())
            .is_some_and(|(target, _)| self.game.actors().entity_at(at) == Some(target))
        {
            badges.push(TerminalEffectBadge::Alternation);
        }
        if let Some((_, turns)) = self
            .game
            .pending_weapon_echoes()
            .find(|(position, _)| *position == at)
        {
            badges.push(TerminalEffectBadge::Echo { turns });
        }
        for status in actor
            .statuses()
            .filter(|status| status.remaining_turns.is_some())
        {
            if let Some(threshold) = self.fracture_threshold(&status.definition) {
                badges.push(TerminalEffectBadge::Fracture {
                    charges: status.stacks,
                    threshold,
                });
                continue;
            }
            if self
                .game
                .rules()
                .statuses
                .get(&status.definition)
                .is_some_and(|definition| {
                    definition.modifiers().iter().any(|modifier| {
                        matches!(
                            modifier,
                            project_rl::status::StatusModifier::DamageGuard { .. }
                        )
                    })
                })
            {
                badges.push(TerminalEffectBadge::Guard);
                continue;
            }
            let damage_type =
                self.game
                    .rules()
                    .statuses
                    .get(&status.definition)
                    .and_then(|definition| {
                        [StatusTrigger::TurnStart, StatusTrigger::TurnEnd]
                            .into_iter()
                            .flat_map(|trigger| definition.hooks_for(trigger))
                            .flat_map(|hook| hook.effects())
                            .find_map(|effect| match effect {
                                StatusEffectPrimitive::DealDamage { packet, .. } => {
                                    Some(packet.damage_type)
                                }
                                _ => None,
                            })
                    });
            badges.push(match damage_type {
                Some(DamageType::Thermal) => TerminalEffectBadge::Burning,
                Some(DamageType::Chemical) => TerminalEffectBadge::Corrosion,
                Some(DamageType::Electrical) => TerminalEffectBadge::Electrical,
                _ => match status.definition.as_str() {
                    "core:armor_fragilized" => TerminalEffectBadge::Fragile,
                    "core:locomotion_hindered" => TerminalEffectBadge::Slowed,
                    "core:suppressed" => TerminalEffectBadge::Suppressed,
                    _ => TerminalEffectBadge::Timed,
                },
            });
        }
        for field in self.game.ground_effects().at(at) {
            badges.push(match field.damage_each_turn().damage_type {
                DamageType::Chemical => TerminalEffectBadge::Caustic,
                DamageType::Thermal => TerminalEffectBadge::Burning,
                DamageType::Electrical => TerminalEffectBadge::Electrical,
                _ => TerminalEffectBadge::Timed,
            });
        }
        badges
    }
}
