//! Single-use snapshot suspension with a verified replay fallback.
use project_rl::{
    companion::CompanionBehavior,
    drone::{
        DroneCondition, DroneConditionalResponse, DroneDeploymentAssignment, DroneDeploymentRole,
        DroneDirective, PatrolBlockedResponse,
    },
    electronic_warfare::{ElectronicChannel, ElectronicDirective, HostileProgramId},
    engineering::{EngineeringDirective, ModuleTuning, WreckId},
    entity::{BodyComponentId, EntityId, GroundItemId, ItemInstanceId},
    game::{GameCommand, GameState},
    intrusion::{DeviceCommand, DigitalRoutine, IntrusionDirective, SecurityTraceId},
    world::{Direction, GridPos},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub const MAX_COMMANDS: usize = 50_000;
pub const MAX_GENERATION_VERSION: u8 = 91;
const REPLAY_RECOVERY_SCHEMA: u8 = 1;
pub const CURRENT_RECOVERY_SCHEMA: u8 = 2;
const CURRENT_CRASH_RECOVERY_SCHEMA: u8 = 1;
const MAX_BYTES: u64 = 16 * 1024 * 1024;
const MAX_SAVED_PACKAGES: usize = 1024;

/// Human-readable package identity stored alongside the authoritative content
/// fingerprints. Strings keep the JSON format independent from semver's serde
/// representation while validation still enforces both identifier formats.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedPackage {
    pub id: String,
    pub version: String,
}

impl SavedPackage {
    pub fn new(id: impl Into<String>, version: impl Into<String>) -> Result<Self, String> {
        let package = Self {
            id: id.into(),
            version: version.into(),
        };
        package.validate()?;
        Ok(package)
    }

    fn validate(&self) -> Result<(), String> {
        self.id
            .parse::<project_rl::content::PackageId>()
            .map_err(|error| {
                format!("Identifiant de paquet invalide dans la suspension : {error}")
            })?;
        semver::Version::parse(&self.version).map_err(|error| {
            format!(
                "Version invalide pour le paquet '{}' dans la suspension : {error}",
                self.id
            )
        })?;
        Ok(())
    }
}

/// Versioned recovery payload, deliberately independent from world generation.
/// Legacy suspensions omit it and continue to use their historical replay path.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecoveryPayload {
    Replay {
        schema: u8,
        command_count: u32,
    },
    Snapshot {
        schema: u8,
        command_count: u32,
        state: String,
    },
}

impl RecoveryPayload {
    pub fn replay(command_count: usize) -> Result<Self, String> {
        Ok(Self::Replay {
            schema: REPLAY_RECOVERY_SCHEMA,
            command_count: u32::try_from(command_count)
                .map_err(|_| "Journal de suspension trop long.".to_owned())?,
        })
    }

    pub fn snapshot(command_count: usize, state: String) -> Result<Self, String> {
        if state.is_empty() {
            return Err("Instantané moteur vide.".to_owned());
        }
        Ok(Self::Snapshot {
            schema: CURRENT_RECOVERY_SCHEMA,
            command_count: u32::try_from(command_count)
                .map_err(|_| "Journal de suspension trop long.".to_owned())?,
            state,
        })
    }

    pub fn snapshot_state(&self) -> Option<&str> {
        match self {
            Self::Snapshot { state, .. } => Some(state),
            Self::Replay { .. } => None,
        }
    }

