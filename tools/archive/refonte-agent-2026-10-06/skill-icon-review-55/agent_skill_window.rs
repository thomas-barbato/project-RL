//! Native reader for the reviewed cryogenic-agent catalogue.
//! No simulation commands: unimplemented effects must not spend points or time.
use super::*;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct Catalogue {
    skills: Vec<Skill>,
    labels: BTreeMap<String, String>,
    art: BTreeMap<String, BTreeMap<String, String>>,
}
#[derive(Deserialize)]
struct Skill {
    id: String,
    title: String,
    branch: String,
    kind: String,
    roleplay: String,
    learned_cap: u8,
    requires: Option<String>,
    parameters: BTreeMap<String, serde_json::Value>,
}
fn catalogue() -> &'static Catalogue {
    static DATA: OnceLock<Catalogue> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/skills/agent/catalogue.json"))
            .expect("packaged reviewed skill catalogue")
    })
}
const BRANCHES: [&str; 17] = [
    "Mêlée",
    "Survie",
    "Drones",
    "Affaiblissements",
    "Attaques à distance",
    "Esquive",
    "Feu",
    "Froid",
    "Énergie occulte",
    "Transformations",
    "Pouvoirs hybrides",
    "Buffs de mêlée",
    "Buffs à distance",
    "Armes lourdes",
    "Furtivité",
    "Mines",
    "Grenades",
];
fn skills(branch: usize) -> Vec<&'static Skill> {
    catalogue()
        .skills
        .iter()
        .filter(|s| s.branch == BRANCHES[branch.min(16)])
        .collect()
}

fn curve(p: &serde_json::Value, rank: u8) -> f64 {
    let n = f64::from(rank.saturating_sub(1));
    let number = |key: &str| p[key].as_f64().unwrap_or(0.0);
    let mut value = match p["kind"].as_str().unwrap_or("") {
        "constant" => number("value"),
        "exponential_ratio" => (number("numerator") / number("denominator")).powi(i32::from(rank)),
        "stepped" => {
            number("base")
                + ((n + number("rank_offset")) / number("every_ranks")).floor()
                    * number("gain_per_step")
        }
        "scaled_linear_damage" => {
            (number("base") + n * number("gain_per_rank")) * number("numerator")
                / number("denominator")
        }
        "linear" => number("base") + n * number("gain_per_rank"),
        kind => panic!("unsupported packaged skill curve {kind}"),
    };
    if p["minimum"].is_number() {
        value = value.max(number("minimum"));
    }
    if p["maximum"].is_number() {
        value = value.min(number("maximum"));
    }
    value
}
fn value_label(key: &str, value: f64) -> String {
    if key == "detection_range_multiplier" {
        return format!("{} cases", (8.0 * value).ceil().max(1.0) as u32);
    }
    if key.ends_with("_percent") || key.ends_with("_points") {
        let number = if value.fract().abs() < 0.001 {
            format!("{value:.0}")
        } else {
            format!("{value:.1}").replace('.', ",")
        };
        return format!("{number} %");
    }
    let value = value.floor() as i64;
    let unit = if key.ends_with("_turns") {
        if value == 1 { " tour" } else { " tours" }
    } else if key.ends_with("_cells") {
        if value == 1 { " case" } else { " cases" }
    } else if key.ends_with("_kg") {
        " kg"
    } else if key.ends_with("_degrees") {
        "°"
    } else {
        ""
    };
    format!("{value}{unit}")
}

