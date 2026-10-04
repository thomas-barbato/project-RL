//! Small, persistent installations. State lives in append-only capabilities,
//! preserving the binary representation of facilities without these devices.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstallationIntel {
    pub record: ContentId,
    pub label: String,
    pub target: Option<GridPos>,
    pub consumes_turn: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InstallationAction {
    ReadIntel { index: u8 },
    SetPower { powered: bool },
    ActivateDiversion,
}

pub(super) fn validate_interactive_installations(
    installations: &BTreeMap<InstallationId, InstallationState>,
) -> Result<(), FacilityBuildError> {
    let mut controlled_doors = BTreeSet::new();
    for installation in installations.values() {
        for capability in &installation.capabilities {
            if let InstallationCapability::DoorActuator { door } = capability {
                controlled_doors.insert(*door);
            }
        }
    }
    for installation in installations.values() {
        let mut kinds = BTreeSet::new();
        for capability in &installation.capabilities {
            let kind = match capability {
                InstallationCapability::IntelTerminal { .. } => Some(0),
                InstallationCapability::PowerControl { .. } => Some(1),
                InstallationCapability::SwitchableRelay { .. } => Some(2),
                InstallationCapability::DiversionPost { .. } => Some(3),
                _ => None,
            };
            if kind.is_some_and(|kind| !kinds.insert(kind)) {
                return Err(FacilityBuildError::InvalidInstallation(
                    installation.id.clone(),
                ));
            }
            let valid = match capability {
                InstallationCapability::IntelTerminal { records } => {
                    !records.is_empty()
                        && records.len() <= 8
                        && records.iter().all(|record| {
                            !record.label.trim().is_empty() && record.label.len() <= 160
                        })
                        && records
                            .iter()
                            .map(|record| &record.record)
                            .collect::<BTreeSet<_>>()
                            .len()
                            == records.len()
                }
                InstallationCapability::PowerControl { relay } => {
                    installations.get(relay).is_some_and(|target| {
                        target.capabilities.iter().any(|capability| {
                            matches!(capability, InstallationCapability::SwitchableRelay { .. })
                        })
                    })
                }
                InstallationCapability::SwitchableRelay { door, .. } => {
                    controlled_doors.insert(*door)
                }
                InstallationCapability::DiversionPost {
                    intensity,
                    duration,
                    ..
                } => *intensity > 0 && *intensity <= 100 && *duration > 0 && *duration <= 100,
                _ => true,
            };
            if !valid {
                return Err(FacilityBuildError::InvalidInstallation(
                    installation.id.clone(),
                ));
            }
        }
    }
    Ok(())
}

impl FacilityState {
    pub fn has_interaction_menu_at(&self, position: GridPos) -> bool {
        self.installation_at(position).is_some_and(|installation| {
            installation.capabilities.iter().any(|capability| {
                matches!(
                    capability,
                    InstallationCapability::IntelTerminal { .. }
                        | InstallationCapability::PowerControl { .. }
                        | InstallationCapability::DiversionPost { .. }
                )
            })
        })
    }

    pub fn intel_at(&self, position: GridPos) -> &[InstallationIntel] {
        self.installation_at(position)
            .and_then(|installation| {
                installation
                    .capabilities
                    .iter()
                    .find_map(|capability| match capability {
                        InstallationCapability::IntelTerminal { records } => {
                            Some(records.as_slice())
                        }
                        _ => None,
                    })
            })
            .unwrap_or(&[])
    }

    pub fn power_setting_at(&self, position: GridPos) -> Option<bool> {
        let relay = self
            .installation_at(position)?
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::PowerControl { relay } => Some(relay),
                _ => None,
            })?;
        self.installation(relay)?
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::SwitchableRelay { powered, .. } => Some(*powered),
                _ => None,
            })
    }

    pub fn diversion_at(&self, position: GridPos) -> Option<(u8, u16, u16)> {
        self.installation_at(position)?
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::DiversionPost {
                    charges,
                    intensity,
                    duration,
                    ..
                } => Some((*charges, *intensity, *duration)),
                _ => None,
            })
    }

    pub fn diversion_outlet_at(&self, position: GridPos) -> Option<GridPos> {
        self.installation_at(position)?
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::DiversionPost { outlet, .. } => Some(*outlet),
                _ => None,
            })
    }

    pub fn intel_was_read(&self, record: &ContentId) -> bool {
        self.retained_data_terminal_records.contains(record)
    }

    pub(crate) fn remember_intel(&mut self, record: ContentId) {
        self.retained_data_terminal_records.insert(record);
    }

    pub(crate) fn set_power_at(&mut self, position: GridPos, powered: bool) {
        let relay = self
            .installation_at(position)
            .unwrap()
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::PowerControl { relay } => Some(relay.clone()),
                _ => None,
            })
            .unwrap();
        for capability in &mut self.installations.get_mut(&relay).unwrap().capabilities {
            if let InstallationCapability::SwitchableRelay {
                powered: setting, ..
            } = capability
            {
                *setting = powered;
            }
        }
    }

    pub(crate) fn spend_diversion_charge(&mut self, position: GridPos) {
        let id = self.installation_at(position).unwrap().id.clone();
        for capability in &mut self.installations.get_mut(&id).unwrap().capabilities {
            if let InstallationCapability::DiversionPost { charges, .. } = capability {
                *charges -= 1;
            }
        }
    }

    pub(crate) fn synchronize_interactive_outputs(
        &self,
        map: &mut Map,
        actors: &ActorRegistry,
        ground: Option<&GroundItemRegistry>,
    ) -> Result<(), FacilityRuntimeError> {
        for installation in self.installations.values() {
            for capability in &installation.capabilities {
                let InstallationCapability::SwitchableRelay { door, .. } = capability else {
                    continue;
                };
                let operational = self.is_operational(&installation.id);
                let current = map
                    .tile(*door)
                    .ok_or(FacilityRuntimeError::InvalidControlledDoor(*door))?
                    .terrain;
                if !matches!(current, Terrain::Door(_)) {
                    return Err(FacilityRuntimeError::InvalidControlledDoor(*door));
                }
                let occupied = actors.entity_at(*door).is_some()
                    || ground.is_some_and(|ground| ground.item_at(*door).is_some());
                if !operational && occupied && current == Terrain::Door(DoorState::Open) {
                    continue;
                }
                let next = Terrain::Door(if operational {
                    DoorState::Open
                } else {
                    DoorState::Unpowered
                });
                if next != current {
                    map.set_terrain(*door, next)
                        .map_err(|_| FacilityRuntimeError::InvalidControlledDoor(*door))?;
                }
            }
        }
        Ok(())
    }
}
