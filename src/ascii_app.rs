use std::collections::BTreeMap;

use macroquad::prelude::*;
use project_rl::ai::AiProfile;
use project_rl::combat::{AttackProfile, DamageType};
use project_rl::content::ContentLoader;
use project_rl::effects::{AbilityProfile, ApplyStatusEffect, EffectPrimitive};
use project_rl::entity::{Actor, EntityId, ItemInstanceId};
use project_rl::game::{
    CommandOutcome, CommandRejection, GameCommand, GameEvent, GameRng, GameRules, GameState,
    RunStatus,
};
use project_rl::progression::DefeatReward;
use project_rl::status::StatusId;
use project_rl::weapon::WeaponId;
use project_rl::world::generation::{RoomsGenerator, RoomsGeneratorConfig};
use project_rl::world::{Direction, DistanceMetric, GridPos, Terrain};

const INITIAL_SEED: u64 = 20_260_909;
const LOG_CAPACITY: usize = 6;

pub struct AsciiApp {
    game: GameState,
    rules: GameRules,
    seed: u64,
    actor_glyphs: BTreeMap<EntityId, char>,
    selected_target: Option<EntityId>,
    effect_cells: Vec<GridPos>,
    effect_visible_until: f64,
    log: Vec<String>,
    active_weapon_slot: u8,
    inventory_open: bool,
    inventory_selection: usize,
    inventory_message: String,
}

impl AsciiApp {
    pub fn new() -> Result<Self, String> {
        Self::from_seed(INITIAL_SEED, ascii_game_rules()?)
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::I) || (self.inventory_open && is_key_pressed(KeyCode::Escape)) {
            self.inventory_open = !self.inventory_open;
            self.clamp_inventory_selection();
            if self.inventory_open {
                self.inventory_message =
                    "SELECT AN ITEM, THEN CHOOSE ITS CHANNEL WITH 1/2/3".to_owned();
            }
            return;
        }

        if self.inventory_open {
            self.update_inventory();
            return;
        }

        if is_key_pressed(KeyCode::R) {
            let next_seed = self.seed.wrapping_add(1);
            match Self::from_seed(next_seed, self.rules.clone()) {
                Ok(next_run) => *self = next_run,
                Err(error) => self.push_log(format!("RESTART ERROR: {error}")),
            }
            return;
        }

        if self.game.status() != RunStatus::Active {
            return;
        }

        if let Some(slot) = pressed_weapon_slot() {
            self.select_weapon_slot(slot);
            return;
        }

        if is_key_pressed(KeyCode::Tab) {
            self.cycle_target();
            return;
        }

        let command = if is_key_pressed(KeyCode::H) {
            self.corrosion_command()
        } else if is_key_pressed(KeyCode::G) {
            self.ability_command()
        } else if is_key_pressed(KeyCode::F) {
            self.weapon_command()
        } else {
            read_movement_command(&self.game, self.active_weapon_slot)
        };
        if let Some(command) = command {
            let outcome = self.game.process_player_command(command);
            if let CommandOutcome::Rejected(reason) = outcome {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            self.capture_events();
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(5, 8, 12, 255));

        let map = self.game.map();
        let available_height = (screen_height() - 150.0).max(100.0);
        let height_limited = available_height / map.height() as f32;
        let width_limited = (screen_width() - 40.0).max(100.0) / (map.width() as f32 * 0.62);
        let cell_height = height_limited.min(width_limited).clamp(10.0, 24.0);
        let cell_width = cell_height * 0.62;
        let map_width = cell_width * map.width() as f32;
        let map_height = cell_height * map.height() as f32;
        let origin_x = ((screen_width() - map_width) * 0.5).round();
        let origin_y = 72.0 + ((available_height - map_height) * 0.5).max(0.0);

        for y in 0..map.height() {
            for x in 0..map.width() {
                let position = GridPos::new(x as i32, y as i32);
                let Some((glyph, color)) = self.glyph_at(position) else {
                    continue;
                };
                let glyph_x = origin_x + x as f32 * cell_width;
                let glyph_y = origin_y + (y + 1) as f32 * cell_height;
                if self.is_selected_target_at(position) {
                    draw_target_cursor(glyph_x, glyph_y, cell_width, cell_height);
                }
                draw_text(glyph.to_string(), glyph_x, glyph_y, cell_height, color);
            }
        }

        self.draw_header();
        self.draw_footer();
        self.draw_end_message();
        if self.inventory_open {
            self.draw_inventory();
        }
    }

