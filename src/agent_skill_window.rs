//! Native reader for the reviewed cryogenic-agent catalogue.
//! Rank purchases use recorded engine commands; combat effects are integrated separately.
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
    #[serde(default)]
    common_rules: Vec<String>,
    learned_cap: u8,
    requires: Option<String>,
    parameters: BTreeMap<String, serde_json::Value>,
    centred_damage: Option<project_rl::skills::area_damage::CentredDamageBinding>,
}
pub(super) fn parameter_label(key: &str) -> Option<&'static str> {
    catalogue().labels.get(key).map(String::as_str)
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
    if key == "taunt_square_side_cells" {
        return format!("{0:.0} × {0:.0} cases", value);
    }
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
    close: Rect,
    columns: usize,
    demo: Option<Rect>,
}
impl Layout {
    fn new(width: f32, height: f32, branch: usize, selected: usize, count: usize) -> Self {
        Self::with_demo(width, height, branch, selected, count, 0.)
    }
    fn with_demo(
        width: f32,
        height: f32,
        branch: usize,
        selected: usize,
        count: usize,
        demo_height: f32,
    ) -> Self {
        let panel = Rect::new(16.0, 16.0, width - 32.0, height - 32.0);
        let body = Rect::new(
            panel.x + 12.0,
            panel.y + 70.0,
            panel.w - 24.0,
            panel.h - 194.0,
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
        // Leave room for the rightmost purchase button without moving the icons
        // or changing the centering of their existing captions.
        let columns = ((grid.w - 24.0) / 124.0).floor().max(1.0) as usize;
        let visible_rows = ((grid.h - 42.0 - demo_height) / 148.0).floor().max(1.0) as usize;
        let selected_row = selected / columns;
        let first_row = selected_row.saturating_sub(visible_rows.saturating_sub(1));
        let first = first_row * columns;
        let stride = (grid.w - 24.0) / columns as f32;
        let tiles: Vec<_> = (first..(first + visible_rows * columns).min(count))
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
        let close = super::agent_skill_bar::skill_window_close_rect(width, height);
        let demo = (demo_height > 0.).then(|| {
            let w = (grid.w - 20.).min(480.);
            let rows = tiles.len().div_ceil(columns);
            Rect::new(
                grid.x + (grid.w - w) * 0.5,
                grid.y + 36. + rows as f32 * 148. + 8.,
                w,
                demo_height - 14.,
            )
        });
        Self {
            panel,
            branches,
            grid,
            detail,
            branch_rows,
            tiles,
            close,
            columns,
            demo,
        }
    }
    fn icon(tile: Rect) -> Rect {
        Rect::new(tile.x + (tile.w - 64.0) * 0.5, tile.y, 64.0, 64.0)
    }
    fn learn_button(tile: Rect) -> Rect {
        let icon = Self::icon(tile);
        Rect::new(icon.right() + 8., icon.y + 16., 32., 32.)
    }
}

thread_local! {
    static ART: std::cell::RefCell<BTreeMap<(String, String), Texture2D>> = std::cell::RefCell::new(BTreeMap::new());
}
// The bank contains only accepted assets, loaded once rather than per frame.
pub(super) fn art(id: &str, category: &str, rect: Rect, clip: Option<Rect>) -> bool {
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
                let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/skills/agent")
                    .join(filename);
                let bytes = std::fs::read(path).expect("packaged accepted skill drawing");
                let texture = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
                texture.set_filter(FilterMode::Nearest);
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
    fn agent_skills_layout(
        &self,
        width: f32,
        height: f32,
        branch: usize,
        selected: usize,
        count: usize,
    ) -> Layout {
        if self.ux.skill_demo_request.is_some() && height >= 600. {
            let reserve = (height - 32. - 194. - 190.).clamp(0., 236.);
            return Layout::with_demo(width, height, branch, selected, count, reserve);
        }
        Layout::new(width, height, branch, selected, count)
    }
    #[cfg(debug_assertions)]
    pub(super) fn agent_skill_demo_rect(&self) -> Option<Rect> {
        self.agent_skills_layout(
            self.ui_width(),
            self.ui_height(),
            self.skill_discipline_selection,
            self.skill_technique_selection,
            skills(self.skill_discipline_selection).len(),
        )
        .demo
    }
    pub(super) fn agent_skill_use_profile(
        &self,
        id: &str,
    ) -> Result<project_rl::game::AgentSkillUseProfile, CommandRejection> {
        let slot = self.game.player_weapon_slot_for_technique(
            self.active_weapon_slot,
            &project_rl::skills::agent::technique(id),
        );
        self.game.agent_skill_ready_with_weapon(id, slot)
    }

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
        self.ux.skill_demo_request = None;
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

    fn agent_device_variant_choices(
        &self,
        id: &str,
    ) -> Vec<(&'static str, bool, bool, GameCommand)> {
        use project_rl::game::{AgentGrenadeVariant as Grenade, AgentMineVariant as Mine};
        let learned = self.game.player_agent_skill_rank(id) > 0;
        if id == "SM00" {
            [
                ("Feu", Mine::Fire, "SM00"),
                ("Poison", Mine::Poison, "SM02"),
                ("Froid", Mine::Cold, "SM03"),
            ]
            .into_iter()
            .map(|(name, variant, required)| {
                (
                    name,
                    learned && self.game.player_agent_skill_rank(required) > 0,
                    self.game.player_agent_mine_variant() == variant,
                    GameCommand::SetAgentMineVariant { variant },
                )
            })
            .collect()
        } else if id == "SG00" {
            [
                ("Explosive", Grenade::Explosive, "SG00"),
                ("Aveuglante", Grenade::Blinding, "SG01"),
                ("Leurre", Grenade::Decoy, "SG02"),
            ]
            .into_iter()
            .map(|(name, variant, required)| {
                (
                    name,
                    learned && self.game.player_agent_skill_rank(required) > 0,
                    self.game.player_agent_grenade_variant() == variant,
                    GameCommand::SetAgentGrenadeVariant { variant },
                )
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    fn agent_device_variant_rect(panel: Rect, index: usize) -> Rect {
        let width = (panel.w - 28.0) / 3.0;
        Rect::new(
            panel.x + 10.0 + index as f32 * (width + 4.0),
            panel.y + 8.0,
            width,
            28.0,
        )
    }

    pub(super) fn update_agent_skills(&mut self, input: &InputFrame) {
        self.ux.skill_learn_pointer = input.pointer;
        self.ux.skill_learn_down = input.mouse_left_down;
        self.skill_discipline_selection = self.skill_discipline_selection.min(16);
        let entries = skills(self.skill_discipline_selection);
        self.skill_technique_selection = self
            .skill_technique_selection
            .min(entries.len().saturating_sub(1));
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = self.agent_skills_layout(
            width,
            height,
            self.skill_discipline_selection,
            self.skill_technique_selection,
            entries.len(),
        );
        let click = input.pressed.contains(&controls::Binding::MouseLeft);
        // Resolve purchase controls before the tile hit area: the button can
        // extend into the gutter, and one press must issue at most one purchase.
        if click
            && let Some((index, _)) = layout.tiles.iter().find(|(_, r)| {
                input
                    .pointer
                    .is_some_and(|p| Layout::learn_button(*r).contains(p.into()))
            })
        {
            if let Some(skill) = entries.get(*index) {
                self.skill_technique_selection = *index;
                if self.ux.skill_demo_request.as_deref() != Some(skill.id.as_str()) {
                    self.ux.skill_demo_request = None;
                }
                self.ux.skill_scroll.offset = 0.;
                if self.game.agent_skill_next_rank(&skill.id).is_ok() {
                    self.execute_command(GameCommand::LearnAgentSkill {
                        skill: skill.id.clone(),
                    });
                }
            }
            return;
        }
        if click && let Some(skill) = entries.get(self.skill_technique_selection) {
            for (index, (_, enabled, _, command)) in self
                .agent_device_variant_choices(&skill.id)
                .into_iter()
                .enumerate()
            {
                if input.pointer.is_some_and(|p| {
                    Self::agent_device_variant_rect(layout.detail, index).contains(p.into())
                }) {
                    if enabled {
                        self.execute_command(command);
                    }
                    return;
                }
            }
        }
        if input.mouse_left_down
            && click
            && let Some((index, _)) = layout.tiles.iter().find(|(_, r)| {
                input
                    .pointer
                    .is_some_and(|p| Layout::icon(*r).contains(p.into()))
            })
            && let Some(skill) = entries.get(*index)
            && self.can_assign_agent_skill(&skill.id)
        {
            self.skill_technique_selection = *index;
            self.ux.skill_demo_request = Some(skill.id.clone());
            self.begin_agent_skill_drag(&skill.id, None, input.pointer.unwrap());
            return;
        }
        if self.controls.pressed(Action::Use, input)
            && let Some(skill) = entries.get(self.skill_technique_selection)
        {
            if let Some(reason) = self
                .quick_technique_status(&project_rl::skills::agent::technique(&skill.id))
                .1
            {
                self.skill_message = reason;
                return;
            }
            self.begin_technique_attack_aim(project_rl::skills::agent::technique(&skill.id));
            if self.attack_aim.is_some() {
                self.skills_open = false;
                self.level_up_notice = None;
            }
            return;
        }
        if self.controls.pressed(Action::QuickTechniques, input)
            || click
                && input
                    .pointer
                    .is_some_and(|p| layout.close.contains(p.into()))
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
            && let Some((index, tile)) = layout
                .tiles
                .iter()
                .find(|(_, r)| input.pointer.is_some_and(|p| r.contains(p.into())))
        {
            self.skill_technique_selection = *index;
            self.ux.skill_demo_request = if entries[*index].kind == "active"
                && input
                    .pointer
                    .is_some_and(|p| Layout::icon(*tile).contains(p.into()))
            {
                Some(entries[*index].id.clone())
            } else {
                None
            };
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
            let selected = skills(self.skill_discipline_selection)
                .get(self.skill_technique_selection)
                .map(|s| s.id.as_str());
            if self.ux.skill_demo_request.as_deref() != selected {
                self.ux.skill_demo_request = None;
            }
        }
        if self.controls.pressed(Action::Learn, input)
            && let Some(skill) =
                skills(self.skill_discipline_selection).get(self.skill_technique_selection)
            && self.game.agent_skill_next_rank(&skill.id).is_ok()
        {
            self.execute_command(GameCommand::LearnAgentSkill {
                skill: skill.id.clone(),
            });
        }
    }

    pub(super) fn draw_agent_skills(&self) {
        let branch = self.skill_discipline_selection.min(16);
        let entries = skills(branch);
        let selected = self
            .skill_technique_selection
            .min(entries.len().saturating_sub(1));
        let layout = self.agent_skills_layout(
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
            &self.skill_message,
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
            let rank = self.game.player_agent_skill_rank(&skill.id);
            let learn = Layout::learn_button(*tile);
            let enabled = self.game.agent_skill_next_rank(&skill.id).is_ok();
            let hovered = self
                .ux
                .skill_learn_pointer
                .is_some_and(|p| learn.contains(p.into()));
            use crate::skill_learn_button::State;
            let state =
                if self.game.player_skills().agent_skill_rank(&skill.id) >= skill.learned_cap {
                    State::Capped
                } else if !enabled {
                    State::Disabled
                } else if hovered && self.ux.skill_learn_down {
                    State::Pressed
                } else if hovered {
                    State::Hover
                } else {
                    State::Ready
                };
            crate::skill_learn_button::draw(learn, state);
            draw_text_bold_centered(
                &rank.to_string(),
                Rect::new(learn.x, icon.y + 51., learn.w, 14.),
                12,
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
                layout
                    .demo
                    .map_or(layout.grid.h - 44., |r| r.y - layout.grid.y - 44.),
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
        self.draw_agent_skill_bar(true);
        if let Some(rect) = layout.demo {
            self.ux.skill_demo.draw(self, rect);
        }
    }

    fn draw_agent_skill_detail(&self, skill: &Skill, panel: Rect) {
        let rank = self.game.player_skills().agent_skill_rank(&skill.id);
        let effective = self.game.player_agent_skill_rank(&skill.id).max(1);
        let variants = self.agent_device_variant_choices(&skill.id);
        for (index, (label, enabled, selected, _)) in variants.iter().enumerate() {
            UiTheme.button(
                Self::agent_device_variant_rect(panel, index),
                label,
                *selected,
                false,
                *enabled,
                ButtonTone::Secondary,
            );
        }
        let variant_height = if variants.is_empty() { 0.0 } else { 38.0 };
        let pane = Rect::new(
            panel.x + 10.0,
            panel.y + 8.0 + variant_height,
            panel.w - 20.0,
            panel.h - 16.0 - variant_height,
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
                if self.game.player_agent_skill_rank(&skill.id) == 0 {
                    " · aperçu"
                } else {
                    ""
                }
            ),
            x,
            y + 14.0,
            13.0,
            UiTheme.muted(),
        );
        y = draw_wrapped_text(&skill.roleplay, x, y + 42.0, width, 16, 15, UiTheme.text());
        for rule in &skill.common_rules {
            y = draw_wrapped_text(rule, x, y + 7., width, 4, 14, UiTheme.text());
        }
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
                "Mêlée" => Some("Arme de mêlée en main requise."),
                "Attaques à distance" | "Armes lourdes" => {
                    Some("Arme à distance en main requise.")
                }
                _ => None,
            };
            if let Some(requirement) = weapon_requirement {
                y = draw_wrapped_text(requirement, x, y + 20.0, width, 3, 13, UiTheme.muted());
            }
        }
        y = self.draw_agent_skill_values(skill, effective, None, x, y + 28.0, width, "Effet");
        let variant = match skill.id.as_str() {
            "SM00" => Some(self.game.player_agent_mine_variant().skill()),
            "SG00" => Some(self.game.player_agent_grenade_variant().skill()),
            _ => None,
        };
        if let Some(variant) = variant.filter(|id| *id != skill.id)
            && let Some(selected) = catalogue().skills.iter().find(|s| s.id == variant)
        {
            y = self.draw_agent_skill_values(
                selected,
                self.game.player_agent_skill_rank(variant).max(1),
                None,
                x,
                y + 22.0,
                width,
                &selected.title,
            );
        }

        if rank < skill.learned_cap {
            let next = rank
                .saturating_add(1)
                .max(self.game.player_agent_equipment_skill_rank(&skill.id))
                .max(1);
            if self.game.player_agent_skill_rank(&skill.id) > 0 {
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

    fn agent_parameter_value(
        &self,
        skill: &Skill,
        key: &str,
        parameter: &serde_json::Value,
        rank: u8,
    ) -> f64 {
        if let Some(value) = self.game.agent_device_parameter_value(&skill.id, key, rank) {
            return f64::from(value);
        }
        if skill.id == "DR01" && key == "raw_robot_heal_HP" {
            return f64::from(self.game.agent_drone_repair_amount(rank));
        }
        if skill.id == "HY07" && key == "raw_fire_explosion_damage" {
            return f64::from(self.game.agent_drone_self_destruct_damage(rank));
        }
        if let Some(profile) = self.game.agent_spell_profile(&skill.id, rank) {
            match key {
                "raw_occult_damage"
                | "raw_fire_damage"
                | "raw_occult_damage_per_target"
                | "raw_explosion_fire_damage" => {
                    return f64::from(profile.damage_before_defenses());
                }
                "raw_burn_damage_per_turn" => {
                    if let Some((power, _, _)) = self.game.agent_spell_burn_profile(&skill.id, rank)
                    {
                        return (f64::from(power.base_damage) * power.numerator as f64
                            / power.denominator.max(1) as f64)
                            .floor();
                    }
                }
                "energy" => return f64::from(profile.energy),
                _ => {}
            }
        }
        let value = curve(parameter, rank);
        if key == "energy"
            && matches!(
                skill.id.as_str(),
                "B01" | "D56" | "D57" | "D58" | "D59" | "D66" | "DR01" | "HY04" | "HY07"
            )
            && let Some(start) = &self.game.rules().agent_class_start
        {
            return (value * f64::from(start.human_skill_energy_percent) / 100.)
                .ceil()
                .clamp(1., 65535.);
        }
        if parameter["current_level_binding"].as_str().is_some() {
            value * (1. + f64::from(self.game.player_progression().level().saturating_sub(1)) / 10.)
        } else {
            value
        }
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
                !key.contains("cooldown")
                    && p["historical_only"] != true
                    && !(skill.id == "SM00"
                        && self.game.player_agent_mine_variant()
                            != project_rl::game::AgentMineVariant::Fire
                        && key.as_str() == "raw_fire_explosion_damage")
                    && !(skill.id == "SG00"
                        && self.game.player_agent_grenade_variant()
                            != project_rl::game::AgentGrenadeVariant::Explosive
                        && key.as_str() == "raw_physical_explosion_damage")
                    && !(skill.id == "SG00"
                        && self.game.player_agent_grenade_variant()
                            == project_rl::game::AgentGrenadeVariant::Decoy
                        && key.as_str() == "radius_cells")
                    && !hidden.contains(&key.as_str())
                    && previous.is_none_or(|old| {
                        (self.agent_parameter_value(skill, key, p, old)
                            - self.agent_parameter_value(skill, key, p, rank))
                        .abs()
                            > 0.00001
                    })
            })
            .collect();
        if values.is_empty() {
            return y;
        }
        draw_text_bold(heading, x, y, 15.0, UiTheme.text());
        y += 24.0;
        for (key, p) in values {
            let value = self.agent_parameter_value(skill, key, p, rank);
            let label = if p["selection"] == "up_to_maximum" {
                "Déplacement maximum"
            } else {
                p["display_label"]
                    .as_str()
                    .unwrap_or(&catalogue().labels[key])
            };
            if skill.id == "SG00" && key == "radius_cells" {
                let side = (value.floor() as u32).saturating_mul(2).saturating_add(1);
                y = draw_wrapped_text(
                    &format!("Carré de {side} × {side} cases"),
                    x,
                    y,
                    width,
                    4,
                    14,
                    UiTheme.text(),
                ) + 7.0;
                continue;
            }
            if let Some(binding) = &skill.centred_damage
                && binding.damage_key == *key
            {
                let centre = value.floor().clamp(0.0, f64::from(u16::MAX)) as u16;
                // The farthest occupied cell of every supported shape is at 50%.
                // Preserve all existing caster/equipment modifiers above this stage.
                for text in [
                    format!("{label} au centre : {centre}"),
                    format!("{label} en bordure : {}", centre / 2),
                ] {
                    y = draw_wrapped_text(&text, x, y, width, 4, 14, UiTheme.text()) + 7.0;
                }
                continue;
            }
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
    fn icon_press_and_release_are_free_and_one_button_press_buys_only_one_rank() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE01").unwrap();
        let layout = Layout::new(
            1280.,
            800.,
            app.skill_discipline_selection,
            app.skill_technique_selection,
            skills(app.skill_discipline_selection).len(),
        );
        let tile = layout
            .tiles
            .iter()
            .find(|(i, _)| *i == app.skill_technique_selection)
            .unwrap()
            .1;
        let frame = |point: Vec2, down, press| InputFrame {
            pointer: Some(point.into()),
            mouse_left_down: down,
            pressed: if press {
                [controls::Binding::MouseLeft].into()
            } else {
                Default::default()
            },
            viewport: Some((1280., 800.)),
            ..Default::default()
        };
        let history = app.history.len();
        let before = suspension::fingerprint(&app.game);
        let icon = Layout::icon(tile).center();
        app.update_input(&frame(icon, true, true));
        app.update_input(&frame(icon, false, false));
        assert!(app.ux.skill_drag.is_none());
        assert_eq!(app.ux.skill_demo_request.as_deref(), Some("FE01"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);

        let rank = app.game.player_skills().agent_skill_rank("FE01");
        let points = app.game.player_progression().unspent_skill_points();
        let turn = app.game.turn();
        let button = Layout::learn_button(tile).center();
        app.update_input(&frame(button, true, true));
        app.update_input(&frame(button, true, false));
        app.update_input(&frame(button, false, false));
        assert_eq!(app.game.player_skills().agent_skill_rank("FE01"), rank + 1);
        assert_eq!(
            app.game.player_progression().unspent_skill_points(),
            points - 1
        );
        assert_eq!(app.game.turn(), turn);
        assert_eq!(app.history.len(), history + 1);

        app.select_agent_skill_for_capture("AF06").unwrap();
        let layout = Layout::new(
            1280.,
            800.,
            app.skill_discipline_selection,
            app.skill_technique_selection,
            skills(app.skill_discipline_selection).len(),
        );
        let tile = layout
            .tiles
            .iter()
            .find(|(i, _)| *i == app.skill_technique_selection)
            .unwrap()
            .1;
        let before = suspension::fingerprint(&app.game);
        let icon = Layout::icon(tile).center();
        app.update_input(&frame(icon, true, true));
        app.update_input(&frame(icon, false, false));
        assert!(app.ux.skill_demo_request.is_none());
        assert!(app.ux.skill_drag.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn dedicated_button_upgrades_passive_without_starting_an_active_skill_drag() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("AF06").unwrap();
        let layout = Layout::new(
            1280.0,
            800.0,
            app.skill_discipline_selection,
            app.skill_technique_selection,
            skills(app.skill_discipline_selection).len(),
        );
        let tile = layout
            .tiles
            .iter()
            .find(|(index, _)| *index == app.skill_technique_selection)
            .unwrap()
            .1;
        let icon = Layout::learn_button(tile).center();
        let slots = app.ux.skill_bar.clone();
        let points = app.game.player_progression().unspent_skill_points();
        let turn = app.game.turn();
        app.update_agent_skills(&InputFrame {
            pointer: Some(icon.into()),
            pressed: [controls::Binding::MouseLeft].into(),
            mouse_left_down: true,
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert!(app.ux.skill_drag.is_none());
        assert_eq!(app.game.player_agent_skill_rank("AF06"), 2);
        assert_eq!(
            app.game.player_progression().unspent_skill_points(),
            points - 1
        );
        assert_eq!(app.game.turn(), turn);
        assert_eq!(app.ux.skill_bar, slots);
    }

    #[test]
    fn connected_skill_cards_show_the_same_energy_cost_as_execution_with_class_modifiers() {
        let mut app = AsciiApp::new_agent_drone_trial().unwrap();
        let mut rules = app.game.rules().clone();
        rules
            .agent_class_start
            .as_mut()
            .unwrap()
            .human_skill_energy_percent = 80;
        app.game = WorldState::from_recovery_snapshot_bytes(
            &app.game.recovery_snapshot_bytes().unwrap(),
            rules,
        )
        .unwrap();
        for id in ["D58", "D59", "D66"] {
            let skill = catalogue().skills.iter().find(|s| s.id == id).unwrap();
            assert_eq!(
                app.agent_parameter_value(skill, "energy", &skill.parameters["energy"], 1),
                f64::from(app.agent_skill_use_profile(id).unwrap().energy)
            );
        }
    }
    #[test]
    fn ember_keyboard_action_and_card_share_cast_values_and_apply_immediate_burn() {
        let mut app = AsciiApp::new_agent_ember_trial().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE01").unwrap();
        let skill = catalogue().skills.iter().find(|s| s.id == "FE01").unwrap();
        for rank in [1, 10, 11, 25] {
            let profile = app.game.agent_spell_profile("FE01", rank).unwrap();
            assert_eq!(
                app.agent_parameter_value(
                    skill,
                    "raw_fire_damage",
                    &skill.parameters["raw_fire_damage"],
                    rank
                ),
                f64::from(profile.damage_before_defenses())
            );
            assert_eq!(
                app.agent_parameter_value(skill, "energy", &skill.parameters["energy"], rank),
                f64::from(profile.energy)
            );
        }
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(!app.skills_open);
        let at = app.attack_aim.unwrap().cursor;
        let target = app.game.actors().entity_at(at).unwrap();
        let hp = app.game.actors().get(target).unwrap().integrity();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        let a = app.game.actors().get(target).unwrap();
        assert!(a.integrity() < hp);
        assert_eq!(
            a.statuses()
                .find(|s| s.definition.as_str() == "core:burning")
                .unwrap()
                .remaining_turns,
            Some(2)
        );
    }

    #[test]
    fn fireball_keyboard_action_previews_a_real_area_then_hits_multiple_targets() {
        let mut app = AsciiApp::new_agent_fireball_trial().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE05").unwrap();
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(!app.skills_open);
        assert!(
            app.aimed_attack_preview(app.attack_aim.unwrap())
                .unwrap()
                .cells()
                .len()
                > 9
        );
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        assert_eq!(
            app.game
                .actors()
                .iter()
                .filter(|(id, a)| *id != app.game.player_id() && a.integrity() < 500)
                .count(),
            3
        );
    }
    #[test]
    fn occult_blast_keyboard_action_previews_a_real_area_then_hits_multiple_targets() {
        let mut app = AsciiApp::new_agent_blast_trial().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("A02").unwrap();
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(!app.skills_open);
        assert_eq!(
            app.aimed_attack_preview(app.attack_aim.unwrap())
                .unwrap()
                .cells()
                .len(),
            9
        );
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        assert_eq!(
            app.game
                .actors()
                .iter()
                .filter(|(id, a)| *id != app.game.player_id() && a.integrity() < 500)
                .count(),
            3
        );
    }
    #[test]
    fn drone_keyboard_actions_prepare_free_placement_then_create_real_companions() {
        for id in ["D58", "D59", "D66"] {
            let mut app = AsciiApp::new_agent_drone_trial().unwrap();
            app.open_agent_skill_window();
            app.select_agent_skill_for_capture(id).unwrap();
            let before = suspension::fingerprint(&app.game);
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Use).clone()].into(),
                viewport: Some((1280., 800.)),
                ..Default::default()
            });
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert!(app.attack_aim.is_some());
            assert!(!app.skills_open);
            let at = app.attack_aim.unwrap().cursor;
            assert!(app.game.actors().entity_at(at).is_none());
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                ..Default::default()
            });
            assert_eq!(app.game.player_agent_drone_count(), 1);
            assert_eq!(app.game.turn(), 1);
            assert!(
                app.front_actor_visual(app.game.actors().entity_at(at).unwrap())
                    .is_some()
            );
        }
    }

    #[test]
    fn rupture_keyboard_action_opens_free_aim_with_the_melee_hand_and_confirmation_attacks() {
        let mut app = AsciiApp::new_agent_melee_trial().unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("B01").unwrap();
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert!(!app.skills_open);
        assert_eq!(app.attack_aim.unwrap().slot, 0);
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        assert!(
            app.game
                .actors()
                .get(app.game.actors().entity_at(GridPos::new(5, 4)).unwrap())
                .unwrap()
                .integrity()
                < 500
        );
    }

    #[test]
    fn only_dedicated_button_buys_ranks_and_purchases_survive_suspension() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.suspension_path = std::env::temp_dir().join(format!(
            "project-rl-agent-progression-test-{}.json",
            std::process::id()
        ));
        app.begin_character_creation_with_seed(false, app.seed)
            .unwrap();
        let creation = app.character_creation.take().unwrap();
        app.rebuild_run_with_character_class(&creation).unwrap();
        let mut saved: serde_json::Value =
            serde_json::from_str(&app.game.export_player_progression().unwrap()).unwrap();
        saved["level"] = serde_json::json!(3);
        saved["experience"] =
            serde_json::json!(app.game.rules().progression.curve.cumulative_thresholds()[1]);
        saved["skill_points_earned"] = serde_json::json!(2);
        saved["unspent_skill_points"] = serde_json::json!(2);
        app.game
            .restore_player_progression(&saved.to_string())
            .unwrap();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE01").unwrap();
        let layout = Layout::new(
            1280.0,
            800.0,
            app.skill_discipline_selection,
            app.skill_technique_selection,
            skills(app.skill_discipline_selection).len(),
        );
        let tile = layout
            .tiles
            .iter()
            .find(|(index, _)| *index == app.skill_technique_selection)
            .unwrap()
            .1;
        let click = |point| InputFrame {
            pointer: Some(point),
            pressed: [controls::Binding::MouseLeft].into(),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        };
        let history = app.history.len();
        let turn = app.game.turn();
        app.update_agent_skills(&click((tile.x + tile.w * 0.5, tile.y + 90.0)));
        assert_eq!(app.game.player_progression().unspent_skill_points(), 2);
        let icon = Layout::icon(tile);
        let before = suspension::fingerprint(&app.game);
        app.update_agent_skills(&click((icon.x + 32.0, icon.y + 32.0)));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.ux.skill_demo_request.as_deref(), Some("FE01"));
        let button = Layout::learn_button(tile).center();
        app.update_agent_skills(&click(button.into()));
        assert_eq!(app.game.player_skills().agent_skill_rank("FE01"), 1);
        app.update_agent_skills(&click(button.into()));
        assert_eq!(app.game.player_skills().agent_skill_rank("FE01"), 2);
        assert_eq!(app.game.player_progression().unspent_skill_points(), 0);
        assert_eq!(app.history.len(), history + 2);
        assert_eq!(app.game.turn(), turn);
        let before = suspension::fingerprint(&app.game);
        app.update_agent_skills(&click(button.into()));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history + 2);
        app.skills_open = false;
        app.character_open = true;
        let plus = CharacterLayout::new(1280.0, 800.0).attribute_increases[0];
        let starting = app.rules.player_starting_attributes;
        app.update_input(&click((plus.x + 12.0, plus.y + 12.0)));
        assert_eq!(
            app.game
                .player_primary_attributes()
                .unwrap()
                .value(PrimaryAttribute::Power),
            starting.value(PrimaryAttribute::Power) + 1
        );
        assert_eq!(app.game.unspent_attribute_points(), 9);
        app.ux.skill_bar[2] = Some("FE01".into());
        let saved = app.suspension().unwrap();
        assert_eq!(
            saved.starting_attributes,
            Some(starting.values().map(|v| v as u8))
        );
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(restored.game.unspent_attribute_points(), 9);
        assert_eq!(
            restored.game.player_primary_attributes(),
            app.game.player_primary_attributes()
        );
        assert_eq!(restored.game.player_skills(), app.game.player_skills());
        assert_eq!(restored.ux.skill_bar, app.ux.skill_bar);
    }
    #[test]
    fn accepted_area_bindings_reference_real_values_and_replace_flat_reductions() {
        let data = catalogue();
        let bound: Vec<_> = data
            .skills
            .iter()
            .filter(|s| s.centred_damage.is_some())
            .collect();
        assert_eq!(bound.len(), 10);
        for s in bound {
            let binding = s.centred_damage.as_ref().unwrap();
            assert!(s.parameters.contains_key(&binding.damage_key));
            assert!(s.parameters.contains_key(&binding.extent_key));
            assert!(binding.extent_divisor > 0);
        }
        let frozen = data.skills.iter().find(|s| s.id == "FR06").unwrap();
        assert!(
            !frozen
                .parameters
                .contains_key("splash_percent_of_source_damage")
        );
        let eruption = data.skills.iter().find(|s| s.id == "FE04").unwrap();
        assert!(
            !eruption
                .parameters
                .contains_key("maximum_edge_reduction_percent")
        );
    }
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
    fn review_demonstration_fits_below_cards_without_covering_actions_or_native_icons() {
        for (width, height) in [(1280., 960.), (1920., 1080.)] {
            for selection in 0..skills(6).len() {
                let layout = Layout::with_demo(width, height, 6, selection, skills(6).len(), 236.);
                let demo = layout.demo.unwrap();
                assert_eq!(demo.size(), vec2(480., 222.));
                assert!(demo.y > layout.tiles.last().unwrap().1.bottom());
                assert!(demo.bottom() <= layout.grid.bottom());
                assert!(demo.right() < layout.detail.x);
                let ordinary = Layout::new(width, height, 6, selection, skills(6).len());
                assert_eq!(layout.detail, ordinary.detail);
                assert_eq!(layout.branches, ordinary.branches);
                assert!(demo.bottom() < layout.close.y);
                assert!(demo.right() <= layout.panel.right());
                for (_, tile) in &layout.tiles {
                    assert_eq!(Layout::icon(*tile).size(), vec2(64., 64.));
                    assert!(tile.bottom() <= layout.grid.bottom());
                    assert!(Layout::learn_button(*tile).right() <= layout.grid.right());
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
            (2560.0, 1440.0),
            (3840.0, 2160.0),
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
                        let button = Layout::learn_button(*tile);
                        assert!(
                            contains(layout.grid, button),
                            "{width}x{height}: {button:?} / {:?}",
                            layout.grid
                        );
                        assert!(!Layout::icon(*tile).overlaps(&button));
                    }
                    assert!(contains(layout.panel, layout.close));
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
        for button in [Layout::learn_button(layout.tiles[0].1)] {
            app.update_agent_skills(&InputFrame {
                pointer: Some((button.x + 5.0, button.y + 5.0)),
                pressed: [controls::Binding::MouseLeft].into(),
                viewport: Some((1280., 800.)),
                ..Default::default()
            });
        }
        assert!(app.skills_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
    }
    #[test]
    fn device_variant_buttons_are_locked_until_learned_and_record_timeless_choices() {
        use project_rl::game::{AgentGrenadeVariant, AgentMineVariant};
        let mut app = AsciiApp::new_agent_drone_trial().unwrap();
        for skill in ["SG00", "SG01", "SM00", "SM02"] {
            assert_eq!(
                app.execute_command(GameCommand::LearnAgentSkill {
                    skill: skill.into()
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        app.open_agent_skill_window();
        for (skill, index) in [("SM00", 1), ("SG00", 1)] {
            app.select_agent_skill_for_capture(skill).unwrap();
            let layout = Layout::new(
                1280.,
                800.,
                app.skill_discipline_selection,
                app.skill_technique_selection,
                skills(app.skill_discipline_selection).len(),
            );
            let turn = app.game.turn();
            let energy = app.game.player_energy().available();
            let history = app.history.len();
            app.update_agent_skills(&InputFrame {
                pointer: Some(
                    AsciiApp::agent_device_variant_rect(layout.detail, index)
                        .center()
                        .into(),
                ),
                pressed: [controls::Binding::MouseLeft].into(),
                viewport: Some((1280., 800.)),
                ..Default::default()
            });
            assert_eq!(app.game.turn(), turn);
            assert_eq!(app.game.player_energy().available(), energy);
            assert_eq!(app.history.len(), history + 1);
            assert!(app.skills_open);
            assert!(app.attack_aim.is_none());
            let recorded = app.history.last().unwrap();
            let json = serde_json::to_vec(recorded).unwrap();
            let restored: RecordedCommand = serde_json::from_slice(&json).unwrap();
            assert_eq!(&restored, recorded);
            assert!(
                !restored
                    .command(&app.game)
                    .unwrap()
                    .consumes_time_on_success()
            );
            let history = app.history.len();
            app.update_agent_skills(&InputFrame {
                pointer: Some(
                    AsciiApp::agent_device_variant_rect(layout.detail, 2)
                        .center()
                        .into(),
                ),
                pressed: [controls::Binding::MouseLeft].into(),
                viewport: Some((1280., 800.)),
                ..Default::default()
            });
            assert_eq!(app.history.len(), history); // Cold and decoy passives have not been learned.
        }
        assert_eq!(
            app.game.player_agent_mine_variant(),
            AgentMineVariant::Poison
        );
        assert_eq!(
            app.game.player_agent_grenade_variant(),
            AgentGrenadeVariant::Blinding
        );
    }
}