fn title_lines(title: &str, width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut line = String::new();
    for word in title.split_whitespace() {
        let candidate = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure_text_bold(&candidate, 13).width > width {
            lines.push(std::mem::take(&mut line));
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        lines.push(line);
    }
    if lines.len() > 3 {
        let tail = lines[2..].join(" ");
        lines.truncate(2);
        lines.push(tail);
    }
    lines
}

struct Layout {
    panel: Rect,
    branches: Rect,
    grid: Rect,
    detail: Rect,
    branch_rows: Vec<(usize, Rect)>,
    tiles: Vec<(usize, Rect)>,
    actions: [Rect; 3],
    columns: usize,
}
impl Layout {
    fn new(width: f32, height: f32, branch: usize, selected: usize, count: usize) -> Self {
        let panel = Rect::new(16.0, 16.0, width - 32.0, height - 32.0);
        let body = Rect::new(
            panel.x + 12.0,
            panel.y + 70.0,
            panel.w - 24.0,
            panel.h - 130.0,
        );
        let side = (body.w * 0.18).clamp(112.0, 208.0);
        let detail_width = (body.w * 0.36).clamp(240.0, 440.0);
        let branches = Rect::new(body.x, body.y, side, body.h);
        let detail = Rect::new(body.right() - detail_width, body.y, detail_width, body.h);
        let grid = Rect::new(
            branches.right() + 10.0,
            body.y,
            detail.x - branches.right() - 20.0,
            body.h,
        );
        let branch_height = 23.0;
        let branch_visible = ((branches.h - 34.0) / branch_height).floor().max(1.0) as usize;
        let first_branch = branch.saturating_sub(branch_visible.saturating_sub(1));
        let branch_rows = (first_branch..(first_branch + branch_visible).min(BRANCHES.len()))
            .enumerate()
            .map(|(row, index)| {
                (
                    index,
                    Rect::new(
                        branches.x + 5.0,
                        branches.y + 32.0 + row as f32 * branch_height,
                        branches.w - 10.0,
                        22.0,
                    ),
                )
            })
            .collect();
        let columns = ((grid.w - 12.0) / 124.0).floor().max(1.0) as usize;
        let visible_rows = ((grid.h - 42.0) / 148.0).floor().max(1.0) as usize;
        let selected_row = selected / columns;
        let first_row = selected_row.saturating_sub(visible_rows.saturating_sub(1));
        let first = first_row * columns;
        let stride = (grid.w - 12.0) / columns as f32;
        let tiles = (first..(first + visible_rows * columns).min(count))
            .enumerate()
            .map(|(visible, index)| {
                (
                    index,
                    Rect::new(
                        grid.x + 6.0 + (visible % columns) as f32 * stride,
                        grid.y + 36.0 + (visible / columns) as f32 * 148.0,
                        stride,
                        144.0,
                    ),
                )
            })
            .collect();
        let button_width = (detail.right() - grid.x - 16.0) / 3.0;
        let actions = std::array::from_fn(|index| {
            Rect::new(
                grid.x + index as f32 * (button_width + 8.0),
                panel.bottom() - 44.0,
                button_width,
                32.0,
            )
        });
        Self {
            panel,
            branches,
            grid,
            detail,
            branch_rows,
            tiles,
            actions,
            columns,
        }
    }
    fn icon(tile: Rect) -> Rect {
        Rect::new(tile.x + (tile.w - 64.0) * 0.5, tile.y, 64.0, 64.0)
    }
}

thread_local! {
    static ART: std::cell::RefCell<BTreeMap<(String, String), Texture2D>> = std::cell::RefCell::new(BTreeMap::new());
}
// The bank contains only accepted assets, loaded once rather than per frame.
fn art(id: &str, category: &str, rect: Rect, clip: Option<Rect>) -> bool {
    let Some(filename) = catalogue()
        .art
        .get(id)
        .and_then(|entry| entry.get(category))
    else {
        return false;
    };
    ART.with_borrow_mut(|cache| {
        let texture = cache
            .entry((id.to_owned(), category.to_owned()))
            .or_insert_with(|| {
                let path = if id == "T01" && category == "compact" {
                    crate::skill_art_review::mark_reticle_preview_path()
                } else {
                    None
                }
                .unwrap_or_else(|| {
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("assets/skills/agent")
                        .join(filename)
                });
                let bytes = std::fs::read(path).expect("packaged accepted skill drawing");
                let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
                texture.set_filter(FilterMode::Linear);
                texture
            });
        let visible = clip.map_or(rect, |clip| {
            rect.intersect(clip)
                .unwrap_or(Rect::new(0.0, 0.0, 0.0, 0.0))
        });
        if visible.w <= 0.0 || visible.h <= 0.0 {
            return;
        }
        let source = Rect::new(
            (visible.x - rect.x) / rect.w * texture.width(),
            (visible.y - rect.y) / rect.h * texture.height(),
            visible.w / rect.w * texture.width(),
            visible.h / rect.h * texture.height(),
        );
        draw_texture_ex(
            texture,
            visible.x,
            visible.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(visible.w, visible.h)),
                source: Some(source),
                ..Default::default()
            },
        );
    });
    true
}

impl AsciiApp {
    pub(super) fn uses_agent_skill_window(&self) -> bool {
        self.game.rules().agent_class_start.is_some() || self.test_lab
    }