    fn from_seed(seed: u64, rules: GameRules) -> Result<Self, String> {
        let config = RoomsGeneratorConfig {
            width: 48,
            height: 28,
            room_count: 9,
            ..RoomsGeneratorConfig::default()
        };
        let generator = RoomsGenerator::new(config).map_err(|error| error.to_string())?;
        let mut rng = GameRng::from_seed(seed);
        let generated = generator
            .generate(&mut rng)
            .map_err(|error| error.to_string())?;
        let room_centers: Vec<GridPos> =
            generated.rooms().iter().map(|room| room.center()).collect();
        let exit = generated.exit();
        let mut game = GameState::from_generated(generated, rng.state(), rules.clone())
            .map_err(|error| error.to_string())?;
        let mut actor_glyphs = BTreeMap::new();

        for (index, position) in room_centers.into_iter().skip(1).take(5).enumerate() {
            if position == exit {
                continue;
            }
            let integrity = 5 + index as u16 * 2;
            let (attack, ai, glyph, reward) = match index % 3 {
                0 => (
                    AttackProfile::melee(DamageType::Kinetic, 3),
                    AiProfile::hunter(10, 0),
                    'd',
                    DefeatReward::persistent(5, 1),
                ),
                1 => (
                    AttackProfile::new(
                        6,
                        DistanceMetric::Euclidean,
                        true,
                        DamageType::Electrical,
                        2,
                        0,
                    ),
                    AiProfile::sentry(8, 0),
                    't',
                    DefeatReward::persistent(7, 2),
                ),
                _ => (
                    AttackProfile::new(
                        5,
                        DistanceMetric::Euclidean,
                        true,
                        DamageType::Piercing,
                        2,
                        1,
                    ),
                    AiProfile::skirmisher(9, 0, 3),
                    'r',
                    DefeatReward::persistent(8, 2),
                ),
            };
            let enemy = Actor::new(position, integrity)
                .map_err(|error| error.to_string())?
                .with_attack(attack)
                .with_ai(ai)
                .with_defeat_reward(reward);
            let id = game.spawn_actor(enemy).map_err(|error| error.to_string())?;
            actor_glyphs.insert(id, glyph);
        }
        game.drain_events();

        Ok(Self {
            game,
            rules,
            seed,
            actor_glyphs,
            selected_target: None,
            effect_cells: Vec::new(),
            effect_visible_until: 0.0,
            log: vec![format!("RUN INITIALIZED — SEED {seed}")],
            active_weapon_slot: 0,
            inventory_open: false,
            inventory_selection: 0,
            inventory_message: String::new(),
        })
    }

    fn glyph_at(&self, position: GridPos) -> Option<(char, Color)> {
        let visibility = self.game.player_visibility();
        if !visibility.is_explored(position) {
            return None;
        }

        if get_time() <= self.effect_visible_until
            && self.effect_cells.contains(&position)
            && visibility.is_visible(position)
        {
            return Some(('*', Color::from_rgba(255, 173, 66, 255)));
        }

        if visibility.is_visible(position) {
            if self.game.player_position() == Some(position) {
                return Some(('@', Color::from_rgba(99, 242, 210, 255)));
            }
            if let Some(entity) = self.game.actors().entity_at(position)
                && let Some(glyph) = self.actor_glyphs.get(&entity)
            {
                let has_status = self
                    .game
                    .actors()
                    .get(entity)
                    .is_some_and(|actor| actor.statuses().next().is_some());
                let color = if has_status {
                    Color::from_rgba(143, 221, 107, 255)
                } else {
                    Color::from_rgba(244, 105, 90, 255)
                };
                return Some((*glyph, color));
            }
            if self.game.exit() == Some(position) {
                return Some(('>', Color::from_rgba(255, 211, 92, 255)));
            }
        }

        let tile = self.game.map().tile(position)?;
        let glyph = match tile.terrain {
            Terrain::Floor => '.',
            Terrain::Wall => '#',
        };
        let color = if visibility.is_visible(position) {
            match tile.terrain {
                Terrain::Floor => Color::from_rgba(92, 127, 137, 255),
                Terrain::Wall => Color::from_rgba(151, 180, 181, 255),
            }
        } else {
            match tile.terrain {
                Terrain::Floor => Color::from_rgba(35, 51, 59, 255),
                Terrain::Wall => Color::from_rgba(58, 73, 78, 255),
            }
        };
        Some((glyph, color))
    }

