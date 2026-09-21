use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::stats::{PrimaryAttribute, PrimaryAttributes};
use crate::world::{
    GridPos, Map, NeighborMode, PropagationRequest, TerrainPropagationPolicy, propagate,
};

/// Independent perception channels. A modifier must always name the channel
/// it affects; no stealth action can silently become global invisibility.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum SignatureChannel {
    Optical,
    Acoustic,
    ActiveEmission,
}

/// Versioned coefficients for deterministic concealment checks.
///
/// Geometry and sensor range are checked before this calculation. The result
/// therefore cannot reveal through a wall or manufacture a sensor channel.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StealthRules {
    pub base_detection: i16,
    pub perception_points_per_attribute: i16,
    pub base_concealment_difficulty: i16,
    pub coordination_points_per_attribute: i16,
    pub distance_penalty_start: u16,
    pub distance_penalty_per_cell: i16,
    pub partial_cover_occultation: i16,
    pub sound_attenuation_per_cell: u16,
    pub sound_wall_attenuation_multiplier: u16,
}

impl StealthRules {
    pub fn validate(self) -> Result<(), StealthRulesError> {
        if self.perception_points_per_attribute < 0
            || self.coordination_points_per_attribute < 0
            || self.distance_penalty_per_cell < 0
        {
            return Err(StealthRulesError::NegativeCoefficient);
        }
        if self.sound_attenuation_per_cell == 0 || self.sound_wall_attenuation_multiplier == 0 {
            return Err(StealthRulesError::ZeroSoundAttenuation);
        }
        Ok(())
    }

    pub fn optical_detection_score(
        self,
        observer: Option<PrimaryAttributes>,
        distance: u16,
        sensor_bonus: i16,
        state_modifier: i16,
    ) -> i16 {
        let perception = observer
            .unwrap_or_else(|| PrimaryAttributes::new(5, 5, 5, 5, 5))
            .value(PrimaryAttribute::Perception);
        let attribute = i16::from(perception).saturating_sub(5);
        let distance_penalty = distance
            .saturating_sub(self.distance_penalty_start)
            .saturating_mul(u16::try_from(self.distance_penalty_per_cell).unwrap_or(u16::MAX));
        self.base_detection
            .saturating_add(attribute.saturating_mul(self.perception_points_per_attribute))
            .saturating_add(sensor_bonus)
            .saturating_add(state_modifier)
            .saturating_sub(i16::try_from(distance_penalty).unwrap_or(i16::MAX))
            .max(0)
    }

    pub fn optical_concealment_difficulty(
        self,
        target: Option<PrimaryAttributes>,
        occultation: i16,
        state_bonus: i16,
    ) -> i16 {
        let coordination = target
            .unwrap_or_else(|| PrimaryAttributes::new(5, 5, 5, 5, 5))
            .value(PrimaryAttribute::Coordination);
        let attribute = i16::from(coordination).saturating_sub(5);
        self.base_concealment_difficulty
            .saturating_add(attribute.saturating_mul(self.coordination_points_per_attribute))
            .saturating_add(occultation)
            .saturating_add(state_bonus)
            .max(0)
    }

    pub fn sound_reaches(self, intensity: u16, origin: GridPos, listener: GridPos) -> bool {
        let distance = origin
            .x
            .abs_diff(listener.x)
            .max(origin.y.abs_diff(listener.y));
        let distance = u16::try_from(distance).unwrap_or(u16::MAX);
        intensity > distance.saturating_mul(self.sound_attenuation_per_cell)
    }

    /// Propagates a sound through the actual map. Open cells retain Chebyshev
    /// distance while opaque/solid cells consume additional attenuation.
    pub fn sound_reaches_on_map(
        self,
        map: &Map,
        intensity: u16,
        origin: GridPos,
        listener: GridPos,
    ) -> bool {
        if intensity == 0 || self.sound_attenuation_per_cell == 0 {
            return false;
        }
        let maximum_cost = intensity
            .saturating_sub(1)
            .checked_div(self.sound_attenuation_per_cell)
            .unwrap_or(0);
        propagate(
            map,
            PropagationRequest {
                origin,
                maximum_cost,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
            },
            &TerrainPropagationPolicy {
                floor_cost: Some(1),
                shallow_water_cost: Some(1),
                deep_water_cost: Some(self.sound_wall_attenuation_multiplier),
                wall_cost: Some(self.sound_wall_attenuation_multiplier),
            },
        )
        .iter()
        .any(|cell| cell.position == listener)
    }
}

impl Default for StealthRules {
    fn default() -> Self {
        Self {
            base_detection: 50,
            perception_points_per_attribute: 4,
            base_concealment_difficulty: 40,
            coordination_points_per_attribute: 4,
            distance_penalty_start: 2,
            distance_penalty_per_cell: 2,
            partial_cover_occultation: 15,
            sound_attenuation_per_cell: 5,
            sound_wall_attenuation_multiplier: 3,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StealthRulesError {
    NegativeCoefficient,
    ZeroSoundAttenuation,
}

impl Display for StealthRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NegativeCoefficient => {
                formatter.write_str("stealth coefficients cannot be negative")
            }
            Self::ZeroSoundAttenuation => {
                formatter.write_str("sound attenuation per cell must be positive")
            }
        }
    }
}

impl Error for StealthRulesError {}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct SoundEmitterId(u64);

impl SoundEmitterId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// A physical, persistent decoy occupying a known world position. It does not
/// block movement, but it can be addressed and removed independently from the
/// transient sound observations it produces.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SoundEmitter {
    id: SoundEmitterId,
    position: GridPos,
    intensity: u16,
    remaining_phases: u16,
    integrity: u16,
}

