//! A code-native terminal tileset: square pixel scenery, ASCII actors, no
//! raster assets. Only remembered terrain and currently perceived overlays
//! reach the renderer.
use std::collections::{BTreeMap, BTreeSet};

use crate::ui_theme::{
    UiIcon, UiTheme, draw_text as draw_ui_text, draw_text_bold as draw_ui_text_bold,
    draw_text_bold_centered as draw_ui_text_bold_centered, draw_ui_icon,
    measure_text as measure_ui_text, measure_text_bold as measure_ui_text_bold,
};

pub(crate) fn player_location_name(name: &str) -> String {
    let base = name.split_once(" · région ").map_or(name, |(base, _)| base);
    match base {
        "HUMAN HABITAT" => "Territoires habités".to_owned(),
        "SURFACE WILDS" => "Étendues sauvages".to_owned(),
        "MAINTENANCE" => "Galeries de maintenance".to_owned(),
        "PRODUCTION" => "Complexe de production".to_owned(),
        "RESEARCH" => "Secteur de recherche".to_owned(),
        "SECURITY" => "Secteur sécurisé".to_owned(),
        "NETWORK" => "Nœud du réseau".to_owned(),
        "CORRUPTED" => "Profondeurs corrompues".to_owned(),
        _ if base
            .chars()
            .filter(|character| character.is_alphabetic())
            .all(|character| character.is_uppercase()) =>
        {
            base.split_whitespace()
                .map(|word| {
                    let mut characters = word.chars();
                    let first = characters
                        .next()
                        .map(|character| character.to_uppercase().collect::<String>())
                        .unwrap_or_default();
                    format!("{first}{}", characters.as_str().to_lowercase())
                })
                .collect::<Vec<_>>()
                .join(" ")
        }
        _ => base.to_owned(),
    }
}
use macroquad::prelude::*;
use project_rl::ai::AiState;
use project_rl::combat::AttackAreaCell;
use project_rl::explosive::ExplosiveActivation;
use project_rl::game::{ActorObservationField, GameState, WorldState};
use project_rl::world::{DoorState, GridPos, Map, Terrain, VisibilityState};

use crate::test_sector::{Decor, SectorDecor, TestSector};