    fn capture_events(&mut self) {
        for event in self.game.drain_events() {
            match event {
                GameEvent::EntityMoved { entity, to, .. } if entity == self.game.player_id() => {
                    self.push_log(format!("MOVE → {},{}", to.x, to.y));
                }
                GameEvent::DamageApplied {
                    target,
                    amount,
                    damage_type,
                    ..
                } if target == self.game.player_id() => {
                    self.push_log(format!(
                        "INTEGRITY -{amount}  {} DAMAGE",
                        format!("{damage_type:?}").to_uppercase()
                    ));
                }
                GameEvent::DamageApplied { amount, .. } => {
                    self.push_log(format!("TARGET DAMAGED -{amount}"));
                }
                GameEvent::EntityDied { entity } if entity != self.game.player_id() => {
                    self.actor_glyphs.remove(&entity);
                    if self.selected_target == Some(entity) {
                        self.selected_target = None;
                    }
                    self.push_log("TARGET DESTROYED".to_owned());
                }
                GameEvent::ExperienceAwarded { amount, total, .. } => {
                    self.push_log(format!("EXPERIENCE +{amount}  TOTAL {total}"));
                }
                GameEvent::LevelGained {
                    level,
                    skill_points_awarded,
                } => {
                    self.push_log(format!(
                        "CORE LEVEL {level}  SKILL POINTS +{skill_points_awarded}"
                    ));
                }
                GameEvent::StatusApplied {
                    target,
                    status,
                    stacks,
                    remaining_turns,
                    ..
                } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    let duration = remaining_turns
                        .map(|turns| format!("{turns} TURNS"))
                        .unwrap_or_else(|| "PERMANENT".to_owned());
                    self.push_log(format!(
                        "{} ON {subject} x{stacks} ({duration})",
                        display_content_name(&status),
                    ));
                }
                GameEvent::StatusTriggered { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    self.push_log(format!(
                        "{} TRIGGERS ON {subject}",
                        display_content_name(&status)
                    ));
                }
                GameEvent::StatusRemoved { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "CORE"
                    } else {
                        "TARGET"
                    };
                    self.push_log(format!(
                        "{} EXPIRED ON {subject}",
                        display_content_name(&status)
                    ));
                }
                GameEvent::WeaponEquipped {
                    slot,
                    weapon,
                    displaced,
                    ..
                } => {
                    let displaced = displaced
                        .map(|_| " / PREVIOUS WEAPON STORED".to_owned())
                        .unwrap_or_default();
                    self.push_log(format!(
                        "CHANNEL {} <- {}{}",
                        slot + 1,
                        display_content_name(&weapon),
                        displaced
                    ));
                }
                GameEvent::ExitReached { .. } => {
                    self.push_log("EXIT REACHED — SIMULATION LAYER CLEARED".to_owned());
                }
                GameEvent::PropagationResolved { cells, .. } => {
                    self.effect_cells = cells.into_iter().map(|cell| cell.position).collect();
                    self.effect_visible_until = get_time() + 0.35;
                    self.push_log("RADIAL PULSE RELEASED".to_owned());
                }
                _ => {}
            }
        }
    }

    fn is_selected_target_at(&self, position: GridPos) -> bool {
        self.selected_target.is_some_and(|target| {
            self.game
                .actors()
                .get(target)
                .is_some_and(|actor| actor.position() == position)
        })
    }

    fn push_log(&mut self, message: String) {
        self.log.push(message);
        if self.log.len() > LOG_CAPACITY {
            self.log.remove(0);
        }
    }

    fn cycle_target(&mut self) {
        let targets = self.visible_targets();
        if targets.is_empty() {
            self.selected_target = None;
            self.push_log("NO VISIBLE TARGET".to_owned());
            return;
        }

        let next_index = self
            .selected_target
            .and_then(|selected| targets.iter().position(|target| *target == selected))
            .map_or(0, |index| (index + 1) % targets.len());
        self.selected_target = targets.get(next_index).copied();
        if self.selected_target.is_some() {
            self.push_log("TARGET LOCKED".to_owned());
        }
    }

    fn select_weapon_slot(&mut self, slot: u8) {
        let Some(weapon) = self.game.equipped_player_weapon(slot) else {
            self.push_log(format!("CHANNEL {} IS EMPTY", slot + 1));
            return;
        };
        let name = display_content_name(weapon.id());
        self.active_weapon_slot = slot;
        self.push_log(format!("ACTIVE [{}] {name}", slot + 1));
    }

    fn weapon_command(&mut self) -> Option<GameCommand> {
        if self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .is_none()
        {
            self.push_log(format!(
                "ACTIVE CHANNEL {} IS EMPTY",
                self.active_weapon_slot + 1
            ));
            return None;
        }
        let target = self.ensure_visible_target()?;
        Some(GameCommand::Attack {
            slot: self.active_weapon_slot,
            target,
        })
    }

    fn ability_command(&mut self) -> Option<GameCommand> {
        let target = self.ensure_visible_target()?;
        let Some(position) = self.game.actors().get(target).map(|actor| actor.position()) else {
            self.selected_target = None;
            return None;
        };
        Some(GameCommand::UseAbility {
            slot: 0,
            target: position,
        })
    }

    fn corrosion_command(&mut self) -> Option<GameCommand> {
        let target = self.ensure_visible_target()?;
        let Some(position) = self.game.actors().get(target).map(|actor| actor.position()) else {
            self.selected_target = None;
            return None;
        };
        Some(GameCommand::UseAbility {
            slot: 1,
            target: position,
        })
    }

    fn ensure_visible_target(&mut self) -> Option<EntityId> {
        let targets = self.visible_targets();
        let selected_is_visible = self
            .selected_target
            .is_some_and(|selected| targets.contains(&selected));
        if !selected_is_visible {
            self.selected_target = targets.first().copied();
        }

        let Some(target) = self.selected_target else {
            self.push_log("NO VISIBLE TARGET".to_owned());
            return None;
        };
        Some(target)
    }

    fn visible_targets(&self) -> Vec<EntityId> {
        let Some(player_position) = self.game.player_position() else {
            return Vec::new();
        };
        let mut targets: Vec<(u32, EntityId)> = self
            .game
            .actors()
            .iter()
            .filter_map(|(entity, actor)| {
                (entity != self.game.player_id()
                    && self.game.player_visibility().is_visible(actor.position()))
                .then_some((grid_distance(player_position, actor.position()), entity))
            })
            .collect();
        targets.sort_unstable();
        targets.into_iter().map(|(_, entity)| entity).collect()
    }

    fn draw_header(&self) {
        let player_integrity = self
            .game
            .actors()
            .get(self.game.player_id())
            .map(|actor| format!("{}/{}", actor.integrity(), actor.maximum_integrity()))
            .unwrap_or_else(|| "0/--".to_owned());
        let target = self.selected_target.map_or_else(
            || "--".to_owned(),
            |entity| {
                let label = match self.actor_glyphs.get(&entity) {
                    Some('d') => "HUNTER",
                    Some('t') => "SENTRY",
                    Some('r') => "SKIRMISHER",
                    _ => "HOSTILE",
                };
                let status = self
                    .game
                    .actors()
                    .get(entity)
                    .and_then(|actor| actor.statuses().next())
                    .map(|status| {
                        let duration = status
                            .remaining_turns
                            .map(|turns| format!("{turns}T"))
                            .unwrap_or_else(|| "PERMANENT".to_owned());
                        format!(
                            " / {} x{} {duration}",
                            display_content_name(&status.definition),
                            status.stacks
                        )
                    })
                    .unwrap_or_default();
                format!("{label}{status}")
            },
        );
        let progression = self.game.player_progression();
        let header = format!(
            "PROJECT RL  |  SEED {}  |  TURN {}  |  CORE L{} XP {}  |  INTEGRITY {}  |  TARGET {}",
            self.seed,
            self.game.turn(),
            progression.level(),
            progression.experience(),
            player_integrity,
            target,
        );
        draw_text(
            &header,
            20.0,
            34.0,
            22.0,
            Color::from_rgba(99, 242, 210, 255),
        );
        let active_weapon = self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .map(|weapon| display_content_name(weapon.id()))
            .unwrap_or_else(|| "EMPTY".to_owned());
        let view_label = format!(
            "ASCII ENGINE VIEW — d HUNTER / t SENTRY / r SKIRMISHER   ACTIVE [{}] {}",
            self.active_weapon_slot + 1,
            active_weapon
        );
        draw_text(
            &view_label,
            20.0,
            58.0,
            17.0,
            Color::from_rgba(94, 126, 137, 255),
        );
    }

    fn draw_footer(&self) {
        let controls = "MOVE: ARROWS WASD ZQSD   WEAPON: 1/2/3   ATTACK: F   INVENTORY: I   TARGET: TAB   PULSE: G   CORRODE: H   WAIT: SPACE/.   NEW: R";
        draw_text(
            controls,
            20.0,
            screen_height() - 72.0,
            18.0,
            Color::from_rgba(156, 178, 184, 255),
        );

        for (index, message) in self.log.iter().rev().take(2).rev().enumerate() {
            draw_text(
                message,
                20.0,
                screen_height() - 42.0 + index as f32 * 20.0,
                16.0,
                Color::from_rgba(107, 137, 145, 255),
            );
        }
    }

    fn draw_end_message(&self) {
        let message = if self.game.status() == RunStatus::PlayerDestroyed {
            Some("CORE DESTROYED — PRESS R TO RESTART")
        } else if self.game.status() == RunStatus::Escaped {
            Some("EXIT REACHED — PRESS R FOR A NEW SEED")
        } else {
            None
        };

        if let Some(message) = message {
            let metrics = measure_text(message, None, 28, 1.0);
            let x = (screen_width() - metrics.width) * 0.5;
            let y = screen_height() * 0.5;
            draw_rectangle(
                x - 18.0,
                y - 34.0,
                metrics.width + 36.0,
                52.0,
                Color::from_rgba(4, 8, 12, 235),
            );
            draw_text(message, x, y, 28.0, Color::from_rgba(255, 211, 92, 255));
        }
    }

    fn clamp_inventory_selection(&mut self) {
        let count = self.game.player_inventory().len();
        self.inventory_selection = self.inventory_selection.min(count.saturating_sub(1));
    }

    fn update_inventory(&mut self) {
        let count = self.game.player_inventory().len();
        if count == 0 {
            return;
        }
        if any_key_pressed(&[KeyCode::Up, KeyCode::W, KeyCode::Z]) {
            self.inventory_selection = self.inventory_selection.saturating_sub(1);
            return;
        }
        if any_key_pressed(&[KeyCode::Down, KeyCode::S]) {
            self.inventory_selection = (self.inventory_selection + 1).min(count - 1);
            return;
        }

        let Some(slot) = pressed_weapon_slot() else {
            return;
        };
        let Some(item) = self
            .game
            .player_inventory()
            .iter()
            .nth(self.inventory_selection)
            .map(|entry| entry.instance())
        else {
            return;
        };
        if self
            .game
            .rules()
            .player_weapon_slots
            .get(usize::from(slot))
            .and_then(|slot_id| self.game.player_equipment().equipped(slot_id))
            == Some(item)
        {
            self.active_weapon_slot = slot;
            self.inventory_message = format!("CHANNEL {} IS NOW ACTIVE", slot + 1);
            return;
        }
        let item_name = self
            .game
            .player_inventory()
            .get(item)
            .map(|entry| display_content_name(entry.item()))
            .unwrap_or_else(|| "UNKNOWN ITEM".to_owned());
        let outcome = self
            .game
            .process_player_command(GameCommand::EquipWeapon { slot, item });
        match outcome {
            CommandOutcome::Applied => {
                self.active_weapon_slot = slot;
                self.inventory_message =
                    format!("{item_name} EQUIPPED + ACTIVE ON CHANNEL {}", slot + 1);
            }
            CommandOutcome::Rejected(_reason) => {
                self.inventory_message = "EQUIPMENT CHANGE IMPOSSIBLE".to_owned();
                self.push_log(self.inventory_message.clone());
            }
        }
        self.capture_events();
    }

    fn draw_inventory(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (screen_width() - margin * 2.0).max(620.0);
        let height = (screen_height() - top * 2.0).max(400.0);
        let left_width = width * 0.43;
        let panel = Color::from_rgba(5, 12, 17, 248);
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);

        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::from_rgba(0, 2, 4, 210),
        );
        draw_rectangle(margin, top, width, height, panel);
        draw_rectangle_lines(margin, top, width, height, 2.0, cyan);
        draw_line(
            margin + left_width,
            top + 58.0,
            margin + left_width,
            top + height - 52.0,
            1.0,
            muted,
        );
        draw_line(margin, top + 58.0, margin + width, top + 58.0, 1.0, muted);
        draw_line(
            margin,
            top + height - 52.0,
            margin + width,
            top + height - 52.0,
            1.0,
            muted,
        );

        draw_text(
            "INVENTORY // LOADOUT",
            margin + 20.0,
            top + 37.0,
            25.0,
            cyan,
        );
        let inventory = self.game.player_inventory();
        let capacity = format!("SLOTS {:02}/{:02}", inventory.len(), inventory.capacity());
        let capacity_width = measure_text(&capacity, None, 18, 1.0).width;
        draw_text(
            &capacity,
            margin + width - capacity_width - 20.0,
            top + 35.0,
            18.0,
            muted,
        );

        draw_text("WEAPONS", margin + 20.0, top + 88.0, 17.0, muted);
        let row_height = 48.0;
        let visible_rows = ((height - 220.0) / row_height).floor().max(1.0) as usize;
        let first_visible = self
            .inventory_selection
            .saturating_sub(visible_rows.saturating_sub(1));
        for (visible_index, (index, entry)) in inventory
            .iter()
            .enumerate()
            .skip(first_visible)
            .take(visible_rows)
            .enumerate()
        {
            let row_y = top + 135.0 + visible_index as f32 * row_height;
            if index == self.inventory_selection {
                draw_rectangle(
                    margin + 10.0,
                    row_y - 25.0,
                    left_width - 20.0,
                    39.0,
                    Color::from_rgba(18, 58, 63, 230),
                );
                draw_rectangle(margin + 10.0, row_y - 25.0, 3.0, 39.0, amber);
            }
            let equipped = self.equipped_channel_label(entry.instance());
            let prefix = if index == self.inventory_selection {
                ">"
            } else {
                " "
            };
            draw_text(
                format!("{prefix} {}", display_content_name(entry.item())),
                margin + 19.0,
                row_y,
                20.0,
                if index == self.inventory_selection {
                    amber
                } else {
                    text
                },
            );
            let quantity = (entry.quantity() > 1).then(|| format!("x{}  ", entry.quantity()));
            draw_text(
                format!("{}{}", quantity.unwrap_or_default(), equipped),
                margin + 40.0,
                row_y + 17.0,
                14.0,
                muted,
            );
        }
        if first_visible > 0 {
            draw_text(
                "^ MORE",
                margin + left_width - 76.0,
                top + 88.0,
                14.0,
                amber,
            );
        }
        if first_visible + visible_rows < inventory.len() {
            draw_text(
                "v MORE",
                margin + left_width - 76.0,
                top + height - 64.0,
                14.0,
                amber,
            );
        }

        let detail_x = margin + left_width + 24.0;
        let selected = inventory.iter().nth(self.inventory_selection);
        if let Some(entry) = selected {
            let weapon = self.game.rules().weapons.get(entry.item());
            draw_text(
                display_content_name(entry.item()),
                detail_x,
                top + 94.0,
                27.0,
                amber,
            );
            draw_text(
                self.equipped_channel_label(entry.instance()),
                detail_x,
                top + 120.0,
                15.0,
                cyan,
            );
            draw_text("COMBAT PROFILE", detail_x, top + 151.0, 16.0, muted);
            if let Some(weapon) = weapon {
                let attack = weapon.attack();
                let damage = attack.damage();
                draw_stat_line(
                    detail_x,
                    top + 188.0,
                    "DAMAGE",
                    &damage.amount.to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 218.0,
                    "TYPE",
                    &format!("{:?}", damage.damage_type).to_uppercase(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 248.0,
                    "PENETRATION",
                    &damage.penetration.to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 278.0,
                    "RANGE",
                    &attack.range().to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 308.0,
                    "LINE OF SIGHT",
                    if attack.requires_line_of_sight() {
                        "REQUIRED"
                    } else {
                        "NO"
                    },
                    text,
                    muted,
                );
            }

            draw_text(
                "COMBAT CHANNELS",
                detail_x,
                top + height - 142.0,
                15.0,
                muted,
            );
            for slot in 0..self.game.rules().player_weapon_slots.len().min(3) {
                let y = top + height - 116.0 + slot as f32 * 21.0;
                let equipped = self
                    .game
                    .equipped_player_weapon(slot as u8)
                    .map(|weapon| display_content_name(weapon.id()))
                    .unwrap_or_else(|| "-- EMPTY --".to_owned());
                draw_text(
                    format!(
                        "{} [{}] {equipped}",
                        if slot as u8 == self.active_weapon_slot {
                            ">"
                        } else {
                            " "
                        },
                        slot + 1
                    ),
                    detail_x,
                    y,
                    16.0,
                    if slot as u8 == self.active_weapon_slot {
                        amber
                    } else {
                        text
                    },
                );
            }
        }

        draw_text(
            &self.inventory_message,
            margin + 20.0,
            top + height - 34.0,
            14.0,
            amber,
        );
        draw_text(
            "UP/DOWN SELECT   1/2/3 EQUIP + ACTIVATE (1 TURN)   I/ESC CLOSE",
            margin + 20.0,
            top + height - 14.0,
            16.0,
            cyan,
        );
    }

    fn equipped_channel_label(&self, item: ItemInstanceId) -> String {
        let channels = self
            .game
            .rules()
            .player_weapon_slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                (self.game.player_equipment().equipped(slot) == Some(item))
                    .then_some((index + 1).to_string())
            })
            .collect::<Vec<_>>()
            .join("/");
        if channels.is_empty() {
            "STORED".to_owned()
        } else if channels == (usize::from(self.active_weapon_slot) + 1).to_string() {
            format!("ACTIVE CHANNEL {channels}")
        } else {
            format!("CHANNEL {channels}")
        }
    }
}