    pub(super) fn clamp_agent_skill_selection(&mut self) {
        self.skill_discipline_selection = self.skill_discipline_selection.min(16);
        self.skill_technique_selection = self.skill_technique_selection.min(
            skills(self.skill_discipline_selection)
                .len()
                .saturating_sub(1),
        );
    }

    pub(super) fn open_agent_skill_window(&mut self) {
        self.technique_menu_open = false;
        self.skills_open = true;
        self.inventory_open = false;
        self.character_open = false;
        self.attack_aim = None;
        self.attack_aim_technique = None;
        self.attack_aim_pointer = None;
        self.skill_message.clear();
        self.menu_focus.reset();
        if let Some(first) = self
            .game
            .rules()
            .agent_class_start
            .as_ref()
            .and_then(|s| s.skills.first())
            && let Some(skill) = catalogue().skills.iter().find(|s| &s.id == first)
        {
            self.skill_discipline_selection = BRANCHES
                .iter()
                .position(|b| *b == skill.branch)
                .unwrap_or(0);
            self.skill_technique_selection = skills(self.skill_discipline_selection)
                .iter()
                .position(|s| s.id == skill.id)
                .unwrap_or(0);
        }
        self.ux.skill_scroll.offset = 0.0;
    }

    #[cfg(debug_assertions)]
    pub(super) fn select_agent_skill_for_capture(&mut self, id: &str) -> Result<(), String> {
        let skill = catalogue()
            .skills
            .iter()
            .find(|skill| skill.id == id)
            .ok_or_else(|| format!("Unknown agent capture skill: {id}"))?;
        self.skill_discipline_selection = BRANCHES
            .iter()
            .position(|branch| *branch == skill.branch)
            .expect("packaged skill branch");
        self.skill_technique_selection = skills(self.skill_discipline_selection)
            .iter()
            .position(|entry| entry.id == id)
            .expect("packaged skill in its branch");
        self.ux.skill_scroll.offset = 0.0;
        Ok(())
    }

    pub(super) fn update_agent_skills(&mut self, input: &InputFrame) {
        self.skill_discipline_selection = self.skill_discipline_selection.min(16);
        let entries = skills(self.skill_discipline_selection);
        self.skill_technique_selection = self
            .skill_technique_selection
            .min(entries.len().saturating_sub(1));
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = Layout::new(
            width,
            height,
            self.skill_discipline_selection,
            self.skill_technique_selection,
            entries.len(),
        );
        let click = input.pressed.contains(&controls::Binding::MouseLeft);
        if self.controls.pressed(Action::QuickTechniques, input)
            || click
                && input
                    .pointer
                    .is_some_and(|p| layout.actions[2].contains(p.into()))
        {
            self.skills_open = false;
            self.level_up_notice = None;
            return;
        }
        let initial = (
            self.skill_discipline_selection,
            self.skill_technique_selection,
        );
        if click
            && let Some((index, _)) = layout
                .branch_rows
                .iter()
                .find(|(_, r)| input.pointer.is_some_and(|p| r.contains(p.into())))
        {
            self.skill_discipline_selection = *index;
            self.skill_technique_selection = 0;
        } else if click
            && let Some((index, _)) = layout
                .tiles
                .iter()
                .find(|(_, r)| input.pointer.is_some_and(|p| r.contains(p.into())))
        {
            self.skill_technique_selection = *index;
        } else if self.controls.pressed(Action::MenuLeft, input) {
            self.skill_discipline_selection = self.skill_discipline_selection.saturating_sub(1);
            self.skill_technique_selection = 0;
        } else if self.controls.pressed(Action::MenuRight, input) {
            self.skill_discipline_selection = (self.skill_discipline_selection + 1).min(16);
            self.skill_technique_selection = 0;
        } else if self.controls.pressed(Action::MenuUp, input) {
            self.skill_technique_selection = self.skill_technique_selection.saturating_sub(1);
        } else if self.controls.pressed(Action::MenuDown, input) {
            self.skill_technique_selection =
                (self.skill_technique_selection + 1).min(entries.len().saturating_sub(1));
        } else if input.wheel_y != 0.0 {
            if input
                .pointer
                .is_some_and(|p| layout.detail.contains(p.into()))
            {
                self.ux.skill_scroll.update(input, &self.controls);
            } else if input
                .pointer
                .is_some_and(|p| layout.branches.contains(p.into()))
            {
                self.skill_discipline_selection = if input.wheel_y > 0.0 {
                    self.skill_discipline_selection.saturating_sub(1)
                } else {
                    (self.skill_discipline_selection + 1).min(16)
                };
                self.skill_technique_selection = 0;
            } else if input
                .pointer
                .is_some_and(|p| layout.grid.contains(p.into()))
            {
                let steps = wheel_steps(input.wheel_y) * layout.columns;
                self.skill_technique_selection = if input.wheel_y > 0.0 {
                    self.skill_technique_selection.saturating_sub(steps)
                } else {
                    (self.skill_technique_selection + steps).min(entries.len().saturating_sub(1))
                };
            }
        } else if input.pressed.contains(&controls::Binding::key("PageDown"))
            || input.pressed.contains(&controls::Binding::key("PageUp"))
        {
            self.ux.skill_scroll.update(input, &self.controls);
        }
        if initial
            != (
                self.skill_discipline_selection,
                self.skill_technique_selection,
            )
        {
            self.ux.skill_scroll.offset = 0.0;
        }
        // Learn, Use and their mouse buttons intentionally issue no commands.
        // Reading definitions must never grant ranks with unapplied effects.
    }