#[path = "terminal_fx.rs"]
mod terminal_fx;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct KnownTile {
    pub terrain: Terrain,
    pub decor: Decor,
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalDrawOptions<'a> {
    pub bounds: Rect,
    pub ui_scale: f32,
    pub cell_size: u16,
    pub reduced_motion: bool,
    pub interact_label: &'a str,
    pub interaction_focus: Option<GridPos>,
    pub multiple_interactions: bool,
    pub legend_label: &'a str,
    pub observation_label: &'a str,
    pub legend_open: bool,
    pub attack_preview: Option<TerminalAttackPreview<'a>>,
    pub navigation_signal: Option<&'a str>,
    pub target_summary: Option<&'a TerminalTargetSummary>,
    pub observation_fields: &'a [ActorObservationField],
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InteractionHint {
    title: String,
    is_loot: bool,
    verb: &'static str,
    unavailable: Option<&'static str>,
}

// Only live, perceived state may be passed here; remembered decor never implies
// an interaction still exists.
fn interaction_hint(game: &WorldState, position: GridPos) -> Option<InteractionHint> {
    if !game.player_visibility().is_visible(position) {
        return None;
    }
    if game.ground_items().item_at(position).is_some() {
        return Some(InteractionHint {
            title: match game.ground_items().count_at(position) {
                1 => "Objet au sol".to_owned(),
                count => format!("{count} objets au sol"),
            },
            is_loot: true,
            verb: "ramasser",
            unavailable: None,
        });
    }
    if let Some(link) = game.passage(position) {
        return Some(InteractionHint {
            title: format!("Vers {}", player_location_name(game.destination_name(link))),
            is_loot: false,
            verb: "voyager",
            unavailable: None,
        });
    }
    if game
        .threat_sources()
        .iter()
        .any(|source| source.position() == position && source.is_active())
    {
        return Some(InteractionHint {
            title: "Camp hostile actif".to_owned(),
            is_loot: false,
            verb: "neutraliser",
            unavailable: None,
        });
    }
    if let Some(entity) = game.actors().entity_at(position) {
        use project_rl::facility::WorkerRole;
        let title = if game.active_merchant(entity) {
            Some("Marchande")
        } else if game.active_clinic(entity) {
            Some("Soigneur")
        } else if let Some(role) = game.active_worker_role(entity) {
            Some(match role {
                WorkerRole::Retriever => "Récupérateur",
                WorkerRole::Technician => "Technicien",
            })
        } else if game.active_resident(entity)
            || game.active_quest_provider(entity)
            || game
                .current_zone()
                .is_some_and(|zone| game.narrative_name_key_in(&zone.id, entity).is_some())
        {
            Some("Habitant")
        } else {
            None
        };
        if let Some(title) = title {
            return Some(InteractionHint {
                title: title.to_owned(),
                is_loot: false,
                verb: "parler",
                unavailable: None,
            });
        }
    }
    if let Some(facility) = game.active_facility()
        && facility.is_player_interactive_at(position)
    {
        let is_direction_board = facility
            .installation_at(position)
            .is_some_and(|installation| installation.id().as_str() == "core:orme_direction_board");
        let is_service_plan = facility
            .installation_at(position)
            .is_some_and(|installation| installation.id().as_str() == "core:relay_service_plan");
        return Some(InteractionHint {
            title: if is_direction_board {
                "Panneau d'orientation"
            } else if is_service_plan {
                "Plan du relais"
            } else if facility.data_terminal_record_at(position).is_some() {
                "Terminal de données"
            } else {
                "Dépôt de maintenance"
            }
            .to_owned(),
            is_loot: false,
            verb: if facility.data_terminal_record_at(position).is_some() {
                if is_direction_board || is_service_plan {
                    "lire"
                } else {
                    "consulter"
                }
            } else {
                "utiliser"
            },
            unavailable: None,
        });
    }
    let terrain = game.map().tile(position)?.terrain;
    let (title, verb, unavailable) = match terrain {
        Terrain::Door(DoorState::Open) => ("Porte ouverte", "fermer", None),
        Terrain::Door(DoorState::Closed) => ("Porte fermée", "ouvrir", None),
        Terrain::Door(DoorState::Locked) => (
            "Accès verrouillé",
            "ouvrir",
            Some("Autorisation nécessaire"),
        ),
        Terrain::Door(DoorState::Unpowered) => {
            ("Porte hors service", "ouvrir", Some("Alimentation absente"))
        }
        Terrain::ControlPanel {
            activated: true, ..
        } => ("Console utilisée", "activer", Some("Déjà activée")),
        Terrain::ControlPanel {
            activated: false, ..
        } => ("Console active", "activer", None),
        _ => return None,
    };
    Some(InteractionHint {
        title: title.to_owned(),
        is_loot: false,
        verb,
        unavailable,
    })
}

fn interaction_action(
    player: Option<GridPos>,
    position: GridPos,
    focus: Option<GridPos>,
    hint: &InteractionHint,
    interact_label: &str,
    multiple_interactions: bool,
) -> String {
    let Some(player) = player else {
        return "Approchez-vous".to_owned();
    };
    if hint.is_loot {
        return if player == position {
            if multiple_interactions {
                format!("{interact_label} · choisir : ramasser")
            } else {
                format!("{interact_label} · ramasser")
            }
        } else {
            "Rejoignez la case pour ramasser".to_owned()
        };
    }
    if player.x.abs_diff(position.x) + player.y.abs_diff(position.y) > 1 {
        return "Approchez-vous pour interagir".to_owned();
    }
    if let Some(reason) = hint.unavailable {
        return reason.to_owned();
    }
    if multiple_interactions || focus != Some(position) {
        return format!("{interact_label} · choisir : {}", hint.verb);
    }
    format!("{interact_label} · {}", hint.verb)
}

fn inside_actor_visual_field(fields: &[ActorObservationField], position: GridPos) -> bool {
    fields.iter().any(|field| field.contains(position))
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalTargetSummary {
    pub name: String,
    pub distance: u32,
    pub visible_state: String,
    pub analysis: Option<TerminalTargetAnalysis>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalTargetAnalysis {
    pub integrity: u16,
    pub maximum_integrity: u16,
    pub armor: u16,
    pub resistances: String,
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalAttackPreview<'a> {
    pub cells: &'a [AttackAreaCell],
    pub cursor: GridPos,
    pub valid: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum TerminalAlertKind {
    LocalWitness,
    SecuritySystem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalStatusIcon {
    Burning,
    Sheltered,
    RequiredMaterial,
    QuestAvailable,
    QuestReady,
    QuestInProgress,
    QuestObjective,
}

impl TerminalStatusIcon {
    fn is_quest(self) -> bool {
        matches!(
            self,
            Self::QuestAvailable | Self::QuestReady | Self::QuestInProgress | Self::QuestObjective
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalEffectBadge {
    Fracture { charges: u16, threshold: u16 },
    Alternation,
    Echo { turns: u16 },
    Guard,
    Burning,
    Corrosion,
    Caustic,
    Electrical,
    Fragile,
    Slowed,
    Suppressed,
    Timed,
}

#[derive(Clone, Debug)]
pub struct TerminalOverlay {
    pub symbol: char,
    pub color: Color,
    pub accent_color: Option<Color>,
    pub highlight_color: Option<Color>,
    pub selected: bool,
    /// A directly perceived alert source. Hidden actors and installations
    /// never reach this layer.
    pub alert: Option<TerminalAlertKind>,
    /// A small shape displayed in addition to the entity glyph, so a status
    /// never relies on color alone.
    pub status_icon: Option<TerminalStatusIcon>,
    pub effect_badges: Vec<TerminalEffectBadge>,
    pub sustained_effects: Vec<crate::visual_effects::TerminalEffectSample>,
    pub effect: Option<crate::visual_effects::TerminalEffectSample>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SensorContactKind {
    Hostile,
    Service,
    Neutral,
    Unknown,
}

#[derive(Clone, Copy, Debug)]
struct SensorContact {
    position: GridPos,
    kind: SensorContactKind,
    selected: bool,
    alerted: bool,
}

fn sensor_contact_kind(symbol: char) -> SensorContactKind {
    match symbol {
        'd' | 't' | 'r' | 'j' | 'w' | 'B' | 'V' | 'K' | 'X' | 'Y' | 'L' | 'A' => {
            SensorContactKind::Hostile
        }
        'm' | 'h' | 'v' => SensorContactKind::Service,
        'c' | 'i' | 'u' | 'b' | 'G' | 'N' => SensorContactKind::Neutral,
        _ => SensorContactKind::Unknown,
    }
}

#[derive(Clone, Copy, Debug)]
struct TerminalGlyphPalette {
    primary: Color,
    accent: Option<Color>,
    highlight: Option<Color>,
}

impl TerminalGlyphPalette {
    const fn new(primary: Color, accent: Option<Color>, highlight: Option<Color>) -> Self {
        Self {
            primary,
            accent,
            highlight,
        }
    }

    const fn monochrome(primary: Color) -> Self {
        Self::new(primary, None, None)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TerminalView {
    pub title: String,
    pub decor: SectorDecor,
    remembered: BTreeMap<GridPos, KnownTile>,
}

impl TerminalView {
    /// Developer-only plan, deliberately labelled and separate from gameplay.
    /// Does not alter player perception or this view's remembered map.
    #[cfg(debug_assertions)]
    pub fn draw_debug_overview(&self, game: &GameState) {
        clear_background(Color::from_rgba(6, 13, 19, 255));
        draw_ui_text_bold(
            "PLAN DE DIAGNOSTIC · CARTE COMPLÈTE · HORS JEU",
            28.0,
            36.0,
            23.0,
            WHITE,
        );
        draw_ui_text(
            "Ville fixe et sûre à gauche / extérieur variable à droite et au sud",
            28.0,
            62.0,
            18.0,
            SKYBLUE,
        );
        let size = ((screen_width() - 60.0) / game.map().width() as f32)
            .min((screen_height() - 130.0) / game.map().height() as f32)
            .floor();
        let ox = ((screen_width() - game.map().width() as f32 * size) / 2.0).floor();
        for y in 0..game.map().height() as i32 {
            for x in 0..game.map().width() as i32 {
                let p = GridPos::new(x, y);
                let tile = game.map().tile(p).unwrap();
                draw_tile(
                    Rect::new(ox + x as f32 * size, 88.0 + y as f32 * size, size, size),
                    self.decor.at(p, tile.terrain),
                    p.cardinal_neighbors().map(|n| game.map().blocks_vision(n)),
                    true,
                    p,
                );
            }
        }
        draw_ui_text(
            "Les parties normales affichent uniquement la perception et la mémoire du joueur.",
            28.0,
            screen_height() - 22.0,
            17.0,
            LIGHTGRAY,
        );
    }

    pub fn new(decor: SectorDecor, map: &Map, visibility: &VisibilityState) -> Self {
        let mut view = Self {
            title: TestSector::NAME.to_owned(),
            decor,
            remembered: BTreeMap::new(),
        };
        view.observe(map, visibility);
        view
    }

    pub fn observe(&mut self, map: &Map, visibility: &VisibilityState) {
        for position in visibility.visible_positions() {
            if let Some(tile) = map.tile(position) {
                self.remembered.insert(
                    position,
                    KnownTile {
                        terrain: tile.terrain,
                        decor: self.decor.at(position, tile.terrain),
                    },
                );
            }
        }
    }

    pub fn known(&self, position: GridPos) -> Option<KnownTile> {
        self.remembered.get(&position).copied()
    }

    /// Maps a screen-space pointer through the exact camera used by `draw`.
    /// Unknown cells are excluded so pointing cannot reveal the map.
    pub fn hit_test(
        &self,
        game: &WorldState,
        bounds: Rect,
        ui_scale: f32,
        cell_size: u16,
        navigation_signal_visible: bool,
        pointer: (f32, f32),
    ) -> Option<GridPos> {
        terminal_grid_camera(game, bounds, ui_scale, cell_size, navigation_signal_visible)
            .1
            .hit(pointer)
            .filter(|position| self.known(*position).is_some())
    }

    /// Returns the exact on-screen rectangle used to draw a world cell.
    /// Presentation overlays can therefore follow the terminal camera without
    /// duplicating its zoom, sidebar or sensor-footprint calculations.
    pub fn world_cell_rect(
        &self,
        game: &WorldState,
        bounds: Rect,
        ui_scale: f32,
        cell_size: u16,
        navigation_signal_visible: bool,
        position: GridPos,
    ) -> Option<Rect> {
        let camera =
            terminal_grid_camera(game, bounds, ui_scale, cell_size, navigation_signal_visible).1;
        camera.contains(position).then(|| camera.rect(position))
    }

    /// Neighbour joins never consult unknown terrain, including in the minimap.
    fn wall_joins(&self, position: GridPos) -> [bool; 4] {
        position
            .cardinal_neighbors()
            .map(|p| self.known(p).is_some_and(|t| t.terrain == Terrain::Wall))
    }

    pub fn draw(
        &self,
        game: &WorldState,
        options: TerminalDrawOptions<'_>,
        overlay: impl Fn(GridPos) -> Option<TerminalOverlay>,
        interaction_name: impl Fn(GridPos) -> Option<String>,
    ) {
        let TerminalDrawOptions {
            bounds,
            ui_scale,
            cell_size,
            reduced_motion,
            interact_label,
            interaction_focus,
            multiple_interactions,
            legend_label,
            observation_label,
            legend_open,
            attack_preview,
            navigation_signal,
            target_summary,
            observation_fields,
        } = options;
        let cyan = Color::from_rgba(104, 201, 201, 255);
        let muted = Color::from_rgba(135, 162, 167, 255);
        let layout = terminal_ui_layout(bounds, ui_scale, navigation_signal.is_some());
        draw_rectangle(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            Color::from_rgba(7, 15, 21, 255),
        );
        draw_rectangle_lines(
            bounds.x,
            bounds.y,
            bounds.w,
            bounds.h,
            1.0,
            Color::from_rgba(45, 79, 86, 185),
        );
        let protected = game
            .player_position()
            .is_some_and(|p| game.map().is_protected(p));
        let safety_label = if protected {
            "ZONE SÛRE"
        } else {
            "ZONE HOSTILE"
        };
        let safety_width = measure_ui_text(safety_label, None, 16, 1.0).width;
        let safety_reserved = if layout.header.w > 560.0 {
            safety_width + 23.0
        } else {
            0.0
        };
        draw_bounded_text(
            &self.title,
            layout.header.x,
            layout.header.y + 23.0,
            layout.header.w - safety_reserved,
            18,
            cyan,
        );
        if layout.header.w > 560.0 {
            let safety_chip = Rect::new(
                layout.header.x + layout.header.w - safety_width - 13.0,
                layout.header.y + 3.0,
                safety_width + 13.0,
                25.0,
            );
            UiTheme.hud_chip(safety_chip);
            draw_ui_text(
                safety_label,
                layout.header.x + layout.header.w - safety_width,
                layout.header.y + 22.0,
                16.0,
                if protected { cyan } else { ORANGE },
            );
        }
        if let (Some(signal), Some(signal_rect)) = (navigation_signal, layout.route) {
            UiTheme.hud_chip(signal_rect);
            draw_ui_text_bold(
                "ITINÉRAIRE",
                signal_rect.x + 10.0,
                signal_rect.y + 19.0,
                13.0,
                muted,
            );
            draw_bounded_text(
                signal,
                signal_rect.x + 94.0,
                signal_rect.y + 19.0,
                signal_rect.w - 104.0,
                15,
                Color::from_rgba(142, 234, 215, 255),
            );
        }
        let (sidebar, camera) = terminal_grid_camera(
            game,
            bounds,
            ui_scale,
            cell_size,
            navigation_signal.is_some(),
        );
        let focus = game.player_position().unwrap_or(GridPos::new(12, 12));
        let visibility = game.player_visibility();
        for row in 0..camera.rows {
            for column in 0..camera.columns {
                let position = GridPos::new(camera.first.x + column, camera.first.y + row);
                let rect = camera.rect(position);
                let known = self.known(position);
                let observed = observation_fields
                    .iter()
                    .any(|field| field.contains(position) || field.origin() == position);
                if known.is_none() && !observed {
                    continue;
                }
                let visible = visibility.is_visible(position);
                if let Some(tile) = known {
                    draw_tile(
                        rect,
                        tile.decor,
                        self.wall_joins(position),
                        visible,
                        position,
                    );
                } else {
                    draw_rectangle(
                        rect.x,
                        rect.y,
                        rect.w,
                        rect.h,
                        Color::from_rgba(4, 10, 15, 255),
                    );
                }
                draw_observation_fields(rect, observation_fields, position);
                if visible
                    && let Some(preview) = attack_preview
                    && let Some(cell) = preview.cells.iter().find(|cell| cell.position == position)
                {
                    draw_attack_preview_area(rect, preview.valid, cell.step);
                }
                // Even a buggy caller cannot render a live actor/effect in memory.
                if visible && let Some(cell) = overlay(position) {
                    for effect in cell.sustained_effects {
                        terminal_fx::draw(rect, effect, cell.symbol != '\0');
                    }
                    if let Some(effect) = cell.effect {
                        terminal_fx::draw(rect, effect, cell.symbol != '\0');
                    }
                    // Effects never replace the occupant glyph or its identity
                    // in the sensor panel during the animation.
                    if cell.symbol != '\0' {
                        draw_entity(
                            rect,
                            cell.symbol,
                            TerminalGlyphPalette::new(
                                cell.color,
                                cell.accent_color,
                                cell.highlight_color,
                            ),
                            cell.selected,
                            cell.alert.is_some(),
                            cell.status_icon.filter(|icon| !icon.is_quest()),
                        );
                    }
                }
                if visible
                    && game.active_facility().is_some_and(|facility| {
                        facility
                            .security_door_lockdown_at(position, game.turn())
                            .is_some()
                    })
                {
                    draw_alert_marker(rect);
                }
                if visible
                    && let Some(preview) = attack_preview
                    && preview.cursor == position
                {
                    draw_attack_preview_cursor(rect, preview.valid);
                }
            }
        }
        // A second pass keeps overhead markers above neighbouring terrain and
        // supports installations even when no actor/item overlay occupies them.
        // A gentle 0–3 px float every 2.4 seconds uses presentation time only.
        let quest_marker_rise = if reduced_motion {
            0.0
        } else {
            (1.5 * (1.0 - (get_time() * std::f64::consts::TAU / 2.4).cos())) as f32
        };
        for position in visibility
            .visible_positions()
            .filter(|position| camera.contains(*position))
        {
            if let Some(cell) = overlay(position) {
                draw_effect_badges(camera.rect(position), &cell.effect_badges);
            }
            let Some(marker) = game.quest_marker_at(position) else {
                continue;
            };
            let icon = match marker {
                project_rl::game::QuestMarker::Available => TerminalStatusIcon::QuestAvailable,
                project_rl::game::QuestMarker::ReadyToComplete => TerminalStatusIcon::QuestReady,
                project_rl::game::QuestMarker::InProgress => TerminalStatusIcon::QuestInProgress,
                project_rl::game::QuestMarker::Objective => TerminalStatusIcon::QuestObjective,
            };
            let mut marker_rect = camera.rect(position);
            marker_rect.y -= quest_marker_rise;
            draw_status_icon(marker_rect, icon);
        }
        // Telegraphs never reveal a hidden shooter or an unobserved cell.
        for (_, actor) in game.actors().iter() {
            if let project_rl::ai::AiState::Aiming { target_at, .. } = actor.ai_state()
                && visibility.is_visible(actor.position())
                && visibility.is_visible(target_at)
                && camera.contains(target_at)
            {
                let rect = camera.rect(target_at);
                let inset = 3.0;
                let color = Color::from_rgba(255, 125, 100, 255);
                draw_rectangle_lines(
                    rect.x + inset,
                    rect.y + inset,
                    rect.w - 2.0 * inset,
                    rect.h - 2.0 * inset,
                    2.0,
                    color,
                );
                draw_line(
                    rect.x + inset,
                    rect.y + inset,
                    rect.x + rect.w - inset,
                    rect.y + rect.h - inset,
                    1.5,
                    color,
                );
                draw_line(
                    rect.x + rect.w - inset,
                    rect.y + inset,
                    rect.x + inset,
                    rect.y + rect.h - inset,
                    1.5,
                    color,
                );
            }
        }
        let pointer = camera.hit(mouse_position());
        let pointer_inspected = pointer.and_then(|p| self.known(p).map(|t| (p, t)));
        let mut selected_inspected = None;
        let mut visible_hostiles = 0;
        let mut visible_neutrals = 0;
        let mut visible_items = 0;
        let mut visible_local_alerts = 0;
        let mut visible_security_alarms = 0;
        let mut visible_security_lockdowns = 0;
        let mut sensor_contacts = Vec::new();
        for position in visibility.visible_positions() {
            if game.active_facility().is_some_and(|facility| {
                facility
                    .security_door_lockdown_at(position, game.turn())
                    .is_some()
            }) {
                visible_security_lockdowns += 1;
            }
            if let Some(cell) = overlay(position) {
                let contact_kind = sensor_contact_kind(cell.symbol);
                match contact_kind {
                    SensorContactKind::Hostile => visible_hostiles += 1,
                    SensorContactKind::Service | SensorContactKind::Neutral => {
                        visible_neutrals += 1
                    }
                    SensorContactKind::Unknown => {}
                }
                if matches!(cell.symbol, ')' | '!' | '=') {
                    visible_items += 1;
                }
                match cell.alert {
                    Some(TerminalAlertKind::LocalWitness) => visible_local_alerts += 1,
                    Some(TerminalAlertKind::SecuritySystem) => visible_security_alarms += 1,
                    None => {}
                }
                if cell.selected {
                    selected_inspected = self.known(position).map(|tile| (position, tile));
                }
                // Visible contacts do not depend on activating the F2 field overlay.
                if contact_kind != SensorContactKind::Unknown {
                    sensor_contacts.push(SensorContact {
                        position,
                        kind: contact_kind,
                        selected: cell.selected,
                        alerted: cell.alert.is_some(),
                    });
                }
            }
        }
        // Mouse inspection takes priority; keyboard target selection remains a
        // complete alternative when no known map cell is hovered.
        let inspected = pointer_inspected.or(selected_inspected);
        if let Some((position, _)) = inspected
            && game.player_position() != Some(position)
        {
            let rect = camera.rect(position);
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.0,
                Color::from_rgba(151, 191, 198, 255),
            );
        }
        let hovered_interaction = pointer.and_then(|position| {
            visibility
                .is_visible(position)
                .then(|| interaction_hint(game, position).map(|hint| (position, hint)))
                .flatten()
        });
        let focused_interaction = interaction_focus.and_then(|position| {
            (camera.contains(position) && visibility.is_visible(position))
                .then(|| interaction_hint(game, position).map(|hint| (position, hint)))
                .flatten()
        });
        if !legend_open && let Some((position, hint)) = hovered_interaction.or(focused_interaction)
        {
            let rect = camera.rect(position);
            let action = interaction_action(
                game.player_position(),
                position,
                interaction_focus,
                &hint,
                interact_label,
                multiple_interactions,
            );
            let title = interaction_name(position).unwrap_or(hint.title);
            draw_interaction_tooltip(rect, layout.map, &title, &action);
        }
        let description = if let Some((position, tile)) = inspected {
            if visibility.is_visible(position) {
                if let Some(lockdown) = game
                    .active_facility()
                    .and_then(|facility| facility.security_door_lockdown_at(position, game.turn()))
                {
                    format!(
                        "Porte verrouillée · VERROUILLAGE D'ALARME · {}T",
                        lockdown.remaining_turns(game.turn())
                    )
                } else if let Some(link) = game.passage(position) {
                    format!(
                        "Vers {} · {} à proximité pour voyager",
                        player_location_name(game.destination_name(link)),
                        interact_label
                    )
                } else if let Some(source) = game
                    .threat_sources()
                    .iter()
                    .find(|source| source.position() == position)
                {
                    if source.is_active() {
                        format!(
                            "Camp hostile actif · cycle {}T · plafond actif {} · quota restant {}/{} · {} à côté pour neutraliser",
                            source.remaining_turns(),
                            source.maximum_active(),
                            source
                                .maximum_total()
                                .saturating_sub(source.spawned_total()),
                            source.maximum_total(),
                            interact_label,
                        )
                    } else {
                        "Camp hostile neutralisé · aucun nouveau renfort".to_owned()
                    }
                } else if let Some(device) = game
                    .explosive_devices()
                    .at(position)
                    .find(|device| device.is_identified())
                {
                    let state = if device.is_neutralized() {
                        "neutralisée".to_owned()
                    } else if device.triggered_on().is_some() {
                        "séquence de détonation engagée".to_owned()
                    } else {
                        match device.activation() {
                            ExplosiveActivation::Timed { trigger_turn } => format!(
                                "temporisée · {}T",
                                trigger_turn.saturating_sub(game.turn())
                            ),
                            ExplosiveActivation::Proximity { armed_turn, radius } => {
                                if armed_turn > game.turn() {
                                    format!(
                                        "proximité · armement dans {}T · rayon {radius}",
                                        armed_turn - game.turn()
                                    )
                                } else {
                                    format!("proximité armée · rayon {radius}")
                                }
                            }
                            ExplosiveActivation::Remote { maximum_link_range } => {
                                format!("commande distante · liaison {maximum_link_range}")
                            }
                        }
                    };
                    format!("Dispositif explosif · {state}")
                } else if let Some(emitter) = game.sound_emitters().at(position).next() {
                    format!(
                        "Leurre sonore · intensité {} · {} phase(s)",
                        emitter.intensity(),
                        emitter.remaining_phases()
                    )
                } else {
                    overlay(position).map_or_else(
                        || tile.decor.label().to_owned(),
                        |cell| {
                            let label = if cell.symbol == '\0' {
                                ground_effect_description(game.ground_effects(), position)
                                    .unwrap_or_else(|| tile.decor.label().to_owned())
                            } else {
                                overlay_label(cell.symbol).to_owned()
                            };
                            let alert_label = match cell.alert {
                                Some(TerminalAlertKind::LocalWitness) => {
                                    format!("{label} · ALERTE LOCALE")
                                }
                                Some(TerminalAlertKind::SecuritySystem) => {
                                    format!("{label} · ALARME DE SÉCURITÉ")
                                }
                                None => label.to_owned(),
                            };
                            let state_label = game
                                .actors()
                                .entity_at(position)
                                .and_then(|entity| game.actors().get(entity))
                                .and_then(|actor| {
                                    heavy_fauna_state_label(actor).or_else(|| {
                                        actor.ai().map(|_| ai_state_label(actor.ai_state()))
                                    })
                                });
                            let alert_label = if let Some(state) = state_label {
                                format!("{alert_label} · {state}")
                            } else {
                                alert_label
                            };
                            match cell.status_icon {
                                Some(TerminalStatusIcon::Sheltered) => alert_label,
                                Some(TerminalStatusIcon::Burning) => {
                                    format!("{alert_label} · EN FEU")
                                }
                                Some(TerminalStatusIcon::RequiredMaterial) => {
                                    format!("{alert_label} · REQUIS POUR LA MAINTENANCE")
                                }
                                Some(TerminalStatusIcon::QuestAvailable) => {
                                    format!("{alert_label} · QUÊTE DISPONIBLE")
                                }
                                Some(TerminalStatusIcon::QuestReady) => {
                                    format!("{alert_label} · RAPPORT DE QUÊTE ATTENDU")
                                }
                                Some(TerminalStatusIcon::QuestInProgress) => {
                                    format!("{alert_label} · QUÊTE EN COURS")
                                }
                                Some(TerminalStatusIcon::QuestObjective) => {
                                    format!("{alert_label} · OBJECTIF DE QUÊTE")
                                }
                                None => alert_label,
                            }
                        },
                    )
                }
            } else {
                format!("Mémoire · {}", tile.decor.label())
            }
        } else if let Some(link) = std::iter::once(focus)
            .chain(focus.cardinal_neighbors())
            .filter(|p| visibility.is_visible(*p))
            .find_map(|p| game.passage(p))
        {
            format!(
                "{} : rejoindre {} · retour possible",
                interact_label,
                player_location_name(game.destination_name(link))
            )
        } else {
            format!(
                "{}  ·  survolez une case pour l'inspecter",
                if self.decor.zones.is_empty() {
                    &self.title
                } else {
                    self.decor.zone_at(focus)
                }
            )
        };
        let description = inspected
            .filter(|(position, _)| visibility.is_visible(*position))
            .map_or(description.clone(), |(position, _)| {
                let mut result = description.clone();
                if let Some((_, turns)) =
                    game.pending_weapon_echoes().find(|(at, _)| *at == position)
                {
                    result.push_str(&format!(" · ÉCHO DANS {turns} TOUR(S)"));
                }
                if let Some((target, turns)) = game.alternation_previous(game.player_id())
                    && game.actors().entity_at(position) == Some(target)
                {
                    result.push_str(&format!(" · ALTERNANCE {turns} TOUR(S)"));
                }
                result
            });
        let description = inspected.map_or(description.clone(), |(position, _)| {
            if inside_actor_visual_field(observation_fields, position) {
                format!("{description} · CHAMP VISUEL PNJ")
            } else {
                description.clone()
            }
        });
        let description = if inspected.is_some_and(|(position, _)| {
            overlay(position).is_none()
                && game.quest_marker_at(position) == Some(project_rl::game::QuestMarker::Objective)
        }) {
            format!("{description} · OBJECTIF DE QUÊTE")
        } else {
            description
        };
        let description = if sidebar {
            description
        } else {
            format!(
                "POSITION · {} · {description}",
                local_coordinates(game.player_position())
            )
        };
        draw_bounded_text(
            &description,
            layout.footer.x,
            layout.footer.y + 17.0,
            layout.footer.w,
            16,
            muted,
        );
        if sidebar {
            self.draw_sidebar(
                game,
                layout.sidebar.expect("visible sidebar has layout bounds"),
                (
                    visible_hostiles,
                    visible_neutrals,
                    visible_items,
                    visible_local_alerts,
                    visible_security_alarms,
                    visible_security_lockdowns,
                ),
                target_summary,
                observation_fields,
                observation_label,
                &sensor_contacts,
            );
        }
        if legend_open {
            draw_legend_overlay(game, bounds, legend_label);
        }
    }

    fn draw_sidebar(
        &self,
        game: &WorldState,
        panel: Rect,
        visible_counts: (usize, usize, usize, usize, usize, usize),
        target: Option<&TerminalTargetSummary>,
        observation_fields: &[ActorObservationField],
        observation_label: &str,
        sensor_contacts: &[SensorContact],
    ) {
        let (
            visible_hostiles,
            visible_neutrals,
            visible_items,
            visible_local_alerts,
            visible_security_alarms,
            visible_security_lockdowns,
        ) = visible_counts;
        let bright = Color::from_rgba(203, 222, 221, 255);
        let muted = Color::from_rgba(133, 163, 170, 255);
        let cyan = Color::from_rgba(100, 221, 201, 255);
        UiTheme.hud_panel(panel);
        let rect = Rect::new(
            panel.x + 10.0,
            panel.y + 10.0,
            panel.w - 20.0,
            panel.h - 20.0,
        );
        draw_ui_text_bold("CAPTEURS", rect.x, rect.y + 18.0, 18.0, bright);
        draw_ui_text_bold(
            &format!("POSITION · {}", local_coordinates(game.player_position())),
            rect.x,
            rect.y + 40.0,
            14.0,
            cyan,
        );
        let map_area = Rect::new(
            rect.x,
            rect.y + 52.0,
            rect.w,
            (rect.h * 0.38).clamp(180.0, 240.0) - 22.0,
        );
        let (known_min, known_max) = if observation_fields.is_empty() {
            remembered_bounds(
                &self.remembered,
                game.player_position(),
                game.map().width(),
                game.map().height(),
            )
        } else {
            (
                GridPos::new(0, 0),
                GridPos::new(
                    i32::try_from(game.map().width().saturating_sub(1)).unwrap_or(i32::MAX),
                    i32::try_from(game.map().height().saturating_sub(1)).unwrap_or(i32::MAX),
                ),
            )
        };
        let known_width = (known_max.x - known_min.x + 1).max(1) as f32;
        let known_height = (known_max.y - known_min.y + 1).max(1) as f32;
        let pixel = (map_area.w / known_width)
            .min(map_area.h / known_height)
            .floor()
            .clamp(1.0, 12.0);
        let map_width = known_width * pixel;
        let map_height = known_height * pixel;
        let origin = vec2(
            map_area.x + (map_area.w - map_width) * 0.5,
            map_area.y + (map_area.h - map_height) * 0.5,
        );
        draw_sensor_scope(
            map_area,
            &self.remembered,
            game,
            known_min,
            known_max,
            pixel,
            origin,
            observation_fields,
            sensor_contacts,
        );
        let mut y = map_area.y + map_area.h + 20.0;
        UiTheme.hud_chip(Rect::new(rect.x, y - 21.0, rect.w, 29.0));
        draw_ui_icon(UiIcon::All, Rect::new(rect.x, y - 15.0, 16.0, 16.0), cyan);
        draw_ui_text_bold("EN VUE", rect.x + 24.0, y, 16.0, bright);
        y += 23.0;
        draw_bounded_text(
            &format!("Hostiles {visible_hostiles}  ·  Neutres {visible_neutrals}"),
            rect.x,
            y,
            rect.w,
            15,
            muted,
        );
        y += 18.0;
        draw_ui_text(format!("Objets {visible_items}"), rect.x, y, 15.0, muted);
        let alert = Color::from_rgba(255, 175, 83, 255);
        for line in [
            (visible_local_alerts, "Alertes locales"),
            (visible_security_alarms, "Alarmes réseau"),
            (visible_security_lockdowns, "Verrouillages"),
        ]
        .into_iter()
        .filter(|(count, _)| *count > 0)
        {
            y += 19.0;
            draw_bounded_text(
                &format!("{} {}", line.1, line.0),
                rect.x,
                y,
                rect.w,
                14,
                alert,
            );
        }

        if !observation_fields.is_empty() {
            let observers: BTreeSet<_> = observation_fields
                .iter()
                .map(ActorObservationField::observer)
                .collect();
            y += 19.0;
            draw_bounded_text(
                &format!(
                    "{observation_label} · lecture tactique · {} PNJ visibles",
                    observers.len()
                ),
                rect.x,
                y,
                rect.w,
                14,
                cyan,
            );
            y += 18.0;
            draw_bounded_text(
                "Trame contourée · champ visuel",
                rect.x,
                y,
                rect.w,
                13,
                Color::from_rgba(79, 218, 183, 255),
            );
        }

        y += 23.0;
        UiTheme.hud_chip(Rect::new(rect.x, y - 21.0, rect.w, 29.0));
        draw_ui_icon(
            UiIcon::Target,
            Rect::new(rect.x, y - 15.0, 16.0, 16.0),
            cyan,
        );
        draw_ui_text_bold("CIBLE", rect.x + 24.0, y, 16.0, bright);
        y += 23.0;
        if let Some(target) = target {
            draw_bounded_text(&target.name, rect.x, y, rect.w, 16, bright);
            y += 20.0;
            draw_bounded_text(
                &format!("Distance {}", target.distance),
                rect.x,
                y,
                rect.w,
                14,
                muted,
            );
            for state in target.visible_state.split(" · ") {
                // Leave room for the analysis panel even with many statuses.
                if y + 125.0 >= rect.bottom() {
                    break;
                }
                y += 17.0;
                draw_bounded_text(state, rect.x, y, rect.w, 14, muted);
            }
            y += 22.0;
            if let Some(analysis) = &target.analysis {
                draw_ui_text_bold(
                    format!(
                        "Intégrité {}/{}",
                        analysis.integrity, analysis.maximum_integrity
                    ),
                    rect.x,
                    y,
                    14.0,
                    bright,
                );
                y += 7.0;
                draw_hud_progress(
                    Rect::new(rect.x, y, rect.w, 5.0),
                    analysis.integrity,
                    analysis.maximum_integrity,
                    UiTheme.success(),
                );
                y += 23.0;
                draw_ui_text(
                    format!("Armure {}", analysis.armor),
                    rect.x,
                    y,
                    14.0,
                    bright,
                );
                y += 19.0;
                draw_wrapped_lines(&analysis.resistances, rect.x, y, rect.w, 13, 2, muted);
            } else {
                draw_bounded_text("DONNÉES TACTIQUES MASQUÉES", rect.x, y, rect.w, 13, muted);
                y += 19.0;
                draw_ui_text("Analyse de cible · REC-01", rect.x, y, 14.0, cyan);
            }
        } else {
            draw_ui_text("Aucune cible sélectionnée", rect.x, y, 15.0, muted);
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_sensor_scope(
    area: Rect,
    remembered: &BTreeMap<GridPos, KnownTile>,
    game: &WorldState,
    known_min: GridPos,
    known_max: GridPos,
    pixel: f32,
    origin: Vec2,
    observation_fields: &[ActorObservationField],
    contacts: &[SensorContact],
) {
    let phosphor = Color::from_rgba(74, 207, 187, 255);
    let phosphor_dim = Color::from_rgba(38, 104, 105, 255);
    let memory = Color::from_rgba(57, 81, 91, 255);
    draw_rectangle(
        area.x,
        area.y,
        area.w,
        area.h,
        Color::from_rgba(2, 8, 12, 255),
    );

    // A hardware-like phosphor grid. It is purely presentational and never
    // participates in perception or delays already available information.
    let grid_step = 16.0;
    let mut grid_x = area.x + grid_step;
    while grid_x < area.right() {
        draw_line(
            grid_x,
            area.y,
            grid_x,
            area.bottom(),
            1.0,
            Color::from_rgba(24, 62, 68, 74),
        );
        grid_x += grid_step;
    }
    let mut grid_y = area.y + grid_step;
    while grid_y < area.bottom() {
        draw_line(
            area.x,
            grid_y,
            area.right(),
            grid_y,
            1.0,
            Color::from_rgba(24, 62, 68, 74),
        );
        grid_y += grid_step;
    }
    let mut scanline_y = area.y + 3.0;
    while scanline_y < area.bottom() {
        draw_line(
            area.x + 1.0,
            scanline_y,
            area.right() - 1.0,
            scanline_y,
            1.0,
            Color::from_rgba(0, 0, 0, 38),
        );
        scanline_y += 5.0;
    }

    for (position, tile) in remembered {
        if !sensor_position_is_inside(*position, known_min, known_max) {
            continue;
        }
        let cell = sensor_cell_rect(*position, known_min, pixel, origin);
        let visible = game.player_visibility().is_visible(*position);
        if tile.terrain.blocks_movement() {
            let edge_color = if visible { phosphor_dim } else { memory };
            let neighbours = position.cardinal_neighbors();
            let joined = neighbours.map(|neighbour| {
                remembered
                    .get(&neighbour)
                    .is_some_and(|known| known.terrain.blocks_movement())
            });
            if !joined[0] {
                draw_sensor_edge(cell.x, cell.y, cell.right(), cell.y, edge_color, !visible);
            }
            if !joined[1] {
                draw_sensor_edge(
                    cell.right(),
                    cell.y,
                    cell.right(),
                    cell.bottom(),
                    edge_color,
                    !visible,
                );
            }
            if !joined[2] {
                draw_sensor_edge(
                    cell.x,
                    cell.bottom(),
                    cell.right(),
                    cell.bottom(),
                    edge_color,
                    !visible,
                );
            }
            if !joined[3] {
                draw_sensor_edge(cell.x, cell.y, cell.x, cell.bottom(), edge_color, !visible);
            }
        } else if visible {
            let dot = (pixel * 0.2).clamp(0.7, 1.35);
            draw_circle(
                cell.x + cell.w * 0.5,
                cell.y + cell.h * 0.5,
                dot,
                Color::from_rgba(56, 159, 149, 150),
            );
        } else if sensor_memory_sample(*position) {
            let dot = (pixel * 0.13).clamp(0.5, 0.9);
            draw_circle(
                cell.x + cell.w * 0.5,
                cell.y + cell.h * 0.5,
                dot,
                Color::from_rgba(66, 88, 96, 135),
            );
        }
    }

    // The solid contour is the player's current reliable sensor footprint.
    // Remembered terrain remains dashed and dim outside it.
    for position in game.player_visibility().visible_positions() {
        if !sensor_position_is_inside(position, known_min, known_max) {
            continue;
        }
        let cell = sensor_cell_rect(position, known_min, pixel, origin);
        let [north, east, south, west] = position.cardinal_neighbors();
        let boundary = Color::from_rgba(88, 225, 201, 125);
        if !game.player_visibility().is_visible(north) {
            draw_line(cell.x, cell.y, cell.right(), cell.y, 1.0, boundary);
        }
        if !game.player_visibility().is_visible(east) {
            draw_line(
                cell.right(),
                cell.y,
                cell.right(),
                cell.bottom(),
                1.0,
                boundary,
            );
        }
        if !game.player_visibility().is_visible(south) {
            draw_line(
                cell.x,
                cell.bottom(),
                cell.right(),
                cell.bottom(),
                1.0,
                boundary,
            );
        }
        if !game.player_visibility().is_visible(west) {
            draw_line(cell.x, cell.y, cell.x, cell.bottom(), 1.0, boundary);
        }
    }

    // REC-06 overlays only fields already authorized by the game state. The
    // dotted fill never draws terrain that is absent from remembered data.
    for field in observation_fields {
        for position in field.positions() {
            if !sensor_position_is_inside(position, known_min, known_max) {
                continue;
            }
            let cell = sensor_cell_rect(position, known_min, pixel, origin);
            if (position.x + position.y).rem_euclid(2) == 0 {
                draw_circle(
                    cell.x + cell.w * 0.5,
                    cell.y + cell.h * 0.5,
                    (pixel * 0.22).clamp(0.65, 1.15),
                    Color::from_rgba(73, 218, 182, 125),
                );
            }
            let [north, east, south, west] = position.cardinal_neighbors();
            let boundary = Color::from_rgba(89, 233, 195, 188);
            if !field.contains(north) {
                draw_sensor_edge(cell.x, cell.y, cell.right(), cell.y, boundary, true);
            }
            if !field.contains(east) {
                draw_sensor_edge(
                    cell.right(),
                    cell.y,
                    cell.right(),
                    cell.bottom(),
                    boundary,
                    true,
                );
            }
            if !field.contains(south) {
                draw_sensor_edge(
                    cell.x,
                    cell.bottom(),
                    cell.right(),
                    cell.bottom(),
                    boundary,
                    true,
                );
            }
            if !field.contains(west) {
                draw_sensor_edge(cell.x, cell.y, cell.x, cell.bottom(), boundary, true);
            }
        }
    }

    // Live affordances live on the sensor map; they never appear in stale
    // explored memory, even when the underlying terrain remains drawn there.
    for position in game.player_visibility().visible_positions() {
        if !sensor_position_is_inside(position, known_min, known_max)
            || game.player_position() == Some(position)
        {
            continue;
        }
        if let Some(hint) = interaction_hint(game, position) {
            draw_sensor_interaction(sensor_cell_rect(position, known_min, pixel, origin), &hint);
        }
    }
    for contact in contacts {
        if sensor_position_is_inside(contact.position, known_min, known_max) {
            draw_sensor_contact(
                sensor_cell_rect(contact.position, known_min, pixel, origin),
                *contact,
            );
        }
    }
    for position in game.player_visibility().visible_positions() {
        if !sensor_position_is_inside(position, known_min, known_max) {
            continue;
        }
        if let Some(marker) = game.quest_marker_at(position) {
            draw_sensor_quest_marker(sensor_cell_rect(position, known_min, pixel, origin), marker);
        }
    }
    if let Some(position) = game.player_position()
        && sensor_position_is_inside(position, known_min, known_max)
    {
        draw_sensor_player(sensor_cell_rect(position, known_min, pixel, origin));
    }

    // The moving sweep and its fading tail are feedback only: all reliable
    // information is drawn before it and is available immediately.
    let sweep = area.x + (get_time() as f32 * 0.19).fract() * area.w;
    draw_rectangle(
        (sweep - 8.0).max(area.x),
        area.y + 1.0,
        (sweep - area.x).min(8.0),
        area.h - 2.0,
        Color::from_rgba(45, 185, 165, 18),
    );
    draw_line(
        sweep,
        area.y + 1.0,
        sweep,
        area.bottom() - 1.0,
        1.0,
        Color::from_rgba(110, 246, 216, 155),
    );
    draw_triangle(
        vec2(sweep - 3.0, area.y + 1.0),
        vec2(sweep + 3.0, area.y + 1.0),
        vec2(sweep, area.y + 5.0),
        Color::from_rgba(110, 246, 216, 190),
    );

    draw_rectangle_lines(
        area.x,
        area.y,
        area.w,
        area.h,
        1.0,
        Color::from_rgba(44, 105, 111, 255),
    );
    draw_sensor_corners(area, phosphor);
}

fn sensor_position_is_inside(position: GridPos, minimum: GridPos, maximum: GridPos) -> bool {
    position.x >= minimum.x
        && position.y >= minimum.y
        && position.x <= maximum.x
        && position.y <= maximum.y
}

fn sensor_cell_rect(position: GridPos, minimum: GridPos, pixel: f32, origin: Vec2) -> Rect {
    Rect::new(
        origin.x + (position.x - minimum.x) as f32 * pixel,
        origin.y + (position.y - minimum.y) as f32 * pixel,
        pixel,
        pixel,
    )
}

fn sensor_memory_sample(position: GridPos) -> bool {
    (position.x.wrapping_mul(31) ^ position.y.wrapping_mul(17)).rem_euclid(5) == 0
}

fn draw_sensor_edge(from_x: f32, from_y: f32, to_x: f32, to_y: f32, color: Color, dashed: bool) {
    if !dashed || (from_x - to_x).abs().max((from_y - to_y).abs()) <= 2.0 {
        draw_line(from_x, from_y, to_x, to_y, 1.0, color);
        return;
    }
    for segment in [0.0, 0.5] {
        let start = segment;
        let end = (segment + 0.28_f32).min(1.0);
        draw_line(
            from_x + (to_x - from_x) * start,
            from_y + (to_y - from_y) * start,
            from_x + (to_x - from_x) * end,
            from_y + (to_y - from_y) * end,
            1.0,
            color,
        );
    }
}

fn draw_sensor_interaction(cell: Rect, hint: &InteractionHint) {
    let center = vec2(cell.x + cell.w * 0.5, cell.y + cell.h * 0.5);
    let radius = (cell.w * 0.65).clamp(3.5, 5.2);
    let color = if hint.is_loot {
        Color::from_rgba(255, 211, 92, 255)
    } else if hint.unavailable.is_some() {
        Color::from_rgba(152, 177, 178, 255)
    } else {
        Color::from_rgba(121, 231, 218, 255)
    };
    draw_circle(
        center.x,
        center.y,
        radius + 1.2,
        Color::from_rgba(2, 8, 12, 255),
    );
    if hint.is_loot {
        draw_circle_lines(center.x, center.y, radius, 1.5, color);
        draw_circle(center.x, center.y, 1.0, color);
    } else if hint.unavailable.is_some() {
        draw_line(
            center.x - radius,
            center.y - radius,
            center.x + radius,
            center.y + radius,
            1.5,
            color,
        );
        draw_line(
            center.x + radius,
            center.y - radius,
            center.x - radius,
            center.y + radius,
            1.5,
            color,
        );
    } else {
        draw_line(
            center.x - radius,
            center.y,
            center.x + radius,
            center.y,
            1.5,
            color,
        );
        draw_line(
            center.x,
            center.y - radius,
            center.x,
            center.y + radius,
            1.5,
            color,
        );
    }
}

fn draw_sensor_quest_marker(cell: Rect, marker: project_rl::game::QuestMarker) {
    let symbol = match marker {
        project_rl::game::QuestMarker::Available => "!",
        project_rl::game::QuestMarker::ReadyToComplete => "?",
        project_rl::game::QuestMarker::InProgress => "…",
        project_rl::game::QuestMarker::Objective => "*",
    };
    let size = (cell.w * 1.5).round().clamp(11.0, 14.0) as u16;
    let rect = Rect::new(
        cell.x + cell.w * 0.5 - 7.0,
        cell.y + cell.h * 0.5 - 7.0,
        14.0,
        14.0,
    );
    let shadow = Rect::new(rect.x + 1.0, rect.y + 1.0, rect.w, rect.h);
    draw_ui_text_bold_centered(symbol, shadow, size, Color::from_rgba(2, 8, 12, 255));
    draw_ui_text_bold_centered(symbol, rect, size, Color::from_rgba(255, 230, 130, 255));
}

fn draw_sensor_contact(cell: Rect, contact: SensorContact) {
    let center = vec2(cell.x + cell.w * 0.5, cell.y + cell.h * 0.5);
    let radius = (cell.w * 0.65).clamp(2.7, 5.0);
    let color = match contact.kind {
        SensorContactKind::Hostile => Color::from_rgba(255, 134, 105, 255),
        SensorContactKind::Service => Color::from_rgba(112, 222, 212, 255),
        SensorContactKind::Neutral => Color::from_rgba(204, 230, 223, 255),
        SensorContactKind::Unknown => Color::from_rgba(244, 190, 101, 255),
    };
    match contact.kind {
        SensorContactKind::Hostile => {
            draw_triangle(
                vec2(center.x, center.y - radius),
                vec2(center.x + radius, center.y),
                vec2(center.x, center.y + radius),
                color,
            );
            draw_triangle(
                vec2(center.x, center.y - radius),
                vec2(center.x - radius, center.y),
                vec2(center.x, center.y + radius),
                color,
            );
            draw_circle(center.x, center.y, (radius * 0.28).max(1.0), BLACK);
        }
        SensorContactKind::Service => {
            draw_rectangle_lines(
                center.x - radius,
                center.y - radius,
                radius * 2.0,
                radius * 2.0,
                1.0,
                color,
            );
            draw_line(
                center.x - radius * 0.55,
                center.y,
                center.x + radius * 0.55,
                center.y,
                1.0,
                color,
            );
            draw_line(
                center.x,
                center.y - radius * 0.55,
                center.x,
                center.y + radius * 0.55,
                1.0,
                color,
            );
        }
        SensorContactKind::Neutral => {
            draw_circle_lines(center.x, center.y, radius, 1.0, color);
            draw_circle(center.x, center.y, (radius * 0.3).max(1.0), color);
        }
        SensorContactKind::Unknown => {
            draw_line(
                center.x - radius,
                center.y - radius,
                center.x + radius,
                center.y + radius,
                1.0,
                color,
            );
            draw_line(
                center.x + radius,
                center.y - radius,
                center.x - radius,
                center.y + radius,
                1.0,
                color,
            );
        }
    }
    if contact.alerted {
        let pulse = ((get_time() * 5.0).sin() * 0.5 + 0.5) as f32;
        draw_circle_lines(
            center.x,
            center.y,
            radius + 2.0 + pulse * 2.0,
            1.0,
            Color::from_rgba(255, 183, 91, 215),
        );
    }
    if contact.selected {
        draw_circle_lines(
            center.x,
            center.y,
            radius + 3.0,
            1.0,
            Color::from_rgba(230, 244, 238, 235),
        );
    }
}

fn draw_sensor_player(cell: Rect) {
    let center = vec2(cell.x + cell.w * 0.5, cell.y + cell.h * 0.5);
    let radius = (cell.w * 0.72).clamp(4.0, 5.8);
    let color = Color::from_rgba(109, 243, 218, 255);
    draw_triangle(
        vec2(center.x, center.y - radius),
        vec2(center.x + radius * 0.72, center.y + radius * 0.72),
        vec2(center.x - radius * 0.72, center.y + radius * 0.72),
        color,
    );
    draw_circle(
        center.x,
        center.y + radius * 0.15,
        1.2,
        Color::from_rgba(2, 8, 12, 255),
    );
}

fn draw_sensor_corners(area: Rect, color: Color) {
    let arm = 11.0_f32.min(area.w * 0.12).min(area.h * 0.12);
    for (x, y, x_direction, y_direction) in [
        (area.x + 2.0, area.y + 2.0, 1.0, 1.0),
        (area.right() - 2.0, area.y + 2.0, -1.0, 1.0),
        (area.x + 2.0, area.bottom() - 2.0, 1.0, -1.0),
        (area.right() - 2.0, area.bottom() - 2.0, -1.0, -1.0),
    ] {
        draw_line(x, y, x + arm * x_direction, y, 2.0, color);
        draw_line(x, y, x, y + arm * y_direction, 2.0, color);
    }
}

fn draw_observation_fields(rect: Rect, fields: &[ActorObservationField], position: GridPos) {
    for field in fields
        .iter()
        .filter(|field| field.contains(position) || field.origin() == position)
    {
        let base = Color::from_rgba(55, 208, 174, 255);
        if field.contains(position) {
            let dx = (position.x - field.origin().x) as f32;
            let dy = (position.y - field.origin().y) as f32;
            let distance = (dx * dx + dy * dy).sqrt();
            let ratio = (distance / f32::from(field.radius().max(1))).clamp(0.0, 1.0);
            let fill_alpha = 0.19 - ratio * 0.09;
            draw_rectangle(
                rect.x + 0.5,
                rect.y + 0.5,
                rect.w - 1.0,
                rect.h - 1.0,
                Color::new(base.r, base.g, base.b, fill_alpha),
            );

            let line = (rect.w * 0.055).clamp(0.8, 1.6);
            let [north, east, south, west] = position.cardinal_neighbors();
            let boundary = Color::new(base.r, base.g, base.b, 0.78);
            if !field.contains(north) {
                draw_line(rect.x, rect.y, rect.x + rect.w, rect.y, line, boundary);
            }
            if !field.contains(east) {
                draw_line(
                    rect.x + rect.w,
                    rect.y,
                    rect.x + rect.w,
                    rect.y + rect.h,
                    line,
                    boundary,
                );
            }
            if !field.contains(south) {
                draw_line(
                    rect.x,
                    rect.y + rect.h,
                    rect.x + rect.w,
                    rect.y + rect.h,
                    line,
                    boundary,
                );
            }
            if !field.contains(west) {
                draw_line(rect.x, rect.y, rect.x, rect.y + rect.h, line, boundary);
            }
        }

        if field.origin() == position {
            let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.5);
            let radius = (rect.w * 0.2).clamp(2.0, 4.5);
            draw_circle(center.x, center.y, radius, Color::from_rgba(4, 10, 15, 220));
            draw_circle_lines(
                center.x,
                center.y,
                radius,
                (rect.w * 0.07).clamp(1.0, 1.8),
                Color::new(base.r, base.g, base.b, 0.95),
            );
            draw_circle(
                center.x,
                center.y,
                (radius * 0.32).max(1.0),
                Color::new(base.r, base.g, base.b, 0.95),
            );
        }
    }
}

fn remembered_bounds(
    remembered: &BTreeMap<GridPos, KnownTile>,
    player: Option<GridPos>,
    map_width: usize,
    map_height: usize,
) -> (GridPos, GridPos) {
    let fallback = player.unwrap_or(GridPos::new(0, 0));
    let mut minimum = fallback;
    let mut maximum = fallback;
    for position in remembered.keys().copied().chain(player) {
        minimum.x = minimum.x.min(position.x);
        minimum.y = minimum.y.min(position.y);
        maximum.x = maximum.x.max(position.x);
        maximum.y = maximum.y.max(position.y);
    }
    let maximum_x = i32::try_from(map_width.saturating_sub(1)).unwrap_or(i32::MAX);
    let maximum_y = i32::try_from(map_height.saturating_sub(1)).unwrap_or(i32::MAX);
    (
        GridPos::new(
            minimum.x.saturating_sub(2).max(0),
            minimum.y.saturating_sub(2).max(0),
        ),
        GridPos::new(
            maximum.x.saturating_add(2).min(maximum_x),
            maximum.y.saturating_add(2).min(maximum_y),
        ),
    )
}

fn draw_hud_progress(rect: Rect, value: u16, maximum: u16, color: Color) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(19, 39, 48, 255),
    );
    let ratio = if maximum == 0 {
        0.0
    } else {
        f32::from(value).clamp(0.0, f32::from(maximum)) / f32::from(maximum)
    };
    draw_rectangle(rect.x, rect.y, rect.w * ratio, rect.h, color);
}

pub(crate) fn heavy_fauna_state_label(actor: &project_rl::entity::Actor) -> Option<String> {
    if actor.ai()?.behavior != project_rl::ai::AiBehavior::TelegraphedBiter {
        return None;
    }
    if let Some(remaining) = actor.recovery_remaining() {
        Some(format!("Récupère · immobile ({} UT)", remaining.get()))
    } else if matches!(actor.ai_state(), AiState::Aiming { .. }) {
        Some("Morsure imminente".to_owned())
    } else {
        None
    }
}

fn ai_state_label(state: AiState) -> String {
    match state {
        AiState::Listening { .. } => "RECHERCHE D'UN BRUIT".to_owned(),
        AiState::Sheltered { .. } => "REPLIÉ · CARAPACE RENFORCÉE".to_owned(),
        AiState::Fleeing => "FUIT LE CONTACT".to_owned(),
        AiState::Cornered => "ACCULÉ · PEUT MORDRE".to_owned(),
        AiState::SupportStock { .. } => "SOUTIEN".to_owned(),
        AiState::Aiming { .. } => "TIR IMMINENT · QUITTER LA CASE VISÉE".to_owned(),
        AiState::Unaware => "NON ALERTÉ".to_owned(),
        AiState::Pursuing {
            remaining_turns, ..
        } => format!("POURSUITE · {remaining_turns}T"),
        AiState::Searching {
            remaining_turns, ..
        } => format!("RECHERCHE LA DERNIÈRE POSITION · {remaining_turns}T"),
        AiState::Responding {
            remaining_turns, ..
        } => format!("RÉPOND À UNE ALARME · {remaining_turns}T"),
        AiState::Returning => "RETOUR AU TERRITOIRE".to_owned(),
        AiState::Cooldown { remaining_turns } => {
            format!("RÉENGAGEMENT BLOQUÉ · {remaining_turns}T")
        }
    }
}

fn legend_panel(bounds: Rect) -> Rect {
    Rect::new(
        bounds.x + (bounds.w - (bounds.w - 24.0).min(980.0)) * 0.5,
        bounds.y + (bounds.h - (bounds.h - 24.0).min(680.0)) * 0.5,
        (bounds.w - 24.0).min(980.0),
        (bounds.h - 24.0).min(680.0),
    )
}

fn draw_legend_overlay(game: &GameState, bounds: Rect, legend_label: &str) {
    let bright = Color::from_rgba(218, 234, 232, 255);
    let muted = Color::from_rgba(146, 174, 179, 255);
    let cyan = Color::from_rgba(100, 221, 201, 255);
    let panel = legend_panel(bounds);
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::from_rgba(5, 12, 18, 248),
    );
    draw_rectangle_lines(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        2.0,
        Color::from_rgba(73, 139, 145, 255),
    );
    draw_ui_text_bold(
        "LÉGENDE · TERMINAL À GLYPHES",
        panel.x + 22.0,
        panel.y + 38.0,
        25.0,
        bright,
    );

    let columns = [
        panel.x + 22.0,
        panel.x + panel.w / 3.0,
        panel.x + panel.w * 2.0 / 3.0,
    ];
    let column_width = panel.w / 3.0;
    draw_ui_text_bold("ENTITÉS ET EFFETS", columns[0], panel.y + 68.0, 14.0, cyan);
    draw_ui_text_bold("DÉCORS", columns[1], panel.y + 68.0, 14.0, cyan);
    draw_ui_text_bold("DÉCORS ET SYSTÈMES", columns[2], panel.y + 68.0, 14.0, cyan);
    let top = panel.y + 91.0;
    let entity_legend = [
        ('@', "Vous", cyan, false, None),
        (
            'c',
            "Récupérateur neutre",
            Color::from_rgba(112, 207, 190, 255),
            false,
            None,
        ),
        (
            'm',
            "Technicien neutre",
            Color::from_rgba(112, 207, 190, 255),
            false,
            None,
        ),
        (
            'h',
            "Soigneur de la clinique",
            Color::from_rgba(112, 207, 190, 255),
            false,
            None,
        ),
        (
            'v',
            "Marchande de la place",
            Color::from_rgba(112, 207, 190, 255),
            false,
            None,
        ),
        (
            'i',
            "Habitant du quartier",
            Color::from_rgba(112, 207, 190, 255),
            false,
            None,
        ),
        (
            'u',
            "Drone allié · unité physique",
            Color::from_rgba(105, 205, 238, 255),
            false,
            None,
        ),
        (
            'b',
            "Balise de saturation · unité physique",
            Color::from_rgba(112, 216, 226, 255),
            false,
            None,
        ),
        (
            'd',
            "Traqueur hostile",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            't',
            "Sentinelle hostile",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'r',
            "Tirailleur hostile",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'j',
            "Artilleur · tir annoncé sur une case",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'w',
            "Soigneur de terrain · soutien allié au contact",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'o',
            "Conteneur instable · explosion et feu",
            Color::from_rgba(241, 177, 72, 255),
            false,
            None,
        ),
        (
            'B',
            "Mordeur · charognard · meute",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'K',
            "Brise-os · prépare, mord, récupère",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'V',
            "Fouisseur · suit les bruits",
            Color::from_rgba(244, 132, 113, 255),
            false,
            None,
        ),
        (
            'G',
            "Herbivore à carapace · neutre",
            Color::from_rgba(161, 204, 137, 255),
            false,
            None,
        ),
        (
            'G',
            "Petit bouclier · replié, carapace renforcée",
            Color::from_rgba(161, 204, 137, 255),
            false,
            Some(TerminalStatusIcon::Sheltered),
        ),
        (
            'N',
            "Grignoteur · fuit, mord si acculé",
            Color::from_rgba(161, 204, 137, 255),
            false,
            None,
        ),
        (
            'q',
            "Relais conducteur · décharge amplifiée par l'eau",
            Color::from_rgba(89, 221, 237, 255),
            false,
            None,
        ),
        (
            'd',
            "Icône flamme · en feu",
            Color::from_rgba(255, 137, 48, 255),
            false,
            Some(TerminalStatusIcon::Burning),
        ),
        (
            '^',
            "Feu au sol · dégâts",
            Color::from_rgba(255, 151, 46, 255),
            false,
            None,
        ),
        (
            'z',
            "Sol électrifié · dégâts",
            Color::from_rgba(89, 221, 237, 255),
            false,
            None,
        ),
        (
            ')',
            "Arme au sol",
            Color::from_rgba(239, 200, 111, 255),
            false,
            None,
        ),
        (
            '!',
            "Consommable",
            Color::from_rgba(127, 211, 157, 255),
            false,
            None,
        ),
        (
            '=',
            "Matériau au sol",
            Color::from_rgba(118, 202, 207, 255),
            false,
            None,
        ),
        (
            '=',
            "Badge + · matériau requis par une intervention locale",
            Color::from_rgba(118, 202, 207, 255),
            false,
            Some(TerminalStatusIcon::RequiredMaterial),
        ),
        (
            'i',
            "! au-dessus · quête disponible",
            Color::from_rgba(112, 207, 190, 255),
            false,
            Some(TerminalStatusIcon::QuestAvailable),
        ),
        (
            'i',
            "? au-dessus · quête à rendre",
            Color::from_rgba(112, 207, 190, 255),
            false,
            Some(TerminalStatusIcon::QuestReady),
        ),
        (
            'i',
            "… au-dessus · quête en cours",
            Color::from_rgba(112, 207, 190, 255),
            false,
            Some(TerminalStatusIcon::QuestInProgress),
        ),
        (
            '=',
            "* au-dessus · élément recherché pour une quête",
            Color::from_rgba(118, 202, 207, 255),
            false,
            Some(TerminalStatusIcon::QuestObjective),
        ),
        (
            '¤',
            "Dispositif explosif identifié",
            Color::from_rgba(255, 185, 82, 255),
            false,
            None,
        ),
        (
            '♪',
            "Leurre sonore actif",
            Color::from_rgba(143, 211, 232, 255),
            false,
            None,
        ),
        (
            'c',
            "Badge ! · état de sécurité",
            Color::from_rgba(255, 175, 83, 255),
            true,
            None,
        ),
        (
            'S',
            "Bandeau · signal de navigation",
            Color::from_rgba(142, 234, 215, 255),
            false,
            None,
        ),
    ];
    let available = (panel.h - 140.0).max(120.0);
    // This is the longest column. Its actual length reserves the footer even
    // when new creatures or status markers are added.
    let row_height = (available / entity_legend.len() as f32).min(38.0);
    let icon_size = (row_height - 2.0).clamp(11.0, 20.0);
    let legend_font_size = row_height.floor().clamp(11.0, 15.0) as u16;
    for (index, (symbol, label, color, alerted, status_icon)) in
        entity_legend.into_iter().enumerate()
    {
        let y = top + index as f32 * row_height;
        let (accent_color, highlight_color) = if symbol == '^' {
            (
                Some(Color::from_rgba(246, 108, 22, 230)),
                Some(Color::from_rgba(255, 205, 68, 250)),
            )
        } else {
            (None, None)
        };
        draw_entity(
            Rect::new(columns[0], y - icon_size * 0.75, icon_size, icon_size),
            symbol,
            TerminalGlyphPalette::new(color, accent_color, highlight_color),
            false,
            alerted,
            status_icon,
        );
        draw_bounded_text(
            label,
            columns[0] + icon_size + 9.0,
            y,
            column_width - icon_size - 30.0,
            legend_font_size,
            muted,
        );
    }
    let terrain_legend = [
        (Decor::Wall, "Cloison"),
        (Decor::Grass, "Prairie"),
        (Decor::Scrub, "Broussailles"),
        (Decor::Mud, "Sol humide"),
        (Decor::ShallowWater, "Eau peu profonde"),
        (Decor::DeepWater, "Eau profonde"),
        (Decor::Tree, "Arbre"),
        (Decor::Boulder, "Bloc rocheux"),
        (Decor::RuinWall, "Ruines de surface"),
        (Decor::ForeignFloor, "Substrat étranger"),
        (Decor::VeinedFloor, "Veine minérale"),
        (Decor::MembraneWall, "Masse étrangère"),
        (Decor::Resonator, "Résonateur étranger"),
        (Decor::GrowthNode, "Nœud minéral"),
        (Decor::ChitinFloor, "Plaque chitineuse"),
        (Decor::PulseChannel, "Canal pulsatile"),
        (Decor::VoidWall, "Masse creuse"),
        (Decor::EyeNode, "Œil dormant"),
        (Decor::RootMass, "Racine calcifiée"),
        (Decor::MemoryFloor, "Mémoire stable"),
        (Decor::WindowFrame, "Cadre d'interface brisé"),
        (Decor::FaultTrace, "Erreur d'exécution"),
        (Decor::DeadScreen, "Écran mort"),
        (Decor::Barricade, "Ancien accès barricadé"),
        (Decor::KernelFault, "Faute noyau"),
        (Decor::OrphanProcess, "Processus orphelin"),
        (Decor::SupplyCache, "Cache de récupération"),
        (Decor::ThreatCamp, "Camp hostile actif"),
        (Decor::ThreatCampDisabled, "Camp neutralisé"),
        (Decor::DoorClosed, "Porte fermée"),
        (Decor::DoorLocked, "Porte verrouillée"),
        (Decor::DoorUnpowered, "Porte sans alimentation"),
        (Decor::ControlReady, "Console active"),
        (Decor::ClinicBed, "Lit de soin"),
        (Decor::ClinicCounter, "Comptoir médical"),
        (Decor::Depot, "Dépôt de maintenance"),
        (Decor::RelayOffline, "Relais en panne"),
        (Decor::SensorOffline, "Capteur hors ligne"),
        (Decor::DataTerminalOnline, "Terminal de données"),
        (Decor::DataTerminalUpdated, "Terminal · registre mis à jour"),
        (Decor::DataTerminalOffline, "Terminal hors ligne"),
        (Decor::DirectionBoard, "Panneau d'orientation"),
        (
            Decor::DirectionBoardUpdated,
            "Panneau · indication corrigée",
        ),
        (Decor::ServicePlan, "Plan du relais"),
        (
            Decor::Passage,
            if game.exit().is_some() {
                "Sortie"
            } else {
                "Passage interzone"
            },
        ),
        (Decor::Ascent, "Montée inter-couche"),
        (Decor::Descent, "Descente inter-couche"),
    ];
    let terrain_rows = terrain_legend.len().div_ceil(2);
    for (index, (decor, label)) in terrain_legend.into_iter().enumerate() {
        let column = 1 + index / terrain_rows;
        let row = index % terrain_rows;
        let y = top + row as f32 * row_height;
        draw_tile(
            Rect::new(columns[column], y - icon_size * 0.75, icon_size, icon_size),
            decor,
            [false; 4],
            true,
            GridPos::new(0, 0),
        );
        draw_bounded_text(
            label,
            columns[column] + icon_size + 9.0,
            y,
            column_width - icon_size - 30.0,
            legend_font_size,
            muted,
        );
    }
    for (offset, (label, cursor)) in [
        ("Zone d'attaque prévisualisée", false),
        ("Curseur de validation", true),
    ]
    .into_iter()
    .enumerate()
    {
        let y = top + (terrain_rows + offset) as f32 * row_height;
        let rect = Rect::new(columns[2], y - icon_size * 0.75, icon_size, icon_size);
        draw_attack_preview_area(rect, true, 0);
        if cursor {
            draw_attack_preview_cursor(rect, true);
        }
        draw_bounded_text(
            label,
            columns[2] + icon_size + 9.0,
            y,
            column_width - icon_size - 30.0,
            legend_font_size,
            muted,
        );
    }
    draw_bounded_text(
        "Minimap : + interaction · cercle objet · × accès bloqué · !/?/…/* quête visible",
        panel.x + 22.0,
        panel.y + panel.h - 37.0,
        panel.w - 44.0,
        14,
        cyan,
    );
    draw_ui_text(
        format!("{legend_label}, Échap ou clic gauche · fermer"),
        panel.x + 22.0,
        panel.y + panel.h - 16.0,
        15.0,
        muted,
    );
}

fn draw_wrapped_lines(
    text: &str,
    x: f32,
    mut y: f32,
    width: f32,
    size: u16,
    maximum_lines: usize,
    color: Color,
) -> f32 {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in text.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure_ui_text(&candidate, None, size, 1.0).width > width {
            lines.push(std::mem::take(&mut line));
            if lines.len() == maximum_lines {
                break;
            }
            line.push_str(word);
        } else {
            line = candidate;
        }
    }
    if lines.len() < maximum_lines && !line.is_empty() {
        lines.push(line);
    }
    for line in lines.into_iter().take(maximum_lines) {
        draw_bounded_text(&line, x, y, width, size, color);
        y += f32::from(size) + 4.0;
    }
    y
}

fn draw_bounded_text(text: &str, x: f32, y: f32, width: f32, size: u16, color: Color) {
    if measure_ui_text(text, None, size, 1.0).width <= width {
        draw_ui_text(text, x, y, f32::from(size), color);
        return;
    }
    let mut clipped = String::new();
    let suffix = "…";
    for character in text.chars() {
        let previous = clipped.len();
        clipped.push(character);
        if measure_ui_text(&clipped, None, size, 1.0).width
            + measure_ui_text(suffix, None, size, 1.0).width
            > width
        {
            clipped.truncate(previous);
            break;
        }
    }
    clipped.push_str(suffix);
    draw_ui_text(&clipped, x, y, f32::from(size), color);
}

pub fn overlay_label(symbol: char) -> &'static str {
    match symbol {
        'X' => "Mannequin d'essai · passif, sans riposte",
        'L' => "Mannequin lourd · résiste à Percussion",
        'A' => "Mannequin ancré · inamovible",
        'Y' => "Dispositif d'essai · frappe à 1 case : 4 dégâts électriques",
        '@' => "Vous · noyau mobile",
        'c' => "Récupérateur neutre · transporte des matériaux",
        'm' => "Technicien neutre · entretient les installations",
        'h' => "Soigneur neutre · soins accessibles à la clinique",
        'v' => "Marchande neutre · achat, vente et paris",
        'i' => "Habitant neutre · routine locale et conversation",
        'u' => "Drone allié · unité physique sur sa propre case",
        'b' => "Balise de saturation · unité physique sur sa propre case",
        'd' => "Traqueur · hostile",
        't' => "Sentinelle · hostile",
        'r' => "Tirailleur · hostile",
        'j' => "Artilleur humanoïde · annonce son tir ; quitter la case marquée",
        'w' => "Soigneur humanoïde · aide ses alliés blessés au contact",
        'B' => "Mordeur des friches · charognard · chasse en meute",
        'K' => "Brise-os · morsure annoncée puis récupération immobile",
        'V' => "Fouisseur vibrant · fouisseur · vision courte, suit les bruits",
        'G' => "Herbivore à carapace · neutre, se protège après un coup",
        'N' => "Grignoteur de gravats · neutre, fuit et mord seulement si acculé",
        'o' => "Conteneur instable · explosion et feu persistant",
        'q' => "Relais conducteur · décharge électrique amplifiée par l'eau",
        ')' => "Arme au sol",
        '!' => "Consommable au sol",
        '=' => "Matériau au sol",
        '¤' => "Dispositif explosif identifié",
        '♪' => "Leurre sonore actif",
        '>' => "Sortie du secteur",
        '.' | '-' | '*' | '+' | 'f' | 'F' | 'x' | '~' | 'a' => "Effet visuel en cours",
        '^' => "Feu au sol · dégâts thermiques persistants",
        'z' => "Sol électrifié · dégâts électriques persistants",
        's' => "Capteur de sécurité",
        _ => "Trace de déplacement",
    }
}

/// A field no longer needs a fake occupant glyph to remain inspectable.
fn ground_effect_description(
    ground: &project_rl::effects::GroundEffectMap,
    at: GridPos,
) -> Option<String> {
    use project_rl::combat::DamageType;
    let labels: Vec<_> = ground
        .at(at)
        .map(|field| {
            let name = match field.damage_each_turn().damage_type {
                DamageType::Thermal => "Flammes au sol",
                DamageType::Chemical => "Flaque caustique",
                DamageType::Electrical => "Sol électrifié",
                _ => "Champ dangereux",
            };
            format!("{name} · {}t", field.remaining_turns())
        })
        .collect();
    (!labels.is_empty()).then(|| labels.join(" / "))
}

pub(crate) fn detected_navigation_signal_summary(game: &WorldState) -> Option<String> {
    let observer = game.player_position()?;
    let signals = game
        .active_facility()?
        .detected_navigation_signals(observer);
    let nearest = signals.first()?;
    Some(navigation_signal_summary(
        observer,
        nearest.position,
        nearest.distance,
        signals.len(),
    ))
}

pub(crate) fn navigation_signal_summary(
    observer: GridPos,
    target: GridPos,
    distance: u32,
    signal_count: usize,
) -> String {
    directional_signal_summary("SIGNAL DE SITE", observer, target, distance, signal_count)
}

pub(crate) fn directional_signal_summary(
    label: &str,
    observer: GridPos,
    target: GridPos,
    distance: u32,
    signal_count: usize,
) -> String {
    let distance_label = match distance {
        0..=3 => "SUR PLACE",
        4..=12 => "TOUT PROCHE",
        13..=28 => "PROCHE",
        29..=52 => "À DISTANCE",
        _ => "LOINTAIN",
    };
    let mut summary = if distance <= 3 {
        format!("{label} · {distance_label}")
    } else {
        format!(
            "{label} · {} · {distance_label}",
            approximate_direction(observer, target)
        )
    };
    if signal_count > 1 {
        summary.push_str(&format!(" · +{} AUTRE(S)", signal_count - 1));
    }
    summary
}

pub(crate) fn approximate_direction(observer: GridPos, target: GridPos) -> &'static str {
    let horizontal = observer.x.abs_diff(target.x);
    let vertical = observer.y.abs_diff(target.y);
    if horizontal > vertical.saturating_mul(2) {
        return if target.x > observer.x {
            "EST"
        } else {
            "OUEST"
        };
    }
    if vertical > horizontal.saturating_mul(2) {
        return if target.y > observer.y { "SUD" } else { "NORD" };
    }
    match (target.x > observer.x, target.y > observer.y) {
        (true, true) => "SUD-EST",
        (true, false) => "NORD-EST",
        (false, true) => "SUD-OUEST",
        (false, false) => "NORD-OUEST",
    }
}

/// Whole square cells only, centred on the player. Neither GUI scale nor window
/// size changes perception, and off-map/unknown cells simply remain empty.
pub struct GridCamera {
    pub first: GridPos,
    pub columns: i32,
    pub rows: i32,
    origin: Vec2,
    cell: f32,
}

fn fitted_cell_size(bounds: Rect, preferred: u16, radius: u16) -> f32 {
    let diameter = f32::from(radius) * 2.0 + 1.0;
    // Fit the entire normal sensor footprint on small windows. Large modded
    // ranges may exceed the 8px legibility floor; perception itself is unchanged.
    f32::from(preferred)
        .min(bounds.w / diameter)
        .min(bounds.h / diameter)
        .floor()
        .max(8.0)
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TerminalUiLayout {
    header: Rect,
    route: Option<Rect>,
    map: Rect,
    footer: Rect,
    sidebar: Option<Rect>,
}

pub fn terminal_status_panel(bounds: Rect, ui_scale: f32) -> Option<Rect> {
    // The world uses screen pixels, but the status text uses logical UI units.
    // Reserve its scaled width in the shared layout (including pointer hits).
    let width = 260.0 * ui_scale;
    let height = bounds.h - 16.0;
    (bounds.w >= 1120.0 && bounds.w - width >= 780.0 && height >= 480.0 * ui_scale)
        .then(|| Rect::new(bounds.x + 8.0, bounds.y + 8.0, width, height))
}

fn terminal_ui_layout(
    bounds: Rect,
    ui_scale: f32,
    navigation_signal_visible: bool,
) -> TerminalUiLayout {
    let status_sidebar = terminal_status_panel(bounds, ui_scale);
    let sidebar = status_sidebar.map(|_| {
        Rect::new(
            bounds.x + bounds.w - 336.0,
            bounds.y + 8.0,
            320.0,
            bounds.h - 16.0,
        )
    });
    let main_right = sidebar.map_or(bounds.x + bounds.w - 8.0, |panel| panel.x - 12.0);
    let main_x = status_sidebar.map_or(bounds.x + 8.0, |panel| panel.x + panel.w + 12.0);
    let main_width = (main_right - main_x).max(80.0);
    let header = Rect::new(main_x, bounds.y + 5.0, main_width, 30.0);
    let route =
        navigation_signal_visible.then(|| Rect::new(main_x, bounds.y + 40.0, main_width, 28.0));
    let map_top = route.map_or(bounds.y + 42.0, |route| route.y + route.h + 7.0);
    let footer = Rect::new(
        main_x + 6.0,
        bounds.y + bounds.h - 29.0,
        main_width - 12.0,
        23.0,
    );
    let map = Rect::new(
        main_x,
        map_top,
        main_width,
        (footer.y - map_top - 5.0).max(40.0),
    );
    TerminalUiLayout {
        header,
        route,
        map,
        footer,
        sidebar,
    }
}

fn terminal_grid_camera(
    game: &WorldState,
    bounds: Rect,
    ui_scale: f32,
    preferred_cell: u16,
    navigation_signal_visible: bool,
) -> (bool, GridCamera) {
    let layout = terminal_ui_layout(bounds, ui_scale, navigation_signal_visible);
    let focus = game.player_position().unwrap_or(GridPos::new(12, 12));
    let effective_cell = fitted_cell_size(
        layout.map,
        preferred_cell,
        game.rules().player_field_of_view.radius,
    );
    (
        layout.sidebar.is_some(),
        GridCamera::new(layout.map, effective_cell, focus),
    )
}

impl GridCamera {
    pub fn new(bounds: Rect, cell: f32, focus: GridPos) -> Self {
        let columns = (bounds.w / cell).floor().max(1.0) as i32;
        let rows = (bounds.h / cell).floor().max(1.0) as i32;
        Self {
            first: GridPos::new(focus.x - columns / 2, focus.y - rows / 2),
            columns,
            rows,
            origin: vec2(
                (bounds.x + (bounds.w - columns as f32 * cell) * 0.5).floor(),
                (bounds.y + (bounds.h - rows as f32 * cell) * 0.5).floor(),
            ),
            cell,
        }
    }

    pub fn rect(&self, position: GridPos) -> Rect {
        Rect::new(
            self.origin.x + (position.x - self.first.x) as f32 * self.cell,
            self.origin.y + (position.y - self.first.y) as f32 * self.cell,
            self.cell,
            self.cell,
        )
    }

    fn contains(&self, position: GridPos) -> bool {
        position.x >= self.first.x
            && position.y >= self.first.y
            && position.x < self.first.x + self.columns
            && position.y < self.first.y + self.rows
    }

    pub fn hit(&self, (x, y): (f32, f32)) -> Option<GridPos> {
        let x = (x - self.origin.x) / self.cell;
        let y = (y - self.origin.y) / self.cell;
        (x >= 0.0 && y >= 0.0 && x < self.columns as f32 && y < self.rows as f32).then_some(
            GridPos::new(
                self.first.x + x.floor() as i32,
                self.first.y + y.floor() as i32,
            ),
        )
    }
}

fn dim(color: Color, visible: bool) -> Color {
    if visible {
        color
    } else {
        Color::new(color.r * 0.32, color.g * 0.36, color.b * 0.43, 1.0)
    }
}

fn draw_tile(rect: Rect, kind: Decor, joins: [bool; 4], visible: bool, position: GridPos) {
    let background = match kind {
        Decor::Grate => Color::from_rgba(20, 34, 39, 255),
        Decor::Gravel => Color::from_rgba(27, 31, 30, 255),
        Decor::Grass => Color::from_rgba(24, 44, 33, 255),
        Decor::Scrub => Color::from_rgba(47, 45, 29, 255),
        Decor::Mud => Color::from_rgba(48, 37, 31, 255),
        Decor::ShallowWater => Color::from_rgba(20, 54, 66, 255),
        Decor::DeepWater => Color::from_rgba(12, 34, 54, 255),
        Decor::RuinFloor => Color::from_rgba(43, 42, 39, 255),
        Decor::Tree => Color::from_rgba(24, 48, 35, 255),
        Decor::Boulder => Color::from_rgba(50, 53, 50, 255),
        Decor::RuinWall | Decor::Barricade => Color::from_rgba(55, 53, 49, 255),
        Decor::Lane | Decor::Threshold => Color::from_rgba(34, 39, 37, 255),
        Decor::ForeignFloor => Color::from_rgba(25, 21, 43, 255),
        Decor::VeinedFloor => Color::from_rgba(24, 39, 48, 255),
        Decor::MembraneWall => Color::from_rgba(48, 34, 66, 255),
        Decor::ChitinFloor => Color::from_rgba(48, 39, 27, 255),
        Decor::PulseChannel => Color::from_rgba(57, 22, 30, 255),
        Decor::VoidWall => Color::from_rgba(29, 18, 21, 255),
        Decor::MemoryFloor => Color::from_rgba(8, 12, 13, 255),
        Decor::WindowFrame => Color::from_rgba(10, 15, 16, 255),
        Decor::FaultTrace => Color::from_rgba(19, 8, 9, 255),
        Decor::DeadScreen => Color::from_rgba(2, 3, 4, 255),
        _ if kind.blocks() => Color::from_rgba(42, 61, 71, 255),
        _ => Color::from_rgba(18, 31, 38, 255),
    };
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, dim(background, visible));
    if kind == Decor::Passage {
        draw_pixel_glyph(
            rect,
            &[
                "########", "#......#", "#.##...#", "#..##..#", "#...##.#", "#....#.#", "#......#",
                "########",
            ],
            dim(Color::from_rgba(255, 215, 120, 255), visible),
        );
        return;
    }
    if matches!(kind, Decor::Ascent | Decor::Descent) {
        let pattern = if kind == Decor::Ascent {
            &[
                "...##...", "..####..", ".##..##.", "...##...", "...##...", ".######.", ".#....#.",
                ".######.",
            ]
        } else {
            &[
                ".######.", ".#....#.", ".######.", "...##...", "...##...", ".##..##.", "..####..",
                "...##...",
            ]
        };
        draw_pixel_glyph(
            rect,
            pattern,
            dim(Color::from_rgba(113, 220, 207, 255), visible),
        );
        return;
    }
    if matches!(kind, Decor::ShallowWater | Decor::DeepWater) {
        let primary = if kind == Decor::ShallowWater {
            Color::from_rgba(84, 174, 187, 255)
        } else {
            Color::from_rgba(65, 126, 169, 255)
        };
        let highlight = if kind == Decor::ShallowWater {
            Color::from_rgba(151, 219, 207, 255)
        } else {
            Color::from_rgba(92, 169, 194, 255)
        };
        for row in 0..3 {
            let offset = (position.x + position.y + row).rem_euclid(3) as f32;
            draw_rectangle(
                rect.x + 3.0 + offset * 2.0,
                rect.y + 5.0 + row as f32 * (rect.h - 10.0) / 2.0,
                (rect.w * 0.42).max(4.0),
                1.5,
                dim(if row == 1 { highlight } else { primary }, visible),
            );
        }
        return;
    }
    if kind == Decor::Tree {
        draw_pixel_glyph(
            rect,
            &TREE_CANOPY,
            dim(Color::from_rgba(86, 166, 103, 255), visible),
        );
        draw_rectangle(
            rect.x + rect.w * 0.44,
            rect.y + rect.h * 0.57,
            (rect.w * 0.12).max(2.0),
            rect.h * 0.28,
            dim(Color::from_rgba(156, 113, 72, 255), visible),
        );
        return;
    }
    if matches!(
        kind,
        Decor::DoorClosed
            | Decor::DoorOpen
            | Decor::DoorLocked
            | Decor::DoorUnpowered
            | Decor::ControlReady
            | Decor::ControlUsed
    ) {
        let (pattern, color) = match kind {
            Decor::DoorOpen => (&OPEN_DOOR, Color::from_rgba(122, 197, 160, 255)),
            Decor::DoorClosed => (&CLOSED_DOOR, Color::from_rgba(215, 190, 128, 255)),
            Decor::DoorLocked => (&LOCKED_DOOR, Color::from_rgba(239, 151, 99, 255)),
            Decor::DoorUnpowered => (&UNPOWERED_DOOR, Color::from_rgba(126, 141, 148, 255)),
            Decor::ControlReady => (&CONTROL_READY, Color::from_rgba(127, 223, 204, 255)),
            _ => (&CONTROL_USED, Color::from_rgba(133, 156, 161, 255)),
        };
        draw_pixel_glyph(rect, pattern, dim(color, visible));
        return;
    }
    if matches!(
        kind,
        Decor::Wall | Decor::MembraneWall | Decor::VoidWall | Decor::DeadScreen
    ) {
        let (edge, inner) = match kind {
            Decor::MembraneWall => (
                dim(Color::from_rgba(169, 111, 194, 255), visible),
                dim(Color::from_rgba(82, 51, 105, 255), visible),
            ),
            Decor::VoidWall => (
                dim(Color::from_rgba(181, 139, 91, 255), visible),
                dim(Color::from_rgba(72, 49, 37, 255), visible),
            ),
            Decor::DeadScreen => (
                dim(Color::from_rgba(126, 145, 142, 255), visible),
                dim(Color::from_rgba(3, 5, 6, 255), visible),
            ),
            _ => (
                dim(Color::from_rgba(128, 165, 174, 255), visible),
                dim(Color::from_rgba(61, 88, 98, 255), visible),
            ),
        };
        let [n, e, s, w] = joins;
        draw_rectangle(
            rect.x + 3.0,
            rect.y + 3.0,
            rect.w - 6.0,
            rect.h - 6.0,
            inner,
        );
        if !n {
            draw_rectangle(rect.x, rect.y, rect.w, 2.0, edge);
        }
        if !s {
            draw_rectangle(rect.x, rect.y + rect.h - 2.0, rect.w, 2.0, edge);
        }
        if !w {
            draw_rectangle(rect.x, rect.y, 2.0, rect.h, edge);
        }
        if !e {
            draw_rectangle(rect.x + rect.w - 2.0, rect.y, 2.0, rect.h, edge);
        }
        draw_rectangle(rect.x + 6.0, rect.y + 6.0, 2.0, 2.0, edge);
        return;
    }
    if !kind.blocks() {
        let seam = dim(Color::from_rgba(35, 52, 61, 255), visible);
        if !matches!(
            kind,
            Decor::MemoryFloor | Decor::WindowFrame | Decor::FaultTrace
        ) {
            draw_rectangle(rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, 1.0, seam);
            draw_rectangle(rect.x + 1.0, rect.y + 1.0, 1.0, rect.h - 2.0, seam);
        }
        match kind {
            Decor::Gravel => {
                let span_x = (rect.w as i32 - 4).max(1);
                let span_y = (rect.h as i32 - 4).max(1);
                for (dx, dy) in [(5, 7), (19, 16), (9, 24)] {
                    let px = (dx + position.x * 7 + position.y * 3).rem_euclid(span_x) + 2;
                    let py = (dy + position.x * 3 + position.y * 7).rem_euclid(span_y) + 2;
                    draw_rectangle(
                        rect.x + px as f32,
                        rect.y + py as f32,
                        2.0,
                        1.0,
                        dim(Color::from_rgba(79, 87, 78, 255), visible),
                    );
                }
            }
            Decor::Grass => {
                let span_x = (rect.w as i32 - 4).max(1);
                let span_y = (rect.h as i32 - 5).max(1);
                for (dx, dy) in [(5, 20), (14, 10), (23, 22)] {
                    let px = (dx + position.x * 5 + position.y * 2).rem_euclid(span_x) + 2;
                    let py = (dy + position.x * 2 + position.y * 5).rem_euclid(span_y) + 3;
                    let color = if (px + py) % 2 == 0 {
                        Color::from_rgba(92, 151, 91, 255)
                    } else {
                        Color::from_rgba(139, 171, 91, 255)
                    };
                    draw_line(
                        rect.x + px as f32,
                        rect.y + py as f32,
                        rect.x + px as f32 + 1.0,
                        rect.y + py as f32 - 3.0,
                        1.0,
                        dim(color, visible),
                    );
                }
            }
            Decor::Scrub => {
                let color = dim(Color::from_rgba(161, 143, 78, 255), visible);
                draw_line(
                    rect.x + rect.w * 0.25,
                    rect.y + rect.h * 0.68,
                    rect.x + rect.w * 0.48,
                    rect.y + rect.h * 0.43,
                    1.5,
                    color,
                );
                draw_line(
                    rect.x + rect.w * 0.48,
                    rect.y + rect.h * 0.43,
                    rect.x + rect.w * 0.74,
                    rect.y + rect.h * 0.67,
                    1.5,
                    color,
                );
            }
            Decor::Mud => {
                let dark = dim(Color::from_rgba(100, 78, 58, 255), visible);
                let wet = dim(Color::from_rgba(82, 117, 105, 255), visible);
                draw_rectangle(
                    rect.x + rect.w * 0.18,
                    rect.y + rect.h * 0.62,
                    rect.w * 0.55,
                    2.0,
                    dark,
                );
                draw_rectangle(
                    rect.x + rect.w * 0.55,
                    rect.y + rect.h * 0.30,
                    rect.w * 0.25,
                    1.0,
                    wet,
                );
            }
            Decor::RuinFloor => {
                let crack = dim(Color::from_rgba(113, 108, 91, 255), visible);
                draw_line(
                    rect.x + rect.w * 0.22,
                    rect.y + rect.h * 0.15,
                    rect.x + rect.w * 0.50,
                    rect.y + rect.h * 0.50,
                    1.0,
                    crack,
                );
                draw_line(
                    rect.x + rect.w * 0.50,
                    rect.y + rect.h * 0.50,
                    rect.x + rect.w * 0.78,
                    rect.y + rect.h * 0.42,
                    1.0,
                    crack,
                );
            }
            Decor::Grate => {
                for step in 1..4 {
                    draw_rectangle(
                        rect.x + 4.0,
                        rect.y + rect.h * step as f32 / 4.0,
                        rect.w - 8.0,
                        1.0,
                        dim(Color::from_rgba(64, 86, 91, 255), visible),
                    );
                }
            }
            Decor::Lane if (position.x + position.y) % 2 == 0 => {
                draw_rectangle(
                    rect.x + rect.w / 2.0 - 3.0,
                    rect.y + rect.h / 2.0,
                    6.0,
                    2.0,
                    dim(Color::from_rgba(121, 116, 81, 255), visible),
                );
            }
            Decor::Threshold => {
                for step in 0..4 {
                    draw_rectangle(
                        rect.x + 2.0 + step as f32 * (rect.w - 4.0) / 4.0,
                        rect.y + 3.0,
                        (rect.w - 4.0) / 8.0,
                        rect.h - 6.0,
                        dim(Color::from_rgba(152, 130, 74, 255), visible),
                    );
                }
            }
            Decor::ForeignFloor => {
                let mineral = dim(Color::from_rgba(112, 83, 145, 255), visible);
                let offset = (position.x * 3 + position.y * 5).rem_euclid(5) as f32;
                draw_line(
                    rect.x + 4.0 + offset,
                    rect.y + rect.h * 0.72,
                    rect.x + rect.w * 0.48,
                    rect.y + 4.0 + offset * 0.4,
                    1.0,
                    mineral,
                );
                draw_rectangle(
                    rect.x + rect.w * 0.62,
                    rect.y + rect.h * 0.33,
                    2.0,
                    2.0,
                    mineral,
                );
            }
            Decor::VeinedFloor => {
                let vein = dim(Color::from_rgba(81, 188, 176, 255), visible);
                let reverse = (position.x + position.y).rem_euclid(2) == 0;
                let (start_x, end_x) = if reverse {
                    (rect.x + 3.0, rect.x + rect.w - 3.0)
                } else {
                    (rect.x + rect.w - 3.0, rect.x + 3.0)
                };
                draw_line(
                    start_x,
                    rect.y + rect.h * 0.68,
                    end_x,
                    rect.y + rect.h * 0.32,
                    1.5,
                    vein,
                );
            }
            Decor::ChitinFloor => {
                let edge = dim(Color::from_rgba(154, 118, 72, 255), visible);
                let offset = (position.x + position.y * 2).rem_euclid(4) as f32;
                draw_line(
                    rect.x + 3.0,
                    rect.y + rect.h * 0.28 + offset,
                    rect.x + rect.w * 0.48,
                    rect.y + rect.h - 3.0,
                    1.0,
                    edge,
                );
                draw_line(
                    rect.x + rect.w * 0.48,
                    rect.y + rect.h - 3.0,
                    rect.x + rect.w - 3.0,
                    rect.y + rect.h * 0.28 + offset,
                    1.0,
                    edge,
                );
            }
            Decor::PulseChannel => {
                let pulse = dim(Color::from_rgba(211, 78, 89, 255), visible);
                let center_y = rect.y + rect.h / 2.0;
                draw_line(
                    rect.x + 2.0,
                    center_y,
                    rect.x + rect.w - 2.0,
                    center_y,
                    1.5,
                    pulse,
                );
                let knot = if (position.x + position.y).rem_euclid(3) == 0 {
                    4.0
                } else {
                    2.0
                };
                draw_rectangle(
                    rect.x + rect.w / 2.0 - knot / 2.0,
                    center_y - knot / 2.0,
                    knot,
                    knot,
                    pulse,
                );
            }
            Decor::MemoryFloor => {
                if (position.x * 7 + position.y * 11).rem_euclid(5) == 0 {
                    let data = dim(Color::from_rgba(116, 145, 139, 255), visible);
                    let y = rect.y + rect.h * 0.68;
                    draw_rectangle(rect.x + 4.0, y, (rect.w * 0.22).max(2.0), 1.0, data);
                    draw_rectangle(
                        rect.x + rect.w * 0.64,
                        rect.y + rect.h * 0.29,
                        1.5,
                        1.5,
                        data,
                    );
                }
            }
            Decor::WindowFrame => {
                let frame = dim(Color::from_rgba(174, 196, 190, 255), visible);
                let cold = dim(Color::from_rgba(78, 172, 167, 255), visible);
                let offset = (position.x * 3 + position.y).rem_euclid(3) as f32;
                draw_line(
                    rect.x + 1.0,
                    rect.y + 3.0 + offset,
                    rect.x + rect.w - 1.0,
                    rect.y + 3.0 + offset,
                    1.0,
                    frame,
                );
                if (position.x + position.y).rem_euclid(4) == 0 {
                    draw_rectangle(
                        rect.x + 3.0,
                        rect.y + rect.h - 5.0,
                        (rect.w * 0.44).max(3.0),
                        1.5,
                        cold,
                    );
                }
            }
            Decor::FaultTrace => {
                let fault = dim(Color::from_rgba(225, 64, 58, 255), visible);
                let y = rect.y
                    + rect.h * (0.34 + 0.16 * (position.x + position.y).rem_euclid(3) as f32);
                draw_line(rect.x + 1.0, y, rect.x + rect.w * 0.34, y, 1.5, fault);
                draw_line(
                    rect.x + rect.w * 0.57,
                    y - 1.0,
                    rect.x + rect.w - 1.0,
                    y - 1.0,
                    1.5,
                    fault,
                );
                draw_rectangle(rect.x + rect.w * 0.45, y - 2.0, 2.0, 3.0, fault);
            }
            Decor::SupplyCache => {
                draw_pixel_glyph(
                    rect,
                    &SUPPLY_CACHE,
                    dim(Color::from_rgba(235, 190, 92, 255), visible),
                );
            }
            Decor::ThreatCamp => {
                draw_pixel_glyph(
                    rect,
                    &THREAT_CAMP,
                    dim(Color::from_rgba(234, 104, 72, 255), visible),
                );
            }
            Decor::ThreatCampDisabled => {
                draw_pixel_glyph(
                    rect,
                    &THREAT_CAMP_DISABLED,
                    dim(Color::from_rgba(116, 137, 145, 255), visible),
                );
            }
            _ => {
                draw_rectangle(rect.x + rect.w - 5.0, rect.y + rect.h - 5.0, 1.0, 1.0, seam);
            }
        }
        return;
    }
    let (pattern, color) = match kind {
        Decor::Pillar => (&PILLAR, Color::from_rgba(156, 176, 181, 255)),
        Decor::Crate => (&CRATE, Color::from_rgba(186, 156, 105, 255)),
        Decor::Server => (&SERVER, Color::from_rgba(107, 201, 190, 255)),
        Decor::Console => (&CONSOLE, Color::from_rgba(130, 185, 207, 255)),
        Decor::Coolant => (&COOLANT, Color::from_rgba(105, 169, 187, 255)),
        Decor::Resonator => (&RESONATOR, Color::from_rgba(185, 111, 219, 255)),
        Decor::GrowthNode => (&GROWTH_NODE, Color::from_rgba(91, 213, 174, 255)),
        Decor::EyeNode => (&EYE_NODE, Color::from_rgba(232, 92, 104, 255)),
        Decor::RootMass => (&ROOT_MASS, Color::from_rgba(194, 151, 84, 255)),
        Decor::KernelFault => (&KERNEL_FAULT, Color::from_rgba(235, 67, 59, 255)),
        Decor::OrphanProcess => (&ORPHAN_PROCESS, Color::from_rgba(190, 89, 151, 255)),
        Decor::ClinicBed => (&CLINIC_BED, Color::from_rgba(130, 204, 184, 255)),
        Decor::ClinicCounter => (&CLINIC_COUNTER, Color::from_rgba(112, 181, 176, 255)),
        Decor::Boulder => (&BOULDER, Color::from_rgba(143, 151, 142, 255)),
        Decor::RuinWall => (&RUIN_WALL, Color::from_rgba(165, 151, 122, 255)),
        Decor::Barricade => (&BARRICADE, Color::from_rgba(153, 132, 114, 255)),
        Decor::Depot => (&DEPOT, Color::from_rgba(200, 166, 105, 255)),
        Decor::RelayOffline => (&RELAY_OFFLINE, Color::from_rgba(180, 111, 92, 255)),
        Decor::RelayOnline => (&RELAY_ONLINE, Color::from_rgba(105, 211, 176, 255)),
        Decor::ActuatorOffline => (&ACTUATOR_OFFLINE, Color::from_rgba(116, 137, 145, 255)),
        Decor::ActuatorOnline => (&ACTUATOR_ONLINE, Color::from_rgba(112, 198, 190, 255)),
        Decor::SensorOffline => (&SENSOR_OFFLINE, Color::from_rgba(116, 137, 145, 255)),
        Decor::SensorOnline => (&SENSOR_ONLINE, Color::from_rgba(104, 207, 194, 255)),
        Decor::DataTerminalOffline => {
            (&DATA_TERMINAL_OFFLINE, Color::from_rgba(116, 137, 145, 255))
        }
        Decor::DataTerminalOnline => (&DATA_TERMINAL_ONLINE, Color::from_rgba(128, 218, 202, 255)),
        Decor::DataTerminalUpdated => {
            (&DATA_TERMINAL_UPDATED, Color::from_rgba(151, 226, 169, 255))
        }
        Decor::DirectionBoard => (&DIRECTION_BOARD, Color::from_rgba(142, 163, 171, 255)),
        Decor::DirectionBoardUpdated => (
            &DIRECTION_BOARD_UPDATED,
            Color::from_rgba(128, 190, 177, 255),
        ),
        Decor::ServicePlan => (&SERVICE_PLAN, Color::from_rgba(125, 185, 184, 255)),
        _ => unreachable!("non-blocking floor and wall already rendered"),
    };
    draw_pixel_glyph(rect, pattern, dim(color, visible));
}

fn attack_preview_color(valid: bool) -> Color {
    if valid {
        Color::from_rgba(255, 174, 64, 255)
    } else {
        Color::from_rgba(255, 91, 73, 255)
    }
}

fn draw_attack_preview_area(rect: Rect, valid: bool, step: u16) {
    let color = attack_preview_color(valid);
    let intensity = (0.16 - f32::from(step.min(8)) * 0.008).max(0.08);
    draw_rectangle(
        rect.x + 2.0,
        rect.y + 2.0,
        rect.w - 4.0,
        rect.h - 4.0,
        Color::new(color.r, color.g, color.b, intensity),
    );
    draw_rectangle_lines(
        rect.x + 1.5,
        rect.y + 1.5,
        rect.w - 3.0,
        rect.h - 3.0,
        1.0,
        Color::new(color.r, color.g, color.b, 0.62),
    );
    let tick = (rect.w * 0.18).clamp(3.0, 7.0);
    for (x, y, dx, dy) in [
        (rect.x + 3.0, rect.y + 3.0, tick, 0.0),
        (rect.x + 3.0, rect.y + 3.0, 0.0, tick),
        (rect.x + rect.w - 3.0, rect.y + rect.h - 3.0, -tick, 0.0),
        (rect.x + rect.w - 3.0, rect.y + rect.h - 3.0, 0.0, -tick),
    ] {
        draw_line(x, y, x + dx, y + dy, 1.0, color);
    }
}

fn draw_attack_preview_cursor(rect: Rect, valid: bool) {
    let color = attack_preview_color(valid);
    draw_rectangle_lines(
        rect.x + 1.5,
        rect.y + 1.5,
        rect.w - 3.0,
        rect.h - 3.0,
        3.0,
        color,
    );
    let tick = (rect.w * 0.18).clamp(3.0, 7.0);
    let cx = rect.x + rect.w / 2.0;
    let cy = rect.y + rect.h / 2.0;
    draw_line(cx - tick, cy, cx + tick, cy, 2.0, color);
    draw_line(cx, cy - tick, cx, cy + tick, 2.0, color);
}

fn draw_entity(
    rect: Rect,
    symbol: char,
    palette: TerminalGlyphPalette,
    selected: bool,
    alerted: bool,
    status_icon: Option<TerminalStatusIcon>,
) {
    if let Some(ascii) = actor_ascii_glyph(symbol) {
        draw_ascii_actor(rect, ascii, palette.primary);
    } else {
        let pattern = match symbol {
            'o' => &VOLATILE_CONTAINER,
            'q' => &SUBMERGED_RELAY,
            'b' => &SATURATION_BEACON,
            ')' => &WEAPON,
            '!' => &REPAIR,
            '=' => &MATERIAL,
            '>' => &EXIT,
            '.' => &EFFECT_DOT,
            '-' => &EFFECT_PROJECTILE,
            '*' => &EFFECT,
            '+' => &EFFECT_BURST,
            'f' => &FLAME_SMALL,
            'F' => &FLAME,
            'x' => &EFFECT_IMPACT,
            '~' => &EFFECT_WAVE,
            'a' => &EFFECT_ALARM,
            '\u{e000}' => &SHOCK_SMALL,
            '\u{e001}' => &SHOCK_WIDE,
            '\u{e002}' => &RING_SMALL,
            '\u{e003}' => &RING_WIDE,
            '\u{e004}' => &ARC_FORK,
            '\u{e005}' => &ARC_SPLIT,
            '\u{e006}' => &ACID_DROP,
            '\u{e007}' => &ACID_SPLASH,
            '\u{e008}' => &ACID_POOL,
            's' => &SENSOR_ONLINE,
            '^' => &FLAME_LARGE,
            'z' => &ELECTRIFIED_FIELD,
            '→' => &TRACE_EAST,
            '↓' => &TRACE_SOUTH,
            '←' => &TRACE_WEST,
            _ => &TRACE,
        };
        draw_pixel_glyph_palette(rect, pattern, palette);
    }
    if let Some(icon) = status_icon {
        draw_status_icon(rect, icon);
    }
    if alerted {
        draw_alert_marker(rect);
    }
    if selected {
        let x = rect.x;
        let y = rect.y;
        let w = rect.w;
        for (dx, dy, sx, sy) in [
            (0.0, 0.0, 1.0, 1.0),
            (w, 0.0, -1.0, 1.0),
            (0.0, w, 1.0, -1.0),
            (w, w, -1.0, -1.0),
        ] {
            draw_line(x + dx, y + dy, x + dx + 7.0 * sx, y + dy, 2.0, YELLOW);
            draw_line(x + dx, y + dy, x + dx, y + dy + 7.0 * sy, 2.0, YELLOW);
        }
    }
}

/// The simulation keeps compact, stable internal symbols. The Terminal client
/// translates only their presentation so service roles remain readable in the
/// current language without changing commands, saves or content IDs.
const fn actor_ascii_glyph(symbol: char) -> Option<&'static str> {
    match symbol {
        'X' => Some("X"), // Laboratory target, not a campaign creature.
        'Y' => Some("!"), // Laboratory probe, not a consumable item.
        'L' => Some("L"),
        'A' => Some("A"),
        '@' => Some("@"),
        'c' => Some("R"),
        'm' => Some("T"),
        'v' => Some("M"),
        'h' => Some("S"),
        'i' => Some("H"),
        'u' => Some("u"),
        'd' => Some("d"),
        't' => Some("t"),
        'r' => Some("r"),
        'j' => Some("a"),
        'w' => Some("s"),
        'B' => Some("b"),
        'K' => Some("k"),
        'V' => Some("f"),
        'G' => Some("g"),
        'N' => Some("n"),
        _ => None,
    }
}

fn draw_ascii_actor(rect: Rect, glyph: &str, color: Color) {
    let mut font_size = (rect.h * 0.84).round().clamp(8.0, 34.0) as u16;
    while font_size > 8 && measure_ui_text_bold(glyph, font_size).width > rect.w - 2.0 {
        font_size -= 1;
    }
    let shadow = Rect::new(rect.x + 1.0, rect.y + 1.0, rect.w, rect.h);
    draw_ui_text_bold_centered(glyph, shadow, font_size, Color::from_rgba(0, 5, 7, 230));
    draw_ui_text_bold_centered(glyph, rect, font_size, color);
}

pub(crate) fn local_coordinates(position: Option<GridPos>) -> String {
    position.map_or_else(|| "—".to_owned(), |p| format!("X {} · Y {}", p.x, p.y))
}

fn draw_interaction_tooltip(anchor: Rect, map: Rect, title: &str, action: &str) {
    let width = (measure_ui_text(title, None, 15, 1.0)
        .width
        .max(measure_ui_text(action, None, 14, 1.0).width)
        + 22.0)
        .clamp(156.0, (map.w - 12.0).max(156.0));
    let height = 53.0;
    let preferred_x = anchor.x + anchor.w + 8.0;
    let x = if preferred_x + width <= map.x + map.w - 4.0 {
        preferred_x
    } else {
        (anchor.x - width - 8.0).max(map.x + 4.0)
    };
    let y = anchor.y.clamp(map.y + 4.0, map.y + map.h - height - 4.0);
    let panel = Rect::new(x, y, width, height);
    UiTheme.hud_panel(panel);
    draw_bounded_text(
        title,
        x + 11.0,
        y + 20.0,
        width - 20.0,
        15,
        Color::from_rgba(224, 240, 237, 255),
    );
    draw_bounded_text(
        action,
        x + 11.0,
        y + 40.0,
        width - 20.0,
        14,
        Color::from_rgba(142, 226, 210, 255),
    );
}

fn draw_effect_badges(rect: Rect, badges: &[TerminalEffectBadge]) {
    // Four per row; more rows never discard an effect or impose a gameplay cap.
    let columns = badges.len().min(4).max(1);
    let size = (rect.w / columns as f32).clamp(8.0, 13.0);
    for (index, icon) in badges.iter().enumerate() {
        let badge = Rect::new(
            rect.center().x - columns as f32 * size * 0.5 + (index % 4) as f32 * size,
            rect.y - 2.0 - (index / 4 + 1) as f32 * (size + 1.0),
            size,
            size,
        );
        draw_rectangle(
            badge.x,
            badge.y,
            badge.w,
            badge.h,
            Color::from_rgba(3, 12, 17, 240),
        );
        let (pattern, color) = match icon {
            TerminalEffectBadge::Alternation => {
                (&BADGE_ALTERNATION, Color::from_rgba(195, 177, 255, 255))
            }
            TerminalEffectBadge::Echo { turns } => {
                draw_text(
                    &turns.to_string(),
                    badge.x + 2.0,
                    badge.y + badge.h - 1.0,
                    badge.h,
                    Color::from_rgba(220, 240, 255, 255),
                );
                continue;
            }
            TerminalEffectBadge::Guard => (&BADGE_GUARD, Color::from_rgba(131, 204, 255, 255)),
            TerminalEffectBadge::Burning => (&FLAME, Color::from_rgba(255, 152, 63, 255)),
            TerminalEffectBadge::Corrosion => (&ACID_DROP, Color::from_rgba(170, 232, 94, 255)),
            TerminalEffectBadge::Caustic => (&ACID_POOL, Color::from_rgba(170, 232, 94, 255)),
            TerminalEffectBadge::Electrical => (&ARC_FORK, Color::from_rgba(107, 235, 238, 255)),
            TerminalEffectBadge::Fragile => (&EFFECT_IMPACT, Color::from_rgba(243, 170, 105, 255)),
            TerminalEffectBadge::Slowed => (&BADGE_SLOWED, Color::from_rgba(147, 195, 247, 255)),
            TerminalEffectBadge::Suppressed => {
                (&BADGE_SUPPRESSED, Color::from_rgba(211, 171, 247, 255))
            }
            TerminalEffectBadge::Timed => (&BADGE_TIMED, Color::from_rgba(208, 228, 230, 255)),
            TerminalEffectBadge::Fracture { charges, threshold } => {
                let count = (*threshold).clamp(1, 5);
                for step in 0..count {
                    let filled = u32::from(step) * u32::from(*threshold)
                        < u32::from(*charges) * u32::from(count);
                    let width = badge.w / f32::from(count);
                    let height = badge.h * (0.40 + f32::from(step + 1) / f32::from(count) * 0.45);
                    draw_rectangle(
                        badge.x + f32::from(step) * width,
                        badge.bottom() - height,
                        (width - 1.0).max(1.0),
                        height,
                        if filled {
                            Color::from_rgba(247, 224, 177, 255)
                        } else {
                            Color::from_rgba(77, 88, 90, 255)
                        },
                    );
                }
                continue;
            }
        };
        draw_pixel_glyph_palette(badge, pattern, TerminalGlyphPalette::monochrome(color));
    }
}

fn draw_status_icon(rect: Rect, icon: TerminalStatusIcon) {
    let badge_size = if icon.is_quest() {
        (rect.w * 0.70).clamp(20.0, 24.0)
    } else {
        (rect.w * 0.43).clamp(11.0, 15.0)
    };
    let badge = if icon.is_quest() {
        Rect::new(
            rect.x + (rect.w - badge_size) * 0.5,
            rect.y - badge_size * 0.65,
            badge_size,
            badge_size,
        )
    } else {
        Rect::new(
            rect.x + rect.w - badge_size + 1.0,
            rect.y + rect.h - badge_size + 1.0,
            badge_size,
            badge_size,
        )
    };
    if icon.is_quest() {
        let marker = match icon {
            TerminalStatusIcon::QuestAvailable => "!",
            TerminalStatusIcon::QuestReady => "?",
            TerminalStatusIcon::QuestInProgress => "…",
            TerminalStatusIcon::QuestObjective => "*",
            _ => unreachable!(),
        };
        let size = (badge.h * 1.1).round().clamp(18.0, 23.0) as u16;
        let shadow = Rect::new(badge.x + 1.0, badge.y + 1.0, badge.w, badge.h);
        draw_ui_text_bold_centered(marker, shadow, size, Color::from_rgba(2, 8, 12, 255));
        draw_ui_text_bold_centered(marker, badge, size, Color::from_rgba(255, 230, 130, 255));
        return;
    }
    if icon == TerminalStatusIcon::RequiredMaterial {
        let center_x = badge.x + badge.w * 0.5;
        let center_y = badge.y + badge.h * 0.5;
        let arm = badge.w * 0.24;
        let shadow = Color::from_rgba(2, 8, 12, 255);
        let color = Color::from_rgba(255, 218, 135, 255);
        draw_line(
            center_x - arm + 1.0,
            center_y + 1.0,
            center_x + arm + 1.0,
            center_y + 1.0,
            3.0,
            shadow,
        );
        draw_line(
            center_x + 1.0,
            center_y - arm + 1.0,
            center_x + 1.0,
            center_y + arm + 1.0,
            3.0,
            shadow,
        );
        draw_line(
            center_x - arm,
            center_y,
            center_x + arm,
            center_y,
            2.0,
            color,
        );
        draw_line(
            center_x,
            center_y - arm,
            center_x,
            center_y + arm,
            2.0,
            color,
        );
        return;
    }
    if icon == TerminalStatusIcon::Sheltered {
        let color = Color::from_rgba(177, 229, 182, 255);
        let points = [
            (badge.x + 1.0, badge.y + 1.0),
            (badge.right() - 1.0, badge.y + 1.0),
            (badge.right() - 2.0, badge.y + badge.h * 0.65),
            (badge.x + badge.w * 0.5, badge.bottom() - 1.0),
            (badge.x + 2.0, badge.y + badge.h * 0.65),
        ];
        for i in 0..points.len() {
            let (x, y) = points[i];
            let (nx, ny) = points[(i + 1) % points.len()];
            draw_line(x, y, nx, ny, 3.0, Color::from_rgba(3, 12, 10, 255));
            draw_line(x, y, nx, ny, 1.5, color);
        }
        return;
    }
    draw_rectangle(
        badge.x,
        badge.y,
        badge.w,
        badge.h,
        Color::from_rgba(7, 15, 21, 245),
    );
    draw_rectangle_lines(
        badge.x,
        badge.y,
        badge.w,
        badge.h,
        1.0,
        Color::from_rgba(255, 218, 135, 255),
    );
    match icon {
        TerminalStatusIcon::Burning => draw_pixel_glyph_palette(
            badge,
            &FLAME_LARGE,
            TerminalGlyphPalette::new(
                Color::from_rgba(194, 43, 18, 225),
                Some(Color::from_rgba(255, 105, 22, 240)),
                Some(Color::from_rgba(255, 220, 82, 255)),
            ),
        ),
        TerminalStatusIcon::RequiredMaterial | TerminalStatusIcon::Sheltered => unreachable!(),
        TerminalStatusIcon::QuestAvailable
        | TerminalStatusIcon::QuestReady
        | TerminalStatusIcon::QuestInProgress
        | TerminalStatusIcon::QuestObjective => unreachable!(),
    }
}

fn draw_alert_marker(rect: Rect) {
    let pulse = ((get_time() * 5.0).sin() * 0.5 + 0.5) as f32;
    let alert_color = Color::new(1.0, 0.48 + pulse * 0.2, 0.17, 0.75 + pulse * 0.25);
    draw_rectangle_lines(
        rect.x - 2.0,
        rect.y - 2.0,
        rect.w + 4.0,
        rect.h + 4.0,
        3.0 + pulse,
        alert_color,
    );
    draw_rectangle_lines(
        rect.x + 2.0,
        rect.y + 2.0,
        rect.w - 4.0,
        rect.h - 4.0,
        2.0,
        Color::from_rgba(255, 222, 145, 255),
    );
    let badge_size = (rect.w * 0.55).clamp(12.0, 18.0);
    draw_rectangle(
        rect.x + rect.w - badge_size + 2.0,
        rect.y - 3.0,
        badge_size,
        badge_size,
        Color::from_rgba(171, 48, 17, 255),
    );
    draw_rectangle_lines(
        rect.x + rect.w - badge_size + 2.0,
        rect.y - 3.0,
        badge_size,
        badge_size,
        2.0,
        Color::from_rgba(255, 226, 151, 255),
    );
    draw_text(
        "!",
        rect.x + rect.w - badge_size + 5.0,
        rect.y + badge_size - 5.0,
        badge_size + 2.0,
        WHITE,
    );
}

type PixelGlyph = [&'static str; 8];
const BADGE_ALTERNATION: PixelGlyph = [
    "........", ".##..##.", "#..##..#", "#..##..#", ".##..##.", "........", "........", "........",
];
fn draw_pixel_glyph(rect: Rect, pattern: &PixelGlyph, color: Color) {
    draw_pixel_glyph_palette(rect, pattern, TerminalGlyphPalette::monochrome(color));
}

fn draw_pixel_glyph_palette(rect: Rect, pattern: &PixelGlyph, palette: TerminalGlyphPalette) {
    let pixel = (rect.w / 10.0).floor().max(1.0);
    let ox = (rect.x + (rect.w - 8.0 * pixel) / 2.0).floor();
    let oy = (rect.y + (rect.h - 8.0 * pixel) / 2.0).floor();
    for (y, row) in pattern.iter().enumerate() {
        for (x, value) in row.bytes().enumerate() {
            if value == b'.' {
                continue;
            }
            let ink = match value {
                b'+' => palette.accent.unwrap_or_else(|| {
                    Color::new(
                        palette.primary.r * 0.46,
                        palette.primary.g * 0.55,
                        palette.primary.b * 0.6,
                        palette.primary.a,
                    )
                }),
                b'*' => palette
                    .highlight
                    .or(palette.accent)
                    .unwrap_or(palette.primary),
                _ => palette.primary,
            };
            draw_rectangle(
                ox + x as f32 * pixel,
                oy + y as f32 * pixel,
                pixel,
                pixel,
                ink,
            );
        }
    }
}

const PILLAR: PixelGlyph = [
    "..####..", ".#++++#.", "#++##++#", "#+####+#", "#+####+#", "#++##++#", ".#++++#.", "..####..",
];
const CRATE: PixelGlyph = [
    "########", "#++++++#", "#+#++#+#", "#++##++#", "#++##++#", "#+#++#+#", "#++++++#", "########",
];
const SERVER: PixelGlyph = [
    "########", "#++++#+#", "########", "#++++#+#", "########", "#++++#+#", "########", ".++..++.",
];
const CONSOLE: PixelGlyph = [
    ".######.", "#++++++#", "#+####+#", "#+#++#+#", "#++++++#", "########", "#.#.#..#", "########",
];
const COOLANT: PixelGlyph = [
    "..####..", ".#++++#.", "##+##+##", "#+#++#+#", "#+#++#+#", "##+##+##", ".#++++#.", "..####..",
];
const RESONATOR: PixelGlyph = [
    "...##...", ".##++##.", "##+**+##", ".#+##+#.", ".#+##+#.", "##+**+##", ".##++##.", "...##...",
];
const GROWTH_NODE: PixelGlyph = [
    "#......#", ".#....#.", "..#++#..", ".#+**+#.", "..#**#..", "...##...", "..####..", ".##..##.",
];
const EYE_NODE: PixelGlyph = [
    "........", "..####..", ".##++##.", "##+**+##", "##+**+##", ".##++##.", "..####..", "........",
];
const ROOT_MASS: PixelGlyph = [
    "...##...", "..####..", ".##++##.", "##+##+##", ".#+##+#.", "..####..", ".##..##.", "##....##",
];
const KERNEL_FAULT: PixelGlyph = [
    "########", "#..##..#", "#.#++#.#", "##+##+##", "##+##+##", "#.#++#.#", "#..##..#", "########",
];
const ORPHAN_PROCESS: PixelGlyph = [
    ".######.", "##++++##", "#++##++#", "#++..++#", "#++##++#", "##++++##", ".###.##.", "##....##",
];
const CLINIC_BED: PixelGlyph = [
    "########", "#++++++#", "#++##++#", "#++##++#", "#++++++#", "#++++++#", "########", "##....##",
];
const CLINIC_COUNTER: PixelGlyph = [
    "########", "#++++++#", "#++##++#", "#++##++#", "#++++++#", "########", "##....##", "##....##",
];
const TREE_CANOPY: PixelGlyph = [
    "...##...", ".######.", "########", "##+##+##", "########", ".######.", "...##...", "...##...",
];
const BOULDER: PixelGlyph = [
    "........", "..####..", ".######.", "##++++##", "##+##+##", "########", ".######.", "........",
];
const RUIN_WALL: PixelGlyph = [
    "##..####", "##..####", "########", "####..##", "####..##", "##..####", "########", "########",
];
const BARRICADE: PixelGlyph = [
    "#..##..#", ".######.", "##.##.##", ".######.", "##.##.##", ".######.", "#..##..#", "........",
];
const SATURATION_BEACON: PixelGlyph = [
    "...##...", "..####..", ".#+**+#.", ".#++++#.", "..####..", "...##...", "..####..", ".######.",
];
const VOLATILE_CONTAINER: PixelGlyph = [
    "..####..", ".##++##.", ".#+**+#.", ".#+**+#.", ".#+**+#.", ".######.", "..####..", "...##...",
];
const SUBMERGED_RELAY: PixelGlyph = [
    "..####..", ".##++##.", ".#+**+#.", ".#*##+#.", ".#+##*#.", ".######.", "..#++#..", ".##..##.",
];
const WEAPON: PixelGlyph = [
    "........", ".....##.", "..#####.", ".######.", "..##....", ".##.....", ".##.....", "........",
];
const REPAIR: PixelGlyph = [
    "........", ".######.", ".#+##+#.", ".######.", ".######.", ".#+##+#.", ".######.", "........",
];
const EXIT: PixelGlyph = [
    "#######.", "#.....#.", "#..#..#.", "#..##...", "#..###..", "#..##...", "#..#..#.", "#######.",
];
const EFFECT: PixelGlyph = [
    "#..++..#", ".#....#.", "..+**+..", "+.*##*.+", "+.*##*.+", "..+**+..", ".#....#.", "#..++..#",
];
const EFFECT_DOT: PixelGlyph = [
    "........", "........", "...##...", "..#++#..", "..#**#..", "...##...", "........", "........",
];
const EFFECT_PROJECTILE: PixelGlyph = [
    "........", "........", "...++...", ".#+**+#.", ".#+**+#.", "...++...", "........", "........",
];
const EFFECT_BURST: PixelGlyph = [
    "...##...", "..+##+..", "...**...", "#+****+#", "#+****+#", "...**...", "..+##+..", "...##...",
];
const EFFECT_IMPACT: PixelGlyph = [
    "##....##", ".+#..#+.", "..+**+..", "...##...", "...##...", "..+**+..", ".+#..#+.", "##....##",
];
const EFFECT_WAVE: PixelGlyph = [
    "........", "..+..+..", ".#*..*#.", "#+*##*+#", ".#*..*#.", "..+..+..", "........", "........",
];
const EFFECT_ALARM: PixelGlyph = [
    "...##...", "..+**+..", "..+**+..", ".#****#.", ".#+**+#.", "##+**+##", "...##...", "...##...",
];
const SHOCK_SMALL: PixelGlyph = [
    "........", "........", "...++...", "..+**+..", "..+**+..", "...++...", "........", "........",
];
const BADGE_GUARD: PixelGlyph = [
    "...##...", ".##..##.", ".#....#.", ".#....#.", ".#....#.", "..#..#..", "...##...", "........",
];
const BADGE_TIMED: PixelGlyph = [
    ".######.", "..#..#..", "...##...", "...##...", "..#..#..", ".######.", "........", "........",
];
const BADGE_SLOWED: PixelGlyph = [
    "........", "..#..#..", "..#..#..", "..#..#..", "..#..#..", "........", "........", "........",
];
const BADGE_SUPPRESSED: PixelGlyph = [
    "...##...", "..#..#..", ".#....#.", "..#..#..", "...##...", "........", "..####..", "........",
];
const SHOCK_WIDE: PixelGlyph = [
    ".#....#.", "..+..+..", ".+.**.+.", "..*..*..", "..*..*..", ".+.**.+.", "..+..+..", ".#....#.",
];
const RING_SMALL: PixelGlyph = [
    "........", "........", "...++...", "..+..+..", "..+..+..", "...++...", "........", "........",
];
const RING_WIDE: PixelGlyph = [
    "..++++..", ".+....+.", "+......+", "+......+", "+......+", "+......+", ".+....+.", "..++++..",
];
const ARC_FORK: PixelGlyph = [
    "....+...", "...*....", "+..*....", ".++**...", "....*++.", "...*...+", "..+.....", ".+......",
];
const ARC_SPLIT: PixelGlyph = [
    ".+....+.", "..+..*..", "...**...", "....*...", "...**...", "..*..+..", ".*....+.", "+.......",
];
const ACID_DROP: PixelGlyph = [
    "....+...", "....+...", "...+*+..", "...+*+..", "....+...", "........", "..#..#..", "........",
];
const ACID_SPLASH: PixelGlyph = [
    "........", "........", ".+....+.", "..+..+..", "...**...", ".++**++.", "..####..", "........",
];
const ACID_POOL: PixelGlyph = [
    "........", "........", "........", ".....+..", "..+.....", ".#+##+#.", "#++**++#", ".######.",
];
const FLAME_SMALL: PixelGlyph = [
    "........", "....#...", "...#+...", "...++#..", "..#+*#..", "..#**#..", "...##...", "........",
];
const FLAME: PixelGlyph = [
    "....#...", "...#+...", "..#++...", "..#+*#..", ".#+**+#.", ".#+***#.", "..#++#..", "...##...",
];
const FLAME_LARGE: PixelGlyph = [
    "...#....", "..#+#...", "..#++...", ".#+++#..", ".#+**+#.", "##+***##", ".#++**#.", "..####..",
];
const ELECTRIFIED_FIELD: PixelGlyph = [
    "........", ".#....#.", "..#+.#..", "...**...", "..**....", ".#+..+#.", "#......#", "........",
];
const TRACE: PixelGlyph = [
    "........", "...#....", "..###...", ".#####..", "...#....", "...#....", "........", "........",
];
const TRACE_EAST: PixelGlyph = [
    "........", "....#...", "....##..", ".######.", "....##..", "....#...", "........", "........",
];
const TRACE_SOUTH: PixelGlyph = [
    "........", "...#....", "...#....", ".#####..", "..###...", "...#....", "........", "........",
];
const TRACE_WEST: PixelGlyph = [
    "........", "...#....", "..##....", ".######.", "..##....", "...#....", "........", "........",
];
const CLOSED_DOOR: PixelGlyph = [
    "########", "#++##++#", "#++##++#", "#+####+#", "#+####+#", "#++##++#", "#++##++#", "########",
];
const OPEN_DOOR: PixelGlyph = [
    "##....##", "##....##", "#......#", "#......#", "#......#", "#......#", "##....##", "##....##",
];
const LOCKED_DOOR: PixelGlyph = [
    "########", "#+####+#", "#+#..#+#", "#++++++#", "#+####+#", "#+####+#", "#++++++#", "########",
];
const UNPOWERED_DOOR: PixelGlyph = [
    "########", "#++##++#", "#++##++#", "#++##++#", "#.#..#.#", "#..##..#", "#.#..#.#", "########",
];
const CONTROL_READY: PixelGlyph = [
    "########", "#++++++#", "#++#+++#", "#++##++#", "#++#+++#", "########", "#.#.#..#", "########",
];
const CONTROL_USED: PixelGlyph = [
    "########", "#++++++#", "#++++#+#", "#+#++#+#", "#++##++#", "########", "#......#", "########",
];
const DEPOT: PixelGlyph = [
    "########", "#++++++#", "#+####+#", "#+#..#+#", "#+#..#+#", "#+####+#", "#++++++#", "########",
];
const RELAY_OFFLINE: PixelGlyph = [
    "########", "#...##.#", "#..##..#", "#.##...#", "#...##.#", "#..##..#", "#.##...#", "########",
];
const RELAY_ONLINE: PixelGlyph = [
    "########", "#...##.#", "#..##..#", "#.#####.", ".#####.#", "#..##..#", "#.##...#", "########",
];
const ACTUATOR_OFFLINE: PixelGlyph = [
    "########", "#......#", "#.####.#", "#.#..#.#", "#.#..#.#", "#.####.#", "#......#", "########",
];
const ACTUATOR_ONLINE: PixelGlyph = [
    "########", "#..##..#", "#.####.#", "###..###", "###..###", "#.####.#", "#..##..#", "########",
];
const SENSOR_OFFLINE: PixelGlyph = [
    "########", "#......#", "#..##..#", "#.#..#.#", "#.#..#.#", "#..##..#", "#......#", "########",
];
const SENSOR_ONLINE: PixelGlyph = [
    "########", "#.#..#.#", "#..##..#", "#.####.#", "#.####.#", "#..##..#", "#.#..#.#", "########",
];
const DATA_TERMINAL_OFFLINE: PixelGlyph = [
    "########", "#......#", "#.####.#", "#.#..#.#", "#.####.#", "#......#", "#..##..#", "########",
];
const DATA_TERMINAL_ONLINE: PixelGlyph = [
    "########", "#++++++#", "#+####+#", "#+#..#+#", "#+####+#", "#++++++#", "#..##..#", "########",
];
const DATA_TERMINAL_UPDATED: PixelGlyph = [
    "########", "#++++++#", "#+....+#", "#+...#+#", "#+#.##+#", "#+.##.+#", "#++++++#", "########",
];
const DIRECTION_BOARD: PixelGlyph = [
    "........", ".######.", ".#....#.", ".#..#.#.", ".#....#.", ".######.", "...##...", "...##...",
];
const DIRECTION_BOARD_UPDATED: PixelGlyph = [
    "........", ".######.", ".#..#.#.", ".#...##.", ".#..#.#.", ".######.", "...##...", "...##...",
];
const SERVICE_PLAN: PixelGlyph = [
    "........", ".######.", ".#.#..#.", ".#..#.#.", ".#.#..#.", ".######.", "...##...", "...##...",
];
const SUPPLY_CACHE: PixelGlyph = [
    "........", ".######.", ".#....#.", ".######.", ".#..#.#.", ".######.", "........", "........",
];
const THREAT_CAMP: PixelGlyph = [
    "...##...", "..####..", ".######.", "##.##.##", "...##...", "..####..", ".#....#.", "........",
];
const THREAT_CAMP_DISABLED: PixelGlyph = [
    "........", ".#....#.", "..#..#..", "...##...", "...##...", "..#..#..", ".#....#.", "........",
];
const MATERIAL: PixelGlyph = [
    "........", "..####..", ".##..##.", "##.##.##", "##.##.##", ".##..##.", "..####..", "........",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_hover_labels_keep_all_live_fields_without_an_occupant_glyph() {
        use project_rl::combat::{DamagePacket, DamageType};
        use project_rl::effects::{GroundEffectMap, GroundEffectSpec};
        let at = GridPos::new(2, 2);
        let mut ground = GroundEffectMap::default();
        for (id, damage, label) in [
            ("test:fire", DamageType::Thermal, "Flammes au sol"),
            ("test:acid", DamageType::Chemical, "Flaque caustique"),
            ("test:electric", DamageType::Electrical, "Sol électrifié"),
        ] {
            ground.apply(
                at,
                None,
                &GroundEffectSpec::new(id.parse().unwrap(), 3, DamagePacket::new(1, damage, 0))
                    .unwrap(),
                0,
            );
            assert!(
                ground_effect_description(&ground, at)
                    .unwrap()
                    .contains(label)
            );
        }
        let labels = ground_effect_description(&ground, at).unwrap();
        assert_eq!(labels.matches("3t").count(), 3);
        assert!(ground_effect_description(&ground, GridPos::new(1, 1)).is_none());
        for id in ["test:fire", "test:acid", "test:electric"] {
            for _ in 0..3 {
                ground.elapse(at, &id.parse().unwrap());
            }
        }
        assert!(ground_effect_description(&ground, at).is_none());
    }
    use project_rl::world::FieldOfViewRules;

    #[test]
    fn interaction_hint_only_advertises_a_key_when_it_can_target_the_element() {
        let player = GridPos::new(4, 4);
        let adjacent = GridPos::new(5, 4);
        let far = GridPos::new(7, 4);
        let door = InteractionHint {
            title: "Porte fermée".to_owned(),
            is_loot: false,
            verb: "ouvrir",
            unavailable: None,
        };
        assert_eq!(
            interaction_action(Some(player), adjacent, Some(adjacent), &door, "E", false),
            "E · ouvrir"
        );
        assert_eq!(
            interaction_action(Some(player), adjacent, None, &door, "E", false),
            "E · choisir : ouvrir"
        );
        assert_eq!(
            interaction_action(Some(player), far, None, &door, "E", false),
            "Approchez-vous pour interagir"
        );
        let locked = InteractionHint {
            unavailable: Some("Autorisation nécessaire"),
            ..door.clone()
        };
        assert_eq!(
            interaction_action(Some(player), adjacent, Some(adjacent), &locked, "E", false),
            "Autorisation nécessaire"
        );
        let loot = InteractionHint {
            title: "Objet au sol".to_owned(),
            is_loot: true,
            verb: "ramasser",
            unavailable: None,
        };
        assert_eq!(
            interaction_action(Some(player), player, None, &loot, "E", false),
            "E · ramasser"
        );
        assert_eq!(
            interaction_action(Some(player), player, None, &loot, "E", true),
            "E · choisir : ramasser"
        );
        assert_eq!(
            interaction_action(Some(player), adjacent, None, &loot, "E", false),
            "Rejoignez la case pour ramasser"
        );
    }

    #[test]
    fn memory_never_refreshes_hidden_terrain_or_joins_unknown_walls() {
        let mut map = Map::from_ascii("#########\n#.......#\n#.......#\n#########").unwrap();
        let mut visibility = VisibilityState::default();
        let rules = FieldOfViewRules {
            radius: 2,
            ..Default::default()
        };
        visibility.recompute(&map, GridPos::new(1, 1), rules);
        let mut view = TerminalView::new(SectorDecor::default(), &map, &visibility);
        assert!(view.known(GridPos::new(7, 1)).is_none());
        let wall = GridPos::new(2, 0);
        assert!(!view.wall_joins(wall)[1]); // (3,0) outside perception circle
        let remembered = view.known(GridPos::new(1, 1));
        visibility.recompute(&map, GridPos::new(7, 1), rules);
        map.set_terrain(GridPos::new(1, 1), Terrain::Wall).unwrap();
        view.observe(&map, &visibility);
        assert_eq!(view.known(GridPos::new(1, 1)), remembered);
        assert!(view.known(GridPos::new(7, 1)).is_some());
    }

    #[test]
    fn camera_tiles_and_pointer_agree_at_every_zoom_and_window_size() {
        for (width, height) in [
            (640.0, 480.0),
            (960.0, 540.0),
            (1280.0, 800.0),
            (3440.0, 1440.0),
        ] {
            for size in [24.0, 32.0, 40.0, 48.0] {
                let camera = GridCamera::new(
                    Rect::new(24.0, 110.0, width - 48.0, height - 220.0),
                    size,
                    GridPos::new(12, 12),
                );
                for row in 0..camera.rows {
                    for col in 0..camera.columns {
                        let pos = GridPos::new(camera.first.x + col, camera.first.y + row);
                        let rect = camera.rect(pos);
                        assert_eq!(camera.hit((rect.x + 0.5, rect.y + 0.5)), Some(pos));
                        assert!(rect.x >= 24.0 && rect.y >= 110.0);
                        assert!(rect.x + rect.w <= width - 24.0);
                        assert!(rect.y + rect.h <= height - 110.0);
                    }
                }
                assert!(camera.hit((0.0, 0.0)).is_none());
                assert_eq!(camera.rect(GridPos::new(12, 12)).w, size);
            }
        }
    }

    #[test]
    fn route_map_and_cartography_use_separate_non_overlapping_regions() {
        let bounds = Rect::new(20.0, 76.0, 1240.0, 620.0);
        let layout = terminal_ui_layout(bounds, 1.0, true);
        let route = layout.route.expect("navigation route should be visible");
        let sidebar = layout.sidebar.expect("wide view should have cartography");
        let status =
            terminal_status_panel(bounds, 1.0).expect("wide view should have player status");
        assert!(status.x + status.w < route.x);
        assert!(route.x + route.w < sidebar.x);
        assert!(layout.map.x + layout.map.w < sidebar.x);
        assert!(layout.header.x + layout.header.w < sidebar.x);
        assert!(route.y >= layout.header.y + layout.header.h);
        assert!(layout.map.y > route.y + route.h);

        let narrow = terminal_ui_layout(Rect::new(20.0, 76.0, 900.0, 420.0), 1.0, true);
        assert!(narrow.sidebar.is_none());
        assert!(narrow.route.is_some());
    }

    #[test]
    fn scaled_status_panel_preserves_text_space_and_world_pointer_alignment() {
        let map = Map::from_ascii("#########\n#.......#\n#.......#\n#.......#\n#########").unwrap();
        let game = WorldState::single(GameState::new(map, GridPos::new(4, 2), 1).unwrap());
        let view = TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        for (width, height) in [
            (960.0, 540.0),
            (1280.0, 800.0),
            (1920.0, 1080.0),
            (3840.0, 2160.0),
        ] {
            for percent in [75, 100, 125, 150, 200] {
                let mut settings = crate::graphics::GraphicsSettings::default();
                settings.ui_scale_percent = percent;
                let scale = settings.ui_scale(width, height);
                let bounds = Rect::new(20.0, 7.0 * scale, width - 40.0, height - 111.0 * scale);
                for navigation in [false, true] {
                    let layout = terminal_ui_layout(bounds, scale, navigation);
                    if let Some(panel) = terminal_status_panel(bounds, scale) {
                        assert!((panel.w / scale - 260.0).abs() < 0.01);
                        assert!(panel.h / scale >= 480.0);
                        assert!(panel.right() < layout.map.x);
                        assert!(layout.map.w >= 400.0);
                        assert!(layout.map.right() < layout.sidebar.unwrap().x);
                        assert!(
                            view.hit_test(
                                &game,
                                bounds,
                                scale,
                                32,
                                navigation,
                                (panel.x + 20.0, panel.y + 20.0)
                            )
                            .is_none()
                        );
                    } else {
                        assert!(layout.sidebar.is_none());
                        assert!(layout.map.w >= width - 60.0);
                    }
                    for cell_size in [24, 32, 40, 48] {
                        let at = game.player_position().unwrap();
                        let cell = view
                            .world_cell_rect(&game, bounds, scale, cell_size, navigation, at)
                            .unwrap();
                        assert_eq!(
                            view.hit_test(
                                &game,
                                bounds,
                                scale,
                                cell_size,
                                navigation,
                                (cell.x + cell.w * 0.5, cell.y + cell.h * 0.5)
                            ),
                            Some(at)
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn cartography_focuses_the_remembered_region_with_a_bounded_margin() {
        let mut remembered = BTreeMap::new();
        for y in 40..=44 {
            for x in 70..=76 {
                remembered.insert(
                    GridPos::new(x, y),
                    KnownTile {
                        terrain: Terrain::Floor,
                        decor: Decor::default(),
                    },
                );
            }
        }
        assert_eq!(
            remembered_bounds(&remembered, Some(GridPos::new(73, 42)), 160, 96),
            (GridPos::new(68, 38), GridPos::new(78, 46))
        );
        assert_eq!(
            remembered_bounds(&BTreeMap::new(), Some(GridPos::new(0, 0)), 160, 96),
            (GridPos::new(0, 0), GridPos::new(2, 2))
        );
    }

    #[test]
    fn sensor_contacts_use_shapes_with_stable_semantic_categories() {
        for symbol in ['d', 't', 'r', 'j', 'w', 'B', 'V', 'K', 'X', 'Y'] {
            assert_eq!(sensor_contact_kind(symbol), SensorContactKind::Hostile);
        }
        for symbol in ['m', 'h', 'v'] {
            assert_eq!(sensor_contact_kind(symbol), SensorContactKind::Service);
        }
        for symbol in ['c', 'i', 'u', 'b', 'G', 'N'] {
            assert_eq!(sensor_contact_kind(symbol), SensorContactKind::Neutral);
        }
        assert_eq!(sensor_contact_kind('?'), SensorContactKind::Unknown);
    }

    #[test]
    fn sensor_projection_rejects_positions_outside_its_authorized_bounds() {
        let minimum = GridPos::new(4, 7);
        let maximum = GridPos::new(11, 16);
        assert!(sensor_position_is_inside(minimum, minimum, maximum));
        assert!(sensor_position_is_inside(maximum, minimum, maximum));
        assert!(!sensor_position_is_inside(
            GridPos::new(3, 7),
            minimum,
            maximum
        ));
        assert!(!sensor_position_is_inside(
            GridPos::new(11, 17),
            minimum,
            maximum
        ));
    }

    #[test]
    fn procedural_location_names_hide_internal_coordinates_and_use_player_language() {
        assert_eq!(
            player_location_name("HUMAN HABITAT · région +1, +0"),
            "Territoires habités"
        );
        assert_eq!(
            player_location_name("CORRUPTED · région -12, +7"),
            "Profondeurs corrompues"
        );
        assert_eq!(
            player_location_name("Ville de départ / Friches"),
            "Ville de départ / Friches"
        );
    }

    #[test]
    fn legend_panel_stays_inside_responsive_world_bounds() {
        for bounds in [
            Rect::new(20.0, 76.0, 600.0, 300.0),
            Rect::new(20.0, 76.0, 1240.0, 620.0),
            Rect::new(20.0, 100.0, 1880.0, 800.0),
        ] {
            let panel = legend_panel(bounds);
            assert!(bounds.contains(panel.point()));
            assert!(bounds.contains(vec2(panel.right(), panel.bottom())));
        }
    }

    #[test]
    fn small_windows_fit_the_default_sensor_footprint_without_altering_range() {
        for bounds in [
            Rect::new(28.0, 110.0, 584.0, 250.0),
            Rect::new(28.0, 110.0, 920.0, 260.0),
        ] {
            for preferred in [24, 32, 40, 48] {
                let camera = GridCamera::new(
                    bounds,
                    fitted_cell_size(bounds, preferred, 8),
                    GridPos::new(20, 20),
                );
                assert!(camera.rows >= 17 && camera.columns >= 17);
            }
        }
    }

    #[test]
    fn navigation_signal_uses_stable_approximate_directions_and_distance_bands() {
        let observer = GridPos::new(10, 10);
        assert_eq!(
            navigation_signal_summary(observer, GridPos::new(30, 0), 20, 1),
            "SIGNAL DE SITE · NORD-EST · PROCHE"
        );
        assert_eq!(
            navigation_signal_summary(observer, GridPos::new(40, 9), 30, 2),
            "SIGNAL DE SITE · EST · À DISTANCE · +1 AUTRE(S)"
        );
        assert_eq!(
            navigation_signal_summary(observer, GridPos::new(11, 12), 2, 1),
            "SIGNAL DE SITE · SUR PLACE"
        );
        assert_eq!(
            directional_signal_summary("ACCÈS INFÉRIEUR", observer, GridPos::new(-30, 10), 40, 1,),
            "ACCÈS INFÉRIEUR · OUEST · À DISTANCE"
        );
    }

    #[test]
    fn ascii_actors_are_unique_and_terminal_shapes_are_well_formed() {
        let actors = [
            '@', 'c', 'm', 'v', 'h', 'i', 'u', 'd', 't', 'r', 'j', 'w', 'B', 'V', 'G', 'N', 'K',
        ]
        .map(|symbol| actor_ascii_glyph(symbol).expect("known actor must have an ASCII glyph"));
        for (index, glyph) in actors.iter().enumerate() {
            assert!(!actors[..index].contains(glyph));
        }
        assert_eq!(actor_ascii_glyph('v'), Some("M"));
        assert_eq!(actor_ascii_glyph('Y'), Some("!"));
        assert_ne!(overlay_label('Y'), overlay_label('!'));
        assert!(overlay_label('Y').contains("4 dégâts électriques"));
        assert_eq!(actor_ascii_glyph('i'), Some("H"));
        assert_eq!(
            overlay_label('v'),
            "Marchande neutre · achat, vente et paris"
        );

        for glyph in [
            SHOCK_SMALL,
            SHOCK_WIDE,
            RING_SMALL,
            RING_WIDE,
            ARC_FORK,
            ARC_SPLIT,
            ACID_DROP,
            ACID_SPLASH,
            ACID_POOL,
            BADGE_TIMED,
            BADGE_GUARD,
            BADGE_SLOWED,
            BADGE_SUPPRESSED,
            VOLATILE_CONTAINER,
            SUBMERGED_RELAY,
            WEAPON,
            REPAIR,
            EXIT,
            EFFECT,
            FLAME_SMALL,
            FLAME,
            FLAME_LARGE,
            ELECTRIFIED_FIELD,
            TRACE,
            PILLAR,
            CRATE,
            SERVER,
            CONSOLE,
            COOLANT,
            CLINIC_BED,
            CLINIC_COUNTER,
        ] {
            for row in glyph {
                assert_eq!(row.len(), 8);
                assert!(row.bytes().all(|b| b".#+*".contains(&b)));
            }
        }
    }
}