fn ascii_game_rules() -> Result<GameRules, String> {
    let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let loaded = ContentLoader::load(
        &[project_root.join("content"), project_root.join("mods")],
        &semver::Version::new(0, 1, 0),
    )
    .map_err(|error| error.to_string())?;
    let corrosion_id: StatusId = "core:corroded"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let melee_id: WeaponId = "core:integrity_blade"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let ranged_id: WeaponId = "core:needle_launcher"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let player_starting_equipment = vec![Some(melee_id.clone()), Some(ranged_id.clone())];
    let mut player_starting_weapons = vec![melee_id, ranged_id];
    player_starting_weapons.extend(
        loaded
            .weapons()
            .iter()
            .filter(|(id, _)| id.namespace().as_str() != "core")
            .map(|(id, _)| id.clone()),
    );
    let player_base_attacks = player_starting_weapons
        .iter()
        .map(|id| {
            loaded
                .weapons()
                .get(id)
                .ok_or_else(|| format!("missing starting weapon '{id}'"))
                .map(|weapon| weapon.attack())
        })
        .collect::<Result<Vec<_>, _>>()?;
    let player_weapon_slots = [
        "core:close_combat_channel",
        "core:ranged_combat_channel",
        "core:auxiliary_combat_channel",
    ]
    .into_iter()
    .map(str::parse)
    .collect::<Result<Vec<_>, project_rl::content::ContentIdError>>()
    .map_err(|error| error.to_string())?;
    let (statuses, weapons) = loaded.into_registries();
    let mut rules = GameRules {
        player_base_attacks,
        player_weapon_slots,
        player_starting_weapons,
        player_starting_equipment,
        statuses,
        weapons,
        ..GameRules::default()
    };
    rules.player_base_abilities.push(AbilityProfile::new(
        6,
        DistanceMetric::Euclidean,
        true,
        true,
        vec![EffectPrimitive::ApplyStatus(
            ApplyStatusEffect::new(corrosion_id, 1).map_err(|error| error.to_string())?,
        )],
    ));
    Ok(rules)
}

