//! A code-native terminal tileset: square cells, pixel glyphs, no raster assets.
//! Only remembered terrain and currently perceived overlays reach the renderer.
use std::collections::BTreeMap;

use macroquad::prelude::*;
use project_rl::combat::AttackAreaCell;
use project_rl::game::{GameState, WorldState};
use project_rl::world::{GridPos, Map, Terrain, VisibilityState};

use crate::test_sector::{Decor, SectorDecor, TestSector};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KnownTile {
    pub terrain: Terrain,
    pub decor: Decor,
}

#[derive(Clone, Copy, Debug)]
pub struct TerminalDrawOptions<'a> {
    pub bounds: Rect,
    pub cell_size: u16,
    pub interact_label: &'a str,
    pub legend_label: &'a str,
    pub legend_open: bool,
    pub attack_preview: Option<TerminalAttackPreview<'a>>,
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
}

#[derive(Clone, Copy, Debug)]
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
        draw_text(
            "PLAN DE DIAGNOSTIC · CARTE COMPLÈTE · HORS JEU",
            28.0,
            36.0,
            23.0,
            WHITE,
        );
        draw_text(
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
        draw_text(
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
        cell_size: u16,
        pointer: (f32, f32),
    ) -> Option<GridPos> {
        terminal_grid_camera(game, bounds, cell_size)
            .1
            .hit(pointer)
            .filter(|position| self.known(*position).is_some())
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
    ) {
        let TerminalDrawOptions {
            bounds,
            cell_size,
            interact_label,
            legend_label,
            legend_open,
            attack_preview,
        } = options;
        let cyan = Color::from_rgba(104, 201, 201, 255);
        let muted = Color::from_rgba(135, 162, 167, 255);
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
            Color::from_rgba(39, 73, 82, 255),
        );
        let protected = game
            .player_position()
            .is_some_and(|p| game.map().is_protected(p));
        let safety_label = if protected {
            "VILLE PROTÉGÉE"
        } else {
            "ZONE HOSTILE"
        };
        let safety_width = measure_text(safety_label, None, 16, 1.0).width;
        draw_bounded_text(
            &self.title,
            bounds.x + 14.0,
            bounds.y + 22.0,
            bounds.w
                - 28.0
                - if bounds.w > 750.0 {
                    safety_width + 24.0
                } else {
                    0.0
                },
            18,
            cyan,
        );
        if bounds.w > 750.0 {
            draw_text(
                safety_label,
                bounds.x + bounds.w - safety_width - 14.0,
                bounds.y + 22.0,
                16.0,
                if protected { cyan } else { ORANGE },
            );
        }
        let (sidebar, camera) = terminal_grid_camera(game, bounds, cell_size);
        let focus = game.player_position().unwrap_or(GridPos::new(12, 12));
        let visibility = game.player_visibility();
        for row in 0..camera.rows {
            for column in 0..camera.columns {
                let position = GridPos::new(camera.first.x + column, camera.first.y + row);
                let rect = camera.rect(position);
                let Some(tile) = self.known(position) else {
                    continue;
                };
                let visible = visibility.is_visible(position);
                draw_tile(
                    rect,
                    tile.decor,
                    self.wall_joins(position),
                    visible,
                    position,
                );
                if visible
                    && let Some(preview) = attack_preview
                    && let Some(cell) = preview.cells.iter().find(|cell| cell.position == position)
                {
                    draw_attack_preview_area(rect, preview.valid, cell.step);
                }
                // Even a buggy caller cannot render a live actor/effect in memory.
                if visible && let Some(cell) = overlay(position) {
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
                        cell.status_icon,
                    );
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
        let pointer = camera.hit(mouse_position());
        let pointer_inspected = pointer.and_then(|p| self.known(p).map(|t| (p, t)));
        let mut selected_inspected = None;
        let mut visible_hostiles = 0;
        let mut visible_neutrals = 0;
        let mut visible_items = 0;
        let mut visible_local_alerts = 0;
        let mut visible_security_alarms = 0;
        let mut visible_security_lockdowns = 0;
        for position in visibility.visible_positions() {
            if game.active_facility().is_some_and(|facility| {
                facility
                    .security_door_lockdown_at(position, game.turn())
                    .is_some()
            }) {
                visible_security_lockdowns += 1;
            }
            if let Some(cell) = overlay(position) {
                if matches!(cell.symbol, 'd' | 't' | 'r') {
                    visible_hostiles += 1;
                } else if matches!(cell.symbol, 'c' | 'm') {
                    visible_neutrals += 1;
                } else if matches!(cell.symbol, ')' | '!' | '=') {
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
            }
        }
        // Mouse inspection takes priority; keyboard target selection remains a
        // complete alternative when no known map cell is hovered.
        let inspected = pointer_inspected.or(selected_inspected);
        if let Some((position, _)) = inspected {
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
                        game.destination_name(link),
                        interact_label
                    )
                } else {
                    overlay(position).map_or_else(
                        || tile.decor.label().to_owned(),
                        |cell| {
                            let label = overlay_label(cell.symbol);
                            let alert_label = match cell.alert {
                                Some(TerminalAlertKind::LocalWitness) => {
                                    format!("{label} · ALERTE LOCALE")
                                }
                                Some(TerminalAlertKind::SecuritySystem) => {
                                    format!("{label} · ALARME DE SÉCURITÉ")
                                }
                                None => label.to_owned(),
                            };
                            match cell.status_icon {
                                Some(TerminalStatusIcon::Burning) => {
                                    format!("{alert_label} · EN FEU")
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
                game.destination_name(link)
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
        draw_bounded_text(
            &description,
            bounds.x + 14.0,
            bounds.y + bounds.h - 11.0,
            bounds.w - 28.0,
            16,
            muted,
        );
        if sidebar {
            self.draw_sidebar(
                game,
                Rect::new(
                    bounds.x + bounds.w - 232.0,
                    bounds.y + 38.0,
                    216.0,
                    bounds.h - 60.0,
                ),
                inspected.map(|_| description.as_str()),
                (
                    visible_hostiles,
                    visible_neutrals,
                    visible_items,
                    visible_local_alerts,
                    visible_security_alarms,
                    visible_security_lockdowns,
                ),
                legend_label,
            );
        }
        if visible_local_alerts + visible_security_alarms + visible_security_lockdowns > 0 {
            let pulse = ((get_time() * 4.0).sin() * 0.5 + 0.5) as f32;
            draw_rectangle_lines(
                bounds.x + 1.0,
                bounds.y + 1.0,
                bounds.w - 2.0,
                bounds.h - 2.0,
                3.0 + pulse,
                Color::new(1.0, 0.43 + pulse * 0.2, 0.13, 0.82 + pulse * 0.18),
            );
        }
        if legend_open {
            draw_legend_overlay(game, bounds, legend_label);
        }
    }

    fn draw_sidebar(
        &self,
        game: &GameState,
        rect: Rect,
        inspection: Option<&str>,
        visible_counts: (usize, usize, usize, usize, usize, usize),
        legend_label: &str,
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
        draw_line(
            rect.x - 12.0,
            rect.y,
            rect.x - 12.0,
            rect.y + rect.h,
            1.0,
            Color::from_rgba(32, 61, 72, 255),
        );
        draw_text("CARTOGRAPHIE", rect.x, rect.y + 14.0, 18.0, bright);
        draw_text(
            "Uniquement les zones explorées",
            rect.x,
            rect.y + 35.0,
            13.0,
            muted,
        );
        let pixel = (rect.w / game.map().width() as f32).floor().max(1.0);
        let origin = vec2(rect.x, rect.y + 48.0);
        draw_rectangle(
            origin.x,
            origin.y,
            game.map().width() as f32 * pixel,
            game.map().height() as f32 * pixel,
            Color::from_rgba(3, 8, 13, 255),
        );
        for (position, tile) in &self.remembered {
            let color = match (
                tile.terrain.blocks_movement(),
                game.player_visibility().is_visible(*position),
            ) {
                (true, true) => Color::from_rgba(119, 160, 167, 255),
                (true, false) => Color::from_rgba(43, 68, 78, 255),
                (false, true) => Color::from_rgba(36, 77, 78, 255),
                (false, false) => Color::from_rgba(18, 33, 44, 255),
            };
            draw_rectangle(
                origin.x + position.x as f32 * pixel,
                origin.y + position.y as f32 * pixel,
                pixel,
                pixel,
                color,
            );
        }
        if let Some(position) = game.player_position() {
            draw_rectangle(
                origin.x + position.x as f32 * pixel,
                origin.y + position.y as f32 * pixel,
                pixel,
                pixel,
                cyan,
            );
        }
        let mut y = origin.y + game.map().height() as f32 * pixel + 26.0;
        draw_text("INSPECTION", rect.x, y, 18.0, bright);
        y += 23.0;
        if let Some(description) = inspection {
            y = draw_wrapped_lines(description, rect.x, y, rect.w, 15, 3, muted);
        } else {
            draw_text("Survolez une case connue", rect.x, y, 15.0, muted);
            y += 19.0;
            draw_text("ou sélectionnez une cible.", rect.x, y, 15.0, muted);
            y += 19.0;
        }
        y += 12.0;
        if y + 86.0 < rect.y + rect.h {
            draw_text("DÉTECTIONS ACTUELLES", rect.x, y, 16.0, bright);
            y += 22.0;
            draw_text(
                format!(
                    "Hostiles {visible_hostiles}  ·  Neutres {visible_neutrals}  ·  Objets {visible_items}"
                ),
                rect.x,
                y,
                15.0,
                muted,
            );
            if visible_local_alerts > 0 {
                draw_text(
                    format!("Alertes locales visibles {visible_local_alerts}"),
                    rect.x,
                    y + 21.0,
                    15.0,
                    Color::from_rgba(255, 175, 83, 255),
                );
            }
            if visible_security_alarms > 0 {
                draw_text(
                    format!("Alarmes de sécurité visibles {visible_security_alarms}"),
                    rect.x,
                    y + 42.0,
                    15.0,
                    Color::from_rgba(255, 175, 83, 255),
                );
            }
            if visible_security_lockdowns > 0 {
                draw_text(
                    format!("Verrouillages visibles {visible_security_lockdowns}"),
                    rect.x,
                    y + 63.0,
                    15.0,
                    Color::from_rgba(255, 175, 83, 255),
                );
            }
        }
        draw_text(
            format!("{legend_label} · légende"),
            rect.x,
            rect.y + rect.h - 4.0,
            15.0,
            cyan,
        );
    }
}

fn legend_panel(bounds: Rect) -> Rect {
    Rect::new(
        bounds.x + (bounds.w - (bounds.w - 24.0).min(660.0)) * 0.5,
        bounds.y + (bounds.h - (bounds.h - 24.0).min(440.0)) * 0.5,
        (bounds.w - 24.0).min(660.0),
        (bounds.h - 24.0).min(440.0),
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
    draw_text(
        "LÉGENDE · TERMINAL À GLYPHES",
        panel.x + 22.0,
        panel.y + 38.0,
        25.0,
        bright,
    );

    let columns = [panel.x + 22.0, panel.x + panel.w * 0.5];
    let top = panel.y + 68.0;
    let available = (panel.h - 107.0).max(120.0);
    let row_height = (available / 12.0).min(38.0);
    let icon_size = (row_height - 7.0).max(20.0);
    for (index, (symbol, label, color, alerted, status_icon)) in [
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
            'c',
            "Badge ! · état de sécurité",
            Color::from_rgba(255, 175, 83, 255),
            true,
            None,
        ),
    ]
    .into_iter()
    .enumerate()
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
            panel.w * 0.5 - icon_size - 42.0,
            15,
            muted,
        );
    }
    for (index, (decor, label)) in [
        (Decor::Wall, "Cloison"),
        (Decor::DoorClosed, "Porte fermée"),
        (Decor::DoorLocked, "Porte verrouillée"),
        (Decor::DoorUnpowered, "Porte sans alimentation"),
        (Decor::ControlReady, "Console active"),
        (Decor::Depot, "Dépôt de maintenance"),
        (Decor::RelayOffline, "Relais en panne"),
        (Decor::SensorOffline, "Capteur hors ligne"),
        (
            Decor::Passage,
            if game.exit().is_some() {
                "Sortie"
            } else {
                "Passage interzone"
            },
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let y = top + index as f32 * row_height;
        draw_tile(
            Rect::new(columns[1], y - icon_size * 0.75, icon_size, icon_size),
            decor,
            [false; 4],
            true,
            GridPos::new(0, 0),
        );
        draw_bounded_text(
            label,
            columns[1] + icon_size + 9.0,
            y,
            panel.w * 0.5 - icon_size - 42.0,
            15,
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
        let y = top + (9 + offset) as f32 * row_height;
        let rect = Rect::new(columns[1], y - icon_size * 0.75, icon_size, icon_size);
        draw_attack_preview_area(rect, true, 0);
        if cursor {
            draw_attack_preview_cursor(rect, true);
        }
        draw_bounded_text(
            label,
            columns[1] + icon_size + 9.0,
            y,
            panel.w * 0.5 - icon_size - 42.0,
            15,
            muted,
        );
    }
    draw_text(
        format!("{legend_label} ou Échap · fermer"),
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
        if !line.is_empty() && measure_text(&candidate, None, size, 1.0).width > width {
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
    if measure_text(text, None, size, 1.0).width <= width {
        draw_text(text, x, y, f32::from(size), color);
        return;
    }
    let mut clipped = String::new();
    let suffix = "…";
    for character in text.chars() {
        let previous = clipped.len();
        clipped.push(character);
        if measure_text(&clipped, None, size, 1.0).width
            + measure_text(suffix, None, size, 1.0).width
            > width
        {
            clipped.truncate(previous);
            break;
        }
    }
    clipped.push_str(suffix);
    draw_text(&clipped, x, y, f32::from(size), color);
}

pub fn overlay_label(symbol: char) -> &'static str {
    match symbol {
        '@' => "Vous · noyau mobile",
        'c' => "Récupérateur neutre · transporte des matériaux",
        'm' => "Technicien neutre · entretient les installations",
        'd' => "Traqueur · hostile",
        't' => "Sentinelle · hostile",
        'r' => "Tirailleur · hostile",
        ')' => "Arme au sol",
        '!' => "Consommable au sol",
        '=' => "Matériau au sol",
        '>' => "Sortie du secteur",
        '.' | '-' | '*' | '+' | 'f' | 'F' | 'x' | '~' | 'a' => "Effet visuel en cours",
        '^' => "Feu au sol · dégâts thermiques persistants",
        's' => "Capteur de sécurité",
        _ => "Trace de déplacement",
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

fn terminal_grid_camera(
    game: &WorldState,
    bounds: Rect,
    preferred_cell: u16,
) -> (bool, GridCamera) {
    let sidebar = bounds.w >= 1050.0 && bounds.h >= 400.0;
    let map_bounds = Rect::new(
        bounds.x + 8.0,
        bounds.y + 34.0,
        bounds.w - if sidebar { 252.0 } else { 16.0 },
        bounds.h - 64.0,
    );
    let focus = game.player_position().unwrap_or(GridPos::new(12, 12));
    let effective_cell = fitted_cell_size(
        map_bounds,
        preferred_cell,
        game.rules().player_field_of_view.radius,
    );
    (sidebar, GridCamera::new(map_bounds, effective_cell, focus))
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
        Decor::Lane | Decor::Threshold => Color::from_rgba(34, 39, 37, 255),
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
    if kind == Decor::Wall {
        let edge = dim(Color::from_rgba(128, 165, 174, 255), visible);
        let inner = dim(Color::from_rgba(61, 88, 98, 255), visible);
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
        draw_rectangle(rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, 1.0, seam);
        draw_rectangle(rect.x + 1.0, rect.y + 1.0, 1.0, rect.h - 2.0, seam);
        match kind {
            Decor::Gravel => {
                for (dx, dy) in [(5, 7), (19, 16), (9, 24)] {
                    let px =
                        (dx + position.x * 7 + position.y * 3).rem_euclid(rect.w as i32 - 4) + 2;
                    let py =
                        (dy + position.x * 3 + position.y * 7).rem_euclid(rect.h as i32 - 4) + 2;
                    draw_rectangle(
                        rect.x + px as f32,
                        rect.y + py as f32,
                        2.0,
                        1.0,
                        dim(Color::from_rgba(79, 87, 78, 255), visible),
                    );
                }
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
        Decor::Depot => (&DEPOT, Color::from_rgba(200, 166, 105, 255)),
        Decor::RelayOffline => (&RELAY_OFFLINE, Color::from_rgba(180, 111, 92, 255)),
        Decor::RelayOnline => (&RELAY_ONLINE, Color::from_rgba(105, 211, 176, 255)),
        Decor::ActuatorOffline => (&ACTUATOR_OFFLINE, Color::from_rgba(116, 137, 145, 255)),
        Decor::ActuatorOnline => (&ACTUATOR_ONLINE, Color::from_rgba(112, 198, 190, 255)),
        Decor::SensorOffline => (&SENSOR_OFFLINE, Color::from_rgba(116, 137, 145, 255)),
        Decor::SensorOnline => (&SENSOR_ONLINE, Color::from_rgba(104, 207, 194, 255)),
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
    let pattern = match symbol {
        '@' => &PLAYER,
        'd' => &HUNTER,
        't' => &SENTRY,
        'r' => &SKIRMISHER,
        'c' => &RETRIEVER,
        'm' => &TECHNICIAN,
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
        's' => &SENSOR_ONLINE,
        '^' => &FLAME_LARGE,
        '→' => &TRACE_EAST,
        '↓' => &TRACE_SOUTH,
        '←' => &TRACE_WEST,
        _ => &TRACE,
    };
    draw_pixel_glyph_palette(rect, pattern, palette);
    if symbol == '@' {
        draw_rectangle_lines(
            rect.x + 1.0,
            rect.y + 1.0,
            rect.w - 2.0,
            rect.h - 2.0,
            1.0,
            palette.primary,
        );
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

fn draw_status_icon(rect: Rect, icon: TerminalStatusIcon) {
    let badge_size = (rect.w * 0.43).clamp(11.0, 15.0);
    let badge = Rect::new(
        rect.x + rect.w - badge_size + 1.0,
        rect.y + rect.h - badge_size + 1.0,
        badge_size,
        badge_size,
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
const PLAYER: PixelGlyph = [
    "...##...", ".######.", ".#++++#.", "##+##+##", "##+##+##", ".#++++#.", ".######.", "...##...",
];
const HUNTER: PixelGlyph = [
    ".#....#.", ".##..##.", "..####..", ".##++##.", ".######.", "..#..#..", ".##..##.", ".#....#.",
];
const SENTRY: PixelGlyph = [
    "...##...", "...##...", "..####..", ".##++##.", ".######.", "..####..", ".##..##.", "##....##",
];
const SKIRMISHER: PixelGlyph = [
    "......#.", "..##.##.", ".#####..", "##++##..", ".#####..", "..####..", ".##..##.", "##....##",
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
const FLAME_SMALL: PixelGlyph = [
    "........", "....#...", "...#+...", "...++#..", "..#+*#..", "..#**#..", "...##...", "........",
];
const FLAME: PixelGlyph = [
    "....#...", "...#+...", "..#++...", "..#+*#..", ".#+**+#.", ".#+***#.", "..#++#..", "...##...",
];
const FLAME_LARGE: PixelGlyph = [
    "...#....", "..#+#...", "..#++...", ".#+++#..", ".#+**+#.", "##+***##", ".#++**#.", "..####..",
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
const RETRIEVER: PixelGlyph = [
    "...##...", "..####..", ".##++##.", "######..", "#######.", "..####..", ".##..##.", "##....##",
];
const TECHNICIAN: PixelGlyph = [
    "....#...", "...##...", "..####..", ".##++##.", ".######.", "..####.#", ".##..##.", "##....##",
];
const MATERIAL: PixelGlyph = [
    "........", "..####..", ".##..##.", "##.##.##", "##.##.##", ".##..##.", "..####..", "........",
];

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::FieldOfViewRules;

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
    fn entity_silhouettes_are_distinct_without_color_and_glyphs_are_well_formed() {
        let entities = [
            PLAYER,
            HUNTER,
            SENTRY,
            SKIRMISHER,
            WEAPON,
            REPAIR,
            EXIT,
            EFFECT,
            FLAME_SMALL,
            FLAME,
            FLAME_LARGE,
            TRACE,
        ];
        for (index, glyph) in entities.iter().enumerate() {
            assert!(!entities[..index].contains(glyph));
        }
        for glyph in entities
            .into_iter()
            .chain([PILLAR, CRATE, SERVER, CONSOLE, COOLANT])
        {
            for row in glyph {
                assert_eq!(row.len(), 8);
                assert!(row.bytes().all(|b| b".#+*".contains(&b)));
            }
        }
    }
}