    fn validate(&self, commands: usize) -> Result<(), String> {
        match self {
            Self::Replay {
                schema,
                command_count,
            } => {
                if *schema != REPLAY_RECOVERY_SCHEMA {
                    return Err(
                        "Schéma de restauration non pris en charge ; suspension conservée."
                            .to_owned(),
                    );
                }
                if usize::try_from(*command_count).ok() != Some(commands) {
                    return Err(
                        "Le descripteur de restauration ne correspond pas au journal.".to_owned(),
                    );
                }
            }
            Self::Snapshot {
                schema,
                command_count,
                state,
            } => {
                if *schema != CURRENT_RECOVERY_SCHEMA || state.is_empty() {
                    return Err(
                        "Schéma de restauration non pris en charge ; suspension conservée."
                            .to_owned(),
                    );
                }
                if usize::try_from(*command_count).ok() != Some(commands) {
                    return Err(
                        "L'instantané ne correspond pas au journal de commandes.".to_owned()
                    );
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "directive", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedDroneDirective {
    Manifest {
        position: [i32; 2],
    },
    Escort {
        drone: u64,
        distance: u8,
    },
    Patrol {
        drone: u64,
        waypoints: Vec<[i32; 2]>,
        blocked_response: RecordedPatrolBlockedResponse,
        #[serde(default)]
        autonomous: bool,
    },
    MobileDecoy {
        drone: u64,
        destination: [i32; 2],
    },
    Collect {
        drone: u64,
        item: u64,
    },
    CoordinateFire {
        drones: Vec<u64>,
        target: u64,
    },
    Interpose {
        drone: u64,
        ally: u64,
    },
    Conditional {
        drone: u64,
        condition: RecordedDroneCondition,
        response: RecordedDroneConditionalResponse,
    },
    Deploy {
        assignments: Vec<RecordedDroneDeploymentAssignment>,
    },
    EmergencyReturn {
        drones: Vec<u64>,
        destination: [i32; 2],
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedPatrolBlockedResponse {
    Stop,
    Return,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "condition", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedDroneCondition {
    IntegrityBelowPercent { percent: u8 },
    EnergyBelowPercent { percent: u8 },
    LocallyPerceivedDanger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "response", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedDroneConditionalResponse {
    Stop,
    Return,
    Protect { ally: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedDroneDeploymentRole {
    Hold,
    Escort,
    Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedDroneDeploymentAssignment {
    drone: u64,
    destination: [i32; 2],
    role: RecordedDroneDeploymentRole,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "directive", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedEngineeringDirective {
    Component {
        target: u64,
        component: String,
    },
    WreckComponent {
        wreck: u64,
        component: String,
    },
    TuneModule {
        module: u64,
        tuning: RecordedModuleTuning,
    },
    OverclockModule {
        module: u64,
    },
    Bypass {
        target: u64,
        receiver: String,
        donor: String,
    },
    Module {
        module: u64,
    },
    AssembleAt {
        position: [i32; 2],
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "directive", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedIntrusionDirective {
    Interface {
        position: [i32; 2],
    },
    Command {
        position: [i32; 2],
        command: RecordedDeviceCommand,
    },
    Routine {
        position: [i32; 2],
        routine: RecordedDigitalRoutine,
    },
    Trace {
        trace: u64,
    },
    Subnet {
        positions: Vec<[i32; 2]>,
        command: RecordedDeviceCommand,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "directive", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedElectronicDirective {
    Pulse { direction: Option<u8> },
    Target { target: u64 },
    Jam { channel: RecordedElectronicChannel },
    Purge { target: u64, program: u64 },
    Cascade { targets: Vec<u64> },
    DeployBeacon { position: [i32; 2] },
    ActivateBeacon { beacon: u64 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedElectronicChannel {
    OpticalSensor,
    ThermalSensor,
    ControlLink,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedDeviceCommand {
    Open,
    Close,
    Disable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedDigitalRoutine {
    AutomaticResponse,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedModuleTuning {
    Economy,
    Power,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecordedCompanionBehavior {
    Follow,
    Defensive,
    Aggressive,
    Passive,
}

impl RecordedCompanionBehavior {
    fn record(behavior: CompanionBehavior) -> Self {
        match behavior {
            CompanionBehavior::Follow => Self::Follow,
            CompanionBehavior::Defensive => Self::Defensive,
            CompanionBehavior::Aggressive => Self::Aggressive,
            CompanionBehavior::Passive => Self::Passive,
        }
    }

    const fn restore(self) -> CompanionBehavior {
        match self {
            Self::Follow => CompanionBehavior::Follow,
            Self::Defensive => CompanionBehavior::Defensive,
            Self::Aggressive => CompanionBehavior::Aggressive,
            Self::Passive => CompanionBehavior::Passive,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedCommand {
    Move {
        direction: u8,
    },
    Wait,
    CompanionBehavior {
        behavior: RecordedCompanionBehavior,
    },
    Interact {
        x: i32,
        y: i32,
    },
    Attack {
        slot: u8,
        target: u64,
    },
    AttackAt {
        slot: u8,
        x: i32,
        y: i32,
    },
    Equip {
        slot: u8,
        item: u64,
    },
    EquipItem {
        slot: String,
        item: u64,
    },
    UseItem {
        item: u64,
    },
    PickUp,
    Drop {
        item: u64,
    },
    Buy {
        merchant: u64,
        item: String,
    },
    BuyResale {
        merchant: u64,
        listing: u64,
    },
    Gamble {
        merchant: u64,
        item: String,
    },
    Sell {
        merchant: u64,
        item: u64,
    },
    Treatment {
        healer: u64,
    },
    AcceptQuest {
        giver: u64,
        quest: String,
    },
    CompleteQuest {
        giver: u64,
        quest: String,
    },
    ChooseDialogue {
        speaker: u64,
        node: String,
        choice: u16,
    },
    Ability {
        slot: u8,
        x: i32,
        y: i32,
    },
    Learn {
        technique: String,
    },
    Technique {
        technique: String,
        targets: Vec<u64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        weapon_slot: Option<u8>,
    },
    TechniqueOnComponent {
        technique: String,
        target: u64,
        component: String,
        weapon_slot: u8,
    },
    TechniqueAt {
        technique: String,
        x: i32,
        y: i32,
        weapon_slot: u8,
    },
    DroneTechnique {
        technique: String,
        directive: RecordedDroneDirective,
    },
    EngineeringTechnique {
        technique: String,
        directive: RecordedEngineeringDirective,
    },
    IntrusionTechnique {
        technique: String,
        directive: RecordedIntrusionDirective,
    },
    ElectronicWarfareTechnique {
        technique: String,
        directive: RecordedElectronicDirective,
    },
}

impl RecordedCommand {
    pub fn record(command: &GameCommand) -> Self {
        match command {
            GameCommand::Move(direction) => Self::Move {
                direction: match direction {
                    Direction::North => 0,
                    Direction::East => 1,
                    Direction::South => 2,
                    Direction::West => 3,
                },
            },
            GameCommand::Wait => Self::Wait,
            GameCommand::SetCompanionBehavior { behavior } => Self::CompanionBehavior {
                behavior: RecordedCompanionBehavior::record(*behavior),
            },
            GameCommand::Interact { target } => Self::Interact {
                x: target.x,
                y: target.y,
            },
            GameCommand::Attack { slot, target } => Self::Attack {
                slot: *slot,
                target: target.get(),
            },
            GameCommand::AttackAt { slot, target } => Self::AttackAt {
                slot: *slot,
                x: target.x,
                y: target.y,
            },
            GameCommand::EquipWeapon { slot, item } => Self::Equip {
                slot: *slot,
                item: item.get(),
            },
            GameCommand::EquipItem { slot, item } => Self::EquipItem {
                slot: slot.to_string(),
                item: item.get(),
            },
            GameCommand::UseItem { item } => Self::UseItem { item: item.get() },
            GameCommand::PickUp => Self::PickUp,
            GameCommand::DropItem { item } => Self::Drop { item: item.get() },
            GameCommand::BuyItem { merchant, item } => Self::Buy {
                merchant: merchant.get(),
                item: item.to_string(),
            },
            GameCommand::BuyResaleItem { merchant, listing } => Self::BuyResale {
                merchant: merchant.get(),
                listing: *listing,
            },
            GameCommand::GambleItem { merchant, item } => Self::Gamble {
                merchant: merchant.get(),
                item: item.to_string(),
            },
            GameCommand::SellItem { merchant, item } => Self::Sell {
                merchant: merchant.get(),
                item: item.get(),
            },
            GameCommand::ReceiveTreatment { healer } => Self::Treatment {
                healer: healer.get(),
            },
            GameCommand::AcceptQuest { giver, quest } => Self::AcceptQuest {
                giver: giver.get(),
                quest: quest.to_string(),
            },
            GameCommand::CompleteQuest { giver, quest } => Self::CompleteQuest {
                giver: giver.get(),
                quest: quest.to_string(),
            },
            GameCommand::ChooseDialogue {
                speaker,
                node,
                choice,
            } => Self::ChooseDialogue {
                speaker: speaker.get(),
                node: node.clone(),
                choice: *choice,
            },
            GameCommand::UseAbility { slot, target } => Self::Ability {
                slot: *slot,
                x: target.x,
                y: target.y,
            },
            GameCommand::LearnTechnique { technique } => Self::Learn {
                technique: technique.to_string(),
            },
            GameCommand::UseTechnique {
                technique,
                targets,
                weapon_slot,
            } => Self::Technique {
                technique: technique.to_string(),
                targets: targets.iter().map(|id| id.get()).collect(),
                weapon_slot: *weapon_slot,
            },
            GameCommand::UseTechniqueOnComponent {
                technique,
                target,
                component,
                weapon_slot,
            } => Self::TechniqueOnComponent {
                technique: technique.to_string(),
                target: target.get(),
                component: component.to_string(),
                weapon_slot: *weapon_slot,
            },
            GameCommand::UseTechniqueAt {
                technique,
                target,
                weapon_slot,
            } => Self::TechniqueAt {
                technique: technique.to_string(),
                x: target.x,
                y: target.y,
                weapon_slot: *weapon_slot,
            },
            GameCommand::UseDroneTechnique {
                technique,
                directive,
            } => Self::DroneTechnique {
                technique: technique.to_string(),
                directive: RecordedDroneDirective::record(directive),
            },
            GameCommand::UseEngineeringTechnique {
                technique,
                directive,
            } => Self::EngineeringTechnique {
                technique: technique.to_string(),
                directive: RecordedEngineeringDirective::record(directive),
            },
            GameCommand::UseIntrusionTechnique {
                technique,
                directive,
            } => Self::IntrusionTechnique {
                technique: technique.to_string(),
                directive: RecordedIntrusionDirective::record(directive),
            },
            GameCommand::UseElectronicWarfareTechnique {
                technique,
                directive,
            } => Self::ElectronicWarfareTechnique {
                technique: technique.to_string(),
                directive: RecordedElectronicDirective::record(directive),
            },
        }
    }

    pub fn command(&self, game: &GameState) -> Result<GameCommand, String> {
        let entity = |value| -> Result<EntityId, String> {
            game.actors()
                .iter()
                .map(|(id, _)| id)
                .find(|id| id.get() == value)
                .ok_or_else(|| format!("Cible absente du rejeu : {value}"))
        };
        let item = |value| -> Result<ItemInstanceId, String> {
            game.player_inventory()
                .iter()
                .map(|entry| entry.instance())
                .find(|id| id.get() == value)
                .ok_or_else(|| format!("Objet absent du rejeu : {value}"))
        };
        let ground_item = |value| -> Result<GroundItemId, String> {
            game.ground_items()
                .iter()
                .map(|(id, _)| id)
                .find(|id| id.get() == value)
                .ok_or_else(|| format!("Butin absent du rejeu : {value}"))
        };
        let wreck = |value| -> Result<WreckId, String> {
            game.wrecks()
                .iter()
                .map(|wreck| wreck.id())
                .find(|id| id.get() == value)
                .ok_or_else(|| format!("Carcasse absente du rejeu : {value}"))
        };
        Ok(match self {
            Self::Move { direction } => GameCommand::Move(match direction {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => return Err("Direction invalide.".to_owned()),
            }),
            Self::Wait => GameCommand::Wait,
            Self::CompanionBehavior { behavior } => GameCommand::SetCompanionBehavior {
                behavior: behavior.restore(),
            },
            Self::Interact { x, y } => GameCommand::Interact {
                target: GridPos::new(*x, *y),
            },
            Self::Attack { slot, target } => GameCommand::Attack {
                slot: *slot,
                target: entity(*target)?,
            },
            Self::AttackAt { slot, x, y } => GameCommand::AttackAt {
                slot: *slot,
                target: GridPos::new(*x, *y),
            },
            Self::Equip { slot, item: value } => GameCommand::EquipWeapon {
                slot: *slot,
                item: item(*value)?,
            },
            Self::EquipItem { slot, item: value } => GameCommand::EquipItem {
                slot: slot
                    .parse()
                    .map_err(|error| format!("Emplacement d'équipement invalide : {error}"))?,
                item: item(*value)?,
            },
            Self::UseItem { item: value } => GameCommand::UseItem {
                item: item(*value)?,
            },
            Self::PickUp => GameCommand::PickUp,
            Self::Drop { item: value } => GameCommand::DropItem {
                item: item(*value)?,
            },
            Self::Buy {
                merchant,
                item: definition,
            } => GameCommand::BuyItem {
                merchant: entity(*merchant)?,
                item: definition
                    .parse()
                    .map_err(|error| format!("Objet marchand invalide : {error}"))?,
            },
            Self::BuyResale { merchant, listing } => GameCommand::BuyResaleItem {
                merchant: entity(*merchant)?,
                listing: *listing,
            },
            Self::Gamble {
                merchant,
                item: definition,
            } => GameCommand::GambleItem {
                merchant: entity(*merchant)?,
                item: definition
                    .parse()
                    .map_err(|error| format!("Objet de pari invalide : {error}"))?,
            },
            Self::Sell {
                merchant,
                item: instance,
            } => GameCommand::SellItem {
                merchant: entity(*merchant)?,
                item: item(*instance)?,
            },
            Self::Treatment { healer } => GameCommand::ReceiveTreatment {
                healer: entity(*healer)?,
            },
            Self::AcceptQuest { giver, quest } => GameCommand::AcceptQuest {
                giver: entity(*giver)?,
                quest: quest
                    .parse()
                    .map_err(|error| format!("Quête invalide : {error}"))?,
            },
            Self::CompleteQuest { giver, quest } => GameCommand::CompleteQuest {
                giver: entity(*giver)?,
                quest: quest
                    .parse()
                    .map_err(|error| format!("Quête invalide : {error}"))?,
            },
            Self::Ability { slot, x, y } => GameCommand::UseAbility {
                slot: *slot,
                target: GridPos::new(*x, *y),
            },
            Self::ChooseDialogue {
                speaker,
                node,
                choice,
            } => {
                if node.is_empty() || node.len() > 128 || *choice >= 12 {
                    return Err("Choix de dialogue invalide.".to_owned());
                }
                GameCommand::ChooseDialogue {
                    speaker: entity(*speaker)?,
                    node: node.clone(),
                    choice: *choice,
                }
            }
            Self::Learn { technique } => GameCommand::LearnTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
            },
            Self::Technique {
                technique,
                targets,
                weapon_slot,
            } => GameCommand::UseTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                targets: targets
                    .iter()
                    .map(|id| entity(*id))
                    .collect::<Result<_, _>>()?,
                weapon_slot: *weapon_slot,
            },
            Self::TechniqueOnComponent {
                technique,
                target,
                component,
                weapon_slot,
            } => GameCommand::UseTechniqueOnComponent {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                target: entity(*target)?,
                component: component
                    .parse()
                    .map_err(|error| format!("Composant invalide : {error}"))?,
                weapon_slot: *weapon_slot,
            },
            Self::TechniqueAt {
                technique,
                x,
                y,
                weapon_slot,
            } => GameCommand::UseTechniqueAt {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                target: GridPos::new(*x, *y),
                weapon_slot: *weapon_slot,
            },
            Self::DroneTechnique {
                technique,
                directive,
            } => GameCommand::UseDroneTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                directive: directive.restore(&entity, &ground_item)?,
            },
            Self::EngineeringTechnique {
                technique,
                directive,
            } => GameCommand::UseEngineeringTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                directive: directive.restore(&entity, &item, &wreck)?,
            },
            Self::IntrusionTechnique {
                technique,
                directive,
            } => GameCommand::UseIntrusionTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                directive: directive.restore()?,
            },
            Self::ElectronicWarfareTechnique {
                technique,
                directive,
            } => GameCommand::UseElectronicWarfareTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                directive: directive.restore(&entity)?,
            },
        })
    }
}

impl RecordedElectronicDirective {
    fn record(directive: &ElectronicDirective) -> Self {
        let direction = |value: Direction| match value {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        };
        let channel = |value: ElectronicChannel| match value {
            ElectronicChannel::OpticalSensor => RecordedElectronicChannel::OpticalSensor,
            ElectronicChannel::ThermalSensor => RecordedElectronicChannel::ThermalSensor,
            ElectronicChannel::ControlLink => RecordedElectronicChannel::ControlLink,
        };
        match directive {
            ElectronicDirective::Pulse { direction: value } => Self::Pulse {
                direction: value.map(direction),
            },
            ElectronicDirective::Target { target } => Self::Target {
                target: target.get(),
            },
            ElectronicDirective::Jam { channel: value } => Self::Jam {
                channel: channel(*value),
            },
            ElectronicDirective::Purge { target, program } => Self::Purge {
                target: target.get(),
                program: program.get(),
            },
            ElectronicDirective::Cascade { targets } => Self::Cascade {
                targets: targets.iter().map(|target| target.get()).collect(),
            },
            ElectronicDirective::DeployBeacon { position } => Self::DeployBeacon {
                position: [position.x, position.y],
            },
            ElectronicDirective::ActivateBeacon { beacon } => Self::ActivateBeacon {
                beacon: beacon.get(),
            },
        }
    }

    fn restore(
        &self,
        entity: &impl Fn(u64) -> Result<EntityId, String>,
    ) -> Result<ElectronicDirective, String> {
        let direction = |value| match value {
            0 => Ok(Direction::North),
            1 => Ok(Direction::East),
            2 => Ok(Direction::South),
            3 => Ok(Direction::West),
            _ => Err(format!("Direction électronique invalide : {value}")),
        };
        let channel = |value: RecordedElectronicChannel| match value {
            RecordedElectronicChannel::OpticalSensor => ElectronicChannel::OpticalSensor,
            RecordedElectronicChannel::ThermalSensor => ElectronicChannel::ThermalSensor,
            RecordedElectronicChannel::ControlLink => ElectronicChannel::ControlLink,
        };
        Ok(match self {
            Self::Pulse { direction: value } => ElectronicDirective::Pulse {
                direction: value.map(direction).transpose()?,
            },
            Self::Target { target } => ElectronicDirective::Target {
                target: entity(*target)?,
            },
            Self::Jam { channel: value } => ElectronicDirective::Jam {
                channel: channel(*value),
            },
            Self::Purge { target, program } => ElectronicDirective::Purge {
                target: entity(*target)?,
                program: HostileProgramId::from_raw(*program)
                    .ok_or_else(|| "Programme hostile invalide : 0".to_owned())?,
            },
            Self::Cascade { targets } => ElectronicDirective::Cascade {
                targets: targets
                    .iter()
                    .map(|target| entity(*target))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            Self::DeployBeacon { position } => ElectronicDirective::DeployBeacon {
                position: GridPos::new(position[0], position[1]),
            },
            Self::ActivateBeacon { beacon } => ElectronicDirective::ActivateBeacon {
                beacon: entity(*beacon)?,
            },
        })
    }
}

impl RecordedIntrusionDirective {
    fn record(directive: &IntrusionDirective) -> Self {
        let position = |value: GridPos| [value.x, value.y];
        let command = |value: DeviceCommand| match value {
            DeviceCommand::Open => RecordedDeviceCommand::Open,
            DeviceCommand::Close => RecordedDeviceCommand::Close,
            DeviceCommand::Disable => RecordedDeviceCommand::Disable,
        };
        match directive {
            IntrusionDirective::Interface { position: at } => Self::Interface {
                position: position(*at),
            },
            IntrusionDirective::Command {
                position: at,
                command: value,
            } => Self::Command {
                position: position(*at),
                command: command(*value),
            },
            IntrusionDirective::Routine {
                position: at,
                routine,
            } => Self::Routine {
                position: position(*at),
                routine: match routine {
                    DigitalRoutine::AutomaticResponse => RecordedDigitalRoutine::AutomaticResponse,
                },
            },
            IntrusionDirective::Trace { trace } => Self::Trace { trace: trace.get() },
            IntrusionDirective::Subnet {
                positions,
                command: value,
            } => Self::Subnet {
                positions: positions.iter().copied().map(position).collect(),
                command: command(*value),
            },
        }
    }

    fn restore(&self) -> Result<IntrusionDirective, String> {
        let position = |value: [i32; 2]| GridPos::new(value[0], value[1]);
        let command = |value: RecordedDeviceCommand| match value {
            RecordedDeviceCommand::Open => DeviceCommand::Open,
            RecordedDeviceCommand::Close => DeviceCommand::Close,
            RecordedDeviceCommand::Disable => DeviceCommand::Disable,
        };
        Ok(match self {
            Self::Interface { position: at } => IntrusionDirective::Interface {
                position: position(*at),
            },
            Self::Command {
                position: at,
                command: value,
            } => IntrusionDirective::Command {
                position: position(*at),
                command: command(*value),
            },
            Self::Routine {
                position: at,
                routine,
            } => IntrusionDirective::Routine {
                position: position(*at),
                routine: match routine {
                    RecordedDigitalRoutine::AutomaticResponse => DigitalRoutine::AutomaticResponse,
                },
            },
            Self::Trace { trace } => IntrusionDirective::Trace {
                trace: SecurityTraceId::from_raw(*trace)
                    .ok_or_else(|| "Trace numérique invalide : 0".to_owned())?,
            },
            Self::Subnet {
                positions,
                command: value,
            } => IntrusionDirective::Subnet {
                positions: positions.iter().copied().map(position).collect(),
                command: command(*value),
            },
        })
    }
}

impl RecordedEngineeringDirective {
    fn record(directive: &EngineeringDirective) -> Self {
        match directive {
            EngineeringDirective::Component { target, component } => Self::Component {
                target: target.get(),
                component: component.to_string(),
            },
            EngineeringDirective::WreckComponent { wreck, component } => Self::WreckComponent {
                wreck: wreck.get(),
                component: component.to_string(),
            },
            EngineeringDirective::TuneModule { module, tuning } => Self::TuneModule {
                module: module.get(),
                tuning: match tuning {
                    ModuleTuning::Economy => RecordedModuleTuning::Economy,
                    ModuleTuning::Power => RecordedModuleTuning::Power,
                },
            },
            EngineeringDirective::OverclockModule { module } => Self::OverclockModule {
                module: module.get(),
            },
            EngineeringDirective::Bypass {
                target,
                receiver,
                donor,
            } => Self::Bypass {
                target: target.get(),
                receiver: receiver.to_string(),
                donor: donor.to_string(),
            },
            EngineeringDirective::Module { module } => Self::Module {
                module: module.get(),
            },
            EngineeringDirective::AssembleAt { position } => Self::AssembleAt {
                position: [position.x, position.y],
            },
        }
    }

    fn restore(
        &self,
        entity: &impl Fn(u64) -> Result<EntityId, String>,
        item: &impl Fn(u64) -> Result<ItemInstanceId, String>,
        wreck: &impl Fn(u64) -> Result<WreckId, String>,
    ) -> Result<EngineeringDirective, String> {
        let component = |value: &str| -> Result<BodyComponentId, String> {
            value
                .parse()
                .map_err(|error| format!("Composant invalide : {error}"))
        };
        Ok(match self {
            Self::Component {
                target,
                component: component_id,
            } => EngineeringDirective::Component {
                target: entity(*target)?,
                component: component(component_id)?,
            },
            Self::WreckComponent {
                wreck: wreck_id,
                component: component_id,
            } => EngineeringDirective::WreckComponent {
                wreck: wreck(*wreck_id)?,
                component: component(component_id)?,
            },
            Self::TuneModule { module, tuning } => EngineeringDirective::TuneModule {
                module: item(*module)?,
                tuning: match tuning {
                    RecordedModuleTuning::Economy => ModuleTuning::Economy,
                    RecordedModuleTuning::Power => ModuleTuning::Power,
                },
            },
            Self::OverclockModule { module } => EngineeringDirective::OverclockModule {
                module: item(*module)?,
            },
            Self::Bypass {
                target,
                receiver,
                donor,
            } => EngineeringDirective::Bypass {
                target: entity(*target)?,
                receiver: component(receiver)?,
                donor: component(donor)?,
            },
            Self::Module { module } => EngineeringDirective::Module {
                module: item(*module)?,
            },
            Self::AssembleAt { position } => EngineeringDirective::AssembleAt {
                position: GridPos::new(position[0], position[1]),
            },
        })
    }
}

impl RecordedDroneDirective {
    fn record(directive: &DroneDirective) -> Self {
        match directive {
            DroneDirective::Manifest { position } => Self::Manifest {
                position: [position.x, position.y],
            },
            DroneDirective::Escort { drone, distance } => Self::Escort {
                drone: drone.get(),
                distance: *distance,
            },
            DroneDirective::Patrol {
                drone,
                waypoints,
                blocked_response,
                autonomous,
            } => Self::Patrol {
                drone: drone.get(),
                waypoints: waypoints.iter().map(|point| [point.x, point.y]).collect(),
                blocked_response: match blocked_response {
                    PatrolBlockedResponse::Stop => RecordedPatrolBlockedResponse::Stop,
                    PatrolBlockedResponse::Return => RecordedPatrolBlockedResponse::Return,
                },
                autonomous: *autonomous,
            },
            DroneDirective::MobileDecoy { drone, destination } => Self::MobileDecoy {
                drone: drone.get(),
                destination: [destination.x, destination.y],
            },
            DroneDirective::Collect { drone, item } => Self::Collect {
                drone: drone.get(),
                item: item.get(),
            },
            DroneDirective::CoordinateFire { drones, target } => Self::CoordinateFire {
                drones: drones.iter().map(|entity| entity.get()).collect(),
                target: target.get(),
            },
            DroneDirective::Interpose { drone, ally } => Self::Interpose {
                drone: drone.get(),
                ally: ally.get(),
            },
            DroneDirective::Conditional {
                drone,
                condition,
                response,
            } => Self::Conditional {
                drone: drone.get(),
                condition: match condition {
                    DroneCondition::IntegrityBelowPercent(percent) => {
                        RecordedDroneCondition::IntegrityBelowPercent { percent: *percent }
                    }
                    DroneCondition::EnergyBelowPercent(percent) => {
                        RecordedDroneCondition::EnergyBelowPercent { percent: *percent }
                    }
                    DroneCondition::LocallyPerceivedDanger => {
                        RecordedDroneCondition::LocallyPerceivedDanger
                    }
                },
                response: match response {
                    DroneConditionalResponse::Stop => RecordedDroneConditionalResponse::Stop,
                    DroneConditionalResponse::Return => RecordedDroneConditionalResponse::Return,
                    DroneConditionalResponse::Protect(ally) => {
                        RecordedDroneConditionalResponse::Protect { ally: ally.get() }
                    }
                },
            },
            DroneDirective::Deploy { assignments } => Self::Deploy {
                assignments: assignments
                    .iter()
                    .map(|assignment| RecordedDroneDeploymentAssignment {
                        drone: assignment.drone.get(),
                        destination: [assignment.destination.x, assignment.destination.y],
                        role: match assignment.role {
                            DroneDeploymentRole::Hold => RecordedDroneDeploymentRole::Hold,
                            DroneDeploymentRole::Escort => RecordedDroneDeploymentRole::Escort,
                            DroneDeploymentRole::Guard => RecordedDroneDeploymentRole::Guard,
                        },
                    })
                    .collect(),
            },
            DroneDirective::EmergencyReturn {
                drones,
                destination,
            } => Self::EmergencyReturn {
                drones: drones.iter().map(|entity| entity.get()).collect(),
                destination: [destination.x, destination.y],
            },
        }
    }

    fn restore(
        &self,
        entity: &impl Fn(u64) -> Result<EntityId, String>,
        ground_item: &impl Fn(u64) -> Result<GroundItemId, String>,
    ) -> Result<DroneDirective, String> {
        Ok(match self {
            Self::Manifest { position } => DroneDirective::Manifest {
                position: GridPos::new(position[0], position[1]),
            },
            Self::Escort { drone, distance } => DroneDirective::Escort {
                drone: entity(*drone)?,
                distance: *distance,
            },
            Self::Patrol {
                drone,
                waypoints,
                blocked_response,
                autonomous,
            } => DroneDirective::Patrol {
                drone: entity(*drone)?,
                waypoints: waypoints
                    .iter()
                    .map(|point| GridPos::new(point[0], point[1]))
                    .collect(),
                blocked_response: match blocked_response {
                    RecordedPatrolBlockedResponse::Stop => PatrolBlockedResponse::Stop,
                    RecordedPatrolBlockedResponse::Return => PatrolBlockedResponse::Return,
                },
                autonomous: *autonomous,
            },
            Self::MobileDecoy { drone, destination } => DroneDirective::MobileDecoy {
                drone: entity(*drone)?,
                destination: GridPos::new(destination[0], destination[1]),
            },
            Self::Collect { drone, item } => DroneDirective::Collect {
                drone: entity(*drone)?,
                item: ground_item(*item)?,
            },
            Self::CoordinateFire { drones, target } => DroneDirective::CoordinateFire {
                drones: drones
                    .iter()
                    .map(|drone| entity(*drone))
                    .collect::<Result<_, _>>()?,
                target: entity(*target)?,
            },
            Self::Interpose { drone, ally } => DroneDirective::Interpose {
                drone: entity(*drone)?,
                ally: entity(*ally)?,
            },
            Self::Conditional {
                drone,
                condition,
                response,
            } => DroneDirective::Conditional {
                drone: entity(*drone)?,
                condition: match condition {
                    RecordedDroneCondition::IntegrityBelowPercent { percent } => {
                        DroneCondition::IntegrityBelowPercent(*percent)
                    }
                    RecordedDroneCondition::EnergyBelowPercent { percent } => {
                        DroneCondition::EnergyBelowPercent(*percent)
                    }
                    RecordedDroneCondition::LocallyPerceivedDanger => {
                        DroneCondition::LocallyPerceivedDanger
                    }
                },
                response: match response {
                    RecordedDroneConditionalResponse::Stop => DroneConditionalResponse::Stop,
                    RecordedDroneConditionalResponse::Return => DroneConditionalResponse::Return,
                    RecordedDroneConditionalResponse::Protect { ally } => {
                        DroneConditionalResponse::Protect(entity(*ally)?)
                    }
                },
            },
            Self::Deploy { assignments } => DroneDirective::Deploy {
                assignments: assignments
                    .iter()
                    .map(|assignment| {
                        Ok(DroneDeploymentAssignment {
                            drone: entity(assignment.drone)?,
                            destination: GridPos::new(
                                assignment.destination[0],
                                assignment.destination[1],
                            ),
                            role: match assignment.role {
                                RecordedDroneDeploymentRole::Hold => DroneDeploymentRole::Hold,
                                RecordedDroneDeploymentRole::Escort => DroneDeploymentRole::Escort,
                                RecordedDroneDeploymentRole::Guard => DroneDeploymentRole::Guard,
                            },
                        })
                    })
                    .collect::<Result<_, String>>()?,
            },
            Self::EmergencyReturn {
                drones,
                destination,
            } => DroneDirective::EmergencyReturn {
                drones: drones
                    .iter()
                    .map(|drone| entity(*drone))
                    .collect::<Result<_, _>>()?,
                destination: GridPos::new(destination[0], destination[1]),
            },
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Suspension {
    pub version: u8,
    pub build: String,
    pub rules: u64,
    /// Absent only in legacy suspensions written before package identities were
    /// persisted explicitly. Fingerprints remain the final compatibility proof.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<SavedPackage>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loot_rules: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_rules: Option<u64>,
    pub seed: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub character_class: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub starting_attributes: Option<[u8; 5]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<RecoveryPayload>,
    pub commands: Vec<RecordedCommand>,
    pub state: u64,
    pub active_weapon_slot: u8,
    pub selected_target: Option<u64>,
    pub report: Vec<String>,
    pub log: Vec<String>,
}

pub fn fingerprint(value: &impl std::fmt::Debug) -> u64 {
    format!("{value:?}")
        .bytes()
        .fold(0xcbf29ce484222325_u64, |hash, byte| {
            (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3)
        })
}

impl Suspension {
    pub fn validate(&self) -> Result<(), String> {
        if !(1..=MAX_GENERATION_VERSION).contains(&self.version) {
            return Err(
                "Format de sauvegarde non pris en charge ; partie suspendue conservée.".to_owned(),
            );
        }
        if (self.version >= 3) != self.loot_rules.is_some() {
            return Err("Empreinte de butin incompatible avec le format de suspension.".to_owned());
        }
        if (self.version >= 4) != self.world_rules.is_some() {
            return Err("Empreinte du monde incompatible avec le format de suspension.".to_owned());
        }
        if self.character_class.is_some() != self.starting_attributes.is_some() {
            return Err(
                "Protocole et attributs de départ doivent être enregistrés ensemble.".to_owned(),
            );
        }
        if let Some(packages) = &self.packages {
            if packages.is_empty() || packages.len() > MAX_SAVED_PACKAGES {
                return Err("Liste de paquets invalide dans la suspension.".to_owned());
            }
            let mut has_core = false;
            for package in packages {
                package.validate()?;
                if package.id == "core" {
                    has_core = true;
                }
            }
            let unique = packages
                .iter()
                .map(|package| package.id.as_str())
                .collect::<std::collections::BTreeSet<_>>();
            if unique.len() != packages.len() {
                return Err("Un paquet est dupliqué dans la suspension.".to_owned());
            }
            if !has_core {
                return Err("Paquet central absent de la suspension.".to_owned());
            }
        }
        if self.version < 34 && self.character_class.is_some() {
            return Err(
                "Un ancien format de suspension ne peut pas contenir de protocole.".to_owned(),
            );
        }
        if self
            .character_class
            .as_ref()
            .is_some_and(|id| id.parse::<project_rl::content::ContentId>().is_err())
        {
            return Err("Identifiant de protocole invalide ; suspension conservée.".to_owned());
        }
        // Build identifies the producer and gates only the opaque snapshot.
        // Replay compatibility is still proved independently from this value.
        if self.build.len() != 16 || !self.build.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err("Identifiant de compilation invalide ; suspension conservée.".to_owned());
        }
        if self.commands.len() > MAX_COMMANDS
            || self.report.len() > 4096
            || self.log.len() > 6
            || self
                .report
                .iter()
                .chain(&self.log)
                .any(|text| text.len() > 8192)
        {
            return Err("Suspension trop volumineuse pour ce prototype.".to_owned());
        }
        if let Some(recovery) = &self.recovery {
            recovery.validate(self.commands.len())?;
        }
        Ok(())
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|error| error.to_string())?;
        let mut source = Vec::new();
        file.take(MAX_BYTES + 1)
            .read_to_end(&mut source)
            .map_err(|error| error.to_string())?;
        if source.len() as u64 > MAX_BYTES {
            return Err("Fichier de suspension trop volumineux.".to_owned());
        }
        let result: Self = serde_json::from_slice(&source).map_err(|error| error.to_string())?;
        result.validate()?;
        Ok(result)
    }

    /// Caller holds the session lock. Never replaces another pending run.
    pub fn write(&self, path: &Path) -> Result<(), String> {
        self.validate()?;
        if path.try_exists().map_err(|error| error.to_string())? {
            return Err("Une suspension existe déjà ; elle n'a pas été remplacée.".to_owned());
        }
        let source = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        if source.len() as u64 > MAX_BYTES {
            return Err("Suspension trop volumineuse.".to_owned());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let temporary = path.with_extension(format!(
            "tmp-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|error| error.to_string())?
                .as_nanos()
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| error.to_string())?;
        let result = file.write_all(&source).and_then(|()| file.sync_all());
        drop(file);
        let result = result.and_then(|()| std::fs::rename(&temporary, path));
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result.map_err(|error| error.to_string())
    }
}

/// One of two alternating emergency checkpoints. It is deliberately wrapped
/// around the ordinary verified suspension format so a crash never gains a
/// weaker restoration path.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CrashRecovery {
    schema: u8,
    sequence: u64,
    suspension: Suspension,
}

impl CrashRecovery {
    pub fn new(sequence: u64, suspension: Suspension) -> Result<Self, String> {
        let recovery = Self {
            schema: CURRENT_CRASH_RECOVERY_SCHEMA,
            sequence,
            suspension,
        };
        recovery.validate()?;
        Ok(recovery)
    }

    pub const fn sequence(&self) -> u64 {
        self.sequence
    }

    pub const fn suspension(&self) -> &Suspension {
        &self.suspension
    }

    fn validate(&self) -> Result<(), String> {
        if self.schema != CURRENT_CRASH_RECOVERY_SCHEMA || self.sequence == 0 {
            return Err("Point de récupération après incident non pris en charge.".to_owned());
        }
        self.suspension.validate()
    }

    pub fn read(path: &Path) -> Result<Self, String> {
        let file = File::open(path).map_err(|error| error.to_string())?;
        let mut source = Vec::new();
        file.take(MAX_BYTES + 1)
            .read_to_end(&mut source)
            .map_err(|error| error.to_string())?;
        if source.len() as u64 > MAX_BYTES {
            return Err("Point de récupération trop volumineux.".to_owned());
        }
        let result: Self = serde_json::from_slice(&source).map_err(|error| error.to_string())?;
        result.validate()?;
        Ok(result)
    }

    /// Replaces only one slot. The other alternating slot remains valid while
    /// this file is serialized, synchronized and renamed.
    pub fn write_replacing(&self, path: &Path) -> Result<(), String> {
        self.validate()?;
        let source = serde_json::to_vec(self).map_err(|error| error.to_string())?;
        if source.len() as u64 > MAX_BYTES {
            return Err("Point de récupération trop volumineux.".to_owned());
        }
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        let temporary = path.with_extension(format!(
            "tmp-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|error| error.to_string())?
                .as_nanos()
        ));
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| error.to_string())?;
        let result = file.write_all(&source).and_then(|()| file.sync_all());
        drop(file);
        if let Err(error) = result {
            let _ = std::fs::remove_file(&temporary);
            return Err(error.to_string());
        }
        if path.try_exists().map_err(|error| error.to_string())? {
            std::fs::remove_file(path).map_err(|error| error.to_string())?;
        }
        if let Err(error) = std::fs::rename(&temporary, path) {
            let _ = std::fs::remove_file(&temporary);
            return Err(error.to_string());
        }
        Ok(())
    }
}

/// OS-held lock, released even after a crash. The empty file can safely persist.
pub fn session_lock(path: &Path) -> Result<File, String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)
        .map_err(|error| error.to_string())?;
    file.try_lock().map_err(|error| {
        format!("Une autre instance utilise déjà cette partie, ou verrou indisponible : {error}")
    })?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::{entity::Actor, world::Map};

    #[test]
    fn recorded_technique_preserves_weapon_slot_and_reads_legacy_commands() {
        let mut game = GameState::new(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            7,
        )
        .unwrap();
        let target = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let command = GameCommand::UseTechnique {
            technique: "core:test_weapon_technique".parse().unwrap(),
            targets: vec![target],
            weapon_slot: Some(2),
        };
        let recorded = RecordedCommand::record(&command);
        assert_eq!(recorded.command(&game), Ok(command));
        assert_eq!(
            serde_json::to_value(&recorded).unwrap()["weapon_slot"],
            serde_json::json!(2)
        );

        let legacy: RecordedCommand = serde_json::from_value(serde_json::json!({
            "action": "technique",
            "technique": "core:test_weapon_technique",
            "targets": [target.get()],
        }))
        .unwrap();
        assert_eq!(
            legacy.command(&game),
            Ok(GameCommand::UseTechnique {
                technique: "core:test_weapon_technique".parse().unwrap(),
                targets: vec![target],
                weapon_slot: None,
            })
        );

        let aimed = GameCommand::UseTechniqueAt {
            technique: "core:test_sweep".parse().unwrap(),
            target: GridPos::new(2, 1),
            weapon_slot: 0,
        };
        let recorded = RecordedCommand::record(&aimed);
        assert_eq!(recorded.command(&game), Ok(aimed));
    }

    #[test]
    fn recovery_schema_is_independent_and_bound_to_its_command_journal() {
        let payload = RecoveryPayload::replay(3).unwrap();
        assert_eq!(payload.validate(3), Ok(()));
        assert!(payload.validate(2).is_err());
        assert_eq!(
            serde_json::to_value(&payload).unwrap(),
            serde_json::json!({
                "mode": "replay",
                "schema": REPLAY_RECOVERY_SCHEMA,
                "command_count": 3,
            })
        );

        let snapshot = RecoveryPayload::snapshot(3, "encoded-state".to_owned()).unwrap();
        assert_eq!(snapshot.validate(3), Ok(()));
        assert_eq!(snapshot.snapshot_state(), Some("encoded-state"));
        assert!(snapshot.validate(2).is_err());

        let future: RecoveryPayload = serde_json::from_value(serde_json::json!({
            "mode": "snapshot",
            "schema": CURRENT_RECOVERY_SCHEMA + 1,
            "command_count": 3,
            "state": "encoded-state",
        }))
        .unwrap();
        assert!(future.validate(3).is_err());
    }

    #[test]
    fn saved_package_requires_canonical_id_and_semantic_version() {
        assert_eq!(
            SavedPackage::new("alice.tools", "1.2.3").unwrap(),
            SavedPackage {
                id: "alice.tools".to_owned(),
                version: "1.2.3".to_owned(),
            }
        );
        assert!(SavedPackage::new("Alice Tools", "1.2.3").is_err());
        assert!(SavedPackage::new("alice.tools", "latest").is_err());
    }

    #[test]
    fn recorded_drone_technique_preserves_physical_assignments_and_roles() {
        let mut game = GameState::new(
            Map::from_ascii("#######\n#.....#\n#.....#\n#######").unwrap(),
            GridPos::new(1, 1),
            11,
        )
        .unwrap();
        let first = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let second = game
            .spawn_actor(Actor::new(GridPos::new(3, 1), 10).unwrap())
            .unwrap();
        let command = GameCommand::UseDroneTechnique {
            technique: "core:drn_09".parse().unwrap(),
            directive: DroneDirective::Deploy {
                assignments: vec![
                    DroneDeploymentAssignment {
                        drone: first,
                        destination: GridPos::new(4, 1),
                        role: DroneDeploymentRole::Guard,
                    },
                    DroneDeploymentAssignment {
                        drone: second,
                        destination: GridPos::new(5, 2),
                        role: DroneDeploymentRole::Escort,
                    },
                ],
            },
        };

        let recorded = RecordedCommand::record(&command);
        assert_eq!(recorded.command(&game), Ok(command));
        let json = serde_json::to_value(&recorded).unwrap();
        assert_eq!(json["action"], serde_json::json!("drone_technique"));
        assert_eq!(json["directive"]["directive"], serde_json::json!("deploy"));
        assert_eq!(json["directive"]["assignments"][0]["drone"], first.get());
    }

    #[test]
    fn recorded_companion_behavior_round_trips_without_consuming_an_actor_id() {
        let game = GameState::new(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            13,
        )
        .unwrap();
        for behavior in CompanionBehavior::ALL {
            let command = GameCommand::SetCompanionBehavior { behavior };
            let recorded = RecordedCommand::record(&command);
            assert_eq!(recorded.command(&game), Ok(command));
            assert_eq!(
                serde_json::to_value(&recorded).unwrap()["action"],
                serde_json::json!("companion_behavior")
            );
        }
    }

    #[test]
    fn recorded_clinic_treatment_preserves_the_provider() {
        let mut game = GameState::new(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            13,
        )
        .unwrap();
        let healer = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let command = GameCommand::ReceiveTreatment { healer };
        let recorded = RecordedCommand::record(&command);
        assert_eq!(recorded.command(&game), Ok(command));
        assert_eq!(
            serde_json::to_value(&recorded).unwrap(),
            serde_json::json!({ "action": "treatment", "healer": healer.get() })
        );
    }

    #[test]
    fn recorded_quest_commands_preserve_the_giver_and_quest_id() {
        let mut game = GameState::new(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            13,
        )
        .unwrap();
        let giver = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let quest: project_rl::content::ContentId = "core:delivery_test".parse().unwrap();

        for (command, action) in [
            (
                GameCommand::AcceptQuest {
                    giver,
                    quest: quest.clone(),
                },
                "accept_quest",
            ),
            (
                GameCommand::CompleteQuest {
                    giver,
                    quest: quest.clone(),
                },
                "complete_quest",
            ),
        ] {
            let recorded = RecordedCommand::record(&command);
            assert_eq!(recorded.command(&game), Ok(command));
            let json = serde_json::to_value(&recorded).unwrap();
            assert_eq!(json["action"], serde_json::json!(action));
            assert_eq!(json["giver"], serde_json::json!(giver.get()));
            assert_eq!(json["quest"], serde_json::json!("core:delivery_test"));
        }
    }

    #[test]
    fn recorded_engineering_technique_preserves_explicit_component_assignment() {
        let mut game = GameState::new(
            Map::from_ascii("#######\n#.....#\n#.....#\n#######").unwrap(),
            GridPos::new(1, 1),
            13,
        )
        .unwrap();
        let target = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let command = GameCommand::UseEngineeringTechnique {
            technique: "core:ing_03".parse().unwrap(),
            directive: EngineeringDirective::Component {
                target,
                component: "core:test_sensor".parse().unwrap(),
            },
        };

        let recorded = RecordedCommand::record(&command);
        assert_eq!(recorded.command(&game), Ok(command));
        let json = serde_json::to_value(&recorded).unwrap();
        assert_eq!(json["action"], serde_json::json!("engineering_technique"));
        assert_eq!(
            json["directive"]["directive"],
            serde_json::json!("component")
        );
        assert_eq!(json["directive"]["target"], target.get());
        assert_eq!(
            json["directive"]["component"],
            serde_json::json!("core:test_sensor")
        );
    }

    #[test]
    fn recorded_intrusion_technique_preserves_subnet_targets_and_command() {
        let game = GameState::new(
            Map::from_ascii("#######\n#.....#\n#.....#\n#######").unwrap(),
            GridPos::new(1, 1),
            17,
        )
        .unwrap();
        let command = GameCommand::UseIntrusionTechnique {
            technique: "core:int_09".parse().unwrap(),
            directive: IntrusionDirective::Subnet {
                positions: vec![GridPos::new(2, 1), GridPos::new(3, 1)],
                command: DeviceCommand::Disable,
            },
        };

        let recorded = RecordedCommand::record(&command);
        assert_eq!(recorded.command(&game), Ok(command));
        let json = serde_json::to_value(&recorded).unwrap();
        assert_eq!(json["action"], serde_json::json!("intrusion_technique"));
        assert_eq!(json["directive"]["directive"], serde_json::json!("subnet"));
        assert_eq!(json["directive"]["positions"][1], serde_json::json!([3, 1]));
        assert_eq!(json["directive"]["command"], serde_json::json!("disable"));

        let invalid: RecordedCommand = serde_json::from_value(serde_json::json!({
            "action": "intrusion_technique",
            "technique": "core:int_08",
            "directive": { "directive": "trace", "trace": 0 }
        }))
        .unwrap();
        assert_eq!(
            invalid.command(&game),
            Err("Trace numérique invalide : 0".to_owned())
        );
    }

    #[test]
    fn recorded_electronic_warfare_preserves_every_directive_shape() {
        let mut game = GameState::new(
            Map::from_ascii("#######\n#.....#\n#.....#\n#######").unwrap(),
            GridPos::new(1, 1),
            19,
        )
        .unwrap();
        let first = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 10).unwrap())
            .unwrap();
        let second = game
            .spawn_actor(Actor::new(GridPos::new(3, 1), 10).unwrap())
            .unwrap();
        let program = HostileProgramId::from_raw(1).unwrap();
        let directives = [
            ElectronicDirective::Pulse {
                direction: Some(Direction::East),
            },
            ElectronicDirective::Target { target: first },
            ElectronicDirective::Jam {
                channel: ElectronicChannel::ThermalSensor,
            },
            ElectronicDirective::Purge {
                target: first,
                program,
            },
            ElectronicDirective::Cascade {
                targets: vec![first, second],
            },
            ElectronicDirective::DeployBeacon {
                position: GridPos::new(4, 2),
            },
            ElectronicDirective::ActivateBeacon { beacon: second },
        ];

        for directive in directives {
            let command = GameCommand::UseElectronicWarfareTechnique {
                technique: "core:gel_test".parse().unwrap(),
                directive,
            };
            let recorded = RecordedCommand::record(&command);
            assert_eq!(recorded.command(&game), Ok(command));
            assert_eq!(
                serde_json::to_value(&recorded).unwrap()["action"],
                serde_json::json!("electronic_warfare_technique")
            );
        }

        let invalid: RecordedCommand = serde_json::from_value(serde_json::json!({
            "action": "electronic_warfare_technique",
            "technique": "core:gel_04",
            "directive": { "directive": "purge", "target": first.get(), "program": 0 }
        }))
        .unwrap();
        assert_eq!(
            invalid.command(&game),
            Err("Programme hostile invalide : 0".to_owned())
        );
    }
}