fn display_content_name(id: &project_rl::content::ContentId) -> String {
    id.name().replace(['_', '-'], " ").to_uppercase()
}

fn draw_stat_line(
    x: f32,
    y: f32,
    label: &str,
    value: &str,
    value_color: Color,
    label_color: Color,
) {
    draw_text(label, x, y, 16.0, label_color);
    draw_text(value, x + 148.0, y, 18.0, value_color);
}

fn command_rejection_message(reason: CommandRejection) -> &'static str {
    match reason {
        CommandRejection::RunEnded => "THE RUN HAS ENDED",
        CommandRejection::NotPlayersTurn => "WAIT FOR YOUR TURN",
        CommandRejection::MissingPlayer => "CORE CONNECTION LOST",
        CommandRejection::BlockedByTerrain(_) => "PATH BLOCKED",
        CommandRejection::Occupied(_) => "SPACE OCCUPIED",
        CommandRejection::UnknownTarget(_) => "TARGET LOST",
        CommandRejection::MissingAttackSlot(_) => "NO WEAPON EQUIPPED IN THIS CHANNEL",
        CommandRejection::TargetOutOfRange(_) => "TARGET OUT OF RANGE",
        CommandRejection::NoLineOfSight(_) => "NO LINE OF SIGHT",
        CommandRejection::MissingEquipmentSlot(_) => "UNKNOWN EQUIPMENT CHANNEL",
        CommandRejection::UnknownInventoryItem(_) => "ITEM IS NO LONGER AVAILABLE",
        CommandRejection::ItemIsNotWeapon(_) => "THIS ITEM IS NOT A WEAPON",
        CommandRejection::ItemAlreadyEquippedInSlot { .. } => "WEAPON ALREADY EQUIPPED",
        CommandRejection::MissingAbilitySlot(_) => "ABILITY UNAVAILABLE",
        CommandRejection::AbilityTargetOutsideMap(_) => "INVALID TARGET AREA",
        CommandRejection::AbilityTargetBlocked(_) => "TARGET AREA BLOCKED",
        CommandRejection::AbilityTargetOutOfRange(_) => "ABILITY TARGET OUT OF RANGE",
        CommandRejection::AbilityNoLineOfSight(_) => "ABILITY HAS NO LINE OF SIGHT",
        CommandRejection::AbilityTargetHasNoActor(_) => "NO ACTOR AT TARGET",
        CommandRejection::AbilityUnknownStatusDefinition => "ABILITY DATA IS UNAVAILABLE",
    }
}