impl SoundEmitter {
    pub const fn id(&self) -> SoundEmitterId {
        self.id
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn intensity(&self) -> u16 {
        self.intensity
    }

    pub const fn remaining_phases(&self) -> u16 {
        self.remaining_phases
    }

    pub const fn integrity(&self) -> u16 {
        self.integrity
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SoundEmitterMap {
    next_id: u64,
    emitters: std::collections::BTreeMap<SoundEmitterId, SoundEmitter>,
}

impl Default for SoundEmitterMap {
    fn default() -> Self {
        Self {
            next_id: 1,
            emitters: std::collections::BTreeMap::new(),
        }
    }
}

impl SoundEmitterMap {
    pub fn deploy(
        &mut self,
        position: GridPos,
        intensity: u16,
        duration_phases: u16,
        integrity: u16,
    ) -> Result<SoundEmitterId, SoundEmitterError> {
        if intensity == 0 {
            return Err(SoundEmitterError::ZeroIntensity);
        }
        if duration_phases == 0 {
            return Err(SoundEmitterError::ZeroDuration);
        }
        if integrity == 0 {
            return Err(SoundEmitterError::ZeroIntegrity);
        }
        let id = SoundEmitterId(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(SoundEmitterError::IdSpaceExhausted)?;
        self.emitters.insert(
            id,
            SoundEmitter {
                id,
                position,
                intensity,
                remaining_phases: duration_phases,
                integrity,
            },
        );
        self.next_id = next_id;
        Ok(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SoundEmitter> {
        self.emitters.values()
    }

    pub fn get(&self, id: SoundEmitterId) -> Option<&SoundEmitter> {
        self.emitters.get(&id)
    }

    pub fn at(&self, position: GridPos) -> impl Iterator<Item = &SoundEmitter> {
        self.emitters
            .values()
            .filter(move |emitter| emitter.position == position)
    }

    pub fn damage(&mut self, id: SoundEmitterId, amount: u16) -> Result<bool, SoundEmitterError> {
        let emitter = self
            .emitters
            .get_mut(&id)
            .ok_or(SoundEmitterError::UnknownEmitter(id))?;
        emitter.integrity = emitter.integrity.saturating_sub(amount);
        if emitter.integrity == 0 {
            self.emitters.remove(&id);
            return Ok(true);
        }
        Ok(false)
    }

    /// Advances all emitters once and returns those whose finite battery ended.
    pub fn elapse(&mut self) -> Vec<SoundEmitterId> {
        let expired = self
            .emitters
            .iter_mut()
            .filter_map(|(id, emitter)| {
                emitter.remaining_phases = emitter.remaining_phases.saturating_sub(1);
                (emitter.remaining_phases == 0).then_some(*id)
            })
            .collect::<Vec<_>>();
        for id in &expired {
            self.emitters.remove(id);
        }
        expired
    }

    pub fn is_empty(&self) -> bool {
        self.emitters.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SoundEmitterError {
    ZeroIntensity,
    ZeroDuration,
    ZeroIntegrity,
    IdSpaceExhausted,
    UnknownEmitter(SoundEmitterId),
}

impl Display for SoundEmitterError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroIntensity => formatter.write_str("sound intensity must be positive"),
            Self::ZeroDuration => formatter.write_str("sound duration must be positive"),
            Self::ZeroIntegrity => formatter.write_str("emitter integrity must be positive"),
            Self::IdSpaceExhausted => formatter.write_str("sound emitter ID space is exhausted"),
            Self::UnknownEmitter(id) => write!(formatter, "unknown sound emitter {}", id.get()),
        }
    }
}

impl Error for SoundEmitterError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_optical_examples_are_deterministic() {
        let rules = StealthRules::default();
        assert_eq!(
            rules.optical_detection_score(Some(PrimaryAttributes::new(5, 5, 5, 5, 5)), 2, 0, 0),
            50
        );
        assert_eq!(
            rules.optical_detection_score(Some(PrimaryAttributes::new(5, 5, 5, 8, 5)), 4, 0, 0),
            58
        );
        assert_eq!(
            rules.optical_concealment_difficulty(
                Some(PrimaryAttributes::new(5, 5, 5, 5, 5)),
                15,
                0
            ),
            55
        );
    }

    #[test]
    fn walls_attenuate_sound_without_becoming_absolute_silence() {
        let rules = StealthRules::default();
        let obstructed = Map::from_ascii("#########\n#..#....#\n#########").unwrap();
        let open = Map::from_ascii("#########\n#.......#\n#########").unwrap();
        let origin = GridPos::new(1, 1);
        let listener = GridPos::new(5, 1);

        assert!(rules.sound_reaches_on_map(&open, 30, origin, listener));
        assert!(!rules.sound_reaches_on_map(&obstructed, 30, origin, listener));
        assert!(rules.sound_reaches_on_map(&obstructed, 40, origin, listener));
    }

    #[test]
    fn emitters_have_stable_ids_and_expire_exactly() {
        let mut emitters = SoundEmitterMap::default();
        let first = emitters.deploy(GridPos::new(1, 2), 30, 2, 1).unwrap();
        let second = emitters.deploy(GridPos::new(2, 2), 10, 1, 2).unwrap();
        assert!(first < second);
        assert_eq!(emitters.elapse(), vec![second]);
        assert_eq!(emitters.get(first).unwrap().remaining_phases(), 1);
        assert_eq!(emitters.elapse(), vec![first]);
        assert!(emitters.is_empty());
    }
}
