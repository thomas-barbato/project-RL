//! Single-use, prototype replay suspension. No rewind or checkpoint loading UI.
use project_rl::{
    entity::{EntityId, ItemInstanceId},
    game::{GameCommand, GameState},
    world::{Direction, GridPos},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub const MAX_COMMANDS: usize = 50_000;
const MAX_BYTES: u64 = 16 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecordedCommand {
    Move {
        direction: u8,
    },
    Wait,
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
    UseItem {
        item: u64,
    },
    PickUp,
    Drop {
        item: u64,
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
            GameCommand::UseItem { item } => Self::UseItem { item: item.get() },
            GameCommand::PickUp => Self::PickUp,
            GameCommand::DropItem { item } => Self::Drop { item: item.get() },
            GameCommand::UseAbility { slot, target } => Self::Ability {
                slot: *slot,
                x: target.x,
                y: target.y,
            },
            GameCommand::LearnTechnique { technique } => Self::Learn {
                technique: technique.to_string(),
            },
            GameCommand::UseTechnique { technique, targets } => Self::Technique {
                technique: technique.to_string(),
                targets: targets.iter().map(|id| id.get()).collect(),
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
        Ok(match self {
            Self::Move { direction } => GameCommand::Move(match direction {
                0 => Direction::North,
                1 => Direction::East,
                2 => Direction::South,
                3 => Direction::West,
                _ => return Err("Direction invalide.".to_owned()),
            }),
            Self::Wait => GameCommand::Wait,
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
            Self::UseItem { item: value } => GameCommand::UseItem {
                item: item(*value)?,
            },
            Self::PickUp => GameCommand::PickUp,
            Self::Drop { item: value } => GameCommand::DropItem {
                item: item(*value)?,
            },
            Self::Ability { slot, x, y } => GameCommand::UseAbility {
                slot: *slot,
                target: GridPos::new(*x, *y),
            },
            Self::Learn { technique } => GameCommand::LearnTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
            },
            Self::Technique { technique, targets } => GameCommand::UseTechnique {
                technique: technique
                    .parse()
                    .map_err(|error| format!("Technique invalide : {error}"))?,
                targets: targets
                    .iter()
                    .map(|id| entity(*id))
                    .collect::<Result<_, _>>()?,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loot_rules: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub world_rules: Option<u64>,
    pub seed: u64,
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
        if !matches!(self.version, 1..=10) {
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
        // Build identifies the producer, not replay compatibility: a UI-only
        // patch changes it too. The caller must still validate the active rules,
        // replay every command and compare the complete final state before use.
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