fn read_movement_command(game: &GameState, attack_slot: u8) -> Option<GameCommand> {
    let direction = if any_key_pressed(&[KeyCode::Up, KeyCode::W, KeyCode::Z]) {
        Some(Direction::North)
    } else if any_key_pressed(&[KeyCode::Right, KeyCode::D]) {
        Some(Direction::East)
    } else if any_key_pressed(&[KeyCode::Down, KeyCode::S]) {
        Some(Direction::South)
    } else if any_key_pressed(&[KeyCode::Left, KeyCode::A, KeyCode::Q]) {
        Some(Direction::West)
    } else {
        None
    };

    if let Some(direction) = direction {
        let player_position = game.player_position()?;
        let destination = player_position.step(direction);
        if let Some(target) = game.actors().entity_at(destination)
            && target != game.player_id()
        {
            return Some(GameCommand::Attack {
                slot: attack_slot,
                target,
            });
        }
        return Some(GameCommand::Move(direction));
    }

    (is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Period)).then_some(GameCommand::Wait)
}

fn pressed_weapon_slot() -> Option<u8> {
    if any_key_pressed(&[KeyCode::Key1, KeyCode::Kp1]) {
        Some(0)
    } else if any_key_pressed(&[KeyCode::Key2, KeyCode::Kp2]) {
        Some(1)
    } else if any_key_pressed(&[KeyCode::Key3, KeyCode::Kp3]) {
        Some(2)
    } else {
        None
    }
}

