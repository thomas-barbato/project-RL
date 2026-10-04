//! Refundable activation payments owned by live maintained effects.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MaintainedEffectId {
    Camouflage,
    Jamming,
    Access(GridPos),
    Device(GridPos),
    ControlLock(GridPos),
    DroneOrder(EntityId),
    Sound(crate::stealth::SoundEmitterId),
    Explosive(ExplosiveDeviceId),
    Beacon(EntityId),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaintainedEnergyReservation {
    pub effect: MaintainedEffectId,
    pub technique: TechniqueId,
    pub amount: u16,
}

impl GameState {
    pub fn player_companion_count(&self) -> usize {
        self.away_companions
            + self
                .actors
                .iter()
                .filter(|(_, a)| a.companion_origin().is_some_and(|o| o.uses_slot()))
                .count()
    }

    pub fn player_maintained_energy(&self) -> Vec<MaintainedEnergyReservation> {
        self.maintained_energy.values().cloned().collect()
    }

    pub(super) fn maintained_command_technique(command: &GameCommand) -> Option<&TechniqueId> {
        match command {
            GameCommand::UseTechnique { technique, .. }
            | GameCommand::UseTechniqueAt { technique, .. }
            | GameCommand::UseDroneTechnique { technique, .. }
            | GameCommand::UseEngineeringTechnique { technique, .. }
            | GameCommand::UseIntrusionTechnique { technique, .. }
            | GameCommand::UseElectronicWarfareTechnique { technique, .. } => Some(technique),
            _ => None,
        }
    }

    // A small description of each live effect is used only to detect creations
    // and replacements during one successful command, before time advances.
    pub(super) fn live_maintained_effects(&self) -> BTreeMap<MaintainedEffectId, String> {
        let mut live = BTreeMap::new();
        if !self.rules.maintained_energy_reservations {
            return live;
        }
        if let Some(state) = &self.player_active_camouflage {
            live.insert(MaintainedEffectId::Camouflage, format!("{state:?}"));
        }
        if let Some(state) = self.electronic_warfare.jamming() {
            live.insert(MaintainedEffectId::Jamming, format!("{state:?}"));
        }
        for (position, state) in self.intrusion.sessions() {
            if state.bandwidth_reserved() > 0 {
                live.insert(MaintainedEffectId::Access(position), format!("{state:?}"));
            }
        }
        for (position, state) in self.intrusion.controls() {
            if state.bandwidth_reserved > 0 {
                live.insert(MaintainedEffectId::Device(position), format!("{state:?}"));
            }
        }
        for (position, state) in self.intrusion.control_locks() {
            if state.bandwidth_reserved > 0 {
                live.insert(
                    MaintainedEffectId::ControlLock(position),
                    format!("{state:?}"),
                );
            }
        }
        for (entity, actor) in self.actors.iter() {
            if let Some(drone) = actor
                .drone()
                .filter(|d| d.controller() == self.player && d.order_bandwidth() > 0)
            {
                live.insert(
                    MaintainedEffectId::DroneOrder(entity),
                    format!("{:?}", drone.order()),
                );
            }
        }
        for (id, reservation) in &self.manifested_sound_emitters {
            if reservation.bandwidth > 0 && self.sound_emitters.get(*id).is_some() {
                live.insert(MaintainedEffectId::Sound(*id), format!("{reservation:?}"));
            }
        }
        for (id, reservation) in &self.manifested_explosives {
            if reservation.bandwidth > 0
                && self
                    .explosive_devices
                    .get(*id)
                    .is_some_and(|d| !d.is_neutralized())
            {
                live.insert(
                    MaintainedEffectId::Explosive(*id),
                    format!("{reservation:?}"),
                );
            }
        }
        for (id, reservation) in &self.manifested_beacons {
            if reservation.bandwidth > 0 && self.actors.get(*id).is_some() {
                live.insert(MaintainedEffectId::Beacon(*id), format!("{reservation:?}"));
            }
        }
        live
    }

    fn release_energy_for_effect(&mut self, effect: MaintainedEffectId) {
        if let Some(reservation) = self.maintained_energy.remove(&effect) {
            let amount = self.player_energy.release(reservation.amount);
            self.events.push(GameEvent::MaintainedEnergyChanged {
                technique: reservation.technique,
                amount,
                reserved: false,
            });
        }
    }

    pub(super) fn release_finished_maintained_energy(&mut self) {
        if !self.rules.maintained_energy_reservations {
            return;
        }
        let live = self.live_maintained_effects();
        let ended: Vec<_> = self
            .maintained_energy
            .keys()
            .filter(|key| !live.contains_key(key))
            .copied()
            .collect();
        for key in ended {
            self.release_energy_for_effect(key);
        }
    }