    pub(super) fn draw_agent_skills(&self) {
        let branch = self.skill_discipline_selection.min(16);
        let entries = skills(branch);
        let selected = self
            .skill_technique_selection
            .min(entries.len().saturating_sub(1));
        let layout = Layout::new(
            self.ui_width(),
            self.ui_height(),
            branch,
            selected,
            entries.len(),
        );
        let text = UiTheme.text();
        let muted = UiTheme.muted();
        let accent = UiTheme.accent();
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 225),
        );
        UiTheme.panel(layout.panel);
        draw_text_bold(
            "COMPÉTENCES",
            layout.panel.x + 18.0,
            layout.panel.y + 31.0,
            23.0,
            text,
        );
        draw_text_bold_centered(
            format!(
                "{} POINTS DISPONIBLES",
                self.game.player_progression().unspent_skill_points()
            ),
            Rect::new(
                layout.panel.right() - 274.0,
                layout.panel.y + 12.0,
                250.0,
                28.0,
            ),
            16,
            accent,
        );
        draw_text(
            "Technique à venir",
            layout.panel.x + 18.0,
            layout.panel.y + 55.0,
            14.0,
            muted,
        );
        for panel in [layout.branches, layout.grid, layout.detail] {
            UiTheme.card(panel, false);
        }
        draw_text(
            "BRANCHES",
            layout.branches.x + 8.0,
            layout.branches.y + 23.0,
            14.0,
            muted,
        );
        for (index, row) in &layout.branch_rows {
            if *index == branch {
                draw_rectangle(row.x, row.y, row.w, row.h, UiTheme.surface_selected());
            }
            crate::ui_theme::draw_text_in_rect(
                BRANCHES[*index],
                *row,
                14,
                if *index == branch { accent } else { text },
            );
        }
        if layout.branch_rows.len() < BRANCHES.len() {
            let track = Rect::new(
                layout.branches.right() - 3.0,
                layout.branches.y + 32.0,
                2.0,
                layout.branches.h - 40.0,
            );
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            let thumb = track.h * layout.branch_rows.len() as f32 / BRANCHES.len() as f32;
            let first = layout.branch_rows.first().map_or(0, |(index, _)| *index);
            let maximum = BRANCHES.len() - layout.branch_rows.len();
            draw_rectangle(
                track.x,
                track.y + (track.h - thumb) * first as f32 / maximum as f32,
                track.w,
                thumb,
                accent,
            );
        }
        crate::ui_theme::draw_text_in_rect(
            BRANCHES[branch],
            Rect::new(
                layout.grid.x + 8.0,
                layout.grid.y + 6.0,
                layout.grid.w - 16.0,
                24.0,
            ),
            18,
            text,
        );
        for (index, tile) in &layout.tiles {
            let skill = entries[*index];
            let icon = Layout::icon(*tile);
            if !art(&skill.id, "compact", icon, None) {
                // An ID is a neutral placeholder, never unapproved substitute art.
                draw_text_bold_centered(&skill.id, icon, 18, muted);
            }
            if *index == selected {
                draw_rectangle_lines(
                    icon.x - 2.0,
                    icon.y - 2.0,
                    icon.w + 4.0,
                    icon.h + 4.0,
                    1.0,
                    accent,
                );
            }
            let rank = self.game.player_skills().agent_skill_rank(&skill.id);
            draw_text_bold(
                rank.to_string(),
                icon.right() + 6.0,
                icon.y + 38.0,
                17.0,
                if rank > 0 { text } else { muted },
            );
            // The name is centered on the icon, without underlining or selection color.
            let lines = title_lines(&skill.title, tile.w - 12.0);
            let mut y = icon.bottom() + 6.0;
            for line in &lines {
                draw_text_bold_centered(
                    line,
                    Rect::new(tile.x + 6.0, y, tile.w - 12.0, 17.0),
                    13,
                    text,
                );
                y += 17.0;
            }
            draw_text_bold_centered(
                if skill.kind == "active" {
                    "Actif"
                } else {
                    "Passif"
                },
                Rect::new(tile.x + 6.0, y + 2.0, tile.w - 12.0, 14.0),
                11,
                muted,
            );
        }
        if layout.tiles.len() < entries.len() {
            let track = Rect::new(
                layout.grid.right() - 3.0,
                layout.grid.y + 36.0,
                2.0,
                layout.grid.h - 44.0,
            );
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            let total_rows = entries.len().div_ceil(layout.columns);
            let visible_rows = layout.tiles.len().div_ceil(layout.columns);
            let thumb = track.h * visible_rows as f32 / total_rows as f32;
            let first_row = layout
                .tiles
                .first()
                .map_or(0, |(index, _)| *index / layout.columns);
            let maximum = total_rows.saturating_sub(visible_rows).max(1);
            draw_rectangle(
                track.x,
                track.y + (track.h - thumb) * first_row as f32 / maximum as f32,
                track.w,
                thumb,
                accent,
            );
        }
        if let Some(skill) = entries.get(selected) {
            self.draw_agent_skill_detail(skill, layout.detail);
        }
        for (index, label) in ["Apprendre · 1 point", "Utiliser", "Fermer"]
            .iter()
            .enumerate()
        {
            UiTheme.button(
                layout.actions[index],
                label,
                false,
                false,
                index == 2,
                ButtonTone::Secondary,
            );
        }
    }

    fn draw_agent_skill_detail(&self, skill: &Skill, panel: Rect) {
        let rank = self.game.player_skills().agent_skill_rank(&skill.id);
        let effective = rank.max(1);
        let pane = Rect::new(
            panel.x + 10.0,
            panel.y + 8.0,
            panel.w - 20.0,
            panel.h - 16.0,
        );
        let x = pane.x + 2.0;
        let width = pane.w - 12.0;
        let offset = self.ux.skill_scroll.offset;
        crate::ui_theme::begin_text_pane(pane, offset);
        let mut y = draw_wrapped_text(&skill.title, x, pane.y + 23.0, width, 3, 20, UiTheme.text());
        draw_text(
            format!(
                "{} · Rang {}{}",
                if skill.kind == "active" {
                    "Actif"
                } else {
                    "Passif"
                },
                effective,
                if rank == 0 { " · aperçu" } else { "" }
            ),
            x,
            y + 14.0,
            13.0,
            UiTheme.muted(),
        );
        y = draw_wrapped_text(&skill.roleplay, x, y + 42.0, width, 16, 15, UiTheme.text());
        if let Some(required) = skill
            .requires
            .as_ref()
            .and_then(|id| catalogue().skills.iter().find(|s| &s.id == id))
        {
            y = draw_wrapped_text(
                &format!("{} apprise requise.", required.title),
                x,
                y + 20.0,
                width,
                4,
                13,
                UiTheme.muted(),
            );
        }
        if skill.kind == "active" {
            let weapon_requirement = match skill.branch.as_str() {
                "Mêlée" => Some("Arme de mêlée active requise."),
                "Attaques à distance" | "Armes lourdes" => Some("Arme à distance active requise."),
                _ => None,
            };
            if let Some(requirement) = weapon_requirement {
                y = draw_wrapped_text(requirement, x, y + 20.0, width, 3, 13, UiTheme.muted());
            }
        }
        y = self.draw_agent_skill_values(skill, effective, None, x, y + 28.0, width, "Effet");
        if rank < skill.learned_cap {
            let next = rank.saturating_add(1).max(1);
            if rank > 0 {
                y = self.draw_agent_skill_values(
                    skill,
                    next,
                    Some(effective),
                    x,
                    y + 22.0,
                    width,
                    "Prochain rang",
                );
            }
        } else {
            draw_text("Plafond atteint", x, y + 24.0, 14.0, UiTheme.muted());
            y += 28.0;
        }
        crate::ui_theme::end_text_pane();
        self.ux.skill_scroll.finish(pane, y + 12.0);
    }

    fn draw_agent_skill_values(
        &self,
        skill: &Skill,
        rank: u8,
        previous: Option<u8>,
        x: f32,
        mut y: f32,
        width: f32,
        heading: &str,
    ) -> f32 {
        let hidden = [
            "positive_payable_cost_floor",
            "application_power_rank_base",
            "minimum_detection_cells",
        ];
        let values: Vec<_> = skill
            .parameters
            .iter()
            .filter(|(key, p)| {
                !hidden.contains(&key.as_str())
                    && previous.is_none_or(|old| (curve(p, old) - curve(p, rank)).abs() > 0.00001)
            })
            .collect();
        if values.is_empty() {
            return y;
        }
        draw_text_bold(heading, x, y, 15.0, UiTheme.text());
        y += 24.0;
        for (key, p) in values {
            let value = curve(p, rank);
            let value = if p["current_level_binding"].as_str().is_some() {
                value
                    * (1.0
                        + f64::from(self.game.player_progression().level().saturating_sub(1))
                            / 10.0)
            } else {
                value
            };
            let label = &catalogue().labels[key];
            y = draw_wrapped_text(
                &format!("{label} : {}", value_label(key, value)),
                x,
                y,
                width,
                4,
                14,
                UiTheme.text(),
            ) + 7.0;
        }
        y
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reviewed_catalogue_is_complete_and_all_curves_are_finite() {
        let data = catalogue();
        assert_eq!(data.skills.len(), 94);
        assert_eq!(
            data.art
                .values()
                .filter(|a| a.contains_key("compact"))
                .count(),
            94
        );
        assert_eq!(
            data.art
                .values()
                .filter(|a| a.contains_key("illustration"))
                .count(),
            16
        );
        let ids: std::collections::BTreeSet<_> = data.skills.iter().map(|s| &s.id).collect();
        assert_eq!(ids.len(), 94);
        for skill in &data.skills {
            assert!(BRANCHES.contains(&skill.branch.as_str()));
            assert!(skill.requires.as_ref().is_none_or(|id| ids.contains(id)));
            for (key, parameter) in &skill.parameters {
                assert!(data.labels.contains_key(key));
                for rank in [1, 2, 10, 11, 20] {
                    assert!(curve(parameter, rank).is_finite(), "{} {key}", skill.id);
                }
            }
        }
    }
    #[test]
    fn native_skill_layout_keeps_selected_branch_tile_and_buttons_inside_panels() {
        let contains = |outer: Rect, inner: Rect| {
            inner.x >= outer.x
                && inner.y >= outer.y
                && inner.right() <= outer.right() + 0.01
                && inner.bottom() <= outer.bottom() + 0.01
        };
        for (width, height) in [
            (684.0, 460.0),
            (960.0, 540.0),
            (1280.0, 800.0),
            (1920.0, 1080.0),
        ] {
            for branch in 0..BRANCHES.len() {
                for selection in 0..skills(branch).len() {
                    let layout =
                        Layout::new(width, height, branch, selection, skills(branch).len());
                    assert!(contains(Rect::new(0.0, 0.0, width, height), layout.panel));
                    for panel in [layout.branches, layout.grid, layout.detail] {
                        assert!(contains(layout.panel, panel));
                    }
                    assert!(layout.branch_rows.iter().any(|(index, _)| *index == branch));
                    assert!(layout.tiles.iter().any(|(index, _)| *index == selection));
                    for (_, row) in &layout.branch_rows {
                        assert!(contains(layout.branches, *row));
                    }
                    for (_, tile) in &layout.tiles {
                        assert!(contains(layout.grid, *tile));
                    }
                    for button in layout.actions {
                        assert!(contains(layout.panel, button));
                    }
                }
            }
        }
    }

    #[test]
    fn test_room_without_a_class_opens_new_catalogue_and_disabled_buttons_are_readers() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.test_lab = true;
        assert!(app.game.rules().agent_class_start.is_none());
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        app.open_technique_menu();
        assert!(app.skills_open);
        assert!(!app.technique_menu_open);
        let layout = Layout::new(
            1280.0,
            800.0,
            app.skill_discipline_selection,
            app.skill_technique_selection,
            skills(app.skill_discipline_selection).len(),
        );
        for button in &layout.actions[..2] {
            app.update_agent_skills(&InputFrame {
                pointer: Some((button.x + 5.0, button.y + 5.0)),
                pressed: [controls::Binding::MouseLeft].into(),
                ..Default::default()
            });
        }
        assert!(app.skills_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
    }
}