fn any_key_pressed(keys: &[KeyCode]) -> bool {
    keys.iter().any(|key| is_key_pressed(*key))
}

fn grid_distance(first: GridPos, second: GridPos) -> u32 {
    let delta_x = (i64::from(first.x) - i64::from(second.x)).unsigned_abs();
    let delta_y = (i64::from(first.y) - i64::from(second.y)).unsigned_abs();
    delta_x.max(delta_y).min(u64::from(u32::MAX)) as u32
}

fn draw_target_cursor(x: f32, baseline_y: f32, cell_width: f32, cell_height: f32) {
    let color = Color::from_rgba(255, 211, 92, 255);
    let top = baseline_y - cell_height + 2.0;
    let left = x - 2.0;
    let width = cell_width + 3.0;
    let height = cell_height;
    let corner = (cell_width * 0.3).max(2.0);

    draw_line(left, top, left + corner, top, 1.5, color);
    draw_line(left, top, left, top + corner, 1.5, color);
    draw_line(left + width - corner, top, left + width, top, 1.5, color);
    draw_line(left + width, top, left + width, top + corner, 1.5, color);
    draw_line(left, top + height - corner, left, top + height, 1.5, color);
    draw_line(left, top + height, left + corner, top + height, 1.5, color);
    draw_line(
        left + width,
        top + height - corner,
        left + width,
        top + height,
        1.5,
        color,
    );
    draw_line(
        left + width - corner,
        top + height,
        left + width,
        top + height,
        1.5,
        color,
    );
}