    pub(super) fn convert_activation_to_reservation(
        &mut self,
        technique: Option<&TechniqueId>,
        before: &BTreeMap<MaintainedEffectId, String>,
        event_checkpoint: usize,
    ) {
        if !self.rules.maintained_energy_reservations {
            return;
        }
        self.release_finished_maintained_energy();
        let Some(technique) = technique else {
            return;
        };
        let live = self.live_maintained_effects();
        let changed: Vec<_> = live
            .iter()
            .filter(|(key, state)| before.get(key) != Some(state))
            .map(|(key, _)| *key)
            .collect();
        if changed.is_empty() {
            return;
        }
        // Only this command's actual activation payment is locked. Failed
        // commands never reach here; preparation alone creates no reservation.
        let paid = self.events[event_checkpoint..]
            .iter()
            .filter_map(|event| match event {
                GameEvent::EnergySpent { entity, amount, .. } if *entity == self.player => {
                    Some(*amount)
                }
                _ => None,
            })
            .fold(0_u16, u16::saturating_add);
        if paid == 0 {
            return;
        }
        for key in &changed {
            self.release_energy_for_effect(*key);
        }
        self.player_energy.retain_activation_payment(paid);
        let count = changed.len() as u16;
        for (index, effect) in changed.into_iter().enumerate() {
            let amount = paid / count + u16::from(index < usize::from(paid % count));
            self.maintained_energy.insert(
                effect,
                MaintainedEnergyReservation {
                    effect,
                    technique: technique.clone(),
                    amount,
                },
            );
        }
        // The UI must describe the payment as reserved, not lost energy.
        let tail: Vec<_> = self.events.drain(event_checkpoint..).filter(|event| !matches!(event, GameEvent::EnergySpent { entity, .. } if *entity == self.player)).collect();
        self.events.extend(tail);
        self.events.push(GameEvent::MaintainedEnergyChanged {
            technique: technique.clone(),
            amount: paid,
            reserved: true,
        });
    }

    pub(super) fn end_maintained_effect(
        &mut self,
        effect: MaintainedEffectId,
    ) -> Result<(), CommandRejection> {
        if !self.maintained_energy.contains_key(&effect) {
            return Err(CommandRejection::MaintainedEffectUnavailable);
        }
        self.stop_maintained_effect(effect);
        Ok(())
    }

    fn stop_maintained_effect(&mut self, effect: MaintainedEffectId) {
        match effect {
            MaintainedEffectId::Camouflage => self.end_player_active_camouflage(),
            MaintainedEffectId::Jamming => {
                self.electronic_warfare.take_jamming();
            }
            MaintainedEffectId::Access(at) => {
                self.intrusion.remove_session(at);
            }
            MaintainedEffectId::Device(at) => {
                if let Some(control) = self.intrusion.remove_control(at) {
                    self.revert_device_command(at, control.command);
                }
                self.intrusion.remove_control_lock(at);
            }
            MaintainedEffectId::ControlLock(at) => {
                self.intrusion.remove_control_lock(at);
            }
            MaintainedEffectId::DroneOrder(entity) => {
                self.set_drone_order(
                    entity,
                    DroneOrder::Companion {
                        controller: self.player,
                        behavior: CompanionBehavior::Follow,
                    },
                );
                self.release_drone_order_bandwidth(entity);
            }
            MaintainedEffectId::Sound(id) => {
                let _ = self.sound_emitters.damage(id, u16::MAX);
                self.manifested_sound_emitters.remove(&id);
            }
            MaintainedEffectId::Explosive(id) => {
                let _ = self.explosive_devices.neutralize(id);
                let _ = self.explosive_devices.recover(id);
                self.manifested_explosives.remove(&id);
            }
            MaintainedEffectId::Beacon(entity) => {
                self.electronic_warfare.remove_beacon(entity);
                self.actors.remove(entity);
                self.manifested_beacons.remove(&entity);
            }
        }
        self.release_finished_maintained_energy();
    }

    pub(super) fn dismiss_player_companion(
        &mut self,
        entity: EntityId,
    ) -> Result<(), CommandRejection> {
        let actor = self
            .actors
            .get(entity)
            .ok_or(CommandRejection::NoControlledCompanion)?;
        if !actor
            .companion_origin()
            .is_some_and(|origin| origin.uses_slot())
        {
            return Err(CommandRejection::NoControlledCompanion);
        }
        self.actors.remove(entity);
        self.release_finished_maintained_energy();
        Ok(())
    }

    pub(super) fn fit_maintained_energy_capacity(&mut self, capacity: u16) {
        while self.player_energy.reserved() > capacity {
            let Some(key) = self.maintained_energy.keys().next_back().copied() else {
                break;
            };
            self.stop_maintained_effect(key);
        }
    }

    pub(in crate::game) fn end_local_maintained_effects_for_travel(&mut self) {
        let local: Vec<_> = self
            .maintained_energy
            .keys()
            .filter(|key| !matches!(key, MaintainedEffectId::Camouflage))
            .copied()
            .collect();
        for key in local {
            self.stop_maintained_effect(key);
        }
    }
}
