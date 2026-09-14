use std::collections::BTreeMap;

use crate::controls::{self, Action, Controls, InputFrame, KeySemantics};
use crate::graphics::{self, GraphicsSettings, GraphicsState, WindowMode};
use crate::pause_menu::{ControlsLayout, MenuFocus, MenuLayout, MenuScreen, wheel_steps};
use crate::suspension::{self, RecordedCommand, Suspension};
use crate::terminal_view::{
    TerminalAlertKind, TerminalAttackPreview, TerminalDrawOptions, TerminalOverlay,
    TerminalStatusIcon, TerminalView, player_location_name,
};
use crate::test_sector::TestSector;
use crate::ui_theme::{
    ButtonTone, UiTheme, draw_text, draw_text_bold, draw_text_bold_centered, measure_text,
    measure_text_bold,
};

use macroquad::prelude::*;
use project_rl::ai::{AiProfile, PursuitLifecycle};
use project_rl::character_class::{CharacterClassCatalog, CharacterClassId};
use project_rl::combat::{
    ArmorRules, AttackArea, AttackProfile, DamageImpact, DamageRules, DamageType, HitRules,
    MeleeImpactProfile,
};
use project_rl::content::{
    ContentId, ContentLoader, ExpeditionCatalog, RegionCoord, RegionDirection,
    RegionVerticalDirection, RegionalWorldCatalog,
};
use project_rl::drone::{
    DroneCapabilities, DroneCondition, DroneConditionalResponse, DroneDeploymentAssignment,
    DroneDeploymentRole, DroneDirective, DroneOrder, DroneProfile, PatrolBlockedResponse,
};
use project_rl::effects::{AbilityProfile, ApplyStatusEffect, EffectPrimitive};
use project_rl::electronic_warfare::{ElectronicChannel, ElectronicDirective};
use project_rl::engineering::{EngineeringDirective, ModuleTuning};
use project_rl::entity::{
    Actor, BodyComponentId, BodyComponentProfile, ComponentFailureEffect, EntityId, InventoryEntry,
    ItemInstanceId,
};
use project_rl::explosive::ExplosiveActivation;
use project_rl::facility::{
    DoorLockdownPrevention, FacilityEvent, InstallationCapability, ReinforcementRequestFailure,
    WorkerRole,
};
use project_rl::game::{
    CommandOutcome, CommandRejection, CompanionBehavior, CounterattackOutcome,
    ForcedMovementOutcome, GameCommand, GameEvent, GameRules, GameState, InterceptionOutcome,
    PreparationDisruptionOutcome, RunStatus, StartingItemStack, SystemResourceRules,
    TechniqueEffectFailure, WorldState, ZoneConnectionBlueprint,
};
use project_rl::intrusion::{DeviceCommand, DigitalRoutine, IntrusionDirective};
use project_rl::item::{ItemEffect, ItemId, ItemKind};
use project_rl::localization::TextCatalog;
use project_rl::loot::LootCatalog;
use project_rl::presentation::{VisualCue, VisualCueCatalog, VisualCueCell, VisualCueId};
use project_rl::progression::DefeatReward;
use project_rl::skills::{
    DisciplineAvailability, DisciplineId, ExplosiveDeployment, SystemFeatureId, SystemFeatureSet,
    TechniqueAction, TechniqueEffectResistance, TechniqueEngagementRequirement, TechniqueId,
    TechniqueImprovement, TechniqueKind, TechniqueTargetRequirement,
};
use project_rl::social::PlayerRelation;
use project_rl::stats::{
    BodyProfile, DisplacementProfile, LocomotionProfile, PhysicalRules, PrimaryAttribute,
    PrimaryAttributes, StabilityRules,
};
use project_rl::status::{StatusApplyKind, StatusId, StatusModifier};
use project_rl::stealth::{SignatureChannel, StealthRules};
use project_rl::weapon::WeaponId;
use project_rl::world::{Direction, DistanceMetric, GridPos, Terrain};

use crate::visual_effects::VisualCuePlayer;

const INITIAL_SEED: u64 = 20_260_909;
const FLAMETHROWER_GENERATION_VERSION: u8 = 10;
const DEFINED_POPULATION_GENERATION_VERSION: u8 = 11;
const EXPANDED_WORLD_GENERATION_VERSION: u8 = 12;
const PURSUIT_LEASH_GENERATION_VERSION: u8 = 13;
const LAZY_ZONE_GENERATION_VERSION: u8 = 14;
const REGIONAL_TRAVEL_GENERATION_VERSION: u8 = 15;
const REGIONAL_POPULATION_GENERATION_VERSION: u8 = 16;
const PURSUIT_LIFECYCLE_GENERATION_VERSION: u8 = 17;
const REGIONAL_LOOT_GENERATION_VERSION: u8 = 18;
const REGIONAL_LANDMARK_GENERATION_VERSION: u8 = 19;
const THREAT_RENEWAL_GENERATION_VERSION: u8 = 20;
const REGIONAL_ENCOUNTER_GENERATION_VERSION: u8 = 21;
const REGIONAL_SITE_GENERATION_VERSION: u8 = 22;
const REGIONAL_SITE_INTERACTION_GENERATION_VERSION: u8 = 23;
const REGIONAL_SITE_SECURITY_GENERATION_VERSION: u8 = 24;
const INVESTIGATING_REINFORCEMENTS_GENERATION_VERSION: u8 = 25;
const SITE_NAVIGATION_SIGNALS_GENERATION_VERSION: u8 = 26;
const DATA_TERMINALS_GENERATION_VERSION: u8 = 27;
const REGIONAL_SITE_TERMINALS_GENERATION_VERSION: u8 = 28;
const REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION: u8 = 29;
const REGIONAL_DESTRUCTIBLES_GENERATION_VERSION: u8 = 30;
const HIT_CHANCE_GENERATION_VERSION: u8 = 31;
const ENEMY_ATTRIBUTES_GENERATION_VERSION: u8 = 32;
const PHYSICAL_PROFILES_GENERATION_VERSION: u8 = 33;
const CHARACTER_CLASSES_GENERATION_VERSION: u8 = 34;
const ARMOR_GENERATION_VERSION: u8 = 35;
const ARMOR_EQUIPMENT_GENERATION_VERSION: u8 = 36;
const MIXED_DAMAGE_GENERATION_VERSION: u8 = 37;
const EFFECT_TRIGGER_GENERATION_VERSION: u8 = MIXED_DAMAGE_GENERATION_VERSION + 1;
const PARRY_REACTION_GENERATION_VERSION: u8 = EFFECT_TRIGGER_GENERATION_VERSION + 1;
const MULTI_UT_PREPARATION_GENERATION_VERSION: u8 = PARRY_REACTION_GENERATION_VERSION + 1;
const ACTION_RECOVERY_GENERATION_VERSION: u8 = MULTI_UT_PREPARATION_GENERATION_VERSION + 1;
const MELEE_SKILLS_GENERATION_VERSION: u8 = ACTION_RECOVERY_GENERATION_VERSION + 1;
const RANGED_SKILLS_GENERATION_VERSION: u8 = MELEE_SKILLS_GENERATION_VERSION + 1;
const DEMOLITION_SKILLS_GENERATION_VERSION: u8 = RANGED_SKILLS_GENERATION_VERSION + 1;
const SYSTEM_RESOURCES_GENERATION_VERSION: u8 = DEMOLITION_SKILLS_GENERATION_VERSION + 1;
const MANOEUVRE_SKILLS_GENERATION_VERSION: u8 = SYSTEM_RESOURCES_GENERATION_VERSION + 1;
const FURTIVITE_SKILLS_GENERATION_VERSION: u8 = MANOEUVRE_SKILLS_GENERATION_VERSION + 1;
const DRONE_SKILLS_GENERATION_VERSION: u8 = FURTIVITE_SKILLS_GENERATION_VERSION + 1;
const RECONNAISSANCE_COMPLETION_GENERATION_VERSION: u8 = DRONE_SKILLS_GENERATION_VERSION + 1;
const ENGINEERING_SKILLS_GENERATION_VERSION: u8 = RECONNAISSANCE_COMPLETION_GENERATION_VERSION + 1;
const INTRUSION_SKILLS_GENERATION_VERSION: u8 = ENGINEERING_SKILLS_GENERATION_VERSION + 1;
const ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION: u8 = INTRUSION_SKILLS_GENERATION_VERSION + 1;
const AUTHORED_SKILL_REQUIREMENTS_GENERATION_VERSION: u8 =
    ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION + 1;
const INTRINSIC_SKILL_MANIFESTATIONS_GENERATION_VERSION: u8 =
    AUTHORED_SKILL_REQUIREMENTS_GENERATION_VERSION + 1;
const WAIT_CONTINUES_PREPARATION_GENERATION_VERSION: u8 =
    INTRINSIC_SKILL_MANIFESTATIONS_GENERATION_VERSION + 1;
const PREPARATION_DISRUPTION_GENERATION_VERSION: u8 =
    WAIT_CONTINUES_PREPARATION_GENERATION_VERSION + 1;
const DRONE_DEFAULT_SUPPORT_GENERATION_VERSION: u8 = PREPARATION_DISRUPTION_GENERATION_VERSION + 1;
const DRONE_ENERGY_LIFETIME_GENERATION_VERSION: u8 = DRONE_DEFAULT_SUPPORT_GENERATION_VERSION + 1;
const DRONE_LINK_AWARENESS_GENERATION_VERSION: u8 = DRONE_ENERGY_LIFETIME_GENERATION_VERSION + 1;
const PLAYER_RELATIONS_GENERATION_VERSION: u8 = DRONE_LINK_AWARENESS_GENERATION_VERSION + 1;
const CURRENT_GENERATION_VERSION: u8 = PLAYER_RELATIONS_GENERATION_VERSION;
const _: () = assert!(CURRENT_GENERATION_VERSION == suspension::MAX_GENERATION_VERSION);
const LOG_CAPACITY: usize = 6;
const FLOATING_MESSAGE_CAPACITY: usize = 32;
const DISPLAY_LOCALE: &str = "fr";
const WAIT_ACTION_FOCUS: usize = usize::MAX;
const COMPANION_ACTION_FOCUS_BASE: usize = usize::MAX - 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct AttackAim {
    slot: u8,
    cursor: GridPos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ComponentSelection {
    technique: TechniqueId,
    target: EntityId,
    components: Vec<BodyComponentId>,
    selected: usize,
    weapon_slot: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FloatingMessageTone {
    Damage,
    Recovery,
    Status,
    Alert,
    Progression,
    Information,
}

impl FloatingMessageTone {
    const fn lifetime(self) -> f64 {
        match self {
            Self::Progression => 2.4,
            Self::Alert => 2.0,
            Self::Status => 1.8,
            Self::Damage | Self::Recovery | Self::Information => 1.35,
        }
    }

    const fn font_size(self) -> f32 {
        match self {
            Self::Progression => 22.0,
            Self::Alert => 19.0,
            Self::Damage | Self::Recovery | Self::Status | Self::Information => 17.0,
        }
    }

    fn color(self) -> Color {
        match self {
            Self::Damage => Color::from_rgba(255, 113, 91, 255),
            Self::Recovery => Color::from_rgba(111, 232, 145, 255),
            Self::Status => Color::from_rgba(255, 174, 74, 255),
            Self::Alert => Color::from_rgba(255, 210, 82, 255),
            Self::Progression => Color::from_rgba(255, 226, 105, 255),
            Self::Information => Color::from_rgba(132, 225, 229, 255),
        }
    }
}

#[derive(Clone, Debug)]
struct FloatingMessage {
    text: String,
    at: GridPos,
    tone: FloatingMessageTone,
    started_at: f64,
    lane: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct LevelUpNotice {
    level: u16,
    skill_points_awarded: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DossierLineKind {
    Section,
    Content,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct DossierLine {
    text: String,
    kind: DossierLineKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CharacterCreationStage {
    Protocol,
    Attributes,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum InventoryFilter {
    #[default]
    All,
    Weapons,
    Armor,
    Consumables,
    Materials,
}

impl InventoryFilter {
    const ALL: [Self; 5] = [
        Self::All,
        Self::Weapons,
        Self::Armor,
        Self::Consumables,
        Self::Materials,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::All => "Tout",
            Self::Weapons => "Armes",
            Self::Armor => "Armures",
            Self::Consumables => "Conso.",
            Self::Materials => "Matériaux",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum InventoryGlyph {
    Blade,
    RangedWeapon,
    FlameProjector,
    Armor,
    Consumable,
    Material,
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum InventorySort {
    #[default]
    Type,
    Name,
}

impl InventorySort {
    const fn label(self) -> &'static str {
        match self {
            Self::Type => "Type",
            Self::Name => "Nom",
        }
    }

    const fn other(self) -> Self {
        match self {
            Self::Type => Self::Name,
            Self::Name => Self::Type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CharacterCreationHover {
    Class(usize),
    Attribute(usize),
    AttributeMinus(usize),
    AttributePlus(usize),
    Preset,
    Cancel,
    Continue,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CharacterCreation {
    stage: CharacterCreationStage,
    selected_class: usize,
    selected_attribute: usize,
    attributes: PrimaryAttributes,
    replace_suspension: bool,
    message: String,
    hovered: Option<CharacterCreationHover>,
}

struct CharacterCreationLayout {
    panel: Rect,
    class_rows: Vec<Rect>,
    cancel: Rect,
    continue_button: Rect,
    attribute_rows: Vec<Rect>,
    attribute_minus: Vec<Rect>,
    attribute_plus: Vec<Rect>,
    preset: Rect,
}

struct InventoryLayout {
    rows: Vec<(usize, Rect)>,
    actions: [Rect; 6],
    filters: [Rect; 5],
    sort: Rect,
    left_width: f32,
    stats_panel: Option<Rect>,
    character_details: Option<Rect>,
    first_visible: usize,
    visible_rows: usize,
}

impl InventoryLayout {
    fn new(width: f32, height: f32, selection: usize, count: usize) -> Self {
        let margin = 32.0;
        let top = 30.0;
        let panel_width = (width - margin * 2.0).max(620.0);
        let panel_height = (height - top * 2.0).max(400.0);
        let (left_width, stats_width) = if panel_width >= 1080.0 && panel_height >= 620.0 {
            (panel_width * 0.36, Some((panel_width * 0.215).max(250.0)))
        } else {
            (panel_width * 0.43, None)
        };
        let row_height = 48.0;
        let visible_rows = ((panel_height - 246.0) / row_height).floor().max(1.0) as usize;
        let first_visible = selection.saturating_sub(visible_rows.saturating_sub(1));
        let rows = (first_visible..(first_visible + visible_rows).min(count))
            .enumerate()
            .map(|(visible, index)| {
                (
                    index,
                    Rect::new(
                        margin + 10.0,
                        top + 136.0 + visible as f32 * row_height,
                        left_width - 20.0,
                        39.0,
                    ),
                )
            })
            .collect();
        let action_gap = 7.0;
        let action_width = (panel_width - 40.0 - action_gap * 5.0) / 6.0;
        let actions = std::array::from_fn(|index| {
            Rect::new(
                margin + 20.0 + index as f32 * (action_width + action_gap),
                top + panel_height - 43.0,
                action_width,
                31.0,
            )
        });
        let filter_gap = 5.0;
        let filter_width = (left_width - 20.0 - filter_gap * 4.0) / 5.0;
        let filters = std::array::from_fn(|index| {
            Rect::new(
                margin + 10.0 + index as f32 * (filter_width + filter_gap),
                top + 68.0,
                filter_width,
                29.0,
            )
        });
        let sort = Rect::new(margin + left_width - 100.0, top + 101.0, 90.0, 25.0);
        let stats_panel = stats_width.map(|stats_width| {
            Rect::new(
                margin + panel_width - stats_width,
                top + 68.0,
                stats_width,
                panel_height - 128.0,
            )
        });
        let character_details = stats_panel.map(|panel| {
            Rect::new(
                panel.x + 12.0,
                panel.y + panel.h - 39.0,
                panel.w - 24.0,
                29.0,
            )
        });
        Self {
            rows,
            actions,
            filters,
            sort,
            left_width,
            stats_panel,
            character_details,
            first_visible,
            visible_rows,
        }
    }
}

struct SkillsLayout {
    panel: Rect,
    discipline_panel: Rect,
    technique_panel: Rect,
    detail_panel: Rect,
    discipline_rows: Vec<Rect>,
    technique_rows: Vec<(usize, Rect)>,
    actions: [Rect; 3],
    first_technique: usize,
}

struct TechniqueQuickMenuLayout {
    panel: Rect,
    rows: Vec<(usize, Rect)>,
    actions: [Rect; 2],
    first_visible: usize,
}

impl TechniqueQuickMenuLayout {
    fn new(width: f32, height: f32, selection: usize, count: usize) -> Self {
        let panel_width = (width - 48.0).clamp(360.0, 680.0);
        let row_height = 48.0;
        let maximum_rows = (((height - 196.0).max(row_height)) / row_height)
            .floor()
            .max(1.0) as usize;
        let visible_count = count.max(1).min(maximum_rows).min(9);
        let panel_height = (144.0 + visible_count as f32 * row_height)
            .max(220.0)
            .min((height - 48.0).max(220.0));
        let panel = Rect::new(
            (width - panel_width) * 0.5,
            (height - panel_height) * 0.5,
            panel_width,
            panel_height,
        );
        let visible_rows = (((panel.h - 144.0) / row_height).floor().max(1.0)) as usize;
        let first_visible = selection.saturating_sub(visible_rows.saturating_sub(1));
        let rows = (first_visible..(first_visible + visible_rows).min(count))
            .enumerate()
            .map(|(visible, index)| {
                (
                    index,
                    Rect::new(
                        panel.x + 12.0,
                        panel.y + 66.0 + visible as f32 * row_height,
                        panel.w - 24.0,
                        41.0,
                    ),
                )
            })
            .collect();
        let button_width = (panel.w - 31.0) * 0.5;
        let actions = [
            Rect::new(panel.x + 12.0, panel.y + panel.h - 49.0, button_width, 35.0),
            Rect::new(
                panel.x + 19.0 + button_width,
                panel.y + panel.h - 49.0,
                button_width,
                35.0,
            ),
        ];
        Self {
            panel,
            rows,
            actions,
            first_visible,
        }
    }
}

struct CharacterLayout {
    attribute_rows: [Rect; 5],
    actions: [Rect; 3],
}

#[derive(Clone, Copy, Debug)]
struct CompanionBarLayout {
    panel: Rect,
    behavior_buttons: [Rect; 4],
}

impl CompanionBarLayout {
    fn new(width: f32, height: f32) -> Self {
        let panel = Rect::new(12.0, height - 158.0, (width - 24.0).max(440.0), 54.0);
        let info_width = (panel.w * 0.34).clamp(235.0, 360.0);
        let gap = 6.0;
        let button_x = panel.x + info_width;
        let button_width = ((panel.w - info_width - gap * 4.0) / 4.0).max(72.0);
        let behavior_buttons = std::array::from_fn(|index| {
            Rect::new(
                button_x + index as f32 * (button_width + gap),
                panel.y + 9.0,
                button_width,
                36.0,
            )
        });
        Self {
            panel,
            behavior_buttons,
        }
    }
}

impl CharacterLayout {
    fn new(width: f32, height: f32) -> Self {
        let margin = 32.0;
        let top = 30.0;
        let panel_width = (width - margin * 2.0).max(620.0);
        let panel_height = (height - top * 2.0).max(400.0);
        let left_width = panel_width * 0.48;
        let attribute_stride = ((panel_height - 255.0) / 5.0).clamp(29.0, 42.0);
        let attribute_rows = std::array::from_fn(|index| {
            Rect::new(
                margin + 14.0,
                top + 207.0 + index as f32 * attribute_stride,
                left_width - 28.0,
                attribute_stride - 5.0,
            )
        });
        let button_width = (panel_width - 54.0) / 3.0;
        let actions = std::array::from_fn(|index| {
            Rect::new(
                margin + 20.0 + index as f32 * (button_width + 7.0),
                top + panel_height - 43.0,
                button_width,
                31.0,
            )
        });
        Self {
            attribute_rows,
            actions,
        }
    }
}

impl SkillsLayout {
    fn new(
        width: f32,
        height: f32,
        discipline_count: usize,
        technique_selection: usize,
        technique_count: usize,
    ) -> Self {
        let margin = 32.0;
        let top = 30.0;
        let panel_width = (width - margin * 2.0).max(620.0);
        let panel_height = (height - top * 2.0).max(400.0);
        let panel = Rect::new(margin, top, panel_width, panel_height);
        let content = Rect::new(
            panel.x + 10.0,
            panel.y + 68.0,
            panel.w - 20.0,
            panel.h - 128.0,
        );
        let gap = 10.0;
        let discipline_width = (content.w * 0.23).clamp(180.0, 260.0);
        let discipline_panel = Rect::new(content.x, content.y, discipline_width, content.h);
        let remaining_width = content.w - discipline_width - gap;
        let (technique_panel, detail_panel) = if panel_width >= 1_040.0 {
            let technique_width = (content.w * 0.34)
                .clamp(280.0, 400.0)
                .min(remaining_width - gap - 260.0);
            (
                Rect::new(
                    discipline_panel.x + discipline_panel.w + gap,
                    content.y,
                    technique_width,
                    content.h,
                ),
                Rect::new(
                    discipline_panel.x + discipline_panel.w + gap * 2.0 + technique_width,
                    content.y,
                    remaining_width - gap - technique_width,
                    content.h,
                ),
            )
        } else {
            let right_x = discipline_panel.x + discipline_panel.w + gap;
            let upper_height = (content.h * 0.48).max(126.0).min(content.h - gap - 126.0);
            (
                Rect::new(right_x, content.y, remaining_width, upper_height),
                Rect::new(
                    right_x,
                    content.y + upper_height + gap,
                    remaining_width,
                    content.h - upper_height - gap,
                ),
            )
        };
        let discipline_header_height = 34.0;
        let discipline_stride = if discipline_count == 0 {
            0.0
        } else {
            ((discipline_panel.h - discipline_header_height) / discipline_count as f32)
                .clamp(23.0, 42.0)
        };
        let discipline_rows = (0..discipline_count)
            .map(|index| {
                Rect::new(
                    discipline_panel.x + 7.0,
                    discipline_panel.y
                        + discipline_header_height
                        + index as f32 * discipline_stride,
                    discipline_panel.w - 14.0,
                    (discipline_stride - 3.0).max(20.0),
                )
            })
            .collect();
        let row_height = 43.0;
        let visible = ((technique_panel.h - 58.0) / row_height).floor().max(1.0) as usize;
        let first_technique = technique_selection.saturating_sub(visible.saturating_sub(1));
        let technique_rows = (first_technique..(first_technique + visible).min(technique_count))
            .enumerate()
            .map(|(visible_index, index)| {
                (
                    index,
                    Rect::new(
                        technique_panel.x + 7.0,
                        technique_panel.y + 46.0 + visible_index as f32 * row_height,
                        technique_panel.w - 14.0,
                        37.0,
                    ),
                )
            })
            .collect();
        let action_x = technique_panel.x;
        let action_width = detail_panel.x + detail_panel.w - action_x;
        let button_width = ((action_width - 14.0) / 3.0).max(80.0);
        let actions = std::array::from_fn(|index| {
            Rect::new(
                action_x + index as f32 * (button_width + 7.0),
                top + panel_height - 43.0,
                button_width,
                31.0,
            )
        });
        Self {
            panel,
            discipline_panel,
            technique_panel,
            detail_panel,
            discipline_rows,
            technique_rows,
            actions,
            first_technique,
        }
    }
}

impl CharacterCreationLayout {
    fn new(width: f32, height: f32, class_count: usize) -> Self {
        let panel = Rect::new(
            24.0,
            24.0,
            (width - 48.0).max(592.0),
            (height - 48.0).max(432.0),
        );
        let left_width = panel.w * 0.43;
        let row_height = ((panel.h - 210.0) / class_count.max(1) as f32).clamp(50.0, 78.0);
        let class_rows = (0..class_count)
            .map(|index| {
                Rect::new(
                    panel.x + 24.0,
                    panel.y + 105.0 + index as f32 * row_height,
                    left_width - 36.0,
                    row_height - 8.0,
                )
            })
            .collect();
        let attribute_stride =
            ((panel.h - 190.0) / PrimaryAttribute::ALL.len() as f32).clamp(43.0, 58.0);
        let attribute_rows = (0..PrimaryAttribute::ALL.len())
            .map(|index| {
                Rect::new(
                    panel.x + 28.0,
                    panel.y + 112.0 + index as f32 * attribute_stride,
                    left_width - 44.0,
                    attribute_stride - 7.0,
                )
            })
            .collect::<Vec<_>>();
        let attribute_minus = attribute_rows
            .iter()
            .map(|row| Rect::new(row.x + row.w - 112.0, row.y + 5.0, 30.0, 30.0))
            .collect();
        let attribute_plus = attribute_rows
            .iter()
            .map(|row| Rect::new(row.x + row.w - 36.0, row.y + 5.0, 30.0, 30.0))
            .collect();
        Self {
            panel,
            class_rows,
            cancel: Rect::new(panel.x + 24.0, panel.y + panel.h - 61.0, 150.0, 37.0),
            continue_button: Rect::new(
                panel.x + panel.w - 224.0,
                panel.y + panel.h - 61.0,
                200.0,
                37.0,
            ),
            attribute_rows,
            attribute_minus,
            attribute_plus,
            preset: Rect::new(
                panel.x + left_width + 24.0,
                panel.y + panel.h - 119.0,
                panel.w - left_width - 48.0,
                37.0,
            ),
        }
    }

    fn hit(
        &self,
        point: (f32, f32),
        stage: CharacterCreationStage,
    ) -> Option<CharacterCreationHover> {
        let point = point.into();
        if self.cancel.contains(point) {
            return Some(CharacterCreationHover::Cancel);
        }
        if self.continue_button.contains(point) {
            return Some(CharacterCreationHover::Continue);
        }
        match stage {
            CharacterCreationStage::Protocol => self
                .class_rows
                .iter()
                .position(|row| row.contains(point))
                .map(CharacterCreationHover::Class),
            CharacterCreationStage::Attributes => self
                .attribute_minus
                .iter()
                .position(|button| button.contains(point))
                .map(CharacterCreationHover::AttributeMinus)
                .or_else(|| {
                    self.attribute_plus
                        .iter()
                        .position(|button| button.contains(point))
                        .map(CharacterCreationHover::AttributePlus)
                })
                .or_else(|| {
                    self.preset
                        .contains(point)
                        .then_some(CharacterCreationHover::Preset)
                })
                .or_else(|| {
                    self.attribute_rows
                        .iter()
                        .position(|row| row.contains(point))
                        .map(CharacterCreationHover::Attribute)
                }),
        }
    }
}

pub struct AsciiApp {
    game: WorldState,
    terminal: TerminalView,
    zone_views: BTreeMap<project_rl::content::ContentId, TerminalView>,
    zone_decor: BTreeMap<project_rl::content::ContentId, crate::test_sector::SectorDecor>,
    facing: Direction,
    rules: GameRules,
    character_classes: CharacterClassCatalog,
    character_class: Option<CharacterClassId>,
    character_creation: Option<CharacterCreation>,
    texts: TextCatalog,
    loot: LootCatalog,
    expeditions: ExpeditionCatalog,
    regional_worlds: RegionalWorldCatalog,
    regional_zones: BTreeMap<ContentId, RegionCoord>,
    generation_version: u8,
    seed: u64,
    actor_glyphs: BTreeMap<EntityId, char>,
    selected_target: Option<EntityId>,
    attack_aim: Option<AttackAim>,
    attack_aim_technique: Option<TechniqueId>,
    attack_aim_pointer: Option<(f32, f32)>,
    component_selection: Option<ComponentSelection>,
    visual_cues: VisualCuePlayer,
    floating_messages: Vec<FloatingMessage>,
    trace_cells: BTreeMap<GridPos, Direction>,
    traces_visible_until: f64,
    log: Vec<String>,
    active_weapon_slot: u8,
    inventory_open: bool,
    inventory_selection: usize,
    inventory_filter: InventoryFilter,
    inventory_sort: InventorySort,
    inventory_message: String,
    character_open: bool,
    character_attribute_selection: usize,
    skills_open: bool,
    skill_discipline_selection: usize,
    skill_technique_selection: usize,
    skill_availability_cache: BTreeMap<DisciplineId, DisciplineAvailability>,
    skill_message: String,
    technique_menu_open: bool,
    technique_menu_selection: usize,
    technique_menu_message: String,
    level_up_notice: Option<LevelUpNotice>,
    observation_report: Vec<String>,
    report_open: bool,
    report_scroll: usize,
    legend_open: bool,
    controls: Controls,
    movement_repeat: controls::MovementRepeater,
    controls_path: std::path::PathBuf,
    graphics: GraphicsState,
    menu: MenuScreen,
    options_return: MenuScreen,
    menu_selection: usize,
    menu_message: String,
    menu_focus: MenuFocus,
    cursor_icon: miniquad::CursorIcon,
    quit_requested: bool,
    options_selection: usize,
    options_scroll: usize,
    rebinding: bool,
    options_message: String,
    history: Vec<RecordedCommand>,
    suspension_path: std::path::PathBuf,
    session_lock: Option<std::fs::File>,
}

impl AsciiApp {
    pub fn new() -> Result<Self, String> {
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        let (bindings, message) = Controls::load(&app.controls_path, controls::detect_layout());
        app.controls = bindings;
        app.options_message = message;
        let (settings, message) = graphics::startup();
        app.graphics.active = *settings;
        app.graphics.draft = *settings;
        app.graphics.message = message.clone();
        app.session_lock = Some(suspension::session_lock(
            &app.suspension_path.with_extension("lock"),
        )?);
        app.open_menu(MenuScreen::Main);
        Ok(app)
    }

    pub fn update(&mut self) {
        if self.quit_requested {
            return;
        }
        let previous_cursor = self.cursor_icon;
        let now = get_time();
        if !self.tick_graphics(now) {
            let input = self.graphics.active.transform_input(InputFrame::capture());
            self.update_input_at(&input, Some(now));
        }
        self.apply_graphics_window_change();
        let cursor_icon = if self.attack_aim.is_some() {
            miniquad::CursorIcon::Crosshair
        } else if self.menu_focus.hovered.is_some() && !self.rebinding {
            miniquad::CursorIcon::Pointer
        } else {
            miniquad::CursorIcon::Default
        };
        if cursor_icon != previous_cursor {
            miniquad::window::set_mouse_cursor(cursor_icon);
        }
        self.cursor_icon = cursor_icon;
    }

    fn apply_graphics_window_change(&mut self) {
        if let Some((previous, settings)) = self.graphics.pending.take() {
            if previous.mode != settings.mode {
                set_fullscreen(settings.mode == WindowMode::Borderless);
            }
            if settings.mode == WindowMode::Windowed
                && (previous.mode != settings.mode
                    || previous.windowed_size != settings.windowed_size)
            {
                request_new_screen_size(
                    settings.windowed_size[0] as f32,
                    settings.windowed_size[1] as f32,
                );
            }
        }
    }

    /// Native render smoke check, debug builds only. Never loads or consumes the
    /// user's run/configuration and never writes settings. Images contain only
    /// this deterministic fixture's framebuffer, not the user's desktop.
    #[cfg(debug_assertions)]
    pub async fn capture_cold_start(output: &std::path::Path, scene: &str) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic-run.json");
        app.graphics.active.mode = WindowMode::Windowed;
        for _ in 0..60 {
            app.draw();
            next_frame().await;
        }
        match scene {
            "game" => {}
            "main" => app.open_menu(MenuScreen::Main),
            "resume" => {
                app.suspension()?.write(&app.suspension_path)?;
                app.open_menu(MenuScreen::Main);
            }
            "new-run-confirmation" => {
                app.suspension()?.write(&app.suspension_path)?;
                app.open_menu(MenuScreen::ConfirmNewRun);
            }
            "pause" => app.open_menu(MenuScreen::Pause),
            "options" => app.open_menu(MenuScreen::Options),
            "graphics" => app.open_menu(MenuScreen::Graphics),
            "controls" => app.open_menu(MenuScreen::Controls),
            "legend" => app.legend_open = true,
            "character" => {
                app.begin_character_creation(false)?;
                let creation = app
                    .character_creation
                    .take()
                    .ok_or("Création de personnage de diagnostic absente.")?;
                app.rebuild_run_with_character_class(&creation)?;
                app.character_open = true;
            }
            "character-creation" => app.begin_character_creation(false)?,
            "character-attributes" => {
                app.begin_character_creation(false)?;
                if let Some(creation) = &mut app.character_creation {
                    creation.stage = CharacterCreationStage::Attributes;
                }
            }
            "floating-feedback" => {
                let at = app
                    .game
                    .player_position()
                    .ok_or("Joueur de diagnostic absent.")?;
                let now = get_time();
                app.push_floating_message("−4 PV", at, FloatingMessageTone::Damage, now);
                app.push_floating_message("BRÛLURE ×1", at, FloatingMessageTone::Status, now);
                app.push_floating_message(
                    "NIVEAU 2 !  +1 PT COMP.",
                    at,
                    FloatingMessageTone::Progression,
                    now,
                );
            }
            "level-up" => app.open_level_up_screen(LevelUpNotice {
                level: 2,
                skill_points_awarded: 1,
            }),
            "maintenance" => {
                // Observe the checkpoint without occupying a worker route or
                // an interaction cell around the relay.
                app.walk_fixture_to(GridPos::new(45, 27))?;
                for _ in 0..96 {
                    if app.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                        return Err("Maintenance diagnostic wait rejected".into());
                    }
                    app.capture_events_at(Some(0.0));
                }
                let order = "core:restore_checkpoint_power"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                if app
                    .game
                    .active_facility()
                    .and_then(|facility| facility.repair_status(&order))
                    != Some(project_rl::facility::RepairStatus::Completed)
                {
                    return Err("Maintenance diagnostic did not complete".into());
                }
            }
            "maintenance-delivery" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                app.walk_fixture_to(GridPos::new(27, 31))?;
                app.facing = Direction::East;
                if app.execute_command(GameCommand::Interact {
                    target: GridPos::new(28, 31),
                }) != CommandOutcome::Applied
                {
                    return Err("Maintenance material delivery rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if app
                    .game
                    .player_inventory()
                    .iter()
                    .any(|entry| entry.item() == &regulator)
                    || !app.log.iter().any(|line| line.contains("vous livrez"))
                {
                    return Err("Maintenance material delivery was not committed".into());
                }
            }
            "maintenance-alert" => {
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(get_time()));
                if app.visible_local_alert_summary().is_none()
                    || !app.log.iter().any(|line| line.contains("ALERTE LOCALE"))
                {
                    return Err("Visible maintenance alert was not presented".into());
                }
            }
            "maintenance-security-alarm" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                let owner = "core:maintenance_collective"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(45, 27))?;
                for _ in 0..96 {
                    if app.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                        return Err("Security alarm diagnostic wait rejected".into());
                    }
                    app.capture_events_at(Some(0.0));
                }
                // Open the checkpoint door, then step back outside so its
                // configured lockdown can engage without enclosing the player.
                app.walk_fixture_to(GridPos::new(52, 29))?;
                app.walk_fixture_to(GridPos::new(52, 27))?;
                app.game
                    .spawn_ground_item_with_owner(GridPos::new(52, 27), regulator, 1, Some(owner))
                    .map_err(|error| error.to_string())?;
                app.game.drain_events();
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Security alarm diagnostic pickup rejected".into());
                }
                app.capture_events_at(Some(get_time()));
                if app.visible_security_alarm_summary().is_none()
                    || !app
                        .log
                        .iter()
                        .any(|line| line.contains("ALARME DE SÉCURITÉ"))
                {
                    return Err("Visible installed security alarm was not presented".into());
                }
            }
            "maintenance-inventory" => {
                let regulator: ItemId = "core:power_regulator"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                app.walk_fixture_to(GridPos::new(16, 23))?;
                if app.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                    return Err("Maintenance material pickup rejected".into());
                }
                app.capture_events_at(Some(0.0));
                app.inventory_selection = app
                    .game
                    .player_inventory()
                    .iter()
                    .position(|entry| entry.item() == &regulator)
                    .ok_or("Maintenance material missing from inventory")?;
                app.inventory_open = true;
            }
            "expedition" => {
                app.walk_expedition_fixture(0)?;
            }
            "expedition-return" => {
                app.walk_expedition_fixture(1)?;
            }
            "regional" => {
                let passage = TestSector::EXPANDED_REGIONAL_PASSAGES
                    .into_iter()
                    .find_map(|(direction, passage)| {
                        (direction == Direction::West).then_some(passage)
                    })
                    .ok_or("Regional diagnostic passage missing")?;
                app.walk_fixture_to(passage.step(Direction::East))?;
                if app.execute_command(GameCommand::Interact { target: passage })
                    != CommandOutcome::Applied
                {
                    return Err("Regional diagnostic travel rejected".into());
                }
                app.capture_events_at(Some(0.0));
            }
            "regional-depth" => {
                let passage = TestSector::EXPANDED_REGIONAL_PASSAGES
                    .into_iter()
                    .find_map(|(direction, passage)| {
                        (direction == Direction::West).then_some(passage)
                    })
                    .ok_or("Regional depth diagnostic passage missing")?;
                app.walk_fixture_to(passage.step(Direction::East))?;
                if app.execute_command(GameCommand::Interact { target: passage })
                    != CommandOutcome::Applied
                {
                    return Err("Regional depth surface travel rejected".into());
                }
                app.capture_events_at(Some(0.0));
                let world =
                    app.regional_worlds
                        .get(&"core:simulation_overworld".parse().map_err(
                            |error: project_rl::content::ContentIdError| error.to_string(),
                        )?)
                        .ok_or("Regional depth world missing")?;
                let descent = project_rl::world::generation::vertical_passage(
                    world.local_map_size(),
                    project_rl::content::RegionVerticalDirection::Down,
                );
                app.walk_fixture_to(descent)?;
                if app.execute_command(GameCommand::Interact { target: descent })
                    != CommandOutcome::Applied
                {
                    return Err("Regional depth descent rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if app.game.current_zone().map(|zone| zone.depth) != Some(1)
                    || app
                        .game
                        .player_position()
                        .and_then(|position| app.terminal.decor.cells.get(&position))
                        != Some(&crate::test_sector::Decor::Ascent)
                {
                    return Err("Regional depth diagnostic did not reach the lower layer".into());
                }
            }
            "regional-signal" => {
                let passage = TestSector::EXPANDED_REGIONAL_PASSAGES
                    .into_iter()
                    .find_map(|(direction, passage)| {
                        (direction == Direction::East).then_some(passage)
                    })
                    .ok_or("Regional signal diagnostic passage missing")?;
                app.walk_fixture_to(passage.step(Direction::West))?;
                if app.execute_command(GameCommand::Interact { target: passage })
                    != CommandOutcome::Applied
                {
                    return Err("Regional signal diagnostic travel rejected".into());
                }
                app.capture_events_at(Some(0.0));
                if crate::terminal_view::detected_navigation_signal_summary(&app.game).is_none() {
                    return Err("Regional diagnostic site emitted no navigation signal".into());
                }
            }
            "expedition-revisit" => {
                app.walk_expedition_fixture(2)?;
            }
            "attack-preview" => {
                app.walk_fixture_to(GridPos::new(65, 21))?;
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                app.begin_attack_aim(None);
                let mut aim = app.attack_aim.ok_or("Area preview did not open")?;
                aim.cursor = aim.cursor.step(Direction::West);
                if app.game.actors().entity_at(aim.cursor).is_some() {
                    return Err("Area preview diagnostic cursor is not on empty ground".into());
                }
                if app
                    .game
                    .player_attack_preview(aim.slot, aim.cursor)
                    .is_err()
                {
                    return Err("Area preview is invalid in diagnostic scene".into());
                }
                app.attack_aim = Some(aim);
            }
            "attack-preview-protected" => {
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                app.begin_attack_aim(None);
                let aim = app
                    .attack_aim
                    .ok_or("Protected area preview did not open")?;
                if app.game.player_attack_preview(aim.slot, aim.cursor)
                    != Err(CommandRejection::ProtectedZone)
                {
                    return Err("Protected area preview did not expose its rejection".into());
                }
                if app
                    .game
                    .player_attack_footprint(aim.slot, aim.cursor)
                    .map_err(|error| format!("Protected area footprint failed: {error:?}"))?
                    .cells()
                    .len()
                    <= 1
                {
                    return Err("Protected area preview collapsed to its cursor".into());
                }
            }
            "empty-area-attack" => {
                let map = project_rl::world::Map::from_ascii(
                    "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
                )
                .map_err(|error| error.to_string())?;
                let origin = GridPos::new(1, 3);
                let target = GridPos::new(6, 3);
                let game = GameState::new_with_rules(map, origin, INITIAL_SEED, app.rules.clone())
                    .map_err(|error| error.to_string())?;
                app.terminal = TerminalView::new(
                    crate::test_sector::SectorDecor::default(),
                    game.map(),
                    game.player_visibility(),
                );
                app.game = WorldState::single(game);
                app.actor_glyphs.clear();
                app.active_weapon_slot = 2;
                app.attack_aim = Some(AttackAim {
                    slot: 2,
                    cursor: target,
                });
                let turn = app.game.turn();
                app.update_input(&InputFrame {
                    pressed: [controls::Binding::key("F")].into(),
                    ..Default::default()
                });
                if app.attack_aim.is_some()
                    || app.game.turn() != turn + 1
                    || app.game.ground_effects().is_empty()
                    || app.visual_cues.active_count() == 0
                    || !app
                        .log
                        .iter()
                        .any(|line| line.contains("ATTAQUE CONFIRMÉE"))
                {
                    return Err("Empty area attack did not complete its visible live path".into());
                }
            }
            "visual-effects" => {
                let origin = app
                    .game
                    .player_position()
                    .ok_or("Visual effect diagnostic has no player")?;
                let id = visual_cue_id("core:flamethrower");
                let attack = app
                    .game
                    .rules()
                    .weapons
                    .get(&id)
                    .ok_or("Visual effect diagnostic has no flamethrower")?
                    .attack();
                let cells = attack
                    .affected_cells(app.game.map(), origin, GridPos::new(origin.x + 5, origin.y))
                    .into_iter()
                    .map(|cell| VisualCueCell::new(cell.position, cell.step));
                let cue = VisualCue::world(id, origin, cells).map_err(|error| error.to_string())?;
                // Capture the overlap where the pressurized jet has reached
                // its long tip while the narrow base is still burning.
                app.visual_cues.play(cue, get_time() - 0.34);
            }
            "burning-status" => {
                let map = project_rl::world::Map::from_ascii(
                    "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
                )
                .map_err(|error| error.to_string())?;
                let mut game = GameState::new_with_rules(
                    map,
                    GridPos::new(1, 3),
                    INITIAL_SEED,
                    app.rules.clone(),
                )
                .map_err(|error| error.to_string())?;
                let target = game
                    .spawn_actor(
                        Actor::new(GridPos::new(8, 3), 30)
                            .map_err(|error| error.to_string())?
                            .with_ai(AiProfile::skirmisher(12, 0, 9)),
                    )
                    .map_err(|error| error.to_string())?;
                game.drain_events();
                app.terminal = TerminalView::new(
                    crate::test_sector::SectorDecor::default(),
                    game.map(),
                    game.player_visibility(),
                );
                app.game = WorldState::single(game);
                app.actor_glyphs.clear();
                app.actor_glyphs.insert(target, 'd');
                app.selected_target = Some(target);
                app.active_weapon_slot = 2;
                app.facing = Direction::East;
                if app.execute_command(GameCommand::Attack { slot: 2, target })
                    != CommandOutcome::Applied
                {
                    return Err("Burning status diagnostic attack rejected".into());
                }
                // The persistent mechanical state remains after its transient
                // attack/status cues have finished.
                app.capture_events_at(Some(get_time() - 5.0));
                if app.game.actors().get(target).map(Actor::position) != Some(GridPos::new(9, 3))
                    || app.burning_status_icon_at(GridPos::new(9, 3))
                        != Some(TerminalStatusIcon::Burning)
                {
                    return Err("Burning status diagnostic has no status icon".into());
                }
            }
            "resume-error" => {
                app.suspension_path = output.join("rejected-run.json");
                let mut saved = app.suspension()?;
                saved.state ^= 1;
                saved.write(&app.suspension_path)?;
                app.open_menu(MenuScreen::Main);
                let rect = MenuLayout::new(
                    app.ui_width(),
                    app.ui_height(),
                    MenuScreen::Main.buttons().len(),
                )
                .buttons[0];
                app.update_input(&InputFrame {
                    pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
                    viewport: Some((app.ui_width(), app.ui_height())),
                    pressed: [controls::Binding::MouseLeft].into(),
                    ..Default::default()
                });
                if app.menu_message.is_empty() || !app.suspension_path.exists() {
                    return Err(
                        "L'échec de reprise doit être visible et préserver le fichier.".to_owned(),
                    );
                }
            }
            "dossier" => {
                for (destination, target) in [
                    (GridPos::new(50, 18), TestSector::CONTROL),
                    (GridPos::new(52, 16), TestSector::LOCKED_DOOR),
                    (
                        TestSector::ARCHIVE_TERMINAL.step(Direction::West),
                        TestSector::ARCHIVE_TERMINAL,
                    ),
                ] {
                    app.walk_fixture_to(destination)?;
                    if app.execute_command(GameCommand::Interact { target })
                        != CommandOutcome::Applied
                    {
                        return Err("Dossier diagnostic interaction rejected".into());
                    }
                    app.capture_events_at(Some(0.0));
                }
                if !app.dossier_available()
                    || app.game.discovered_data_terminal_records().is_empty()
                {
                    return Err("Dossier diagnostic contains no discovered archive".into());
                }
                app.report_open = true;
            }
            "inventory" => {
                let breche: CharacterClassId = "core:breche"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                let (selected_class, attributes) = app
                    .character_classes
                    .iter()
                    .enumerate()
                    .find(|(_, (id, _))| *id == &breche)
                    .map(|(index, (_, class))| (index, class.recommended_attributes()))
                    .ok_or("Le diagnostic d'inventaire ne trouve pas la Brèche")?;
                app.rebuild_run_with_character_class(&CharacterCreation {
                    stage: CharacterCreationStage::Attributes,
                    selected_class,
                    selected_attribute: 0,
                    attributes,
                    replace_suspension: false,
                    message: String::new(),
                    hovered: None,
                })?;
                app.inventory_filter = InventoryFilter::Armor;
                app.inventory_open = true;
            }
            "skills" => app.skills_open = true,
            "techniques" => {
                let technique: TechniqueId = "core:rec_01"
                    .parse()
                    .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
                if app.execute_command(GameCommand::LearnTechnique { technique })
                    != CommandOutcome::AppliedWithoutTime
                {
                    return Err("Technique menu diagnostic learning rejected".into());
                }
                app.capture_events_at(Some(0.0));
                app.open_technique_menu();
            }
            _ => return Err(format!("Scène de diagnostic inconnue : {scene}")),
        }
        for _ in 0..3 {
            app.draw();
            next_frame().await;
        }
        app.draw();
        let path = output.join("cold-start.png");
        if path.exists() {
            return Err("Capture déjà présente".to_owned());
        }
        // Capture once, at the end: neither a warm-up nor a previous framebuffer
        // readback may change the texture bindings before the reproduction.
        let screenshot = crate::ui_capture::framebuffer()?;
        screenshot.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
        let mut probes = if let Some(creation) = &app.character_creation {
            let layout = CharacterCreationLayout::new(
                app.ui_width(),
                app.ui_height(),
                app.character_classes.iter().count(),
            );
            let mut probes = vec![layout.cancel, layout.continue_button];
            if creation.stage == CharacterCreationStage::Protocol {
                probes.extend(layout.class_rows);
            } else {
                probes.push(layout.preset);
            }
            probes
        } else if app.menu == MenuScreen::Controls {
            vec![Rect::new(18.0, 24.0, 300.0, 38.0)]
        } else if app.menu != MenuScreen::Hidden {
            MenuLayout::new(app.ui_width(), app.ui_height(), app.menu_labels().len())
                .buttons
                .into_iter()
                .enumerate()
                .filter_map(|(index, rect)| app.menu_row_enabled(index).then_some(rect))
                .collect()
        } else if app.technique_menu_open {
            let layout = TechniqueQuickMenuLayout::new(
                app.ui_width(),
                app.ui_height(),
                app.technique_menu_selection,
                app.active_learned_techniques().len(),
            );
            vec![Rect::new(
                layout.panel.x + 14.0,
                layout.panel.y + 7.0,
                layout.panel.w - 28.0,
                38.0,
            )]
        } else if app.inventory_open || app.character_open || app.skills_open {
            vec![Rect::new(38.0, 37.0, 380.0, 38.0)]
        } else if app.report_open {
            vec![Rect::new(55.0, 50.0, 360.0, 35.0)]
        } else {
            vec![Rect::new(6.0, 10.0, 300.0, 30.0)]
        };
        if scene == "resume-error" {
            let area = MenuLayout::new(
                app.ui_width(),
                app.ui_height(),
                MenuScreen::Main.buttons().len(),
            )
            .resume_error();
            probes.push(Rect::new(area.x, area.y + 30.0, area.w, area.h - 30.0));
        }
        for probe in probes {
            if !crate::ui_capture::text_is_visible(
                &screenshot,
                probe,
                app.ui_scale() * screen_dpi_scale(),
            ) {
                return Err(format!(
                    "{scene} : texte illisible au démarrage à froid ({probe:?})."
                ));
            }
        }
        eprintln!("[COLD UI] {scene} : contraste OK, {}", path.display());
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub async fn capture_ui_checks(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.graphics.active.mode = WindowMode::Windowed;
        let before = suspension::fingerprint(&app.game);
        // Start exactly like gameplay. A special text warm-up used to hide a
        // texture-cache bug when menus expanded the font atlas after the world.
        for _ in 0..60 {
            app.draw();
            next_frame().await;
        }
        for (name, size, percent, menu, row, mode) in [
            (
                "pause-hover",
                [1280, 800],
                100,
                MenuScreen::Pause,
                2,
                WindowMode::Windowed,
            ),
            (
                "abandon-confirmation",
                [1280, 800],
                100,
                MenuScreen::ConfirmAbandon,
                0,
                WindowMode::Windowed,
            ),
            (
                "options-hover",
                [1280, 800],
                100,
                MenuScreen::Options,
                1,
                WindowMode::Windowed,
            ),
            (
                "graphics-small",
                [960, 540],
                150,
                MenuScreen::Graphics,
                2,
                WindowMode::Windowed,
            ),
            (
                "controls-scrolled",
                [960, 540],
                200,
                MenuScreen::Controls,
                1,
                WindowMode::Windowed,
            ),
            (
                "borderless",
                [1280, 800],
                100,
                MenuScreen::Graphics,
                0,
                WindowMode::Borderless,
            ),
            (
                "restored-window",
                [1280, 800],
                100,
                MenuScreen::Graphics,
                0,
                WindowMode::Windowed,
            ),
        ] {
            if is_quit_requested() {
                return Err("Diagnostic interrompu.".to_owned());
            }
            let previous = app.graphics.active;
            app.graphics.active.mode = mode;
            app.graphics.active.windowed_size = size;
            app.graphics.active.ui_scale_percent = percent;
            app.graphics.pending = Some((previous, app.graphics.active));
            app.apply_graphics_window_change();
            // Resize requests are asynchronous. Require a usable, stable viewport
            // instead of mistaking a transient/minimized 1x1 buffer for success.
            let deadline = get_time() + 3.0;
            let mut stable_frames = 0;
            loop {
                clear_background(BLACK);
                next_frame().await;
                let ready = screen_width() >= 640.0
                    && screen_height() >= 480.0
                    && (mode == WindowMode::Borderless
                        || ((screen_width() - size[0] as f32).abs() < 1.0
                            && (screen_height() - size[1] as f32).abs() < 1.0));
                stable_frames = if ready { stable_frames + 1 } else { 0 };
                if stable_frames >= 3 {
                    break;
                }
                if is_quit_requested() || get_time() >= deadline {
                    return Err(format!(
                        "{name} : fenêtre non disponible ou taille non appliquée ({} × {}).",
                        screen_width(),
                        screen_height()
                    ));
                }
            }
            app.open_menu(menu);
            let rect = if menu == MenuScreen::Controls {
                ControlsLayout::new(app.ui_width(), app.ui_height(), 0, Action::ALL.len() + 1)
                    .row(row)
                    .unwrap()
            } else {
                MenuLayout::new(app.ui_width(), app.ui_height(), menu.buttons().len()).buttons[row]
            };
            app.update_input(&InputFrame {
                pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
                viewport: Some((app.ui_width(), app.ui_height())),
                wheel_y: if menu == MenuScreen::Controls {
                    -10.0
                } else {
                    0.0
                },
                ..Default::default()
            });
            // New labels/sizes can grow Macroquad's glyph atlas on the first
            // draw. Capture the settled frame, as seen after normal frame updates.
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let screenshot = crate::ui_capture::framebuffer()?;
            if !crate::ui_capture::text_is_visible(
                &screenshot,
                rect,
                app.ui_scale() * screen_dpi_scale(),
            ) {
                return Err(format!(
                    "{name} : le texte du bouton n'est pas lisible dans la capture."
                ));
            }
            let path = output.join(format!("{name}.png"));
            if path.exists() {
                return Err(format!("Capture déjà présente : {}", path.display()));
            }
            screenshot.export_png(path.to_str().ok_or("Chemin de capture non UTF-8")?);
            eprintln!(
                "[UI CHECK] {name}: window {}x{}, GUI {:.0} %, frame {}x{}",
                screen_width(),
                screen_height(),
                app.ui_scale() * 100.0,
                screenshot.width,
                screenshot.height
            );
            next_frame().await;
        }
        if suspension::fingerprint(&app.game) != before {
            return Err("La simulation a été modifiée.".to_owned());
        }
        app.open_menu(MenuScreen::Hidden);
        for (name, destination, interaction) in [
            ("ville-place", GridPos::new(27, 23), None),
            ("ville-porte-fermee", GridPos::new(32, 16), None),
            (
                "ville-porte-ouverte",
                GridPos::new(32, 16),
                Some(GridPos::new(32, 15)),
            ),
            ("ville-archives-verrouillees", GridPos::new(52, 16), None),
            (
                "ville-console",
                GridPos::new(50, 18),
                Some(TestSector::CONTROL),
            ),
            (
                "ville-archives-ouvertes",
                GridPos::new(52, 16),
                Some(TestSector::LOCKED_DOOR),
            ),
            (
                "ville-terminal-archives",
                GridPos::new(47, 7),
                Some(TestSector::ARCHIVE_TERMINAL),
            ),
            ("ville-sortie", GridPos::new(61, 21), None),
            ("exterieur", GridPos::new(65, 21), None),
        ] {
            app.walk_fixture_to(destination)?;
            if let Some(target) = interaction {
                if let CommandOutcome::Rejected(error) =
                    app.execute_command(GameCommand::Interact { target })
                {
                    return Err(format!("Interaction de contrôle refusée : {error:?}"));
                }
                app.capture_events_at(Some(0.0));
            }
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let path = output.join(format!("{name}.png"));
            if path.exists() {
                return Err("Capture déjà présente".to_owned());
            }
            crate::ui_capture::framebuffer()?.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
            eprintln!(
                "[WORLD CHECK] {name}: {:?}, tour {}",
                app.game.player_position(),
                app.game.turn()
            );
            next_frame().await;
        }
        for _ in 0..2 {
            app.terminal.draw_debug_overview(&app.game);
            next_frame().await;
        }
        app.terminal.draw_debug_overview(&app.game);
        let path = output.join("plan-diagnostic-ville-exterieur.png");
        if path.exists() {
            return Err("Capture déjà présente".to_owned());
        }
        crate::ui_capture::framebuffer()?.export_png(path.to_str().ok_or("Chemin non UTF-8")?);
        eprintln!(
            "[UI CHECK] OK: 7 menus sans mutation + 9 étapes jouées + 1 plan de diagnostic. Aucun réglage ni suspension utilisateur touché."
        );
        Ok(())
    }

    fn tick_graphics(&mut self, now: f64) -> bool {
        let expired = self.graphics.tick(now);
        if expired {
            self.open_menu(MenuScreen::Graphics);
        }
        expired
    }

    /// Diagnostic/test traversal uses real commands and opens ordinary doors;
    /// it never teleports, reveals the map or touches a user's files.
    #[cfg(any(debug_assertions, test))]
    fn walk_expedition_fixture(&mut self, stage: u8) -> Result<(), String> {
        let source = if self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION {
            TestSector::EXPANDED_EXPEDITION_PASSAGE
        } else {
            TestSector::LEGACY_EXPEDITION_PASSAGE
        };
        self.walk_fixture_to(source.step(Direction::West))?;
        self.game
            .passage(source)
            .ok_or("Missing expedition passage")?;
        if self.execute_command(GameCommand::Interact { target: source }) != CommandOutcome::Applied
        {
            return Err("Outbound travel rejected".into());
        }
        self.capture_events_at(Some(0.0));
        let return_passage = self
            .game
            .player_position()
            .ok_or("Missing arrival position")?;
        let loot = self
            .game
            .ground_items()
            .iter()
            .next()
            .map(|(_, stack)| stack.position())
            .ok_or("Missing destination loot")?;
        self.walk_fixture_to(loot)?;
        if self.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
            return Err("Pickup failed".into());
        }
        self.capture_events_at(Some(0.0));
        if stage >= 1 {
            self.walk_fixture_to(return_passage)?;
            let outcome = self.execute_command(GameCommand::Interact {
                target: return_passage,
            });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Return failed: {outcome:?}"));
            }
            self.capture_events_at(Some(0.0));
        }
        if stage >= 2 {
            let outcome = self.execute_command(GameCommand::Interact { target: source });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Revisit failed: {outcome:?}"));
            }
            self.capture_events_at(Some(0.0));
            if self.game.ground_items().item_at(loot).is_some() {
                return Err("Loot respawned on revisit".into());
            }
        }
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    fn walk_fixture_to(&mut self, goal: GridPos) -> Result<(), String> {
        for _ in 0..300 {
            let origin = self.game.player_position().ok_or("Joueur absent")?;
            if origin == goal {
                return Ok(());
            }
            let mut navigation = self.game.map().clone();
            for y in 0..navigation.height() as i32 {
                for x in 0..navigation.width() as i32 {
                    let p = GridPos::new(x, y);
                    if navigation.tile(p).is_some_and(|t| {
                        t.terrain == Terrain::Door(project_rl::world::DoorState::Closed)
                    }) {
                        navigation
                            .set_terrain(p, Terrain::Floor)
                            .map_err(|e| e.to_string())?;
                    }
                }
            }
            if project_rl::world::find_path(&navigation, origin, goal, 15000, |p| {
                Some(p) != self.game.exit()
            })
            .is_none()
            {
                return Err("Trajet de contrôle introuvable".to_owned());
            }
            let Some(path) = project_rl::world::find_path(&navigation, origin, goal, 15000, |p| {
                self.game.actors().entity_at(p).is_none() && Some(p) != self.game.exit()
            }) else {
                // A worker can temporarily occupy the only doorway. Diagnostic
                // traversal waits through the real turn loop instead of
                // teleporting or declaring the static route invalid.
                if self.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                    return Err("Attente de contrôle refusée".to_owned());
                }
                self.capture_events_at(Some(0.0));
                continue;
            };
            let next = *path.get(1).ok_or("Trajet de contrôle vide")?;
            let command = if self.game.map().is_walkable(next) {
                GameCommand::Move(
                    Direction::from_delta(next.x - origin.x, next.y - origin.y).unwrap(),
                )
            } else {
                GameCommand::Interact { target: next }
            };
            if let CommandOutcome::Rejected(error) = self.execute_command(command) {
                return Err(format!("Trajet refusé : {error:?}"));
            }
            self.capture_events_at(Some(0.0));
        }
        Err("Trajet de contrôle trop long".to_owned())
    }

    fn update_input(&mut self, input: &InputFrame) {
        self.update_input_at(input, None);
    }

    fn update_input_at(&mut self, input: &InputFrame, captured_at: Option<f64>) {
        // A successful save freezes the snapshot until the application exits.
        // In particular, input captured alongside an OS close cannot take a turn.
        if self.quit_requested {
            self.movement_repeat.clear();
            return;
        }
        if input.pause
            || self.menu != MenuScreen::Hidden
            || self.legend_open
            || self.inventory_open
            || self.character_open
            || self.skills_open
            || self.technique_menu_open
            || self.report_open
            || self.attack_aim.is_some()
            || self.component_selection.is_some()
            || self.character_creation.is_some()
            || self.game.status() != RunStatus::Active
        {
            self.movement_repeat.clear();
        }
        self.menu_focus.begin_frame(input.pointer);
        if self.character_creation.is_some() {
            self.update_character_creation(input);
            return;
        }
        if input.pause {
            if self.rebinding {
                self.rebinding = false;
                self.options_message = "Réattribution annulée.".to_owned();
            } else if self.menu != MenuScreen::Hidden {
                self.open_menu(self.menu_back());
            } else if self.legend_open {
                self.legend_open = false;
            } else if self.inventory_open {
                self.inventory_open = false;
            } else if self.character_open {
                self.character_open = false;
            } else if self.skills_open {
                self.skills_open = false;
                self.level_up_notice = None;
            } else if self.technique_menu_open {
                self.close_technique_menu();
            } else if self.report_open {
                self.report_open = false;
            } else if self.attack_aim.take().is_some() {
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
                self.push_log("VISÉE ANNULÉE".to_owned());
            } else if self.component_selection.take().is_some() {
                self.push_log("SÉLECTION DE COMPOSANT ANNULÉE".to_owned());
            } else {
                self.open_menu(MenuScreen::Pause);
            }
            return;
        }
        if self.menu == MenuScreen::Controls {
            self.update_options(input);
            return;
        }
        if self.menu != MenuScreen::Hidden {
            self.update_menu(input);
            return;
        }
        if self.component_selection.is_some() {
            self.update_component_selection(input);
            return;
        }
        if self.technique_menu_open {
            self.update_technique_menu(input);
            return;
        }
        if self.controls.pressed(Action::Legend, input) {
            self.legend_open = !self.legend_open;
            if self.legend_open {
                self.attack_aim = None;
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
                self.inventory_open = false;
                self.character_open = false;
                self.skills_open = false;
                self.level_up_notice = None;
                self.report_open = false;
            }
            return;
        }
        if self.legend_open {
            if input.pointer.is_some() {
                self.menu_focus.hovered = Some(0);
            }
            if input.pressed.contains(&controls::Binding::MouseLeft) {
                self.legend_open = false;
            }
            return;
        }
        if self.controls.pressed(Action::Character, input) {
            self.character_open = !self.character_open;
            if self.character_open {
                self.attack_aim = None;
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
                self.inventory_open = false;
                self.skills_open = false;
                self.level_up_notice = None;
                self.report_open = false;
            }
            return;
        }
        if self.character_open {
            let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
            let layout = CharacterLayout::new(width, height);
            let hovered_attribute = input.pointer.and_then(|point| {
                layout
                    .attribute_rows
                    .iter()
                    .position(|rect| rect.contains(point.into()))
            });
            let hovered_action = input.pointer.and_then(|point| {
                layout
                    .actions
                    .iter()
                    .position(|rect| rect.contains(point.into()))
            });
            self.menu_focus.hovered = hovered_attribute
                .or_else(|| hovered_action.map(|index| PrimaryAttribute::ALL.len() + index));
            let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
            if self.controls.pressed(Action::MenuUp, input) {
                self.character_attribute_selection =
                    self.character_attribute_selection.saturating_sub(1);
            } else if self.controls.pressed(Action::MenuDown, input) {
                self.character_attribute_selection =
                    (self.character_attribute_selection + 1).min(PrimaryAttribute::ALL.len() - 1);
            } else if clicked && let Some(index) = hovered_attribute {
                self.character_attribute_selection = index;
            } else if self.controls.pressed(Action::Inventory, input)
                || clicked && hovered_action == Some(0)
            {
                self.character_open = false;
                self.inventory_open = true;
                self.clamp_inventory_selection();
                self.inventory_message =
                    "Sélectionner un objet pour l'équiper ou l'utiliser.".to_owned();
            } else if self.controls.pressed(Action::Skills, input)
                || clicked && hovered_action == Some(1)
            {
                self.character_open = false;
                self.skills_open = true;
                self.level_up_notice = None;
                self.clamp_skill_selection();
                self.skill_message =
                    "Sélectionner une technique pour lire sa description.".to_owned();
            } else if clicked && hovered_action == Some(2) {
                self.character_open = false;
            }
            return;
        }
        if self.report_open {
            let dossier_line_count = self.dossier_lines().len();
            let (width, _) = input.viewport.unwrap_or((1280.0, 800.0));
            let close = Rect::new((width - 140.0).max(60.0), 50.0, 80.0, 31.0);
            let close_hovered = input
                .pointer
                .is_some_and(|point| close.contains(point.into()));
            self.menu_focus.hovered = close_hovered.then_some(0);
            if self.controls.pressed(Action::Report, input)
                || input.pressed.contains(&controls::Binding::MouseLeft) && close_hovered
            {
                self.report_open = false;
            } else if self.controls.pressed(Action::MenuDown, input) {
                self.report_scroll =
                    (self.report_scroll + 1).min(dossier_line_count.saturating_sub(1));
            } else if self.controls.pressed(Action::MenuUp, input) {
                self.report_scroll = self.report_scroll.saturating_sub(1);
            } else if input.wheel_y > 0.0 {
                self.report_scroll = self
                    .report_scroll
                    .saturating_sub(wheel_steps(input.wheel_y));
            } else if input.wheel_y < 0.0 {
                self.report_scroll = self
                    .report_scroll
                    .saturating_add(wheel_steps(input.wheel_y))
                    .min(dossier_line_count.saturating_sub(1));
            }
            return;
        }
        if self.controls.pressed(Action::Report, input) {
            if !self.dossier_available() {
                self.push_log("Dossier vide : aucun relevé ni archive découverte.".to_owned());
            } else {
                self.attack_aim = None;
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
                self.report_open = true;
                self.report_scroll = 0;
                self.character_open = false;
            }
            return;
        }
        if self.controls.pressed(Action::Skills, input) {
            self.skills_open = !self.skills_open;
            self.level_up_notice = None;
            if self.skills_open {
                self.attack_aim = None;
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
            }
            self.inventory_open = false;
            self.character_open = false;
            self.clamp_skill_selection();
            if self.skills_open {
                self.skill_message =
                    "Sélectionner une technique pour lire sa description.".to_owned();
            }
            return;
        }
        if self.controls.pressed(Action::Inventory, input) {
            self.inventory_open = !self.inventory_open;
            if self.inventory_open {
                self.attack_aim = None;
                self.attack_aim_technique = None;
                self.attack_aim_pointer = None;
            }
            self.skills_open = false;
            self.level_up_notice = None;
            self.character_open = false;
            self.clamp_inventory_selection();
            if self.inventory_open {
                self.inventory_message =
                    "Sélectionner un objet pour l'équiper ou l'utiliser.".to_owned();
            }
            return;
        }

        if self.inventory_open {
            self.update_inventory(input);
            return;
        }
        if self.skills_open {
            self.update_skills(input);
            return;
        }

        if self.controls.pressed(Action::Restart, input) {
            let next_seed = self.seed.wrapping_add(1);
            match Self::from_seed(
                next_seed,
                self.rules.clone(),
                self.texts.clone(),
                self.loot.clone(),
                self.expeditions.clone(),
            ) {
                Ok(mut next_run) => {
                    next_run.character_class = self.character_class.clone();
                    next_run.controls = self.controls.clone();
                    next_run.controls_path = self.controls_path.clone();
                    next_run.options_message = self.options_message.clone();
                    next_run.graphics = self.graphics.clone();
                    next_run.suspension_path = self.suspension_path.clone();
                    next_run.session_lock = self.session_lock.take();
                    *self = next_run;
                }
                Err(_) => self.push_log("Impossible de commencer une nouvelle partie.".to_owned()),
            }
            return;
        }

        if self.game.status() != RunStatus::Active {
            return;
        }

        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        if self.has_controlled_companion() {
            let layout = CompanionBarLayout::new(width, height);
            let hovered_behavior = input.pointer.and_then(|point| {
                layout
                    .behavior_buttons
                    .iter()
                    .position(|rect| rect.contains(point.into()))
            });
            if let Some(index) = hovered_behavior {
                self.menu_focus.hovered = Some(COMPANION_ACTION_FOCUS_BASE + index);
            }
            if input.pressed.contains(&controls::Binding::MouseLeft)
                && let Some(index) = hovered_behavior
            {
                let behavior = CompanionBehavior::ALL[index];
                if let CommandOutcome::Rejected(reason) =
                    self.execute_command(GameCommand::SetCompanionBehavior { behavior })
                {
                    self.push_log(command_rejection_message(reason).to_owned());
                }
                self.capture_events();
                return;
            }
        }
        let wait_button = if self.game.player_technique_preparation().is_some() {
            preparation_continue_rect(width, height)
        } else {
            wait_turn_rect(height, &self.controls.label(Action::Wait))
        };
        let wait_hovered = input
            .pointer
            .is_some_and(|point| wait_button.contains(point.into()));
        if wait_hovered {
            self.menu_focus.hovered = Some(WAIT_ACTION_FOCUS);
        }
        if wait_hovered && input.pressed.contains(&controls::Binding::MouseLeft) {
            if let CommandOutcome::Rejected(reason) =
                self.execute_command(self.preparation_wait_command())
            {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            self.capture_events();
            return;
        }

        if self.attack_aim.is_some() {
            self.update_attack_aim(input);
            return;
        }

        if self.controls.pressed(Action::QuickTechniques, input) {
            self.open_technique_menu();
            return;
        }

        if input.pressed.contains(&controls::Binding::MouseLeft)
            && let Some(target) = self
                .attack_pointer_cell(input)
                .filter(|position| self.game.player_visibility().is_visible(*position))
            && self.begin_pointer_attack_aim(target, input.pointer)
        {
            return;
        }

        if let Some(slot) = pressed_weapon_slot(&self.controls, input) {
            self.select_weapon_slot(slot);
            return;
        }

        if self.controls.pressed(Action::CycleTarget, input) {
            self.cycle_target();
            return;
        }

        let repeated_movement = captured_at.and_then(|now| {
            self.movement_repeat
                .poll(&self.controls, input, now)
                .and_then(movement_direction)
        });

        for (action, direction) in [
            (Action::MoveNorth, Direction::North),
            (Action::MoveEast, Direction::East),
            (Action::MoveSouth, Direction::South),
            (Action::MoveWest, Direction::West),
        ] {
            if self.controls.pressed(action, input) {
                self.facing = direction;
                break;
            }
        }
        if let Some(direction) = repeated_movement {
            self.facing = direction;
        }
        let command = if self.controls.pressed(Action::Interact, input) {
            self.interaction_command()
        } else if self.controls.pressed(Action::PickUp, input) {
            Some(GameCommand::PickUp)
        } else if self.controls.pressed(Action::Analyze, input) {
            self.target_analysis_command()
        } else if self.controls.pressed(Action::Traces, input) {
            self.trace_reading_command()
        } else if self.controls.pressed(Action::Walls, input) {
            self.wall_analysis_command()
        } else if self.controls.pressed(Action::Threat, input) {
            self.threat_analysis_command()
        } else if self.controls.pressed(Action::Multiple, input) {
            technique_id("core:rec_09").and_then(|id| self.technique_command(id))
        } else if self.controls.pressed(Action::Corrosion, input) {
            self.corrosion_command()
        } else if self.controls.pressed(Action::Pulse, input) {
            self.ability_command()
        } else if self.controls.pressed(Action::Attack, input) {
            self.weapon_command(input.pointer)
        } else {
            read_movement_command(&self.game, self.active_weapon_slot, &self.controls, input)
                .or_else(|| repeated_movement.map(GameCommand::Move))
        };
        if let Some(command) = command {
            let command = if matches!(command, GameCommand::Wait) {
                self.preparation_wait_command()
            } else {
                command
            };
            let movement_attempt = matches!(command, GameCommand::Move(_));
            let outcome = self.execute_command(command);
            if let CommandOutcome::Rejected(reason) = outcome {
                if movement_attempt {
                    self.movement_repeat.clear();
                }
                self.push_log(command_rejection_message(reason).to_owned());
            }
            self.capture_events();
        }
    }

    fn preparation_wait_command(&self) -> GameCommand {
        if self.game.rules().wait_continues_technique_preparation {
            GameCommand::Wait
        } else {
            self.game
                .player_preparation_continuation_command()
                .unwrap_or(GameCommand::Wait)
        }
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(5, 8, 12, 255));

        let resolved_attack_preview = self.attack_aim.map(|aim| self.aimed_attack_preview(aim));
        let resolved_attack_footprint = self.attack_aim.map(|aim| self.aimed_attack_footprint(aim));
        let attack_preview = self.attack_aim.map(|aim| TerminalAttackPreview {
            cells: resolved_attack_footprint
                .as_ref()
                .and_then(|preview| preview.as_ref().ok())
                .map_or(&[], |preview| preview.cells()),
            cursor: aim.cursor,
            valid: resolved_attack_preview.as_ref().is_some_and(Result::is_ok),
        });
        let navigation_signal = self.navigation_signal_summary();
        self.terminal.draw(
            &self.game,
            TerminalDrawOptions {
                bounds: self.terminal_bounds(),
                cell_size: self.graphics.active.world_cell_px,
                interact_label: &self.controls.label(Action::Interact),
                legend_label: &self.controls.label(Action::Legend),
                legend_open: self.legend_open,
                attack_preview,
                navigation_signal: navigation_signal.as_deref(),
            },
            |position| {
                self.glyph_at(position)
                    .map(|(glyph, color, accent_color, highlight_color)| {
                        let alert = self.visible_alert_at(position).map(|(kind, _)| kind);
                        TerminalOverlay {
                            symbol: glyph,
                            color: if alert.is_some() {
                                Color::from_rgba(255, 175, 83, 255)
                            } else {
                                color
                            },
                            accent_color,
                            highlight_color,
                            selected: self.is_selected_target_at(position),
                            alert,
                            status_icon: self.burning_status_icon_at(position),
                        }
                    })
            },
        );

        if self.menu == MenuScreen::Hidden
            && !self.legend_open
            && !self.inventory_open
            && !self.character_open
            && !self.skills_open
            && !self.technique_menu_open
            && !self.report_open
            && self.component_selection.is_none()
            && self.character_creation.is_none()
        {
            self.draw_floating_messages(navigation_signal.is_some());
        }

        set_camera(&graphics::ui_camera(self.ui_width(), self.ui_height()));
        self.draw_header();
        self.draw_companion_bar();
        self.draw_footer();
        self.draw_end_message();
        if self.inventory_open {
            self.draw_inventory();
        }
        if self.character_open {
            self.draw_character();
        }
        if self.skills_open {
            self.draw_skills();
        }
        if self.report_open {
            self.draw_dossier();
        }
        if self.menu == MenuScreen::Controls {
            self.draw_options();
        } else if self.menu != MenuScreen::Hidden {
            self.draw_menu();
        }
        if self.character_creation.is_some() {
            self.draw_character_creation();
        }
        if self.component_selection.is_some() {
            self.draw_component_selection();
        }
        if self.technique_menu_open {
            self.draw_technique_menu();
        }
        set_default_camera();
    }

    fn draw_technique_menu(&self) {
        let techniques = self.active_learned_techniques();
        if techniques.is_empty() {
            return;
        }
        let layout = TechniqueQuickMenuLayout::new(
            self.ui_width(),
            self.ui_height(),
            self.technique_menu_selection,
            techniques.len(),
        );
        let theme = UiTheme;
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 4, 7, 205),
        );
        theme.card(layout.panel, true);
        draw_text_bold(
            "TECHNIQUES ACTIVES",
            layout.panel.x + 18.0,
            layout.panel.y + 31.0,
            21.0,
            theme.focus(),
        );
        let hint = if self.technique_menu_message.is_empty() {
            format!(
                "{} ou Entrée : utiliser · Échap : fermer",
                self.controls.label(Action::QuickTechniques)
            )
        } else {
            self.technique_menu_message.clone()
        };
        draw_wrapped_text(
            &hint,
            layout.panel.x + 18.0,
            layout.panel.y + 52.0,
            layout.panel.w - 36.0,
            1,
            13,
            if self.technique_menu_message.is_empty() {
                theme.muted()
            } else {
                theme.focus()
            },
        );

        for (index, row) in &layout.rows {
            let Some(id) = techniques.get(*index) else {
                continue;
            };
            let selected = *index == self.technique_menu_selection;
            let hovered = self.menu_focus.hovered == Some(*index);
            if selected || hovered {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    if selected {
                        theme.surface_selected()
                    } else {
                        theme.surface_raised()
                    },
                );
            }
            if selected {
                draw_rectangle(row.x, row.y, 3.0, row.h, theme.focus());
            }
            draw_wrapped_text(
                &self.technique_name(id),
                row.x + 11.0,
                row.y + 18.0,
                row.w - 22.0,
                1,
                17,
                if selected {
                    theme.focus()
                } else {
                    theme.text()
                },
            );
            let discipline = self
                .game
                .rules()
                .skills
                .technique(id)
                .map(|definition| self.discipline_name(definition.discipline()))
                .unwrap_or_else(|| "Discipline inconnue".to_owned());
            draw_wrapped_text(
                &format!("{} · {discipline}", technical_reference(id)),
                row.x + 22.0,
                row.y + 35.0,
                row.w - 33.0,
                1,
                12,
                theme.accent(),
            );
        }

        if techniques.len() > layout.rows.len() {
            let track = Rect::new(
                layout.panel.x + layout.panel.w - 7.0,
                layout.panel.y + 66.0,
                3.0,
                (layout.actions[0].y - layout.panel.y - 78.0).max(20.0),
            );
            draw_rectangle(track.x, track.y, track.w, track.h, theme.surface_raised());
            let thumb_h = (track.h * layout.rows.len() as f32 / techniques.len() as f32).max(16.0);
            let maximum_first = techniques.len().saturating_sub(layout.rows.len()).max(1);
            let thumb_y =
                track.y + (track.h - thumb_h) * layout.first_visible as f32 / maximum_first as f32;
            draw_rectangle(track.x, thumb_y, track.w, thumb_h, theme.accent());
        }

        for (index, (button, label)) in layout
            .actions
            .iter()
            .zip(["UTILISER", "ANNULER"])
            .enumerate()
        {
            theme.button(
                *button,
                label,
                self.menu_focus.hovered == Some(techniques.len() + index),
                false,
                true,
                if index == 0 {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Secondary
                },
            );
        }
    }

    fn draw_component_selection(&self) {
        let Some(selection) = &self.component_selection else {
            return;
        };
        let viewport = (self.ui_width(), self.ui_height());
        let first_visible = selection.selected.saturating_sub(7);
        let visible_count = selection
            .components
            .len()
            .saturating_sub(first_visible)
            .min(8);
        let (panel, rows, cancel, confirm) =
            Self::component_selection_layout(viewport, visible_count);
        let theme = UiTheme;
        draw_rectangle(
            0.0,
            0.0,
            viewport.0,
            viewport.1,
            Color::from_rgba(0, 5, 8, 205),
        );
        theme.card(panel, true);
        draw_text_bold(
            "CHOISIR LE COMPOSANT VISÉ",
            panel.x + 18.0,
            panel.y + 31.0,
            21.0,
            theme.focus(),
        );

        for (visible_index, row) in rows.iter().enumerate() {
            let index = first_visible + visible_index;
            let Some(component_id) = selection.components.get(index) else {
                continue;
            };
            let selected = index == selection.selected;
            let hovered = self.menu_focus.hovered == Some(index);
            if selected || hovered {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    if selected {
                        theme.surface_selected()
                    } else {
                        theme.surface_raised()
                    },
                );
            }
            if selected {
                draw_rectangle(row.x, row.y, 3.0, row.h, theme.focus());
            }
            let state = self
                .game
                .actors()
                .get(selection.target)
                .and_then(|actor| actor.body_component(component_id));
            let name = state
                .and_then(|component| {
                    self.texts
                        .resolve(DISPLAY_LOCALE, component.profile().name_key())
                })
                .unwrap_or(component_id.as_str());
            let durability = state.map_or_else(
                || "état indisponible".to_owned(),
                |component| {
                    format!(
                        "{} / {}{}",
                        component.durability(),
                        component.maximum_durability(),
                        if component.is_failed() {
                            " · DÉFAILLANT"
                        } else {
                            ""
                        }
                    )
                },
            );
            draw_text(
                name,
                row.x + 12.0,
                row.y + 23.0,
                17.0,
                if selected {
                    theme.focus()
                } else {
                    theme.text()
                },
            );
            let measured = measure_text(&durability, None, 14, 1.0);
            draw_text(
                &durability,
                row.x + row.w - measured.width - 12.0,
                row.y + 22.0,
                14.0,
                theme.muted(),
            );
        }

        theme.button(
            cancel,
            "ANNULER",
            self.menu_focus.hovered == Some(selection.components.len()),
            false,
            true,
            ButtonTone::Secondary,
        );
        theme.button(
            confirm,
            "CONFIRMER",
            self.menu_focus.hovered == Some(selection.components.len() + 1),
            false,
            true,
            ButtonTone::Primary,
        );
    }

    fn ui_scale(&self) -> f32 {
        self.graphics
            .active
            .ui_scale(screen_width(), screen_height())
    }

    fn interaction_command(&mut self) -> Option<GameCommand> {
        let origin = self.game.player_position()?;
        if self.game.passage(origin).is_some() {
            return Some(GameCommand::Interact { target: origin });
        }
        let candidates: Vec<_> = origin
            .cardinal_neighbors()
            .into_iter()
            .filter(|p| {
                self.game.player_visibility().is_visible(*p)
                    && (self.game.passage(*p).is_some()
                        || self
                            .game
                            .active_facility()
                            .is_some_and(|facility| facility.is_player_interactive_at(*p))
                        || self
                            .game
                            .map()
                            .tile(*p)
                            .is_some_and(|t| t.terrain.is_interactive()))
            })
            .collect();
        let target = if candidates.contains(&origin.step(self.facing)) {
            Some(origin.step(self.facing))
        } else if candidates.len() == 1 {
            candidates.first().copied()
        } else {
            None
        };
        if let Some(target) = target {
            Some(GameCommand::Interact { target })
        } else {
            self.push_log(
                if candidates.is_empty() {
                    "Approchez-vous d'une porte, d'une console, d'une installation ou d'un passage."
                } else {
                    "Plusieurs interactions : faites face à celle souhaitée avec une direction."
                }
                .to_owned(),
            );
            None
        }
    }

    fn ui_width(&self) -> f32 {
        screen_width() / self.ui_scale()
    }
    fn ui_height(&self) -> f32 {
        screen_height() / self.ui_scale()
    }

    fn terminal_bounds(&self) -> Rect {
        let ui_scale = self.ui_scale();
        let companion_height = if self.has_controlled_companion() {
            64.0 * ui_scale
        } else {
            0.0
        };
        Rect::new(
            20.0,
            76.0 * ui_scale,
            screen_width() - 40.0,
            (screen_height() - 180.0 * ui_scale - companion_height).max(100.0),
        )
    }

    fn has_controlled_companion(&self) -> bool {
        !self.game.player_controlled_companions().is_empty()
    }

    fn from_seed(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
    ) -> Result<Self, String> {
        Self::from_seed_version(
            seed,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )
    }

    fn from_seed_version(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
        version: u8,
    ) -> Result<Self, String> {
        let regional_worlds = ascii_regional_world_catalog()?;
        Self::from_seed_version_with_regions(
            seed,
            rules,
            texts,
            loot,
            expeditions,
            regional_worlds,
            version,
        )
    }

    fn from_seed_version_with_regions(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
        regional_worlds: RegionalWorldCatalog,
        version: u8,
    ) -> Result<Self, String> {
        let loot = loot_for_generation_version(&loot, &rules.items, version);
        let rules = rules_for_generation_version(rules, version);
        let mut app = if version >= 5 {
            Self::from_seed_base(
                seed,
                rules,
                texts,
                loot,
                expeditions,
                regional_worlds,
                version,
            )?
        } else {
            Self::from_seed_legacy(
                seed,
                rules,
                texts,
                loot,
                expeditions,
                regional_worlds,
                version,
            )?
        };
        app.generation_version = version;
        app.enable_expedition()?;
        if version >= REGIONAL_TRAVEL_GENERATION_VERSION {
            app.enable_regional_world()?;
        }
        if version >= 5 {
            app.enable_maintenance_fixture()?;
        }
        Ok(app)
    }

    fn enable_expedition(&mut self) -> Result<(), String> {
        let expedition_id = "core:starter_expedition"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let definition = self
            .expeditions
            .get(&expedition_id)
            .ok_or("Définition d'expédition de départ absente.")?
            .clone();
        if self.generation_version >= 7 {
            for owner in definition.player_property_take_authorizations() {
                self.game
                    .grant_player_property_take_authorization(owner.clone());
            }
        }
        let presentation = if self.generation_version >= LAZY_ZONE_GENERATION_VERSION {
            crate::test_expedition::declare(
                &mut self.game,
                &definition,
                self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
            )?
        } else {
            crate::test_expedition::attach(
                &mut self.game,
                self.seed,
                (self.generation_version >= 3).then_some(&self.loot),
                &definition,
                crate::test_expedition::ExpeditionGenerationFeatures {
                    defined_population: self.generation_version
                        >= DEFINED_POPULATION_GENERATION_VERSION,
                    expanded_world: self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
                    pursuit_limits: self.generation_version >= PURSUIT_LEASH_GENERATION_VERSION,
                    pursuit_lifecycle: self.generation_version
                        >= PURSUIT_LIFECYCLE_GENERATION_VERSION,
                    primary_attributes: self.generation_version
                        >= ENEMY_ATTRIBUTES_GENERATION_VERSION,
                    physical_profiles: self.generation_version
                        >= PHYSICAL_PROFILES_GENERATION_VERSION,
                    electronic_systems: self.generation_version
                        >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION,
                    preparation_disruption: self.generation_version
                        >= PREPARATION_DISRUPTION_GENERATION_VERSION,
                    player_relations: self.generation_version
                        >= PLAYER_RELATIONS_GENERATION_VERSION,
                },
            )?
        };
        self.terminal.decor.cells.insert(
            presentation.source_passage,
            crate::test_sector::Decor::Passage,
        );
        self.zone_decor = presentation.decor;
        self.refresh_zone_title();
        self.terminal
            .observe(self.game.map(), self.game.player_visibility());
        Ok(())
    }

    fn enable_regional_world(&mut self) -> Result<(), String> {
        let world_id: ContentId = "core:simulation_overworld"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let world = self
            .regional_worlds
            .get(&world_id)
            .ok_or("Atlas régional de départ absent.")?
            .clone();
        let origin = RegionCoord::new(0, 0, 0);
        let hub = self
            .game
            .current_zone()
            .cloned()
            .ok_or("Zone de départ absente de l'atlas.")?;
        let mut next_regions = self.regional_zones.clone();
        Self::insert_regional_zone(&mut next_regions, hub.id.clone(), origin)?;

        let mut connections = Vec::new();
        for (direction, passage) in TestSector::EXPANDED_REGIONAL_PASSAGES {
            let regional_direction = region_direction(direction);
            let coordinate = origin
                .step(regional_direction)
                .ok_or("Coordonnée régionale hors limites numériques.")?;
            let descriptor = world
                .region(self.seed, coordinate)
                .ok_or("Région voisine hors des limites de l'atlas.")?;
            let destination = crate::test_regional::zone_info(&world, &descriptor)?;
            let arrival = project_rl::world::generation::cardinal_passage(
                world.local_map_size(),
                direction_from_region(regional_direction.opposite()),
            );
            connections.push(ZoneConnectionBlueprint {
                at: passage,
                destination: destination.clone(),
                arrival,
            });
            Self::insert_regional_zone(&mut next_regions, destination.id, coordinate)?;
        }
        self.game
            .declare_deferred_connections_at(hub.id, &connections)?;
        self.regional_zones = next_regions;
        Ok(())
    }

    fn insert_regional_zone(
        regions: &mut BTreeMap<ContentId, RegionCoord>,
        zone: ContentId,
        coordinate: RegionCoord,
    ) -> Result<(), String> {
        if regions.iter().any(|(known_zone, known_coordinate)| {
            known_coordinate == &coordinate && known_zone != &zone
        }) {
            return Err("Deux zones revendiquent la même coordonnée régionale.".into());
        }
        if regions.get(&zone).is_some_and(|known| known != &coordinate) {
            return Err("Une zone revendique deux coordonnées régionales.".into());
        }
        regions.insert(zone, coordinate);
        Ok(())
    }

    fn regional_zone_info(
        &self,
        world: &project_rl::content::RegionalWorldDefinition,
        coordinate: RegionCoord,
    ) -> Result<project_rl::game::ZoneInfo, String> {
        if coordinate == RegionCoord::new(0, 0, 0) {
            let hub_id: ContentId = "core:starter_city"
                .parse()
                .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
            return self
                .game
                .zone_info(&hub_id)
                .cloned()
                .ok_or("Ville de départ absente de l'atlas.".into());
        }
        let descriptor = world
            .region(self.seed, coordinate)
            .ok_or("Coordonnée située hors de l'atlas régional.")?;
        crate::test_regional::zone_info(world, &descriptor)
    }

    fn hub_regional_passage(direction: RegionDirection) -> Option<GridPos> {
        TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(candidate, passage)| {
                (region_direction(candidate) == direction).then_some(passage)
            })
    }

    fn enable_maintenance_fixture(&mut self) -> Result<(), String> {
        let expedition_id = "core:starter_expedition"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let definition = self
            .expeditions
            .get(&expedition_id)
            .ok_or("Définition d'expédition de départ absente.")?;
        let mut facility = definition
            .hub_facility
            .clone()
            .ok_or("Circuit de maintenance de départ absent.")?;
        if self.generation_version < 6 {
            facility.blueprint.owner = None;
            for worker in &mut facility.blueprint.workers {
                worker.affiliation = None;
                worker.witness_profile = None;
                worker.local_alert_profile = None;
            }
            for material in &mut facility.materials {
                material.owner = None;
            }
        } else if self.generation_version < 7 {
            for worker in &mut facility.blueprint.workers {
                worker.local_alert_profile = None;
            }
        }
        if self.generation_version < 8 {
            for installation in &mut facility.blueprint.installations {
                installation.security_alarm_profile = None;
            }
        } else if self.generation_version < 9 {
            for installation in &mut facility.blueprint.installations {
                installation.security_alarm_profile = installation
                    .security_alarm_profile
                    .as_ref()
                    .map(|profile| profile.without_responses());
            }
        }
        if self.generation_version < DATA_TERMINALS_GENERATION_VERSION {
            facility.blueprint.installations.retain(|installation| {
                !installation.capabilities.iter().any(|capability| {
                    matches!(capability, InstallationCapability::DataTerminal { .. })
                })
            });
        }
        for worker in &facility.blueprint.workers {
            let mut actor = Actor::new(worker.actor_position, worker.maximum_integrity)
                .map_err(|error| error.to_string())?
                .with_ai(AiProfile::idle());
            if let Some(affiliation) = &worker.affiliation {
                actor = actor.with_affiliation(affiliation.clone());
            }
            if let Some(profile) = worker.witness_profile {
                actor = actor.with_witness_profile(profile);
            }
            if let Some(profile) = worker.local_alert_profile {
                actor = actor.with_local_alert_profile(profile);
            }
            self.game
                .spawn_actor(actor)
                .map_err(|error| error.to_string())?;
        }
        for material in &facility.materials {
            self.game
                .spawn_ground_item_with_owner(
                    material.position,
                    material.item.clone(),
                    material.quantity,
                    material.owner.clone(),
                )
                .map_err(|error| error.to_string())?;
        }
        self.game
            .register_facility(definition.hub.id.clone(), facility.blueprint)?;
        self.game.drain_events();
        self.sync_facility_presentation();
        self.terminal
            .observe(self.game.map(), self.game.player_visibility());
        Ok(())
    }

    fn sync_facility_presentation(&mut self) {
        let Some(facility) = self.game.active_facility() else {
            return;
        };
        for (id, installation) in facility.installations() {
            let operational = facility.is_operational(id);
            let decor =
                if installation
                    .capabilities()
                    .contains(&InstallationCapability::Storage)
                {
                    crate::test_sector::Decor::Depot
                } else if installation
                    .capabilities()
                    .contains(&InstallationCapability::PowerRelay)
                {
                    if operational {
                        crate::test_sector::Decor::RelayOnline
                    } else {
                        crate::test_sector::Decor::RelayOffline
                    }
                } else if installation
                    .capabilities()
                    .contains(&InstallationCapability::SecuritySensor)
                {
                    if operational {
                        crate::test_sector::Decor::SensorOnline
                    } else {
                        crate::test_sector::Decor::SensorOffline
                    }
                } else if installation.capabilities().iter().any(|capability| {
                    matches!(capability, InstallationCapability::DataTerminal { .. })
                }) {
                    if operational {
                        crate::test_sector::Decor::DataTerminalOnline
                    } else {
                        crate::test_sector::Decor::DataTerminalOffline
                    }
                } else if installation.capabilities().iter().any(|capability| {
                    matches!(capability, InstallationCapability::DoorActuator { .. })
                }) {
                    if operational {
                        crate::test_sector::Decor::ActuatorOnline
                    } else {
                        crate::test_sector::Decor::ActuatorOffline
                    }
                } else {
                    continue;
                };
            self.terminal
                .decor
                .cells
                .insert(installation.position(), decor);
        }
    }

    fn refresh_zone_title(&mut self) {
        if let Some(zone) = self.game.current_zone() {
            self.terminal.title = player_zone_title(&zone.name, zone.depth);
        }
    }

    fn from_seed_legacy(
        seed: u64,
        mut rules: GameRules,
        texts: TextCatalog,
        loot_catalog: LootCatalog,
        expedition_catalog: ExpeditionCatalog,
        regional_world_catalog: RegionalWorldCatalog,
        generation_version: u8,
    ) -> Result<Self, String> {
        rules.items = rules.items.without_kind(ItemKind::Material);
        Self::from_seed_base(
            seed,
            rules,
            texts,
            loot_catalog,
            expedition_catalog,
            regional_world_catalog,
            generation_version,
        )
    }

    fn from_seed_base(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot_catalog: LootCatalog,
        expedition_catalog: ExpeditionCatalog,
        regional_world_catalog: RegionalWorldCatalog,
        generation_version: u8,
    ) -> Result<Self, String> {
        let sector = TestSector::build_for_generation(
            seed,
            generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
            generation_version >= REGIONAL_TRAVEL_GENERATION_VERSION,
        )?;
        let exit = sector.level.exit();
        let mut game = GameState::from_generated(sector.level, seed, rules.clone())
            .map_err(|error| error.to_string())?;
        let mut actor_glyphs = BTreeMap::new();

        for (index, position) in sector.enemies.iter().copied().enumerate() {
            if position == exit {
                continue;
            }
            let integrity = 5 + index as u16 * 2;
            let (attack, ai, glyph, reward) = match index % 3 {
                0 => (
                    if generation_version >= PHYSICAL_PROFILES_GENERATION_VERSION {
                        AttackProfile::melee(DamageType::Kinetic, 3)
                            .with_melee_impact(MeleeImpactProfile::new(12, 0))
                            .expect("kinetic melee prototype accepts an Impact profile")
                    } else {
                        AttackProfile::melee(DamageType::Kinetic, 3)
                    },
                    territorial_ai_for_generation(AiProfile::hunter(10, 0), generation_version, 14),
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
                    territorial_ai_for_generation(
                        AiProfile::skirmisher(9, 0, 3),
                        generation_version,
                        16,
                    ),
                    'r',
                    DefeatReward::persistent(8, 2),
                ),
            };
            let mut enemy = Actor::new(position, integrity)
                .map_err(|error| error.to_string())?
                .with_attack(attack)
                .with_ai(ai)
                .with_defeat_reward(reward);
            if generation_version >= ENEMY_ATTRIBUTES_GENERATION_VERSION {
                enemy = enemy.with_primary_attributes(prototype_enemy_attributes(index));
            }
            if generation_version >= PHYSICAL_PROFILES_GENERATION_VERSION {
                let mut body = BodyProfile::new(integrity, 0)
                    .expect("fixed prototype body base must be positive");
                if generation_version >= MELEE_SKILLS_GENERATION_VERSION {
                    body = body
                        .with_displacement_profile(
                            DisplacementProfile::new(70_000, 0)
                                .expect("prototype enemy mass must be positive"),
                        )
                        .with_locomotion_profile(LocomotionProfile::new(true));
                }
                if generation_version >= RANGED_SKILLS_GENERATION_VERSION {
                    body = body.with_suppression_compatibility(true);
                }
                enemy = enemy.with_body_profile(body);
                if generation_version >= RANGED_SKILLS_GENERATION_VERSION {
                    enemy = enemy.with_body_components([BodyComponentProfile::new(
                        "core:locomotion_assembly"
                            .parse()
                            .expect("built-in component ID must remain valid"),
                        "component.locomotion_assembly.name".to_owned(),
                        integrity,
                        0,
                        ComponentFailureEffect::DisableMovement,
                    )
                    .expect("prototype component durability must be positive")]);
                }
            }
            if generation_version >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
                enemy = enemy.with_electronic_system(
                    project_rl::electronic_warfare::ElectronicSystemProfile::new(
                        45_u16.saturating_add(index as u16 * 5),
                        30,
                        45,
                        3,
                        30,
                    )
                    .expect("prototype electronic profile is valid"),
                );
            }
            if generation_version >= PLAYER_RELATIONS_GENERATION_VERSION {
                enemy = enemy.with_player_relation(PlayerRelation::Hostile);
            }
            let id = game.spawn_actor(enemy).map_err(|error| error.to_string())?;
            actor_glyphs.insert(id, glyph);
        }

        if (DRONE_SKILLS_GENERATION_VERSION..INTRINSIC_SKILL_MANIFESTATIONS_GENERATION_VERSION)
            .contains(&generation_version)
        {
            let player_position = game
                .player_position()
                .ok_or("Position du joueur absente à l'initialisation du drone.")?;
            let drone_position = player_position
                .cardinal_neighbors()
                .into_iter()
                .find(|position| {
                    game.map().is_walkable(*position)
                        && game.actors().entity_at(*position).is_none()
                })
                .ok_or("Aucune case physique libre pour le drone de test.")?;
            let profile = DroneProfile::new(
                "core:utility_drone"
                    .parse()
                    .expect("built-in drone visual ID must remain valid"),
                6,
                50,
                40,
                1,
                3,
                6,
                1,
                1,
                DroneCapabilities {
                    manipulator_capacity_grams: Some(8_000),
                    decoy_intensity: Some(30),
                    can_interpose: true,
                    autonomous_scout_range: 6,
                },
            )
            .map_err(|error| error.to_string())?;
            let drone_actor = Actor::new(drone_position, 16)
                .map_err(|error| error.to_string())?
                .with_attack(AttackProfile::new(
                    5,
                    DistanceMetric::Euclidean,
                    true,
                    DamageType::Electrical,
                    2,
                    0,
                ))
                .with_body_components([
                    BodyComponentProfile::new(
                        "core:drone_drive".parse().expect("valid component ID"),
                        "component.drone_drive.name".to_owned(),
                        16,
                        3,
                        ComponentFailureEffect::DisableMovement,
                    )
                    .expect("drone drive durability is valid"),
                    BodyComponentProfile::new(
                        "core:drone_sensor".parse().expect("valid component ID"),
                        "component.drone_sensor.name".to_owned(),
                        12,
                        2,
                        ComponentFailureEffect::ReducePerception(3),
                    )
                    .expect("drone sensor durability is valid"),
                ]);
            let drone = game
                .spawn_player_drone(drone_actor, profile, 30, 30)
                .map_err(|error| error.to_string())?;
            actor_glyphs.insert(drone, 'u');
        }

        let player_position = game.player_position();
        let loot_positions = sector
            .loot
            .iter()
            .copied()
            .filter(|position| {
                Some(*position) != player_position
                    && *position != exit
                    && game.actors().entity_at(*position).is_none()
            })
            .collect::<Vec<_>>();
        let mut loot = game
            .rules()
            .weapons
            .iter()
            .find(|(id, _)| id.namespace().as_str() != "core")
            .map(|(id, _)| (id.clone(), 1))
            .into_iter()
            .collect::<Vec<_>>();
        loot.extend(
            game.rules()
                .items
                .iter()
                .find(|(id, _)| id.as_str() == "core:repair_patch")
                .map(|(id, _)| (id.clone(), 1)),
        );
        for ((item, quantity), position) in loot.into_iter().zip(loot_positions) {
            game.spawn_ground_item(position, item, quantity)
                .map_err(|error| error.to_string())?;
        }
        game.drain_events();
        // Discipline availability depends only on the immutable ruleset and
        // enabled system features. Its exhaustive path analysis belongs at
        // run construction, never in the per-frame renderer.
        let skill_availability_cache = game
            .rules()
            .skills
            .disciplines()
            .map(|(discipline, _)| {
                game.discipline_availability(discipline)
                    .map(|availability| (discipline.clone(), availability))
                    .map_err(|error| {
                        format!("Analyse de la discipline {discipline} impossible : {error}")
                    })
            })
            .collect::<Result<BTreeMap<_, _>, _>>()?;
        let terminal = TerminalView::new(sector.decor, game.map(), game.player_visibility());

        Ok(Self {
            game: WorldState::single(game),
            terminal,
            zone_views: BTreeMap::new(),
            zone_decor: BTreeMap::new(),
            facing: Direction::North,
            rules,
            character_classes: ascii_character_class_catalog()?,
            character_class: None,
            character_creation: None,
            texts,
            loot: loot_catalog,
            expeditions: expedition_catalog,
            regional_worlds: regional_world_catalog,
            regional_zones: BTreeMap::new(),
            generation_version: 1,
            seed,
            actor_glyphs,
            selected_target: None,
            attack_aim: None,
            attack_aim_technique: None,
            attack_aim_pointer: None,
            component_selection: None,
            visual_cues: VisualCuePlayer::with_catalog(ascii_visual_cue_catalog()?),
            floating_messages: Vec::new(),
            trace_cells: BTreeMap::new(),
            traces_visible_until: 0.0,
            log: vec![
                "La ville est calme. Les menaces commencent au-delà de ses remparts.".to_owned(),
            ],
            active_weapon_slot: 0,
            inventory_open: false,
            inventory_selection: 0,
            inventory_filter: InventoryFilter::default(),
            inventory_sort: InventorySort::default(),
            inventory_message: String::new(),
            character_open: false,
            character_attribute_selection: 0,
            skills_open: false,
            skill_discipline_selection: 0,
            skill_technique_selection: 0,
            skill_availability_cache,
            skill_message: String::new(),
            technique_menu_open: false,
            technique_menu_selection: 0,
            technique_menu_message: String::new(),
            level_up_notice: None,
            observation_report: Vec::new(),
            report_open: false,
            report_scroll: 0,
            legend_open: false,
            controls: Controls::preset(controls::Layout::Qwerty, KeySemantics::native()),
            movement_repeat: controls::MovementRepeater::default(),
            controls_path: controls::config_path(),
            graphics: GraphicsState::default(),
            menu: MenuScreen::Hidden,
            options_return: MenuScreen::Pause,
            menu_selection: 0,
            menu_message: String::new(),
            menu_focus: MenuFocus::default(),
            cursor_icon: miniquad::CursorIcon::Default,
            quit_requested: false,
            options_selection: 0,
            options_scroll: 0,
            rebinding: false,
            options_message: String::new(),
            history: Vec::new(),
            suspension_path: controls::config_path().with_file_name("city-test-run.json"),
            session_lock: None,
        })
    }

    fn glyph_at(&self, position: GridPos) -> Option<(char, Color, Option<Color>, Option<Color>)> {
        let visibility = self.game.player_visibility();
        if !visibility.is_explored(position) {
            return None;
        }

        if let Some(sample) =
            self.visual_cues
                .sample_world(position, visibility.is_visible(position), get_time())
        {
            return Some((
                sample.symbol,
                sample.color,
                sample.accent_color,
                sample.highlight_color,
            ));
        }

        if visibility.is_visible(position) {
            if self.game.player_position() == Some(position) {
                return Some(('@', Color::from_rgba(99, 242, 210, 255), None, None));
            }
            if self.security_alarm_remaining_at(position).is_some() {
                return Some(('s', Color::from_rgba(255, 175, 83, 255), None, None));
            }
            if let Some(entity) = self.game.actors().entity_at(position) {
                let role = self.game.active_worker_role(entity);
                let drone = self
                    .game
                    .actors()
                    .get(entity)
                    .is_some_and(|actor| actor.drone().is_some());
                let destructible = self
                    .game
                    .actors()
                    .get(entity)
                    .is_some_and(|actor| actor.destruction_effect().is_some());
                let glyph = role.map_or_else(
                    || self.hostile_glyph(entity),
                    |role| match role {
                        WorkerRole::Retriever => 'c',
                        WorkerRole::Technician => 'm',
                    },
                );
                let burning = self.game.actors().get(entity).is_some_and(|actor| {
                    actor
                        .statuses()
                        .any(|status| status.definition.as_str() == "core:burning")
                });
                let has_status = self
                    .game
                    .actors()
                    .get(entity)
                    .is_some_and(|actor| actor.statuses().next().is_some());
                let color = if drone {
                    Color::from_rgba(105, 205, 238, 255)
                } else if role.is_some() {
                    Color::from_rgba(112, 207, 190, 255)
                } else if destructible {
                    Color::from_rgba(241, 177, 72, 255)
                } else if burning {
                    Color::from_rgba(255, 137, 48, 255)
                } else if has_status {
                    Color::from_rgba(143, 221, 107, 255)
                } else {
                    Color::from_rgba(244, 105, 90, 255)
                };
                return Some((glyph, color, None, None));
            }
            if self.game.exit() == Some(position) {
                return Some(('>', Color::from_rgba(255, 211, 92, 255), None, None));
            }
            if let Some(device) = self
                .game
                .explosive_devices()
                .at(position)
                .find(|device| device.is_identified())
            {
                let color = if device.is_neutralized() {
                    Color::from_rgba(135, 162, 167, 255)
                } else if device.triggered_on().is_some() {
                    Color::from_rgba(255, 113, 91, 255)
                } else {
                    match device.activation() {
                        ExplosiveActivation::Timed { .. } => Color::from_rgba(255, 185, 82, 255),
                        ExplosiveActivation::Proximity { .. } => {
                            Color::from_rgba(244, 105, 90, 255)
                        }
                        ExplosiveActivation::Remote { .. } => Color::from_rgba(100, 221, 201, 255),
                    }
                };
                return Some(('¤', color, None, None));
            }
            if self.game.sound_emitters().at(position).next().is_some() {
                return Some(('♪', Color::from_rgba(143, 211, 232, 255), None, None));
            }
            if let Some(ground_item) = self.game.ground_items().item_at(position)
                && let Some(stack) = self.game.ground_items().get(ground_item)
            {
                if self.game.rules().weapons.get(stack.item()).is_some() {
                    return Some((')', Color::from_rgba(255, 211, 92, 255), None, None));
                }
                if self
                    .game
                    .rules()
                    .items
                    .get(stack.item())
                    .is_some_and(|item| item.kind() == ItemKind::Material)
                {
                    return Some(('=', Color::from_rgba(118, 202, 207, 255), None, None));
                }
                return Some(('!', Color::from_rgba(111, 224, 143, 255), None, None));
            }
            if let Some(effect) = self.game.ground_effects().at(position).next() {
                let palette = self.visual_cues.palette_for(effect.definition());
                return Some((
                    '^',
                    palette.color,
                    palette.accent_color,
                    palette.highlight_color,
                ));
            }
        }

        if get_time() <= self.traces_visible_until
            && visibility.is_visible(position)
            && let Some(direction) = self.trace_cells.get(&position)
        {
            let glyph = match direction {
                Direction::North => '↑',
                Direction::East => '→',
                Direction::South => '↓',
                Direction::West => '←',
            };
            return Some((glyph, Color::from_rgba(196, 158, 98, 255), None, None));
        }

        None
    }

    fn hostile_glyph(&self, entity: EntityId) -> char {
        self.actor_glyphs.get(&entity).copied().unwrap_or_else(|| {
            if self
                .game
                .electronic_warfare_state()
                .beacon(entity)
                .is_some()
            {
                return 'b';
            }
            if self
                .game
                .actors()
                .get(entity)
                .is_some_and(|actor| actor.drone().is_some())
            {
                return 'u';
            }
            if self
                .game
                .actors()
                .get(entity)
                .is_some_and(|actor| actor.destruction_effect().is_some())
            {
                return 'o';
            }
            match self
                .game
                .actors()
                .get(entity)
                .and_then(Actor::ai)
                .map(|ai| ai.behavior)
            {
                Some(project_rl::ai::AiBehavior::Sentry) => 't',
                Some(project_rl::ai::AiBehavior::Skirmisher) => 'r',
                _ => 'd',
            }
        })
    }

    fn burning_status_icon_at(&self, position: GridPos) -> Option<TerminalStatusIcon> {
        self.game
            .actors()
            .entity_at(position)
            .and_then(|entity| self.game.actors().get(entity))
            .is_some_and(|actor| {
                actor
                    .statuses()
                    .any(|status| status.definition.as_str() == "core:burning")
            })
            .then_some(TerminalStatusIcon::Burning)
    }

    fn local_alert_remaining_at(&self, position: GridPos) -> Option<u64> {
        if !self.game.player_visibility().is_visible(position) {
            return None;
        }
        let entity = self.game.actors().entity_at(position)?;
        self.game
            .actors()
            .get(entity)?
            .local_alert()
            .filter(|alert| alert.is_active(self.game.turn()))
            .map(|alert| alert.remaining_turns(self.game.turn()))
    }

    fn security_alarm_remaining_at(&self, position: GridPos) -> Option<u64> {
        if !self.game.player_visibility().is_visible(position) {
            return None;
        }
        self.game
            .active_facility()?
            .security_alarm_at(position, self.game.turn())
            .map(|(_, alarm)| alarm.remaining_turns(self.game.turn()))
    }

    fn visible_alert_at(&self, position: GridPos) -> Option<(TerminalAlertKind, u64)> {
        self.security_alarm_remaining_at(position)
            .map(|remaining| (TerminalAlertKind::SecuritySystem, remaining))
            .or_else(|| {
                self.local_alert_remaining_at(position)
                    .map(|remaining| (TerminalAlertKind::LocalWitness, remaining))
            })
    }

    fn visible_local_alert_summary(&self) -> Option<(usize, u64)> {
        let mut count = 0;
        let mut longest_remaining = 0;
        self.game
            .actors()
            .iter()
            .filter(|(_, actor)| self.game.player_visibility().is_visible(actor.position()))
            .filter_map(|(_, actor)| actor.local_alert())
            .filter(|alert| alert.is_active(self.game.turn()))
            .for_each(|alert| {
                count += 1;
                longest_remaining = longest_remaining.max(alert.remaining_turns(self.game.turn()));
            });
        (count > 0).then_some((count, longest_remaining))
    }

    fn visible_security_alarm_summary(&self) -> Option<(usize, u64)> {
        let mut count = 0;
        let mut longest_remaining = 0;
        if let Some(facility) = self.game.active_facility() {
            facility
                .active_security_alarms(self.game.turn())
                .filter(|(source, _)| {
                    self.game.player_visibility().is_visible(source.position())
                        || facility
                            .active_security_door_lockdowns(self.game.turn())
                            .any(|(door, lockdown)| {
                                lockdown.installation == *source.id()
                                    && self.game.player_visibility().is_visible(door)
                            })
                })
                .for_each(|(_, alarm)| {
                    count += 1;
                    longest_remaining =
                        longest_remaining.max(alarm.remaining_turns(self.game.turn()));
                });
        }
        (count > 0).then_some((count, longest_remaining))
    }

    fn visible_alert_summary(&self) -> Option<(usize, usize, u64)> {
        let (local, local_remaining) = self.visible_local_alert_summary().unwrap_or((0, 0));
        let (security, security_remaining) =
            self.visible_security_alarm_summary().unwrap_or((0, 0));
        (local + security > 0).then_some((local, security, local_remaining.max(security_remaining)))
    }

    fn navigation_signal_summary(&self) -> Option<String> {
        self.vertical_navigation_signal_summary()
            .or_else(|| crate::terminal_view::detected_navigation_signal_summary(&self.game))
            .or_else(|| self.regional_depth_route_signal_summary())
    }

    fn vertical_navigation_signal_summary(&self) -> Option<String> {
        if self.generation_version < REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION {
            return None;
        }
        let current_zone = &self.game.current_zone()?.id;
        let coordinate = *self.regional_zones.get(current_zone)?;
        let world_id: ContentId = "core:simulation_overworld".parse().ok()?;
        let world = self.regional_worlds.get(&world_id)?;
        let neighbors = world.vertical_neighbors(coordinate);
        let (direction, _) = neighbors
            .iter()
            .find(|(direction, _)| *direction == RegionVerticalDirection::Down)
            .or_else(|| neighbors.first())?;
        let target =
            project_rl::world::generation::vertical_passage(world.local_map_size(), *direction);
        // Legacy generations know the current atlas catalogue too, but only an
        // actually declared runtime passage may advertise a route to the user.
        self.game.passage(target)?;
        let observer = self.game.player_position()?;
        let label = match direction {
            RegionVerticalDirection::Down => "ACCÈS INFÉRIEUR",
            RegionVerticalDirection::Up => "RETOUR SURFACE",
        };
        Some(crate::terminal_view::directional_signal_summary(
            label,
            observer,
            target,
            grid_distance(observer, target),
            1,
        ))
    }

    fn regional_depth_route_signal_summary(&self) -> Option<String> {
        if self.generation_version < REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION {
            return None;
        }
        let current_zone = &self.game.current_zone()?.id;
        let coordinate = *self.regional_zones.get(current_zone)?;
        let world_id: ContentId = "core:simulation_overworld".parse().ok()?;
        let world = self.regional_worlds.get(&world_id)?;
        let destination = world
            .vertical_links()
            .iter()
            .map(|link| link.upper())
            .filter(|destination| destination.depth == coordinate.depth)
            .min_by_key(|destination| {
                (
                    coordinate
                        .x
                        .abs_diff(destination.x)
                        .saturating_add(coordinate.y.abs_diff(destination.y)),
                    *destination,
                )
            })?;
        let direction = regional_route_direction(coordinate, destination)?;
        let target = if coordinate == RegionCoord::new(0, 0, 0) {
            Self::hub_regional_passage(direction)?
        } else {
            project_rl::world::generation::cardinal_passage(
                world.local_map_size(),
                direction_from_region(direction),
            )
        };
        // The atlas can describe a route that an older save never materialized.
        // Only advertise a passage that really exists in the active runtime.
        self.game.passage(target)?;
        let observer = self.game.player_position()?;
        Some(crate::terminal_view::directional_signal_summary(
            "ROUTE VERS LES PROFONDEURS",
            observer,
            target,
            grid_distance(observer, target),
            1,
        ))
    }

    fn materialize_deferred_destination_for(
        &mut self,
        command: &GameCommand,
    ) -> Result<(), String> {
        if self.generation_version < LAZY_ZONE_GENERATION_VERSION {
            return Ok(());
        }
        let GameCommand::Interact { target } = command else {
            return Ok(());
        };
        if !self.game.can_materialize_passage(*target) {
            return Ok(());
        }
        let destination = self
            .game
            .unmaterialized_passage_destination(*target)
            .cloned()
            .ok_or("Destination régionale différée absente.")?;
        if let Some(coordinate) = self.regional_zones.get(&destination).copied() {
            return self.materialize_regional_destination(*target, destination, coordinate);
        }
        let definition = self
            .expeditions
            .iter()
            .find_map(|(_, definition)| {
                (definition.destination.zone.id == destination).then_some(definition.clone())
            })
            .ok_or_else(|| format!("Aucun fournisseur pour la zone '{destination}'."))?;
        let generated = crate::test_expedition::generate_destination(
            &self.rules,
            self.seed,
            Some(&self.loot),
            &definition,
            crate::test_expedition::ExpeditionGenerationFeatures {
                defined_population: self.generation_version
                    >= DEFINED_POPULATION_GENERATION_VERSION,
                expanded_world: self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
                pursuit_limits: self.generation_version >= PURSUIT_LEASH_GENERATION_VERSION,
                pursuit_lifecycle: self.generation_version >= PURSUIT_LIFECYCLE_GENERATION_VERSION,
                primary_attributes: self.generation_version >= ENEMY_ATTRIBUTES_GENERATION_VERSION,
                physical_profiles: self.generation_version >= PHYSICAL_PROFILES_GENERATION_VERSION,
                electronic_systems: self.generation_version
                    >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION,
                preparation_disruption: self.generation_version
                    >= PREPARATION_DISRUPTION_GENERATION_VERSION,
                player_relations: self.generation_version >= PLAYER_RELATIONS_GENERATION_VERSION,
            },
        )?;
        self.game
            .materialize_passage_destination(*target, generated.blueprint)?;
        self.zone_decor.insert(destination, generated.decor);
        Ok(())
    }

    fn materialize_regional_destination(
        &mut self,
        target: GridPos,
        destination: ContentId,
        coordinate: RegionCoord,
    ) -> Result<(), String> {
        let world_id: ContentId = "core:simulation_overworld"
            .parse()
            .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
        let world = self
            .regional_worlds
            .get(&world_id)
            .ok_or("Atlas régional de départ absent.")?
            .clone();
        let descriptor = world
            .region(self.seed, coordinate)
            .ok_or("Destination située hors de l'atlas régional.")?;
        let info = self
            .game
            .zone_info(&destination)
            .cloned()
            .ok_or("Métadonnées de la destination régionale absentes.")?;
        let entrance = self
            .game
            .passage(target)
            .and_then(|link| link.arrival)
            .ok_or("Arrivée régionale absente du passage.")?;
        let generated = crate::test_regional::generate(
            &world,
            &descriptor,
            info,
            entrance,
            Some(&self.loot),
            crate::test_regional::RegionalGenerationFeatures {
                vertical_travel: self.generation_version
                    >= REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION,
                population: self.generation_version >= REGIONAL_POPULATION_GENERATION_VERSION,
                encounters: self.generation_version >= REGIONAL_ENCOUNTER_GENERATION_VERSION,
                pursuit_lifecycle: self.generation_version >= PURSUIT_LIFECYCLE_GENERATION_VERSION,
                primary_attributes: self.generation_version >= ENEMY_ATTRIBUTES_GENERATION_VERSION,
                physical_profiles: self.generation_version >= PHYSICAL_PROFILES_GENERATION_VERSION,
                loot: self.generation_version >= REGIONAL_LOOT_GENERATION_VERSION,
                landmarks: self.generation_version >= REGIONAL_LANDMARK_GENERATION_VERSION,
                sites: self.generation_version >= REGIONAL_SITE_GENERATION_VERSION,
                site_interactions: self.generation_version
                    >= REGIONAL_SITE_INTERACTION_GENERATION_VERSION,
                site_security: self.generation_version >= REGIONAL_SITE_SECURITY_GENERATION_VERSION,
                site_terminals: self.generation_version
                    >= REGIONAL_SITE_TERMINALS_GENERATION_VERSION,
                reinforcement_investigation: self.generation_version
                    >= INVESTIGATING_REINFORCEMENTS_GENERATION_VERSION,
                site_navigation_signals: self.generation_version
                    >= SITE_NAVIGATION_SIGNALS_GENERATION_VERSION,
                threat_renewal: self.generation_version >= THREAT_RENEWAL_GENERATION_VERSION,
                destructibles: self.generation_version >= REGIONAL_DESTRUCTIBLES_GENERATION_VERSION,
                electronic_systems: self.generation_version
                    >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION,
                player_relations: self.generation_version >= PLAYER_RELATIONS_GENERATION_VERSION,
            },
        )?;
        let mut connections = Vec::new();
        let mut mappings = Vec::new();
        for (direction, passage) in generated.passages {
            let Some(neighbor) = coordinate.step(direction) else {
                continue;
            };
            if !world.bounds().contains(neighbor) {
                continue;
            }
            let neighbor_info = self.regional_zone_info(&world, neighbor)?;
            let arrival = if neighbor == RegionCoord::new(0, 0, 0) {
                Self::hub_regional_passage(direction.opposite())
                    .ok_or("Passage régional de la ville absent.")?
            } else {
                project_rl::world::generation::cardinal_passage(
                    world.local_map_size(),
                    direction_from_region(direction.opposite()),
                )
            };
            connections.push(ZoneConnectionBlueprint {
                at: passage,
                destination: neighbor_info.clone(),
                arrival,
            });
            mappings.push((neighbor_info.id, neighbor));
        }
        for (direction, passage) in &generated.vertical_passages {
            let Some(neighbor) = world.vertical_neighbor(coordinate, *direction) else {
                continue;
            };
            let neighbor_info = self.regional_zone_info(&world, neighbor)?;
            let arrival = project_rl::world::generation::vertical_passage(
                world.local_map_size(),
                direction.opposite(),
            );
            connections.push(ZoneConnectionBlueprint {
                at: *passage,
                destination: neighbor_info.clone(),
                arrival,
            });
            mappings.push((neighbor_info.id, neighbor));
        }
        let mut next_regions = self.regional_zones.clone();
        for (zone, coordinate) in &mappings {
            Self::insert_regional_zone(&mut next_regions, zone.clone(), *coordinate)?;
        }
        self.game
            .materialize_passage_destination_with_connections_and_facility(
                target,
                generated.blueprint,
                &connections,
                generated.facility,
            )?;
        self.regional_zones = next_regions;
        self.zone_decor.insert(destination, generated.decor);
        Ok(())
    }

    fn execute_command(&mut self, command: GameCommand) -> CommandOutcome {
        if self.materialize_deferred_destination_for(&command).is_err() {
            self.push_log("Le passage ne mène nulle part pour le moment.".to_owned());
        }
        let recorded = RecordedCommand::record(&command);
        let previous_zone = self.game.current_zone().map(|zone| zone.id.clone());
        let outcome = self.game.process_player_command(command);
        if !matches!(outcome, CommandOutcome::Rejected(_)) {
            self.history.push(recorded);
            let current_zone = self.game.current_zone().map(|zone| zone.id.clone());
            if previous_zone != current_zone
                && let (Some(previous), Some(current)) = (previous_zone, current_zone)
            {
                let next = self.zone_views.remove(&current).unwrap_or_else(|| {
                    TerminalView::new(
                        self.zone_decor.remove(&current).unwrap_or_default(),
                        self.game.map(),
                        self.game.player_visibility(),
                    )
                });
                let old = std::mem::replace(&mut self.terminal, next);
                self.zone_views.insert(previous, old);
                self.refresh_zone_title();
                self.selected_target = None;
                self.visual_cues.clear_world();
                self.trace_cells.clear();
                self.observation_report.clear();
                self.report_open = false;
            }
            self.sync_facility_presentation();
            self.terminal
                .observe(self.game.map(), self.game.player_visibility());
        }
        outcome
    }

    fn capture_events(&mut self) {
        #[cfg(test)]
        self.capture_events_with_navigation(Some(0.0), true);
        #[cfg(not(test))]
        self.capture_events_with_navigation(None, true);
    }

    fn capture_events_at(&mut self, replay_time: Option<f64>) {
        self.capture_events_with_navigation(replay_time, false);
    }

    fn capture_events_with_navigation(
        &mut self,
        replay_time: Option<f64>,
        open_level_up_screen: bool,
    ) {
        let mut player_attack_confirmation = None;
        let mut level_up_notice: Option<LevelUpNotice> = None;
        let visual_time = replay_time.unwrap_or_else(get_time);
        for event in self.game.drain_events() {
            match event {
                GameEvent::ZoneChanged { .. } => {
                    self.floating_messages.clear();
                    if let Some(zone) = self.game.current_zone() {
                        self.push_log(format!("Vous entrez dans {}.", player_zone_title(&zone.name, zone.depth)));
                    }
                }
                GameEvent::Facility(event) => match event {
                    FacilityEvent::WorkerMoved { .. } => {}
                    FacilityEvent::DoorOpened { .. } => {
                        self.push_log("Un agent de maintenance ouvre une porte.".to_owned())
                    }
                    FacilityEvent::MaterialCollected { item, quantity, .. } => {
                        self.push_log(format!(
                            "Récupérateur : {} récupéré x{quantity}.",
                            self.item_name(&item)
                        ))
                    }
                    FacilityEvent::MaterialDelivered { item, quantity, .. } => self.push_log(
                        format!("Dépôt : {} livré x{quantity}.", self.item_name(&item)),
                    ),
                    FacilityEvent::PlayerMaterialDeposited { item, quantity, .. } => self.push_log(
                        format!("Dépôt : vous livrez {} x{quantity}.", self.item_name(&item)),
                    ),
                    FacilityEvent::DataTerminalAccessed {
                        record,
                        first_access,
                        ..
                    } => {
                        let text = self
                            .texts
                            .resolve(DISPLAY_LOCALE, record.as_str())
                            .map(str::to_owned)
                            .unwrap_or_else(|| format!("Archive {record}"));
                        self.push_log(format!(
                            "{} · {text}{}",
                            if first_access {
                                "ARCHIVE DÉCOUVERTE"
                            } else {
                                "ARCHIVE CONSULTÉE"
                            },
                            if first_access {
                                format!(" · {} : dossier", self.controls.label(Action::Report))
                            } else {
                                String::new()
                            },
                        ));
                    }
                    FacilityEvent::RepairAssigned { .. } => {
                        self.push_log("Technicien : pièce réservée au dépôt.".to_owned())
                    }
                    FacilityEvent::RepairStarted { turns, .. } => self.push_log(format!(
                        "Technicien : remise en service commencée ({turns} tours)."
                    )),
                    FacilityEvent::InstallationRepaired { .. } => self.push_log(
                        "Relais réparé : porte et capteur de sécurité rétablis.".to_owned(),
                    ),
                    FacilityEvent::SecurityAlarmRaised {
                        at, duration_turns, ..
                    } => {
                        if self.game.player_visibility().is_visible(at) {
                            self.push_floating_message(
                                "ALARME !",
                                at,
                                FloatingMessageTone::Alert,
                                visual_time,
                            );
                        }
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:security_alarm"), at),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "ALARME DE SÉCURITÉ VISIBLE · CAPTEUR · {duration_turns} TOURS"
                        ));
                    }
                    FacilityEvent::ReinforcementsRequested {
                        source,
                        delay_turns,
                        ..
                    } => {
                        if self.game.player_visibility().is_visible(source) {
                            self.push_floating_message(
                                "RENFORTS EN ROUTE",
                                source,
                                FloatingMessageTone::Alert,
                                visual_time,
                            );
                        }
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:security_alarm"), source),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "ALARME · RENFORTS DEMANDÉS · ARRIVÉE ESTIMÉE DANS {delay_turns} TOURS"
                        ));
                    }
                    FacilityEvent::InvestigatingReinforcementsRequested {
                        source,
                        delay_turns,
                        ..
                    } => {
                        if self.game.player_visibility().is_visible(source) {
                            self.push_floating_message(
                                "ENQUÊTE EN ROUTE",
                                source,
                                FloatingMessageTone::Alert,
                                visual_time,
                            );
                        }
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:security_alarm"), source),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "ALARME · RENFORTS EN ROUTE VERS LE LIEU SIGNALÉ · {delay_turns} TOURS"
                        ));
                    }
                    FacilityEvent::ReinforcementsUnavailable { reason, .. } => self.push_log(
                        match reason {
                            ReinforcementRequestFailure::SourceInactive => {
                                "ALARME · RENFORTS IMPOSSIBLES · SOURCE NEUTRALISÉE"
                            }
                            ReinforcementRequestFailure::QuotaExhausted => {
                                "ALARME · RENFORTS IMPOSSIBLES · QUOTA ÉPUISÉ"
                            }
                        }
                        .to_owned(),
                    ),
                    FacilityEvent::DoorLockdownStarted {
                        door,
                        duration_turns,
                        ..
                    } => {
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:door_lockdown"), door),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log(format!(
                            "VERROUILLAGE DE SÉCURITÉ ACTIF · {duration_turns} TOURS"
                        ));
                    }
                    FacilityEvent::DoorLockdownPrevented { reason, .. } => self.push_log(
                        match reason {
                            DoorLockdownPrevention::ActuatorUnavailable => {
                                "Verrouillage de sécurité impossible : commande indisponible."
                            }
                            DoorLockdownPrevention::DoorAlreadyLocked => {
                                "Verrouillage de sécurité déjà actif sur cet accès."
                            }
                            DoorLockdownPrevention::DoorObstructed => {
                                "Verrouillage de sécurité suspendu : accès encombré."
                            }
                            DoorLockdownPrevention::NoSafeEgress => {
                                "Verrouillage de sécurité suspendu : aucune issue sûre."
                            }
                        }
                        .to_owned(),
                    ),
                    FacilityEvent::DoorLockdownEnded { door, .. } => {
                        self.visual_cues.play(
                            VisualCue::point(visual_cue_id("core:door_lockdown"), door),
                            replay_time.unwrap_or_else(get_time),
                        );
                        self.push_log("Verrouillage de sécurité levé.".to_owned())
                    }
                    FacilityEvent::MaterialSpilled { item, quantity, .. } => self.push_log(
                        format!("{} x{quantity} abandonné au sol.", self.item_name(&item)),
                    ),
                    FacilityEvent::WorkInterrupted { .. } => {
                        self.push_log("Travail de maintenance interrompu.".to_owned())
                    }
                    FacilityEvent::SimulationFault(error) => {
                        self.push_log(format!("Maintenance suspendue : {error}"))
                    }
                },
                GameEvent::TerrainInteracted { terrain, .. } => {
                    self.push_log(
                        match terrain {
                            Terrain::Door(project_rl::world::DoorState::Open) => "Porte ouverte.",
                            Terrain::Door(_) => "Porte fermée.",
                            Terrain::ControlPanel { .. } => {
                                "Console utilisée : accès déverrouillé."
                            }
                            _ => "Interaction effectuée.",
                        }
                        .to_owned(),
                    );
                }
                GameEvent::EntityMoved { entity, to, .. } if entity == self.game.player_id() => {
                    if let Some(ground_item) = self.game.ground_items().item_at(to)
                        && let Some(stack) = self.game.ground_items().get(ground_item)
                    {
                        self.push_log(format!(
                            "{} DÉTECTÉ{} — {} : RAMASSER",
                            display_content_name(stack.item()),
                            stack.owner().map_or("", |owner| {
                                if self.game.player_may_take_property_of(owner) {
                                    " · MATÉRIEL ATTRIBUÉ · PRISE AUTORISÉE"
                                } else {
                                    " · MATÉRIEL ATTRIBUÉ · PRISE SIGNALÉE SI OBSERVÉE"
                                }
                            }),
                            self.controls.label(Action::PickUp)
                        ));
                    }
                }
                GameEvent::MovementTimeCommitted { entity, time_units }
                    if entity == self.game.player_id() =>
                {
                    self.push_log(format!(
                        "Locomotion entravée · ce déplacement consomme {time_units} UT."
                    ));
                }
                GameEvent::ForcedMovementResolved {
                    from, to, outcome, ..
                } => {
                    if !self.game.player_visibility().is_visible(from)
                        && !self.game.player_visibility().is_visible(to)
                    {
                        continue;
                    }
                    let (floating, log) = match outcome {
                        ForcedMovementOutcome::Moved => ("REPOUSSÉ", "Cible repoussée."),
                        ForcedMovementOutcome::Resisted => {
                            ("RÉSISTE", "La cible résiste à la poussée.")
                        }
                        ForcedMovementOutcome::Blocked => {
                            ("BLOQUÉ", "La destination de la poussée est bloquée.")
                        }
                        ForcedMovementOutcome::Fixed | ForcedMovementOutcome::Incompatible => {
                            ("INAMOVIBLE", "Cette cible ne peut pas être déplacée.")
                        }
                    };
                    self.push_floating_message(
                        floating,
                        to,
                        FloatingMessageTone::Information,
                        visual_time,
                    );
                    self.push_log(log.to_owned());
                }
                GameEvent::AttackPerformed {
                    attacker,
                    origin,
                    target_at,
                    weapon,
                    affected_cells,
                    ..
                } => {
                    let weapon_name = weapon
                        .as_ref()
                        .map(|id| self.item_name(id))
                        .unwrap_or_else(|| "Attaque".to_owned());
                    let visibility = self.game.player_visibility();
                    let id = weapon.unwrap_or_else(|| visual_cue_id("core:generic_attack"));
                    let cue = if affected_cells.len() > 1 {
                        VisualCue::world(
                            id,
                            origin,
                            affected_cells
                                .into_iter()
                                .filter(|cell| visibility.is_visible(cell.position))
                                .map(|cell| VisualCueCell::new(cell.position, cell.step)),
                        )
                    } else if visibility.is_visible(origin) && visibility.is_visible(target_at) {
                        VisualCue::line(id, origin, target_at)
                    } else {
                        continue;
                    };
                    if let Ok(cue) = cue {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    if attacker == self.game.player_id() {
                        player_attack_confirmation =
                            Some(format!("ATTAQUE CONFIRMÉE · {weapon_name}"));
                    }
                }
                GameEvent::AmmunitionSpent {
                    entity,
                    weapon,
                    amount,
                    remaining,
                } if entity == self.game.player_id() => {
                    self.push_log(format!(
                        "{} · {amount} projectile(s) · {remaining} restant(s).",
                        self.item_name(&weapon)
                    ));
                }
                GameEvent::AttackHitResolved {
                    attacker,
                    target,
                    at,
                    hit,
                    ..
                } => {
                    if !hit
                        && (target == self.game.player_id()
                            || self.game.player_visibility().is_visible(at))
                    {
                        self.push_floating_message(
                            if target == self.game.player_id() {
                                "ESQUIVÉ"
                            } else {
                                "RATÉ"
                            },
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    if attacker == self.game.player_id() {
                        let details = player_attack_confirmation
                            .as_deref()
                            .and_then(|message| message.strip_prefix("ATTAQUE CONFIRMÉE · "))
                            .unwrap_or("CIBLE");
                        player_attack_confirmation = Some(format!(
                            "{} · {details}",
                            if hit {
                                "IMPACT CONFIRMÉ"
                            } else {
                                "ATTAQUE MANQUÉE"
                            }
                        ));
                    } else if target == self.game.player_id() && !hit {
                        self.push_log("ATTAQUE ENNEMIE MANQUÉE".to_owned());
                    } else if self.game.player_visibility().is_visible(at) && !hit {
                        self.push_log("ATTAQUE MANQUÉE".to_owned());
                    }
                }
                GameEvent::GroundEffectCreated { effect, at, .. }
                | GameEvent::GroundEffectTriggered { effect, at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.visual_cues.play(
                            VisualCue::point(effect, at),
                            replay_time.unwrap_or_else(get_time),
                        );
                    }
                }
                GameEvent::ExplosiveDeployed {
                    entity,
                    material,
                    at,
                    ..
                } => {
                    if entity == self.game.player_id()
                        || self.game.player_visibility().is_visible(at)
                    {
                        self.push_floating_message(
                            "CHARGE POSÉE",
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                        self.push_log(format!("{} déployé.", self.item_name(&material)));
                    }
                }
                GameEvent::ExplosivePlacementResolved {
                    aimed_at,
                    placed_at,
                    chance,
                    roll,
                    ..
                } => {
                    if placed_at != aimed_at {
                        if self.game.player_visibility().is_visible(placed_at) {
                            self.push_floating_message(
                                "DÉVIATION",
                                placed_at,
                                FloatingMessageTone::Alert,
                                visual_time,
                            );
                        }
                        self.push_log(format!(
                            "Jet imprécis · {roll} > {chance} · impact dévié en {},{}.",
                            placed_at.x, placed_at.y
                        ));
                    } else {
                        self.push_log(format!(
                            "Placement exact · {roll} ≤ {chance} · impact en {},{}.",
                            placed_at.x, placed_at.y
                        ));
                    }
                }
                GameEvent::ExplosiveTriggered { at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "DÉTONATION !",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log("CHARGE DÉCLENCHÉE".to_owned());
                }
                GameEvent::ExplosivePayloadResolved {
                    at,
                    stage,
                    cells,
                    final_stage,
                    ..
                } => {
                    let visible = cells
                        .into_iter()
                        .filter(|cell| self.game.player_visibility().is_visible(cell.position))
                        .map(|cell| VisualCueCell::new(cell.position, cell.step))
                        .collect::<Vec<_>>();
                    if let Ok(cue) =
                        VisualCue::world(visual_cue_id("core:radial_damage"), at, visible)
                    {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    if stage > 1 || !final_stage {
                        self.push_log(format!(
                            "Explosion · étape {stage}{}.",
                            if final_stage {
                                " · séquence terminée"
                            } else {
                                ""
                            }
                        ));
                    }
                }
                GameEvent::TerrainBreached { cells, .. } => {
                    if let Some(at) = cells
                        .iter()
                        .copied()
                        .find(|cell| self.game.player_visibility().is_visible(*cell))
                    {
                        self.push_floating_message(
                            "BRÈCHE",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log(format!("Brèche structurelle · {} paroi(s).", cells.len()));
                }
                GameEvent::ExplosiveNeutralized { entity, at, .. } => {
                    if entity == self.game.player_id()
                        || self.game.player_visibility().is_visible(at)
                    {
                        self.push_floating_message(
                            "DÉSAMORCÉE",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log("Charge neutralisée.".to_owned());
                    }
                }
                GameEvent::ExplosiveRecovered {
                    entity, material, ..
                } => {
                    if entity == self.game.player_id() {
                        if let Some(at) = self.game.player_position() {
                            self.push_floating_message(
                                "CHARGE RÉCUPÉRÉE",
                                at,
                                FloatingMessageTone::Information,
                                visual_time,
                            );
                        }
                        self.push_log(format!("{} récupéré.", self.item_name(&material)));
                    }
                }
                GameEvent::ExplosivesProgrammed { entity, devices } => {
                    if entity == self.game.player_id() {
                        if let Some(at) = self.game.player_position() {
                            self.push_floating_message(
                                "SÉQUENCE PROGRAMMÉE",
                                at,
                                FloatingMessageTone::Status,
                                visual_time,
                            );
                        }
                        self.push_log(format!(
                            "Détonation séquencée · {} charge(s) programmée(s).",
                            devices.len()
                        ));
                    }
                }
                GameEvent::ExplosiveCamouflaged {
                    entity,
                    at,
                    optical_difficulty_bonus,
                    ..
                } if entity == self.game.player_id() => {
                    self.push_floating_message(
                        format!("DISSIMULÉ +{optical_difficulty_bonus}"),
                        at,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                    self.push_log("Dispositif explosif camouflé.".to_owned());
                }
                GameEvent::SoundEmitterDeployed {
                    entity,
                    at,
                    remaining_phases,
                    ..
                } if entity == self.game.player_id() => {
                    self.push_floating_message(
                        "LEURRE ACTIF",
                        at,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                    self.push_log(format!(
                        "Leurre sonore déployé · {remaining_phases} phase(s)."
                    ));
                }
                GameEvent::SoundEmitterExpired { at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "LEURRE ÉTEINT",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                }
                GameEvent::DamageApplied {
                    target,
                    at,
                    amount,
                    damage_type,
                    absorbed_by_armor,
                    ..
                } if target == self.game.player_id() => {
                    if amount == 0 {
                        self.push_floating_message(
                            if absorbed_by_armor > 0 {
                                "BLOQUÉ · BLINDAGE"
                            } else {
                                "AUCUN DÉGÂT"
                            },
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log(if absorbed_by_armor > 0 {
                            format!("Votre Blindage absorbe {absorbed_by_armor} dégât(s).")
                        } else {
                            "Impact reçu sans perte de PV.".to_owned()
                        });
                    } else {
                        self.push_floating_message(
                            format!("−{amount} PV"),
                            at,
                            FloatingMessageTone::Damage,
                            visual_time,
                        );
                        let armor = if absorbed_by_armor > 0 {
                            format!(" · Blindage absorbe {absorbed_by_armor}")
                        } else {
                            String::new()
                        };
                        self.push_log(format!(
                            "PV -{amount} · dégâts {}{armor}",
                            damage_type_label(damage_type).to_uppercase()
                        ));
                    }
                }
                GameEvent::DamageApplied { at, amount, .. }
                    if self.game.player_visibility().is_visible(at) =>
                {
                    if amount == 0 {
                        self.push_floating_message(
                            "AUCUN DÉGÂT",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log("Impact confirmé · aucun dégât observable.".to_owned());
                    } else {
                        self.push_floating_message(
                            format!("−{amount}"),
                            at,
                            FloatingMessageTone::Damage,
                            visual_time,
                        );
                        self.push_log(format!("Cible endommagée · -{amount} PV"));
                    }
                }
                GameEvent::DamageImpactApplied {
                    target,
                    at,
                    amount,
                    components,
                    absorbed_by_armor,
                    ..
                } if target == self.game.player_id() => {
                    if amount == 0 {
                        self.push_floating_message(
                            if absorbed_by_armor > 0 {
                                "BLOQUÉ · BLINDAGE"
                            } else {
                                "AUCUN DÉGÂT"
                            },
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log(if absorbed_by_armor > 0 {
                            format!("Votre Blindage absorbe {absorbed_by_armor} dégât(s).")
                        } else {
                            "Impact mixte reçu sans perte de PV.".to_owned()
                        });
                    } else {
                        self.push_floating_message(
                            format!("−{amount} PV"),
                            at,
                            FloatingMessageTone::Damage,
                            visual_time,
                        );
                        let types = components
                            .iter()
                            .map(|component| damage_type_label(component.damage_type))
                            .collect::<Vec<_>>()
                            .join(" + ")
                            .to_uppercase();
                        let armor = if absorbed_by_armor > 0 {
                            format!(" · Blindage absorbe {absorbed_by_armor}")
                        } else {
                            String::new()
                        };
                        self.push_log(format!("PV -{amount} · dégâts {types}{armor}"));
                    }
                }
                GameEvent::DamageImpactApplied { at, amount, .. }
                    if self.game.player_visibility().is_visible(at) =>
                {
                    if amount == 0 {
                        self.push_floating_message(
                            "AUCUN DÉGÂT",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log("Impact mixte confirmé · aucun dégât observable.".to_owned());
                    } else {
                        self.push_floating_message(
                            format!("−{amount}"),
                            at,
                            FloatingMessageTone::Damage,
                            visual_time,
                        );
                        self.push_log(format!("Cible endommagée · -{amount} PV"));
                    }
                }
                GameEvent::DroneEnergyDepleted { entity, at } => {
                    self.actor_glyphs.remove(&entity);
                    if self.selected_target == Some(entity) {
                        self.selected_target = None;
                    }
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "- DRONE",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log(
                        "Fin de vie du drone · manifestation dissipée. Drone spectral sera de nouveau utilisable après sa recharge."
                            .to_owned(),
                    );
                }
                GameEvent::DroneLinkLost { at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "LIAISON PERDUE",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log(
                        "Liaison du drone perdue · consigne locale maintenue.".to_owned(),
                    );
                }
                GameEvent::DroneLinkRestored { at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "LIAISON RÉTABLIE",
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log("Liaison du drone rétablie · doctrine reprise.".to_owned());
                }
                GameEvent::CompanionBehaviorChanged { behavior, .. } => {
                    self.push_log(format!(
                        "Comportement des alliés · {}.",
                        companion_behavior_label(behavior)
                    ));
                }
                GameEvent::EntityDied { entity, at } if entity != self.game.player_id() => {
                    self.actor_glyphs.remove(&entity);
                    if self.selected_target == Some(entity) {
                        self.selected_target = None;
                    }
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "DÉTRUIT",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log("Cible détruite.".to_owned());
                    }
                }
                GameEvent::EntityDestructionTriggered { at, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "DÉTONATION !",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                        self.push_log("CONTENEUR INSTABLE · DÉTONATION".to_owned());
                    }
                }
                GameEvent::ExperienceAwarded { amount, total, .. } => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            format!("+{amount} XP"),
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    self.push_log(format!("Expérience +{amount} · total {total}"));
                }
                GameEvent::LevelGained {
                    level,
                    skill_points_awarded,
                } => {
                    if open_level_up_screen {
                        let notice = level_up_notice.get_or_insert(LevelUpNotice {
                            level,
                            skill_points_awarded: 0,
                        });
                        notice.level = level;
                        notice.skill_points_awarded = notice
                            .skill_points_awarded
                            .saturating_add(u32::from(skill_points_awarded));
                    }
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            format!("NIVEAU {level} !  +{skill_points_awarded} PT COMP."),
                            at,
                            FloatingMessageTone::Progression,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "Niveau {level} atteint · points de compétence +{skill_points_awarded}"
                    ));
                }
                GameEvent::TechniqueLearned {
                    technique,
                    skill_points_remaining,
                    ..
                } => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            "TECHNIQUE APPRISE",
                            at,
                            FloatingMessageTone::Progression,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} apprise · {skill_points_remaining} point(s) restant(s)",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::TechniqueUsed {
                    technique,
                    observed_on_turn,
                    ..
                } => {
                    if matches!(
                        self.game
                            .rules()
                            .skills
                            .technique(&technique)
                            .and_then(|definition| definition.action()),
                        Some(
                            TechniqueAction::PrepareMeleeParry { .. }
                                | TechniqueAction::PrepareMeleeInterception
                                | TechniqueAction::WeaponAttack { .. }
                        )
                    ) {
                        continue;
                    }
                    self.observation_report = vec![format!(
                        "{} — relevé · cycle {observed_on_turn}",
                        self.technique_name(&technique)
                    )];
                    self.report_scroll = 0;
                    self.push_log(format!(
                        "Nouveau relevé disponible — {} : dossier.",
                        self.controls.label(Action::Report)
                    ));
                }
                GameEvent::EmissionSilenceChanged {
                    entity, silenced, ..
                } if entity == self.game.player_id() => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            if silenced {
                                "ÉMISSIONS COUPÉES"
                            } else {
                                "ÉMISSIONS ACTIVES"
                            },
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                }
                GameEvent::LowProfileChanged { entity, active, .. }
                    if entity == self.game.player_id() =>
                {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            if active {
                                "PROFIL RÉDUIT"
                            } else {
                                "PROFIL NORMAL"
                            },
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                }
                GameEvent::AmbushResolved {
                    entity,
                    target,
                    bonuses_applied,
                    silent_neutralization,
                } if entity == self.game.player_id() => {
                    if let Some(at) = self.game.actors().get(target).map(Actor::position) {
                        self.push_floating_message(
                            if silent_neutralization {
                                "NEUTRALISATION"
                            } else if bonuses_applied {
                                "EMBUSCADE !"
                            } else {
                                "REPÉRÉ"
                            },
                            at,
                            if bonuses_applied {
                                FloatingMessageTone::Status
                            } else {
                                FloatingMessageTone::Alert
                            },
                            visual_time,
                        );
                    }
                    if !bonuses_applied {
                        self.push_log(
                            "La cible a localisé l'attaque pendant la préparation : bonus perdus."
                                .to_owned(),
                        );
                    }
                }
                GameEvent::TrailBreakStarted {
                    entity,
                    remaining_steps,
                    remaining_turns,
                    ..
                } if entity == self.game.player_id() => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            "PISTE ROMPUE",
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "Rupture de piste · {remaining_steps} pas · {remaining_turns} tours au plus."
                    ));
                }
                GameEvent::TrailBreakEnded { entity, .. } if entity == self.game.player_id() => {
                    self.push_log("Rupture de piste terminée.".to_owned());
                }
                GameEvent::ActiveCamouflageChanged { entity, active, .. }
                    if entity == self.game.player_id() =>
                {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            if active {
                                "CAMOUFLAGE ACTIF"
                            } else {
                                "CAMOUFLAGE COUPÉ"
                            },
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                }
                GameEvent::TechniquePreparationStarted {
                    entity,
                    technique,
                    remaining_steps,
                }
                | GameEvent::TechniquePreparationAdvanced {
                    entity,
                    technique,
                    remaining_steps,
                } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            format!("PRÉPARATION · {remaining_steps}"),
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(format!(
                            "{} · préparation en cours ({remaining_steps} UT restante(s)).",
                            self.technique_name(&technique)
                        ));
                    }
                }
                GameEvent::TechniquePreparationCompleted { entity, technique } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            "EXÉCUTION",
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(format!(
                            "{} · préparation achevée.",
                            self.technique_name(&technique)
                        ));
                    }
                }
                GameEvent::TechniquePreparationCancelled {
                    entity,
                    technique,
                    reason,
                } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            "PRÉPARATION ANNULÉE",
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        let cause = match reason {
                            project_rl::game::PreparationCancellationReason::DifferentAction => {
                                "une autre action a été entreprise"
                            }
                            project_rl::game::PreparationCancellationReason::TargetUnavailable => {
                                "la cible requise n'est plus disponible"
                            }
                            project_rl::game::PreparationCancellationReason::Disrupted => {
                                "une perturbation a rompu la préparation"
                            }
                        };
                        self.push_log(format!(
                            "{} · préparation annulée : {cause}.",
                            self.technique_name(&technique)
                        ));
                    }
                }
                GameEvent::PreparationDisruptionResolved {
                    target, outcome, ..
                } => {
                    if target != self.game.player_id() {
                        continue;
                    }
                    let Some(position) = self
                        .game
                        .actors()
                        .get(target)
                        .map(Actor::position)
                    else {
                        continue;
                    };
                    match outcome {
                        PreparationDisruptionOutcome::Protected => {
                            self.push_floating_message(
                                "STABILISÉ",
                                position,
                                FloatingMessageTone::Status,
                                visual_time,
                            );
                            self.push_log(
                                "La protection transitoire absorbe une nouvelle perturbation."
                                    .to_owned(),
                            );
                        }
                        PreparationDisruptionOutcome::Resisted => {
                            self.push_floating_message(
                                "PRÉPARATION MAINTENUE",
                                position,
                                FloatingMessageTone::Status,
                                visual_time,
                            );
                            self.push_log(
                                "Stabilité suffisante : la préparation est maintenue.".to_owned(),
                            );
                        }
                        PreparationDisruptionOutcome::Interrupted => {
                            self.push_log(
                                "La perturbation franchit la Stabilité et interrompt la préparation."
                                    .to_owned(),
                            );
                        }
                    }
                }
                GameEvent::PreparationInterruptionProtectionChanged {
                    entity,
                    active,
                    ..
                } => {
                    if entity != self.game.player_id() {
                        continue;
                    }
                    self.push_log(if active {
                        "Protection anti-interruption active pour la prochaine phase.".to_owned()
                    } else {
                        "Protection anti-interruption dissipée.".to_owned()
                    });
                }
                GameEvent::TechniqueOnHitEffectRejected {
                    source,
                    target,
                    technique,
                    reason,
                } => {
                    let Some(position) = self.game.actors().get(target).map(Actor::position) else {
                        continue;
                    };
                    if source != self.game.player_id()
                        && !self.game.player_visibility().is_visible(position)
                    {
                        continue;
                    }
                    let (floating, cause) = match reason {
                        TechniqueEffectFailure::TargetHasNoArmor => {
                            ("SANS BLINDAGE", "la cible ne possède aucun Blindage")
                        }
                        TechniqueEffectFailure::TargetHasNoCompatibleLocomotion => (
                            "INCOMPATIBLE",
                            "la cible ne possède aucune locomotion compatible",
                        ),
                        TechniqueEffectFailure::TargetHasNoCompatibleSuppressionResponse => (
                            "INSENSIBLE",
                            "la cible ne possède aucune réponse compatible à la suppression",
                        ),
                        TechniqueEffectFailure::ProtectedFromEffect => {
                            ("PROTÉGÉ", "la cible bénéficie d'une protection temporaire")
                        }
                    };
                    self.push_floating_message(
                        floating,
                        position,
                        FloatingMessageTone::Information,
                        visual_time,
                    );
                    self.push_log(format!(
                        "{} · effet secondaire sans effet : {cause}.",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::StabilityCheckResolved {
                    target, resisted, ..
                } => {
                    let Some(position) = self.game.actors().get(target).map(Actor::position) else {
                        continue;
                    };
                    if self.game.player_visibility().is_visible(position) {
                        self.push_floating_message(
                            if resisted { "RÉSISTE" } else { "ENTRAVÉ" },
                            position,
                            if resisted {
                                FloatingMessageTone::Information
                            } else {
                                FloatingMessageTone::Status
                            },
                            visual_time,
                        );
                    }
                    self.push_log(if resisted {
                        "Test de Stabilité réussi · l'entrave est évitée.".to_owned()
                    } else {
                        "Test de Stabilité échoué · locomotion entravée.".to_owned()
                    });
                }
                GameEvent::ActionRecoveryStarted {
                    entity,
                    remaining_actions,
                }
                | GameEvent::ActionRecoveryAdvanced {
                    entity,
                    remaining_actions,
                } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            format!("RÉCUPÉRATION · {remaining_actions}"),
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(format!(
                            "Récupération offensive · {remaining_actions} action(s) normale(s) restante(s)."
                        ));
                    }
                }
                GameEvent::ActionRecoveryCompleted { entity } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            "PRÊT",
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(
                            "Récupération achevée · actions offensives disponibles.".to_owned(),
                        );
                    }
                }
                GameEvent::TechniqueCooldownStarted {
                    entity,
                    technique,
                    remaining_phases,
                }
                | GameEvent::TechniqueCooldownAdvanced {
                    entity,
                    technique,
                    remaining_phases,
                } => {
                    if entity == self.game.player_id() {
                        self.push_log(format!(
                            "{} · recharge : {remaining_phases} phase(s) restante(s).",
                            self.technique_name(&technique)
                        ));
                    }
                }
                GameEvent::TechniqueCooldownCompleted { entity, technique } => {
                    if entity == self.game.player_id() {
                        self.push_log(format!(
                            "{} · de nouveau disponible.",
                            self.technique_name(&technique)
                        ));
                    }
                }
                GameEvent::ReactionPrepared {
                    entity,
                    technique,
                    reaction,
                } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            "GARDE PRÉPARÉE",
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                        self.push_log(format!(
                            "{} · {} préparée.",
                            self.technique_name(&technique),
                            match reaction {
                                project_rl::reaction::ReactionKind::EvasiveStep => {
                                    "garde d'esquive"
                                }
                                project_rl::reaction::ReactionKind::MeleeParry => {
                                    "garde de parade"
                                }
                                project_rl::reaction::ReactionKind::MeleeInterception => {
                                    "garde d'interception"
                                }
                                project_rl::reaction::ReactionKind::RangedOverwatch => {
                                    "surveillance de tir"
                                }
                            }
                        ));
                    }
                }
                GameEvent::ReactionExpired {
                    entity, reaction, ..
                } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            "GARDE EXPIRÉE",
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(
                            match reaction {
                                project_rl::reaction::ReactionKind::EvasiveStep => {
                                    "Esquive préparée expirée sans se déclencher."
                                }
                                project_rl::reaction::ReactionKind::MeleeParry => {
                                    "Garde de parade expirée sans se déclencher."
                                }
                                project_rl::reaction::ReactionKind::MeleeInterception => {
                                    "Garde d'interception expirée sans se déclencher."
                                }
                                project_rl::reaction::ReactionKind::RangedOverwatch => {
                                    "Surveillance expirée sans se déclencher."
                                }
                            }
                            .to_owned(),
                        );
                    }
                }
                GameEvent::ReactionTriggered {
                    reactor, reaction, ..
                } => {
                    if let Some(position) = self.game.actors().get(reactor).map(Actor::position)
                        && (reactor == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            match reaction {
                                project_rl::reaction::ReactionKind::EvasiveStep => "ESQUIVE !",
                                project_rl::reaction::ReactionKind::MeleeParry => "PARADE !",
                                project_rl::reaction::ReactionKind::MeleeInterception => {
                                    "INTERCEPTION !"
                                }
                                project_rl::reaction::ReactionKind::RangedOverwatch => {
                                    "SURVEILLANCE !"
                                }
                            },
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                }
                GameEvent::PersistentRangedAimStarted {
                    entity,
                    technique,
                    accuracy_modifier,
                    ..
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            "VISÉE MAINTENUE",
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} · Précision {accuracy_modifier:+} conservée sur la même cible.",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::PersistentRangedAimEnded { entity, .. }
                    if entity == self.game.player_id() =>
                {
                    self.push_log("Visée persistante interrompue.".to_owned());
                }
                GameEvent::WeaponBarrageStageResolved {
                    entity,
                    remaining_stages,
                    ..
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            if remaining_stages > 0 {
                                "BARRAGE · MAINTENIR"
                            } else {
                                "BARRAGE TERMINÉ"
                            },
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    if remaining_stages > 0 {
                        self.push_log(format!(
                            "Barrage : {remaining_stages} étape(s) restante(s). Réutilisez la technique sur la même zone."
                        ));
                    }
                }
                GameEvent::WeaponBarrageCancelled {
                    entity,
                    remaining_stages,
                    ..
                } if entity == self.game.player_id() => {
                    self.push_log(format!(
                        "Barrage interrompu : {remaining_stages} étape(s) annulée(s)."
                    ));
                }
                GameEvent::ChargeStarted {
                    entity,
                    technique,
                    required_advances,
                    ..
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            format!("CHARGE · {required_advances}"),
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} · charge engagée sur {required_advances} case(s).",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::ChargeAdvanced {
                    entity,
                    to,
                    completed_advances,
                    required_advances,
                    ..
                } if entity == self.game.player_id() => {
                    self.push_floating_message(
                        format!("ÉLAN {completed_advances}/{required_advances}"),
                        to,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                }
                GameEvent::ChargeCompleted {
                    entity,
                    recovery_suppressed,
                    ..
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            "IMPACT DE CHARGE !",
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(if recovery_suppressed {
                        "Charge achevée · inertie contrôlée, aucune récupération imposée."
                            .to_owned()
                    } else {
                        "Charge achevée · récupération offensive requise.".to_owned()
                    });
                }
                GameEvent::ChargeCancelled {
                    entity,
                    technique,
                    completed_advances,
                    controlled,
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            if controlled {
                                "CHARGE CONTRÔLÉE"
                            } else {
                                "CHARGE ROMPUE"
                            },
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} · charge interrompue après {completed_advances} case(s){}.",
                        self.technique_name(&technique),
                        if controlled {
                            " sans récupération"
                        } else {
                            ""
                        }
                    ));
                }
                GameEvent::AnchorPrepared {
                    entity,
                    technique,
                    displacement_resistance_bonus,
                } if entity == self.game.player_id() => {
                    if let Some(position) = self.game.player_position() {
                        self.push_floating_message(
                            format!("ANCRAGE +{displacement_resistance_bonus}"),
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} · résistance au déplacement +{displacement_resistance_bonus} jusqu'au prochain déplacement.",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::AnchorEnded { entity, technique } if entity == self.game.player_id() => {
                    self.push_log(format!(
                        "{} · ancrage rompu par le déplacement.",
                        self.technique_name(&technique)
                    ));
                }
                GameEvent::EvasiveStepResolved {
                    entity,
                    from,
                    to,
                    moved,
                } => {
                    let visible = self.game.player_visibility().is_visible(from)
                        || self.game.player_visibility().is_visible(to);
                    if entity == self.game.player_id() || visible {
                        self.push_floating_message(
                            if moved {
                                "ESQUIVE !"
                            } else {
                                "ESQUIVE BLOQUÉE"
                            },
                            if moved { to } else { from },
                            if moved {
                                FloatingMessageTone::Status
                            } else {
                                FloatingMessageTone::Information
                            },
                            visual_time,
                        );
                    }
                    if entity == self.game.player_id() {
                        self.push_log(if moved {
                            "Pas d'esquive exécuté avant la résolution de l'attaque.".to_owned()
                        } else {
                            "Pas d'esquive consommé, mais destination désormais bloquée.".to_owned()
                        });
                    }
                }
                GameEvent::ObstacleTraversed {
                    entity, to, ..
                } if entity == self.game.player_id() => {
                    self.push_floating_message(
                        "FRANCHISSEMENT",
                        to,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                    let location = observer_location(self.game.player_position(), to);
                    self.push_log(format!("Obstacle franchi · arrivée {location}."));
                }
                GameEvent::AllyExtracted {
                    entity,
                    ally_to,
                    player_to,
                    ..
                } if entity == self.game.player_id() => {
                    self.push_floating_message(
                        "EXTRACTION",
                        ally_to,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                    let ally_location = observer_location(self.game.player_position(), ally_to);
                    let player_location =
                        observer_location(self.game.player_position(), player_to);
                    self.push_log(format!(
                        "Allié extrait {ally_location} · repli du joueur {player_location}."
                    ));
                }
                GameEvent::PhysicalDamageParried {
                    reactor,
                    before,
                    after,
                    ..
                } if reactor == self.game.player_id() => {
                    self.push_log(format!(
                        "Parade · dégâts physiques bruts {before} → {after}, avant Blindage."
                    ));
                }
                GameEvent::CounterattackResolved {
                    reactor, outcome, ..
                } => {
                    let (message, log) = match outcome {
                        CounterattackOutcome::Performed => ("RIPOSTE !", "Riposte exécutée."),
                        CounterattackOutcome::ReactorUnavailable => (
                            "RIPOSTE ANNULÉE",
                            "Riposte annulée : défenseur hors combat.",
                        ),
                        CounterattackOutcome::SourceUnavailable => (
                            "RIPOSTE ANNULÉE",
                            "Riposte annulée : attaquant hors combat.",
                        ),
                        CounterattackOutcome::NoMeleeWeapon => (
                            "RIPOSTE IMPOSSIBLE",
                            "Riposte impossible : aucune arme de mêlée disponible.",
                        ),
                        CounterattackOutcome::OutOfReach => (
                            "HORS DE PORTÉE",
                            "Riposte impossible : l'attaquant n'est plus au contact.",
                        ),
                    };
                    if let Some(position) = self.game.actors().get(reactor).map(Actor::position)
                        && (reactor == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            message,
                            position,
                            if outcome == CounterattackOutcome::Performed {
                                FloatingMessageTone::Status
                            } else {
                                FloatingMessageTone::Information
                            },
                            visual_time,
                        );
                    }
                    if reactor == self.game.player_id() {
                        self.push_log(log.to_owned());
                    }
                }
                GameEvent::InterceptionResolved {
                    reactor, outcome, ..
                } => {
                    let (message, log) = match outcome {
                        InterceptionOutcome::Performed => (
                            "FRAPPE D'ARRÊT !",
                            "Interception exécutée avant le retrait.",
                        ),
                        InterceptionOutcome::ReactorUnavailable => (
                            "INTERCEPTION ANNULÉE",
                            "Interception annulée : défenseur hors combat.",
                        ),
                        InterceptionOutcome::MoverUnavailable => (
                            "INTERCEPTION ANNULÉE",
                            "Interception annulée : cible déjà hors combat.",
                        ),
                        InterceptionOutcome::NoMeleeWeapon => (
                            "INTERCEPTION IMPOSSIBLE",
                            "Interception impossible : aucune arme de mêlée disponible.",
                        ),
                        InterceptionOutcome::OutOfReach => (
                            "HORS DE PORTÉE",
                            "Interception impossible : le contact n'est plus valide.",
                        ),
                    };
                    if let Some(position) = self.game.actors().get(reactor).map(Actor::position)
                        && (reactor == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            message,
                            position,
                            if outcome == InterceptionOutcome::Performed {
                                FloatingMessageTone::Status
                            } else {
                                FloatingMessageTone::Information
                            },
                            visual_time,
                        );
                    }
                    if reactor == self.game.player_id() {
                        self.push_log(log.to_owned());
                    }
                }
                GameEvent::EnergySpent {
                    amount, remaining, ..
                } => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            format!("−{amount} E"),
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                    self.observation_report
                        .push(format!("Énergie : -{amount} E ; réserve {remaining} E."));
                }
                GameEvent::DroneControlEstablished { entity, .. } => {
                    self.actor_glyphs.insert(entity, 'u');
                    if let Some(at) = self.game.actors().get(entity).map(Actor::position) {
                        self.push_floating_message(
                            "+ DRONE",
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                }
                GameEvent::BandwidthReserved {
                    amount,
                    occupied,
                    capacity,
                    ..
                } => self.observation_report.push(format!(
                    "Bande passante : {amount} B réservé ; {occupied}/{capacity} B occupés."
                )),
                GameEvent::BandwidthReleased {
                    amount,
                    occupied,
                    capacity,
                    ..
                } => self.observation_report.push(format!(
                    "Bande passante : {amount} B libéré ; {occupied}/{capacity} B occupés."
                )),
                GameEvent::HeatGenerated {
                    amount, current, ..
                } => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            format!("+{amount} H"),
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.observation_report
                        .push(format!("Chaleur : +{amount} H ; niveau {current} H."));
                }
                GameEvent::HeatDissipated {
                    amount, current, ..
                } => self
                    .observation_report
                    .push(format!("Dissipation : -{amount} H ; niveau {current} H.")),
                GameEvent::HeatThresholdCrossed {
                    critical, current, ..
                } => {
                    let message = if critical {
                        format!("SEUIL THERMIQUE CRITIQUE · {current} H")
                    } else {
                        format!("ALERTE THERMIQUE · {current} H")
                    };
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            message.clone(),
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log(message);
                }
                GameEvent::TargetAnalyzed {
                    at,
                    integrity,
                    maximum_integrity,
                    armor,
                    resistances,
                    ..
                } => {
                    let resistance_summary = [
                        ("KIN", DamageType::Kinetic),
                        ("PIR", DamageType::Piercing),
                        ("EXP", DamageType::Explosive),
                        ("THR", DamageType::Thermal),
                        ("ELE", DamageType::Electrical),
                        ("CHM", DamageType::Chemical),
                        ("RAD", DamageType::Radiation),
                        ("COR", DamageType::Corruption),
                    ]
                    .into_iter()
                    .filter_map(|(label, damage_type)| {
                        let value = resistances.get(damage_type);
                        (value != 0).then_some(format!("{label} {value:+}%"))
                    })
                    .collect::<Vec<_>>();
                    let resistance_summary = if resistance_summary.is_empty() {
                        "Aucune résistance identifiable".to_owned()
                    } else {
                        resistance_summary.join("  ")
                    };
                    let location = self
                        .game
                        .player_position()
                        .map_or_else(|| "dans la zone".to_owned(), |observer| {
                            relative_location_label(observer, at)
                        });
                    let message = format!(
                        "Cible observée {location} : PV {integrity}/{maximum_integrity} — Blindage {armor} — {resistance_summary}"
                    );
                    self.observation_report.push(message.clone());
                    self.push_log(message);
                }
                GameEvent::PhysicalWeaknessIdentified { observer, .. }
                    if observer == self.game.player_id() =>
                {
                    self.push_log(
                        "Faiblesse physique identifiée : Tir de rupture peut l'exploiter."
                            .to_owned(),
                    );
                }
                GameEvent::BodyComponentIdentified {
                    observer,
                    component,
                    ..
                } if observer == self.game.player_id() => {
                    self.push_log(format!(
                        "Composant identifié : {}.",
                        self.body_component_name(&component)
                    ));
                }
                GameEvent::BodyComponentDamaged {
                    target,
                    component,
                    amount,
                    durability,
                    maximum_durability,
                    failed,
                    ..
                } => {
                    if let Some(position) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(position)
                    {
                        self.push_floating_message(
                            if failed {
                                "COMPOSANT HS"
                            } else {
                                "COMPOSANT TOUCHÉ"
                            },
                            position,
                            if failed {
                                FloatingMessageTone::Alert
                            } else {
                                FloatingMessageTone::Damage
                            },
                            visual_time,
                        );
                        self.push_log(format!(
                            "{} · −{amount} durabilité · {durability}/{maximum_durability}{}.",
                            self.body_component_name(&component),
                            if failed {
                                " · fonction défaillante"
                            } else {
                                ""
                            }
                        ));
                    }
                }
                GameEvent::WreckCreated { at, components, .. } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "CARCASSE",
                            at,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                        self.push_log(format!(
                            "Carcasse exploitable · {} composant(s) récupérable(s).",
                            components.len()
                        ));
                    }
                }
                GameEvent::BodyComponentRepaired {
                    target,
                    component,
                    amount,
                    durability,
                    maximum_durability,
                } => {
                    if let Some(at) = self.game.actors().get(target).map(Actor::position) {
                        self.push_floating_message(
                            format!("+{amount} DUR"),
                            at,
                            FloatingMessageTone::Recovery,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "{} réparé · +{amount} · {durability}/{maximum_durability}.",
                        self.body_component_name(&component)
                    ));
                }
                GameEvent::BodyComponentSalvaged {
                    component,
                    durability,
                    maximum_durability,
                    ..
                } => self.push_log(format!(
                    "{} récupéré avec sa durabilité réelle : {durability}/{maximum_durability}.",
                    self.body_component_name(&component)
                )),
                GameEvent::BodyComponentDiagnosed {
                    component,
                    analysis_score,
                    durability,
                    maximum_durability,
                    failed,
                    destroyed,
                    ..
                } => {
                    let state = if destroyed {
                        "détruit"
                    } else if failed {
                        "défaillant"
                    } else {
                        "opérationnel"
                    };
                    let message = format!(
                        "Diagnostic {} · {durability}/{maximum_durability} · {state} · analyse {analysis_score}.",
                        self.body_component_name(&component)
                    );
                    self.observation_report.push(message.clone());
                    self.push_log(message);
                }
                GameEvent::ModuleTuned {
                    tuning,
                    output_percentage,
                    energy_percentage,
                    ..
                } => self.push_log(format!(
                    "Module réglé en mode {} · sortie {output_percentage}% · énergie {energy_percentage}%.",
                    match tuning {
                        ModuleTuning::Economy => "économie",
                        ModuleTuning::Power => "puissance",
                    }
                )),
                GameEvent::ModuleOverclockChanged {
                    output_percentage,
                    remaining_time_units,
                    ..
                } => {
                    let message = if remaining_time_units == 0 {
                        "SURCADENÇAGE TERMINÉ".to_owned()
                    } else {
                        format!(
                            "SURCADENÇAGE {output_percentage}% · {remaining_time_units} UT"
                        )
                    };
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            message.clone(),
                            at,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                    }
                    self.push_log(message);
                }
                GameEvent::ModuleDurabilityDamaged {
                    amount,
                    durability,
                    maximum_durability,
                    ..
                } => {
                    if let Some(at) = self.game.player_position() {
                        self.push_floating_message(
                            format!("−{amount} DUR"),
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "Surchauffe du module · {durability}/{maximum_durability}."
                    ));
                }
                GameEvent::BodyComponentBypassed {
                    receiver,
                    donor,
                    restored_output_percentage,
                    ..
                } => self.push_log(format!(
                    "Dérivation active · {} restauré à {restored_output_percentage}% via {}.",
                    self.body_component_name(&receiver),
                    self.body_component_name(&donor)
                )),
                GameEvent::BodyComponentBypassEnded { receiver, .. } => self.push_log(format!(
                    "Dérivation terminée pour {}.",
                    self.body_component_name(&receiver)
                )),
                GameEvent::ModuleReconditioned {
                    amount,
                    durability,
                    maximum_durability,
                    ..
                } => self.push_log(format!(
                    "Module reconditionné · +{amount} · {durability}/{maximum_durability}."
                )),
                GameEvent::FieldBeaconAssembled {
                    at, stored_energy, ..
                } => {
                    self.push_floating_message(
                        "BALISE ACTIVE",
                        at,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                    self.push_log(format!(
                        "Balise de terrain assemblée · réserve {stored_energy} E."
                    ));
                }
                GameEvent::DigitalInterfaceProbed {
                    at,
                    analysis_score,
                    rights,
                    defense,
                    ..
                } => {
                    self.push_floating_message(
                        "INTERFACE ANALYSÉE",
                        at,
                        FloatingMessageTone::Information,
                        visual_time,
                    );
                    self.push_log(format!(
                        "Interface analysée · score {analysis_score} · défense {defense} · {} droit(s) observable(s).",
                        rights.len()
                    ));
                }
                GameEvent::IntrusionAttemptResolved {
                    at,
                    chance,
                    roll,
                    succeeded,
                    hardening,
                    ..
                } => {
                    self.push_floating_message(
                        if succeeded { "ACCÈS OUVERT" } else { "INTRUSION ÉCHOUÉE" },
                        at,
                        if succeeded {
                            FloatingMessageTone::Recovery
                        } else {
                            FloatingMessageTone::Alert
                        },
                        visual_time,
                    );
                    self.push_log(format!(
                        "Intrusion · jet {roll}/{chance} · {} · durcissement {hardening}.",
                        if succeeded { "réussite" } else { "échec" }
                    ));
                }
                GameEvent::DigitalAccessGranted {
                    at,
                    remaining_time_units,
                    ..
                } => {
                    let location = observer_location(self.game.player_position(), at);
                    self.push_log(format!(
                        "Session locale accordée {location} · {remaining_time_units} UT."
                    ));
                }
                GameEvent::DigitalAccessExpired { at } => {
                    let location = observer_location(self.game.player_position(), at);
                    self.push_log(format!("Session locale expirée {location}."));
                }
                GameEvent::ElectronicLockForced { door, .. } => {
                    self.push_floating_message(
                        "VERROU FORCÉ",
                        door,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                }
                GameEvent::DataLotExtracted { source, .. } => {
                    let location = observer_location(self.game.player_position(), source);
                    self.push_log(format!("Lot de données extrait depuis {location}."));
                }
                GameEvent::DeviceControlChanged {
                    at,
                    active,
                    ..
                } => self.push_floating_message(
                    if active { "CONTRÔLE DÉTOURNÉ" } else { "CONTRÔLE REPRIS" },
                    at,
                    if active {
                        FloatingMessageTone::Status
                    } else {
                        FloatingMessageTone::Alert
                    },
                    visual_time,
                ),
                GameEvent::DeviceControlRecaptureBlocked { at } => {
                    self.push_floating_message(
                        "REPRISE BLOQUÉE",
                        at,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                }
                GameEvent::DigitalRoutineChanged {
                    at, suspended, ..
                } => {
                    let location = observer_location(self.game.player_position(), at);
                    self.push_log(format!(
                        "Routine {location} : {}.",
                        if suspended { "suspendue" } else { "réactivée" }
                    ));
                }
                GameEvent::BackdoorChanged { at, installed } => {
                    let location = observer_location(self.game.player_position(), at);
                    self.push_log(format!(
                        "Porte dérobée {location} : {}.",
                        if installed { "installée" } else { "supprimée" }
                    ));
                }
                GameEvent::SecurityTraceFalsified { at, .. } => {
                    self.push_floating_message(
                        "TRACE FALSIFIÉE",
                        at,
                        FloatingMessageTone::Information,
                        visual_time,
                    );
                }
                GameEvent::SecurityTraceAudited {
                    at, falsified, ..
                } => {
                    self.push_floating_message(
                        if falsified { "AUDIT TROMPÉ" } else { "TRACE DÉTECTÉE" },
                        at,
                        if falsified {
                            FloatingMessageTone::Information
                        } else {
                            FloatingMessageTone::Alert
                        },
                        visual_time,
                    );
                    self.push_log(if falsified {
                        "Audit local : la preuve falsifiée n'a pas déclenché de réponse.".to_owned()
                    } else {
                        "ALERTE NUMÉRIQUE · une trace non falsifiée a été auditée.".to_owned()
                    });
                }
                GameEvent::SubnetCommandIssued { devices, .. } => self.push_log(format!(
                    "Commande de sous-réseau transmise à {} dispositif(s).",
                    devices.len()
                )),
                GameEvent::DeviceControlLockChanged { at, active } => {
                    let location = observer_location(self.game.player_position(), at);
                    self.push_log(format!(
                        "Verrouillage de contrôle {location} : {}.",
                        if active { "actif" } else { "terminé" }
                    ));
                }
                GameEvent::ElectronicPulseResolved {
                    cells,
                    affected,
                    ..
                } => self.push_log(format!(
                    "Impulsion électronique · {} case(s), {} système(s) affecté(s).",
                    cells.len(),
                    affected.len()
                )),
                GameEvent::HostileProgramAttemptResolved {
                    target,
                    chance,
                    roll,
                    succeeded,
                    ..
                } => {
                    if let Some(at) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(at)
                    {
                        self.push_floating_message(
                            if succeeded { "PROGRAMME IMPLANTÉ" } else { "IMPLANTATION ÉCHOUÉE" },
                            at,
                            if succeeded {
                                FloatingMessageTone::Status
                            } else {
                                FloatingMessageTone::Alert
                            },
                            visual_time,
                        );
                    }
                    self.push_log(format!(
                        "Guerre électronique · jet {roll}/{chance} · {}.",
                        if succeeded { "réussite" } else { "échec" }
                    ));
                }
                GameEvent::HostileProgramChanged { target, active, .. } => {
                    if let Some(at) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(at)
                    {
                        self.push_floating_message(
                            if active { "PROGRAMME HOSTILE" } else { "PROGRAMME PURGÉ" },
                            at,
                            if active {
                                FloatingMessageTone::Alert
                            } else {
                                FloatingMessageTone::Recovery
                            },
                            visual_time,
                        );
                    }
                }
                GameEvent::ElectronicJammingChanged { active, .. } => self.push_log(
                    if active {
                        "Brouillage électronique maintenu.".to_owned()
                    } else {
                        "Brouillage électronique terminé.".to_owned()
                    },
                ),
                GameEvent::ElectronicCascadeResolved { targets, .. } => self.push_log(format!(
                    "Cascade électronique · {} relais touché(s).",
                    targets.len()
                )),
                GameEvent::SaturationBeaconDeployed { beacon, at, active } => {
                    self.actor_glyphs.insert(beacon, 'b');
                    self.push_floating_message(
                        if active { "BALISE ACTIVE" } else { "BALISE ARMÉE" },
                        at,
                        FloatingMessageTone::Status,
                        visual_time,
                    );
                }
                GameEvent::SaturationBeaconActivated { beacon } => {
                    if let Some(at) = self.game.actors().get(beacon).map(Actor::position) {
                        self.push_floating_message(
                            "SATURATION ACTIVE",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                }
                GameEvent::SaturationBeaconExpired { at, .. } => self.push_floating_message(
                    "BALISE ÉPUISÉE",
                    at,
                    FloatingMessageTone::Information,
                    visual_time,
                ),
                GameEvent::ElectronicImplosionDetonated { at, .. } => self.push_floating_message(
                    "IMPLOSION !",
                    at,
                    FloatingMessageTone::Alert,
                    visual_time,
                ),
                GameEvent::HostileProgramTicked { .. } => {}
                GameEvent::MovementTracesRead { traces, .. } => {
                    let observer = self.game.player_position();
                    if traces.is_empty() {
                        self.observation_report
                            .push("Aucune trace accessible dans la zone examinée.".to_owned());
                    }
                    for trace in &traces {
                        let direction = match trace.direction {
                            Direction::North => "nord",
                            Direction::East => "est",
                            Direction::South => "sud",
                            Direction::West => "ouest",
                        };
                        let location = observer.map_or_else(
                            || "dans la zone".to_owned(),
                            |observer| relative_location_label(observer, trace.position),
                        );
                        self.observation_report.push(format!(
                            "Trace repérée {location} : déplacement vers le {direction}, datant d'environ {} tour(s).",
                            trace.age_turns
                        ));
                    }
                    self.trace_cells = traces
                        .iter()
                        .map(|trace| (trace.position, trace.direction))
                        .collect();
                    self.traces_visible_until = replay_time.unwrap_or_else(get_time) + 1.5;
                    let tile_count = self.trace_cells.len();
                    let oldest = traces
                        .iter()
                        .map(|trace| trace.age_turns)
                        .max()
                        .unwrap_or(0);
                    self.push_log(format!(
                        "Traces · {} indice(s) sur {tile_count} case(s) · plus ancien {oldest} tour(s)",
                        traces.len()
                    ));
                }
                GameEvent::SecretsInspected {
                    discovered_explosives,
                    ..
                } => {
                    if discovered_explosives.is_empty() {
                        self.observation_report.push(
                            "Inspection terminée : aucun nouvel indice détectable.".to_owned(),
                        );
                        self.push_log("Inspection · aucun nouvel indice".to_owned());
                    } else {
                        for device in &discovered_explosives {
                            if let Some(device) = self.game.explosive_devices().get(*device) {
                                let location = observer_location(
                                    self.game.player_position(),
                                    device.position(),
                                );
                                self.observation_report.push(format!(
                                    "Dispositif dissimulé découvert {location} ; approchez-vous pour interagir."
                                ));
                            }
                        }
                        self.push_log(format!(
                            "Inspection · {} dispositif(s) découvert(s)",
                            discovered_explosives.len()
                        ));
                    }
                }
                GameEvent::TerrainAnalyzed { tiles, .. } => {
                    if tiles.is_empty() {
                        self.observation_report
                            .push("Aucune paroi accessible dans la zone examinée.".to_owned());
                    }
                    for tile in &tiles {
                        let location = observer_location(self.game.player_position(), tile.position);
                        self.observation_report.push(format!(
                            "Paroi repérée {location} : passage {}, visibilité {}.",
                            if tile.blocks_movement {
                                "bloqué"
                            } else {
                                "libre"
                            },
                            if tile.blocks_vision {
                                "bloquée"
                            } else {
                                "libre"
                            }
                        ));
                    }
                    if let Ok(cue) = VisualCue::world(
                        visual_cue_id("core:structure_scan"),
                        self.game.player_position().unwrap_or(GridPos::new(0, 0)),
                        tiles
                            .iter()
                            .map(|tile| VisualCueCell::new(tile.position, 0)),
                    ) {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    self.push_log(format!(
                        "Structure · {} paroi(s) liée(s) · solides et opaques",
                        tiles.len()
                    ));
                }
                GameEvent::ThreatAnalyzed {
                    target,
                    attacks,
                    armor,
                    resistances,
                    ..
                } => {
                    let specialized = [
                        ("THR", DamageType::Thermal),
                        ("ELE", DamageType::Electrical),
                        ("CHM", DamageType::Chemical),
                    ]
                    .into_iter()
                    .filter_map(|(label, damage_type)| {
                        let value = resistances.get(damage_type);
                        (value != 0).then_some(format!("{label} {value:+}%"))
                    })
                    .collect::<Vec<_>>();
                    self.observation_report.push(format!(
                        "Défenses observées : Blindage {armor}{}.",
                        if specialized.is_empty() {
                            String::new()
                        } else {
                            format!(" · {}", specialized.join(" · "))
                        }
                    ));
                    if attacks.is_empty() {
                        self.observation_report
                            .push("Aucune attaque renseignée dans ce profil.".to_owned());
                    }
                    for attack in &attacks {
                        let authored = attack.damage();
                        let resolved = self
                            .game
                            .resolved_attack_damage(target, *attack)
                            .unwrap_or(authored);
                        let physical = attack.melee_impact().map_or_else(String::new, |impact| {
                            format!(
                                " (référence {}, plafond d'impact {})",
                                physical_damage_total(authored),
                                impact.material_cap
                            )
                        });
                        self.observation_report.push(format!("Attaque : portée {}, dégâts actuels {}{physical}, pénétration {} ; ligne de vue {}.", attack.range(), format_damage_impact(resolved), format_damage_penetration(resolved), if attack.requires_line_of_sight() { "requise" } else { "non requise" }));
                    }
                    let profile = attacks.first().map_or_else(
                        || "Aucune attaque observable".to_owned(),
                        |attack| {
                            let damage = self
                                .game
                                .resolved_attack_damage(target, *attack)
                                .unwrap_or_else(|| attack.damage());
                            format!(
                                "Portée {} · dégâts {} · pénétration {}",
                                attack.range(),
                                format_damage_impact(damage),
                                format_damage_penetration(damage)
                            )
                            .to_uppercase()
                        },
                    );
                    self.push_log(format!("Menace · {profile}"));
                }
                GameEvent::EnergyAnalyzed { target, state, .. } => {
                    let mut channels = Vec::new();
                    if let Some((available, capacity)) =
                        state.energy_available.zip(state.energy_capacity)
                    {
                        channels.push(format!("Énergie {available}/{capacity} E"));
                    }
                    if let Some(heat) = state.heat {
                        channels.push(format!("Chaleur {heat} H"));
                    }
                    if let Some((occupied, capacity)) =
                        state.bandwidth_occupied.zip(state.bandwidth_capacity)
                    {
                        channels.push(format!("Bande passante {occupied}/{capacity} B"));
                    }
                    let summary = if channels.is_empty() {
                        "aucun canal lisible".to_owned()
                    } else {
                        channels.join(" · ")
                    };
                    self.observation_report.push(format!(
                        "Diagnostic énergétique de l'entité {} · Analyse {} · {summary}.",
                        target.get(),
                        state.analysis_score
                    ));
                    self.push_log(format!("Diagnostic · {summary}"));
                }
                GameEvent::StatusApplied {
                    target,
                    status,
                    stacks,
                    remaining_turns,
                    application,
                    ..
                } => {
                    let subject = if target == self.game.player_id() {
                        "le noyau"
                    } else {
                        "la cible"
                    };
                    let duration = remaining_turns
                        .map(|turns| format!("{turns} tour(s)"))
                        .unwrap_or_else(|| "permanent".to_owned());
                    if application == StatusApplyKind::Ignored {
                        self.push_log(format!(
                            "{} déjà actif sur {subject} · durée inchangée ({duration}).",
                            display_content_name(&status),
                        ));
                    } else {
                        self.push_log(format!(
                            "{} sur {subject} ×{stacks} ({duration})",
                            display_content_name(&status),
                        ));
                    }
                    if let Some(position) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(position)
                    {
                        if application == StatusApplyKind::Ignored {
                            self.push_floating_message(
                                "DÉJÀ ACTIF",
                                position,
                                FloatingMessageTone::Information,
                                visual_time,
                            );
                            continue;
                        }
                        self.push_floating_message(
                            format!("{} ×{stacks}", status_display_name(&status)),
                            position,
                            FloatingMessageTone::Status,
                            visual_time,
                        );
                        self.visual_cues.play(
                            VisualCue::point(status, position),
                            replay_time.unwrap_or_else(get_time),
                        );
                    }
                }
                GameEvent::StatusApplicationBlocked {
                    target,
                    status,
                    blocking_status,
                    ..
                } => {
                    self.push_log(format!(
                        "{} bloqué par {}.",
                        display_content_name(&status),
                        display_content_name(&blocking_status)
                    ));
                    if let Some(position) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(position)
                    {
                        self.push_floating_message(
                            "PROTÉGÉ",
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                }
                GameEvent::StatusTriggered { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "le noyau"
                    } else {
                        "la cible"
                    };
                    self.push_log(format!(
                        "{} se déclenche sur {subject}",
                        display_content_name(&status)
                    ));
                }
                GameEvent::StatusRemoved { target, status, .. } => {
                    let subject = if target == self.game.player_id() {
                        "le noyau"
                    } else {
                        "la cible"
                    };
                    self.push_log(format!(
                        "{} expire sur {subject}",
                        display_content_name(&status)
                    ));
                    if let Some(position) = self.game.actors().get(target).map(Actor::position)
                        && self.game.player_visibility().is_visible(position)
                    {
                        self.push_floating_message(
                            format!("{} TERMINÉE", status_display_name(&status)),
                            position,
                            FloatingMessageTone::Information,
                            visual_time,
                        );
                    }
                }
                GameEvent::WeaponEquipped {
                    slot,
                    weapon,
                    displaced,
                    ..
                } => {
                    let displaced = displaced
                        .map(|_| " · arme précédente rangée".to_owned())
                        .unwrap_or_default();
                    self.push_log(format!(
                        "Emplacement d'arme {} ← {}{}",
                        slot + 1,
                        display_content_name(&weapon),
                        displaced
                    ));
                }
                GameEvent::ItemEquipped {
                    definition,
                    displaced,
                    ..
                } => {
                    let displaced = displaced
                        .map(|_| " · protection précédente rangée")
                        .unwrap_or_default();
                    self.push_log(format!(
                        "Protection équipée : {}{displaced}",
                        self.item_name(&definition)
                    ));
                }
                GameEvent::ItemUsed { definition, .. } => {
                    self.push_log(format!("{} utilisé.", self.item_name(&definition)));
                }
                GameEvent::ItemPickedUp {
                    definition,
                    quantity,
                    ..
                } => {
                    self.push_log(format!(
                        "{} acquis ×{quantity}",
                        self.item_name(&definition)
                    ));
                }
                GameEvent::ThreatSourceDisabled { at, .. } => {
                    self.terminal
                        .decor
                        .cells
                        .insert(at, crate::test_sector::Decor::ThreatCampDisabled);
                    self.push_log("CAMP HOSTILE NEUTRALISÉ · RENFORTS COUPÉS".to_owned());
                }
                GameEvent::PropertyTakeWitnessed {
                    witness,
                    definition,
                    quantity,
                    ..
                } => {
                    let witness = match self.game.active_worker_role(witness) {
                        Some(WorkerRole::Retriever) => "Le récupérateur",
                        Some(WorkerRole::Technician) => "Le technicien",
                        None => "Un témoin",
                    };
                    self.push_log(format!(
                        "{witness} vous voit prendre {} x{quantity}, matériel attribué.",
                        self.item_name(&definition)
                    ));
                }
                GameEvent::LocalAlertRaised {
                    source,
                    at,
                    duration_turns,
                    ..
                } => {
                    if self.game.player_visibility().is_visible(at) {
                        self.push_floating_message(
                            "ALERTE !",
                            at,
                            FloatingMessageTone::Alert,
                            visual_time,
                        );
                    }
                    self.visual_cues.play(
                        VisualCue::point(visual_cue_id("core:local_alert"), at),
                        replay_time.unwrap_or_else(get_time),
                    );
                    let source = match self.game.active_worker_role(source) {
                        Some(WorkerRole::Retriever) => "RÉCUPÉRATEUR",
                        Some(WorkerRole::Technician) => "TECHNICIEN",
                        None => "TÉMOIN",
                    };
                    self.push_log(format!(
                        "ALERTE LOCALE VISIBLE · {source} · {duration_turns} TOURS"
                    ));
                }
                GameEvent::ItemDropped {
                    definition,
                    quantity,
                    ..
                } => {
                    self.push_log(format!(
                        "{} déposé ×{quantity}",
                        self.item_name(&definition)
                    ));
                }
                GameEvent::IntegrityRestored { entity, amount } => {
                    if let Some(position) = self.game.actors().get(entity).map(Actor::position)
                        && (entity == self.game.player_id()
                            || self.game.player_visibility().is_visible(position))
                    {
                        self.push_floating_message(
                            format!("+{amount} PV"),
                            position,
                            FloatingMessageTone::Recovery,
                            visual_time,
                        );
                    }
                    self.push_log(format!("PV +{amount}"));
                }
                GameEvent::ExitReached { .. } => {
                    self.push_log("Sortie atteinte · secteur quitté.".to_owned());
                }
                GameEvent::PropagationResolved { origin, cells, .. } => {
                    let cells: Vec<_> = cells
                        .into_iter()
                        .filter(|cell| self.game.player_visibility().is_visible(cell.position))
                        .collect();
                    if cells.is_empty() {
                        continue;
                    }
                    if let Ok(cue) = VisualCue::from_propagation(
                        visual_cue_id("core:radial_damage"),
                        origin,
                        &cells,
                    ) {
                        self.visual_cues
                            .play(cue, replay_time.unwrap_or_else(get_time));
                    }
                    self.push_log("ONDE RADIALE PERÇUE".to_owned());
                }
                _ => {}
            }
        }
        if let Some(message) = player_attack_confirmation {
            self.push_log(message);
        }
        if let Some(notice) = level_up_notice
            && self.game.status() == RunStatus::Active
        {
            self.open_level_up_screen(notice);
        }
    }

    fn open_level_up_screen(&mut self, notice: LevelUpNotice) {
        self.attack_aim = None;
        self.attack_aim_technique = None;
        self.attack_aim_pointer = None;
        self.inventory_open = false;
        self.character_open = false;
        self.report_open = false;
        self.legend_open = false;
        self.skills_open = true;
        self.level_up_notice = Some(notice);
        self.clamp_skill_selection();
        self.skill_message = if notice.skill_points_awarded == 1 {
            "Montée de niveau : 1 nouveau point à dépenser.".to_owned()
        } else {
            format!(
                "Montée de niveau : {} nouveaux points à dépenser.",
                notice.skill_points_awarded
            )
        };
    }

    fn is_selected_target_at(&self, position: GridPos) -> bool {
        self.selected_target.is_some_and(|target| {
            self.game
                .actors()
                .get(target)
                .is_some_and(|actor| actor.position() == position)
        })
    }

    fn push_floating_message(
        &mut self,
        text: impl Into<String>,
        at: GridPos,
        tone: FloatingMessageTone,
        started_at: f64,
    ) {
        self.floating_messages
            .retain(|message| started_at - message.started_at < message.tone.lifetime());
        let lane = self
            .floating_messages
            .iter()
            .filter(|message| message.at == at)
            .count()
            .min(3) as u8;
        if self.floating_messages.len() >= FLOATING_MESSAGE_CAPACITY {
            self.floating_messages.remove(0);
        }
        self.floating_messages.push(FloatingMessage {
            text: text.into(),
            at,
            tone,
            started_at,
            lane,
        });
    }

    fn draw_floating_messages(&self, navigation_signal_visible: bool) {
        let now = get_time();
        let bounds = self.terminal_bounds();
        let scale = self.ui_scale();
        for message in &self.floating_messages {
            let elapsed = (now - message.started_at).max(0.0);
            let lifetime = message.tone.lifetime();
            if elapsed >= lifetime || !self.game.player_visibility().is_visible(message.at) {
                continue;
            }
            let Some(cell) = self.terminal.world_cell_rect(
                &self.game,
                bounds,
                self.graphics.active.world_cell_px,
                navigation_signal_visible,
                message.at,
            ) else {
                continue;
            };
            let progress = (elapsed / lifetime) as f32;
            let alpha = if progress <= 0.72 {
                1.0
            } else {
                ((1.0 - progress) / 0.28).clamp(0.0, 1.0)
            };
            let rise = if self.graphics.active.reduced_motion {
                0.0
            } else {
                progress * 18.0 * scale
            };
            let font_size = (message.tone.font_size() * scale).round().max(12.0);
            let measured = measure_text_bold(&message.text, font_size as u16);
            let x = (cell.x + (cell.w - measured.width) * 0.5).clamp(
                bounds.x + 5.0,
                (bounds.x + bounds.w - measured.width - 5.0).max(bounds.x),
            );
            let baseline = (cell.y - 5.0 * scale - f32::from(message.lane) * 21.0 * scale - rise)
                .max(bounds.y + font_size);
            let mut color = message.tone.color();
            color.a *= alpha;
            let shadow = Color::new(0.005, 0.015, 0.02, 0.9 * alpha);
            let outline = (1.4 * scale).clamp(1.0, 2.8);
            for (dx, dy) in [
                (-outline, 0.0),
                (outline, 0.0),
                (0.0, -outline),
                (0.0, outline),
            ] {
                draw_text_bold(&message.text, x + dx, baseline + dy, font_size, shadow);
            }
            draw_text_bold(&message.text, x, baseline, font_size, color);
        }
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
            self.push_log("Aucune cible visible.".to_owned());
            return;
        }

        let next_index = self
            .selected_target
            .and_then(|selected| targets.iter().position(|target| *target == selected))
            .map_or(0, |index| (index + 1) % targets.len());
        self.selected_target = targets.get(next_index).copied();
        if self.selected_target.is_some() {
            self.push_log("Cible verrouillée.".to_owned());
        }
    }

    fn select_weapon_slot(&mut self, slot: u8) {
        let Some(weapon) = self.game.equipped_player_weapon(slot) else {
            self.push_log(format!("L'emplacement d'arme {} est vide.", slot + 1));
            return;
        };
        let name = self.item_name(weapon.id());
        self.active_weapon_slot = slot;
        self.push_log(format!("ACTIVE [{}] {name}", slot + 1));
    }

    fn weapon_command(&mut self, pointer: Option<(f32, f32)>) -> Option<GameCommand> {
        let Some(weapon) = self.game.equipped_player_weapon(self.active_weapon_slot) else {
            self.push_log(format!(
                "L'emplacement d'arme actif {} est vide.",
                self.active_weapon_slot + 1
            ));
            return None;
        };
        if !matches!(weapon.attack().area(), AttackArea::Single) {
            self.begin_attack_aim(pointer);
            return None;
        }
        let target = self.ensure_visible_target()?;
        Some(GameCommand::Attack {
            slot: self.active_weapon_slot,
            target,
        })
    }

    fn begin_attack_aim(&mut self, pointer: Option<(f32, f32)>) {
        self.begin_attack_aim_at(None, pointer, None);
    }

    fn begin_pointer_attack_aim(&mut self, target: GridPos, pointer: Option<(f32, f32)>) -> bool {
        let Some(weapon) = self.game.equipped_player_weapon(self.active_weapon_slot) else {
            return false;
        };
        if matches!(weapon.attack().area(), AttackArea::Single) {
            return false;
        }
        self.begin_attack_aim_at(Some(target), pointer, None);
        self.attack_aim.is_some()
    }

    fn begin_technique_attack_aim(&mut self, technique: TechniqueId) {
        self.begin_attack_aim_at(None, None, Some(technique));
    }

    fn begin_attack_aim_at(
        &mut self,
        requested: Option<GridPos>,
        pointer: Option<(f32, f32)>,
        technique: Option<TechniqueId>,
    ) {
        if let Some(remaining_actions) = self
            .game
            .actors()
            .get(self.game.player_id())
            .and_then(Actor::recovery_remaining)
        {
            self.push_log(
                command_rejection_message(CommandRejection::OffensiveActionBlockedByRecovery {
                    remaining_actions: remaining_actions.get(),
                })
                .to_owned(),
            );
            return;
        }
        let Some(origin) = self.game.player_position() else {
            return;
        };
        let slot = self.active_weapon_slot;
        let selected = self.selected_target.and_then(|target| {
            self.game
                .actors()
                .get(target)
                .map(|actor| actor.position())
                .filter(|position| self.game.player_visibility().is_visible(*position))
        });
        let range = self
            .game
            .equipped_player_weapon(slot)
            .map_or(1, |weapon| weapon.attack().range());
        let requested = requested.filter(|position| {
            *position != origin && self.game.player_visibility().is_visible(*position)
        });
        let cursor = requested
            .or_else(|| {
                selected.filter(|position| {
                    technique.as_ref().map_or_else(
                        || self.game.player_attack_preview(slot, *position).is_ok(),
                        |technique| {
                            self.game
                                .player_weapon_technique_preview(technique, slot, *position)
                                .is_ok()
                        },
                    )
                })
            })
            .or_else(|| {
                (1..=range).rev().find_map(|distance| {
                    let mut position = origin;
                    for _ in 0..distance {
                        position = position.step(self.facing);
                    }
                    let valid = technique.as_ref().map_or_else(
                        || self.game.player_attack_preview(slot, position).is_ok(),
                        |technique| {
                            self.game
                                .player_weapon_technique_preview(technique, slot, position)
                                .is_ok()
                        },
                    );
                    (self.game.player_visibility().is_visible(position) && valid)
                        .then_some(position)
                })
            })
            .unwrap_or_else(|| origin.step(self.facing));
        self.attack_aim = Some(AttackAim { slot, cursor });
        self.attack_aim_technique = technique;
        self.attack_aim_pointer = pointer;
        self.push_log(if self.attack_aim_technique.is_some() {
            "VISÉE DE TECHNIQUE · DÉPLACEMENT/CURSEUR · ATTAQUER POUR CONFIRMER · ÉCHAP POUR ANNULER"
                .to_owned()
        } else {
            "VISÉE DE ZONE · DÉPLACEMENT/CURSEUR · ATTAQUER POUR CONFIRMER · ÉCHAP POUR ANNULER"
                .to_owned()
        });
    }

    fn update_attack_aim(&mut self, input: &InputFrame) {
        let Some(mut aim) = self.attack_aim else {
            return;
        };
        if input.pressed.contains(&controls::Binding::MouseRight) {
            self.attack_aim = None;
            self.attack_aim_technique = None;
            self.attack_aim_pointer = None;
            self.push_log("VISÉE ANNULÉE".to_owned());
            return;
        }
        if self.controls.pressed(Action::CycleTarget, input) {
            self.cycle_target();
            if let Some(position) = self
                .selected_target
                .and_then(|target| self.game.actors().get(target).map(|actor| actor.position()))
            {
                aim.cursor = position;
            }
            self.attack_aim = Some(aim);
            self.attack_aim_pointer = input.pointer;
            return;
        }

        let mut keyboard_moved = false;
        for (action, direction) in [
            (Action::MoveNorth, Direction::North),
            (Action::MoveEast, Direction::East),
            (Action::MoveSouth, Direction::South),
            (Action::MoveWest, Direction::West),
        ] {
            if self.controls.pressed(action, input) {
                let candidate = aim.cursor.step(direction);
                if self.game.player_visibility().is_visible(candidate) {
                    aim.cursor = candidate;
                    self.facing = direction;
                }
                keyboard_moved = true;
                break;
            }
        }

        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let pointer_moved = input.pointer.is_some() && input.pointer != self.attack_aim_pointer;
        let hovered = if pointer_moved || clicked {
            self.attack_pointer_cell(input)
                .filter(|position| self.game.player_visibility().is_visible(*position))
        } else {
            None
        };
        self.attack_aim_pointer = input.pointer;
        if !keyboard_moved && let Some(position) = hovered {
            aim.cursor = position;
        }
        self.attack_aim = Some(aim);

        let keyboard_confirmed = self.controls.pressed(Action::Attack, input) && !clicked;
        let mouse_confirmed = clicked && hovered.is_some();
        if !keyboard_confirmed && !mouse_confirmed {
            return;
        }
        if let Err(reason) = self.aimed_attack_preview(aim) {
            self.push_log(command_rejection_message(reason).to_owned());
            return;
        }
        let outcome = if let Some(technique) = self.attack_aim_technique.clone() {
            self.execute_command(GameCommand::UseTechniqueAt {
                technique,
                target: aim.cursor,
                weapon_slot: aim.slot,
            })
        } else {
            self.execute_command(GameCommand::AttackAt {
                slot: aim.slot,
                target: aim.cursor,
            })
        };
        match outcome {
            CommandOutcome::Rejected(reason) => {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                let continue_barrage =
                    self.attack_aim_technique.as_ref().is_some_and(|technique| {
                        self.game.player_active_weapon_barrage().is_some_and(
                            |(active, target, slot, _)| {
                                active == technique && target == aim.cursor && slot == aim.slot
                            },
                        )
                    });
                if !continue_barrage {
                    self.attack_aim = None;
                    self.attack_aim_technique = None;
                    self.attack_aim_pointer = None;
                }
            }
        }
        self.capture_events();
    }

    fn aimed_attack_preview(
        &self,
        aim: AttackAim,
    ) -> Result<project_rl::combat::AttackPreview, CommandRejection> {
        self.attack_aim_technique.as_ref().map_or_else(
            || self.game.player_attack_preview(aim.slot, aim.cursor),
            |technique| {
                self.game
                    .player_weapon_technique_preview(technique, aim.slot, aim.cursor)
            },
        )
    }

    fn aimed_attack_footprint(
        &self,
        aim: AttackAim,
    ) -> Result<project_rl::combat::AttackPreview, CommandRejection> {
        self.attack_aim_technique.as_ref().map_or_else(
            || self.game.player_attack_footprint(aim.slot, aim.cursor),
            |technique| {
                self.game
                    .player_weapon_technique_footprint(technique, aim.slot, aim.cursor)
            },
        )
    }

    fn attack_pointer_cell(&self, input: &InputFrame) -> Option<GridPos> {
        let pointer = input.pointer?;
        let scale = self.ui_scale();
        self.terminal.hit_test(
            &self.game,
            self.terminal_bounds(),
            self.graphics.active.world_cell_px,
            self.navigation_signal_summary().is_some(),
            (pointer.0 * scale, pointer.1 * scale),
        )
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
            self.push_log("Aucune cible visible.".to_owned());
            return None;
        };
        Some(target)
    }

    fn target_analysis_command(&mut self) -> Option<GameCommand> {
        self.technique_command(technique_id("core:rec_01")?)
    }

    fn trace_reading_command(&self) -> Option<GameCommand> {
        Some(GameCommand::UseTechnique {
            technique: technique_id("core:rec_02")?,
            targets: Vec::new(),
            weapon_slot: None,
        })
    }

    fn wall_analysis_command(&self) -> Option<GameCommand> {
        Some(GameCommand::UseTechnique {
            technique: technique_id("core:rec_04")?,
            targets: Vec::new(),
            weapon_slot: None,
        })
    }

    fn threat_analysis_command(&mut self) -> Option<GameCommand> {
        self.technique_command(technique_id("core:rec_05")?)
    }

    fn technique_command(&mut self, technique: TechniqueId) -> Option<GameCommand> {
        let action = self.game.rules().skills.technique(&technique)?.action()?;
        if action.is_drone_action() {
            return self.drone_technique_command(technique, action);
        }
        if action.is_engineering_action() {
            return self.engineering_technique_command(technique, action);
        }
        if action.is_intrusion_action() {
            return self.intrusion_technique_command(technique, action);
        }
        if action.is_electronic_warfare_action() {
            return self.electronic_warfare_technique_command(technique, action);
        }
        if matches!(action, TechniqueAction::WeaponComponentAttack { .. }) {
            let candidates = self
                .game
                .player_weapon_technique_targets(&technique, self.active_weapon_slot);
            let target = self
                .selected_target
                .filter(|target| candidates.contains(target))
                .or_else(|| candidates.first().copied());
            let Some(target) = target else {
                self.push_log("Aucune cible visible ne possède de composant identifié.".to_owned());
                return None;
            };
            self.selected_target = Some(target);
            let components = self.game.player_known_body_components(target);
            if components.is_empty() {
                return None;
            }
            if components.len() > 1 {
                self.component_selection = Some(ComponentSelection {
                    technique,
                    target,
                    components: components
                        .iter()
                        .map(|component| component.profile().id().clone())
                        .collect(),
                    selected: 0,
                    weapon_slot: self.active_weapon_slot,
                });
                self.push_log(
                    "TIR LOCALISÉ · CHOISIR UN COMPOSANT · CONFIRMER OU ÉCHAP".to_owned(),
                );
                return None;
            }
            let component = components[0];
            let component_id = component.profile().id().clone();
            let name = self
                .texts
                .resolve(DISPLAY_LOCALE, component.profile().name_key())
                .unwrap_or(component.profile().id().as_str())
                .to_owned();
            self.push_log(format!("Tir localisé : composant ciblé · {name}."));
            return Some(GameCommand::UseTechniqueOnComponent {
                technique,
                target,
                component: component_id,
                weapon_slot: self.active_weapon_slot,
            });
        }
        if matches!(
            action,
            TechniqueAction::WeaponAttack {
                melee_arc: Some(_),
                ..
            } | TechniqueAction::PrepareRangedOverwatch { .. }
                | TechniqueAction::WeaponBarrage { .. }
        ) || action.is_explosive_action()
            || action.is_movement_aim_action()
            || action.is_stealth_world_aim_action()
        {
            self.begin_technique_attack_aim(technique);
            return None;
        }
        let (maximum, weapon_slot) = match action {
            TechniqueAction::AnalyzeTarget { .. }
            | TechniqueAction::AnalyzeThreat { .. }
            | TechniqueAction::DiagnoseEnergy { .. } => (1, None),
            TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets, ..
            } => (usize::from(maximum_targets), None),
            TechniqueAction::WeaponAttack { .. } | TechniqueAction::AmbushAttack { .. } => {
                (1, Some(self.active_weapon_slot))
            }
            TechniqueAction::ChargeAttack { .. } | TechniqueAction::Breakthrough { .. } => {
                (1, Some(self.active_weapon_slot))
            }
            TechniqueAction::ExtractAlly { .. } => (1, None),
            TechniqueAction::WeaponVolley {
                maximum_targets, ..
            } => (usize::from(maximum_targets), Some(self.active_weapon_slot)),
            _ => (0, None),
        };
        let maximum_target_separation = match action {
            TechniqueAction::WeaponVolley {
                maximum_target_separation,
                ..
            } => maximum_target_separation,
            _ => None,
        };
        let targets = if maximum > 0 {
            let candidates = weapon_slot.map_or_else(
                || self.game.player_technique_targets(&technique),
                |slot| self.game.player_weapon_technique_targets(&technique, slot),
            );
            let Some(selected) = self
                .selected_target
                .filter(|id| candidates.contains(id))
                .or_else(|| candidates.first().copied())
            else {
                self.push_log("Aucune cible visible à portée de cette technique.".to_owned());
                return None;
            };
            self.selected_target = Some(selected);
            let selected_position = self.game.actors().get(selected).map(Actor::position);
            std::iter::once(selected)
                .chain(candidates.into_iter().filter(|id| {
                    if *id == selected {
                        return false;
                    }
                    maximum_target_separation.is_none_or(|maximum| {
                        let Some((left, right)) =
                            selected_position.zip(self.game.actors().get(*id).map(Actor::position))
                        else {
                            return false;
                        };
                        let delta_x = (i64::from(left.x) - i64::from(right.x)).abs();
                        let delta_y = (i64::from(left.y) - i64::from(right.y)).abs();
                        delta_x.max(delta_y) <= i64::from(maximum)
                    })
                }))
                .take(maximum)
                .collect()
        } else {
            Vec::new()
        };
        Some(GameCommand::UseTechnique {
            technique,
            targets,
            weapon_slot,
        })
    }

    /// Temporary terminal adapter for the fully parameterized drone commands.
    /// The headless command keeps every choice explicit; this test client uses
    /// facing, selected target and nearest known objects as fast defaults.
    fn drone_technique_command(
        &mut self,
        technique: TechniqueId,
        action: TechniqueAction,
    ) -> Option<GameCommand> {
        let player = self.game.player_id();
        let player_position = self.game.player_position()?;
        if matches!(action, TechniqueAction::ManifestDrone { .. }) {
            let position = player_position
                .cardinal_neighbors()
                .into_iter()
                .find(|position| {
                    self.game.map().is_walkable(*position)
                        && self.game.actors().entity_at(*position).is_none()
                });
            let Some(position) = position else {
                self.push_log("Aucune case adjacente libre pour manifester le drone.".to_owned());
                return None;
            };
            return Some(GameCommand::UseDroneTechnique {
                technique,
                directive: DroneDirective::Manifest { position },
            });
        }
        let drones = self
            .game
            .actors()
            .iter()
            .filter_map(|(entity, actor)| {
                actor
                    .drone()
                    .filter(|drone| drone.controller() == player)
                    .map(|_| entity)
            })
            .collect::<Vec<_>>();
        let Some(first) = drones.first().copied() else {
            self.push_log("Aucun drone physique contrôlé dans cette zone.".to_owned());
            return None;
        };
        let known_line = |origin: GridPos, maximum: u16| {
            let mut cells = Vec::new();
            let mut cursor = origin;
            for _ in 0..maximum {
                cursor = cursor.step(self.facing);
                if !self.game.map().is_walkable(cursor)
                    || !self.game.player_visibility().is_explored(cursor)
                {
                    break;
                }
                cells.push(cursor);
            }
            cells
        };
        let directive = match action {
            TechniqueAction::DroneEscort {
                minimum_distance,
                maximum_distance,
                ..
            } => DroneDirective::Escort {
                drone: first,
                distance: 2_u8.clamp(minimum_distance, maximum_distance),
            },
            TechniqueAction::DronePatrol {
                maximum_waypoints, ..
            } => {
                let origin = self.game.actors().get(first)?.position();
                let mut waypoints = known_line(origin, u16::from(maximum_waypoints));
                waypoints.truncate(usize::from(maximum_waypoints));
                if waypoints.is_empty() {
                    self.push_log(
                        "Patrouille : orientez-vous vers une route connue et praticable."
                            .to_owned(),
                    );
                    return None;
                }
                let autonomous = technique_id("core:drn_07")
                    .is_some_and(|id| self.game.player_skills().has_learned(&id));
                DroneDirective::Patrol {
                    drone: first,
                    waypoints,
                    blocked_response: PatrolBlockedResponse::Return,
                    autonomous,
                }
            }
            TechniqueAction::DroneMobileDecoy { link_range, .. } => {
                let destination = known_line(player_position, link_range)
                    .last()
                    .copied()
                    .unwrap_or_else(|| self.game.actors().get(first).unwrap().position());
                DroneDirective::MobileDecoy {
                    drone: first,
                    destination,
                }
            }
            TechniqueAction::DroneCollect { .. } => {
                let item = self
                    .game
                    .ground_items()
                    .iter()
                    .filter(|(_, item)| self.game.player_visibility().is_explored(item.position()))
                    .min_by_key(|(_, item)| {
                        (
                            item.position().x.abs_diff(player_position.x)
                                + item.position().y.abs_diff(player_position.y),
                            item.position(),
                        )
                    })
                    .map(|(item, _)| item);
                let Some(item) = item else {
                    self.push_log("Aucun objet connu à collecter dans cette zone.".to_owned());
                    return None;
                };
                DroneDirective::Collect { drone: first, item }
            }
            TechniqueAction::DroneCoordinateFire { maximum_drones, .. } => {
                let Some(target) = self.ensure_visible_target() else {
                    return None;
                };
                DroneDirective::CoordinateFire {
                    drones: drones
                        .iter()
                        .copied()
                        .take(usize::from(maximum_drones))
                        .collect(),
                    target,
                }
            }
            TechniqueAction::DroneInterpose { .. } => DroneDirective::Interpose {
                drone: first,
                ally: player,
            },
            TechniqueAction::DroneConditionalRoutine { .. } => DroneDirective::Conditional {
                drone: first,
                condition: DroneCondition::IntegrityBelowPercent(40),
                response: DroneConditionalResponse::Return,
            },
            TechniqueAction::DroneCoordinatedDeployment { maximum_drones, .. } => {
                let destinations = player_position
                    .cardinal_neighbors()
                    .into_iter()
                    .filter(|position| {
                        self.game.map().is_walkable(*position)
                            && self.game.player_visibility().is_explored(*position)
                            && self.game.actors().entity_at(*position).is_none()
                    })
                    .collect::<Vec<_>>();
                let assignments = drones
                    .iter()
                    .copied()
                    .zip(destinations)
                    .take(usize::from(maximum_drones))
                    .map(|(drone, destination)| DroneDeploymentAssignment {
                        drone,
                        destination,
                        role: DroneDeploymentRole::Guard,
                    })
                    .collect::<Vec<_>>();
                if assignments.is_empty() {
                    self.push_log("Aucune position connue libre pour le déploiement.".to_owned());
                    return None;
                }
                DroneDirective::Deploy { assignments }
            }
            TechniqueAction::DroneEmergencyReturn { maximum_drones, .. } => {
                DroneDirective::EmergencyReturn {
                    drones: drones
                        .iter()
                        .copied()
                        .take(usize::from(maximum_drones))
                        .collect(),
                    destination: player_position,
                }
            }
            _ => return None,
        };
        Some(GameCommand::UseDroneTechnique {
            technique,
            directive,
        })
    }

    /// Temporary terminal adapter. The engine command records every selected
    /// physical target; this client chooses a nearby sensible default until a
    /// dedicated engineering target panel replaces it.
    fn engineering_technique_command(
        &mut self,
        technique: TechniqueId,
        action: TechniqueAction,
    ) -> Option<GameCommand> {
        let player = self.game.player_id();
        let player_position = self.game.player_position()?;
        let distance = |position: GridPos| {
            player_position
                .x
                .abs_diff(position.x)
                .max(player_position.y.abs_diff(position.y))
        };
        let controlled_component = |damaged: bool| {
            self.game.actors().iter().find_map(|(entity, actor)| {
                let cooperative = entity == player
                    || actor
                        .drone()
                        .is_some_and(|drone| drone.controller() == player);
                if !cooperative || distance(actor.position()) > 1 {
                    return None;
                }
                actor
                    .body_components()
                    .find(|component| {
                        !damaged
                            || (!component.is_destroyed()
                                && component.durability() < component.maximum_durability())
                    })
                    .map(|component| (entity, component.profile().id().clone()))
            })
        };
        let active_module = || {
            let slot = self
                .game
                .rules()
                .player_weapon_slots
                .get(usize::from(self.active_weapon_slot))?;
            self.game.player_equipment().equipped(slot)
        };

        let directive = match action {
            TechniqueAction::RepairComponent { .. }
            | TechniqueAction::EmergencyRepairComponent { .. } => {
                let Some((target, component)) = controlled_component(true) else {
                    self.push_log(
                        "Aucun composant réparable sur vous ou un drone adjacent.".to_owned(),
                    );
                    return None;
                };
                EngineeringDirective::Component { target, component }
            }
            TechniqueAction::SalvageComponent => {
                let Some((wreck, component)) = self
                    .game
                    .wrecks()
                    .iter()
                    .filter(|wreck| distance(wreck.position()) <= 1)
                    .find_map(|wreck| {
                        wreck
                            .components()
                            .find(|component| !component.is_destroyed())
                            .map(|component| (wreck.id(), component.profile().id().clone()))
                    })
                else {
                    self.push_log("Aucune carcasse exploitable à portée.".to_owned());
                    return None;
                };
                EngineeringDirective::WreckComponent { wreck, component }
            }
            TechniqueAction::DiagnoseComponent { .. } => {
                let selected = self.selected_target.and_then(|target| {
                    let actor = self.game.actors().get(target)?;
                    (distance(actor.position()) <= 1
                        && self.game.player_visibility().is_visible(actor.position()))
                    .then(|| {
                        actor
                            .body_components()
                            .next()
                            .map(|component| (target, component.profile().id().clone()))
                    })
                    .flatten()
                });
                let fallback = self.game.actors().iter().find_map(|(target, actor)| {
                    (distance(actor.position()) <= 1
                        && self.game.player_visibility().is_visible(actor.position()))
                    .then(|| {
                        actor
                            .body_components()
                            .next()
                            .map(|component| (target, component.profile().id().clone()))
                    })
                    .flatten()
                });
                let Some((target, component)) = selected.or(fallback) else {
                    self.push_log(
                        "Aucun composant matériel accessible à diagnostiquer.".to_owned(),
                    );
                    return None;
                };
                EngineeringDirective::Component { target, component }
            }
            TechniqueAction::TuneModule { .. } => {
                let Some(module) = active_module() else {
                    self.push_log("Aucun module équipé dans le slot actif.".to_owned());
                    return None;
                };
                let tuning = match self
                    .game
                    .equipment_engineering_state(module)
                    .and_then(|state| state.tuning())
                {
                    Some(ModuleTuning::Economy) => ModuleTuning::Power,
                    _ => ModuleTuning::Economy,
                };
                EngineeringDirective::TuneModule { module, tuning }
            }
            TechniqueAction::OverclockModule { .. } => {
                let Some(module) = active_module() else {
                    self.push_log("Aucun module équipé dans le slot actif.".to_owned());
                    return None;
                };
                EngineeringDirective::OverclockModule { module }
            }
            TechniqueAction::BypassComponent { .. } => {
                let candidate = self.game.actors().iter().find_map(|(target, actor)| {
                    let cooperative = target == player
                        || actor
                            .drone()
                            .is_some_and(|drone| drone.controller() == player);
                    if !cooperative || distance(actor.position()) > 1 {
                        return None;
                    }
                    let receiver = actor
                        .body_components()
                        .find(|component| component.is_failed() && !component.is_destroyed())?
                        .profile()
                        .id()
                        .clone();
                    let donor = actor
                        .body_components()
                        .find(|component| {
                            !component.is_failed() && component.profile().id() != &receiver
                        })?
                        .profile()
                        .id()
                        .clone();
                    Some((target, receiver, donor))
                });
                let Some((target, receiver, donor)) = candidate else {
                    self.push_log(
                        "Aucun couple fonction dégradée / donneur opérationnel à portée."
                            .to_owned(),
                    );
                    return None;
                };
                EngineeringDirective::Bypass {
                    target,
                    receiver,
                    donor,
                }
            }
            TechniqueAction::ReconditionModule { .. } => {
                let Some(module) = active_module() else {
                    self.push_log("Aucun module équipé dans le slot actif.".to_owned());
                    return None;
                };
                EngineeringDirective::Module { module }
            }
            TechniqueAction::AssembleFieldBeacon { .. } => EngineeringDirective::AssembleAt {
                position: player_position.step(self.facing),
            },
            _ => return None,
        };
        Some(GameCommand::UseEngineeringTechnique {
            technique,
            directive,
        })
    }

    /// Temporary terminal adapter for digital targets. The headless command
    /// keeps the chosen interface, trace, command and subnet explicit; this
    /// client selects the nearest compatible known state as a fast default.
    fn intrusion_technique_command(
        &mut self,
        technique: TechniqueId,
        action: TechniqueAction,
    ) -> Option<GameCommand> {
        let range = match action {
            TechniqueAction::ProbeInterface { range, .. }
            | TechniqueAction::ForceElectronicLock { range, .. }
            | TechniqueAction::ExtractData { range, .. }
            | TechniqueAction::SpoofAuthorization { range, .. }
            | TechniqueAction::DivertDevice { range, .. }
            | TechniqueAction::SuspendDigitalRoutine { range, .. }
            | TechniqueAction::MaintainBackdoor { range, .. }
            | TechniqueAction::FalsifySecurityTrace { range, .. }
            | TechniqueAction::DivertSubnet { range, .. }
            | TechniqueAction::LockDeviceControl { range, .. } => range,
            _ => return None,
        };
        let candidates = self.game.player_digital_interface_positions(range);
        let first = || candidates.first().copied();
        let command_for = |position: GridPos| {
            if matches!(
                self.game.map().tile(position).map(|tile| tile.terrain),
                Some(Terrain::Door(_) | Terrain::ControlPanel { .. })
            ) {
                DeviceCommand::Open
            } else {
                DeviceCommand::Disable
            }
        };
        let directive = match action {
            TechniqueAction::ProbeInterface { .. }
            | TechniqueAction::ForceElectronicLock { .. } => {
                IntrusionDirective::Interface { position: first()? }
            }
            TechniqueAction::ExtractData { .. } => IntrusionDirective::Interface {
                position: candidates.iter().copied().find(|position| {
                    self.game.intrusion_state().has_right(
                        *position,
                        project_rl::intrusion::AccessRight::Read,
                        self.game.turn(),
                    )
                })?,
            },
            TechniqueAction::SpoofAuthorization { .. } => IntrusionDirective::Interface {
                position: candidates
                    .iter()
                    .copied()
                    .find(|position| self.game.intrusion_state().has_credential(*position))?,
            },
            TechniqueAction::DivertDevice { .. } => {
                let position = candidates.iter().copied().find(|position| {
                    self.game.intrusion_state().has_right(
                        *position,
                        project_rl::intrusion::AccessRight::Command,
                        self.game.turn(),
                    )
                })?;
                IntrusionDirective::Command {
                    position,
                    command: command_for(position),
                }
            }
            TechniqueAction::SuspendDigitalRoutine { .. } => {
                let position = candidates.iter().copied().find(|position| {
                    self.game.intrusion_state().has_right(
                        *position,
                        project_rl::intrusion::AccessRight::Command,
                        self.game.turn(),
                    )
                })?;
                IntrusionDirective::Routine {
                    position,
                    routine: DigitalRoutine::AutomaticResponse,
                }
            }
            TechniqueAction::MaintainBackdoor { .. } => IntrusionDirective::Interface {
                position: candidates
                    .iter()
                    .copied()
                    .find(|position| self.game.intrusion_state().has_backdoor(*position))
                    .or_else(|| {
                        candidates.iter().copied().find(|position| {
                            self.game.intrusion_state().session(*position).is_some()
                        })
                    })?,
            },
            TechniqueAction::FalsifySecurityTrace { .. } => {
                let trace = self.game.intrusion_state().traces().find(|trace| {
                    !trace.was_audited()
                        && !trace.is_falsified()
                        && candidates.contains(&trace.source())
                        && self.game.intrusion_state().has_right(
                            trace.source(),
                            project_rl::intrusion::AccessRight::ModifyRegister,
                            self.game.turn(),
                        )
                })?;
                IntrusionDirective::Trace { trace: trace.id() }
            }
            TechniqueAction::DivertSubnet {
                maximum_devices, ..
            } => {
                let positions = candidates
                    .iter()
                    .copied()
                    .filter(|position| {
                        self.game.intrusion_state().has_right(
                            *position,
                            project_rl::intrusion::AccessRight::Command,
                            self.game.turn(),
                        )
                    })
                    .take(usize::from(maximum_devices))
                    .collect::<Vec<_>>();
                let command = command_for(*positions.first()?);
                let positions = positions
                    .into_iter()
                    .filter(|position| command_for(*position) == command)
                    .collect();
                IntrusionDirective::Subnet { positions, command }
            }
            TechniqueAction::LockDeviceControl { .. } => IntrusionDirective::Interface {
                position: candidates
                    .iter()
                    .copied()
                    .find(|position| self.game.intrusion_state().control(*position).is_some())?,
            },
            _ => return None,
        };
        Some(GameCommand::UseIntrusionTechnique {
            technique,
            directive,
        })
    }

    /// Temporary terminal adapter. Every target and channel is still explicit
    /// in the headless command so a later graphical selector can replace these
    /// deterministic nearest-target defaults without changing simulation rules.
    fn electronic_warfare_technique_command(
        &mut self,
        technique: TechniqueId,
        action: TechniqueAction,
    ) -> Option<GameCommand> {
        let player = self.game.player_id();
        let origin = self.game.player_position()?;
        let distance = |position: GridPos| {
            origin
                .x
                .abs_diff(position.x)
                .max(origin.y.abs_diff(position.y))
        };
        let mut compatible = self
            .game
            .actors()
            .iter()
            .filter(|(entity, actor)| {
                *entity != player
                    && actor.electronic_system().is_some()
                    && self.game.player_visibility().is_visible(actor.position())
            })
            .map(|(entity, actor)| (entity, actor.position()))
            .collect::<Vec<_>>();
        compatible.sort_by_key(|(entity, position)| (distance(*position), *entity));
        let selected = |range: u16| {
            self.selected_target
                .filter(|target| {
                    compatible.iter().any(|(entity, position)| {
                        entity == target && distance(*position) <= u32::from(range)
                    })
                })
                .or_else(|| {
                    compatible
                        .iter()
                        .find(|(_, position)| distance(*position) <= u32::from(range))
                        .map(|(entity, _)| *entity)
                })
        };
        let directive = match action {
            TechniqueAction::ElectronicPulse { directional, .. } => ElectronicDirective::Pulse {
                direction: directional.then_some(self.facing),
            },
            TechniqueAction::ImplantOverheat { range, .. }
            | TechniqueAction::ImplantInfection { range, .. }
            | TechniqueAction::ImplantImplosion { range, .. } => {
                let target = selected(range)?;
                self.selected_target = Some(target);
                ElectronicDirective::Target { target }
            }
            TechniqueAction::MaintainJamming { .. } => {
                self.push_log(
                    "Brouillage terminal : canal de liaison de contrôle sélectionné.".to_owned(),
                );
                ElectronicDirective::Jam {
                    channel: ElectronicChannel::ControlLink,
                }
            }
            TechniqueAction::PurgeHostileProgram { range, .. } => {
                let candidate = self
                    .game
                    .electronic_warfare_state()
                    .programs()
                    .filter_map(|program| {
                        let target = program.target();
                        let position = self.game.actors().get(target)?.position();
                        (distance(position) <= u32::from(range)
                            && self.game.player_visibility().is_visible(position))
                        .then_some((distance(position), target, program.id()))
                    })
                    .min();
                let (_, target, program) = candidate?;
                ElectronicDirective::Purge { target, program }
            }
            TechniqueAction::ElectronicCascade {
                range,
                jump_range,
                maximum_targets,
                ..
            } => {
                let first = selected(range)?;
                let mut targets = vec![first];
                while targets.len() < usize::from(maximum_targets) {
                    let previous = self.game.actors().get(*targets.last()?)?.position();
                    let next = compatible
                        .iter()
                        .filter(|(entity, position)| {
                            !targets.contains(entity)
                                && previous
                                    .x
                                    .abs_diff(position.x)
                                    .max(previous.y.abs_diff(position.y))
                                    <= u32::from(jump_range)
                        })
                        .min_by_key(|(entity, position)| {
                            (
                                previous
                                    .x
                                    .abs_diff(position.x)
                                    .max(previous.y.abs_diff(position.y)),
                                *entity,
                            )
                        })
                        .map(|(entity, _)| *entity);
                    let Some(next) = next else {
                        break;
                    };
                    targets.push(next);
                }
                self.selected_target = Some(first);
                ElectronicDirective::Cascade { targets }
            }
            TechniqueAction::DeploySaturationBeacon {
                manual_activation, ..
            } => {
                if manual_activation
                    && let Some(beacon) = self
                        .game
                        .electronic_warfare_state()
                        .beacons()
                        .filter(|beacon| {
                            beacon.owner == player && !beacon.active && beacon.remaining_phases > 0
                        })
                        .map(|beacon| beacon.entity)
                        .min()
                {
                    ElectronicDirective::ActivateBeacon { beacon }
                } else {
                    ElectronicDirective::DeployBeacon {
                        position: origin.step(self.facing),
                    }
                }
            }
            _ => return None,
        };
        Some(GameCommand::UseElectronicWarfareTechnique {
            technique,
            directive,
        })
    }

    fn component_selection_layout(
        viewport: (f32, f32),
        count: usize,
    ) -> (Rect, Vec<Rect>, Rect, Rect) {
        let panel_width = viewport.0.min(620.0).max(420.0);
        let row_height = 42.0;
        let panel_height = (122.0 + count as f32 * row_height).min(viewport.1 - 48.0);
        let panel = Rect::new(
            (viewport.0 - panel_width) * 0.5,
            (viewport.1 - panel_height) * 0.5,
            panel_width,
            panel_height,
        );
        let rows = (0..count)
            .map(|index| {
                Rect::new(
                    panel.x + 18.0,
                    panel.y + 54.0 + index as f32 * row_height,
                    panel.w - 36.0,
                    35.0,
                )
            })
            .collect();
        let cancel = Rect::new(panel.x + 18.0, panel.y + panel.h - 43.0, 120.0, 29.0);
        let confirm = Rect::new(
            panel.x + panel.w - 148.0,
            panel.y + panel.h - 43.0,
            130.0,
            29.0,
        );
        (panel, rows, cancel, confirm)
    }

    fn update_component_selection(&mut self, input: &InputFrame) {
        let Some(mut selection) = self.component_selection.clone() else {
            return;
        };
        if selection.components.is_empty() {
            self.component_selection = None;
            return;
        }
        let viewport = input.viewport.unwrap_or((1280.0, 800.0));
        let first_visible = selection.selected.saturating_sub(7);
        let visible_count = (selection.components.len() - first_visible).min(8);
        let (_, rows, cancel, confirm) = Self::component_selection_layout(viewport, visible_count);
        let hovered_row = input.pointer.and_then(|point| {
            rows.iter()
                .position(|row| row.contains(point.into()))
                .map(|index| first_visible + index)
        });
        let cancel_hovered = input
            .pointer
            .is_some_and(|point| cancel.contains(point.into()));
        let confirm_hovered = input
            .pointer
            .is_some_and(|point| confirm.contains(point.into()));
        self.menu_focus.hovered = hovered_row
            .or_else(|| cancel_hovered.then_some(selection.components.len()))
            .or_else(|| confirm_hovered.then_some(selection.components.len() + 1));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);

        if input.pressed.contains(&controls::Binding::MouseRight) || clicked && cancel_hovered {
            self.component_selection = None;
            self.push_log("SÉLECTION DE COMPOSANT ANNULÉE".to_owned());
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            selection.selected = selection.selected.saturating_sub(1);
            self.component_selection = Some(selection);
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            selection.selected =
                (selection.selected + 1).min(selection.components.len().saturating_sub(1));
            self.component_selection = Some(selection);
            return;
        }
        if clicked && let Some(index) = hovered_row {
            selection.selected = index;
            self.component_selection = Some(selection);
            return;
        }
        if !self.controls.pressed(Action::Attack, input)
            && !self.controls.pressed(Action::Use, input)
            && !(clicked && confirm_hovered)
        {
            return;
        }

        let component = selection.components[selection.selected].clone();
        let outcome = self.execute_command(GameCommand::UseTechniqueOnComponent {
            technique: selection.technique.clone(),
            target: selection.target,
            component,
            weapon_slot: selection.weapon_slot,
        });
        match outcome {
            CommandOutcome::Rejected(reason) => {
                self.push_log(command_rejection_message(reason).to_owned());
                self.component_selection = Some(selection);
            }
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.component_selection = None;
            }
        }
        self.capture_events();
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
                    && self.game.active_worker_role(entity).is_none()
                    && self.game.player_visibility().is_visible(actor.position()))
                .then_some((grid_distance(player_position, actor.position()), entity))
            })
            .collect();
        targets.sort_unstable();
        targets.into_iter().map(|(_, entity)| entity).collect()
    }

    #[cfg(test)]
    fn combat_channels_label(&self) -> String {
        self.game
            .rules()
            .player_weapon_slots
            .iter()
            .enumerate()
            .take(3)
            .map(|(slot, _)| {
                let weapon = self
                    .game
                    .equipped_player_weapon(slot as u8)
                    .map(|weapon| self.item_name(weapon.id()))
                    .unwrap_or_else(|| "VIDE".to_owned());
                format!(
                    "{}[{}] {weapon}",
                    if slot as u8 == self.active_weapon_slot {
                        ">"
                    } else {
                        ""
                    },
                    slot + 1
                )
            })
            .collect::<Vec<_>>()
            .join("  ·  ")
    }

    fn draw_header(&self) {
        let theme = UiTheme;
        let player_pv = self
            .game
            .actors()
            .get(self.game.player_id())
            .map(|actor| format!("{}/{}", actor.integrity(), actor.maximum_integrity()))
            .unwrap_or_else(|| "0/--".to_owned());
        let recovery = self
            .game
            .actors()
            .get(self.game.player_id())
            .and_then(Actor::recovery_remaining)
            .map_or_else(String::new, |remaining| {
                format!(" · Récupération {} tour(s)", remaining.get())
            });
        let system_resources = match (self.game.player_bandwidth(), self.game.player_heat()) {
            (Some(bandwidth), Some(heat)) => format!(
                " · Bande passante {}/{} · Chaleur {}",
                bandwidth.available(),
                bandwidth.capacity(),
                heat.current()
            ),
            _ => String::new(),
        };
        let target = self.attack_aim.map_or_else(
            || {
                self.selected_target.map_or_else(
                    || "Aucune cible".to_owned(),
                    |entity| {
                        let label = match self.hostile_glyph(entity) {
                            'd' => "TRAQUEUR",
                            't' => "SENTINELLE",
                            'r' => "TIRAILLEUR",
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
                                    status_display_name(&status.definition),
                                    status.stacks
                                )
                            })
                            .unwrap_or_default();
                        format!("{label}{status}")
                    },
                )
            },
            |aim| {
                let result = self.aimed_attack_preview(aim);
                let footprint = self.aimed_attack_footprint(aim);
                let affected = footprint.as_ref().map_or(0, |preview| {
                    preview
                        .cells()
                        .iter()
                        .filter(|cell| {
                            !self.game.map().is_protected(cell.position)
                                && self
                                    .game
                                    .actors()
                                    .entity_at(cell.position)
                                    .is_some_and(|entity| entity != self.game.player_id())
                        })
                        .count()
                });
                let status = match result.as_ref() {
                    Ok(_) => "VALIDE".to_owned(),
                    Err(reason) => {
                        format!("INVALIDE : {}", attack_preview_rejection_label(reason))
                    }
                };
                format!(
                    "VISÉE · {} · {} CIBLE{}",
                    status,
                    affected,
                    if affected > 1 { "S" } else { "" }
                )
            },
        );
        let progression = self.game.player_progression();
        let active_weapon = self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .map(|weapon| {
                let name = self.item_name(weapon.id());
                self.game
                    .player_weapon_ammunition(weapon.id())
                    .map_or(name.clone(), |(remaining, capacity)| {
                        format!("{name} · MUN. {remaining}/{capacity}")
                    })
            })
            .unwrap_or_else(|| "Vide".to_owned());
        if let Some((local_alerts, security_alarms, remaining_turns)) = self.visible_alert_summary()
        {
            let pulse = if self.graphics.active.reduced_motion {
                0.0
            } else {
                ((get_time() * 4.0).sin() * 0.5 + 0.5) as f32
            };
            draw_rectangle(
                12.0,
                7.0,
                self.ui_width() - 24.0,
                58.0,
                Color::from_rgba(104, 31, 17, 255),
            );
            draw_rectangle_lines(
                12.0,
                7.0,
                self.ui_width() - 24.0,
                58.0,
                if self.graphics.active.high_contrast {
                    3.0
                } else {
                    2.0 + pulse
                },
                Color::new(1.0, 0.48 + pulse * 0.18, 0.17, 1.0),
            );
            let (warning, marker) = match (local_alerts, security_alarms) {
                (local, 0) => (format!("ALERTE LOCALE · {local} source(s) visible(s)"), "!"),
                (0, security) => (
                    format!("ALARME RÉSEAU · {security} système(s) actif(s)"),
                    "#",
                ),
                (local, security) => (
                    format!("ALERTE LOCALE + RÉSEAU · {local} témoin(s) · {security} système(s)"),
                    "!#",
                ),
            };
            draw_text_bold(
                marker,
                24.0,
                45.0,
                30.0,
                Color::from_rgba(255, 225, 183, 255),
            );
            draw_text_bold(
                &warning,
                66.0,
                32.0,
                19.0,
                Color::from_rgba(255, 237, 199, 255),
            );
            draw_text(
                format!(
                    "Encore {remaining_turns} tour(s) · Intégrité {player_pv} · Énergie {}/{}{system_resources}{recovery}",
                    self.game.player_energy().available(),
                    self.game.player_energy().capacity(),
                ),
                66.0,
                53.0,
                15.0,
                Color::from_rgba(255, 211, 163, 255),
            );
        } else {
            let x = 12.0;
            let gap = 6.0;
            let available = self.ui_width() - x * 2.0 - gap * 3.0;
            let first = available * 0.2;
            let second = available * 0.3;
            let third = available * 0.25;
            let fourth = available - first - second - third;
            let cards = [
                (
                    Rect::new(x, 7.0, first, 58.0),
                    format!("NIVEAU {}", progression.level()),
                    skill_points_hud_label(progression.unspent_skill_points()),
                    theme.accent(),
                ),
                (
                    Rect::new(x + first + gap, 7.0, second, 58.0),
                    "ÉTAT".to_owned(),
                    format!(
                        "PV {player_pv} · Énergie {}/{}{system_resources}{recovery}",
                        self.game.player_energy().available(),
                        self.game.player_energy().capacity()
                    ),
                    theme.success(),
                ),
                (
                    Rect::new(x + first + second + gap * 2.0, 7.0, third, 58.0),
                    format!("ARME · EMPLACEMENT {}", self.active_weapon_slot + 1),
                    active_weapon,
                    theme.focus(),
                ),
                (
                    Rect::new(x + first + second + third + gap * 3.0, 7.0, fourth, 58.0),
                    "CIBLE".to_owned(),
                    target,
                    theme.accent(),
                ),
            ];
            for (rect, label, value, accent) in cards {
                draw_hud_card(
                    rect,
                    &label,
                    &value,
                    accent,
                    self.graphics.active.high_contrast,
                );
            }
        }
    }

    fn draw_companion_bar(&self) {
        let companions = self.game.player_controlled_companions();
        let Some(entity) = companions.first().copied() else {
            return;
        };
        let Some(actor) = self.game.actors().get(entity) else {
            return;
        };
        let Some(drone) = actor.drone() else {
            return;
        };
        let layout = CompanionBarLayout::new(self.ui_width(), self.ui_height());
        UiTheme.card(layout.panel, false);
        let linked = companions
            .iter()
            .all(|entity| self.game.player_companion_is_linked(*entity));
        let active_behavior = match drone.order() {
            DroneOrder::Companion { behavior, .. } => Some(*behavior),
            DroneOrder::Escort { .. } => Some(CompanionBehavior::Follow),
            _ => None,
        };
        let title = if companions.len() == 1 {
            "ALLIÉ · DRONE".to_owned()
        } else {
            format!("ALLIÉS · {} UNITÉS", companions.len())
        };
        draw_text_bold(
            &title,
            layout.panel.x + 12.0,
            layout.panel.y + 21.0,
            15.0,
            if linked {
                UiTheme.success()
            } else {
                UiTheme.danger()
            },
        );
        draw_text(
            format!(
                "PV {}/{}  ·  BAT {}/{}  ·  {}",
                actor.integrity(),
                actor.maximum_integrity(),
                drone.energy().available(),
                drone.energy().capacity(),
                if linked { "LIAISON OK" } else { "HORS LIAISON" }
            ),
            layout.panel.x + 12.0,
            layout.panel.y + 42.0,
            14.0,
            UiTheme.text(),
        );
        for (index, behavior) in CompanionBehavior::ALL.into_iter().enumerate() {
            UiTheme.button(
                layout.behavior_buttons[index],
                companion_behavior_label(behavior),
                self.menu_focus.hovered == Some(COMPANION_ACTION_FOCUS_BASE + index),
                active_behavior == Some(behavior),
                linked,
                ButtonTone::Secondary,
            );
        }
    }

    fn draw_footer(&self) {
        if let Some(aim) = self.attack_aim {
            let footprint = self.aimed_attack_footprint(aim);
            let affected = footprint.as_ref().map_or(0, |preview| {
                preview
                    .cells()
                    .iter()
                    .filter(|cell| {
                        self.game
                            .actors()
                            .entity_at(cell.position)
                            .is_some_and(|entity| entity != self.game.player_id())
                    })
                    .count()
            });
            let (state, state_color, confirmation) = match self.aimed_attack_preview(aim) {
                Ok(_) => (
                    "ZONE VALIDE",
                    UiTheme.success(),
                    format!(
                        "{} ou clic gauche · CONFIRMER",
                        self.controls.label(Action::Attack)
                    ),
                ),
                Err(reason) => (
                    "ZONE INVALIDE",
                    UiTheme.danger(),
                    attack_preview_rejection_label(&reason).to_owned(),
                ),
            };
            let rect = Rect::new(12.0, self.ui_height() - 94.0, self.ui_width() - 24.0, 82.0);
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, UiTheme.surface());
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, state_color);
            draw_text_bold(state, rect.x + 14.0, rect.y + 26.0, 18.0, state_color);
            draw_text(
                format!(
                    "{} case(s) couvertes · {} cible(s)",
                    footprint.as_ref().map_or(0, |area| area.cells().len()),
                    affected
                ),
                rect.x + 14.0,
                rect.y + 50.0,
                16.0,
                UiTheme.text(),
            );
            draw_text_bold(
                &confirmation,
                rect.x + rect.w * 0.42,
                rect.y + 29.0,
                17.0,
                state_color,
            );
            draw_text(
                format!(
                    "Déplacer : {} {} {} {} / souris · Annuler : Échap ou clic droit",
                    self.controls.label(Action::MoveNorth),
                    self.controls.label(Action::MoveWest),
                    self.controls.label(Action::MoveSouth),
                    self.controls.label(Action::MoveEast),
                ),
                rect.x + rect.w * 0.42,
                rect.y + 54.0,
                15.0,
                UiTheme.muted(),
            );
            return;
        }
        if let Some(preparation) = self.game.player_technique_preparation() {
            let technique = self.technique_name(preparation.technique());
            let button = preparation_continue_rect(self.ui_width(), self.ui_height());
            let text_width = (button.x - 38.0).max(180.0);
            draw_wrapped_text(
                &format!("PRÉPARATION · {technique}"),
                20.0,
                self.ui_height() - 86.0,
                text_width,
                1,
                17,
                UiTheme.focus(),
            );
            draw_wrapped_text(
                &format!(
                    "{} UT restante(s) · se déplacer ou attaquer annule",
                    preparation.remaining_steps().get()
                ),
                20.0,
                self.ui_height() - 64.0,
                text_width,
                1,
                14,
                UiTheme.muted(),
            );
            UiTheme.button(
                button,
                &format!(
                    "CONTINUER · {}",
                    self.controls.label(Action::Wait).to_uppercase()
                ),
                self.menu_focus.hovered == Some(WAIT_ACTION_FOCUS),
                false,
                true,
                ButtonTone::Primary,
            );
        } else {
            let wait_binding = self.controls.label(Action::Wait);
            let hint_y = self.ui_height() - 94.0;
            let mut hint_x = draw_control_hint(20.0, hint_y, &wait_binding, "Attendre");
            for (binding, label) in [
                (self.controls.label(Action::Interact), "Interagir"),
                (self.controls.label(Action::Attack), "Attaquer"),
                (self.controls.label(Action::QuickTechniques), "Techniques"),
                (self.controls.label(Action::Inventory), "Inventaire"),
                (self.controls.label(Action::Legend), "Aide"),
                ("ÉCHAP".to_owned(), "Menu"),
            ] {
                hint_x = draw_control_hint(hint_x, hint_y, &binding, label);
            }
        }

        for (index, message) in self.log.iter().rev().take(2).rev().enumerate() {
            draw_text(
                message,
                20.0,
                self.ui_height() - 42.0 + index as f32 * 20.0,
                16.0,
                UiTheme.muted(),
            );
        }
    }

    fn draw_end_message(&self) {
        let message = if self.game.status() == RunStatus::PlayerDestroyed {
            Some(format!(
                "NOYAU DÉTRUIT — {} : RECOMMENCER",
                self.controls.label(Action::Restart)
            ))
        } else if self.game.status() == RunStatus::Escaped {
            Some(format!(
                "SORTIE ATTEINTE — {} : NOUVELLE PARTIE",
                self.controls.label(Action::Restart)
            ))
        } else {
            None
        };

        if let Some(message) = message {
            let metrics = measure_text(&message, None, 28, 1.0);
            let x = (self.ui_width() - metrics.width) * 0.5;
            let y = self.ui_height() * 0.5;
            draw_rectangle(
                x - 18.0,
                y - 34.0,
                metrics.width + 36.0,
                52.0,
                Color::from_rgba(4, 8, 12, 235),
            );
            draw_text(&message, x, y, 28.0, Color::from_rgba(255, 211, 92, 255));
        }
    }

    fn clamp_inventory_selection(&mut self) {
        let count = self.inventory_entries().len();
        self.inventory_selection = self.inventory_selection.min(count.saturating_sub(1));
    }

    fn skill_disciplines(&self) -> Vec<DisciplineId> {
        self.game
            .rules()
            .skills
            .disciplines()
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn discipline_name(&self, id: &DisciplineId) -> String {
        self.game
            .rules()
            .skills
            .discipline(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn technique_name(&self, id: &TechniqueId) -> String {
        self.game
            .rules()
            .skills
            .technique(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn item_name(&self, id: &ItemId) -> String {
        self.game
            .rules()
            .items
            .get(id)
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .or_else(|| {
                self.game.rules().weapons.get(id).and_then(|definition| {
                    self.texts.resolve(DISPLAY_LOCALE, definition.name_key())
                })
            })
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn body_component_name(&self, id: &BodyComponentId) -> String {
        self.texts
            .resolve(DISPLAY_LOCALE, &format!("component.{}.name", id.name()))
            .map(str::to_owned)
            .unwrap_or_else(|| "Composant non identifié".to_owned())
    }

    fn equipment_slot_name(&self, id: &ContentId) -> String {
        self.texts
            .resolve(DISPLAY_LOCALE, &format!("equipment_slot.{id}"))
            .map(str::to_owned)
            .unwrap_or_else(|| display_content_name(id))
    }

    fn inventory_category(&self, entry: &InventoryEntry) -> InventoryFilter {
        if self.game.rules().weapons.get(entry.item()).is_some() {
            InventoryFilter::Weapons
        } else {
            match self
                .game
                .rules()
                .items
                .get(entry.item())
                .map(|item| item.kind())
            {
                Some(ItemKind::Armor) => InventoryFilter::Armor,
                Some(ItemKind::Consumable) => InventoryFilter::Consumables,
                Some(ItemKind::Material) => InventoryFilter::Materials,
                None => InventoryFilter::All,
            }
        }
    }

    fn inventory_glyph(&self, entry: &InventoryEntry) -> InventoryGlyph {
        if let Some(weapon) = self.game.rules().weapons.get(entry.item()) {
            let attack = weapon.attack();
            if attack.damage().contains(DamageType::Thermal)
                || matches!(attack.area(), AttackArea::Cone(_))
            {
                InventoryGlyph::FlameProjector
            } else if attack.range() <= 1 {
                InventoryGlyph::Blade
            } else {
                InventoryGlyph::RangedWeapon
            }
        } else {
            match self
                .game
                .rules()
                .items
                .get(entry.item())
                .map(|item| item.kind())
            {
                Some(ItemKind::Armor) => InventoryGlyph::Armor,
                Some(ItemKind::Consumable) => InventoryGlyph::Consumable,
                Some(ItemKind::Material) => InventoryGlyph::Material,
                None => InventoryGlyph::Unknown,
            }
        }
    }

    fn inventory_entries(&self) -> Vec<&InventoryEntry> {
        let mut entries = self
            .game
            .player_inventory()
            .iter()
            .filter(|entry| {
                self.inventory_filter == InventoryFilter::All
                    || self.inventory_category(entry) == self.inventory_filter
            })
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| {
            let left_name = self.item_name(left.item());
            let right_name = self.item_name(right.item());
            match self.inventory_sort {
                InventorySort::Name => left_name.cmp(&right_name),
                InventorySort::Type => inventory_category_order(self.inventory_category(left))
                    .cmp(&inventory_category_order(self.inventory_category(right)))
                    .then_with(|| left_name.cmp(&right_name)),
            }
        });
        entries
    }

    fn selected_skill_techniques(&self) -> Vec<TechniqueId> {
        let disciplines = self.skill_disciplines();
        let Some(discipline) = disciplines.get(self.skill_discipline_selection) else {
            return Vec::new();
        };
        self.game
            .rules()
            .skills
            .techniques()
            .filter(|(_, technique)| technique.discipline() == discipline)
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn clamp_skill_selection(&mut self) {
        let discipline_count = self.game.rules().skills.disciplines().count();
        self.skill_discipline_selection = self
            .skill_discipline_selection
            .min(discipline_count.saturating_sub(1));
        let technique_count = self.selected_skill_techniques().len();
        self.skill_technique_selection = self
            .skill_technique_selection
            .min(technique_count.saturating_sub(1));
    }

    fn active_learned_techniques(&self) -> Vec<TechniqueId> {
        self.game
            .rules()
            .skills
            .techniques()
            .filter(|(id, definition)| {
                definition.action().is_some() && self.game.player_skills().has_learned(id)
            })
            .map(|(id, _)| id.clone())
            .collect()
    }

    fn open_technique_menu(&mut self) {
        let techniques = self.active_learned_techniques();
        if techniques.is_empty() {
            self.push_log("Aucune technique active apprise.".to_owned());
            return;
        }
        self.technique_menu_selection = self
            .technique_menu_selection
            .min(techniques.len().saturating_sub(1));
        self.technique_menu_message.clear();
        self.technique_menu_open = true;
        self.menu_focus.reset();
    }

    fn close_technique_menu(&mut self) {
        self.technique_menu_open = false;
        self.technique_menu_message.clear();
        self.menu_focus.reset();
    }

    fn use_quick_technique(&mut self, techniques: &[TechniqueId]) {
        let Some(technique) = techniques.get(self.technique_menu_selection).cloned() else {
            self.close_technique_menu();
            return;
        };
        let name = self.technique_name(&technique);
        if let Some(command) = self.technique_command(technique) {
            match self.execute_command(command) {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    self.close_technique_menu();
                }
                CommandOutcome::Rejected(reason) => {
                    self.technique_menu_message = command_rejection_message(reason).to_owned();
                }
            }
            self.capture_events();
        } else if self.attack_aim.is_some() || self.component_selection.is_some() {
            self.close_technique_menu();
        } else {
            self.technique_menu_message =
                format!("{name} : conditions non réunies. Consultez le journal.");
        }
    }

    fn update_technique_menu(&mut self, input: &InputFrame) {
        let techniques = self.active_learned_techniques();
        if techniques.is_empty() {
            self.close_technique_menu();
            return;
        }
        self.technique_menu_selection = self
            .technique_menu_selection
            .min(techniques.len().saturating_sub(1));
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = TechniqueQuickMenuLayout::new(
            width,
            height,
            self.technique_menu_selection,
            techniques.len(),
        );
        let hovered_row = input.pointer.and_then(|point| {
            layout
                .rows
                .iter()
                .find(|(_, row)| row.contains(point.into()))
                .map(|(index, _)| *index)
        });
        let hovered_action = input.pointer.and_then(|point| {
            layout
                .actions
                .iter()
                .position(|button| button.contains(point.into()))
        });
        self.menu_focus.hovered =
            hovered_row.or_else(|| hovered_action.map(|index| techniques.len() + index));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);

        if clicked && hovered_action == Some(1) {
            self.close_technique_menu();
            return;
        }
        if input.wheel_y != 0.0
            && input
                .pointer
                .is_some_and(|point| layout.panel.contains(point.into()))
        {
            let steps = wheel_steps(input.wheel_y);
            self.technique_menu_selection = if input.wheel_y > 0.0 {
                self.technique_menu_selection.saturating_sub(steps)
            } else {
                self.technique_menu_selection
                    .saturating_add(steps)
                    .min(techniques.len() - 1)
            };
            self.technique_menu_message.clear();
            return;
        }
        if clicked && let Some(index) = hovered_row {
            self.technique_menu_selection = index;
            self.technique_menu_message.clear();
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            self.technique_menu_selection = self.technique_menu_selection.saturating_sub(1);
            self.technique_menu_message.clear();
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.technique_menu_selection =
                (self.technique_menu_selection + 1).min(techniques.len() - 1);
            self.technique_menu_message.clear();
            return;
        }
        if self.controls.pressed(Action::QuickTechniques, input)
            || self.controls.pressed(Action::Learn, input)
            || clicked && hovered_action == Some(0)
        {
            self.use_quick_technique(&techniques);
        }
    }

    fn update_skills(&mut self, input: &InputFrame) {
        let discipline_count = self.game.rules().skills.disciplines().count();
        if discipline_count == 0 {
            return;
        }
        let initial_techniques = self.selected_skill_techniques();
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = SkillsLayout::new(
            width,
            height,
            discipline_count,
            self.skill_technique_selection,
            initial_techniques.len(),
        );
        let hovered_discipline = input.pointer.and_then(|point| {
            layout
                .discipline_rows
                .iter()
                .position(|rect| rect.contains(point.into()))
        });
        let hovered_technique = input.pointer.and_then(|point| {
            layout
                .technique_rows
                .iter()
                .find(|(_, rect)| rect.contains(point.into()))
                .map(|(index, _)| *index)
        });
        let hovered_action = input.pointer.and_then(|point| {
            layout
                .actions
                .iter()
                .position(|rect| rect.contains(point.into()))
        });
        self.menu_focus.hovered = hovered_discipline
            .or_else(|| hovered_technique.map(|index| discipline_count + index))
            .or_else(|| {
                hovered_action.map(|index| discipline_count + initial_techniques.len() + index)
            });
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked && hovered_action == Some(2) {
            self.skills_open = false;
            self.level_up_notice = None;
            return;
        }
        if clicked && let Some(index) = hovered_discipline {
            self.skill_discipline_selection = index;
            self.skill_technique_selection = 0;
            return;
        }
        if input.wheel_y != 0.0
            && input
                .pointer
                .is_some_and(|point| layout.technique_panel.contains(point.into()))
            && !initial_techniques.is_empty()
        {
            let steps = wheel_steps(input.wheel_y);
            self.skill_technique_selection = if input.wheel_y > 0.0 {
                self.skill_technique_selection.saturating_sub(steps)
            } else {
                self.skill_technique_selection
                    .saturating_add(steps)
                    .min(initial_techniques.len() - 1)
            };
            return;
        }
        if clicked && let Some(index) = hovered_technique {
            self.skill_technique_selection = index;
            return;
        }
        if self.controls.pressed(Action::MenuLeft, input) {
            self.skill_discipline_selection = self.skill_discipline_selection.saturating_sub(1);
            self.skill_technique_selection = 0;
            return;
        }
        if self.controls.pressed(Action::MenuRight, input) {
            self.skill_discipline_selection =
                (self.skill_discipline_selection + 1).min(discipline_count - 1);
            self.skill_technique_selection = 0;
            return;
        }

        let techniques = self.selected_skill_techniques();
        if techniques.is_empty() {
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            self.skill_technique_selection = self.skill_technique_selection.saturating_sub(1);
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.skill_technique_selection =
                (self.skill_technique_selection + 1).min(techniques.len() - 1);
            return;
        }
        if self.controls.pressed(Action::Use, input) || clicked && hovered_action == Some(1) {
            let id = techniques[self.skill_technique_selection].clone();
            if !self.game.player_skills().has_learned(&id) {
                self.skill_message = "Cette technique doit d'abord être apprise.".to_owned();
                return;
            }
            if let Some(command) = self.technique_command(id) {
                match self.execute_command(command) {
                    CommandOutcome::Applied => {
                        self.skills_open = false;
                        self.level_up_notice = None;
                    }
                    CommandOutcome::Rejected(reason) => {
                        self.skill_message = command_rejection_message(reason).to_owned()
                    }
                    CommandOutcome::AppliedWithoutTime => {}
                }
                self.capture_events();
            } else if self.attack_aim.is_some() {
                self.skills_open = false;
                self.level_up_notice = None;
            } else if self.component_selection.is_some() {
                self.skills_open = false;
                self.level_up_notice = None;
            } else {
                self.skill_message = "Aucune cible visible pour cette technique.".to_owned();
            }
            return;
        }
        if !self.controls.pressed(Action::Learn, input) && !(clicked && hovered_action == Some(0)) {
            return;
        }

        let technique = techniques[self.skill_technique_selection].clone();
        let Some(definition) = self.game.rules().skills.technique(&technique) else {
            self.skill_message = "Données de la technique indisponibles.".to_owned();
            return;
        };
        let discipline = definition.discipline().clone();
        let Ok(availability) = self.game.discipline_availability(&discipline) else {
            self.skill_message = "Données de la discipline invalides.".to_owned();
            return;
        };
        if !availability.is_open() {
            self.skill_message =
                "Discipline verrouillée : ses systèmes de jeu ne sont pas encore disponibles."
                    .to_owned();
            return;
        }

        let outcome = self.execute_command(GameCommand::LearnTechnique {
            technique: technique.clone(),
        });
        match outcome {
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.skill_message = format!("{} apprise.", self.technique_name(&technique));
            }
            CommandOutcome::Rejected(reason) => {
                self.skill_message = command_rejection_message(reason).to_owned();
            }
        }
        self.capture_events();
    }

    fn update_inventory(&mut self, input: &InputFrame) {
        let count = self.inventory_entries().len();
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = InventoryLayout::new(width, height, self.inventory_selection, count);
        let hovered_row = input.pointer.and_then(|point| {
            layout
                .rows
                .iter()
                .find(|(_, rect)| rect.contains(point.into()))
                .map(|(index, _)| *index)
        });
        let hovered_action = input.pointer.and_then(|point| {
            layout
                .actions
                .iter()
                .position(|rect| rect.contains(point.into()))
        });
        let hovered_filter = input.pointer.and_then(|point| {
            layout
                .filters
                .iter()
                .position(|rect| rect.contains(point.into()))
        });
        let hovered_sort = input
            .pointer
            .is_some_and(|point| layout.sort.contains(point.into()));
        let hovered_character = input.pointer.is_some_and(|point| {
            layout
                .character_details
                .is_some_and(|rect| rect.contains(point.into()))
        });
        self.menu_focus.hovered = hovered_row
            .or_else(|| hovered_action.map(|index| count + index))
            .or_else(|| hovered_filter.map(|index| count + 6 + index))
            .or_else(|| hovered_sort.then_some(count + 11))
            .or_else(|| hovered_character.then_some(count + 12));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked && let Some(index) = hovered_filter {
            self.inventory_filter = InventoryFilter::ALL[index];
            self.inventory_selection = 0;
            self.inventory_message = format!(
                "Filtre {} appliqué.",
                self.inventory_filter.label().to_lowercase()
            );
            return;
        }
        if clicked && hovered_sort {
            self.inventory_sort = self.inventory_sort.other();
            self.inventory_selection = 0;
            self.inventory_message =
                format!("Tri par {}.", self.inventory_sort.label().to_lowercase());
            return;
        }
        if clicked && hovered_action == Some(5) {
            self.inventory_open = false;
            return;
        }
        if (clicked && hovered_character) || self.controls.pressed(Action::Character, input) {
            self.inventory_open = false;
            self.character_open = true;
            self.menu_focus.reset();
            return;
        }
        if count == 0 {
            return;
        }
        if input.wheel_y != 0.0
            && input
                .pointer
                .is_some_and(|point| point.0 < 32.0 + (width - 64.0).max(620.0) * 0.43)
        {
            let steps = wheel_steps(input.wheel_y);
            self.inventory_selection = if input.wheel_y > 0.0 {
                self.inventory_selection.saturating_sub(steps)
            } else {
                self.inventory_selection
                    .saturating_add(steps)
                    .min(count - 1)
            };
            return;
        }
        if clicked && let Some(index) = hovered_row {
            self.inventory_selection = index;
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            self.inventory_selection = self.inventory_selection.saturating_sub(1);
            return;
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.inventory_selection = (self.inventory_selection + 1).min(count - 1);
            return;
        }

        let Some(item) = self
            .inventory_entries()
            .get(self.inventory_selection)
            .map(|entry| entry.instance())
        else {
            return;
        };
        let selected_definition = self.game.player_inventory().get(item);
        let selected_is_weapon = selected_definition
            .is_some_and(|entry| self.game.rules().weapons.get(entry.item()).is_some());
        let selected_is_consumable = selected_definition.is_some_and(|entry| {
            self.game
                .rules()
                .items
                .get(entry.item())
                .is_some_and(|definition| definition.kind() == ItemKind::Consumable)
        });
        let selected_armor_slot = selected_definition
            .and_then(|entry| self.game.rules().items.get(entry.item()))
            .filter(|definition| definition.kind() == ItemKind::Armor)
            .and_then(|definition| definition.equipment())
            .map(|profile| profile.slot().clone());
        let selected_is_armor = selected_armor_slot.is_some();
        let mouse_action = clicked.then_some(hovered_action).flatten();
        if matches!(mouse_action, Some(0..=2))
            && !selected_is_weapon
            && !(selected_is_armor && mouse_action == Some(0))
            || mouse_action == Some(3) && !selected_is_consumable
        {
            return;
        }
        if self.controls.pressed(Action::Drop, input) || mouse_action == Some(4) {
            let item_name = self
                .game
                .player_inventory()
                .get(item)
                .map(|entry| self.item_name(entry.item()))
                .unwrap_or_else(|| "Objet inconnu".to_owned());
            let outcome = self.execute_command(GameCommand::DropItem { item });
            match outcome {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    self.inventory_message = format!("{item_name} déposé.");
                }
                CommandOutcome::Rejected(reason) => {
                    self.inventory_message = command_rejection_message(reason).to_owned();
                }
            }
            self.capture_events();
            self.clamp_inventory_selection();
            return;
        }
        if selected_is_armor
            && (self.controls.pressed(Action::Use, input) || mouse_action == Some(0))
        {
            let slot = selected_armor_slot.expect("an armor selection retains its slot");
            let item_name = self
                .game
                .player_inventory()
                .get(item)
                .map(|entry| self.item_name(entry.item()))
                .unwrap_or_else(|| "Armure inconnue".to_owned());
            if self.game.player_equipment().equipped(&slot) == Some(item) {
                self.inventory_message = format!("{item_name} est déjà équipée.");
                return;
            }
            let outcome = self.execute_command(GameCommand::EquipItem { slot, item });
            match outcome {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    let armor = self
                        .game
                        .actor_armor_profile(self.game.player_id())
                        .map_or(0, project_rl::combat::ArmorProfile::after_fragilization);
                    self.inventory_message =
                        format!("{item_name} équipée · Blindage total {armor}.");
                }
                CommandOutcome::Rejected(reason) => {
                    self.inventory_message = command_rejection_message(reason).to_owned();
                    self.push_log(self.inventory_message.clone());
                }
            }
            self.capture_events();
            return;
        }
        if self.controls.pressed(Action::Use, input) || mouse_action == Some(3) {
            let item_name = self
                .game
                .player_inventory()
                .get(item)
                .map(|entry| self.item_name(entry.item()))
                .unwrap_or_else(|| "Objet inconnu".to_owned());
            let outcome = self.execute_command(GameCommand::UseItem { item });
            match outcome {
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                    self.inventory_message = format!("{item_name} utilisé.");
                }
                CommandOutcome::Rejected(reason) => {
                    self.inventory_message = command_rejection_message(reason).to_owned();
                }
            }
            self.capture_events();
            self.clamp_inventory_selection();
            return;
        }

        let requested_slot = hovered_action
            .filter(|index| *index < 3)
            .map(|index| index as u8)
            .or_else(|| pressed_weapon_slot(&self.controls, input));
        let Some(slot) = requested_slot else {
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
            self.inventory_message = format!("Emplacement d'arme {} maintenant actif.", slot + 1);
            return;
        }
        let item_name = self
            .game
            .player_inventory()
            .get(item)
            .map(|entry| self.item_name(entry.item()))
            .unwrap_or_else(|| "Objet inconnu".to_owned());
        let outcome = self.execute_command(GameCommand::EquipWeapon { slot, item });
        match outcome {
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                self.active_weapon_slot = slot;
                self.inventory_message = format!(
                    "{item_name} équipé et actif sur l'emplacement d'arme {}.",
                    slot + 1
                );
            }
            CommandOutcome::Rejected(reason) => {
                self.inventory_message = command_rejection_message(reason).to_owned();
                self.push_log(self.inventory_message.clone());
            }
        }
        self.capture_events();
    }

    fn draw_inventory(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (self.ui_width() - margin * 2.0).max(620.0);
        let height = (self.ui_height() - top * 2.0).max(400.0);
        let theme = UiTheme;
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);
        let inventory = self.game.player_inventory();
        let entries = self.inventory_entries();
        let layout = InventoryLayout::new(
            self.ui_width(),
            self.ui_height(),
            self.inventory_selection,
            entries.len(),
        );
        let left_width = layout.left_width;
        let detail_right = layout.stats_panel.map_or(margin + width, |stats| stats.x);

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 210),
        );
        theme.panel(Rect::new(margin, top, width, height));
        draw_line(
            margin + left_width,
            top + 58.0,
            margin + left_width,
            top + height - 55.0,
            1.0,
            muted,
        );
        if let Some(stats) = layout.stats_panel {
            draw_line(
                stats.x,
                top + 58.0,
                stats.x,
                top + height - 55.0,
                1.0,
                muted,
            );
        }
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
            "INVENTAIRE ET ÉQUIPEMENT",
            margin + 20.0,
            top + 37.0,
            25.0,
            cyan,
        );
        let capacity = if layout.stats_panel.is_some() {
            format!(
                "EMPLACEMENTS {:02}/{:02}",
                inventory.len(),
                inventory.capacity()
            )
        } else {
            format!(
                "{} · FICHE  ·  EMPLACEMENTS {:02}/{:02}",
                self.controls.label(Action::Character),
                inventory.len(),
                inventory.capacity()
            )
        };
        let capacity_width = measure_text(&capacity, None, 18, 1.0).width;
        draw_text(
            &capacity,
            margin + width - capacity_width - 20.0,
            top + 35.0,
            18.0,
            muted,
        );

        for (index, (rect, filter)) in layout.filters.iter().zip(InventoryFilter::ALL).enumerate() {
            theme.button(
                *rect,
                filter.label(),
                self.menu_focus.hovered == Some(entries.len() + 6 + index),
                filter == self.inventory_filter,
                true,
                ButtonTone::Secondary,
            );
        }
        theme.button(
            layout.sort,
            &format!("Tri : {}", self.inventory_sort.label()),
            self.menu_focus.hovered == Some(entries.len() + 11),
            false,
            true,
            ButtonTone::Secondary,
        );
        for (index, row) in &layout.rows {
            let Some(entry) = entries.get(*index).copied() else {
                continue;
            };
            let row_y = row.y + 17.0;
            let hovered = self.menu_focus.hovered == Some(*index);
            if *index == self.inventory_selection {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    Color::from_rgba(18, 58, 63, 230),
                );
                draw_rectangle(row.x, row.y, 3.0, row.h, amber);
            } else if hovered {
                draw_rectangle(row.x, row.y, row.w, row.h, UiTheme.surface_raised());
            }
            let classification = if self.game.rules().weapons.get(entry.item()).is_some() {
                format!("ARME · {}", self.equipped_weapon_label(entry.instance()))
            } else if let Some(definition) = self.game.rules().items.get(entry.item()) {
                format!(
                    "{}{}  x{}",
                    match definition.kind() {
                        ItemKind::Armor => {
                            if self
                                .game
                                .player_equipment()
                                .slot_of(entry.instance())
                                .is_some()
                            {
                                "ARMURE · ÉQUIPÉE"
                            } else {
                                "ARMURE · RANGÉE"
                            }
                        }
                        ItemKind::Consumable => "CONSOMMABLE",
                        ItemKind::Material => "MATÉRIAU",
                    },
                    entry.owner().map_or("", |owner| {
                        if self.game.player_may_take_property_of(owner) {
                            " · ATTRIBUÉ · AUTORISÉ"
                        } else {
                            " · ATTRIBUÉ · NON AUTORISÉ"
                        }
                    }),
                    entry.quantity()
                )
            } else {
                "NON CLASSÉ".to_owned()
            };
            let glyph_color = if *index == self.inventory_selection {
                amber
            } else {
                cyan
            };
            draw_inventory_glyph(
                Rect::new(row.x + 7.0, row.y + 6.0, 27.0, 27.0),
                self.inventory_glyph(entry),
                glyph_color,
            );
            let prefix = if *index == self.inventory_selection {
                ">"
            } else {
                " "
            };
            draw_text(
                format!("{prefix} {}", self.item_name(entry.item())),
                margin + 48.0,
                row_y,
                20.0,
                if *index == self.inventory_selection {
                    amber
                } else {
                    text
                },
            );
            draw_text(&classification, margin + 69.0, row_y + 17.0, 14.0, muted);
        }
        if layout.first_visible > 0 {
            draw_text(
                "↑ PLUS",
                margin + left_width - 76.0,
                top + 88.0,
                14.0,
                amber,
            );
        }
        if layout.first_visible + layout.visible_rows < entries.len() {
            draw_text(
                "↓ PLUS",
                margin + left_width - 76.0,
                top + height - 64.0,
                14.0,
                amber,
            );
        }

        let detail_x = margin + left_width + 24.0;
        let selected = entries.get(self.inventory_selection).copied();
        if let Some(entry) = selected {
            let weapon = self.game.rules().weapons.get(entry.item());
            let item_definition = self.game.rules().items.get(entry.item());
            draw_text_bold(
                self.item_name(entry.item()),
                detail_x,
                top + 94.0,
                27.0,
                amber,
            );
            draw_inventory_glyph(
                Rect::new(detail_right - 65.0, top + 68.0, 48.0, 48.0),
                self.inventory_glyph(entry),
                cyan,
            );
            if let Some(weapon) = weapon {
                draw_text(
                    self.equipped_weapon_label(entry.instance()),
                    detail_x,
                    top + 120.0,
                    15.0,
                    cyan,
                );
                draw_text("PROFIL DE COMBAT", detail_x, top + 151.0, 16.0, muted);
                let attack = weapon.attack();
                let selected_damage = self
                    .game
                    .resolved_attack_damage(self.game.player_id(), attack)
                    .unwrap_or_else(|| attack.damage())
                    .raw_total();
                let active_damage = self
                    .game
                    .equipped_player_weapon(self.active_weapon_slot)
                    .and_then(|active| {
                        self.game
                            .resolved_attack_damage(self.game.player_id(), active.attack())
                    })
                    .map(DamageImpact::raw_total);
                if let Some(active_damage) = active_damage {
                    let delta = i32::from(selected_damage) - i32::from(active_damage);
                    draw_text(
                        if delta == 0 {
                            "= arme active".to_owned()
                        } else {
                            format!("{delta:+} dégâts vs arme active")
                        },
                        detail_x + 172.0,
                        top + 151.0,
                        14.0,
                        if delta >= 0 {
                            UiTheme.success()
                        } else {
                            UiTheme.danger()
                        },
                    );
                }
                let authored_damage = attack.damage();
                let damage = self
                    .game
                    .resolved_attack_damage(self.game.player_id(), attack)
                    .unwrap_or(authored_damage);
                let damage_label = attack.melee_impact().map_or_else(
                    || damage.raw_total().to_string(),
                    |impact| {
                        format!(
                            "{} (REF {} · CAP {})",
                            damage.raw_total(),
                            physical_damage_total(authored_damage),
                            impact.material_cap
                        )
                    },
                );
                draw_stat_line(detail_x, top + 188.0, "DÉGÂTS", &damage_label, text, muted);
                draw_stat_line(
                    detail_x,
                    top + 218.0,
                    "TYPE DE DÉGÂTS",
                    &damage_component_types_label(damage).to_uppercase(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 248.0,
                    "PÉNÉTRATION",
                    &format_damage_penetration(damage),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 278.0,
                    "PORTÉE",
                    &attack.range().to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 308.0,
                    "LIGNE DE VUE",
                    if attack.requires_line_of_sight() {
                        "REQUISE"
                    } else {
                        "NON"
                    },
                    text,
                    muted,
                );
                let area = match attack.area() {
                    AttackArea::Single => "CIBLE UNIQUE".to_owned(),
                    AttackArea::Cone(cone) => format!(
                        "CÔNE LONG · LARGEUR {} À {}",
                        1,
                        cone.maximum_half_width()
                            .saturating_mul(2)
                            .saturating_add(1)
                    ),
                };
                draw_stat_line(detail_x, top + 338.0, "ZONE", &area, text, muted);
                draw_text(
                    "EMPLACEMENTS D'ARME",
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
                        .map(|weapon| self.item_name(weapon.id()))
                        .unwrap_or_else(|| "— vide —".to_owned());
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
            } else if let Some(item_definition) = item_definition {
                draw_text(
                    format!(
                        "{}{} x{}",
                        match item_definition.kind() {
                            ItemKind::Armor => "ARMURE",
                            ItemKind::Consumable => "CONSOMMABLE",
                            ItemKind::Material => "MATÉRIAU",
                        },
                        entry.owner().map_or("", |owner| {
                            if self.game.player_may_take_property_of(owner) {
                                " · ATTRIBUÉ · AUTORISÉ"
                            } else {
                                " · ATTRIBUÉ · NON AUTORISÉ"
                            }
                        }),
                        entry.quantity()
                    ),
                    detail_x,
                    top + 120.0,
                    15.0,
                    cyan,
                );
                draw_text(
                    match item_definition.kind() {
                        ItemKind::Armor => "PROTECTION ÉQUIPABLE",
                        ItemKind::Consumable => "CONSOMMABLE",
                        ItemKind::Material => "MATÉRIAU D'ARTISANAT",
                    },
                    detail_x,
                    top + 151.0,
                    16.0,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 188.0,
                    "QUANTITÉ",
                    &entry.quantity().to_string(),
                    text,
                    muted,
                );
                draw_stat_line(
                    detail_x,
                    top + 218.0,
                    "PILE MAXIMALE",
                    &item_definition.maximum_stack().to_string(),
                    text,
                    muted,
                );
                if let Some(equipment) = item_definition.equipment() {
                    draw_stat_line(
                        detail_x,
                        top + 260.0,
                        "BLINDAGE",
                        &format!("+{}", equipment.armor()),
                        text,
                        muted,
                    );
                    draw_stat_line(
                        detail_x,
                        top + 290.0,
                        "EMPLACEMENT",
                        &self.equipment_slot_name(equipment.slot()).to_uppercase(),
                        text,
                        muted,
                    );
                    let total = self
                        .game
                        .actor_armor_profile(self.game.player_id())
                        .map_or(0, project_rl::combat::ArmorProfile::after_fragilization);
                    draw_stat_line(
                        detail_x,
                        top + 320.0,
                        "BLINDAGE ACTUEL",
                        &total.to_string(),
                        text,
                        muted,
                    );
                    draw_stat_line(
                        detail_x,
                        top + 350.0,
                        "ÉTAT",
                        if self
                            .game
                            .player_equipment()
                            .slot_of(entry.instance())
                            .is_some()
                        {
                            "ÉQUIPÉE"
                        } else {
                            "RANGÉE"
                        },
                        text,
                        muted,
                    );
                } else {
                    for (index, effect) in item_definition.effects().iter().enumerate() {
                        let ItemEffect::RestoreIntegrity { amount } = effect;
                        draw_stat_line(
                            detail_x,
                            top + 260.0 + index as f32 * 30.0,
                            "RESTAURE",
                            &format!("{amount} PV"),
                            text,
                            muted,
                        );
                    }
                }
            }
        }

        if selected.is_none() && self.inventory_filter == InventoryFilter::Armor {
            draw_text_bold(
                "AUCUNE ARMURE TRANSPORTÉE",
                detail_x,
                top + 108.0,
                24.0,
                amber,
            );
            draw_inventory_glyph(
                Rect::new(detail_x, top + 140.0, 48.0, 48.0),
                InventoryGlyph::Armor,
                muted,
            );
            let guidance = if self.generation_version < ARMOR_EQUIPMENT_GENERATION_VERSION {
                "Cette partie suspendue conserve ses anciennes règles. Commencez une nouvelle partie pour activer les armures équipables."
            } else {
                "BRÈCHE commence avec un plastron rapiécé dans son sac. Sinon, fouillez les caches à l'extérieur de la ville ; les protections plus solides deviennent plus fréquentes en profondeur."
            };
            draw_wrapped_text(
                guidance,
                detail_x,
                top + 222.0,
                detail_right - detail_x - 24.0,
                4,
                17,
                text,
            );
        }

        if let Some(stats_panel) = layout.stats_panel {
            self.draw_inventory_character_summary(
                stats_panel,
                layout
                    .character_details
                    .expect("a stats panel has a button"),
                entries.len() + 12,
            );
        }

        if entries.len() > layout.visible_rows {
            let track = Rect::new(
                margin + left_width - 7.0,
                top + 108.0,
                3.0,
                (height - 180.0).max(30.0),
            );
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            let thumb_h =
                (track.h * layout.visible_rows as f32 / entries.len() as f32).clamp(20.0, track.h);
            let maximum_first = entries.len().saturating_sub(layout.visible_rows).max(1);
            let thumb_y =
                track.y + (track.h - thumb_h) * layout.first_visible as f32 / maximum_first as f32;
            draw_rectangle(track.x, thumb_y, track.w, thumb_h, cyan);
        }

        let selected_is_weapon =
            selected.is_some_and(|entry| self.game.rules().weapons.get(entry.item()).is_some());
        let selected_is_consumable = selected.is_some_and(|entry| {
            self.game
                .rules()
                .items
                .get(entry.item())
                .is_some_and(|definition| definition.kind() == ItemKind::Consumable)
        });
        let selected_is_armor = selected.is_some_and(|entry| {
            self.game
                .rules()
                .items
                .get(entry.item())
                .is_some_and(|definition| definition.kind() == ItemKind::Armor)
        });
        let armor_actions = self.inventory_filter == InventoryFilter::Armor || selected_is_armor;
        for (index, (rect, (label, enabled))) in layout
            .actions
            .iter()
            .zip([
                (
                    if armor_actions {
                        "Équiper"
                    } else {
                        "Emplacement 1"
                    },
                    selected_is_weapon || selected_is_armor,
                ),
                (
                    if armor_actions {
                        "—"
                    } else {
                        "Emplacement 2"
                    },
                    selected_is_weapon,
                ),
                (
                    if armor_actions {
                        "—"
                    } else {
                        "Emplacement 3"
                    },
                    selected_is_weapon,
                ),
                ("Utiliser", selected_is_consumable),
                ("Déposer", selected.is_some()),
                ("Fermer", true),
            ])
            .enumerate()
        {
            theme.button(
                *rect,
                label,
                self.menu_focus.hovered == Some(entries.len() + index),
                false,
                enabled,
                ButtonTone::Secondary,
            );
        }
        if !self.inventory_message.is_empty() {
            draw_wrapped_text(
                &self.inventory_message,
                detail_x,
                top + 70.0,
                detail_right - detail_x - 20.0,
                1,
                15,
                amber,
            );
        }
    }

    fn draw_inventory_character_summary(
        &self,
        panel: Rect,
        details_button: Rect,
        focus_index: usize,
    ) {
        let theme = UiTheme;
        let card = Rect::new(panel.x + 8.0, panel.y, panel.w - 16.0, panel.h);
        theme.card(card, false);
        let x = card.x + 13.0;
        let right = card.x + card.w - 13.0;

        draw_text_bold("PERSONNAGE", x, card.y + 27.0, 18.0, theme.accent());
        let progression = self.game.player_progression();
        let level = format!("NV {}", progression.level());
        draw_text_bold(
            &level,
            right - measure_text_bold(&level, 16).width,
            card.y + 27.0,
            16.0,
            theme.focus(),
        );
        let class_name = self
            .character_class
            .as_ref()
            .and_then(|id| self.character_classes.get(id))
            .and_then(|class| self.texts.resolve(DISPLAY_LOCALE, class.name_key()))
            .unwrap_or("Restauration antérieure");
        draw_wrapped_text(
            class_name,
            x,
            card.y + 52.0,
            card.w - 26.0,
            1,
            14,
            theme.text(),
        );
        draw_line(x, card.y + 68.0, right, card.y + 68.0, 1.0, theme.muted());

        draw_text_bold("PRIMAIRES", x, card.y + 91.0, 14.0, theme.muted());
        let attributes = self.game.player_primary_attributes();
        for (index, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
            let y = card.y + 117.0 + index as f32 * 25.0;
            draw_text(primary_attribute_label(attribute), x, y, 15.0, theme.text());
            if let Some(attributes) = attributes {
                let value = attributes.value(attribute);
                draw_attribute_meter(
                    right - 91.0,
                    y - 5.0,
                    61.0,
                    value,
                    self.rules.primary_attribute_rules.absolute_minimum,
                    self.rules.primary_attribute_rules.absolute_maximum,
                    theme.accent(),
                );
                let value = value.to_string();
                draw_text_bold(
                    &value,
                    right - measure_text_bold(&value, 17).width,
                    y,
                    17.0,
                    theme.focus(),
                );
            } else {
                draw_text("--", right - 18.0, y, 15.0, theme.muted());
            }
        }

        draw_text_bold("SECONDAIRES", x, card.y + 258.0, 14.0, theme.muted());
        let actor = self.game.actors().get(self.game.player_id());
        let integrity = actor.map_or_else(
            || "--".to_owned(),
            |actor| format!("{} / {}", actor.integrity(), actor.maximum_integrity()),
        );
        let armor = self
            .game
            .actor_armor_profile(self.game.player_id())
            .map_or_else(
                || "--".to_owned(),
                |armor| armor.after_fragilization().to_string(),
            );
        for (index, (label, value)) in [
            ("PV", integrity),
            ("Blindage", armor),
            (
                "Énergie",
                format!(
                    "{} / {}",
                    self.game.player_energy().available(),
                    self.game.player_energy().capacity()
                ),
            ),
            ("Expérience", progression.experience().to_string()),
            (
                "Points comp.",
                progression.unspent_skill_points().to_string(),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            draw_summary_metric(
                x,
                right,
                card.y + 283.0 + index as f32 * 25.0,
                label,
                &value,
            );
        }

        let active_weapon = self.game.equipped_player_weapon(self.active_weapon_slot);
        let hit_chance = active_weapon
            .zip(self.rules.hit_rules)
            .map(|(weapon, rules)| {
                let attack = weapon.attack();
                rules.hit_chance(
                    attack.delivery(),
                    attributes,
                    attack.accuracy_modifier(),
                    None,
                    0,
                    0,
                )
            });
        let evasion = self
            .rules
            .hit_rules
            .map(|rules| rules.evasion(attributes, 0));
        let damage = active_weapon.and_then(|weapon| {
            self.game
                .resolved_attack_damage(self.game.player_id(), weapon.attack())
        });
        let impact = active_weapon
            .and_then(|weapon| weapon.attack().melee_impact())
            .zip(self.rules.physical_rules)
            .map(|(profile, rules)| {
                let weapon = active_weapon.expect("paired weapon remains present");
                let resolved = rules.impact.resolve_melee_damage(
                    attributes,
                    profile.impact_modifier,
                    profile.material_cap,
                    physical_damage_total(weapon.attack().damage()),
                );
                format!("{} / {}", resolved.available, resolved.used)
            });
        draw_text_bold("COMBAT ACTIF", x, card.y + 427.0, 14.0, theme.muted());
        for (index, (label, value)) in [
            (
                "Touche",
                hit_chance.map_or_else(|| "--".to_owned(), |chance| format!("{chance} %")),
            ),
            (
                "Esquive",
                evasion.map_or_else(|| "--".to_owned(), |value| value.to_string()),
            ),
            (
                "Dégâts",
                damage.map_or_else(|| "--".to_owned(), format_damage_impact),
            ),
            ("Impact", impact.unwrap_or_else(|| "--".to_owned())),
        ]
        .into_iter()
        .enumerate()
        {
            draw_summary_metric(
                x,
                right,
                card.y + 452.0 + index as f32 * 25.0,
                label,
                &value,
            );
        }

        theme.button(
            details_button,
            &format!(
                "Fiche détaillée · {}",
                self.controls.label(Action::Character)
            ),
            self.menu_focus.hovered == Some(focus_index),
            false,
            true,
            ButtonTone::Secondary,
        );
    }

    fn draw_character_creation(&self) {
        let Some(creation) = &self.character_creation else {
            return;
        };
        let classes = self.character_classes.iter().collect::<Vec<_>>();
        let layout = CharacterCreationLayout::new(self.ui_width(), self.ui_height(), classes.len());
        let panel = layout.panel;
        let left_width = panel.w * 0.43;
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);
        let disabled = Color::from_rgba(116, 91, 86, 255);

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(2, 8, 12, 254),
        );
        UiTheme.panel(panel);
        draw_line(
            panel.x,
            panel.y + 66.0,
            panel.x + panel.w,
            panel.y + 66.0,
            1.0,
            muted,
        );
        draw_line(
            panel.x + left_width,
            panel.y + 66.0,
            panel.x + left_width,
            panel.y + panel.h - 78.0,
            1.0,
            muted,
        );
        draw_text(
            "RESTAURATION DE L'INSTANCE",
            panel.x + 24.0,
            panel.y + 42.0,
            27.0,
            cyan,
        );
        let step = match creation.stage {
            CharacterCreationStage::Protocol => "ÉTAPE 1 / 2 · PROTOCOLE",
            CharacterCreationStage::Attributes => "ÉTAPE 2 / 2 · ATTRIBUTS",
        };
        let step_width = measure_text(step, None, 15, 1.0).width;
        draw_text(
            step,
            panel.x + panel.w - step_width - 24.0,
            panel.y + 39.0,
            15.0,
            amber,
        );

        let selected = classes
            .get(creation.selected_class)
            .map(|(_, class)| *class);
        match creation.stage {
            CharacterCreationStage::Protocol => {
                draw_text(
                    "PROTOCOLES DISPONIBLES",
                    panel.x + 24.0,
                    panel.y + 92.0,
                    16.0,
                    muted,
                );
                for (index, ((_, class), row)) in classes.iter().zip(&layout.class_rows).enumerate()
                {
                    let selected_row = index == creation.selected_class;
                    let hovered = creation.hovered == Some(CharacterCreationHover::Class(index));
                    draw_rectangle(
                        row.x,
                        row.y,
                        row.w,
                        row.h,
                        if selected_row {
                            Color::from_rgba(18, 58, 63, 245)
                        } else if hovered {
                            Color::from_rgba(19, 44, 52, 245)
                        } else {
                            Color::from_rgba(11, 28, 35, 245)
                        },
                    );
                    draw_rectangle_lines(
                        row.x,
                        row.y,
                        row.w,
                        row.h,
                        if selected_row || hovered { 2.0 } else { 1.0 },
                        if selected_row {
                            amber
                        } else if hovered {
                            cyan
                        } else {
                            muted
                        },
                    );
                    let name = self
                        .texts
                        .resolve(DISPLAY_LOCALE, class.name_key())
                        .unwrap_or("Protocole sans nom");
                    let role = self
                        .texts
                        .resolve(DISPLAY_LOCALE, class.role_key())
                        .unwrap_or("Fonction non décrite");
                    draw_text(
                        name,
                        row.x + 14.0,
                        row.y + 25.0,
                        20.0,
                        if selected_row { amber } else { text },
                    );
                    draw_text(role, row.x + 28.0, row.y + 46.0, 14.0, cyan);
                }
            }
            CharacterCreationStage::Attributes => {
                draw_text(
                    "RÉPARTITION DES ATTRIBUTS",
                    panel.x + 28.0,
                    panel.y + 105.0,
                    16.0,
                    muted,
                );
                for (index, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
                    let row = layout.attribute_rows[index];
                    let selected_row = index == creation.selected_attribute;
                    let hovered = matches!(
                        creation.hovered,
                        Some(CharacterCreationHover::Attribute(candidate))
                            | Some(CharacterCreationHover::AttributeMinus(candidate))
                            | Some(CharacterCreationHover::AttributePlus(candidate))
                            if candidate == index
                    );
                    draw_rectangle(
                        row.x,
                        row.y,
                        row.w,
                        row.h,
                        if selected_row {
                            Color::from_rgba(18, 58, 63, 245)
                        } else if hovered {
                            Color::from_rgba(19, 44, 52, 245)
                        } else {
                            Color::from_rgba(11, 28, 35, 245)
                        },
                    );
                    draw_text(
                        primary_attribute_label(attribute),
                        row.x + 12.0,
                        row.y + 29.0,
                        18.0,
                        if selected_row { amber } else { text },
                    );
                    for (button, symbol, hover) in [
                        (
                            layout.attribute_minus[index],
                            "−",
                            CharacterCreationHover::AttributeMinus(index),
                        ),
                        (
                            layout.attribute_plus[index],
                            "+",
                            CharacterCreationHover::AttributePlus(index),
                        ),
                    ] {
                        let hovered = creation.hovered == Some(hover);
                        UiTheme.button(button, symbol, hovered, false, true, ButtonTone::Secondary);
                    }
                    let value = creation.attributes.value(attribute).to_string();
                    let value_width = measure_text(&value, None, 22, 1.0).width;
                    draw_text(
                        &value,
                        row.x + row.w - 59.0 - value_width * 0.5,
                        row.y + 30.0,
                        22.0,
                        amber,
                    );
                    if let Some(class) = selected {
                        let recommended = class.recommended_attributes().value(attribute);
                        let delta = i16::from(creation.attributes.value(attribute))
                            - i16::from(recommended);
                        draw_text(
                            if delta == 0 {
                                "conseillé".to_owned()
                            } else {
                                format!("{delta:+}")
                            },
                            row.x + row.w - 176.0,
                            row.y + 27.0,
                            14.0,
                            if delta == 0 { cyan } else { muted },
                        );
                    }
                }
            }
        }

        let detail_x = panel.x + left_width + 24.0;
        let detail_width = panel.w - left_width - 48.0;
        if let Some(class) = selected {
            let name = self
                .texts
                .resolve(DISPLAY_LOCALE, class.name_key())
                .unwrap_or("Protocole sans nom");
            let role = self
                .texts
                .resolve(DISPLAY_LOCALE, class.role_key())
                .unwrap_or("Fonction non décrite");
            draw_text_bold(name, detail_x, panel.y + 112.0, 28.0, amber);
            draw_text(role, detail_x, panel.y + 139.0, 17.0, cyan);
            let description = self
                .texts
                .resolve(DISPLAY_LOCALE, class.description_key())
                .unwrap_or("Description indisponible.");
            let weapons = class
                .starting_weapons()
                .iter()
                .map(|id| self.item_name(id))
                .collect::<Vec<_>>()
                .join(" · ");
            let repairs = class
                .starting_items()
                .iter()
                .map(|item| format!("{} ×{}", self.item_name(item.item()), item.quantity()))
                .collect::<Vec<_>>()
                .join(" · ");

            if creation.stage == CharacterCreationStage::Attributes {
                let rules = self.rules.primary_attribute_rules;
                let remaining = rules
                    .creation_total
                    .saturating_sub(creation.attributes.total());
                draw_text_bold(
                    "RÉPARTITION PERSONNALISÉE",
                    detail_x,
                    panel.y + 184.0,
                    18.0,
                    text,
                );
                draw_text_bold(
                    format!(
                        "{} / {} points · {} restant{}",
                        creation.attributes.total(),
                        rules.creation_total,
                        remaining,
                        if remaining > 1 { "s" } else { "" }
                    ),
                    detail_x,
                    panel.y + 218.0,
                    24.0,
                    if remaining == 0 { cyan } else { amber },
                );
                draw_wrapped_text(
                    "Les écarts +/− comparent votre répartition au profil conseillé. Tous les points restent librement modifiables.",
                    detail_x,
                    panel.y + 249.0,
                    detail_width,
                    3,
                    15,
                    muted,
                );
                draw_text_bold(
                    format!(
                        "{} · EFFETS",
                        primary_attribute_label(PrimaryAttribute::ALL[creation.selected_attribute])
                    ),
                    detail_x,
                    panel.y + 309.0,
                    17.0,
                    cyan,
                );
                draw_wrapped_text(
                    primary_attribute_description(
                        PrimaryAttribute::ALL[creation.selected_attribute],
                    ),
                    detail_x,
                    panel.y + 335.0,
                    detail_width,
                    ((panel.h - 445.0) / 20.0).floor().max(2.0) as usize,
                    15,
                    text,
                );
                UiTheme.button(
                    layout.preset,
                    "Rétablir le profil recommandé",
                    creation.hovered == Some(CharacterCreationHover::Preset),
                    false,
                    true,
                    ButtonTone::Secondary,
                );
            } else {
                draw_wrapped_text(
                    description,
                    detail_x,
                    panel.y + 170.0,
                    detail_width,
                    2,
                    16,
                    text,
                );
                draw_wrapped_text(
                    &format!("Équipement initial · {weapons} · {repairs}"),
                    detail_x,
                    panel.y + 218.0,
                    detail_width,
                    1,
                    15,
                    muted,
                );
                let profile = class.recommended_attributes();
                let card = Rect::new(detail_x, panel.y + 241.0, detail_width, 145.0);
                draw_recommended_profile(
                    card,
                    profile,
                    self.rules.primary_attribute_rules.creation_minimum,
                    self.rules.primary_attribute_rules.creation_maximum,
                );
            }
        }

        let can_start = creation.stage == CharacterCreationStage::Protocol
            || creation.attributes.total() == self.rules.primary_attribute_rules.creation_total;
        let cancel_label = if creation.stage == CharacterCreationStage::Protocol {
            "Retour"
        } else {
            "Changer de protocole"
        };
        let continue_label = if creation.stage == CharacterCreationStage::Protocol {
            "Configurer les attributs"
        } else {
            "Commencer la partie"
        };
        UiTheme.button(
            layout.cancel,
            cancel_label,
            creation.hovered == Some(CharacterCreationHover::Cancel),
            false,
            true,
            ButtonTone::Secondary,
        );
        UiTheme.button(
            layout.continue_button,
            continue_label,
            creation.hovered == Some(CharacterCreationHover::Continue),
            false,
            can_start,
            ButtonTone::Primary,
        );
        if !creation.message.is_empty() {
            draw_wrapped_text(
                &creation.message,
                panel.x + 190.0,
                panel.y + panel.h - 35.0,
                panel.w - 430.0,
                1,
                14,
                if can_start { muted } else { disabled },
            );
        }
    }

    fn draw_character(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (self.ui_width() - margin * 2.0).max(620.0);
        let height = (self.ui_height() - top * 2.0).max(400.0);
        let left_width = width * 0.48;
        let theme = UiTheme;
        let layout = CharacterLayout::new(self.ui_width(), self.ui_height());

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            theme.backdrop(),
        );
        theme.panel(Rect::new(margin, top, width, height));
        draw_line(
            margin,
            top + 58.0,
            margin + width,
            top + 58.0,
            1.0,
            theme.muted(),
        );
        draw_line(
            margin + left_width,
            top + 58.0,
            margin + left_width,
            top + height - 55.0,
            1.0,
            theme.muted(),
        );
        draw_line(
            margin,
            top + height - 55.0,
            margin + width,
            top + height - 55.0,
            1.0,
            theme.muted(),
        );

        draw_text_bold(
            "PERSONNAGE",
            margin + 20.0,
            top + 37.0,
            25.0,
            theme.accent(),
        );
        let progression = self.game.player_progression();
        let level = format!("NIVEAU {}", progression.level());
        let level_width = measure_text(&level, None, 18, 1.0).width;
        draw_text(
            &level,
            margin + width - level_width - 20.0,
            top + 35.0,
            18.0,
            theme.focus(),
        );

        let class = self
            .character_class
            .as_ref()
            .and_then(|id| self.character_classes.get(id));
        let class_name = class
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.name_key()))
            .unwrap_or("Restauration antérieure");
        let class_role = class
            .and_then(|definition| self.texts.resolve(DISPLAY_LOCALE, definition.role_key()))
            .unwrap_or("Profil sans protocole enregistré");
        let class_card = Rect::new(margin + 14.0, top + 72.0, left_width - 28.0, 112.0);
        theme.card(class_card, false);
        draw_text_bold(
            class_name,
            class_card.x + 12.0,
            class_card.y + 28.0,
            24.0,
            theme.focus(),
        );
        draw_text(
            class_role,
            class_card.x + 12.0,
            class_card.y + 50.0,
            15.0,
            theme.accent(),
        );
        let class_description = class
            .and_then(|definition| {
                self.texts
                    .resolve(DISPLAY_LOCALE, definition.description_key())
            })
            .unwrap_or(
                "Cette partie a été créée avant l'enregistrement des protocoles de restauration.",
            );
        draw_wrapped_text(
            class_description,
            class_card.x + 12.0,
            class_card.y + 75.0,
            class_card.w - 24.0,
            2,
            14,
            theme.text(),
        );

        draw_text_bold(
            "ATTRIBUTS PRIMAIRES",
            margin + 20.0,
            top + 205.0,
            16.0,
            theme.muted(),
        );
        let attributes = self.game.player_primary_attributes();
        for (index, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
            let row = layout.attribute_rows[index];
            let selected = index == self.character_attribute_selection;
            let hovered = self.menu_focus.hovered == Some(index);
            if selected || hovered {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    if selected {
                        theme.surface_selected()
                    } else {
                        theme.surface_raised()
                    },
                );
            }
            if selected {
                draw_rectangle(row.x, row.y, 3.0, row.h, theme.focus());
            }
            let value = attributes.map_or(0, |values| values.value(attribute));
            draw_text(
                primary_attribute_label(attribute),
                row.x + 10.0,
                row.y + row.h * 0.7,
                17.0,
                if selected {
                    theme.focus()
                } else {
                    theme.text()
                },
            );
            draw_text_bold(
                value.to_string(),
                row.x + row.w - 30.0,
                row.y + row.h * 0.72,
                20.0,
                theme.focus(),
            );
            draw_attribute_meter(
                row.x + row.w * 0.48,
                row.y + row.h * 0.5,
                row.w * 0.36,
                value,
                self.rules.primary_attribute_rules.absolute_minimum,
                self.rules.primary_attribute_rules.absolute_maximum,
                theme.accent(),
            );
        }

        let right_x = margin + left_width + 24.0;
        let right_width = width - left_width - 48.0;
        let selected_attribute = PrimaryAttribute::ALL[self.character_attribute_selection];
        let attribute_card = Rect::new(right_x, top + 72.0, right_width, 91.0);
        theme.card(attribute_card, true);
        draw_text_bold(
            format!("EFFETS · {}", primary_attribute_label(selected_attribute)),
            attribute_card.x + 12.0,
            attribute_card.y + 25.0,
            18.0,
            theme.focus(),
        );
        draw_wrapped_text(
            primary_attribute_description(selected_attribute),
            attribute_card.x + 12.0,
            attribute_card.y + 49.0,
            attribute_card.w - 24.0,
            2,
            14,
            theme.text(),
        );

        let actor = self.game.actors().get(self.game.player_id());
        let integrity = actor.map_or("--".to_owned(), |actor| {
            format!("{} / {}", actor.integrity(), actor.maximum_integrity())
        });
        let armor = self
            .game
            .actor_armor_profile(self.game.player_id())
            .map_or("--".to_owned(), |armor| {
                armor.after_fragilization().to_string()
            });
        let state_card = Rect::new(right_x, top + 174.0, right_width, 88.0);
        theme.card(state_card, false);
        draw_text_bold(
            "ÉTAT ACTUEL",
            state_card.x + 12.0,
            state_card.y + 21.0,
            15.0,
            theme.muted(),
        );
        let state_metrics = [
            ("PV", integrity),
            ("Blindage", armor),
            (
                "Énergie",
                format!(
                    "{} / {}",
                    self.game.player_energy().available(),
                    self.game.player_energy().capacity()
                ),
            ),
            ("Expérience", progression.experience().to_string()),
            (
                "Points comp.",
                progression.unspent_skill_points().to_string(),
            ),
        ];
        for (index, (label, value)) in state_metrics.iter().enumerate() {
            draw_compact_metric(
                state_card.x + 12.0 + (index % 3) as f32 * state_card.w / 3.0,
                state_card.y + 46.0 + (index / 3) as f32 * 28.0,
                label,
                value,
                state_card.w / 3.0 - 18.0,
            );
        }

        let active_weapon = self.game.equipped_player_weapon(self.active_weapon_slot);
        let weapon_name = active_weapon
            .map(|weapon| self.item_name(weapon.id()))
            .unwrap_or_else(|| "Vide".to_owned());
        let hit_chance = active_weapon
            .zip(self.rules.hit_rules)
            .map(|(weapon, rules)| {
                let attack = weapon.attack();
                rules.hit_chance(
                    attack.delivery(),
                    attributes,
                    attack.accuracy_modifier(),
                    None,
                    0,
                    0,
                )
            });
        let evasion = self
            .rules
            .hit_rules
            .map(|rules| rules.evasion(attributes, 0));
        let damage = active_weapon.and_then(|weapon| {
            self.game
                .resolved_attack_damage(self.game.player_id(), weapon.attack())
        });
        let impact = active_weapon
            .and_then(|weapon| weapon.attack().melee_impact())
            .zip(self.rules.physical_rules)
            .map(|(profile, rules)| {
                let resolved = rules.impact.resolve_melee_damage(
                    attributes,
                    profile.impact_modifier,
                    profile.material_cap,
                    physical_damage_total(
                        active_weapon
                            .expect("paired weapon remains present")
                            .attack()
                            .damage(),
                    ),
                );
                format!(
                    "{} disponible · {} transmis",
                    resolved.available, resolved.used
                )
            });
        let combat_card = Rect::new(
            right_x,
            top + 274.0,
            right_width,
            (height - 329.0).max(68.0),
        );
        theme.card(combat_card, false);
        draw_text_bold(
            format!(
                "COMBAT · EMPLACEMENT D'ARME {} · {weapon_name}",
                self.active_weapon_slot + 1
            ),
            combat_card.x + 12.0,
            combat_card.y + 21.0,
            15.0,
            theme.muted(),
        );
        let combat_metrics = [
            (
                "Touche",
                hit_chance.map_or_else(|| "--".to_owned(), |chance| format!("{chance} %")),
            ),
            (
                "Esquive",
                evasion.map_or_else(|| "--".to_owned(), |value| value.to_string()),
            ),
            (
                "Dégâts",
                damage.map_or_else(|| "--".to_owned(), format_damage_impact),
            ),
            ("Impact", impact.unwrap_or_else(|| "--".to_owned())),
        ];
        for (index, (label, value)) in combat_metrics.iter().enumerate() {
            draw_compact_metric(
                combat_card.x + 12.0 + index as f32 * combat_card.w * 0.25,
                combat_card.y + 49.0,
                label,
                value,
                combat_card.w * 0.25 - 14.0,
            );
        }

        for (index, (rect, label)) in layout
            .actions
            .iter()
            .zip(["Inventaire", "Compétences", "Fermer"])
            .enumerate()
        {
            theme.button(
                *rect,
                label,
                self.menu_focus.hovered == Some(PrimaryAttribute::ALL.len() + index),
                false,
                true,
                ButtonTone::Secondary,
            );
        }
    }

    fn draw_skills(&self) {
        let margin = 32.0;
        let top = 30.0;
        let width = (self.ui_width() - margin * 2.0).max(620.0);
        let height = (self.ui_height() - top * 2.0).max(400.0);
        let cyan = Color::from_rgba(99, 242, 210, 255);
        let muted = Color::from_rgba(102, 139, 148, 255);
        let text = Color::from_rgba(205, 225, 225, 255);
        let amber = Color::from_rgba(255, 211, 92, 255);
        let locked = Color::from_rgba(154, 98, 92, 255);
        let disciplines = self.skill_disciplines();
        let techniques = self.selected_skill_techniques();
        let layout = SkillsLayout::new(
            self.ui_width(),
            self.ui_height(),
            disciplines.len(),
            self.skill_technique_selection,
            techniques.len(),
        );

        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(0, 2, 4, 210),
        );
        UiTheme.panel(layout.panel);
        draw_line(margin, top + 58.0, margin + width, top + 58.0, 1.0, muted);
        draw_line(
            margin,
            top + height - 52.0,
            margin + width,
            top + height - 52.0,
            1.0,
            muted,
        );
        for section in [
            layout.discipline_panel,
            layout.technique_panel,
            layout.detail_panel,
        ] {
            draw_rectangle(
                section.x,
                section.y,
                section.w,
                section.h,
                UiTheme.surface(),
            );
            draw_rectangle_lines(section.x, section.y, section.w, section.h, 1.0, muted);
        }

        draw_text_bold("COMPÉTENCES", margin + 20.0, top + 37.0, 25.0, cyan);
        let available_points = self.game.player_progression().unspent_skill_points();
        let (points, points_size) = if let Some(notice) = self.level_up_notice {
            (
                format!(
                    "NIVEAU {} ATTEINT · {available_points} POINTS DISPONIBLES",
                    notice.level
                ),
                16,
            )
        } else {
            (format!("{available_points} POINTS DISPONIBLES"), 18)
        };
        let points_width = measure_text_bold(&points, points_size).width;
        draw_text_bold(
            &points,
            margin + width - points_width - 20.0,
            top + 36.0,
            f32::from(points_size),
            amber,
        );

        draw_text(
            "DISCIPLINES",
            layout.discipline_panel.x + 10.0,
            layout.discipline_panel.y + 24.0,
            16.0,
            muted,
        );
        for (index, discipline) in disciplines.iter().enumerate() {
            let row = layout.discipline_rows[index];
            let selected = index == self.skill_discipline_selection;
            if selected {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    Color::from_rgba(18, 58, 63, 230),
                );
                draw_rectangle(row.x, row.y, 3.0, row.h, amber);
            } else if self.menu_focus.hovered == Some(index) {
                draw_rectangle(row.x, row.y, row.w, row.h, UiTheme.surface_raised());
            }
            let availability = self.skill_availability_cache.get(discipline);
            let is_open = availability.is_some_and(|state| state.is_open());
            let name = format!(
                "{}{}",
                if selected { "> " } else { "" },
                self.discipline_name(discipline)
            );
            draw_wrapped_text(
                &name,
                row.x + 7.0,
                row.y + row.h * 0.62,
                row.w - 14.0,
                1,
                16,
                if selected {
                    amber
                } else if is_open {
                    text
                } else {
                    locked
                },
            );
        }

        let technique_x = layout.technique_panel.x + 12.0;
        let Some(discipline) = disciplines.get(self.skill_discipline_selection) else {
            draw_text(
                "Aucune discipline chargée.",
                technique_x,
                layout.technique_panel.y + 28.0,
                20.0,
                locked,
            );
            return;
        };
        let availability = self.skill_availability_cache.get(discipline);
        let discipline_open = availability.is_some_and(|state| state.is_open());
        draw_text_bold(
            self.discipline_name(discipline),
            technique_x,
            layout.technique_panel.y + 25.0,
            21.0,
            amber,
        );
        let learned_count = techniques
            .iter()
            .filter(|id| self.game.player_skills().has_learned(id))
            .count();

        if let Some(id) = techniques.get(self.skill_technique_selection)
            && let Some(definition) = self.game.rules().skills.technique(id)
        {
            let x = layout.detail_panel.x + 13.0;
            let available_width = layout.detail_panel.w - 26.0;
            draw_text(
                "TECHNIQUE SÉLECTIONNÉE",
                x,
                layout.detail_panel.y + 22.0,
                14.0,
                muted,
            );
            let mut y = layout.detail_panel.y + 49.0;
            y = draw_wrapped_text(
                &self.technique_name(id),
                x,
                y,
                available_width,
                2,
                19,
                amber,
            );
            y = draw_wrapped_text(
                &format!(
                    "{} · {}",
                    technical_reference(id),
                    technique_kind_label(definition.kind())
                ),
                x,
                y + 1.0,
                available_width,
                1,
                12,
                cyan,
            );
            let mut learning_requirements = vec![if definition.minimum_level() == 1 {
                "Accessible dès le niveau 1".to_owned()
            } else {
                format!("Niveau {}", definition.minimum_level())
            }];
            learning_requirements.extend(definition.minimum_attributes().map(|requirement| {
                format!(
                    "{} {}",
                    primary_attribute_label(requirement.attribute()),
                    requirement.minimum()
                )
            }));
            if let Some(prerequisite) = definition.prerequisite() {
                learning_requirements
                    .push(format!("{} apprise", self.technique_name(prerequisite)));
            }
            draw_text("POUR L'APPRENDRE", x, y + 5.0, 12.0, muted);
            y = draw_wrapped_text(
                &learning_requirements.join(" · "),
                x,
                y + 25.0,
                available_width,
                2,
                14,
                cyan,
            );
            let description = self
                .texts
                .resolve(DISPLAY_LOCALE, definition.description_key())
                .unwrap_or("Description indisponible.");
            draw_text("DESCRIPTION", x, y + 5.0, 12.0, muted);
            let max_lines = (((layout.detail_panel.h - 205.0) / 42.0).floor() as usize).clamp(1, 5);
            y = draw_wrapped_text(
                description,
                x,
                y + 27.0,
                available_width,
                max_lines,
                16,
                text,
            );
            draw_text("EN ACTION", x, y + 4.0, 12.0, muted);
            y += 27.0;
            let action_timing = definition.preparation_steps().map_or_else(
                || "1 tour".to_owned(),
                |steps| format!("P{}+A1", steps.get()),
            );
            let usage = if definition.improvement()
                == Some(TechniqueImprovement::MeleeCounterattack)
            {
                "Passif. Après une Parade réussie, tente une frappe ordinaire avec la première arme de mêlée équipée si l'attaquant est encore au contact. Ne consomme pas une seconde réaction."
                    .to_owned()
            } else if definition.improvement()
                == Some(TechniqueImprovement::ExtendedRangedOverwatch)
            {
                "Passif. Remplace la ligne de Surveillance par le secteur visible de 90° montré dans l'aperçu. N'ajoute ni portée, ni tir, ni réaction."
                    .to_owned()
            } else if let Some(TechniqueImprovement::PersistentRangedAim {
                retained_accuracy_modifier,
            }) = definition.improvement()
            {
                format!(
                    "Passif. Après le tir de la technique requise, conserve Précision {retained_accuracy_modifier:+} contre la même cible. Bouger, perdre la vue, changer de cible ou entreprendre une autre action qu'un tir simple ou attendre annule ce bonus."
                )
            } else if definition.improvement()
                == Some(TechniqueImprovement::ControlledChargeInertia)
            {
                "Passif. Une autre action permet d'arrêter volontairement la Charge en cours ; une Charge menée jusqu'à sa frappe finale ne provoque plus de récupération."
                    .to_owned()
            } else if let Some(TechniqueImprovement::CoveredApproach {
                optical_difficulty_bonus,
            }) = definition.improvement()
            {
                format!(
                    "Passif. Après un déplacement, un couvert optique réel augmente de {optical_difficulty_bonus} la difficulté de détection pendant la résolution adverse. Ne crée jamais de couvert artificiel."
                )
            } else if let Some(TechniqueImprovement::SilentNeutralization {
                physical_damage_percentage,
                extra_energy_cost,
                noise_reduction,
            }) = definition.improvement()
            {
                format!(
                    "Passif d'Embuscade au contact d'une vulnérabilité connue : {physical_damage_percentage} % des dégâts physiques, +{extra_energy_cost} E et bruit −{noise_reduction}."
                )
            } else if let Some(TechniqueImprovement::DroneAutonomousScout {
                maximum_unknown_steps,
                energy_cost_override,
                additional_bandwidth,
            }) = definition.improvement()
            {
                format!(
                    "Passif de Patrouille bornée : autorise jusqu'à {maximum_unknown_steps} nouvelles cases avant retour, pour {energy_cost_override} E et +{additional_bandwidth} B durant la routine. Le rapport reste daté et n'accorde aucune vision directe."
                )
            } else {
                match definition.action() {
                    Some(TechniqueAction::AnalyzeTarget { range }) => format!(
                        "{action_timing} / 0 E. Analyse une cible visible à portée {range} et révèle son intégrité, son Blindage et ses résistances observables."
                    ),
                    Some(TechniqueAction::AnalyzeMultipleTargets {
                        maximum_targets,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E. Jusqu'à {maximum_targets} cibles visibles : sélection d'abord, puis les plus proches. Même analyse que le prérequis."
                    ),
                    Some(TechniqueAction::ReadMovementTraces { radius }) => format!(
                        "{action_timing} / 0 E. Rayon {radius}. Indices datés ; aucun suivi de leur auteur."
                    ),
                    Some(TechniqueAction::InspectNearbySecrets {
                        radius,
                        detection_bonus,
                    }) => format!(
                        "{action_timing} / 0 E. Inspecte les cases visibles dans un rayon de {radius}, avec Détection +{detection_bonus}. Découvre un dispositif réellement présent sans l'ouvrir ni le désarmer."
                    ),
                    Some(TechniqueAction::AnalyzeNearbyWalls {
                        maximum_tiles,
                        radius,
                    }) => format!(
                        "{action_timing} / 0 E. Analyse jusqu'à {maximum_tiles} parois liées dans un rayon de {radius} et révèle si elles bloquent le passage ou la vision."
                    ),
                    Some(TechniqueAction::AnalyzeThreat { range }) => format!(
                        "{action_timing} / 0 E. Analyse une cible visible à portée {range} et révèle ses capacités offensives connues, sans prédire ses décisions."
                    ),
                    Some(TechniqueAction::DiagnoseEnergy {
                        range,
                        analysis_bonus,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E. Machine visible, portée {range}, Analyse +{analysis_bonus}. Ne révèle que ses réserves et canaux énergétiques réellement simulés."
                    ),
                    Some(TechniqueAction::RepairComponent {
                        durability_restored,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E de fonctionnement. Restaure jusqu'à {durability_restored} durabilité au composant choisi, sans soigner le corps ni recréer un composant détruit."
                    ),
                    Some(TechniqueAction::SalvageComponent) => format!(
                        "{action_timing}. Préserve dans son état réel un composant survivant choisi sur une carcasse adjacente ; il est retiré définitivement de cette carcasse."
                    ),
                    Some(TechniqueAction::DiagnoseComponent {
                        analysis_bonus,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E de fonctionnement. Analyse matérielle +{analysis_bonus} sur un composant accessible ; ne répare ni ne purge un logiciel."
                    ),
                    Some(TechniqueAction::TuneModule {
                        economy_output_percentage,
                        economy_energy_percentage,
                        power_output_percentage,
                        power_energy_percentage,
                    }) => format!(
                        "{action_timing}. Économie : sortie {economy_output_percentage} %, énergie {economy_energy_percentage} %. Puissance : sortie {power_output_percentage} %, énergie {power_energy_percentage} %. Le nouveau réglage remplace l'ancien."
                    ),
                    Some(TechniqueAction::EmergencyRepairComponent {
                        durability_restored,
                    }) => format!(
                        "{action_timing}. Restaure immédiatement {durability_restored} durabilité au plus ; un composant détruit reste détruit."
                    ),
                    Some(TechniqueAction::OverclockModule {
                        output_percentage,
                        usage_energy_percentage,
                        heat_per_use,
                        safe_heat_threshold,
                        maximum_heat_threshold,
                        duration_time_units,
                        activation_energy,
                        durability_damage_when_hot,
                    }) => format!(
                        "{action_timing} / {activation_energy} E. Pendant {duration_time_units} UT : sortie {output_percentage} %, coût énergétique d'usage {usage_energy_percentage} %, +{heat_per_use} chaleur/usage, seuil volontaire {maximum_heat_threshold}. Au-dessus de {safe_heat_threshold}, −{durability_damage_when_hot} durabilité/usage."
                    ),
                    Some(TechniqueAction::BypassComponent {
                        restored_output_percentage,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E de fonctionnement. Rétablit une fonction électrique dégradée à {restored_output_percentage} % en suspendant un second composant réel du même corps."
                    ),
                    Some(TechniqueAction::ReconditionModule {
                        durability_restored,
                    }) => format!(
                        "{action_timing} / atelier sûr. Restaure jusqu'à {durability_restored} durabilité propre sans dépasser le maximum réparable."
                    ),
                    Some(TechniqueAction::AssembleFieldBeacon {
                        integrity,
                        battery_energy,
                        energy_per_phase,
                        noise_intensity,
                    }) => format!(
                        "{action_timing}. Manifeste sur une case libre une balise de {integrity} durabilité, batterie {battery_energy} E, consommation {energy_per_phase} E/phase, bruit {noise_intensity}."
                    ),
                    Some(TechniqueAction::WeaponAttack {
                        required_delivery,
                        physical_damage_percentage,
                        armor_penetration_bonus,
                        accuracy_modifier,
                        energy_cost,
                        recovery_time_units,
                        forced_movement,
                        melee_arc,
                    }) => {
                        let delivery = match required_delivery {
                            project_rl::combat::AttackDelivery::Melee => "mêlée",
                            project_rl::combat::AttackDelivery::Ranged => "tir",
                        };
                        let recovery = recovery_time_units
                            .map_or_else(String::new, |duration| format!(" Puis R{duration}."));
                        let accuracy = match accuracy_modifier.cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                format!(" Précision +{accuracy_modifier}.")
                            }
                            std::cmp::Ordering::Less => format!(" Précision {accuracy_modifier}."),
                            std::cmp::Ordering::Equal => String::new(),
                        };
                        let physical_damage =
                            physical_damage_percentage.map_or_else(String::new, |percentage| {
                                format!(" {percentage} % des dégâts physiques après Impact.")
                            });
                        let penetration = (armor_penetration_bonus > 0)
                            .then(|| {
                                format!(" Pénétration de Blindage +{armor_penetration_bonus}.")
                            })
                            .unwrap_or_default();
                        let displacement = forced_movement.map_or_else(String::new, |movement| {
                        let modifier = match movement.impact_modifier().cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                format!(" + {}", movement.impact_modifier())
                            }
                            std::cmp::Ordering::Less => {
                                format!(" − {}", movement.impact_modifier().unsigned_abs())
                            }
                            std::cmp::Ordering::Equal => String::new(),
                        };
                        format!(
                            " Sur une touche : poussée de {} case(s), Force = Impact{modifier}.",
                            movement.distance()
                        )
                    });
                        let arc = melee_arc.map_or_else(String::new, |arc| {
                            format!(
                                " Arc de {} cases adjacentes ; chaque occupant est exposé.",
                                arc.maximum_cells()
                            )
                        });
                        let on_hit_effect = definition.on_hit_effect().map_or_else(
                            String::new,
                            |effect| {
                                let target = match effect.target_requirement() {
                                    TechniqueTargetRequirement::HasArmor => {
                                        "une cible dotée de Blindage"
                                    }
                                    TechniqueTargetRequirement::HasCompatibleLocomotion => {
                                        "une cible à locomotion compatible"
                                    }
                                    TechniqueTargetRequirement::HasCompatibleSuppressionResponse => {
                                        "une cible sensible à la suppression"
                                    }
                                };
                                let Some(status) = self
                                    .game
                                    .rules()
                                    .statuses
                                    .get(effect.application().status())
                                else {
                                    return format!(
                                        " Sur une touche contre {target} : applique {}.",
                                        status_display_name(effect.application().status())
                                    );
                                };
                                let duration = status.duration_turns().map_or_else(
                                    || "sans limite de durée".to_owned(),
                                    |turns| format!("pendant {turns} UT"),
                                );
                                let resistance = match effect.resistance() {
                                    Some(TechniqueEffectResistance::Stability { intensity }) => {
                                        format!(" Test passif de Stabilité contre intensité {intensity}.")
                                    }
                                    None => String::new(),
                                };
                                let description = status.modifiers().first().map_or_else(
                                    || {
                                        format!(
                                            " Sur une touche contre {target} : applique {} {duration}.",
                                            status_display_name(status.id())
                                        )
                                    },
                                    |modifier| match modifier {
                                        StatusModifier::ArmorFragilization { amount } => format!(
                                            " Sur une touche contre {target} : Fragilisation du Blindage {amount} {duration}, sans cumul ni rafraîchissement."
                                        ),
                                        StatusModifier::Stability { amount } => format!(
                                            " Sur une touche contre {target} : Stabilité {amount:+} {duration}."
                                        ),
                                        StatusModifier::MovementTimeMinimum { time_units } => format!(
                                            " Sur une touche contre {target} : déplacement ordinaire à {time_units} UT minimum {duration}."
                                        ),
                                        StatusModifier::Accuracy { amount } => format!(
                                            " Sur une touche contre {target} : Précision {amount:+} {duration}."
                                        ),
                                    },
                                );
                                format!("{description}{resistance}")
                            },
                        );
                        let engagement_requirement = definition
                            .engagement_requirement()
                            .map_or_else(String::new, |requirement| match requirement {
                                TechniqueEngagementRequirement::TargetHasAnyStatusFamily(_) => {
                                    " Requiert une cible déjà entravée ou immobilisée.".to_owned()
                                }
                                TechniqueEngagementRequirement::TargetHasKnownPhysicalWeakness => {
                                    " Requiert une faiblesse physique réellement identifiée sur cette cible."
                                        .to_owned()
                                }
                            });
                        let cost = if energy_cost == 0 {
                            "coût natif de l'arme".to_owned()
                        } else {
                            format!("coût natif de l'arme + {energy_cost} E")
                        };
                        let cooldown = definition.cooldown().map_or_else(String::new, |duration| {
                            format!(" Recharge : {} phases d'environnement.", duration.get())
                        });
                        format!(
                            "{action_timing} / {cost}. Attaque de {delivery}.{physical_damage}{penetration}{accuracy}{arc}{displacement}{on_hit_effect}{engagement_requirement}{recovery}{cooldown}"
                        )
                    }
                    Some(TechniqueAction::WeaponVolley {
                        projectiles,
                        maximum_targets,
                        maximum_target_separation,
                        accuracy_modifier,
                        energy_cost,
                        requires_automatic_fire,
                    }) => {
                        let automatic = if requires_automatic_fire {
                            " Requiert un mode de tir automatique."
                        } else {
                            ""
                        };
                        let accuracy = match accuracy_modifier.cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                format!(" Précision +{accuracy_modifier} par projectile.")
                            }
                            std::cmp::Ordering::Less => {
                                format!(" Précision {accuracy_modifier} par projectile.")
                            }
                            std::cmp::Ordering::Equal => String::new(),
                        };
                        let energy = if energy_cost == 0 {
                            String::new()
                        } else {
                            format!(" + {energy_cost} E")
                        };
                        let separation = maximum_target_separation.map_or_else(
                            String::new,
                            |maximum| {
                                format!(
                                    " Les cibles choisies doivent rester à {maximum} case(s) les unes des autres."
                                )
                            },
                        );
                        format!(
                            "{action_timing} / {projectiles} projectiles natifs{energy}. Jusqu'à {maximum_targets} cible(s) visible(s), une résolution indépendante par projectile.{accuracy}{automatic}{separation}"
                        )
                    }
                    Some(TechniqueAction::WeaponComponentAttack {
                        required_delivery,
                        accuracy_modifier,
                        energy_cost,
                    }) => {
                        let delivery = match required_delivery {
                            project_rl::combat::AttackDelivery::Melee => "mêlée",
                            project_rl::combat::AttackDelivery::Ranged => "tir",
                        };
                        format!(
                            "{action_timing} / coût natif de l'arme + {energy_cost} E. Attaque de {delivery} contre la durabilité propre d'un composant identifié ; Précision {accuracy_modifier:+}. Les dégâts ne sont pas aussi appliqués aux PV du corps."
                        )
                    }
                    Some(TechniqueAction::WeaponBarrage {
                        stages,
                        cells,
                        accuracy_modifier,
                        energy_cost_per_stage,
                        requires_automatic_fire,
                    }) => format!(
                        "{stages} étapes A1 / {cells} projectiles natifs par étape + {energy_cost_per_stage} E. Une balle par case de la ligne visée ; Précision {accuracy_modifier:+}.{} Toute autre action interrompt les étapes restantes sans annuler les tirs déjà résolus.",
                        if requires_automatic_fire {
                            " Requiert un mode automatique."
                        } else {
                            ""
                        }
                    ),
                    Some(TechniqueAction::PrepareRangedOverwatch { maximum_line_cells }) => {
                        format!(
                            "{action_timing} / un projectile natif au déclenchement. Désigne une ligne de {maximum_line_cells} case(s) dans la portée et la vision. Le premier ennemi perçu qui y entre déclenche un tir simple de réaction."
                        )
                    }
                    Some(TechniqueAction::PrepareMeleeParry {
                        physical_reduction_percentage,
                        trigger_energy_cost,
                    }) => format!(
                        "{action_timing} / {trigger_energy_cost} E au déclenchement. Requiert une arme apte à parer. Réduit de {physical_reduction_percentage} % les dégâts physiques bruts de la prochaine touche de mêlée, avant Blindage."
                    ),
                    Some(TechniqueAction::PrepareMeleeInterception) => format!(
                        "{action_timing} / coût natif de l'arme au déclenchement. Prépare une frappe de mêlée ordinaire avant le retrait volontaire d'une cible au contact. Une poussée ne la déclenche pas et le mouvement continue si la cible survit."
                    ),
                    Some(TechniqueAction::DeployExplosive { deployment, .. }) => format!(
                        "{action_timing}. {} La zone d'effet est prévisualisée avant la manifestation.",
                        explosive_deployment_description(deployment)
                    ),
                    Some(TechniqueAction::NeutralizeExplosive {
                        range,
                        analysis_bonus,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E. Neutralise un dispositif identifié à portée {range}. Bonus d'analyse {analysis_bonus:+}."
                    ),
                    Some(TechniqueAction::RecoverNeutralizedExplosive { range }) => format!(
                        "{action_timing}. Récupère intacte la charge d'un dispositif déjà neutralisé à portée {range}, si l'inventaire peut la recevoir."
                    ),
                    Some(TechniqueAction::TriggerRemoteExplosive {
                        range,
                        energy_cost,
                        bandwidth_required,
                    }) => format!(
                        "{action_timing} / {energy_cost} E · {bandwidth_required} B pendant la commande. Déclenche un récepteur identifié en liaison directe à portée {range}."
                    ),
                    Some(TechniqueAction::ProgramExplosives {
                        range,
                        maximum_devices,
                        minimum_delay,
                        maximum_delay,
                        energy_cost,
                        bandwidth_required,
                    }) => format!(
                        "{action_timing} / {energy_cost} E · {bandwidth_required} B. Programme jusqu'à {maximum_devices} récepteurs connus à portée {range}, avec des délais de {minimum_delay} à {maximum_delay} UT."
                    ),
                    Some(TechniqueAction::CautiousMove {
                        interception_evasion_modifier,
                    }) => format!(
                        "A1. Déplacement d'une case ; Esquive {interception_evasion_modifier:+} uniquement contre les interceptions de mêlée provoquées par ce retrait."
                    ),
                    Some(TechniqueAction::PrepareAnchor {
                        displacement_resistance_bonus,
                    }) => format!(
                        "A1. Prépare un appui donnant Ancrage {displacement_resistance_bonus:+} tant que vous ne changez pas de case."
                    ),
                    Some(TechniqueAction::TraverseSingleObstacle {
                        maximum_distance,
                        energy_cost,
                    }) => format!(
                        "{action_timing} / {energy_cost} E. Traverse un intervalle compatible d'une case vers une arrivée située à {maximum_distance} cases. Les murs et portes ne sont jamais franchissables."
                    ),
                    Some(TechniqueAction::ChargeAttack {
                        minimum_advance,
                        maximum_advance,
                        physical_damage_percentage,
                        energy_per_step,
                        recovery_time_units,
                    }) => format!(
                        "Une avance par tour, de {minimum_advance} à {maximum_advance} cases, à {energy_per_step} E par case ; confirmez encore pour frapper à {physical_damage_percentage} % des dégâts physiques. Récupération {recovery_time_units} tour sans Inertie maîtrisée."
                    ),
                    Some(TechniqueAction::PrepareEvasiveStep {
                        trigger_energy_cost,
                    }) => format!(
                        "A1 de garde. Choisissez une case adjacente ; la prochaine attaque perçue tente d'y déplacer le joueur pour {trigger_energy_cost} E et consomme la réaction commune."
                    ),
                    Some(TechniqueAction::PropelledMove {
                        distance,
                        energy_cost,
                        heat_generated,
                    }) => format!(
                        "A1 / {energy_cost} E · +{heat_generated} H. Parcourt {distance} cases successives ; chaque entrée conserve ses dangers et tirs de Surveillance."
                    ),
                    Some(TechniqueAction::Breakthrough {
                        impact_modifier,
                        energy_cost,
                        recovery_time_units,
                    }) => format!(
                        "{action_timing} / {energy_cost} E. Tente une poussée sans dégâts avec Impact {impact_modifier:+}, puis occupe la case libérée. Récupération {recovery_time_units} tour."
                    ),
                    Some(TechniqueAction::ExtractAlly { energy_cost }) => format!(
                        "{action_timing} / {energy_cost} E. Recule d'une case avec un allié adjacent coopératif et transportable ; aucun des deux ne reçoit d'action gratuite."
                    ),
                    Some(TechniqueAction::SilentMove {
                        noise_reduction,
                        minimum_time_units,
                    }) => format!(
                        "A{minimum_time_units}. Avance d'une case avec une signature acoustique réduite de {noise_reduction}. Incompatible avec Profil réduit."
                    ),
                    Some(TechniqueAction::ToggleEmissionSilence { channel }) => format!(
                        "A1. Active ou désactive le silence des {}. Les actions qui en dépendent restent indisponibles tant que le silence est actif.",
                        signature_channel_label(channel)
                    ),
                    Some(TechniqueAction::ToggleLowProfile {
                        optical_difficulty_bonus,
                        minimum_movement_time_units,
                    }) => format!(
                        "A1. Posture activable : difficulté optique +{optical_difficulty_bonus} sous couvert réel ; déplacements d'au moins {minimum_movement_time_units} UT."
                    ),
                    Some(TechniqueAction::AmbushAttack {
                        accuracy_modifier,
                        physical_damage_percentage,
                    }) => format!(
                        "{action_timing}. Attaque simple contre une cible non alertée : Précision {accuracy_modifier:+}, {physical_damage_percentage} % des dégâts physiques. Si elle vous localise pendant la préparation, les bonus sont perdus."
                    ),
                    Some(TechniqueAction::DeploySoundDecoy {
                        range,
                        intensity,
                        duration_phases,
                        integrity,
                    }) => format!(
                        "A1. Place à portée {range} un leurre physique (intensité {intensity}, durée {duration_phases} phases, intégrité {integrity}) qui attire les observateurs capables de l'entendre."
                    ),
                    Some(TechniqueAction::BreakTrail {
                        energy_cost,
                        maximum_steps,
                        maximum_duration,
                    }) => format!(
                        "A1 / {energy_cost} E. Hors de toute détection optique, masque jusqu'à {maximum_steps} nouvelles traces pendant {maximum_duration} tours ; une nouvelle localisation interrompt l'effet."
                    ),
                    Some(TechniqueAction::CamouflageExplosive {
                        range,
                        optical_difficulty_bonus,
                    }) => format!(
                        "{action_timing}. Génère une couverture donnant +{optical_difficulty_bonus} de difficulté optique à un explosif identifié et non déclenché à portée {range}."
                    ),
                    Some(TechniqueAction::ToggleActiveCamouflage {
                        channel,
                        optical_difficulty_bonus,
                        maximum_duration,
                        activation_energy,
                        upkeep_energy,
                        heat_per_phase,
                    }) => format!(
                        "A1 / {activation_energy} E, puis {upkeep_energy} E et +{heat_per_phase} H par phase. Utilise les {} : difficulté optique +{optical_difficulty_bonus}, au plus {maximum_duration} phases ; une attaque l'interrompt.",
                        signature_channel_label(channel)
                    ),
                    Some(TechniqueAction::ManifestDrone {
                        integrity,
                        energy_capacity,
                        link_range,
                        sensor_radius,
                        bandwidth_required,
                        attack_range,
                        attack_damage,
                        ..
                    }) => format!(
                        "A1. Manifeste sur une case adjacente un drone physique de {integrity} intégrité et {energy_capacity} E. Liaison ≤{link_range}, capteurs {sensor_radius}, {bandwidth_required} B réservée ; attaque électrique {attack_damage} à portée {attack_range}."
                    ),
                    Some(TechniqueAction::DroneEscort {
                        link_range,
                        minimum_distance,
                        maximum_distance,
                        energy_cost,
                    }) => format!(
                        "A1 / {energy_cost} E. Donne à un drone physique en liaison ≤{link_range} un ordre d'escorte à une distance choisie de {minimum_distance} à {maximum_distance} cases."
                    ),
                    Some(TechniqueAction::DronePatrol {
                        link_range,
                        maximum_waypoints,
                        energy_cost,
                    }) => format!(
                        "P1+A1 / {energy_cost} E. Programme jusqu'à {maximum_waypoints} points connus sur un drone en liaison ≤{link_range}. Le drone suit ensuite la route avec ses propres actions."
                    ),
                    Some(TechniqueAction::DroneMobileDecoy {
                        link_range,
                        controller_energy_cost,
                        drone_energy_per_phase,
                        intensity,
                        maximum_duration,
                    }) => format!(
                        "A1 / {controller_energy_cost} E. Un drone équipé rejoint une destination connue en liaison ≤{link_range}, puis dépense {drone_energy_per_phase} E/phase pour émettre un leurre d'intensité {intensity}, au plus {maximum_duration} phases."
                    ),
                    Some(TechniqueAction::DroneCollect {
                        link_range,
                        energy_cost,
                    }) => format!(
                        "A1 / {energy_cost} E. Ordonne à un drone en liaison ≤{link_range} de rejoindre un objet connu, de le charger réellement, puis de revenir pour une remise adjacente."
                    ),
                    Some(TechniqueAction::DroneCoordinateFire {
                        link_range,
                        maximum_drones,
                        energy_cost,
                        transmission_bandwidth,
                    }) => format!(
                        "A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Jusqu'à {maximum_drones} drones en liaison ≤{link_range} viseront la cible lors de leur prochaine attaque ordinaire."
                    ),
                    Some(TechniqueAction::DroneInterpose {
                        link_range,
                        controller_energy_cost,
                        drone_trigger_energy_cost,
                    }) => format!(
                        "A1 / {controller_energy_cost} E. Un drone compatible en liaison ≤{link_range} rejoint l'allié et peut intercepter un tir simple perçu pour {drone_trigger_energy_cost} E propres."
                    ),
                    Some(TechniqueAction::DroneConditionalRoutine {
                        link_range,
                        energy_cost,
                        additional_bandwidth,
                    }) => format!(
                        "P1+A1 / {energy_cost} E, +{additional_bandwidth} B durant la routine. Programme une condition locale bornée sur un drone en liaison ≤{link_range}, sans boucle ni information globale."
                    ),
                    Some(TechniqueAction::DroneCoordinatedDeployment {
                        link_range,
                        maximum_drones,
                        energy_cost,
                        transmission_bandwidth,
                    }) => format!(
                        "P1+A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Assigne à jusqu'à {maximum_drones} drones en liaison ≤{link_range} des destinations et rôles réels, sans déplacement instantané."
                    ),
                    Some(TechniqueAction::DroneEmergencyReturn {
                        link_range,
                        maximum_drones,
                        energy_cost,
                        transmission_bandwidth,
                        duration_phases,
                    }) => format!(
                        "A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Jusqu'à {maximum_drones} drones en liaison ≤{link_range} reviennent durant {duration_phases} phases, puis attendent."
                    ),
                    Some(TechniqueAction::ProbeInterface {
                        range,
                        analysis_bonus,
                        energy_cost,
                        audit_delay,
                    }) => format!(
                        "A1 / {energy_cost} E. Sonde une interface connue en liaison ≤{range}, Analyse +{analysis_bonus}. Crée une trace locale dont l'audit de référence arrive après {audit_delay} UT."
                    ),
                    Some(TechniqueAction::ForceElectronicLock {
                        range,
                        energy_cost,
                        bandwidth_required,
                        failure_hardening_duration,
                        ..
                    }) => format!(
                        "P1+A1 / {energy_cost} E, {bandwidth_required} B pendant la procédure. Tente une ouverture électronique en liaison ≤{range}. Un échec durcit l'interface jusqu'à +20 pendant {failure_hardening_duration} UT."
                    ),
                    Some(TechniqueAction::ExtractData { range, energy_cost }) => format!(
                        "P1+A1 / {energy_cost} E. Extrait d'une session de lecture à portée {range} un lot persistant, daté et rattaché à sa source."
                    ),
                    Some(TechniqueAction::SpoofAuthorization {
                        range,
                        energy_cost,
                        duration_time_units,
                    }) => format!(
                        "A1 / {energy_cost} E, 1 B de session. Utilise un identifiant déjà extrait à portée {range} et accorde seulement les droits locaux pendant {duration_time_units} UT."
                    ),
                    Some(TechniqueAction::DivertDevice {
                        range,
                        energy_cost,
                        additional_bandwidth,
                        duration_time_units,
                    }) => format!(
                        "A1 / {energy_cost} E, +{additional_bandwidth} B. Maintient une consigne simple autorisée à portée {range} pendant {duration_time_units} UT ; une reprise adverse reste possible."
                    ),
                    Some(TechniqueAction::SuspendDigitalRoutine {
                        range,
                        energy_cost,
                        duration_time_units,
                        repeat_protection_time_units,
                    }) => format!(
                        "A1 / {energy_cost} E, recharge 3. Suspend une routine nommée à portée {range} pendant {duration_time_units} UT, puis protège cette famille {repeat_protection_time_units} UT."
                    ),
                    Some(TechniqueAction::MaintainBackdoor {
                        range,
                        installation_energy_cost,
                        reconnection_energy_cost,
                        maximum_backdoors,
                        session_duration_time_units,
                    }) => format!(
                        "P1+A1 / {installation_energy_cost} E à l'installation, {reconnection_energy_cost} E à la reconnexion et 1 B de session. Jusqu'à {maximum_backdoors} accès dormants, liaison ≤{range}, session {session_duration_time_units} UT."
                    ),
                    Some(TechniqueAction::FalsifySecurityTrace { range, energy_cost }) => format!(
                        "P1+A1 / {energy_cost} E. Avec droit de modification à portée {range}, falsifie une trace identifiée avant son audit ; copies, témoins et transmissions subsistent."
                    ),
                    Some(TechniqueAction::DivertSubnet {
                        range,
                        maximum_devices,
                        energy_cost,
                        bandwidth_per_device,
                        duration_time_units,
                    }) => format!(
                        "P2+A1 / {energy_cost} E, {bandwidth_per_device} B par dispositif. Commande jusqu'à {maximum_devices} interfaces autorisées et joignables à portée {range} pendant {duration_time_units} UT."
                    ),
                    Some(TechniqueAction::LockDeviceControl {
                        range,
                        energy_cost,
                        additional_bandwidth,
                        duration_time_units,
                    }) => format!(
                        "A1 / {energy_cost} E, +{additional_bandwidth} B, recharge 4. Bloque pendant {duration_time_units} UT les reprises ordinaires d'un dispositif déjà détourné à portée {range}."
                    ),
                    Some(TechniqueAction::ElectronicPulse {
                        radius,
                        damage,
                        energy_cost,
                        heat_generated,
                        disruption_intensity,
                        directional,
                        filter_identified_allies,
                        bandwidth_required,
                    }) => format!(
                        "A1 / {energy_cost} E, +{heat_generated} H{}{}. {} de rayon {radius} : {damage} dégâts électriques aux systèmes compatibles ; interruption d'intensité {disruption_intensity}. Les parois bloquent la propagation.",
                        (bandwidth_required > 0)
                            .then(|| format!(", {bandwidth_required} B pendant l'émission"))
                            .unwrap_or_default(),
                        if filter_identified_allies {
                            ", alliés identifiés filtrés"
                        } else {
                            ""
                        },
                        if directional {
                            "Cône de 90°"
                        } else {
                            "Disque"
                        },
                    ),
                    Some(TechniqueAction::ImplantOverheat {
                        range,
                        energy_cost,
                        heat_generated,
                        bandwidth_required,
                        heat_per_tick,
                        dissipation_penalty,
                        duration_time_units,
                        ..
                    }) => format!(
                        "A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Machine visible à portée {range} : test logiciel, puis +{heat_per_tick} H et dissipation −{dissipation_penalty} durant {duration_time_units} UT."
                    ),
                    Some(TechniqueAction::MaintainJamming {
                        radius,
                        penalty,
                        activation_energy,
                        energy_per_phase,
                        heat_per_phase,
                        bandwidth_required,
                        maximum_duration,
                    }) => format!(
                        "A1 / {activation_energy} E, puis {energy_per_phase} E et +{heat_per_phase} H par phase, {bandwidth_required} B. Brouille un canal choisi dans un rayon de {radius} avec un malus de {penalty}, au plus {maximum_duration} UT."
                    ),
                    Some(TechniqueAction::PurgeHostileProgram {
                        range,
                        energy_cost,
                        intrusion_bonus,
                    }) => format!(
                        "A1 / {energy_cost} E. À portée {range}, tente de retirer un programme hostile précis avec un bonus logiciel de {intrusion_bonus:+}."
                    ),
                    Some(TechniqueAction::ElectronicCascade {
                        range,
                        jump_range,
                        maximum_targets,
                        damage_by_target,
                        energy_cost,
                        heat_generated,
                    }) => format!(
                        "A1 / {energy_cost} E, +{heat_generated} H. Première machine visible à portée {range}, puis jusqu'à {maximum_targets} cibles distinctes séparées de {jump_range} cases : dégâts électriques successifs {}. Chaque saut exige une liaison libre.",
                        damage_sequence_label(
                            &damage_by_target
                                [..usize::from(maximum_targets).min(damage_by_target.len())]
                        )
                    ),
                    Some(TechniqueAction::ImplantInfection {
                        range,
                        propagation_range,
                        energy_cost,
                        heat_generated,
                        bandwidth_required,
                        thermal_damage_per_tick,
                        ticks_per_host,
                        maximum_hosts,
                        transmissions_per_host,
                        campaign_duration,
                        ..
                    }) => format!(
                        "A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Infection logicielle à portée {range} : {thermal_damage_per_tick} thermiques pendant {ticks_per_host} UT par hôte, jusqu'à {maximum_hosts} hôtes. Chaque hôte tente {transmissions_per_host} transmission(s) distincte(s) à portée {propagation_range}, campagne {campaign_duration} UT."
                    ),
                    Some(TechniqueAction::DeploySaturationBeacon {
                        radius,
                        damage,
                        duration_time_units,
                        integrity,
                        battery_energy,
                        energy_per_phase,
                        manual_activation,
                        activation_energy,
                        activation_bandwidth,
                        activation_link_range,
                    }) => format!(
                        "P1+A1 / une balise. Acteur physique de {integrity} intégrité sur une case adjacente : disque bloqué par les murs, {damage} électriques, rayon {radius}, {duration_time_units} UT, batterie {battery_energy} E à {energy_per_phase} E/phase.{}",
                        if manual_activation {
                            format!(
                                " Pose inactive ; activation ultérieure à portée {activation_link_range} pour {activation_energy} E et {activation_bandwidth} B temporaire."
                            )
                        } else {
                            String::new()
                        }
                    ),
                    Some(TechniqueAction::ImplantImplosion {
                        range,
                        energy_cost,
                        heat_generated,
                        bandwidth_required,
                        minimum_stored_energy,
                        reserved_energy,
                        delay_time_units,
                        radius,
                        physical_damage,
                        thermal_damage,
                        ..
                    }) => format!(
                        "P1+A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Réserve {reserved_energy} E dans une machine possédant au moins {minimum_stored_energy} E à portée {range}, puis annonce une détonation après {delay_time_units} UT : rayon {radius}, {physical_damage} physiques + {thermal_damage} thermiques."
                    ),
                    None => "Fonction indisponible dans cette version.".to_owned(),
                }
            };
            let usage = definition.activation_cost().map_or(usage.clone(), |cost| {
                let mut resources = Vec::new();
                if cost.energy() > 0 {
                    resources.push(format!("{} E", cost.energy()));
                }
                if cost.heat() > 0 {
                    resources.push(format!("+{} H", cost.heat()));
                }
                if cost.persistent_bandwidth() > 0 {
                    resources.push(format!("{} B réservée", cost.persistent_bandwidth()));
                }
                if let Some(limit) = cost.active_limit() {
                    resources.push(format!("limite active {limit}"));
                }
                format!("COÛT INTRINSÈQUE : {}. {usage}", resources.join(" · "))
            });
            let remaining_lines = (((layout.detail_panel.y + layout.detail_panel.h - 10.0 - y)
                / 19.0)
                .floor()
                .max(0.0)) as usize;
            draw_wrapped_text(&usage, x, y, available_width, remaining_lines, 14, muted);
        }
        for (index, row) in &layout.technique_rows {
            let Some(technique_id) = techniques.get(*index) else {
                continue;
            };
            let row_y = row.y + 25.0;
            let selected = *index == self.skill_technique_selection;
            if selected {
                draw_rectangle(
                    row.x,
                    row.y,
                    row.w,
                    row.h,
                    Color::from_rgba(18, 58, 63, 190),
                );
            } else if self.menu_focus.hovered == Some(disciplines.len() + *index) {
                draw_rectangle(row.x, row.y, row.w, row.h, UiTheme.surface_raised());
            }
            let Some(technique) = self.game.rules().skills.technique(technique_id) else {
                continue;
            };
            let learned = self.game.player_skills().has_learned(technique_id);
            let available_in_version =
                availability.is_some_and(|state| state.available.contains(technique_id));
            let next_choice_number = learned_count.saturating_add(1);
            let status = if learned {
                format!(
                    "Apprise · {} pour utiliser",
                    self.controls.label(Action::Use)
                )
            } else if !discipline_open {
                "Discipline indisponible".to_owned()
            } else if !available_in_version {
                "Technique à venir".to_owned()
            } else if let Some(prerequisite) = technique.prerequisite()
                && !self.game.player_skills().has_learned(prerequisite)
            {
                format!("Demande : {}", self.technique_name(prerequisite))
            } else if technique.minimum_level() > self.game.player_progression().level() {
                format!("Niveau {}", technique.minimum_level())
            } else if let Some(requirement) =
                technique.unmet_attribute_requirement(self.game.player_primary_attributes())
            {
                format!(
                    "{} {}",
                    primary_attribute_label(requirement.attribute()),
                    requirement.minimum()
                )
            } else {
                self.game
                    .rules()
                    .skill_progression
                    .cost_for_choice_number(next_choice_number)
                    .map(|cost| {
                        if self.game.player_progression().unspent_skill_points() < u32::from(cost) {
                            format!("{cost} pt · Points manquants")
                        } else {
                            format!("{cost} pt · Prête à apprendre")
                        }
                    })
                    .unwrap_or_else(|| "Coût indisponible".to_owned())
            };
            draw_wrapped_text(
                &format!(
                    "{} {}",
                    if selected { ">" } else { " " },
                    self.technique_name(technique_id)
                ),
                row.x + 8.0,
                row_y,
                row.w - 16.0,
                1,
                16,
                if selected { amber } else { text },
            );
            draw_wrapped_text(
                &format!(
                    "{} · {} · {status}",
                    technical_reference(technique_id),
                    technique_kind_label(technique.kind())
                ),
                row.x + 22.0,
                row_y + 15.0,
                row.w - 30.0,
                1,
                12,
                if learned || (discipline_open && available_in_version) {
                    cyan
                } else {
                    locked
                },
            );
        }
        if techniques.len() > layout.technique_rows.len() {
            let track_top = layout.technique_panel.y + 46.0;
            let track = Rect::new(
                layout.technique_panel.x + layout.technique_panel.w - 5.0,
                track_top,
                3.0,
                (layout.technique_panel.y + layout.technique_panel.h - track_top - 7.0).max(20.0),
            );
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            let visible = layout.technique_rows.len();
            let thumb_h = (track.h * visible as f32 / techniques.len() as f32).clamp(20.0, track.h);
            let maximum_first = techniques.len().saturating_sub(visible).max(1);
            let thumb_y = track.y
                + (track.h - thumb_h) * layout.first_technique as f32 / maximum_first as f32;
            draw_rectangle(track.x, thumb_y, track.w, thumb_h, cyan);
        }

        let learned = techniques
            .get(self.skill_technique_selection)
            .is_some_and(|id| self.game.player_skills().has_learned(id));
        let discipline_open = availability.is_some_and(|state| state.is_open());
        for (index, (rect, (label, enabled))) in layout
            .actions
            .iter()
            .zip([
                ("Apprendre", discipline_open && !learned),
                ("Utiliser", learned),
                ("Fermer", true),
            ])
            .enumerate()
        {
            UiTheme.button(
                *rect,
                label,
                self.menu_focus.hovered == Some(disciplines.len() + techniques.len() + index),
                false,
                enabled,
                if index < 2 {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Secondary
                },
            );
        }
        if !self.skill_message.is_empty() {
            draw_wrapped_text(
                &self.skill_message,
                layout.discipline_panel.x + 8.0,
                top + height - 18.0,
                layout.discipline_panel.w - 16.0,
                1,
                14,
                amber,
            );
        }
    }

    fn equipped_weapon_label(&self, item: ItemInstanceId) -> String {
        let slots = self
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
        if slots.is_empty() {
            "Rangé".to_owned()
        } else if slots == (usize::from(self.active_weapon_slot) + 1).to_string() {
            format!("Emplacement actif {slots}")
        } else {
            format!("Emplacement {slots}")
        }
    }

    pub const fn should_quit(&self) -> bool {
        self.quit_requested
    }

    pub fn request_quit(&mut self) {
        if self.quit_requested {
            return;
        }
        self.graphics.revert();
        if self.is_main_menu_flow() || self.game.status() != RunStatus::Active {
            // Preserve a pending suspension, and never resurrect a finished run.
            self.quit_requested = true;
            return;
        }
        self.open_menu(MenuScreen::Pause);
        match self.suspend_run() {
            Ok(()) => self.quit_requested = true,
            Err(error) => {
                eprintln!("[SUSPENSION] Save failed: {error}");
                self.menu_message = "Sauvegarde impossible. La partie reste ouverte ; vérifiez que le jeu peut écrire ses fichiers puis réessayez.".to_owned();
            }
        }
    }

    fn has_suspension(&self) -> bool {
        self.suspension_path.try_exists().unwrap_or(false)
    }

    fn is_main_menu_flow(&self) -> bool {
        self.character_creation.is_some()
            || matches!(self.menu, MenuScreen::Main | MenuScreen::ConfirmNewRun)
            || (self.options_return == MenuScreen::Main
                && matches!(
                    self.menu,
                    MenuScreen::Options
                        | MenuScreen::Controls
                        | MenuScreen::Graphics
                        | MenuScreen::ConfirmGraphics
                ))
    }

    fn menu_back(&self) -> MenuScreen {
        match self.menu {
            MenuScreen::Options if self.options_return == MenuScreen::Main => MenuScreen::Main,
            MenuScreen::Main | MenuScreen::ConfirmNewRun => MenuScreen::Main,
            other => other.back(),
        }
    }

    fn begin_character_creation(&mut self, replace_suspension: bool) -> Result<(), String> {
        let first = self
            .character_classes
            .iter()
            .next()
            .map(|(_, definition)| definition.recommended_attributes())
            .ok_or("Aucun protocole de restauration n'est disponible.")?;
        self.open_menu(MenuScreen::Hidden);
        self.inventory_open = false;
        self.character_open = false;
        self.skills_open = false;
        self.level_up_notice = None;
        self.report_open = false;
        self.legend_open = false;
        self.character_creation = Some(CharacterCreation {
            stage: CharacterCreationStage::Protocol,
            selected_class: 0,
            selected_attribute: 0,
            attributes: first,
            replace_suspension,
            message: String::new(),
            hovered: None,
        });
        Ok(())
    }

    fn update_character_creation(&mut self, input: &InputFrame) {
        let Some(mut creation) = self.character_creation.take() else {
            return;
        };
        let class_count = self.character_classes.iter().count();
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = CharacterCreationLayout::new(width, height, class_count);
        creation.hovered = input
            .pointer
            .and_then(|pointer| layout.hit(pointer, creation.stage));
        self.menu_focus.hovered = creation.hovered.map(|_| 0);

        if input.pause {
            if creation.stage == CharacterCreationStage::Attributes {
                creation.stage = CharacterCreationStage::Protocol;
                creation.message.clear();
                self.character_creation = Some(creation);
            } else {
                self.open_menu(MenuScreen::Main);
            }
            return;
        }

        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let activate = self.controls.pressed(Action::Learn, input) && !clicked;
        match creation.stage {
            CharacterCreationStage::Protocol => {
                let previous = self.controls.pressed(Action::MenuUp, input)
                    || self.controls.pressed(Action::MenuLeft, input);
                let next = self.controls.pressed(Action::MenuDown, input)
                    || self.controls.pressed(Action::MenuRight, input);
                if previous && class_count > 0 {
                    creation.selected_class = creation
                        .selected_class
                        .checked_sub(1)
                        .unwrap_or(class_count - 1);
                } else if next && class_count > 0 {
                    creation.selected_class = (creation.selected_class + 1) % class_count;
                }
                if clicked
                    && let Some((index, _)) =
                        layout.class_rows.iter().enumerate().find(|(_, row)| {
                            input
                                .pointer
                                .is_some_and(|point| row.contains(point.into()))
                        })
                {
                    creation.selected_class = index;
                }
                let cancel = clicked
                    && input
                        .pointer
                        .is_some_and(|point| layout.cancel.contains(point.into()));
                if cancel {
                    self.open_menu(MenuScreen::Main);
                    return;
                }
                let continue_requested = activate
                    || (clicked
                        && input
                            .pointer
                            .is_some_and(|point| layout.continue_button.contains(point.into())));
                if continue_requested
                    && let Some(attributes) = self
                        .character_classes
                        .iter()
                        .nth(creation.selected_class)
                        .map(|(_, definition)| definition.recommended_attributes())
                {
                    creation.attributes = attributes;
                    creation.stage = CharacterCreationStage::Attributes;
                    creation.message =
                        "Profil recommandé appliqué ; redistribuez librement les 28 points."
                            .to_owned();
                }
            }
            CharacterCreationStage::Attributes => {
                let previous = self.controls.pressed(Action::MenuUp, input);
                let next = self.controls.pressed(Action::MenuDown, input);
                if previous {
                    creation.selected_attribute = creation.selected_attribute.saturating_sub(1);
                } else if next {
                    creation.selected_attribute =
                        (creation.selected_attribute + 1).min(PrimaryAttribute::ALL.len() - 1);
                }
                if self.controls.pressed(Action::MenuLeft, input) {
                    self.adjust_creation_attribute(&mut creation, -1);
                } else if self.controls.pressed(Action::MenuRight, input) {
                    self.adjust_creation_attribute(&mut creation, 1);
                }
                if clicked && let Some(pointer) = input.pointer {
                    if let Some(index) = layout
                        .attribute_minus
                        .iter()
                        .position(|button| button.contains(pointer.into()))
                    {
                        creation.selected_attribute = index;
                        self.adjust_creation_attribute(&mut creation, -1);
                    } else if let Some(index) = layout
                        .attribute_plus
                        .iter()
                        .position(|button| button.contains(pointer.into()))
                    {
                        creation.selected_attribute = index;
                        self.adjust_creation_attribute(&mut creation, 1);
                    } else if layout.preset.contains(pointer.into()) {
                        if let Some(attributes) = self
                            .character_classes
                            .iter()
                            .nth(creation.selected_class)
                            .map(|(_, definition)| definition.recommended_attributes())
                        {
                            creation.attributes = attributes;
                            creation.message = "Profil recommandé rétabli.".to_owned();
                        }
                    } else if let Some(index) = layout
                        .attribute_rows
                        .iter()
                        .position(|row| row.contains(pointer.into()))
                    {
                        creation.selected_attribute = index;
                    }
                }
                let cancel = clicked
                    && input
                        .pointer
                        .is_some_and(|point| layout.cancel.contains(point.into()));
                if cancel {
                    creation.stage = CharacterCreationStage::Protocol;
                    creation.message.clear();
                } else {
                    let start = activate
                        || (clicked
                            && input.pointer.is_some_and(|point| {
                                layout.continue_button.contains(point.into())
                            }));
                    if start {
                        match self.rebuild_run_with_character_class(&creation) {
                            Ok(()) => return,
                            Err(error) => {
                                eprintln!("[NEW RUN] Character creation failed: {error}");
                                creation.message =
                                    "Impossible de commencer cette partie pour le moment."
                                        .to_owned();
                            }
                        }
                    }
                }
            }
        }
        self.character_creation = Some(creation);
    }

    fn adjust_creation_attribute(&self, creation: &mut CharacterCreation, delta: i8) {
        let attribute = PrimaryAttribute::ALL[creation.selected_attribute];
        let rules = self.rules.primary_attribute_rules;
        let current = creation.attributes.value(attribute);
        let next = if delta < 0 {
            if current <= rules.creation_minimum {
                creation.message = format!(
                    "{} ne peut pas descendre sous {}.",
                    primary_attribute_label(attribute),
                    rules.creation_minimum
                );
                return;
            }
            current - 1
        } else {
            if current >= rules.creation_maximum {
                creation.message = format!(
                    "{} ne peut pas dépasser {} à la création.",
                    primary_attribute_label(attribute),
                    rules.creation_maximum
                );
                return;
            }
            if creation.attributes.total() >= rules.creation_total {
                creation.message = "Retirez d'abord un point à un autre attribut.".to_owned();
                return;
            }
            current + 1
        };
        creation.attributes = creation.attributes.with_value(attribute, next);
        creation.message.clear();
    }

    fn rebuild_run_with_character_class(
        &mut self,
        creation: &CharacterCreation,
    ) -> Result<(), String> {
        let (class_id, definition) = self
            .character_classes
            .iter()
            .nth(creation.selected_class)
            .map(|(id, definition)| (id.clone(), definition.clone()))
            .ok_or("Protocole de restauration sélectionné introuvable.")?;
        creation
            .attributes
            .validate_for_creation(self.rules.primary_attribute_rules)
            .map_err(|error| format!("Répartition incomplète : {error}"))?;
        let mut rules = self.rules.clone();
        definition
            .apply_to_rules(&mut rules, creation.attributes)
            .map_err(|error| error.to_string())?;

        // The complete run is built before the optional old suspension is consumed.
        let mut fresh = Self::from_seed(
            INITIAL_SEED,
            rules,
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        fresh.character_class = Some(class_id);
        fresh.active_weapon_slot = fresh
            .game
            .rules()
            .player_starting_equipment
            .iter()
            .position(Option::is_some)
            .unwrap_or(0) as u8;
        if creation.replace_suspension
            && self
                .suspension_path
                .try_exists()
                .map_err(|error| format!("Impossible de vérifier la suspension : {error}"))?
        {
            std::fs::remove_file(&self.suspension_path)
                .map_err(|error| format!("Impossible de remplacer la suspension : {error}"))?;
        }
        fresh.controls = self.controls.clone();
        fresh.controls_path = self.controls_path.clone();
        fresh.options_message = self.options_message.clone();
        fresh.graphics = self.graphics.clone();
        fresh.suspension_path = self.suspension_path.clone();
        fresh.session_lock = self.session_lock.take();
        fresh.options_return = MenuScreen::Pause;
        fresh.open_menu(MenuScreen::Hidden);
        *self = fresh;
        Ok(())
    }

    fn rebuild_run(
        &mut self,
        destination: MenuScreen,
        replace_suspension: bool,
    ) -> Result<(), String> {
        // Build the complete fresh state before consuming an existing suspension.
        let mut fresh = Self::from_seed(
            INITIAL_SEED,
            self.rules.clone(),
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        if replace_suspension
            && self
                .suspension_path
                .try_exists()
                .map_err(|error| format!("Impossible de vérifier la suspension : {error}"))?
        {
            std::fs::remove_file(&self.suspension_path)
                .map_err(|error| format!("Impossible de remplacer la suspension : {error}"))?;
        }
        fresh.controls = self.controls.clone();
        fresh.controls_path = self.controls_path.clone();
        fresh.options_message = self.options_message.clone();
        fresh.graphics = self.graphics.clone();
        fresh.suspension_path = self.suspension_path.clone();
        fresh.session_lock = self.session_lock.take();
        fresh.options_return = MenuScreen::Pause;
        fresh.open_menu(destination);
        *self = fresh;
        Ok(())
    }

    fn open_menu(&mut self, menu: MenuScreen) {
        if menu != MenuScreen::ConfirmGraphics {
            self.graphics.revert();
        }
        if menu == MenuScreen::Graphics {
            self.graphics.draft = self.graphics.active;
        }
        self.menu = menu;
        self.menu_selection = if menu == MenuScreen::Main && !self.has_suspension() {
            1
        } else {
            0
        };
        self.menu_message.clear();
        self.rebinding = false;
        self.menu_focus.reset();
    }

    fn update_menu(&mut self, input: &InputFrame) {
        let count = self.menu.buttons().len();
        if count == 0 {
            return;
        }
        let hovered = input
            .pointer
            .zip(input.viewport)
            .and_then(|(pointer, (width, height))| {
                MenuLayout::new(width, height, count).hit(pointer)
            })
            .filter(|index| self.menu_row_enabled(*index));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let up = self.controls.pressed(Action::MenuUp, input);
        let down = self.controls.pressed(Action::MenuDown, input);
        let activate = self.controls.pressed(Action::Learn, input) && !clicked;
        if up && !clicked {
            if let Some(index) = (0..self.menu_selection)
                .rev()
                .find(|index| self.menu_row_enabled(*index))
            {
                self.menu_selection = index;
            }
        } else if down
            && !clicked
            && let Some(index) =
                (self.menu_selection + 1..count).find(|index| self.menu_row_enabled(*index))
        {
            self.menu_selection = index;
        }
        let left = self.controls.pressed(Action::MenuLeft, input);
        let right = self.controls.pressed(Action::MenuRight, input);
        self.menu_focus.update(
            hovered,
            &mut self.menu_selection,
            clicked,
            up || down || activate || left || right,
        );
        if self.menu == MenuScreen::Graphics && (left || right) && !clicked {
            self.graphics.draft.cycle(self.menu_selection, right);
            return;
        }
        if ((clicked && hovered.is_some()) || activate)
            && self.menu_row_enabled(self.menu_selection)
        {
            match (self.menu, self.menu_selection) {
                (MenuScreen::Main, 0) => {
                    if let Err(error) = self.resume_run() {
                        eprintln!("[SUSPENSION] Resume failed: {error}");
                        self.menu_message = "Reprise impossible : la sauvegarde ne peut pas être chargée. Elle a été conservée intacte.".to_owned();
                    }
                }
                (MenuScreen::Main, 1) => {
                    if self.has_suspension() {
                        self.open_menu(MenuScreen::ConfirmNewRun);
                    } else if let Err(error) = self.begin_character_creation(false) {
                        eprintln!("[NEW RUN] Character creation failed: {error}");
                        self.menu_message =
                            "Nouvelle partie impossible : veuillez réessayer.".to_owned();
                    }
                }
                (MenuScreen::Main, 2) => {
                    self.options_return = MenuScreen::Main;
                    self.open_menu(MenuScreen::Options);
                }
                (MenuScreen::Main, 3) => self.quit_requested = true,
                (MenuScreen::Pause, 0) => self.open_menu(MenuScreen::Hidden),
                (MenuScreen::Pause, 1) => {
                    self.options_return = MenuScreen::Pause;
                    self.open_menu(MenuScreen::Options);
                }
                (MenuScreen::Pause, 2) => self.request_quit(),
                (MenuScreen::Pause, 3) => self.open_menu(MenuScreen::ConfirmAbandon),
                (MenuScreen::Options, 0) => self.open_menu(MenuScreen::Controls),
                (MenuScreen::Options, 1) => self.open_menu(MenuScreen::Graphics),
                (MenuScreen::Options, 2) => self.open_menu(self.options_return),
                (MenuScreen::ConfirmAbandon, 0) => self.open_menu(MenuScreen::Pause),
                (MenuScreen::Graphics, row @ 0..=2) => self.graphics.draft.cycle(row, true),
                (MenuScreen::Graphics, row @ 4..=6) => self.graphics.draft.cycle(row, true),
                (MenuScreen::Graphics, 7) => self.graphics.draft = GraphicsSettings::default(),
                (MenuScreen::Graphics, 8) => {
                    if self.graphics.begin_preview() {
                        self.open_menu(MenuScreen::ConfirmGraphics);
                    }
                }
                (MenuScreen::Graphics, 9) => self.open_menu(MenuScreen::Options),
                (MenuScreen::ConfirmGraphics, 0) => self.open_menu(MenuScreen::Graphics),
                (MenuScreen::ConfirmGraphics, 1) => {
                    self.graphics.confirm();
                    self.open_menu(MenuScreen::Graphics);
                }
                (MenuScreen::ConfirmAbandon, 1) => {
                    if let Err(error) = self.rebuild_run(MenuScreen::Main, false) {
                        eprintln!("[NEW RUN] Return to main menu failed: {error}");
                        self.menu_message =
                            "Retour au menu impossible : veuillez réessayer.".to_owned();
                    }
                }
                (MenuScreen::ConfirmNewRun, 0) => self.open_menu(MenuScreen::Main),
                (MenuScreen::ConfirmNewRun, 1) => {
                    if let Err(error) = self.begin_character_creation(true) {
                        eprintln!("[NEW RUN] Replacement failed: {error}");
                        self.menu_message =
                            "Nouvelle partie impossible : la partie suspendue a été conservée."
                                .to_owned();
                    }
                }
                _ => {}
            }
        }
    }

    fn menu_row_enabled(&self, index: usize) -> bool {
        if self.menu == MenuScreen::Pause && index == 3 && self.game.status() != RunStatus::Active {
            return false;
        }
        if self.menu == MenuScreen::Main && index == 0 && !self.has_suspension() {
            return false;
        }
        self.menu != MenuScreen::Graphics
            || (index != 3 && (index != 1 || self.graphics.draft.mode == WindowMode::Windowed))
    }

    fn menu_labels(&self) -> Vec<String> {
        if self.menu == MenuScreen::Main {
            vec![
                if self.has_suspension() {
                    if self.menu_message.starts_with("Reprise impossible : ") {
                        "Réessayer la reprise".to_owned()
                    } else {
                        "Reprendre la partie".to_owned()
                    }
                } else {
                    "Reprendre la partie (indisponible)".to_owned()
                },
                "Nouvelle partie".to_owned(),
                "Options".to_owned(),
                "Quitter".to_owned(),
            ]
        } else if self.menu == MenuScreen::Pause && self.game.status() != RunStatus::Active {
            vec![
                "Reprendre".to_owned(),
                "Options".to_owned(),
                "Quitter".to_owned(),
                "Abandonner la partie (terminée)".to_owned(),
            ]
        } else if self.menu == MenuScreen::Graphics {
            let settings = self.graphics.draft;
            vec![
                format!("Mode : {}", settings.mode.label()),
                if settings.mode == WindowMode::Windowed {
                    format!(
                        "Résolution : {} × {}",
                        settings.windowed_size[0], settings.windowed_size[1]
                    )
                } else {
                    "Résolution : bureau (automatique)".to_owned()
                },
                format!("Taille de l'interface : {} %", settings.ui_scale_percent),
                "Rendu : Terminal à glyphes (textures à venir)".to_owned(),
                format!("Taille des cases : {} px", settings.world_cell_px),
                format!(
                    "Contraste renforcé : {}",
                    if settings.high_contrast { "oui" } else { "non" }
                ),
                format!(
                    "Animations réduites : {}",
                    if settings.reduced_motion {
                        "oui"
                    } else {
                        "non"
                    }
                ),
                "Valeurs par défaut".to_owned(),
                "Appliquer les modifications".to_owned(),
                "Retour sans appliquer".to_owned(),
            ]
        } else {
            self.menu
                .buttons()
                .iter()
                .map(|label| (*label).to_owned())
                .collect()
        }
    }

    fn draw_menu(&self) {
        let theme = UiTheme;
        let buttons = self.menu_labels();
        let layout = MenuLayout::new(self.ui_width(), self.ui_height(), buttons.len());
        let panel = layout.panel;
        let graphics_note = if self.menu == MenuScreen::ConfirmGraphics {
            format!(
                "Retour automatique dans {} s sans confirmation. Échap rétablit les anciens réglages.",
                self.graphics.remaining_seconds()
            )
        } else if self.graphics.draft != self.graphics.active {
            "Modifications non appliquées. L'interface s'adapte si la fenêtre est trop petite."
                .to_owned()
        } else if self.ui_scale() * 100.0 + 0.1 < self.graphics.active.ui_scale_percent as f32 {
            format!(
                "{} Échelle adaptée à {:.0} % pour garder les menus accessibles.",
                self.graphics.message,
                self.ui_scale() * 100.0
            )
        } else {
            self.graphics.message.clone()
        };
        let menu_error = matches!(
            self.menu,
            MenuScreen::Main | MenuScreen::ConfirmNewRun | MenuScreen::ConfirmAbandon
        ) && !self.menu_message.is_empty();
        let error_title = if self.menu_message.starts_with("Reprise impossible : ") {
            "REPRISE IMPOSSIBLE"
        } else {
            "ACTION IMPOSSIBLE"
        };
        let error_body = self
            .menu_message
            .strip_prefix("Reprise impossible : ")
            .or_else(|| {
                self.menu_message
                    .strip_prefix("Nouvelle partie impossible : ")
            })
            .or_else(|| {
                self.menu_message
                    .strip_prefix("Retour au menu impossible : ")
            })
            .unwrap_or(&self.menu_message);
        let note = if menu_error {
            "Aucune partie ni suspension n'a été supprimée. Tu peux corriger le problème ou revenir."
        } else if !self.menu_message.is_empty() {
            self.menu_message.as_str()
        } else if matches!(
            self.menu,
            MenuScreen::Graphics | MenuScreen::ConfirmGraphics
        ) {
            graphics_note.as_str()
        } else if self.menu == MenuScreen::ConfirmAbandon {
            "La partie en cours sera perdue sans sauvegarde. Tu reviendras au menu principal."
        } else if self.menu == MenuScreen::ConfirmNewRun {
            "La partie suspendue sera définitivement remplacée. Annuler la conserve intacte."
        } else if self.menu == MenuScreen::Main && self.has_suspension() {
            "Une partie suspendue est disponible. La reprise reste unique et ne permet aucun retour en arrière."
        } else if self.menu == MenuScreen::Main {
            "Commence une nouvelle partie ou règle les options avant de jouer."
        } else if self.menu == MenuScreen::Options {
            if self.options_return == MenuScreen::Main {
                "Réglages personnels. Tu reviendras au menu principal."
            } else {
                "Réglages personnels. La partie reste en pause."
            }
        } else if self.game.status() != RunStatus::Active {
            "Partie terminée : aucune sauvegarde de reprise ne sera créée."
        } else {
            "Sauvegarder et quitter permet une reprise unique. Aucun tour ne s'écoule dans le menu."
        };
        let hint = if self.menu == MenuScreen::Graphics {
            format!(
                "{} / {} : valeur · {} ou clic : choisir · Échap : retour",
                self.controls.label(Action::MenuLeft),
                self.controls.label(Action::MenuRight),
                self.controls.label(Action::Learn)
            )
        } else if self.menu == MenuScreen::Main {
            format!(
                "{} / {} · {} ou clic : choisir",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Learn)
            )
        } else {
            format!(
                "{} / {} · {} ou clic : choisir · Échap : retour",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Learn)
            )
        };

        // Measuring every string first lets Macroquad finish extending its font
        // atlas before any menu quad references the atlas texture for this frame.
        let _ = measure_text_bold(self.menu.title(), 28);
        for label in &buttons {
            let _ = measure_text(label, None, 21, 1.0);
        }
        let _ = measure_text(note, None, 15, 1.0);
        let _ = measure_text(&hint, None, 14, 1.0);
        if menu_error {
            let _ = measure_text(error_title, None, 20, 1.0);
            let _ = measure_text(error_body, None, 16, 1.0);
        }
        let menu_background = if self.is_main_menu_flow() {
            let mut color = theme.surface();
            color.a = 1.0;
            color
        } else {
            theme.backdrop()
        };
        draw_rectangle(0.0, 0.0, self.ui_width(), self.ui_height(), menu_background);
        theme.panel(panel);
        if self.graphics.active.high_contrast {
            draw_rectangle_lines(panel.x, panel.y, panel.w, panel.h, 3.0, theme.accent());
        }
        draw_text_bold(
            self.menu.title(),
            panel.x + 24.0,
            panel.y + 45.0,
            28.0,
            theme.text(),
        );
        let summary = match self.menu {
            MenuScreen::Main if self.has_suspension() => {
                "Suspension disponible · reprise unique".to_owned()
            }
            MenuScreen::Main => {
                "Une vaste expédition où chaque décision laisse des traces".to_owned()
            }
            MenuScreen::Pause => self.game.current_zone().map_or_else(
                || "Partie en cours".to_owned(),
                |zone| {
                    format!(
                        "Partie en cours · {}",
                        player_zone_title(&zone.name, zone.depth)
                    )
                },
            ),
            _ => String::new(),
        };
        if !summary.is_empty() {
            draw_text(
                &summary,
                panel.x + 25.0,
                panel.y + 68.0,
                14.0,
                theme.muted(),
            );
        }
        for (index, (label, rect)) in buttons.iter().zip(&layout.buttons).enumerate() {
            let enabled = self.menu_row_enabled(index);
            let selected = enabled && self.menu_focus.highlighted(index, self.menu_selection);
            theme.button(
                *rect,
                label,
                selected,
                false,
                enabled,
                menu_button_tone(self.menu, index, self.has_suspension()),
            );
        }
        if menu_error {
            let area = layout.resume_error();
            draw_rectangle(
                area.x,
                area.y,
                area.w,
                area.h,
                Color::from_rgba(44, 24, 24, 255),
            );
            draw_rectangle_lines(area.x, area.y, area.w, area.h, 1.0, theme.danger());
            draw_text(
                error_title,
                area.x + 14.0,
                area.y + 26.0,
                20.0,
                theme.focus(),
            );
            draw_wrapped_text(
                error_body,
                area.x + 14.0,
                area.y + 53.0,
                area.w - 28.0,
                4,
                16,
                theme.text(),
            );
        }
        draw_wrapped_text(
            note,
            panel.x + 24.0,
            panel.y + panel.h - 70.0,
            panel.w - 48.0,
            2,
            15,
            theme.text(),
        );
        draw_wrapped_text(
            &hint,
            panel.x + 24.0,
            panel.y + panel.h - 22.0,
            panel.w - 48.0,
            1,
            14,
            theme.accent(),
        );
    }

    fn suspension(&self) -> Result<Suspension, String> {
        if self.game.status() != RunStatus::Active {
            return Err("Une partie terminée ne peut pas être suspendue.".to_owned());
        }
        let saved = Suspension {
            version: self.generation_version,
            build: env!("PROJECT_RL_BUILD_FINGERPRINT").to_owned(),
            rules: rules_fingerprint_for_version(&self.rules, self.generation_version),
            loot_rules: (self.generation_version >= 3).then(|| suspension::fingerprint(&self.loot)),
            world_rules: (self.generation_version >= 4).then(|| {
                world_fingerprint_for_version(
                    &self.expeditions,
                    &self.regional_worlds,
                    self.generation_version,
                )
            }),
            seed: self.seed,
            character_class: self.character_class.as_ref().map(ToString::to_string),
            starting_attributes: self
                .character_class
                .as_ref()
                .and_then(|_| self.game.player_primary_attributes())
                .map(PrimaryAttributes::values),
            commands: self.history.clone(),
            state: if self.generation_version == 1 {
                suspension::fingerprint(self.game.active_game())
            } else {
                suspension::fingerprint(&self.game)
            },
            active_weapon_slot: self.active_weapon_slot,
            selected_target: self.selected_target.map(|id| id.get()),
            report: self.observation_report.clone(),
            log: self.log.clone(),
        };
        saved.validate()?;
        Ok(saved)
    }

    fn restore_suspension(
        saved: &Suspension,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
    ) -> Result<Self, String> {
        // Build provenance is deliberately not a gate. Even after recompiling,
        // a run is installed only when rules AND the entire replay match.
        let regional_worlds = ascii_regional_world_catalog()?;
        saved.validate()?;
        let mut rules = rules;
        let restored_class = if saved.version >= CHARACTER_CLASSES_GENERATION_VERSION {
            saved
                .character_class
                .as_deref()
                .map(str::parse::<CharacterClassId>)
                .transpose()
                .map_err(|error| format!("Identifiant de protocole invalide : {error}"))?
        } else {
            None
        };
        if let Some(class_id) = &restored_class {
            let values = saved
                .starting_attributes
                .ok_or("Attributs du protocole absents de la suspension.")?;
            let attributes =
                PrimaryAttributes::new(values[0], values[1], values[2], values[3], values[4]);
            let catalog = ascii_character_class_catalog()?;
            let definition = catalog
                .get(class_id)
                .ok_or("Le protocole enregistré n'est plus disponible ; suspension conservée.")?;
            definition
                .apply_to_rules(&mut rules, attributes)
                .map_err(|error| format!("Protocole enregistré incompatible : {error}"))?;
        }
        if saved.rules != rules_fingerprint_for_version(&rules, saved.version) {
            return Err("Les règles ou les mods ont changé ; suspension conservée.".to_owned());
        }
        let compatible_loot = loot_for_generation_version(&loot, &rules.items, saved.version);
        if saved.version >= 3 && saved.loot_rules != Some(suspension::fingerprint(&compatible_loot))
        {
            return Err("Les tables de butin ont changé ; suspension conservée.".to_owned());
        }
        let expected_world_rules =
            world_fingerprint_for_version(&expeditions, &regional_worlds, saved.version);
        if saved.version >= 4 && saved.world_rules != Some(expected_world_rules) {
            return Err("Les définitions du monde ont changé ; suspension conservée.".to_owned());
        }
        let mut restored = if saved.version == 1 {
            Self::from_seed_legacy(
                saved.seed,
                rules_for_generation_version(rules, saved.version),
                texts,
                loot,
                expeditions,
                regional_worlds,
                saved.version,
            )?
        } else {
            Self::from_seed_version_with_regions(
                saved.seed,
                rules,
                texts,
                loot,
                expeditions,
                regional_worlds,
                saved.version,
            )?
        };
        for (index, recorded) in saved.commands.iter().enumerate() {
            let command = recorded.command(&restored.game)?;
            if let CommandOutcome::Rejected(error) = restored.execute_command(command) {
                return Err(format!("Rejeu divergent à la commande {index} : {error:?}"));
            }
            restored.capture_events_at(Some(0.0));
        }
        let state = if saved.version == 1 {
            suspension::fingerprint(restored.game.active_game())
        } else {
            suspension::fingerprint(&restored.game)
        };
        if restored.game.status() != RunStatus::Active || state != saved.state {
            return Err(
                "L'état reconstruit ne correspond pas à la suspension ; fichier conservé."
                    .to_owned(),
            );
        }
        if usize::from(saved.active_weapon_slot) >= restored.game.rules().player_weapon_slots.len()
        {
            return Err("Emplacement d'équipement enregistré invalide.".to_owned());
        }
        restored.active_weapon_slot = saved.active_weapon_slot;
        if let Some(target) = saved.selected_target {
            restored.selected_target = Some(
                restored
                    .game
                    .actors()
                    .iter()
                    .map(|(id, _)| id)
                    .find(|id| id.get() == target)
                    .ok_or("Cible sélectionnée absente de l'état reconstruit.")?,
            );
        }
        restored.observation_report = saved.report.clone();
        restored.log = saved.log.clone();
        restored.character_class = restored_class;
        restored.trace_cells.clear();
        restored.visual_cues.clear_world();
        restored.floating_messages.clear();
        restored.traces_visible_until = 0.0;
        if saved.version == 1 {
            // Verify the exact old state first. Enabling zone travel changes no
            // inventory, actors or accepted history and never rewrites the file.
            restored.generation_version = 2;
            restored.enable_expedition()?;
        }
        Ok(restored)
    }

    fn suspend_run(&self) -> Result<(), String> {
        let saved = self.suspension()?;
        // Prove that the entire current state is recoverable before writing or quitting.
        Self::restore_suspension(
            &saved,
            self.rules.clone(),
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        saved.write(&self.suspension_path)
    }

    fn resume_run(&mut self) -> Result<(), String> {
        let saved = Suspension::read(&self.suspension_path)?;
        let mut restored = Self::restore_suspension(
            &saved,
            self.rules.clone(),
            self.texts.clone(),
            self.loot.clone(),
            self.expeditions.clone(),
        )?;
        // No state is installed and nothing is consumed until every check succeeds.
        std::fs::remove_file(&self.suspension_path)
            .map_err(|error| format!("Impossible de consommer la suspension : {error}"))?;
        restored.controls = self.controls.clone();
        restored.controls_path = self.controls_path.clone();
        restored.options_message = self.options_message.clone();
        restored.graphics = self.graphics.clone();
        restored.suspension_path = self.suspension_path.clone();
        restored.session_lock = self.session_lock.take();
        restored.push_log("Partie reprise ; suspension consommée.".to_owned());
        *self = restored;
        Ok(())
    }

    fn update_options(&mut self, input: &InputFrame) {
        if self.rebinding {
            if input.pressed.len() == 1 {
                let action = Action::ALL[self.options_selection - 1];
                let mut next = self.controls.clone();
                match next.rebind(action, input.pressed.first().unwrap().clone()) {
                    Ok(()) => {
                        self.save_controls(next);
                        self.rebinding = false;
                    }
                    Err(error) => self.options_message = error,
                }
            } else if !input.pressed.is_empty() {
                self.options_message =
                    "Appuyer sur une seule touche ou un seul bouton de souris.".to_owned();
            }
            return;
        }
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let up = self.controls.pressed(Action::MenuUp, input);
        let down = self.controls.pressed(Action::MenuDown, input);
        let activate = self.controls.pressed(Action::Learn, input) && !clicked;
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let mut layout =
            ControlsLayout::new(width, height, self.options_scroll, Action::ALL.len() + 1);
        let scrolling = wheel_steps(input.wheel_y) > 0
            && input
                .pointer
                .is_some_and(|pointer| layout.bounds.contains(pointer.into()));
        if scrolling {
            layout.scroll(input.wheel_y);
            self.menu_focus.scroll();
        }
        if up && !clicked {
            self.options_selection = self.options_selection.saturating_sub(1);
        } else if down && !clicked {
            self.options_selection = (self.options_selection + 1).min(Action::ALL.len() + 1);
        }
        if up || down || activate || (self.menu_focus.keyboard_mode() && !scrolling) {
            layout.reveal(self.options_selection.min(Action::ALL.len()));
        }
        self.options_scroll = layout.first;
        let hovered = input.pointer.and_then(|pointer| {
            if layout.back.contains(pointer.into()) {
                Some(Action::ALL.len() + 1)
            } else {
                layout.hit(pointer)
            }
        });
        self.menu_focus.update(
            hovered,
            &mut self.options_selection,
            clicked,
            up || down || activate,
        );
        if (clicked && hovered.is_some()) || activate {
            if self.options_selection > Action::ALL.len() {
                self.open_menu(MenuScreen::Options);
            } else if self.options_selection == 0 {
                let mut next = self.controls.clone();
                match next.change_layout(next.layout.other()) {
                    Ok(()) => self.save_controls(next),
                    Err(error) => self.options_message = error,
                }
            } else {
                self.rebinding = true;
                self.menu_focus.reset();
                self.options_message =
                    "Nouvelle touche ou bouton de souris ; Échap pour annuler.".to_owned();
            }
        }
    }

    fn save_controls(&mut self, next: Controls) {
        match next.save(&self.controls_path) {
            Ok(()) => {
                self.controls = next;
                self.options_message = "Réglages enregistrés. Une seule touche par action ; personnalisations conservées.".to_owned();
            }
            Err(error) => {
                self.options_message =
                    format!("Enregistrement impossible, réglages inchangés : {error}")
            }
        }
    }

    fn draw_options(&self) {
        let layout = ControlsLayout::new(
            self.ui_width(),
            self.ui_height(),
            self.options_scroll,
            Action::ALL.len() + 1,
        );
        let margin = layout.bounds.x;
        let width = layout.bounds.w;
        let bottom = self.ui_height() - 125.0;
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            Color::from_rgba(3, 9, 13, 255),
        );
        draw_text_bold("OPTIONS", margin, 55.0, 30.0, UiTheme.text());
        let back_selected = self
            .menu_focus
            .highlighted(Action::ALL.len() + 1, self.options_selection);
        let back = layout.back;
        UiTheme.button(
            back,
            "Retour",
            back_selected,
            false,
            true,
            ButtonTone::Secondary,
        );
        draw_text_bold("COMMANDES", margin, 87.0, 19.0, UiTheme.accent());
        draw_wrapped_text(
            "Disposition et raccourcis personnels. La partie est en pause. Échap : retour aux options.",
            margin,
            115.0,
            width,
            2,
            16,
            GRAY,
        );
        for index in layout.first..layout.first + layout.visible {
            let rect = layout.row(index).expect("visible controls row");
            let y = rect.y + 21.0;
            let selected = self.menu_focus.highlighted(index, self.options_selection);
            let (name, binding) = if index == 0 {
                (
                    "Disposition du clavier",
                    self.controls.layout.name().to_owned(),
                )
            } else {
                let action = Action::ALL[index - 1];
                (action.name(), self.controls.label(action))
            };
            if selected {
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    Color::from_rgba(18, 58, 63, 255),
                );
                draw_rectangle(rect.x, rect.y + 4.0, 3.0, rect.h - 8.0, YELLOW);
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, SKYBLUE);
            }
            draw_wrapped_text(
                name,
                margin + 10.0,
                y,
                width * 0.68 - 20.0,
                1,
                17,
                if selected { YELLOW } else { LIGHTGRAY },
            );
            UiTheme.button(
                Rect::new(
                    margin + width * 0.7,
                    rect.y + 3.0,
                    width * 0.3 - 10.0,
                    rect.h - 6.0,
                ),
                &binding,
                selected,
                false,
                true,
                ButtonTone::Secondary,
            );
        }
        if layout.count > layout.visible {
            let height = layout.bounds.h;
            let thumb = (height * layout.visible as f32 / layout.count as f32).max(12.0);
            let y = layout.bounds.y
                + (height - thumb) * layout.first as f32 / (layout.count - layout.visible) as f32;
            draw_rectangle(
                layout.bounds.x + layout.bounds.w - 5.0,
                layout.bounds.y,
                4.0,
                height,
                DARKGRAY,
            );
            draw_rectangle(
                layout.bounds.x + layout.bounds.w - 5.0,
                y,
                4.0,
                thumb,
                SKYBLUE,
            );
        }
        draw_wrapped_text(
            &self.options_message,
            margin,
            bottom + 35.0,
            width,
            2,
            15,
            YELLOW,
        );
        let hint = format!(
            "{} / {} : sélectionner · molette : défiler · {} ou clic : {} · Échap : retour",
            self.controls.label(Action::MenuUp),
            self.controls.label(Action::MenuDown),
            self.controls.label(Action::Learn),
            if self.options_selection > Action::ALL.len() {
                "revenir"
            } else if self.options_selection == 0 {
                "changer de disposition"
            } else {
                "réattribuer"
            }
        );
        draw_wrapped_text(
            &hint,
            margin,
            self.ui_height() - 36.0,
            width,
            2,
            15,
            SKYBLUE,
        );
    }

    fn dossier_available(&self) -> bool {
        !self.observation_report.is_empty()
            || !self.game.discovered_data_terminal_records().is_empty()
    }

    fn dossier_lines(&self) -> Vec<DossierLine> {
        let mut lines = Vec::new();
        if !self.observation_report.is_empty() {
            lines.push(DossierLine {
                text: "DERNIER RELEVÉ".to_owned(),
                kind: DossierLineKind::Section,
            });
            lines.extend(
                self.observation_report
                    .iter()
                    .cloned()
                    .map(|text| DossierLine {
                        text,
                        kind: DossierLineKind::Content,
                    }),
            );
        }

        let records = self.game.discovered_data_terminal_records();
        if !records.is_empty() {
            lines.push(DossierLine {
                text: format!("ARCHIVES DÉCOUVERTES · {}", records.len()),
                kind: DossierLineKind::Section,
            });
            for record in records {
                lines.push(DossierLine {
                    text: self
                        .texts
                        .resolve(DISPLAY_LOCALE, record.as_str())
                        .map(str::to_owned)
                        .unwrap_or_else(|| "Archive non déchiffrée".to_owned()),
                    kind: DossierLineKind::Content,
                });
            }
        }
        lines
    }

    fn draw_dossier(&self) {
        let theme = UiTheme;
        let x = 40.0;
        let width = (self.ui_width() - 80.0).max(100.0);
        let height = (self.ui_height() - 80.0).max(150.0);
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            theme.backdrop(),
        );
        theme.panel(Rect::new(x, 40.0, width, height));
        draw_text_bold("DOSSIER DE TERRAIN", x + 20.0, 77.0, 24.0, theme.text());
        let location = self
            .game
            .current_zone()
            .map(|zone| player_zone_title(&zone.name, zone.depth))
            .unwrap_or_else(|| "Zone inconnue".to_owned());
        draw_text(
            format!("{location} · cycle {}", self.game.turn()),
            x + 20.0,
            103.0,
            15.0,
            theme.accent(),
        );
        let close = Rect::new(x + width - 100.0, 50.0, 80.0, 31.0);
        theme.button(
            close,
            "Fermer",
            self.menu_focus.hovered == Some(0),
            false,
            true,
            ButtonTone::Secondary,
        );
        draw_line(x + 20.0, 119.0, x + width - 20.0, 119.0, 1.0, theme.muted());
        let bottom = 40.0 + height - 65.0;
        let mut y = 151.0;
        let lines = self.dossier_lines();
        for line in lines.iter().skip(self.report_scroll) {
            if y + 24.0 > bottom {
                break;
            }
            match line.kind {
                DossierLineKind::Section => {
                    draw_text_bold(&line.text, x + 20.0, y, 16.0, theme.accent());
                    draw_line(
                        x + 20.0,
                        y + 8.0,
                        x + width - 20.0,
                        y + 8.0,
                        1.0,
                        theme.surface_raised(),
                    );
                    y += 30.0;
                }
                DossierLineKind::Content => {
                    let card = Rect::new(x + 18.0, y - 20.0, width - 36.0, 56.0);
                    draw_rectangle(card.x, card.y, card.w, card.h, theme.surface_raised());
                    y = draw_wrapped_text(
                        &line.text,
                        card.x + 12.0,
                        y,
                        card.w - 24.0,
                        2,
                        17,
                        theme.text(),
                    ) + 17.0;
                }
            }
        }
        if lines.len() > 1 {
            let track = Rect::new(x + width - 9.0, 132.0, 3.0, (bottom - 132.0).max(20.0));
            draw_rectangle(track.x, track.y, track.w, track.h, theme.surface_raised());
            let thumb_h = (track.h / lines.len() as f32 * 4.0).clamp(20.0, track.h);
            let thumb_y = track.y
                + (track.h - thumb_h) * self.report_scroll as f32
                    / lines.len().saturating_sub(1).max(1) as f32;
            draw_rectangle(track.x, thumb_y, track.w, thumb_h, theme.accent());
        }
        draw_wrapped_text(
            &format!(
                "{} / {} ou molette : défiler · {} : fermer",
                self.controls.label(Action::MenuUp),
                self.controls.label(Action::MenuDown),
                self.controls.label(Action::Report)
            ),
            x + 20.0,
            40.0 + height - 25.0,
            width - 40.0,
            2,
            14,
            theme.accent(),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_wrapped_text(
    text: &str,
    x: f32,
    mut y: f32,
    width: f32,
    maximum_lines: usize,
    size: u16,
    color: Color,
) -> f32 {
    if maximum_lines == 0 || text.is_empty() {
        return y;
    }
    let mut lines = Vec::with_capacity(maximum_lines);
    let mut line = String::with_capacity(text.len().min(128));
    let mut truncated = false;
    for word in text.split_whitespace() {
        let previous_length = line.len();
        if previous_length > 0 {
            line.push(' ');
        }
        line.push_str(word);
        if previous_length > 0 && measure_text(&line, None, size, 1.0).width > width {
            line.truncate(previous_length);
            lines.push(std::mem::take(&mut line));
            if lines.len() == maximum_lines {
                truncated = true;
                break;
            }
            line.push_str(word);
        }
    }
    if !line.is_empty() && lines.len() < maximum_lines {
        lines.push(line);
    }
    let last_line = lines.len().saturating_sub(1);
    for (index, line) in lines.iter().enumerate() {
        let ellipsized = (truncated && index == last_line).then(|| format!("{line}…"));
        let line = ellipsized.as_deref().unwrap_or(line);
        let measured = measure_text(line, None, size, 1.0).width;
        let font_size = f32::from(size) * (width / measured.max(1.0)).min(1.0);
        draw_text(line, x, y, font_size, color);
        y += f32::from(size) + 5.0;
    }
    y
}

fn rules_fingerprint_for_version(rules: &GameRules, version: u8) -> u64 {
    let mut legacy = rules_for_generation_version(rules.clone(), version);
    if version >= 5 {
        suspension::fingerprint(&legacy)
    } else {
        // ItemKind::Material and its definitions entered the ruleset together
        // with generation v5. Older replays must retain their exact catalogue.
        legacy.items = legacy.items.without_kind(ItemKind::Material);
        suspension::fingerprint(&legacy)
    }
}

fn expedition_fingerprint_for_version(expeditions: &ExpeditionCatalog, version: u8) -> u64 {
    let relation_compatible = if version >= PLAYER_RELATIONS_GENERATION_VERSION {
        expeditions.clone()
    } else {
        expeditions.without_player_relation_metadata()
    };
    let disruption_compatible = if version >= PREPARATION_DISRUPTION_GENERATION_VERSION {
        relation_compatible
    } else {
        relation_compatible.without_preparation_disruption_metadata()
    };
    let electronic_compatible = if version >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
        disruption_compatible
    } else {
        disruption_compatible.without_electronic_system_metadata()
    };
    let ranged_compatible = if version >= RANGED_SKILLS_GENERATION_VERSION {
        electronic_compatible
    } else {
        electronic_compatible.without_ranged_skill_body_metadata()
    };
    let melee_compatible = if version >= MELEE_SKILLS_GENERATION_VERSION {
        ranged_compatible
    } else {
        ranged_compatible.without_melee_skill_body_metadata()
    };
    let physical_compatible = if version >= PHYSICAL_PROFILES_GENERATION_VERSION {
        melee_compatible
    } else {
        melee_compatible.without_physical_metadata()
    };
    let attribute_compatible = if version >= ENEMY_ATTRIBUTES_GENERATION_VERSION {
        physical_compatible
    } else {
        physical_compatible.without_primary_attribute_metadata()
    };
    let terminal_compatible = if version >= DATA_TERMINALS_GENERATION_VERSION {
        attribute_compatible
    } else {
        attribute_compatible.without_data_terminal_metadata()
    };
    let lifecycle_compatible = if version >= PURSUIT_LIFECYCLE_GENERATION_VERSION {
        terminal_compatible
    } else {
        terminal_compatible.without_pursuit_lifecycle_metadata()
    };
    let pursuit_compatible = if version >= PURSUIT_LEASH_GENERATION_VERSION {
        lifecycle_compatible
    } else {
        lifecycle_compatible.without_pursuit_metadata()
    };
    if version >= EXPANDED_WORLD_GENERATION_VERSION {
        return suspension::fingerprint(&pursuit_compatible);
    }
    let legacy = pursuit_compatible.without_expanded_world_metadata();
    let compatible = match version {
        4 => legacy.without_facilities(),
        5 => legacy.without_social_metadata(),
        6 => legacy.without_local_alert_metadata(),
        7 => legacy.without_security_alarm_metadata(),
        8 => legacy.without_security_alarm_response_metadata(),
        9 | 10 => legacy.without_population_metadata(),
        _ => legacy,
    };
    suspension::fingerprint(&compatible)
}

fn world_fingerprint_for_version(
    expeditions: &ExpeditionCatalog,
    regional_worlds: &RegionalWorldCatalog,
    version: u8,
) -> u64 {
    if version < REGIONAL_TRAVEL_GENERATION_VERSION {
        return expedition_fingerprint_for_version(expeditions, version);
    }
    let relation_compatible = if version >= PLAYER_RELATIONS_GENERATION_VERSION {
        expeditions.clone()
    } else {
        expeditions.without_player_relation_metadata()
    };
    let disruption_compatible = if version >= PREPARATION_DISRUPTION_GENERATION_VERSION {
        relation_compatible
    } else {
        relation_compatible.without_preparation_disruption_metadata()
    };
    let electronic_compatible = if version >= ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
        disruption_compatible
    } else {
        disruption_compatible.without_electronic_system_metadata()
    };
    let ranged_compatible = if version >= RANGED_SKILLS_GENERATION_VERSION {
        electronic_compatible
    } else {
        electronic_compatible.without_ranged_skill_body_metadata()
    };
    let melee_compatible = if version >= MELEE_SKILLS_GENERATION_VERSION {
        ranged_compatible
    } else {
        ranged_compatible.without_melee_skill_body_metadata()
    };
    let physical_compatible = if version >= PHYSICAL_PROFILES_GENERATION_VERSION {
        melee_compatible
    } else {
        melee_compatible.without_physical_metadata()
    };
    let attribute_compatible = if version >= ENEMY_ATTRIBUTES_GENERATION_VERSION {
        physical_compatible
    } else {
        physical_compatible.without_primary_attribute_metadata()
    };
    let terminal_compatible = if version >= DATA_TERMINALS_GENERATION_VERSION {
        attribute_compatible
    } else {
        attribute_compatible.without_data_terminal_metadata()
    };
    let compatible_expeditions = if version < PURSUIT_LIFECYCLE_GENERATION_VERSION {
        terminal_compatible.without_pursuit_lifecycle_metadata()
    } else {
        terminal_compatible
    };
    let mut compatible_regions = regional_worlds.clone();
    if version < PLAYER_RELATIONS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_player_relation_metadata();
    }
    if version < ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_electronic_system_metadata();
    }
    if version < RANGED_SKILLS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_ranged_skill_body_metadata();
    }
    if version < MELEE_SKILLS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_melee_skill_body_metadata();
    }
    if version < PHYSICAL_PROFILES_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_physical_metadata();
    }
    if version < ENEMY_ATTRIBUTES_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_primary_attribute_metadata();
    }
    if version < REGIONAL_DESTRUCTIBLES_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_destructible_metadata();
    }
    if version < REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION {
        compatible_regions = compatible_regions
            .without_vertical_link_metadata()
            .without_deeper_layer_gameplay_metadata();
    }
    if version < SITE_NAVIGATION_SIGNALS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_site_navigation_signal_metadata();
    }
    if version < REGIONAL_SITE_SECURITY_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_site_security_metadata();
    }
    if version < REGIONAL_SITE_TERMINALS_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_site_terminal_metadata();
    }
    if version < REGIONAL_SITE_INTERACTION_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_site_interaction_metadata();
    }
    if version < REGIONAL_SITE_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_site_metadata();
    }
    if version < REGIONAL_ENCOUNTER_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_encounter_metadata();
    }
    if version < REGIONAL_POPULATION_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_population_metadata();
    }
    if version < PURSUIT_LIFECYCLE_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_pursuit_lifecycle_metadata();
    }
    if version < REGIONAL_LOOT_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_loot_metadata();
    }
    if version < REGIONAL_LANDMARK_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_landmark_metadata();
    } else if version < THREAT_RENEWAL_GENERATION_VERSION {
        compatible_regions = compatible_regions.without_threat_metadata();
    }
    suspension::fingerprint(&(compatible_expeditions, compatible_regions))
}

fn territorial_ai_for_generation(
    profile: AiProfile,
    generation_version: u8,
    maximum_distance: u16,
) -> AiProfile {
    if generation_version < PURSUIT_LEASH_GENERATION_VERSION {
        return profile;
    }
    let profile = profile.with_maximum_pursuit_distance(
        std::num::NonZeroU16::new(maximum_distance)
            .expect("territorial pursuit distance must be positive"),
    );
    if generation_version < PURSUIT_LIFECYCLE_GENERATION_VERSION {
        return profile;
    }
    profile.with_pursuit_lifecycle(PursuitLifecycle::new(
        std::num::NonZeroU16::new(12).expect("constant pursuit duration is positive"),
        std::num::NonZeroU16::new(4).expect("constant search duration is positive"),
        std::num::NonZeroU16::new(6).expect("constant cooldown duration is positive"),
    ))
}

/// Provisional combat profiles for the three fixed exterior prototypes. Their
/// roles remain content-neutral; only Coordination and Perception currently
/// affect hit resolution.
const fn prototype_enemy_attributes(index: usize) -> PrimaryAttributes {
    match index % 3 {
        0 => PrimaryAttributes::new(6, 6, 5, 6, 4),
        1 => PrimaryAttributes::new(5, 5, 6, 8, 6),
        _ => PrimaryAttributes::new(4, 7, 4, 7, 6),
    }
}

fn rules_for_generation_version(mut rules: GameRules, version: u8) -> GameRules {
    rules.player_drone_expires_without_energy = version >= DRONE_ENERGY_LIFETIME_GENERATION_VERSION;
    rules.player_companion_behaviors = version >= DRONE_ENERGY_LIFETIME_GENERATION_VERSION;
    rules.player_drone_link_awareness = version >= DRONE_LINK_AWARENESS_GENERATION_VERSION;
    rules.player_relation_targeting = version >= PLAYER_RELATIONS_GENERATION_VERSION;
    rules.player_drone_default_support = version >= DRONE_DEFAULT_SUPPORT_GENERATION_VERSION;
    if version < PREPARATION_DISRUPTION_GENERATION_VERSION {
        rules.player_base_attacks = rules
            .player_base_attacks
            .into_iter()
            .map(AttackProfile::without_preparation_disruption)
            .collect();
        rules.weapons = rules.weapons.without_preparation_disruption_metadata();
    }
    rules.wait_continues_technique_preparation =
        version >= WAIT_CONTINUES_PREPARATION_GENERATION_VERSION;
    if version < INTRINSIC_SKILL_MANIFESTATIONS_GENERATION_VERSION {
        rules.skills = rules.skills.without_intrinsic_manifestations();
        let drone_manifestation: TechniqueId = "core:drn_01"
            .parse()
            .expect("built-in drone technique ID must remain valid");
        rules.skills = rules.skills.with_compatibility_action(
            &drone_manifestation,
            TechniqueAction::DroneEscort {
                link_range: 6,
                minimum_distance: 1,
                maximum_distance: 3,
                energy_cost: 2,
            },
        );
    }
    if version < AUTHORED_SKILL_REQUIREMENTS_GENERATION_VERSION {
        rules.skill_progression = rules.skill_progression.without_authored_requirements();
    }
    if version < ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
        let electronic: DisciplineId = "core:guerre_electronique"
            .parse()
            .expect("built-in electronic warfare discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&electronic);
        for feature in ["core:electronic_warfare", "core:hostile_programs"] {
            let feature: SystemFeatureId = feature
                .parse()
                .expect("built-in electronic warfare feature ID must remain valid");
            rules.enabled_system_features = rules.enabled_system_features.without(&feature);
        }
        let beacon: ItemId = "core:saturation_beacon"
            .parse()
            .expect("built-in saturation beacon ID must remain valid");
        rules
            .player_starting_items
            .retain(|entry| entry.item != beacon);
        rules.items = rules.items.without_id(&beacon);
    }
    if version < INTRUSION_SKILLS_GENERATION_VERSION {
        let intrusion: DisciplineId = "core:intrusion"
            .parse()
            .expect("built-in intrusion discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&intrusion);
        for feature in ["core:intrusion", "core:active_security"] {
            let feature: SystemFeatureId = feature
                .parse()
                .expect("built-in intrusion feature ID must remain valid");
            rules.enabled_system_features = rules.enabled_system_features.without(&feature);
        }
    }
    if version < ENGINEERING_SKILLS_GENERATION_VERSION {
        let engineering: DisciplineId = "core:ingenierie"
            .parse()
            .expect("built-in engineering discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&engineering);
        rules.weapons = rules.weapons.without_engineering_metadata();
        for feature in [
            "core:engineering",
            "core:workshops",
            "core:heat",
            "core:sound",
        ] {
            let feature: SystemFeatureId = feature
                .parse()
                .expect("built-in engineering feature ID must remain valid");
            rules.enabled_system_features = rules.enabled_system_features.without(&feature);
        }
        for item in [
            "core:engineering_tool",
            "core:repair_parts",
            "core:tuning_parts",
            "core:salvaged_component",
            "core:charged_battery",
        ] {
            let item: ItemId = item
                .parse()
                .expect("built-in engineering item ID must remain valid");
            rules
                .player_starting_items
                .retain(|entry| entry.item != item);
            rules.items = rules.items.without_id(&item);
        }
    }
    if version < RECONNAISSANCE_COMPLETION_GENERATION_VERSION {
        rules.skills = rules.skills.without_runtime_behaviors([
            (
                "core:rec_03"
                    .parse()
                    .expect("built-in inspection technique ID must remain valid"),
                TechniqueKind::Action,
            ),
            (
                "core:rec_08"
                    .parse()
                    .expect("built-in energy diagnostic technique ID must remain valid"),
                TechniqueKind::Action,
            ),
        ]);
        for feature in ["core:secrets", "core:energy_states"] {
            let feature: SystemFeatureId = feature
                .parse()
                .expect("built-in reconnaissance feature ID must remain valid");
            rules.enabled_system_features = rules.enabled_system_features.without(&feature);
        }
    }
    if version < DRONE_SKILLS_GENERATION_VERSION {
        let drones: DisciplineId = "core:controle_drones"
            .parse()
            .expect("built-in drone discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&drones);
    }
    if version < FURTIVITE_SKILLS_GENERATION_VERSION {
        let furtivite: DisciplineId = "core:furtivite"
            .parse()
            .expect("built-in stealth discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&furtivite);
        rules.stealth_rules = None;
        for item in ["core:sound_decoy", "core:camouflage_bundle"] {
            let item: ItemId = item
                .parse()
                .expect("built-in stealth material ID must remain valid");
            rules
                .player_starting_items
                .retain(|entry| entry.item != item);
            rules.items = rules.items.without_id(&item);
        }
    }
    if version < MANOEUVRE_SKILLS_GENERATION_VERSION {
        let manoeuvre: DisciplineId = "core:manoeuvre"
            .parse()
            .expect("built-in manoeuvre discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&manoeuvre);
    }
    if version < SYSTEM_RESOURCES_GENERATION_VERSION {
        rules.player_system_resources = None;
    }
    if version < DEMOLITION_SKILLS_GENERATION_VERSION {
        let demolition: DisciplineId = "core:demolition"
            .parse()
            .expect("built-in demolition discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&demolition);
        for item in [
            "core:fragmentation_charge",
            "core:breach_charge",
            "core:proximity_mine",
            "core:configurable_charge",
            "core:structural_charge",
        ] {
            let item: ItemId = item
                .parse()
                .expect("built-in explosive material ID must remain valid");
            rules
                .player_starting_items
                .retain(|entry| entry.item != item);
            rules.items = rules.items.without_id(&item);
        }
    }
    if version < RANGED_SKILLS_GENERATION_VERSION {
        let ranged_discipline: DisciplineId = "core:tir"
            .parse()
            .expect("built-in ranged discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&ranged_discipline);
        let suppression: StatusId = "core:suppressed"
            .parse()
            .expect("built-in suppression status ID must remain valid");
        rules.statuses = rules.statuses.without_id(&suppression);
        rules.weapons = rules.weapons.without_ranged_skill_metadata();
        rules.player_body_profile = rules
            .player_body_profile
            .map(BodyProfile::without_ranged_skill_metadata);
        let components: SystemFeatureId = "core:components"
            .parse()
            .expect("built-in component feature ID must remain valid");
        rules.enabled_system_features = rules.enabled_system_features.without(&components);
    }
    if version < MELEE_SKILLS_GENERATION_VERSION {
        let melee_discipline: DisciplineId = "core:combat_rapproche"
            .parse()
            .expect("built-in melee discipline ID must remain valid");
        rules.skills = rules.skills.without_discipline(&melee_discipline);
        for status in [
            "core:armor_fragilized",
            "core:locomotion_hindered",
            "core:locomotion_hindrance_protection",
        ] {
            let status: StatusId = status
                .parse()
                .expect("built-in melee status ID must remain valid");
            rules.statuses = rules.statuses.without_id(&status);
        }
        rules.stability_rules = None;
        rules.player_body_profile = rules
            .player_body_profile
            .map(BodyProfile::without_melee_skill_metadata);
    }
    if version < ACTION_RECOVERY_GENERATION_VERSION {
        rules.player_base_attacks = rules
            .player_base_attacks
            .into_iter()
            .map(AttackProfile::without_recovery_after_attack)
            .collect();
        rules.player_base_abilities = rules
            .player_base_abilities
            .into_iter()
            .map(AbilityProfile::without_action_kind)
            .collect();
        rules.skills = rules.skills.without_action_kinds();
        rules.weapons = rules.weapons.without_recovery_metadata();
    }
    if version < PARRY_REACTION_GENERATION_VERSION {
        rules.weapons = rules.weapons.without_reaction_metadata();
    }
    if version < EFFECT_TRIGGER_GENERATION_VERSION {
        rules.weapons = rules.weapons.without_effect_triggers();
    }
    if version < ARMOR_EQUIPMENT_GENERATION_VERSION {
        rules.player_starting_items.retain(|starting| {
            rules
                .items
                .get(&starting.item)
                .is_none_or(|definition| definition.kind() != ItemKind::Armor)
        });
        rules.items = rules.items.without_kind(ItemKind::Armor);
        rules.player_armor_slots.clear();
    }
    if version < ARMOR_GENERATION_VERSION {
        rules.damage = DamageRules::default();
        rules.armor_rules = None;
        rules.player_body_profile = rules
            .player_body_profile
            .map(BodyProfile::without_base_armor);
    }
    if version < PHYSICAL_PROFILES_GENERATION_VERSION {
        rules.physical_rules = None;
        rules.player_body_profile = None;
        rules.player_base_attacks = rules
            .player_base_attacks
            .into_iter()
            .map(AttackProfile::without_melee_impact)
            .collect();
        rules.weapons = rules.weapons.without_physical_metadata();
    }
    if version < HIT_CHANCE_GENERATION_VERSION {
        rules.hit_rules = None;
    }
    if version < FLAMETHROWER_GENERATION_VERSION {
        let flamethrower: WeaponId = "core:flamethrower"
            .parse()
            .expect("built-in flamethrower ID must remain valid");
        let burning: StatusId = "core:burning"
            .parse()
            .expect("built-in burning status ID must remain valid");
        if let Some(index) = rules
            .player_starting_weapons
            .iter()
            .position(|weapon| weapon == &flamethrower)
        {
            rules.player_starting_weapons.remove(index);
            if index < rules.player_base_attacks.len() {
                rules.player_base_attacks.remove(index);
            }
        }
        rules
            .player_starting_equipment
            .retain(|weapon| weapon.as_ref() != Some(&flamethrower));
        rules.weapons = rules.weapons.without_id(&flamethrower);
        rules.statuses = rules.statuses.without_id(&burning);
    }
    rules
}

fn loot_for_generation_version(
    loot: &LootCatalog,
    items: &project_rl::item::ItemCatalog,
    version: u8,
) -> LootCatalog {
    let mut compatible = if version < ARMOR_EQUIPMENT_GENERATION_VERSION {
        loot.without_item_kind(items, ItemKind::Armor)
    } else {
        loot.clone()
    };
    if version < ENGINEERING_SKILLS_GENERATION_VERSION {
        let excluded = [
            "core:engineering_tool",
            "core:repair_parts",
            "core:tuning_parts",
            "core:salvaged_component",
            "core:charged_battery",
        ]
        .into_iter()
        .map(str::parse)
        .collect::<Result<Vec<ItemId>, project_rl::content::ContentIdError>>()
        .expect("built-in engineering item IDs must remain valid");
        compatible = compatible.without_items(&excluded);
    }
    if version < ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION {
        let beacon: ItemId = "core:saturation_beacon"
            .parse()
            .expect("built-in saturation beacon ID must remain valid");
        compatible = compatible.without_items(&[beacon]);
    }
    compatible
}

fn ascii_game_content() -> Result<(GameRules, TextCatalog, LootCatalog, ExpeditionCatalog), String>
{
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
    let flamethrower_id: WeaponId = "core:flamethrower"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let repair_id: ItemId = "core:repair_patch"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let sound_decoy_id: ItemId = "core:sound_decoy"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let camouflage_bundle_id: ItemId = "core:camouflage_bundle"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let engineering_tool_id: ItemId = "core:engineering_tool"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let repair_parts_id: ItemId = "core:repair_parts"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let tuning_parts_id: ItemId = "core:tuning_parts"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let charged_battery_id: ItemId = "core:charged_battery"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let saturation_beacon_id: ItemId = "core:saturation_beacon"
        .parse()
        .map_err(|error: project_rl::content::ContentIdError| error.to_string())?;
    let player_starting_equipment = vec![
        Some(melee_id.clone()),
        Some(ranged_id.clone()),
        Some(flamethrower_id.clone()),
    ];
    let player_starting_weapons = vec![melee_id, ranged_id, flamethrower_id];
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
    let player_armor_slots = ["core:body_armor"]
        .into_iter()
        .map(str::parse)
        .collect::<Result<Vec<_>, project_rl::content::ContentIdError>>()
        .map_err(|error| error.to_string())?;
    let texts = loaded.texts().clone();
    let loot = loaded.loot().clone();
    let expeditions = loaded.expeditions().clone();
    let (statuses, weapons, items, skills) = loaded.into_registries();
    let mut rules = GameRules {
        damage: DamageRules::specialized(),
        armor_rules: Some(ArmorRules::default()),
        hit_rules: Some(HitRules::default()),
        physical_rules: Some(PhysicalRules::default()),
        stability_rules: Some(StabilityRules::default()),
        player_body_profile: Some(
            BodyProfile::new(15, 0)
                .map_err(|error| error.to_string())?
                .with_base_armor(1)
                .with_displacement_profile(
                    DisplacementProfile::new(75_000, 0).map_err(|error| error.to_string())?,
                )
                .with_locomotion_profile(LocomotionProfile::new(true))
                .with_suppression_compatibility(true),
        ),
        player_base_attacks,
        player_weapon_slots,
        player_armor_slots,
        player_starting_weapons,
        player_starting_items: vec![
            StartingItemStack::new(repair_id, 2),
            StartingItemStack::new(sound_decoy_id, 2),
            StartingItemStack::new(camouflage_bundle_id, 2),
            StartingItemStack::new(engineering_tool_id, 1),
            StartingItemStack::new(repair_parts_id, 6),
            StartingItemStack::new(tuning_parts_id, 3),
            StartingItemStack::new(charged_battery_id, 2),
            StartingItemStack::new(saturation_beacon_id, 2),
        ],
        player_starting_equipment,
        player_system_resources: Some(SystemResourceRules::default()),
        stealth_rules: Some(StealthRules::default()),
        statuses,
        weapons,
        items,
        skills,
        enabled_system_features: SystemFeatureSet::new(
            [
                "core:traces",
                "core:components",
                "core:secrets",
                "core:energy_states",
                "core:engineering",
                "core:workshops",
                "core:heat",
                "core:sound",
                "core:intrusion",
                "core:active_security",
                "core:electronic_warfare",
                "core:hostile_programs",
            ]
            .into_iter()
            .map(str::parse)
            .collect::<Result<Vec<_>, project_rl::content::ContentIdError>>()
            .map_err(|error| error.to_string())?,
        ),
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
    Ok((rules, texts, loot, expeditions))
}

fn ascii_regional_world_catalog() -> Result<RegionalWorldCatalog, String> {
    static CATALOG: std::sync::OnceLock<Result<RegionalWorldCatalog, String>> =
        std::sync::OnceLock::new();
    CATALOG
        .get_or_init(|| {
            let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ContentLoader::load(
                &[project_root.join("content"), project_root.join("mods")],
                &semver::Version::new(0, 1, 0),
            )
            .map(|loaded| loaded.regional_worlds().clone())
            .map_err(|error| error.to_string())
        })
        .clone()
}

fn ascii_character_class_catalog() -> Result<CharacterClassCatalog, String> {
    static CATALOG: std::sync::OnceLock<Result<CharacterClassCatalog, String>> =
        std::sync::OnceLock::new();
    CATALOG
        .get_or_init(|| {
            let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ContentLoader::load(
                &[project_root.join("content"), project_root.join("mods")],
                &semver::Version::new(0, 1, 0),
            )
            .map(|loaded| loaded.character_classes().clone())
            .map_err(|error| error.to_string())
        })
        .clone()
}

fn ascii_visual_cue_catalog() -> Result<VisualCueCatalog, String> {
    static CATALOG: std::sync::OnceLock<Result<VisualCueCatalog, String>> =
        std::sync::OnceLock::new();
    CATALOG
        .get_or_init(|| {
            let project_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            ContentLoader::load(
                &[project_root.join("content"), project_root.join("mods")],
                &semver::Version::new(0, 1, 0),
            )
            .map(|loaded| loaded.visual_cues().clone())
            .map_err(|error| error.to_string())
        })
        .clone()
}

fn display_content_name(id: &project_rl::content::ContentId) -> String {
    id.name().replace(['_', '-'], " ").to_uppercase()
}

fn player_zone_title(name: &str, depth: u16) -> String {
    let name = player_location_name(name);
    if depth == 0 {
        name
    } else {
        format!("{name} · profondeur {depth}")
    }
}

fn skill_points_hud_label(points: u32) -> String {
    match points {
        0 => "Aucun point de compétence".to_owned(),
        1 => "1 point de compétence disponible".to_owned(),
        points => format!("{points} points de compétence disponibles"),
    }
}

fn observer_location(observer: Option<GridPos>, target: GridPos) -> String {
    observer.map_or_else(
        || "dans la zone".to_owned(),
        |observer| relative_location_label(observer, target),
    )
}

fn relative_location_label(observer: GridPos, target: GridPos) -> String {
    let dx = target.x - observer.x;
    let dy = target.y - observer.y;
    if dx == 0 && dy == 0 {
        return "ici".to_owned();
    }
    let horizontal = if dx < 0 { "ouest" } else { "est" };
    let vertical = if dy < 0 { "nord" } else { "sud" };
    let direction = if dx == 0 {
        format!("au {vertical}")
    } else if dy == 0 {
        format!("à l'{horizontal}")
    } else {
        format!("au {vertical}-{horizontal}")
    };
    let distance = dx.unsigned_abs().max(dy.unsigned_abs());
    let band = match distance {
        0..=2 => "tout près",
        3..=6 => "à proximité",
        7..=12 => "à distance",
        _ => "au loin",
    };
    format!("{direction}, {band}")
}

fn status_display_name(id: &project_rl::content::ContentId) -> String {
    match id.as_str() {
        "core:burning" => "BRÛLURE".to_owned(),
        "core:corroded" => "CORROSION".to_owned(),
        "core:armor_fragilized" => "BLINDAGE FRAGILISÉ".to_owned(),
        "core:locomotion_hindered" => "ENTRAVE".to_owned(),
        "core:locomotion_hindrance_protection" => "PROTECTION LOCOMOTRICE".to_owned(),
        "core:suppressed" => "SUPPRESSION".to_owned(),
        _ => display_content_name(id),
    }
}

fn explosive_deployment_description(deployment: ExplosiveDeployment) -> String {
    match deployment {
        ExplosiveDeployment::ThrownImpact {
            range,
            exact_placement_modifier,
        } => format!(
            "Lance une charge à impact jusqu'à {range} cases, avec Précision {exact_placement_modifier:+}."
        ),
        ExplosiveDeployment::AdjacentTimed { delay_turns, .. } => format!(
            "Pose une charge sur une case adjacente ; elle explose après {delay_turns} tour(s)."
        ),
        ExplosiveDeployment::AdjacentProximity {
            arming_delay_turns,
            trigger_radius,
        } => format!(
            "Pose une mine adjacente ; armement après {arming_delay_turns} tour(s), déclenchement dans un rayon de {trigger_radius}."
        ),
        ExplosiveDeployment::AdjacentRemote {
            maximum_link_range, ..
        } => format!(
            "Pose une charge adjacente déclenchable à distance tant que la liaison reste à {maximum_link_range} cases ou moins."
        ),
    }
}

const fn signature_channel_label(channel: SignatureChannel) -> &'static str {
    match channel {
        SignatureChannel::Optical => "capteurs optiques",
        SignatureChannel::Acoustic => "capteurs acoustiques",
        SignatureChannel::ActiveEmission => "émissions actives",
    }
}

fn damage_sequence_label(damage: &[u16]) -> String {
    damage
        .iter()
        .map(u16::to_string)
        .collect::<Vec<_>>()
        .join(" → ")
}

fn technical_reference(id: &project_rl::content::ContentId) -> String {
    id.name().replace('_', "-").to_uppercase()
}

const fn damage_type_label(damage_type: DamageType) -> &'static str {
    match damage_type {
        DamageType::Kinetic => "Cinétique",
        DamageType::Piercing => "Perforant",
        DamageType::Explosive => "Explosif",
        DamageType::Thermal => "Thermique",
        DamageType::Electrical => "Électrique",
        DamageType::Chemical => "Chimique",
        DamageType::Radiation => "Radiation",
        DamageType::Corruption => "Corruption",
    }
}

const fn primary_attribute_label(attribute: PrimaryAttribute) -> &'static str {
    match attribute {
        PrimaryAttribute::Power => "PUISSANCE",
        PrimaryAttribute::Coordination => "COORDINATION",
        PrimaryAttribute::Resilience => "RÉSILIENCE",
        PrimaryAttribute::Perception => "PERCEPTION",
        PrimaryAttribute::Processing => "TRAITEMENT",
    }
}

const fn primary_attribute_description(attribute: PrimaryAttribute) -> &'static str {
    match attribute {
        PrimaryAttribute::Power => {
            "Renforce vos frappes physiques au corps à corps et votre capacité à repousser les adversaires. Améliore la charge que vous pouvez transporter et la maîtrise du recul, dans les limites de votre équipement. N’augmente pas les dégâts des projectiles."
        }
        PrimaryAttribute::Coordination => {
            "Améliore la précision de vos attaques et de vos lancers, ainsi que votre capacité à esquiver et à rester discret. N’augmente pas votre vitesse d’action."
        }
        PrimaryAttribute::Resilience => {
            "Augmente vos points de vie maximaux et améliore la stabilité de vos systèmes, pour mieux résister aux interruptions et à certaines perturbations. N’augmente pas votre blindage et ne restaure pas les points de vie perdus."
        }
        PrimaryAttribute::Perception => {
            "Améliore votre capacité à repérer les présences dissimulées et les indices discrets, ainsi qu’à analyser vos observations. Contribue également à la précision de vos tirs. N’augmente pas la portée de vos capteurs et ne permet pas de voir à travers les murs."
        }
        PrimaryAttribute::Processing => {
            "Améliore l’efficacité de vos intrusions et votre résistance aux attaques logicielles. Facilite l’analyse des informations disponibles. N’augmente ni vos réserves d’énergie ni votre bande passante."
        }
    }
}

const fn primary_attribute_summary(attribute: PrimaryAttribute) -> &'static str {
    match attribute {
        PrimaryAttribute::Power => "Mêlée · recul · charge",
        PrimaryAttribute::Coordination => "Précision · esquive · discrétion",
        PrimaryAttribute::Resilience => "PV maximum · stabilité",
        PrimaryAttribute::Perception => "Détection · analyse · tir",
        PrimaryAttribute::Processing => "Intrusion · défense logicielle",
    }
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

fn draw_hud_card(rect: Rect, label: &str, value: &str, accent: Color, high_contrast: bool) {
    let theme = UiTheme;
    theme.card(rect, false);
    if high_contrast {
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, accent);
    }
    draw_rectangle(rect.x + 6.0, rect.y + 7.0, 3.0, rect.h - 14.0, accent);
    draw_text_bold(label, rect.x + 15.0, rect.y + 17.0, 12.0, accent);
    draw_wrapped_text(
        value,
        rect.x + 15.0,
        rect.y + 36.0,
        rect.w - 26.0,
        2,
        14,
        theme.text(),
    );
}

fn draw_control_hint(x: f32, y: f32, binding: &str, label: &str) -> f32 {
    let (key_width, label_width) = control_hint_widths(binding, label);
    draw_rectangle(x, y, key_width, 24.0, UiTheme.surface_raised());
    draw_rectangle_lines(x, y, key_width, 24.0, 1.0, UiTheme.accent());
    draw_text_bold_centered(
        binding,
        Rect::new(x, y, key_width, 24.0),
        13,
        UiTheme.text(),
    );
    draw_text(label, x + key_width + 7.0, y + 17.0, 14.0, UiTheme.muted());
    x + key_width + label_width + 25.0
}

fn control_hint_widths(binding: &str, label: &str) -> (f32, f32) {
    (
        (measure_text_bold(binding, 13).width + 14.0).max(27.0),
        measure_text(label, None, 14, 1.0).width,
    )
}

fn preparation_continue_rect(width: f32, height: f32) -> Rect {
    Rect::new((width - 294.0).max(20.0), height - 101.0, 274.0, 38.0)
}

fn wait_turn_rect(height: f32, binding: &str) -> Rect {
    // Input tests and headless adapters have no Macroquad text context. Keep
    // the clickable area deterministic and slightly generous while the
    // renderer continues to use the exact measured glyph widths.
    let key_width = (binding.chars().count() as f32 * 7.5 + 14.0).max(27.0);
    let label_width = "Attendre".chars().count() as f32 * 8.0;
    Rect::new(20.0, height - 94.0, key_width + label_width + 18.0, 24.0)
}

fn draw_recommended_profile(rect: Rect, profile: PrimaryAttributes, minimum: u8, maximum: u8) {
    let theme = UiTheme;
    theme.card(rect, false);
    draw_text_bold(
        "PROFIL CONSEILLÉ",
        rect.x + 12.0,
        rect.y + 21.0,
        17.0,
        theme.focus(),
    );
    draw_text(
        "Modifiable · effets détaillés à l'étape suivante",
        rect.x + 12.0,
        rect.y + 39.0,
        13.0,
        theme.muted(),
    );
    let segments = usize::from(maximum.saturating_sub(minimum).saturating_add(1)).max(1);
    for (index, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
        let baseline = rect.y + 61.0 + index as f32 * 16.5;
        let value = profile.value(attribute);
        draw_text(
            primary_attribute_label(attribute),
            rect.x + 12.0,
            baseline,
            13.0,
            theme.text(),
        );
        draw_text(
            primary_attribute_summary(attribute),
            rect.x + (rect.w * 0.28).max(96.0),
            baseline,
            12.0,
            theme.muted(),
        );
        if rect.w >= 520.0 {
            let bar_x = rect.x + (rect.w * 0.67).max(242.0);
            let bar_width = (rect.w - (bar_x - rect.x) - 42.0).max(24.0);
            let gap = 3.0;
            let segment_width =
                (bar_width - gap * (segments.saturating_sub(1)) as f32) / segments as f32;
            let filled = usize::from(value.saturating_sub(minimum).saturating_add(1)).min(segments);
            for segment in 0..segments {
                draw_rectangle(
                    bar_x + segment as f32 * (segment_width + gap),
                    baseline - 8.0,
                    segment_width.max(2.0),
                    6.0,
                    if segment < filled {
                        theme.accent()
                    } else {
                        theme.muted()
                    },
                );
            }
        }
        draw_text_bold(
            value.to_string(),
            rect.x + rect.w - 29.0,
            baseline + 1.0,
            19.0,
            theme.focus(),
        );
    }
}

fn physical_damage_total(damage: DamageImpact) -> u16 {
    damage
        .components()
        .filter(|component| component.damage_type.is_physical())
        .fold(0_u16, |total, component| {
            total.saturating_add(component.amount)
        })
}

fn damage_component_types_label(damage: DamageImpact) -> String {
    let primary = damage.primary_damage_type();
    std::iter::once(primary)
        .chain(
            damage
                .components()
                .map(|component| component.damage_type)
                .filter(move |damage_type| *damage_type != primary),
        )
        .map(damage_type_label)
        .collect::<Vec<_>>()
        .join(" + ")
}

fn format_damage_impact(damage: DamageImpact) -> String {
    format!(
        "{} · {}",
        damage.raw_total(),
        damage_component_types_label(damage)
    )
}

fn format_damage_penetration(damage: DamageImpact) -> String {
    if damage.is_legacy_single() {
        return damage.primary_component().penetration.to_string();
    }
    let mut parts = Vec::new();
    if damage.armor_penetration() > 0 {
        parts.push(format!("Blindage {}", damage.armor_penetration()));
    }
    parts.extend(damage.components().filter_map(|component| {
        let penetration = damage.resistance_penetration(component.damage_type);
        (penetration > 0).then(|| {
            format!(
                "{} {penetration} %",
                damage_type_label(component.damage_type)
            )
        })
    }));
    if parts.is_empty() {
        "0".to_owned()
    } else {
        parts.join(" · ")
    }
}

fn draw_attribute_meter(
    x: f32,
    y: f32,
    width: f32,
    value: u8,
    minimum: u8,
    maximum: u8,
    accent: Color,
) {
    let segments = usize::from(maximum.saturating_sub(minimum).saturating_add(1)).max(1);
    let gap = 2.0;
    let segment_width = (width - gap * segments.saturating_sub(1) as f32) / segments as f32;
    let filled = usize::from(value.saturating_sub(minimum).saturating_add(1)).min(segments);
    for index in 0..segments {
        draw_rectangle(
            x + index as f32 * (segment_width + gap),
            y - 4.0,
            segment_width.max(1.0),
            7.0,
            if index < filled {
                accent
            } else {
                UiTheme.surface_raised()
            },
        );
    }
}

fn draw_compact_metric(x: f32, y: f32, label: &str, value: &str, width: f32) {
    draw_text(label, x, y, 12.0, UiTheme.muted());
    let label_width = measure_text(label, None, 12, 1.0).width + 7.0;
    draw_wrapped_text(
        value,
        x + label_width,
        y,
        (width - label_width).max(18.0),
        1,
        15,
        UiTheme.text(),
    );
}

fn draw_summary_metric(x: f32, right: f32, y: f32, label: &str, value: &str) {
    draw_text(label, x, y, 14.0, UiTheme.muted());
    let available = (right - x) * 0.54;
    draw_wrapped_text(
        value,
        right - available,
        y,
        available,
        1,
        15,
        UiTheme.text(),
    );
}

const fn technique_kind_label(kind: TechniqueKind) -> &'static str {
    match kind {
        TechniqueKind::Action => "Action",
        TechniqueKind::Posture => "Posture",
        TechniqueKind::Procedure => "Procédure",
        TechniqueKind::Behavior => "Comportement",
        TechniqueKind::Improvement => "Amélioration",
    }
}

const fn inventory_category_order(category: InventoryFilter) -> u8 {
    match category {
        InventoryFilter::Weapons => 0,
        InventoryFilter::Armor => 1,
        InventoryFilter::Consumables => 2,
        InventoryFilter::Materials => 3,
        InventoryFilter::All => 4,
    }
}

fn draw_inventory_glyph(rect: Rect, glyph: InventoryGlyph, color: Color) {
    let pattern = match glyph {
        InventoryGlyph::Blade => [
            0b0000010, 0b0000100, 0b0001000, 0b0010000, 0b1110000, 0b0100000, 0b1010000,
        ],
        InventoryGlyph::RangedWeapon => [
            0b0000000, 0b0111110, 0b1111111, 0b0001110, 0b0011000, 0b0010000, 0b0000000,
        ],
        InventoryGlyph::FlameProjector => [
            0b0000010, 0b0011111, 0b1111110, 0b0111111, 0b0011000, 0b0010000, 0b0000000,
        ],
        InventoryGlyph::Armor => [
            0b0111110, 0b1111111, 0b1100011, 0b1100011, 0b0111110, 0b0011100, 0b0001000,
        ],
        InventoryGlyph::Consumable => [
            0b0011100, 0b0010100, 0b1111111, 0b1010101, 0b1111111, 0b0010100, 0b0011100,
        ],
        InventoryGlyph::Material => [
            0b0101010, 0b0011100, 0b1111111, 0b1101011, 0b1111111, 0b0011100, 0b0101010,
        ],
        InventoryGlyph::Unknown => [
            0b0111110, 0b1100011, 0b0000110, 0b0001100, 0b0000000, 0b0001100, 0b0000000,
        ],
    };
    let cell = (rect.w.min(rect.h) / 7.0).max(1.0);
    let x = rect.x + (rect.w - cell * 7.0) * 0.5;
    let y = rect.y + (rect.h - cell * 7.0) * 0.5;
    for (row, bits) in pattern.into_iter().enumerate() {
        for column in 0..7 {
            if bits & (1 << (6 - column)) != 0 {
                draw_rectangle(
                    x + column as f32 * cell,
                    y + row as f32 * cell,
                    (cell - 0.45).max(1.0),
                    (cell - 0.45).max(1.0),
                    color,
                );
            }
        }
    }
}

const fn menu_button_tone(menu: MenuScreen, index: usize, has_suspension: bool) -> ButtonTone {
    match menu {
        MenuScreen::Main if (has_suspension && index == 0) || (!has_suspension && index == 1) => {
            ButtonTone::Primary
        }
        MenuScreen::Pause if index == 0 => ButtonTone::Primary,
        MenuScreen::Pause if index == 3 => ButtonTone::Danger,
        MenuScreen::Graphics if index == 8 => ButtonTone::Primary,
        MenuScreen::ConfirmGraphics if index == 1 => ButtonTone::Primary,
        MenuScreen::ConfirmAbandon | MenuScreen::ConfirmNewRun if index == 1 => ButtonTone::Danger,
        _ => ButtonTone::Secondary,
    }
}

fn command_rejection_message(reason: CommandRejection) -> &'static str {
    match reason {
        CommandRejection::PassageUnavailable => "Passage indisponible.",
        CommandRejection::PassageObstructed => "Arrivée encombrée : attendez avant de réessayer.",
        CommandRejection::InteractionOutOfReach => {
            "Interaction hors de portée : placez-vous juste à côté."
        }
        CommandRejection::NothingToInteract => "Aucune interaction ici.",
        CommandRejection::DoorLocked => "Porte verrouillée : cherchez la console de commande.",
        CommandRejection::DoorUnpowered => {
            "Porte sans alimentation : le relais associé doit être remis en service."
        }
        CommandRejection::DoorObstructed => "Porte encombrée : fermeture impossible.",
        CommandRejection::ControlUnavailable => "Cette console n'a plus d'action disponible.",
        CommandRejection::NoMaterialForDepot => {
            "Vous ne transportez aucune pièce actuellement demandée par ce dépôt."
        }
        CommandRejection::FacilityUnavailable => "L'installation ne répond pas pour le moment.",
        CommandRejection::ProtectedZone => {
            "Zone protégée : aucune attaque depuis ou vers la ville."
        }
        CommandRejection::RunEnded => "Cette partie est terminée.",
        CommandRejection::NotPlayersTurn => "Attendez votre tour.",
        CommandRejection::CompanionCommandsUnavailable => {
            "Les commandes de compagnon ne sont pas disponibles dans cette partie."
        }
        CommandRejection::NoControlledCompanion => "Aucun compagnon contrôlé n'est actif.",
        CommandRejection::CompanionLinkUnavailable => {
            "Liaison interrompue : rapprochez-vous du compagnon avant de modifier son comportement."
        }
        CommandRejection::OffensiveActionBlockedByRecovery { .. } => {
            "Vous récupérez encore. Déplacez-vous, attendez ou choisissez une action de soutien autorisée."
        }
        CommandRejection::MissingPlayer => "Connexion au noyau perdue.",
        CommandRejection::BlockedByTerrain(_) => "Passage bloqué.",
        CommandRejection::Occupied(_) => "Case occupée.",
        CommandRejection::MovementDisabledByFailedComponent => {
            "Déplacement impossible : la fonction de locomotion est défaillante."
        }
        CommandRejection::UnknownTarget(_) => "Cible perdue.",
        CommandRejection::MissingAttackSlot(_) => "Aucune arme équipée sur cet emplacement.",
        CommandRejection::AttackSlotDisabledByFailedComponent(_) => {
            "Cette fonction d'attaque est défaillante."
        }
        CommandRejection::TargetOutOfRange(_) => "Cible hors de portée.",
        CommandRejection::NoLineOfSight(_) => "Aucune ligne de vue.",
        CommandRejection::AttackTargetOutsideMap(_) => "Zone ciblée hors de la carte.",
        CommandRejection::AttackTargetIsOrigin => "Visez ailleurs que votre position.",
        CommandRejection::AttackTargetOutOfRange(_) => "Zone ciblée hors de portée.",
        CommandRejection::AttackNoLineOfSight(_) => "Aucune ligne de vue vers la zone ciblée.",
        CommandRejection::FreeAimRequiresAreaWeapon => "Cette arme exige une cible précise.",
        CommandRejection::InsufficientAmmunition { .. } => {
            "Munitions insuffisantes pour résoudre cette action."
        }
        CommandRejection::WeaponModuleUnavailable(_) => {
            "Ce module est détruit ou temporairement affecté à une dérivation."
        }
        CommandRejection::WeaponModuleHeatLimit { .. } => {
            "Cette utilisation dépasserait la limite thermique du module."
        }
        CommandRejection::MissingEquipmentSlot(_) => "Emplacement d'équipement inconnu.",
        CommandRejection::UnknownInventoryItem(_) => "Cet objet n'est plus disponible.",
        CommandRejection::ItemIsNotWeapon(_) => "Cet objet n'est pas une arme.",
        CommandRejection::ItemAlreadyEquippedInSlot { .. } => {
            "Arme déjà équipée sur cet emplacement."
        }
        CommandRejection::UnknownEquipmentSlot(_) => "Emplacement de protection inconnu.",
        CommandRejection::ItemCannotEquipInSlot { .. } => {
            "Cet objet n'est pas compatible avec cet emplacement."
        }
        CommandRejection::ItemAlreadyEquipped { .. } => "Cette protection est déjà équipée.",
        CommandRejection::ItemIsNotUsable(_) => "Cet objet ne s'utilise pas directement.",
        CommandRejection::ItemHasNoUsefulEffect(_) => "Les PV sont déjà au maximum.",
        CommandRejection::InventoryChanged(_) => "L'inventaire a changé ; réessayez.",
        CommandRejection::NoItemToPickUp => "Aucun objet ici.",
        CommandRejection::InventoryCannotFitItem => "L'inventaire ne peut pas contenir cette pile.",
        CommandRejection::UnknownGroundItemDefinition => "Données du butin indisponibles.",
        CommandRejection::CannotDropItemHere => "Éloignez-vous du butin présent avant de déposer.",
        CommandRejection::MissingAbilitySlot(_) => "Compétence indisponible.",
        CommandRejection::AbilityTargetOutsideMap(_) => "Zone ciblée hors de la carte.",
        CommandRejection::AbilityTargetBlocked(_) => "Zone ciblée bloquée.",
        CommandRejection::AbilityTargetOutOfRange(_) => "Cible de compétence hors de portée.",
        CommandRejection::AbilityNoLineOfSight(_) => "Aucune ligne de vue pour cette compétence.",
        CommandRejection::AbilityTargetHasNoActor(_) => "Aucun acteur sur la cible.",
        CommandRejection::AbilityUnknownStatusDefinition => "Données de compétence indisponibles.",
        CommandRejection::TechniqueLearning(_) => "Cette technique ne peut pas être apprise.",
        CommandRejection::InsufficientSkillPoints { .. } => "Points de compétence insuffisants.",
        CommandRejection::UnknownTechnique(_) => "Données de la technique indisponibles.",
        CommandRejection::TechniqueNotLearned(_) => {
            "Apprenez cette technique dans l'écran Compétences."
        }
        CommandRejection::TechniqueHasNoActiveAction(_) => "Cette technique n'est pas une action.",
        CommandRejection::TechniqueMissingTarget => "Sélectionnez une cible visible.",
        CommandRejection::TechniqueUnexpectedTarget => "Cette technique ne demande aucune cible.",
        CommandRejection::TechniqueTargetNotVisible(_) => "La cible n'est pas visible.",
        CommandRejection::TechniqueTargetOutOfRange(_) => {
            "La cible est hors de portée des capteurs."
        }
        CommandRejection::TechniqueTargetHasNoEnergyState(_) => {
            "Cette cible ne possède aucun état énergétique analysable."
        }
        CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(_) => {
            "La cible ne remplit pas la condition requise par cette technique."
        }
        CommandRejection::TechniqueTooManyTargets { .. } => "Trop de cibles pour cette technique.",
        CommandRejection::TechniqueDuplicateTarget(_) => {
            "Une même cible est sélectionnée plusieurs fois."
        }
        CommandRejection::TechniqueVolleyTargetsTooFarApart { .. } => {
            "Les cibles de cette rafale sont trop éloignées les unes des autres."
        }
        CommandRejection::TechniqueUnknownBodyComponent(_) => {
            "Ce composant n'existe plus sur la cible."
        }
        CommandRejection::TechniqueBodyComponentNotIdentified { .. } => {
            "Ce composant doit être identifié avant le tir localisé."
        }
        CommandRejection::TechniqueRequiresParryWeapon => {
            "Équipez une arme apte à parer avant de préparer cette garde."
        }
        CommandRejection::TechniqueRequiresWeaponSlot => {
            "Sélectionnez un emplacement d'arme pour cette technique."
        }
        CommandRejection::TechniqueUnexpectedWeaponSlot => {
            "Cette technique n'utilise pas d'emplacement d'arme."
        }
        CommandRejection::TechniqueWeaponDeliveryMismatch { .. } => {
            "L'arme active n'est pas compatible avec cette technique."
        }
        CommandRejection::TechniqueWeaponHasNoPhysicalDamage => {
            "L'arme active ne possède aucune composante physique à renforcer."
        }
        CommandRejection::TechniqueWeaponHasNoImpact => {
            "L'arme active ne transmet aucun Impact utilisable pour cette poussée."
        }
        CommandRejection::TechniqueWeaponHasNoAutomaticFire => {
            "L'arme active ne possède aucun mode de tir automatique compatible."
        }
        CommandRejection::TechniqueWeaponMustTargetSingleActor => {
            "Cette poussée exige une arme visant une seule cible."
        }
        CommandRejection::TechniqueOnCooldown { .. } => "Cette technique est encore en recharge.",
        CommandRejection::TechniqueManifestationLimitReached { .. } => {
            "La limite de manifestations actives de cette compétence est atteinte."
        }
        CommandRejection::TechniqueMissingMaterial { .. } => {
            "Matériel requis absent ou en quantité insuffisante."
        }
        CommandRejection::TechniqueExplosivePositionNotVisible(_) => {
            "Le point de pose doit être visible."
        }
        CommandRejection::TechniqueExplosivePositionOutOfRange(_) => {
            "Point de pose ou dispositif hors de portée."
        }
        CommandRejection::TechniqueExplosiveLineBlocked(_) => {
            "La liaison ou la trajectoire vers ce point est bloquée."
        }
        CommandRejection::TechniqueExplosivePlacementBlocked(_) => {
            "Cette case ne peut pas recevoir ce dispositif."
        }
        CommandRejection::TechniqueRequiresDestructible(_) => {
            "Cette pose exige un obstacle destructible."
        }
        CommandRejection::TechniqueRequiresStructuralSupport(_) => {
            "Cette pose exige un support structurel identifié."
        }
        CommandRejection::TechniqueNoCompatibleExplosive(_) => {
            "Aucun dispositif compatible à cet endroit."
        }
        CommandRejection::TechniqueExplosiveStateChanged => {
            "L'état du dispositif a changé ; recommencez l'opération."
        }
        CommandRejection::TechniqueExplosiveInventoryFull => {
            "Inventaire plein : impossible de récupérer la charge."
        }
        CommandRejection::TechniqueUnknownMaterial(_) => {
            "La définition du matériel explosif est indisponible."
        }
        CommandRejection::TechniqueInvalidMovementDestination(_) => {
            "Cette destination n'est pas compatible avec la manœuvre."
        }
        CommandRejection::TechniqueMovementPositionNotVisible(_) => {
            "La destination de la manœuvre doit être visible."
        }
        CommandRejection::TechniqueMovementPathBlocked(_) => {
            "La trajectoire de la manœuvre est bloquée."
        }
        CommandRejection::TechniqueRequiresCompatibleObstacle(_) => {
            "Le franchissement exige un intervalle compatible d'une case."
        }
        CommandRejection::TechniqueRequiresCooperativeAlly(_) => {
            "Extraction impossible : la cible n'est pas un allié coopératif et transportable."
        }
        CommandRejection::TechniqueTargetAlreadyLocalized(_) => {
            "Cette cible vous a déjà localisé : l'embuscade ne peut pas être préparée."
        }
        CommandRejection::TechniqueRequiresBrokenLineOfSight => {
            "Rupture impossible : un observateur vous localise encore optiquement."
        }
        CommandRejection::TechniqueIncompatibleStealthPosture => {
            "Cette action est incompatible avec la posture de furtivité active."
        }
        CommandRejection::TechniqueActiveEmissionSilenced => {
            "Le canal d'émission actif est silencieux : réactivez-le pour cette commande."
        }
        CommandRejection::TechniqueDroneDirectiveMismatch => {
            "Cette consigne ne correspond pas à la technique de drone sélectionnée."
        }
        CommandRejection::TechniqueDroneNotControlled(_) => {
            "Ce drone n'est pas une unité actuellement contrôlée."
        }
        CommandRejection::TechniqueDroneNotLinked(_) => {
            "Liaison impossible avec ce drone depuis la position actuelle."
        }
        CommandRejection::TechniqueDroneMissingCapability(_) => {
            "Le châssis de ce drone ne possède pas le module requis."
        }
        CommandRejection::TechniqueDronePositionUnknown(_) => {
            "La destination doit être une case connue, praticable et non protégée."
        }
        CommandRejection::TechniqueDroneCargoUnavailable(_) => {
            "Le drone ou le chargement demandé n'est plus disponible."
        }
        CommandRejection::TechniqueDroneCargoTooHeavy { .. } => {
            "Ce chargement dépasse la capacité réelle du manipulateur du drone."
        }
        CommandRejection::TechniqueDroneHasNoAttack(_) => {
            "Ce drone ne possède aucune attaque ordinaire compatible."
        }
        CommandRejection::TechniqueInvalidDroneRoutine => {
            "Routine de drone invalide : vérifiez les limites et paramètres choisis."
        }
        CommandRejection::TechniqueEngineeringDirectiveMismatch => {
            "Ces paramètres ne correspondent pas à la technique d'ingénierie sélectionnée."
        }
        CommandRejection::TechniqueEngineeringComponentNotRepairable => {
            "Ce composant n'est pas endommagé ou sa destruction le rend irréparable."
        }
        CommandRejection::TechniqueEngineeringUnknownWreck(_) => {
            "Cette carcasse n'est plus disponible."
        }
        CommandRejection::TechniqueEngineeringUnknownModule(_) => {
            "Ce module n'est plus présent dans l'inventaire."
        }
        CommandRejection::TechniqueEngineeringModuleNotCompatible(_) => {
            "Ce module ne possède pas de sortie chiffrée compatible."
        }
        CommandRejection::TechniqueEngineeringWorkshopRequired => {
            "Cette intervention longue exige un atelier accessible."
        }
        CommandRejection::TechniqueEngineeringBypassInvalid => {
            "La dérivation exige une fonction dégradée réparable et un donneur opérationnel distinct."
        }
        CommandRejection::TechniqueEngineeringPlacementBlocked(_) => {
            "La balise exige une case adjacente, libre et praticable."
        }
        CommandRejection::TechniqueIntrusionDirectiveMismatch => {
            "Ces paramètres ne correspondent pas à la technique d'intrusion sélectionnée."
        }
        CommandRejection::TechniqueNoDigitalInterface(_) => {
            "Aucune interface numérique compatible à cette position."
        }
        CommandRejection::TechniqueDigitalInterfaceOutOfRange(_) => {
            "Interface hors de portée de la liaison locale."
        }
        CommandRejection::TechniqueDigitalLinkBlocked(_) => {
            "Liaison directe bloquée ou interface encore inconnue."
        }
        CommandRejection::TechniqueDigitalAccessRequired { .. } => {
            "La session ne possède pas le droit numérique requis."
        }
        CommandRejection::TechniqueDigitalCredentialRequired(_) => {
            "Aucun identifiant extrait ne permet cette usurpation locale."
        }
        CommandRejection::TechniqueDigitalCommandUnsupported(_) => {
            "Ce dispositif ne prend pas en charge cette consigne."
        }
        CommandRejection::TechniqueTooManyBackdoors { .. } => {
            "La capacité maximale de portes dérobées est déjà occupée."
        }
        CommandRejection::TechniqueUnknownSecurityTrace(_) => {
            "Cette trace de sécurité n'existe plus dans le registre local."
        }
        CommandRejection::TechniqueSecurityTraceAlreadyResolved(_) => {
            "Cette trace a déjà été falsifiée ou examinée."
        }
        CommandRejection::TechniqueDigitalRoutineProtected(_) => {
            "Cette routine est encore active ou protégée contre une nouvelle suspension."
        }
        CommandRejection::TechniqueDigitalControlRequired(_) => {
            "Le dispositif doit déjà être détourné avant de verrouiller son contrôle."
        }
        CommandRejection::TechniqueDigitalSubnetInvalid => {
            "Le sous-réseau choisi doit contenir un à trois dispositifs distincts et compatibles."
        }
        CommandRejection::TechniqueElectronicDirectiveMismatch => {
            "Ces paramètres ne correspondent pas à la technique de guerre électronique sélectionnée."
        }
        CommandRejection::TechniqueElectronicTargetIncompatible(_) => {
            "Cette cible ne possède aucun système électronique compatible."
        }
        CommandRejection::TechniqueElectronicLinkBlocked(_) => {
            "La liaison électronique vers cette cible est bloquée ou hors de portée."
        }
        CommandRejection::TechniqueElectronicProgramFamilyActive(_) => {
            "Un programme hostile de cette famille est déjà actif sur la cible."
        }
        CommandRejection::TechniqueUnknownHostileProgram(_) => {
            "Ce programme hostile n'est plus présent."
        }
        CommandRejection::TechniqueElectronicProgramTargetMismatch => {
            "Le programme hostile n'appartient pas à cette cible."
        }
        CommandRejection::TechniqueElectronicCascadeInvalid => {
            "La chaîne électronique doit contenir des cibles distinctes et reliées dans l'ordre."
        }
        CommandRejection::TechniqueUnknownSaturationBeacon(_) => {
            "Cette balise de saturation n'est plus présente."
        }
        CommandRejection::TechniqueSaturationBeaconUnavailable(_) => {
            "Cette balise ne peut pas être activée par le joueur dans son état actuel."
        }
        CommandRejection::TechniqueElectronicPlacementBlocked(_) => {
            "La balise exige une case adjacente, libre et praticable."
        }
        CommandRejection::TechniqueElectronicStoredEnergyInsufficient { .. } => {
            "La cible ne contient pas assez d'énergie stockée pour cette implosion."
        }
        CommandRejection::TechniqueElectronicProgramTargetChanged(_) => {
            "L'état énergétique de la cible a changé avant la fin de la procédure."
        }
        CommandRejection::InsufficientEnergy { .. } => "Énergie insuffisante.",
        CommandRejection::InsufficientBandwidth { .. } => "Bande passante insuffisante.",
        CommandRejection::SystemResourcesUnavailable => "Bus de ressources système indisponible.",
    }
}

const fn companion_behavior_label(behavior: CompanionBehavior) -> &'static str {
    match behavior {
        CompanionBehavior::Follow => "Suivre",
        CompanionBehavior::Defensive => "Défensif",
        CompanionBehavior::Aggressive => "Agressif",
        CompanionBehavior::Passive => "Passif",
    }
}

fn attack_preview_rejection_label(reason: &CommandRejection) -> &'static str {
    match reason {
        CommandRejection::ProtectedZone => "ZONE PROTÉGÉE",
        CommandRejection::AttackTargetOutsideMap(_) => "HORS CARTE",
        CommandRejection::AttackTargetIsOrigin => "CHOISISSEZ UNE DIRECTION",
        CommandRejection::AttackTargetOutOfRange(_) => "HORS DE PORTÉE",
        CommandRejection::AttackNoLineOfSight(_) => "LIGNE DE VUE BLOQUÉE",
        CommandRejection::TechniqueExplosivePositionNotVisible(_) => "CASE NON VISIBLE",
        CommandRejection::TechniqueExplosivePositionOutOfRange(_) => "HORS DE PORTÉE",
        CommandRejection::TechniqueExplosiveLineBlocked(_) => "LIAISON BLOQUÉE",
        CommandRejection::TechniqueExplosivePlacementBlocked(_) => "POSE IMPOSSIBLE",
        CommandRejection::TechniqueRequiresDestructible(_) => "OBSTACLE REQUIS",
        CommandRejection::TechniqueRequiresStructuralSupport(_) => "SUPPORT REQUIS",
        CommandRejection::TechniqueNoCompatibleExplosive(_) => "AUCUN DISPOSITIF",
        CommandRejection::FreeAimRequiresAreaWeapon => "ARME SANS ZONE",
        CommandRejection::MissingAttackSlot(_) => "EMPLACEMENT VIDE",
        _ => "VISÉE INDISPONIBLE",
    }
}

fn technique_id(value: &str) -> Option<TechniqueId> {
    value.parse().ok()
}

fn movement_direction(action: Action) -> Option<Direction> {
    match action {
        Action::MoveNorth => Some(Direction::North),
        Action::MoveEast => Some(Direction::East),
        Action::MoveSouth => Some(Direction::South),
        Action::MoveWest => Some(Direction::West),
        _ => None,
    }
}

fn visual_cue_id(value: &str) -> VisualCueId {
    value
        .parse()
        .expect("built-in visual cue IDs must be canonical")
}

fn read_movement_command(
    game: &WorldState,
    attack_slot: u8,
    controls: &Controls,
    input: &InputFrame,
) -> Option<GameCommand> {
    let direction = if controls.pressed(Action::MoveNorth, input) {
        Some(Direction::North)
    } else if controls.pressed(Action::MoveEast, input) {
        Some(Direction::East)
    } else if controls.pressed(Action::MoveSouth, input) {
        Some(Direction::South)
    } else if controls.pressed(Action::MoveWest, input) {
        Some(Direction::West)
    } else {
        None
    };

    if let Some(direction) = direction {
        let player_position = game.player_position()?;
        let destination = player_position.step(direction);
        if let Some(target) = game.actors().entity_at(destination)
            && target != game.player_id()
            && game.active_worker_role(target).is_none()
        {
            return Some(GameCommand::Attack {
                slot: attack_slot,
                target,
            });
        }
        return Some(GameCommand::Move(direction));
    }

    controls
        .pressed(Action::Wait, input)
        .then_some(GameCommand::Wait)
}

fn pressed_weapon_slot(controls: &Controls, input: &InputFrame) -> Option<u8> {
    if controls.pressed(Action::Slot1, input) {
        Some(0)
    } else if controls.pressed(Action::Slot2, input) {
        Some(1)
    } else if controls.pressed(Action::Slot3, input) {
        Some(2)
    } else {
        None
    }
}

fn grid_distance(first: GridPos, second: GridPos) -> u32 {
    let delta_x = (i64::from(first.x) - i64::from(second.x)).unsigned_abs();
    let delta_y = (i64::from(first.y) - i64::from(second.y)).unsigned_abs();
    delta_x.max(delta_y).min(u64::from(u32::MAX)) as u32
}

const fn region_direction(direction: Direction) -> RegionDirection {
    match direction {
        Direction::North => RegionDirection::North,
        Direction::East => RegionDirection::East,
        Direction::South => RegionDirection::South,
        Direction::West => RegionDirection::West,
    }
}

const fn direction_from_region(direction: RegionDirection) -> Direction {
    match direction {
        RegionDirection::North => Direction::North,
        RegionDirection::East => Direction::East,
        RegionDirection::South => Direction::South,
        RegionDirection::West => Direction::West,
    }
}

fn regional_route_direction(
    origin: RegionCoord,
    destination: RegionCoord,
) -> Option<RegionDirection> {
    let horizontal = origin.x.abs_diff(destination.x);
    let vertical = origin.y.abs_diff(destination.y);
    if horizontal >= vertical && origin.x != destination.x {
        Some(if destination.x > origin.x {
            RegionDirection::East
        } else {
            RegionDirection::West
        })
    } else if origin.y != destination.y {
        Some(if destination.y > origin.y {
            RegionDirection::South
        } else {
            RegionDirection::North
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{Binding, Layout};
    use project_rl::world::Map;

    fn input(key: &str) -> InputFrame {
        InputFrame {
            pressed: if key == "Escape" {
                Default::default()
            } else {
                [Binding::key(key)].into()
            },
            pause: key == "Escape",
            ..Default::default()
        }
    }

    fn app_with_test_controls() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.controls = Controls::preset(Layout::Azerty, KeySemantics::Physical);
        app.suspension_path = temporary_folder("test-run").join("suspended-run.json");
        app
    }

    fn temporary_folder(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "project-rl-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    fn apply(app: &mut AsciiApp, command: GameCommand) {
        let outcome = app.execute_command(command);
        assert!(
            !matches!(outcome, CommandOutcome::Rejected(_)),
            "{outcome:?}"
        );
        app.capture_events_at(Some(0.0));
    }

    fn menu_pointer(menu: MenuScreen, index: usize, clicked: bool) -> InputFrame {
        let layout = MenuLayout::new(1280.0, 800.0, menu.buttons().len());
        let rect = layout.buttons[index];
        InputFrame {
            pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
            viewport: Some((1280.0, 800.0)),
            pressed: if clicked {
                [Binding::MouseLeft].into()
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }

    fn rect_pointer(rect: Rect, wheel_y: f32) -> InputFrame {
        InputFrame {
            pointer: Some((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
            viewport: Some((1280.0, 800.0)),
            pressed: [Binding::MouseLeft].into(),
            wheel_y,
            ..Default::default()
        }
    }

    fn accept_recommended_character(app: &mut AsciiApp) {
        assert_eq!(
            app.character_creation.as_ref().map(|state| state.stage),
            Some(CharacterCreationStage::Protocol)
        );
        app.update_input(&input("Enter"));
        assert_eq!(
            app.character_creation.as_ref().map(|state| state.stage),
            Some(CharacterCreationStage::Attributes)
        );
        app.update_input(&input("Enter"));
        assert!(app.character_creation.is_none());
        assert!(app.character_class.is_some());
    }

    fn spawn_ui_test_drone(app: &mut AsciiApp, starting_energy: u16) -> EntityId {
        let player = app.game.player_position().unwrap();
        let position = player
            .cardinal_neighbors()
            .into_iter()
            .find(|position| {
                app.game.map().is_walkable(*position)
                    && app.game.actors().entity_at(*position).is_none()
            })
            .expect("the fixture must leave one adjacent cell for its companion");
        let profile = DroneProfile::new(
            "core:ui_test_drone".parse().unwrap(),
            6,
            50,
            40,
            1,
            3,
            6,
            1,
            1,
            DroneCapabilities::default(),
        )
        .unwrap();
        let drone = app
            .game
            .spawn_manifested_player_drone(
                Actor::new(position, 10).unwrap(),
                profile,
                4,
                starting_energy,
            )
            .unwrap();
        app.capture_events_at(Some(0.0));
        drone
    }

    #[test]
    fn menu_hover_does_not_need_a_click_and_keyboard_keeps_focus_until_mouse_moves() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Pause);
        app.update_input(&menu_pointer(MenuScreen::Pause, 1, false));
        assert_eq!(app.menu, MenuScreen::Pause);
        assert_eq!(app.menu_selection, 1);
        assert!(app.menu_focus.highlighted(1, app.menu_selection));
        let mut keyboard = menu_pointer(MenuScreen::Pause, 1, false);
        keyboard.pressed.insert(Binding::key("Down"));
        app.update_input(&keyboard);
        assert_eq!(app.menu_selection, 2);
        app.update_input(&menu_pointer(MenuScreen::Pause, 1, false));
        assert_eq!(app.menu_selection, 2);
        assert!(app.menu_focus.highlighted(2, app.menu_selection));
        app.update_input(&InputFrame {
            pointer: Some((1.0, 1.0)),
            ..Default::default()
        });
        assert!(!app.menu_focus.highlighted(2, app.menu_selection));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
        assert!(!app.should_quit());
    }

    #[test]
    fn preparation_footer_click_waits_and_executes_the_persisted_technique() {
        let mut app = app_with_test_controls();
        app.character_creation = None;
        // A generation-54 suspension still expects the historical repeated
        // technique command. The shared footer must remain usable there too.
        app.rules.wait_continues_technique_preparation = false;
        let mut game = GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            INITIAL_SEED,
            app.rules.clone(),
        )
        .unwrap();
        let technique: TechniqueId = "core:mel_02".parse().unwrap();
        assert_eq!(
            game.process_player_command(GameCommand::LearnTechnique {
                technique: technique.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let target = game
            .spawn_actor(Actor::new(GridPos::new(2, 1), 20).unwrap())
            .unwrap();
        game.drain_events();
        app.game = WorldState::single(game);

        assert_eq!(
            app.execute_command(GameCommand::UseTechnique {
                technique: technique.clone(),
                targets: vec![target],
                weapon_slot: Some(0),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.game
                .player_technique_preparation()
                .map(|preparation| preparation.remaining_steps().get()),
            Some(1)
        );
        let turn_before = app.game.turn();

        app.update_input(&rect_pointer(preparation_continue_rect(1280.0, 800.0), 0.0));

        assert_eq!(app.game.turn(), turn_before + 1);
        assert!(app.game.player_technique_preparation().is_none());
        assert!(
            app.log
                .iter()
                .any(|message| message.contains("préparation achevée"))
        );
    }

    #[test]
    fn footer_wait_button_advances_an_ordinary_turn_without_movement() {
        let mut app = app_with_test_controls();
        app.character_creation = None;
        let position_before = app.game.player_position();
        let turn_before = app.game.turn();
        let wait_binding = app.controls.label(Action::Wait);

        app.update_input(&rect_pointer(wait_turn_rect(800.0, &wait_binding), 0.0));

        assert_eq!(app.game.turn(), turn_before + 1);
        assert_eq!(app.game.player_position(), position_before);
    }

    #[test]
    fn companion_bar_changes_behavior_without_spending_a_turn() {
        let mut app = app_with_test_controls();
        app.character_creation = None;
        let drone = spawn_ui_test_drone(&mut app, 4);
        assert!(
            app.floating_messages
                .iter()
                .any(|message| message.text == "+ DRONE")
        );
        let layout = CompanionBarLayout::new(1280.0, 800.0);
        let turn_before = app.game.turn();

        app.update_input(&rect_pointer(layout.behavior_buttons[2], 0.0));

        assert_eq!(app.game.turn(), turn_before);
        assert!(matches!(
            app.game
                .actors()
                .get(drone)
                .and_then(Actor::drone)
                .map(|drone| drone.order()),
            Some(DroneOrder::Companion {
                behavior: CompanionBehavior::Aggressive,
                ..
            })
        ));
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::CompanionBehavior {
                behavior: suspension::RecordedCompanionBehavior::Aggressive,
            })
        ));
        assert!(app.log.iter().any(|line| line.contains("Agressif")));
    }

    #[test]
    fn depleted_drone_disappears_with_an_explicit_end_of_life_message() {
        let mut app = app_with_test_controls();
        app.character_creation = None;
        let drone = spawn_ui_test_drone(&mut app, 0);

        apply(&mut app, GameCommand::Wait);

        assert!(app.game.actors().get(drone).is_none());
        assert!(!app.actor_glyphs.contains_key(&drone));
        assert!(
            app.floating_messages
                .iter()
                .any(|message| message.text == "- DRONE")
        );
        assert!(app.log.iter().any(|line| {
            line.contains("Fin de vie du drone") && line.contains("de nouveau utilisable")
        }));
    }

    #[test]
    fn holding_a_movement_binding_repeats_real_turns_after_a_short_delay() {
        let mut app = app_with_test_controls();
        let start = app.game.player_position().unwrap();
        let pressed = InputFrame {
            pressed: [Binding::key("D")].into(),
            held: [Binding::key("D")].into(),
            ..Default::default()
        };
        app.update_input_at(&pressed, Some(5.0));
        assert_eq!(
            app.game.player_position(),
            Some(start.step(Direction::East))
        );
        let first_turn = app.game.turn();

        let held = InputFrame {
            held: [Binding::key("D")].into(),
            ..Default::default()
        };
        app.update_input_at(&held, Some(5.27));
        assert_eq!(app.game.turn(), first_turn);
        app.update_input_at(&held, Some(5.28));
        assert_eq!(app.game.turn(), first_turn + 1);
        assert_eq!(
            app.game.player_position(),
            Some(start.step(Direction::East).step(Direction::East))
        );

        app.open_menu(MenuScreen::Pause);
        let before_menu = app.game.player_position();
        app.update_input_at(&held, Some(6.0));
        assert_eq!(app.game.player_position(), before_menu);
        app.menu = MenuScreen::Hidden;
        app.update_input_at(&held, Some(7.0));
        assert_eq!(app.game.player_position(), before_menu);
    }

    #[test]
    fn clicking_outside_menu_never_activates_a_mouse_bound_confirmation() {
        let mut app = app_with_test_controls();
        app.controls
            .rebind(Action::Learn, Binding::MouseLeft)
            .unwrap();
        app.open_menu(MenuScreen::ConfirmAbandon);
        app.menu_selection = 1;
        app.update_input(&InputFrame {
            pointer: Some((1.0, 1.0)),
            viewport: Some((1280.0, 800.0)),
            pressed: [Binding::MouseLeft].into(),
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::ConfirmAbandon);
        assert!(!app.should_quit());
        app.update_input(&menu_pointer(MenuScreen::ConfirmAbandon, 0, true));
        assert_eq!(app.menu, MenuScreen::Pause);
    }

    #[test]
    fn controls_support_hover_click_wheel_and_clickable_back_without_advancing_time() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Controls);
        let mut pointer = InputFrame {
            pointer: Some((50.0, 180.0)),
            viewport: Some((700.0, 480.0)),
            ..Default::default()
        };
        app.update_input(&pointer);
        assert_eq!(app.options_selection, 1);
        assert!(!app.rebinding);
        pointer.pressed.insert(Binding::MouseLeft);
        app.update_input(&pointer);
        assert!(app.rebinding);
        assert_eq!(app.controls.binding(Action::MoveNorth), &Binding::key("W"));
        app.update_input(&input("Escape"));
        assert!(!app.rebinding);
        pointer.pressed.clear();
        pointer.wheel_y = -3.0;
        app.update_input(&pointer);
        assert_eq!(app.options_scroll, 3);
        assert_eq!(app.options_selection, 4);
        let layout = ControlsLayout::new(700.0, 480.0, 3, Action::ALL.len() + 1);
        pointer.pointer = Some((layout.back.x + 5.0, layout.back.y + 5.0));
        pointer.wheel_y = 0.0;
        pointer.pressed.insert(Binding::MouseLeft);
        app.update_input(&pointer);
        assert_eq!(app.menu, MenuScreen::Options);
        app.open_menu(MenuScreen::Hidden);
        app.observation_report = (0..20).map(|index| format!("Ligne {index}")).collect();
        app.report_open = true;
        app.update_input(&InputFrame {
            wheel_y: -3.0,
            ..Default::default()
        });
        assert_eq!(app.report_scroll, 3);
        app.update_input(&InputFrame {
            wheel_y: 2.0,
            ..Default::default()
        });
        assert_eq!(app.report_scroll, 1);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn graphics_menu_applies_confirms_persists_and_preserves_run_and_controls() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("graphics-menu");
        app.graphics.path = folder.join("graphics.json");
        app.controls_path = folder.join("controls.json");
        app.controls.save(&app.controls_path).unwrap();
        let bindings = std::fs::read(&app.controls_path).unwrap();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Options);
        app.update_input(&menu_pointer(MenuScreen::Options, 1, true));
        assert_eq!(app.menu, MenuScreen::Graphics);
        // The automatic desktop resolution and future renderer cannot be selected.
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 2);
        app.update_input(&input("Right"));
        assert_eq!(app.graphics.draft.ui_scale_percent, 125);
        assert_eq!(app.graphics.active.ui_scale_percent, 100);
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 4);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 3, true));
        assert_eq!(app.menu_selection, 4);
        assert_eq!(app.menu_focus.hovered, None);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 0, true));
        assert_eq!(app.graphics.draft.mode, WindowMode::Windowed);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 1, true));
        assert_eq!(app.graphics.draft.windowed_size, [1600, 900]);
        app.update_input(&menu_pointer(MenuScreen::Graphics, 8, true));
        assert_eq!(app.menu, MenuScreen::ConfirmGraphics);
        assert!(app.graphics.previewing());
        assert!(!app.graphics.path.exists());
        // Click the confirmation using physical coordinates at the new UI scale.
        let scale = app.graphics.active.ui_scale(1600.0, 900.0);
        let layout = MenuLayout::new(1600.0 / scale, 900.0 / scale, 2);
        let keep = layout.buttons[1];
        let pointer = app.graphics.active.transform_input(InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((
                (keep.x + keep.w / 2.0) * scale,
                (keep.y + keep.h / 2.0) * scale,
            )),
            viewport: Some((1600.0, 900.0)),
            ..Default::default()
        });
        app.update_input(&pointer);
        assert_eq!(app.menu, MenuScreen::Graphics);
        assert!(!app.graphics.previewing());
        assert_eq!(
            GraphicsSettings::load(&app.graphics.path).0,
            app.graphics.active
        );
        assert_eq!(std::fs::read(&app.controls_path).unwrap(), bindings);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
        // New runs retain display choices, just like bindings.
        let settings = app.graphics.active;
        app.open_menu(MenuScreen::Hidden);
        app.update_input(&input("R"));
        assert_eq!(app.graphics.active, settings);
        assert_eq!(app.graphics.path, folder.join("graphics.json"));
        std::fs::remove_file(app.graphics.path).unwrap();
        std::fs::remove_file(app.controls_path).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn graphics_escape_and_timeout_restore_settings_without_consuming_turns() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        for exit in ["Escape", "timeout"] {
            app.open_menu(MenuScreen::Graphics);
            app.update_input(&input("Enter")); // Borderless -> windowed draft.
            assert_eq!(app.graphics.active.mode, WindowMode::Borderless);
            app.update_input(&menu_pointer(MenuScreen::Graphics, 8, true));
            assert_eq!(app.menu, MenuScreen::ConfirmGraphics);
            for key in ["W", "Space", "F", "R"] {
                app.update_input(&input(key));
            }
            match exit {
                "Escape" => app.update_input(&input("Escape")),
                "timeout" => {
                    assert!(app.tick_graphics(16.0));
                }
                _ => unreachable!(),
            }
            assert_eq!(app.graphics.active.mode, WindowMode::Borderless);
            assert!(!app.graphics.previewing());
            assert!(!app.should_quit());
        }
        app.open_menu(MenuScreen::Graphics);
        app.update_input(&input("Enter"));
        app.update_input(&input("Escape")); // Discard an unapplied draft too.
        app.open_menu(MenuScreen::Graphics);
        assert_eq!(app.graphics.draft, app.graphics.active);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn normal_exit_saves_once_and_no_input_can_change_the_saved_snapshot() {
        for exit in ["keyboard", "mouse", "window"] {
            let mut app = app_with_test_controls();
            apply(&mut app, GameCommand::Wait);
            let before = suspension::fingerprint(&app.game);
            let history = app.history.clone();
            app.open_menu(MenuScreen::Pause);
            assert_eq!(app.menu_labels()[2], "Sauvegarder et quitter");
            match exit {
                "keyboard" => {
                    app.menu_selection = 2;
                    app.update_input(&input("Enter"));
                }
                "mouse" => app.update_input(&menu_pointer(MenuScreen::Pause, 2, true)),
                _ => {
                    // Closing even from the abandonment dialog uses the safe exit.
                    app.open_menu(MenuScreen::ConfirmAbandon);
                    app.request_quit();
                }
            }
            assert!(app.should_quit(), "{}", app.menu_message);
            let saved = Suspension::read(&app.suspension_path).unwrap();
            assert_eq!(saved.state, before);
            assert_eq!(saved.commands, history);
            let bytes = std::fs::read(&app.suspension_path).unwrap();
            app.request_quit(); // Repeated close events cannot overwrite or fail the save.
            for key in ["Escape", "Space", "R", "W", "Enter"] {
                app.update_input(&input(key));
            }
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            app.resume_run().unwrap();
            assert!(!app.should_quit());
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert!(!app.suspension_path.exists()); // Still a single-use suspension.
            std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn window_close_reverts_unconfirmed_graphics_and_saves_the_active_run() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        let display = app.graphics.active;
        app.open_menu(MenuScreen::Graphics);
        app.update_input(&input("Enter"));
        app.update_input(&menu_pointer(MenuScreen::Graphics, 8, true));
        assert!(app.graphics.previewing());
        app.request_quit();
        assert!(app.should_quit());
        assert!(!app.graphics.previewing());
        assert_eq!(app.graphics.active, display);
        assert_eq!(
            Suspension::read(&app.suspension_path).unwrap().state,
            before
        );
        app.resume_run().unwrap();
        assert_eq!(app.graphics.active, display);
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn failed_window_close_keeps_run_open_and_can_retry_without_losing_progress() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("close-failure");
        std::fs::create_dir(&folder).unwrap();
        let blocker = folder.join("not-a-directory");
        std::fs::write(&blocker, "fixture").unwrap();
        app.suspension_path = blocker.join("run.json");
        apply(&mut app, GameCommand::Wait);
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Controls);
        app.rebinding = true;
        app.request_quit();
        assert!(!app.should_quit());
        assert_eq!(app.menu, MenuScreen::Pause);
        assert!(!app.rebinding);
        assert!(app.menu_message.contains("Sauvegarde impossible"));
        assert!(app.menu_message.contains("reste ouverte"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(std::fs::read_to_string(&blocker).unwrap(), "fixture");
        app.suspension_path = folder.join("run.json");
        app.request_quit();
        assert!(app.should_quit());
        app.resume_run().unwrap();
        assert_eq!(suspension::fingerprint(&app.game), before);
        std::fs::remove_file(blocker).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn leaving_the_main_menu_or_abandoning_never_removes_an_existing_suspension() {
        for exit in ["window", "button", "abandon"] {
            let mut app = app_with_test_controls();
            let folder = app.suspension_path.parent().unwrap().to_path_buf();
            std::fs::create_dir(&folder).unwrap();
            std::fs::write(
                &app.suspension_path,
                "existing incompatible suspension fixture",
            )
            .unwrap();
            let bytes = std::fs::read(&app.suspension_path).unwrap();
            if exit == "abandon" {
                app.request_quit();
                assert!(!app.should_quit()); // Normal exit refuses to overwrite.
                assert!(app.menu_message.contains("Sauvegarde impossible"));
                app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
                app.update_input(&menu_pointer(MenuScreen::ConfirmAbandon, 1, true));
                assert_eq!(app.menu, MenuScreen::Main);
                assert!(!app.should_quit());
                app.update_input(&menu_pointer(MenuScreen::Main, 3, true));
            } else {
                app.open_menu(MenuScreen::Main);
                if exit == "window" {
                    app.request_quit();
                } else {
                    app.update_input(&menu_pointer(MenuScreen::Main, 3, true));
                }
            }
            assert!(app.should_quit());
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            std::fs::remove_file(&app.suspension_path).unwrap();
            std::fs::remove_dir(folder).unwrap();
        }
    }

    #[test]
    fn pause_buttons_options_and_abandon_confirmation_are_modal_and_clickable() {
        let mut app = app_with_test_controls();
        assert_eq!(
            MenuScreen::ConfirmAbandon.buttons(),
            &["Annuler", "Confirmer"]
        );
        let before = suspension::fingerprint(&app.game);
        app.update_input(&input("F1"));
        assert!(app.legend_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        app.update_input(&input("W"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&input("Escape"));
        assert!(!app.legend_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        for key in ["W", "F", "C", "Space", "R"] {
            app.update_input(&input(key));
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
        let layout = MenuLayout::new(1280.0, 800.0, 4);
        let button = layout.buttons[1];
        app.update_input(&InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((button.x + 10.0, button.y + 10.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Controls);
        app.update_input(&input("Escape"));
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        app.menu_selection = 3;
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::ConfirmAbandon);
        assert!(!app.should_quit());
        assert_eq!(app.menu_selection, 0);
        app.update_input(&input("Enter")); // default = cancel
        assert_eq!(app.menu, MenuScreen::Pause);
        app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
        app.update_input(&input("Escape"));
        assert!(!app.should_quit());
        app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        assert!(!app.should_quit());
        assert_eq!(app.menu, MenuScreen::Main);
        assert!(!app.suspension_path.exists());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn main_menu_can_resume_replace_and_restart_without_closing_the_game() {
        let mut app = app_with_test_controls();
        let folder = app.suspension_path.parent().unwrap().to_path_buf();
        app.open_menu(MenuScreen::Main);
        assert_eq!(
            app.menu_labels(),
            [
                "Reprendre la partie (indisponible)",
                "Nouvelle partie",
                "Options",
                "Quitter",
            ]
        );
        assert!(!app.menu_row_enabled(0));
        assert_eq!(app.menu_selection, 1);
        app.update_input(&menu_pointer(MenuScreen::Main, 2, true));
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Main);

        apply(&mut app, GameCommand::Wait);
        app.suspension()
            .unwrap()
            .write(&app.suspension_path)
            .unwrap();
        let suspended = std::fs::read(&app.suspension_path).unwrap();
        app.open_menu(MenuScreen::Main);
        assert!(app.menu_row_enabled(0));
        app.update_input(&menu_pointer(MenuScreen::Main, 1, true));
        assert_eq!(app.menu, MenuScreen::ConfirmNewRun);
        assert_eq!(
            MenuScreen::ConfirmNewRun.buttons(),
            &["Annuler", "Confirmer"]
        );
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Main);
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), suspended);

        app.update_input(&menu_pointer(MenuScreen::Main, 1, true));
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        accept_recommended_character(&mut app);
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(app.game.turn(), 0);
        assert!(!app.suspension_path.exists());

        apply(&mut app, GameCommand::Wait);
        app.suspend_run().unwrap();
        app.open_menu(MenuScreen::Main);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Hidden, "{}", app.menu_message);
        assert_eq!(app.game.turn(), 1);
        assert!(!app.suspension_path.exists());

        app.open_menu(MenuScreen::Pause);
        app.update_input(&menu_pointer(MenuScreen::Pause, 3, true));
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Main);
        assert_eq!(app.game.turn(), 0);
        assert!(!app.should_quit());
        app.update_input(&menu_pointer(MenuScreen::Main, 1, true));
        accept_recommended_character(&mut app);
        assert_eq!(app.menu, MenuScreen::Hidden);
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn new_game_selects_a_protocol_and_distributes_attributes_before_turn_zero() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Main);
        app.update_input(&menu_pointer(MenuScreen::Main, 1, true));

        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(app.game.turn(), 0);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(
            app.character_creation.as_ref().map(|state| state.stage),
            Some(CharacterCreationStage::Protocol)
        );

        app.update_input(&input("Down")); // BRÈCHE -> CREUSET
        app.update_input(&input("Enter"));
        app.update_input(&input("Left")); // Puissance 4 -> 3, one point freed.
        app.update_input(&input("Down"));
        app.update_input(&input("Right")); // Coordination 5 -> 6.
        app.update_input(&input("Enter"));

        assert!(app.character_creation.is_none());
        assert_eq!(
            app.character_class.as_ref().map(ContentId::as_str),
            Some("core:creuset")
        );
        assert_eq!(
            app.game.player_primary_attributes(),
            Some(PrimaryAttributes::new(3, 6, 7, 5, 7))
        );
        assert_eq!(app.game.turn(), 0);
        assert!(app.game.equipped_player_weapon(0).is_none());
        assert!(app.game.equipped_player_weapon(1).is_none());
        assert_eq!(
            app.game
                .equipped_player_weapon(2)
                .map(|weapon| weapon.id().as_str()),
            Some("core:flamethrower")
        );
        assert_eq!(app.active_weapon_slot, 2);
    }

    #[test]
    fn character_sheet_is_modal_free_and_escape_closes_it_before_pause() {
        let mut app = app_with_test_controls();
        let before = suspension::fingerprint(&app.game);

        app.update_input(&input("J"));
        assert!(app.character_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
        app.update_input(&input("Escape"));
        assert!(!app.character_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn escape_closes_each_gameplay_overlay_before_opening_pause() {
        let mut app = app_with_test_controls();
        app.observation_report.push("Relevé daté".to_owned());
        apply(
            &mut app,
            GameCommand::LearnTechnique {
                technique: "core:rec_01".parse().unwrap(),
            },
        );
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());

        for open in [
            Action::Inventory,
            Action::Skills,
            Action::Character,
            Action::Report,
            Action::Legend,
            Action::QuickTechniques,
        ] {
            let binding = app.controls.binding(open).clone();
            app.update_input(&InputFrame {
                pressed: [binding].into(),
                ..Default::default()
            });
            assert!(
                app.inventory_open
                    || app.skills_open
                    || app.character_open
                    || app.report_open
                    || app.legend_open
                    || app.technique_menu_open
            );
            app.update_input(&input("Escape"));
            assert_eq!(app.menu, MenuScreen::Hidden);
            assert!(!app.inventory_open);
            assert!(!app.skills_open);
            assert!(!app.character_open);
            assert!(!app.report_open);
            assert!(!app.legend_open);
            assert!(!app.technique_menu_open);
        }

        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
    }

    #[test]
    fn u_opens_the_learned_active_techniques_and_confirms_the_selection() {
        let mut app = app_with_test_controls();
        for technique in ["core:rec_01", "core:rec_02"] {
            apply(
                &mut app,
                GameCommand::LearnTechnique {
                    technique: technique.parse().unwrap(),
                },
            );
        }
        assert_eq!(app.active_learned_techniques().len(), 2);
        let turn_before = app.game.turn();

        app.update_input(&input("U"));
        assert!(app.technique_menu_open);
        assert_eq!(app.game.turn(), turn_before);
        app.update_input(&input("Down"));
        assert_eq!(app.technique_menu_selection, 1);
        assert_eq!(app.game.turn(), turn_before);
        app.update_input(&input("U"));

        assert!(!app.technique_menu_open);
        assert_eq!(app.game.turn(), turn_before + 1);
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::Technique { technique, .. })
                if technique == "core:rec_02"
        ));
    }

    #[test]
    fn technique_quick_menu_supports_mouse_selection_and_confirmation() {
        let mut app = app_with_test_controls();
        for technique in ["core:rec_01", "core:rec_02"] {
            apply(
                &mut app,
                GameCommand::LearnTechnique {
                    technique: technique.parse().unwrap(),
                },
            );
        }
        let turn_before = app.game.turn();
        app.update_input(&input("U"));

        let layout = TechniqueQuickMenuLayout::new(1280.0, 800.0, 0, 2);
        let second_row = layout
            .rows
            .iter()
            .find(|(index, _)| *index == 1)
            .map(|(_, row)| *row)
            .unwrap();
        app.update_input(&rect_pointer(second_row, 0.0));
        assert_eq!(app.technique_menu_selection, 1);
        assert_eq!(app.game.turn(), turn_before);

        let layout = TechniqueQuickMenuLayout::new(1280.0, 800.0, 1, 2);
        app.update_input(&rect_pointer(layout.actions[0], 0.0));
        assert!(!app.technique_menu_open);
        assert_eq!(app.game.turn(), turn_before + 1);
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::Technique { technique, .. })
                if technique == "core:rec_02"
        ));
    }

    #[test]
    fn u_without_an_active_learned_technique_is_non_mutating_and_explains_why() {
        let mut app = app_with_test_controls();
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());

        app.update_input(&input("U"));

        assert!(!app.technique_menu_open);
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("Aucune technique active apprise"))
        );
    }

    #[test]
    fn live_level_gain_opens_skills_after_the_turn_but_replay_does_not() {
        fn queue_level_gain(app: &mut AsciiApp) {
            let mut rules = app.rules.clone();
            rules.progression.curve = project_rl::progression::ExperienceCurve::new(vec![1])
                .expect("one valid level threshold");
            rules.progression.skill_points_per_level = 1;
            rules.hit_rules = None;
            let mut game = GameState::new_with_rules(
                project_rl::world::Map::from_ascii("#####\n#...#\n#####")
                    .expect("valid level-up test map"),
                GridPos::new(1, 1),
                17,
                rules.clone(),
            )
            .expect("valid level-up test game");
            let target = game
                .spawn_actor(
                    Actor::new(GridPos::new(2, 1), 1)
                        .expect("valid adjacent target")
                        .with_defeat_reward(DefeatReward::persistent(10, 1)),
                )
                .expect("target can spawn");
            game.drain_events();
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            assert!(
                game.events()
                    .iter()
                    .any(|event| matches!(event, GameEvent::LevelGained { level: 2, .. }))
            );
            app.rules = rules;
            app.game = WorldState::single(game);
        }

        let mut live = app_with_test_controls();
        queue_level_gain(&mut live);
        let resolved_turn = live.game.turn();
        live.capture_events();
        assert_eq!(live.game.turn(), resolved_turn);
        assert!(live.skills_open);
        assert_eq!(
            live.level_up_notice,
            Some(LevelUpNotice {
                level: 2,
                skill_points_awarded: 1,
            })
        );
        live.update_input(&input("Escape"));
        assert!(!live.skills_open);
        assert_eq!(live.level_up_notice, None);
        assert_eq!(live.menu, MenuScreen::Hidden);

        let mut replay = app_with_test_controls();
        queue_level_gain(&mut replay);
        replay.capture_events_at(Some(0.0));
        assert!(!replay.skills_open);
        assert_eq!(replay.level_up_notice, None);
    }

    #[test]
    fn floating_feedback_is_stacked_bounded_and_purely_transient() {
        let mut app = app_with_test_controls();
        let at = app.game.player_position().unwrap();
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());

        for index in 0..40 {
            app.push_floating_message(format!("−{index}"), at, FloatingMessageTone::Damage, 0.0);
        }
        assert_eq!(app.floating_messages.len(), FLOATING_MESSAGE_CAPACITY);
        assert!(
            app.floating_messages
                .iter()
                .all(|message| message.lane <= 3)
        );

        app.push_floating_message("NIVEAU 2", at, FloatingMessageTone::Progression, 4.0);
        assert_eq!(app.floating_messages.len(), 1);
        assert_eq!(app.floating_messages[0].text, "NIVEAU 2");
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
    }

    #[test]
    fn character_inventory_and_skills_share_clickable_non_turn_navigation() {
        let mut app = app_with_test_controls();
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());

        app.update_input(&input("J"));
        let character = CharacterLayout::new(1280.0, 800.0);
        app.update_input(&rect_pointer(character.attribute_rows[3], 0.0));
        assert_eq!(app.character_attribute_selection, 3);
        app.update_input(&rect_pointer(character.actions[0], 0.0));
        assert!(app.inventory_open);
        assert!(!app.character_open);

        let inventory = InventoryLayout::new(
            1280.0,
            800.0,
            app.inventory_selection,
            app.inventory_entries().len(),
        );
        assert!(inventory.stats_panel.is_some());
        assert!(
            InventoryLayout::new(960.0, 540.0, 0, app.inventory_entries().len())
                .stats_panel
                .is_none()
        );
        app.update_input(&rect_pointer(inventory.character_details.unwrap(), 0.0));
        assert!(app.character_open);
        assert!(!app.inventory_open);
        let character = CharacterLayout::new(1280.0, 800.0);
        app.update_input(&rect_pointer(character.actions[0], 0.0));
        assert!(app.inventory_open);
        assert!(!app.character_open);

        let inventory = InventoryLayout::new(
            1280.0,
            800.0,
            app.inventory_selection,
            app.inventory_entries().len(),
        );
        app.update_input(&rect_pointer(inventory.filters[3], 0.0));
        assert_eq!(app.inventory_filter, InventoryFilter::Consumables);
        assert!(
            app.inventory_entries()
                .iter()
                .all(|entry| app.inventory_category(entry) == InventoryFilter::Consumables)
        );
        let inventory = InventoryLayout::new(
            1280.0,
            800.0,
            app.inventory_selection,
            app.inventory_entries().len(),
        );
        app.update_input(&rect_pointer(inventory.sort, 0.0));
        assert_eq!(app.inventory_sort, InventorySort::Name);
        let inventory = InventoryLayout::new(
            1280.0,
            800.0,
            app.inventory_selection,
            app.inventory_entries().len(),
        );
        app.update_input(&rect_pointer(inventory.actions[5], 0.0));
        assert!(!app.inventory_open);

        app.update_input(&input("K"));
        let techniques = app.selected_skill_techniques();
        let skills = SkillsLayout::new(
            1280.0,
            800.0,
            app.skill_disciplines().len(),
            app.skill_technique_selection,
            techniques.len(),
        );
        if let Some((index, row)) = skills.technique_rows.last() {
            app.update_input(&rect_pointer(*row, 0.0));
            assert_eq!(app.skill_technique_selection, *index);
        }
        app.update_input(&rect_pointer(skills.actions[2], 0.0));
        assert!(!app.skills_open);
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
    }

    #[test]
    fn skills_layout_keeps_navigation_list_and_description_in_distinct_panels() {
        let overlaps = |left: Rect, right: Rect| {
            left.x < right.x + right.w
                && left.x + left.w > right.x
                && left.y < right.y + right.h
                && left.y + left.h > right.y
        };
        let contains = |panel: Rect, child: Rect| {
            child.x >= panel.x
                && child.y >= panel.y
                && child.x + child.w <= panel.x + panel.w
                && child.y + child.h <= panel.y + panel.h
        };

        for (width, height) in [(960.0, 540.0), (1280.0, 800.0), (1920.0, 1080.0)] {
            let layout = SkillsLayout::new(width, height, 10, 9, 10);
            assert!(!overlaps(layout.discipline_panel, layout.technique_panel));
            assert!(!overlaps(layout.discipline_panel, layout.detail_panel));
            assert!(!overlaps(layout.technique_panel, layout.detail_panel));
            assert!(
                layout
                    .discipline_rows
                    .iter()
                    .all(|row| contains(layout.discipline_panel, *row))
            );
            assert!(
                layout
                    .technique_rows
                    .iter()
                    .all(|(_, row)| contains(layout.technique_panel, *row))
            );
        }
    }

    #[test]
    fn skill_availability_is_precomputed_once_for_the_renderer() {
        let app = app_with_test_controls();

        assert_eq!(
            app.skill_availability_cache.len(),
            app.game.rules().skills.disciplines().count()
        );
        for (discipline, _) in app.game.rules().skills.disciplines() {
            let expected = app.game.discipline_availability(discipline).unwrap();
            assert_eq!(
                app.skill_availability_cache.get(discipline),
                Some(&expected),
                "cached availability drifted for {discipline}"
            );
        }
    }

    #[test]
    fn technique_quick_menu_keeps_rows_and_actions_inside_its_panel() {
        let contains = |panel: Rect, child: Rect| {
            child.x >= panel.x
                && child.y >= panel.y
                && child.x + child.w <= panel.x + panel.w
                && child.y + child.h <= panel.y + panel.h
        };

        for (width, height) in [(960.0, 540.0), (1280.0, 800.0), (1920.0, 1080.0)] {
            let layout = TechniqueQuickMenuLayout::new(width, height, 9, 24);
            assert!(
                layout
                    .rows
                    .iter()
                    .all(|(_, row)| contains(layout.panel, *row))
            );
            assert!(
                layout
                    .actions
                    .iter()
                    .all(|button| contains(layout.panel, *button))
            );
            assert!(
                layout
                    .rows
                    .iter()
                    .all(|(_, row)| row.y + row.h <= layout.actions[0].y)
            );
            assert!(layout.rows.iter().any(|(index, _)| *index == 9));
        }
    }

    #[test]
    fn suspension_rebuilds_the_selected_protocol_before_replay() {
        let mut app = app_with_test_controls();
        app.begin_character_creation(false).unwrap();
        {
            let creation = app.character_creation.as_mut().unwrap();
            creation.selected_class = 2; // VIGIE in stable content-ID order.
            creation.stage = CharacterCreationStage::Attributes;
            creation.attributes = app
                .character_classes
                .iter()
                .nth(2)
                .unwrap()
                .1
                .recommended_attributes();
        }
        let creation = app.character_creation.clone().unwrap();
        app.rebuild_run_with_character_class(&creation).unwrap();
        apply(&mut app, GameCommand::Wait);
        let saved = app.suspension().unwrap();
        assert_eq!(saved.character_class.as_deref(), Some("core:vigie"));
        assert_eq!(saved.starting_attributes, Some([4, 8, 4, 8, 4]));

        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(restored.character_class, app.character_class);
        assert_eq!(
            restored.game.player_primary_attributes(),
            Some(PrimaryAttributes::new(4, 8, 4, 8, 4))
        );
        assert_eq!(
            restored
                .game
                .equipped_player_weapon(1)
                .map(|weapon| weapon.id().as_str()),
            Some("core:needle_launcher")
        );
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
    }

    #[test]
    fn breche_armor_can_be_equipped_by_click_and_survives_replay() {
        let mut app = app_with_test_controls();
        let breche: CharacterClassId = "core:breche".parse().unwrap();
        let (selected_class, attributes) = app
            .character_classes
            .iter()
            .enumerate()
            .find(|(_, (id, _))| *id == &breche)
            .map(|(index, (_, class))| (index, class.recommended_attributes()))
            .unwrap();
        app.rebuild_run_with_character_class(&CharacterCreation {
            stage: CharacterCreationStage::Attributes,
            selected_class,
            selected_attribute: 0,
            attributes,
            replace_suspension: false,
            message: String::new(),
            hovered: None,
        })
        .unwrap();
        app.inventory_open = true;
        app.inventory_filter = InventoryFilter::Armor;
        let armor_entry = app.inventory_entries().first().copied().unwrap();
        let armor_item = armor_entry.instance();
        assert_eq!(armor_entry.item().as_str(), "core:patched_plating");
        assert_eq!(
            app.game
                .actor_armor_profile(app.game.player_id())
                .map(project_rl::combat::ArmorProfile::after_fragilization),
            Some(1)
        );

        let layout = InventoryLayout::new(1280.0, 800.0, 0, 1);
        app.update_input(&rect_pointer(layout.actions[0], 0.0));

        let body_slot: ContentId = "core:body_armor".parse().unwrap();
        assert_eq!(
            app.game.player_equipment().equipped(&body_slot),
            Some(armor_item)
        );
        assert_eq!(
            app.game
                .actor_armor_profile(app.game.player_id())
                .map(project_rl::combat::ArmorProfile::after_fragilization),
            Some(2)
        );
        assert!(app.inventory_message.contains("Blindage total 2"));
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::EquipItem { slot, item })
                if slot == "core:body_armor" && *item == armor_item.get()
        ));

        let saved = app.suspension().unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(
            restored.game.player_equipment().equipped(&body_slot),
            Some(armor_item)
        );
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
    }

    #[test]
    fn suspension_replays_the_complete_run_and_is_consumed_only_once() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        rules.progression.starting_skill_points = 2;
        rules.player_maximum_integrity = 500;
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        let folder = temporary_folder("suspension");
        app.suspension_path = folder.join("suspended-run.json");
        app.session_lock = Some(suspension::session_lock(&folder.join("session.lock")).unwrap());
        for id in ["core:rec_01", "core:rec_02"] {
            apply(
                &mut app,
                GameCommand::LearnTechnique {
                    technique: technique_id(id).unwrap(),
                },
            );
        }
        let weapon = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:needle_launcher")
            .unwrap()
            .instance();
        apply(
            &mut app,
            GameCommand::EquipWeapon {
                slot: 2,
                item: weapon,
            },
        );
        app.active_weapon_slot = 2;
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        for _ in 0..150 {
            let origin = app.game.player_position().unwrap();
            if app.visible_targets().iter().any(|target| {
                grid_distance(origin, app.game.actors().get(*target).unwrap().position()) <= 4
            }) {
                break;
            }
            let goal = app
                .game
                .actors()
                .iter()
                .filter(|(id, _)| *id != app.game.player_id())
                .min_by_key(|(_, actor)| grid_distance(origin, actor.position()))
                .unwrap()
                .1
                .position();
            let path = project_rl::world::find_path(app.game.map(), origin, goal, 5000, |pos| {
                Some(pos) != app.game.exit()
            })
            .unwrap();
            let next = path[1];
            apply(
                &mut app,
                GameCommand::Move(
                    Direction::from_delta(next.x - origin.x, next.y - origin.y).unwrap(),
                ),
            );
        }
        let target = app.visible_targets()[0];
        app.selected_target = Some(target);
        apply(
            &mut app,
            GameCommand::UseTechnique {
                technique: technique_id("core:rec_01").unwrap(),
                targets: vec![target],
                weapon_slot: None,
            },
        );
        assert_eq!(app.game.player_energy().available(), 100);
        apply(
            &mut app,
            GameCommand::UseTechnique {
                technique: technique_id("core:rec_02").unwrap(),
                targets: vec![],
                weapon_slot: None,
            },
        );
        let repair = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:repair_patch")
            .unwrap()
            .instance();
        apply(&mut app, GameCommand::DropItem { item: repair });
        apply(&mut app, GameCommand::PickUp);
        let target = app.visible_targets()[0];
        let at = app.game.actors().get(target).unwrap().position();
        apply(
            &mut app,
            GameCommand::UseAbility {
                slot: 1,
                target: at,
            },
        );
        assert!(!app.observation_report.is_empty());
        let before = suspension::fingerprint(&app.game);
        let report = app.observation_report.clone();
        let history = app.history.clone();
        app.open_menu(MenuScreen::Pause);
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(app.should_quit(), "{}", app.menu_message);
        assert!(app.suspension_path.exists());
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.suspend_run().is_err()); // no overwrite of an existing suspended run
        let mut next = AsciiApp::from_seed(
            1,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        next.suspension_path = app.suspension_path.clone();
        next.session_lock = app.session_lock.take();
        next.open_menu(MenuScreen::Main);
        next.update_input(&input("Enter"));
        assert_eq!(next.menu, MenuScreen::Hidden, "{}", next.menu_message);
        assert_eq!(suspension::fingerprint(&next.game), before);
        assert_eq!(next.history, history);
        assert_eq!(next.observation_report, report);
        assert_eq!(next.active_weapon_slot, 2);
        assert!(!next.suspension_path.exists());
        assert!(next.resume_run().is_err());
        assert_eq!(suspension::fingerprint(&next.game), before);
        apply(&mut next, GameCommand::Wait);
        next.suspend_run().unwrap(); // a new suspension continues the same journal
        next.resume_run().unwrap();
        assert_eq!(next.game.player_energy().available(), 100);
        drop(next);
        std::fs::remove_file(folder.join("session.lock")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn version_fifty_two_replays_legacy_learning_journal_before_new_requirements() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        rules.progression.starting_skill_points = 9;
        let mut legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION,
        )
        .unwrap();
        assert!(
            !legacy
                .rules
                .skill_progression
                .enforces_authored_requirements()
        );
        for id in [
            "core:rec_01",
            "core:rec_02",
            "core:rec_09",
            "core:rec_04",
            "core:rec_05",
        ] {
            apply(
                &mut legacy,
                GameCommand::LearnTechnique {
                    technique: technique_id(id).unwrap(),
                },
            );
        }
        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, ELECTRONIC_WARFARE_SKILLS_GENERATION_VERSION);

        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.history, legacy.history);
    }

    #[test]
    fn a_click_resumes_an_older_build_only_after_an_identical_replay() {
        for key in [false, true] {
            let mut app = app_with_test_controls();
            app.walk_fixture_to(GridPos::new(32, 16)).unwrap();
            apply(
                &mut app,
                GameCommand::Interact {
                    target: GridPos::new(32, 15),
                },
            );
            let mut saved = app.suspension().unwrap();
            saved.build = "617e393cfa1b13c7".to_owned();
            assert_ne!(saved.build, env!("PROJECT_RL_BUILD_FINGERPRINT"));
            saved.write(&app.suspension_path).unwrap();
            let mut next = app_with_test_controls();
            next.suspension_path = app.suspension_path.clone();
            next.open_menu(MenuScreen::Main);
            next.update_input(&if key {
                input("Enter")
            } else {
                menu_pointer(MenuScreen::Main, 0, true)
            });
            assert_eq!(next.menu, MenuScreen::Hidden, "{}", next.menu_message);
            assert_eq!(suspension::fingerprint(&next.game), saved.state);
            assert_eq!(next.history, saved.commands);
            assert_eq!(
                next.game.map().tile(GridPos::new(32, 15)),
                app.game.map().tile(GridPos::new(32, 15))
            );
            assert!(!next.suspension_path.exists());
            assert!(next.resume_run().is_err());
            next.suspend_run().unwrap();
            assert_eq!(
                Suspension::read(&next.suspension_path).unwrap().build,
                env!("PROJECT_RL_BUILD_FINGERPRINT")
            );
            std::fs::remove_file(&next.suspension_path).unwrap();
            std::fs::remove_dir(next.suspension_path.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn cross_build_replay_rejects_incompatible_data_and_preserves_file_and_game() {
        for fault in [
            "version", "build", "rules", "loot", "world", "state", "command", "slot", "target",
        ] {
            let mut app = app_with_test_controls();
            let mut saved = app.suspension().unwrap();
            saved.build = "617e393cfa1b13c7".to_owned();
            match fault {
                "version" => saved.version = 255,
                "build" => saved.build = "invalid".to_owned(),
                "rules" => saved.rules ^= 1,
                "loot" => saved.loot_rules = saved.loot_rules.map(|fingerprint| fingerprint ^ 1),
                "world" => saved.world_rules = saved.world_rules.map(|fingerprint| fingerprint ^ 1),
                "state" => saved.state ^= 1,
                "command" => saved
                    .commands
                    .push(RecordedCommand::Move { direction: 255 }),
                "slot" => saved.active_weapon_slot = 255,
                "target" => saved.selected_target = Some(u64::MAX),
                _ => unreachable!(),
            }
            let bytes = serde_json::to_vec(&saved).unwrap();
            std::fs::create_dir_all(app.suspension_path.parent().unwrap()).unwrap();
            std::fs::write(&app.suspension_path, &bytes).unwrap();
            let before = suspension::fingerprint(&app.game);
            app.open_menu(MenuScreen::Main);
            app.update_input(&menu_pointer(MenuScreen::Main, 0, true));
            assert_eq!(app.menu, MenuScreen::Main, "{fault}");
            assert!(!app.should_quit());
            assert!(
                app.menu_message.starts_with("Reprise impossible : "),
                "{fault}"
            );
            assert_eq!(app.menu_labels()[0], "Réessayer la reprise");
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            app.request_quit();
            assert!(app.should_quit());
            assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
            std::fs::remove_file(&app.suspension_path).unwrap();
            std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
        }
    }

    /// Explicit local diagnostic, never run by the normal suite. Read-only:
    /// verifies a chosen real suspension without consuming it or creating a run.
    #[test]
    #[ignore = "Set PROJECT_RL_CHECK_SUSPENSION to a file to verify without consuming it"]
    fn check_suspension_file_without_consuming() {
        let path = std::path::PathBuf::from(
            std::env::var_os("PROJECT_RL_CHECK_SUSPENSION")
                .expect("explicit suspension path required"),
        );
        let before = std::fs::read(&path).unwrap();
        let saved = Suspension::read(&path).unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), before);
        let resaved = restored.suspension().unwrap();
        let checked = AsciiApp::restore_suspension(
            &resaved,
            restored.rules.clone(),
            restored.texts.clone(),
            restored.loot.clone(),
            restored.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&checked.game), resaved.state);
        println!(
            "Suspension vérifiée sans consommation : {} commandes, état {:016x}, position {:?}.",
            saved.commands.len(),
            saved.state,
            restored.game.player_position()
        );
    }

    #[test]
    fn failed_suspension_or_incompatible_resume_never_quits_or_consumes_data() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("bad-suspension");
        std::fs::create_dir(&folder).unwrap();
        app.suspension_path = folder.join("suspended-run.json");
        let mut saved = app.suspension().unwrap();
        saved.state ^= 1;
        saved.write(&app.suspension_path).unwrap();
        let bytes = std::fs::read(&app.suspension_path).unwrap();
        let before = suspension::fingerprint(&app.game);
        app.open_menu(MenuScreen::Main);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Main);
        assert!(!app.should_quit());
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
        assert_eq!(suspension::fingerprint(&app.game), before);
        saved.build = "another build".to_owned();
        assert!(
            AsciiApp::restore_suspension(
                &saved,
                app.rules.clone(),
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone()
            )
            .is_err()
        );
        let mut altered_rules = app.rules.clone();
        altered_rules.player_starting_energy -= 1;
        assert!(
            AsciiApp::restore_suspension(
                &app.suspension().unwrap(),
                altered_rules,
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone()
            )
            .is_err()
        );
        app.open_menu(MenuScreen::Pause);
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(!app.should_quit());
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), bytes);
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::write(folder.join("not-a-directory"), "block").unwrap();
        app.suspension_path = folder.join("not-a-directory/suspend.json");
        app.menu_selection = 2;
        app.update_input(&input("Enter"));
        assert!(!app.should_quit());
        assert!(app.menu_message.contains("Sauvegarde impossible"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        std::fs::remove_file(folder.join("not-a-directory")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn an_unrecorded_mutation_cannot_be_saved_and_session_lock_prevents_double_resume() {
        let mut app = app_with_test_controls();
        let folder = temporary_folder("session");
        let path = folder.join("session.lock");
        let guard = suspension::session_lock(&path).unwrap();
        assert!(suspension::session_lock(&path).is_err());
        drop(guard);
        let guard = suspension::session_lock(&path).unwrap();
        app.suspension_path = folder.join("suspended-run.json");
        app.game.process_player_command(GameCommand::Wait);
        app.game.drain_events();
        assert!(app.suspend_run().is_err());
        assert!(!app.suspension_path.exists());
        drop(guard);
        std::fs::remove_file(path).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn a_finished_run_cannot_create_a_resurrection_checkpoint() {
        let mut app = app_with_test_controls();
        let rules = GameRules {
            player_maximum_integrity: 3,
            ..GameRules::default()
        };
        let mut game = GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            INITIAL_SEED,
            rules,
        )
        .unwrap();
        game.spawn_actor(
            Actor::new(GridPos::new(2, 1), 10)
                .unwrap()
                .with_attack(AttackProfile::melee(DamageType::Kinetic, 500))
                .with_ai(AiProfile::hunter(8, 0)),
        )
        .unwrap();
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(game.status(), RunStatus::PlayerDestroyed);
        app.game = WorldState::single(game);

        assert_eq!(app.game.status(), RunStatus::PlayerDestroyed);
        assert!(app.suspension().is_err());
        app.open_menu(MenuScreen::Pause);
        assert_eq!(app.menu_labels()[2], "Quitter");
        assert!(!app.menu_row_enabled(3));
        app.menu_selection = 2;
        app.update_input(&input("Down"));
        assert_eq!(app.menu_selection, 2); // Skip the unavailable last row.
        app.update_input(&input("Enter"));
        assert!(app.should_quit());
        assert!(!app.suspension_path.exists());
    }

    #[test]
    fn interactions_use_the_binding_and_replay_door_console_and_remembered_states() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(32, 16)).unwrap();
        let door = GridPos::new(32, 15);
        let turn = app.game.turn();
        app.update_input(&input("V"));
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert_eq!(app.game.turn(), turn + 1);
        app.update_input(&input("V"));
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Closed)
        );
        app.walk_fixture_to(GridPos::new(50, 18)).unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: TestSector::CONTROL,
            },
        );
        app.walk_fixture_to(GridPos::new(52, 16)).unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: TestSector::LOCKED_DOOR,
            },
        );
        app.walk_fixture_to(TestSector::ARCHIVE_TERMINAL.step(Direction::West))
            .unwrap();
        app.facing = Direction::East;
        let terminal_turn = app.game.turn();
        app.update_input(&input("V"));
        assert_eq!(app.game.turn(), terminal_turn + 1);
        assert!(
            app.log
                .last()
                .is_some_and(|line| line.contains("ARCHIVE DÉCOUVERTE"))
        );
        app.update_input(&input("V"));
        assert!(
            app.log
                .last()
                .is_some_and(|line| line.contains("ARCHIVE CONSULTÉE"))
        );
        let terminal: ContentId = "core:starter_city_archive_terminal".parse().unwrap();
        assert!(
            app.game
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&terminal)
        );
        assert!(app.observation_report.is_empty());
        let dossier = app.dossier_lines();
        assert!(dossier.iter().any(|line| {
            line.kind == DossierLineKind::Content && line.text.contains("Registre local incomplet")
        }));
        assert!(!dossier.iter().any(|line| line.text.starts_with("core:")));
        app.update_input(&input("O"));
        assert!(app.report_open);
        app.update_input(&input("O"));
        assert!(!app.report_open);
        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
        assert!(
            restored
                .game
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&terminal)
        );
        assert_eq!(restored.dossier_lines(), dossier);
        for p in app.game.player_visibility().explored_positions() {
            assert_eq!(app.terminal.known(p), restored.terminal.known(p));
        }
    }

    #[test]
    fn expedition_round_trip_replays_every_zone_and_its_visual_memory() {
        let mut app = app_with_test_controls();
        app.walk_expedition_fixture(2).unwrap();
        assert_eq!(app.game.visited_zone_count(), 2);
        assert_eq!(app.game.current_zone().unwrap().depth, 1);
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, CURRENT_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.zone_views.len(), 1);
        assert_eq!(restored.history, app.history);
        for p in app.game.player_visibility().explored_positions() {
            assert_eq!(app.terminal.known(p), restored.terminal.known(p));
        }
        for (id, view) in &app.zone_views {
            let other = &restored.zone_views[id];
            for y in 0..68 {
                for x in 0..110 {
                    let p = GridPos::new(x, y);
                    assert_eq!(view.known(p), other.known(p));
                }
            }
        }
        let before = suspension::fingerprint(&app.game);
        app.update_input(&input("Escape"));
        for _ in 0..5 {
            app.update_input(&input("Space"));
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn maintenance_circuit_moves_one_real_part_and_restores_linked_functions() {
        let mut app = app_with_test_controls();
        let relay = "core:maintenance_relay".parse().unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        let order = "core:restore_checkpoint_power".parse().unwrap();
        let regulator = "core:power_regulator".parse().unwrap();
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Unpowered)
        );
        let initial = app.game.active_facility().unwrap();
        assert!(!initial.is_operational(&relay));
        assert!(!initial.is_operational(&sensor));
        assert_eq!(
            initial.repair_status(&order),
            Some(project_rl::facility::RepairStatus::WaitingForMaterial)
        );

        for _ in 0..96 {
            apply(&mut app, GameCommand::Wait);
        }

        let repaired = app.game.active_facility().unwrap();
        assert!(repaired.is_operational(&relay));
        assert!(repaired.is_operational(&sensor));
        assert_eq!(
            app.terminal.decor.cells.get(&GridPos::new(46, 32)),
            Some(&crate::test_sector::Decor::RelayOnline)
        );
        assert_eq!(
            repaired.repair_status(&order),
            Some(project_rl::facility::RepairStatus::Completed)
        );
        assert_eq!(repaired.depot_stock(&regulator), 0);
        assert_eq!(
            app.game.map().tile(GridPos::new(32, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert!(
            app.game
                .ground_items()
                .iter()
                .all(|(_, stack)| stack.item() != &regulator)
        );
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Closed)
        );
    }

    #[test]
    fn installed_security_alarm_is_visible_and_survives_replay() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let regulator: ItemId = "core:power_regulator".parse().unwrap();
        let owner: project_rl::social::SocialGroupId =
            "core:maintenance_collective".parse().unwrap();
        let mut alarm_expeditions = ExpeditionCatalog::default();
        for (id, definition) in expeditions.iter() {
            let mut definition = definition.clone();
            if id.as_str() == "core:starter_expedition" {
                definition.hub_facility.as_mut().unwrap().materials.push(
                    project_rl::content::FacilityMaterialSpawn {
                        position: GridPos::new(52, 27),
                        item: regulator.clone(),
                        quantity: 1,
                        owner: Some(owner.clone()),
                    },
                );
            }
            alarm_expeditions.register(definition).unwrap();
        }
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, alarm_expeditions).unwrap();
        for _ in 0..96 {
            apply(&mut app, GameCommand::Wait);
        }
        app.walk_fixture_to(GridPos::new(52, 29)).unwrap();
        app.walk_fixture_to(GridPos::new(52, 27)).unwrap();
        apply(&mut app, GameCommand::PickUp);

        assert_eq!(app.visible_security_alarm_summary(), Some((1, 8)));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("VERROUILLAGE DE SÉCURITÉ ACTIF"))
        );
        assert_eq!(
            app.game.map().tile(GridPos::new(52, 28)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Locked)
        );
        assert_eq!(
            app.game
                .active_facility()
                .and_then(|facility| {
                    facility.security_door_lockdown_at(GridPos::new(52, 28), app.game.turn())
                })
                .map(|lockdown| lockdown.remaining_turns(app.game.turn())),
            Some(8)
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, CURRENT_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.visible_security_alarm_summary(), Some((1, 8)));
        assert!(
            restored
                .game
                .active_facility()
                .and_then(|facility| facility
                    .security_door_lockdown_at(GridPos::new(52, 28), restored.game.turn()))
                .is_some()
        );
    }

    #[test]
    fn player_can_deliver_the_requested_material_and_replay_the_intervention() {
        let mut app = app_with_test_controls();
        let regulator: ItemId = "core:power_regulator".parse().unwrap();
        let order = "core:restore_checkpoint_power".parse().unwrap();
        app.walk_fixture_to(GridPos::new(16, 23)).unwrap();
        apply(&mut app, GameCommand::PickUp);
        assert!(
            app.game
                .player_inventory()
                .iter()
                .any(|entry| entry.item() == &regulator)
        );
        assert!(
            app.game
                .player_inventory()
                .iter()
                .find(|entry| entry.item() == &regulator)
                .and_then(|entry| entry.owner())
                .is_some()
        );
        assert!(
            app.game
                .actors()
                .iter()
                .any(|(_, actor)| !actor.observed_property_takes().is_empty())
        );
        assert!(app.game.actors().iter().any(|(_, actor)| {
            actor
                .local_alert()
                .is_some_and(|alert| alert.is_active(app.game.turn()))
        }));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("vous voit prendre"))
        );
        assert!(app.log.iter().any(|line| line.contains("ALERTE LOCALE")));
        assert_eq!(app.visible_local_alert_summary(), Some((1, 8)));
        app.walk_fixture_to(GridPos::new(27, 31)).unwrap();
        app.facing = Direction::East;
        assert_eq!(
            app.interaction_command(),
            Some(GameCommand::Interact {
                target: GridPos::new(28, 31),
            })
        );

        let before_turn = app.game.turn();
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: GridPos::new(28, 31),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(app.game.turn(), before_turn + 1);
        app.capture_events_at(Some(0.0));
        assert!(app.log.iter().any(|line| line.contains("vous livrez")));
        assert!(
            app.game
                .player_inventory()
                .iter()
                .all(|entry| entry.item() != &regulator)
        );
        let after_delivery = suspension::fingerprint(&app.game);
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: GridPos::new(28, 31),
            }),
            CommandOutcome::Rejected(CommandRejection::NoMaterialForDepot)
        );
        assert_eq!(suspension::fingerprint(&app.game), after_delivery);

        let saved = app.suspension().unwrap();
        let mut restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        for _ in 0..48 {
            apply(&mut restored, GameCommand::Wait);
        }
        assert_eq!(
            restored
                .game
                .active_facility()
                .unwrap()
                .repair_status(&order),
            Some(project_rl::facility::RepairStatus::Completed)
        );
    }

    #[test]
    fn version_five_suspensions_keep_the_pre_social_facility_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 5).unwrap();
        assert!(
            app.game
                .actors()
                .iter()
                .all(|(_, actor)| actor.affiliation().is_none()
                    && actor.witness_profile().is_none()
                    && actor.observed_property_takes().is_empty())
        );
        assert!(
            app.game
                .ground_items()
                .iter()
                .all(|(_, item)| item.owner().is_none())
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 5);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 5))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_six_suspensions_keep_the_pre_alert_social_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 6).unwrap();
        assert!(app.game.actors().iter().any(|(_, actor)| {
            actor.affiliation().is_some()
                && actor.witness_profile().is_some()
                && actor.local_alert_profile().is_none()
        }));
        app.walk_fixture_to(GridPos::new(16, 23)).unwrap();
        apply(&mut app, GameCommand::PickUp);
        assert!(app.game.actors().iter().any(|(_, actor)| {
            !actor.observed_property_takes().is_empty() && actor.local_alert().is_none()
        }));
        assert!(!app.log.iter().any(|line| line.contains("ALERTE LOCALE")));

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 6);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 6))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_seven_suspensions_keep_the_pre_installed_alarm_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 7).unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        assert!(
            app.game
                .active_facility()
                .and_then(|facility| facility.installation(&sensor))
                .is_some_and(|installation| installation.security_alarm_profile().is_none())
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 7);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 7))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_eight_suspensions_keep_alarms_without_lockdown_responses() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 8).unwrap();
        let sensor = "core:checkpoint_sensor".parse().unwrap();
        let profile = app
            .game
            .active_facility()
            .and_then(|facility| facility.installation(&sensor))
            .and_then(|installation| installation.security_alarm_profile())
            .unwrap();
        assert!(profile.responses().is_empty());
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 8);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 8))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn legacy_suspension_is_verified_then_upgraded_and_can_be_resuspended() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut legacy = AsciiApp::from_seed_legacy(
            INITIAL_SEED,
            rules_for_generation_version(rules, 1),
            texts,
            loot,
            expeditions,
            ascii_regional_world_catalog().unwrap(),
            1,
        )
        .unwrap();
        legacy.walk_fixture_to(GridPos::new(32, 16)).unwrap();
        apply(
            &mut legacy,
            GameCommand::Interact {
                target: GridPos::new(32, 15),
            },
        );
        let mut saved = legacy.suspension().unwrap();
        saved.version = 1;
        saved.state = suspension::fingerprint(legacy.game.active_game());
        let mut upgraded = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(
            upgraded.game.player_position(),
            legacy.game.player_position()
        );
        assert_eq!(upgraded.history, legacy.history);
        assert_eq!(upgraded.game.current_zone().unwrap().depth, 0);
        upgraded.walk_expedition_fixture(2).unwrap();
        let next = upgraded.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &next,
            upgraded.rules.clone(),
            upgraded.texts.clone(),
            upgraded.loot.clone(),
            upgraded.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), next.state);
        saved.state ^= 1;
        assert!(
            AsciiApp::restore_suspension(
                &saved,
                legacy.rules,
                legacy.texts,
                legacy.loot,
                legacy.expeditions
            )
            .is_err()
        );
    }

    #[test]
    fn expedition_generation_is_valid_and_reproducible_across_seeds() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        for seed in 0..64 {
            // Construction validates the finished map, both passage anchors,
            // actor placements and loot before the first accepted command.
            let a = AsciiApp::from_seed(
                seed,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
            )
            .unwrap();
            let b = AsciiApp::from_seed(
                seed,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
            )
            .unwrap();
            assert_eq!(
                suspension::fingerprint(&a.game),
                suspension::fingerprint(&b.game)
            );
            assert_eq!(a.game.visited_zone_count(), 1);
            assert!(
                a.game
                    .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                    .is_some()
            );
        }
    }

    #[test]
    fn version_fourteen_expedition_stays_unmaterialized_until_first_travel() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            LAZY_ZONE_GENERATION_VERSION,
        )
        .unwrap();

        assert_eq!(app.generation_version, LAZY_ZONE_GENERATION_VERSION);
        assert_eq!(app.game.visited_zone_count(), 1);
        assert!(
            app.game
                .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                .is_some_and(|link| link.arrival.is_none())
        );
        assert!(
            !app.zone_decor
                .contains_key(&"core:industrial_sector".parse().unwrap())
        );

        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert!(
            restored
                .game
                .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                .is_some_and(|link| link.arrival.is_none())
        );
    }

    #[test]
    fn version_fifteen_world_fingerprint_includes_the_regional_atlas() {
        let app = app_with_test_controls();
        const STORED_V15_WORLD_FINGERPRINT: u64 = 15_180_535_885_259_714_919;

        assert_eq!(
            world_fingerprint_for_version(
                &app.expeditions,
                &RegionalWorldCatalog::default(),
                LAZY_ZONE_GENERATION_VERSION,
            ),
            expedition_fingerprint_for_version(&app.expeditions, LAZY_ZONE_GENERATION_VERSION,)
        );
        assert_ne!(
            world_fingerprint_for_version(
                &app.expeditions,
                &app.regional_worlds,
                REGIONAL_TRAVEL_GENERATION_VERSION,
            ),
            world_fingerprint_for_version(
                &app.expeditions,
                &RegionalWorldCatalog::default(),
                REGIONAL_TRAVEL_GENERATION_VERSION,
            )
        );
        assert_ne!(
            world_fingerprint_for_version(
                &app.expeditions,
                &app.regional_worlds,
                REGIONAL_TRAVEL_GENERATION_VERSION,
            ),
            world_fingerprint_for_version(
                &app.expeditions,
                &app.regional_worlds,
                REGIONAL_POPULATION_GENERATION_VERSION,
            )
        );
        assert_eq!(
            world_fingerprint_for_version(
                &app.expeditions,
                &app.regional_worlds,
                REGIONAL_TRAVEL_GENERATION_VERSION,
            ),
            STORED_V15_WORLD_FINGERPRINT
        );
    }

    #[test]
    fn stored_version_fifteen_zero_command_run_keeps_its_exact_fingerprints() {
        // Golden values captured before regional population metadata existed.
        // They protect real v15 suspensions from a silent schema drift.
        const STORED_V15_WORLD_FINGERPRINT: u64 = 15_180_535_885_259_714_919;
        const STORED_V15_STATE_FINGERPRINT: u64 = 200_606_596_581_755_117;
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app = AsciiApp::from_seed_version(
            20_260_914,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_TRAVEL_GENERATION_VERSION,
        )
        .unwrap();
        let saved = app.suspension().unwrap();

        assert_eq!(saved.commands.len(), 0);
        assert_eq!(saved.world_rules, Some(STORED_V15_WORLD_FINGERPRINT));
        assert_eq!(saved.state, STORED_V15_STATE_FINGERPRINT);
    }

    #[test]
    fn version_fifteen_materializes_connected_but_unpopulated_atlas_regions() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_TRAVEL_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        assert_eq!(app.generation_version, REGIONAL_TRAVEL_GENERATION_VERSION);
        assert_eq!(app.game.visited_zone_count(), 1);
        assert_eq!(app.regional_zones.len(), 5);
        assert!(
            app.game
                .unmaterialized_passage_destination(west_hub_passage)
                .is_some()
        );

        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let first_zone = app.game.current_zone().unwrap().id.clone();
        assert_eq!(
            app.regional_zones.get(&first_zone),
            Some(&RegionCoord::new(-1, 0, 0))
        );
        assert_eq!(app.game.visited_zone_count(), 2);
        assert_eq!(app.game.map().width(), 96);
        assert_eq!(app.game.map().height(), 64);
        assert_eq!(app.game.actors().iter().count(), 1);
        let outward = project_rl::world::generation::cardinal_passage(
            app.regional_worlds
                .get(&"core:simulation_overworld".parse().unwrap())
                .unwrap()
                .local_map_size(),
            Direction::West,
        );
        app.walk_fixture_to(outward.step(Direction::East)).unwrap();
        apply(&mut app, GameCommand::Interact { target: outward });
        let second_zone = app.game.current_zone().unwrap().id.clone();
        assert_eq!(
            app.regional_zones.get(&second_zone),
            Some(&RegionCoord::new(-2, 0, 0))
        );
        assert_eq!(app.game.visited_zone_count(), 3);

        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.regional_zones, app.regional_zones);
        assert_eq!(restored.game.current_zone().unwrap().id, second_zone);
    }

    #[test]
    fn current_regions_materialize_guarded_encounters_loot_and_threat_sources_once() {
        let mut app = app_with_test_controls();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);

        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let passages = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .map(|direction| {
            project_rl::world::generation::cardinal_passage(
                app.regional_worlds
                    .get(&"core:simulation_overworld".parse().unwrap())
                    .unwrap()
                    .local_map_size(),
                direction,
            )
        });
        let enemies = app
            .game
            .actors()
            .iter()
            .filter(|(entity, _)| *entity != app.game.player_id())
            .map(|(_, actor)| actor)
            .collect::<Vec<_>>();
        assert!((4..=20).contains(&enemies.len()));
        assert!(enemies.iter().all(|actor| {
            let home = actor.ai_home().unwrap();
            actor.primary_attributes().is_some()
                && actor.ai().unwrap().maximum_pursuit_distance().is_some()
                && passages
                    .iter()
                    .all(|passage| home.x.abs_diff(passage.x) + home.y.abs_diff(passage.y) >= 8)
        }));
        assert!(app.game.ground_items().iter().count() >= 2);
        assert!((1..=2).contains(&app.game.threat_sources().len()));
        if app.game.active_facility().is_some_and(|facility| {
            facility.installations().any(|(_, installation)| {
                installation
                    .capabilities()
                    .contains(&InstallationCapability::SecuritySensor)
            })
        }) {
            assert!(app.game.ground_items().iter().any(|(_, stack)| {
                stack
                    .owner()
                    .is_some_and(|owner| owner.as_str() == "core:system_security")
            }));
        }
        assert!(
            app.game
                .threat_sources()
                .iter()
                .all(|source| source.is_active())
        );
        assert!(app.terminal.decor.cells.values().any(|decor| {
            matches!(
                decor,
                crate::test_sector::Decor::SupplyCache | crate::test_sector::Decor::ThreatCamp
            )
        }));
        let terminal_position = app
            .game
            .active_facility()
            .unwrap()
            .installations()
            .find_map(|(_, installation)| {
                installation
                    .capabilities()
                    .iter()
                    .any(|capability| {
                        matches!(capability, InstallationCapability::DataTerminal { .. })
                    })
                    .then_some(installation.position())
            })
            .expect("current regional content guarantees one site terminal");
        assert_eq!(
            app.terminal.decor.cells.get(&terminal_position),
            Some(&crate::test_sector::Decor::DataTerminalOnline)
        );
        assert!(!app.game.map().is_walkable(terminal_position));
        assert!(
            [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .any(|direction| app
                .game
                .map()
                .is_walkable(terminal_position.step(direction)))
        );
        let guarded_landmarks = app
            .terminal
            .decor
            .cells
            .iter()
            .filter_map(|(position, decor)| {
                matches!(
                    decor,
                    crate::test_sector::Decor::SupplyCache
                        | crate::test_sector::Decor::ThreatCamp
                        | crate::test_sector::Decor::Depot
                )
                .then_some(*position)
            })
            .take(3)
            .collect::<Vec<_>>();
        assert_eq!(guarded_landmarks.len(), 3);
        assert!(guarded_landmarks.iter().all(|landmark| {
            enemies.iter().any(|actor| {
                let home = actor.ai_home().unwrap();
                home.x.abs_diff(landmark.x) + home.y.abs_diff(landmark.y) == 1
            })
        }));
        assert!(app.terminal.decor.cells.iter().any(|(camp, decor)| {
            *decor == crate::test_sector::Decor::ThreatCamp
                && app
                    .terminal
                    .decor
                    .cells
                    .get(&GridPos::new(camp.x - 4, camp.y))
                    == Some(&crate::test_sector::Decor::RuinWall)
                && app
                    .terminal
                    .decor
                    .cells
                    .get(&GridPos::new(camp.x + 4, camp.y))
                    == Some(&crate::test_sector::Decor::RuinWall)
                && app
                    .terminal
                    .decor
                    .cells
                    .get(&GridPos::new(camp.x, camp.y - 3))
                    == Some(&crate::test_sector::Decor::RuinWall)
                && app
                    .terminal
                    .decor
                    .cells
                    .get(&GridPos::new(camp.x, camp.y + 3))
                    .is_some_and(|entrance| {
                        matches!(
                            entrance,
                            crate::test_sector::Decor::RuinFloor
                                | crate::test_sector::Decor::DoorClosed
                                | crate::test_sector::Decor::DoorLocked
                        )
                    })
                && [-3, 3].into_iter().any(|offset| {
                    matches!(
                        app.terminal
                            .decor
                            .cells
                            .get(&GridPos::new(camp.x + offset, camp.y)),
                        Some(
                            crate::test_sector::Decor::SupplyCache
                                | crate::test_sector::Decor::Depot
                        )
                    )
                })
        }));

        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(
            restored
                .game
                .actors()
                .iter()
                .filter(|(entity, _)| *entity != restored.game.player_id())
                .count(),
            enemies.len()
        );
        assert_eq!(restored.game.threat_sources(), app.game.threat_sources());
    }

    #[test]
    fn current_run_can_descend_resume_and_return_through_the_same_atlas_link() {
        let mut app = app_with_test_controls();
        assert!(
            app.navigation_signal_summary()
                .is_some_and(|signal| signal.starts_with("ROUTE VERS LES PROFONDEURS · SUD-OUEST"))
        );
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        assert_eq!(
            app.navigation_signal_summary().as_deref(),
            Some("ROUTE VERS LES PROFONDEURS · SUR PLACE")
        );
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let surface_zone = app.game.current_zone().unwrap().id.clone();
        let world = app
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let descent = project_rl::world::generation::vertical_passage(
            world.local_map_size(),
            project_rl::content::RegionVerticalDirection::Down,
        );
        assert_eq!(
            app.terminal.decor.cells.get(&descent),
            Some(&crate::test_sector::Decor::Descent)
        );
        assert!(app.game.passage(descent).is_some());
        assert!(
            app.navigation_signal_summary()
                .is_some_and(|signal| signal == "ACCÈS INFÉRIEUR · OUEST · À DISTANCE")
        );

        app.walk_fixture_to(descent).unwrap();
        assert_eq!(
            app.navigation_signal_summary().as_deref(),
            Some("ACCÈS INFÉRIEUR · SUR PLACE")
        );
        apply(&mut app, GameCommand::Interact { target: descent });
        let lower_zone = app.game.current_zone().unwrap().id.clone();
        assert_eq!(
            app.regional_zones.get(&lower_zone),
            Some(&RegionCoord::new(-1, 0, 1))
        );
        assert_ne!(lower_zone, surface_zone);
        assert!(app.game.actors().iter().count() > 1);
        assert!(
            app.game
                .actors()
                .iter()
                .any(|(_, actor)| actor.destruction_effect().is_some())
        );
        assert!(app.game.ground_items().iter().count() >= 3);
        let ascent = app.game.player_position().unwrap();
        assert_eq!(
            app.terminal.decor.cells.get(&ascent),
            Some(&crate::test_sector::Decor::Ascent)
        );
        assert_eq!(
            app.navigation_signal_summary().as_deref(),
            Some("RETOUR SURFACE · SUR PLACE")
        );

        let saved = app.suspension().unwrap();
        let mut restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(restored.game.current_zone().unwrap().id, lower_zone);
        apply(&mut restored, GameCommand::Interact { target: ascent });
        assert_eq!(restored.game.current_zone().unwrap().id, surface_zone);
        assert_eq!(
            restored.regional_zones.get(&surface_zone),
            Some(&RegionCoord::new(-1, 0, 0))
        );
    }

    #[test]
    fn version_thirty_keeps_automatic_hits_while_current_runs_enable_hit_rules() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        assert_eq!(rules.hit_rules, Some(HitRules::default()));
        let app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_DESTRUCTIBLES_GENERATION_VERSION,
        )
        .unwrap();

        assert_eq!(app.rules.hit_rules, None);
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_DESTRUCTIBLES_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_thirty_eight_keeps_weapons_without_reaction_capabilities() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let blade: WeaponId = "core:integrity_blade".parse().unwrap();
        assert!(
            rules
                .weapons
                .get(&blade)
                .is_some_and(|weapon| weapon.capabilities().can_melee_parry())
        );

        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            EFFECT_TRIGGER_GENERATION_VERSION,
        )
        .unwrap();
        assert!(
            legacy
                .rules
                .weapons
                .get(&blade)
                .is_some_and(|weapon| !weapon.capabilities().can_melee_parry())
        );

        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, EFFECT_TRIGGER_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_thirty_one_keeps_neutral_enemies_while_current_runs_use_profiles() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            HIT_CHANCE_GENERATION_VERSION,
        )
        .unwrap();
        assert!(
            legacy
                .game
                .actors()
                .iter()
                .filter(|(id, actor)| {
                    *id != legacy.game.player_id() && actor.attack(0).is_some()
                })
                .all(|(_, actor)| actor.primary_attributes().is_none())
        );
        legacy
            .walk_fixture_to(TestSector::EXPANDED_EXPEDITION_PASSAGE.step(Direction::West))
            .unwrap();
        apply(
            &mut legacy,
            GameCommand::Interact {
                target: TestSector::EXPANDED_EXPEDITION_PASSAGE,
            },
        );
        assert!(
            legacy
                .game
                .actors()
                .iter()
                .filter(|(id, actor)| {
                    *id != legacy.game.player_id() && actor.attack(0).is_some()
                })
                .all(|(_, actor)| actor.primary_attributes().is_none())
        );
        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, HIT_CHANCE_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(world_fingerprint_for_version(
                &legacy.expeditions,
                &legacy.regional_worlds,
                HIT_CHANCE_GENERATION_VERSION,
            ))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);

        let current = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert!(
            current
                .game
                .actors()
                .iter()
                .filter(|(id, actor)| {
                    *id != current.game.player_id()
                        && actor.drone().is_none()
                        && actor.attack(0).is_some()
                })
                .all(|(_, actor)| actor.primary_attributes().is_some())
        );
    }

    #[test]
    fn version_thirty_four_keeps_legacy_damage_while_current_runs_use_armor() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            CHARACTER_CLASSES_GENERATION_VERSION,
        )
        .unwrap();

        assert_eq!(legacy.rules.armor_rules, None);
        assert_eq!(legacy.rules.damage, DamageRules::default());
        assert_eq!(
            legacy.rules.player_body_profile.map(|body| body.base_armor),
            Some(0)
        );
        assert_eq!(
            legacy
                .game
                .actors()
                .get(legacy.game.player_id())
                .map(|actor| actor.armor_profile().after_fragilization()),
            Some(0)
        );

        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, CHARACTER_CLASSES_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);

        let current = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert_eq!(current.rules.armor_rules, Some(ArmorRules::default()));
        assert_eq!(current.rules.damage, DamageRules::specialized());
        assert_eq!(
            current
                .game
                .actors()
                .get(current.game.player_id())
                .map(|actor| actor.armor_profile().after_fragilization()),
            Some(1)
        );
    }

    #[test]
    fn version_thirty_five_keeps_pre_equipment_items_and_loot() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            ARMOR_GENERATION_VERSION,
        )
        .unwrap();
        let patched: ItemId = "core:patched_plating".parse().unwrap();
        let composite: ItemId = "core:composite_carapace".parse().unwrap();

        assert!(legacy.rules.player_armor_slots.is_empty());
        assert!(legacy.rules.items.get(&patched).is_none());
        assert!(legacy.rules.items.get(&composite).is_none());
        assert!(legacy.loot.iter().all(|(_, table)| {
            table
                .entries()
                .iter()
                .all(|entry| entry.item != patched && entry.item != composite)
        }));
        assert_eq!(
            legacy.game.actor_armor_profile(legacy.game.player_id()),
            Some(project_rl::combat::ArmorProfile::new(1, 0, 0, 0))
        );

        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, ARMOR_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);

        let current = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert_eq!(current.rules.player_armor_slots.len(), 1);
        assert_eq!(
            current.rules.items.get(&patched).map(|item| item.kind()),
            Some(ItemKind::Armor)
        );
        assert_eq!(
            current
                .rules
                .items
                .get(&composite)
                .and_then(|item| item.equipment())
                .map(|profile| profile.armor()),
            Some(2)
        );
        assert!(current.loot.iter().any(|(_, table)| {
            table
                .entries()
                .iter()
                .any(|entry| entry.item == patched || entry.item == composite)
        }));
    }

    #[test]
    fn version_thirty_two_keeps_legacy_physical_values_while_current_runs_use_profiles() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            ENEMY_ATTRIBUTES_GENERATION_VERSION,
        )
        .unwrap();

        assert_eq!(legacy.rules.physical_rules, None);
        assert_eq!(legacy.rules.player_body_profile, None);
        assert_eq!(
            legacy
                .game
                .actors()
                .get(legacy.game.player_id())
                .map(Actor::maximum_integrity),
            Some(20)
        );
        assert_eq!(
            legacy
                .game
                .equipped_player_weapon(0)
                .and_then(|weapon| weapon.attack().melee_impact()),
            None
        );

        legacy
            .walk_fixture_to(TestSector::EXPANDED_EXPEDITION_PASSAGE.step(Direction::West))
            .unwrap();
        apply(
            &mut legacy,
            GameCommand::Interact {
                target: TestSector::EXPANDED_EXPEDITION_PASSAGE,
            },
        );
        let legacy_enemies = legacy
            .game
            .actors()
            .iter()
            .filter(|(id, _)| *id != legacy.game.player_id())
            .map(|(_, actor)| actor)
            .collect::<Vec<_>>();
        assert!(legacy_enemies.iter().all(|actor| {
            actor.maximum_integrity() == 9
                && actor.body_profile().is_none()
                && actor
                    .attack(0)
                    .is_some_and(|attack| attack.melee_impact().is_none())
        }));

        let saved = legacy.suspension().unwrap();
        assert_eq!(saved.version, ENEMY_ATTRIBUTES_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            legacy.rules.clone(),
            legacy.texts.clone(),
            legacy.loot.clone(),
            legacy.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);

        let current = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert_eq!(current.rules.physical_rules, Some(PhysicalRules::default()));
        assert!(current.rules.player_body_profile.is_some());
        assert_eq!(
            current
                .game
                .equipped_player_weapon(0)
                .and_then(|weapon| weapon.attack().melee_impact())
                .map(|impact| impact.material_cap),
            Some(14)
        );
    }

    #[test]
    fn version_twenty_nine_resumes_its_deep_region_without_destructibles() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let world = app
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let descent = project_rl::world::generation::vertical_passage(
            world.local_map_size(),
            RegionVerticalDirection::Down,
        );
        app.walk_fixture_to(descent).unwrap();
        apply(&mut app, GameCommand::Interact { target: descent });

        assert!(
            app.game
                .actors()
                .iter()
                .all(|(_, actor)| actor.destruction_effect().is_none())
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_VERTICAL_TRAVEL_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_destructible_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_eight_keeps_its_world_fingerprint_and_has_no_descent() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_SITE_TERMINALS_GENERATION_VERSION,
        )
        .unwrap();
        assert!(app.navigation_signal_summary().is_none());
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let world = app
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let descent = project_rl::world::generation::vertical_passage(
            world.local_map_size(),
            project_rl::content::RegionVerticalDirection::Down,
        );
        assert!(app.game.passage(descent).is_none());
        assert!(app.vertical_navigation_signal_summary().is_none());
        assert_ne!(
            app.terminal.decor.cells.get(&descent),
            Some(&crate::test_sector::Decor::Descent)
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_SITE_TERMINALS_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_regions_keep_their_original_density_and_resume() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            THREAT_RENEWAL_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );
        let enemies = app
            .game
            .actors()
            .iter()
            .filter(|(entity, _)| *entity != app.game.player_id())
            .count();
        assert!((2..=8).contains(&enemies));

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, THREAT_RENEWAL_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_one_regions_resume_without_site_metadata() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_ENCOUNTER_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_ENCOUNTER_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_data_terminal_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_two_regions_resume_with_legacy_open_site_entrances() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_SITE_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_SITE_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_data_terminal_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_interaction_metadata()
                    .without_site_terminal_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_three_regions_resume_without_generated_site_security() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_SITE_INTERACTION_GENERATION_VERSION,
        )
        .unwrap();
        let west_hub_passage = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find_map(|(direction, passage)| (direction == Direction::West).then_some(passage))
            .unwrap();
        app.walk_fixture_to(west_hub_passage.step(Direction::East))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: west_hub_passage,
            },
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_SITE_INTERACTION_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_data_terminal_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_security_metadata()
                    .without_site_terminal_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_four_regions_resume_with_their_historical_state() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            REGIONAL_SITE_SECURITY_GENERATION_VERSION,
        )
        .unwrap();
        let (_, passage) = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find(|(direction, _)| *direction == Direction::North)
            .unwrap();
        app.walk_fixture_to(passage.step(Direction::South)).unwrap();
        apply(&mut app, GameCommand::Interact { target: passage });

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, REGIONAL_SITE_SECURITY_GENERATION_VERSION);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_five_regions_resume_without_navigation_beacons() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            INVESTIGATING_REINFORCEMENTS_GENERATION_VERSION,
        )
        .unwrap();
        let (_, passage) = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find(|(direction, _)| *direction == Direction::East)
            .unwrap();
        app.walk_fixture_to(passage.step(Direction::West)).unwrap();
        apply(&mut app, GameCommand::Interact { target: passage });

        let facility = app.game.active_facility().unwrap();
        assert!(facility.installations().all(|(_, installation)| {
            installation.capabilities().iter().all(|capability| {
                !matches!(capability, InstallationCapability::NavigationBeacon { .. })
            })
        }));
        let saved = app.suspension().unwrap();
        assert_eq!(
            saved.version,
            INVESTIGATING_REINFORCEMENTS_GENERATION_VERSION
        );
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_data_terminal_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_navigation_signal_metadata()
                    .without_site_terminal_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_six_resumes_without_data_terminals() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            SITE_NAVIGATION_SIGNALS_GENERATION_VERSION,
        )
        .unwrap();

        assert!(
            app.game
                .active_facility()
                .unwrap()
                .installations()
                .all(
                    |(_, installation)| installation.capabilities().iter().all(|capability| {
                        !matches!(capability, InstallationCapability::DataTerminal { .. })
                    })
                )
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, SITE_NAVIGATION_SIGNALS_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_data_terminal_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_terminal_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twenty_seven_regions_resume_without_procedural_site_terminals() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            DATA_TERMINALS_GENERATION_VERSION,
        )
        .unwrap();
        let (_, passage) = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find(|(direction, _)| *direction == Direction::East)
            .unwrap();
        app.walk_fixture_to(passage.step(Direction::West)).unwrap();
        apply(&mut app, GameCommand::Interact { target: passage });

        if let Some(facility) = app.game.active_facility() {
            assert!(facility.installations().all(|(_, installation)| {
                installation.capabilities().iter().all(|capability| {
                    !matches!(capability, InstallationCapability::DataTerminal { .. })
                })
            }));
        }
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, DATA_TERMINALS_GENERATION_VERSION);
        assert_eq!(
            saved.world_rules,
            Some(suspension::fingerprint(&(
                app.expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata(),
                app.regional_worlds
                    .without_player_relation_metadata()
                    .without_ranged_skill_body_metadata()
                    .without_melee_skill_body_metadata()
                    .without_physical_metadata()
                    .without_primary_attribute_metadata()
                    .without_electronic_system_metadata()
                    .without_vertical_link_metadata()
                    .without_deeper_layer_gameplay_metadata()
                    .without_site_terminal_metadata(),
            )))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn current_east_region_exposes_an_approximate_site_navigation_signal() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        let (_, passage) = TestSector::EXPANDED_REGIONAL_PASSAGES
            .into_iter()
            .find(|(direction, _)| *direction == Direction::East)
            .unwrap();
        app.walk_fixture_to(passage.step(Direction::West)).unwrap();
        apply(&mut app, GameCommand::Interact { target: passage });

        let player = app.game.player_position().unwrap();
        let signals = app
            .game
            .active_facility()
            .unwrap()
            .detected_navigation_signals(player);
        assert!(!signals.is_empty());
        let summary = crate::terminal_view::navigation_signal_summary(
            player,
            signals[0].position,
            signals[0].distance,
            signals.len(),
        );
        assert!(summary.starts_with("SIGNAL DE SITE · EST"));
        assert!(!summary.contains('('));
    }

    #[test]
    fn current_expedition_spawns_its_data_defined_population() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let expedition_id = "core:starter_expedition".parse().unwrap();
        let expected_count: usize = expeditions
            .get(&expedition_id)
            .unwrap()
            .destination
            .population
            .iter()
            .map(|group| usize::from(group.count()))
            .sum();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.walk_fixture_to(TestSector::EXPANDED_EXPEDITION_PASSAGE.step(Direction::West))
            .unwrap();
        apply(
            &mut app,
            GameCommand::Interact {
                target: TestSector::EXPANDED_EXPEDITION_PASSAGE,
            },
        );
        assert_eq!(app.game.map().width(), 128);
        assert_eq!(app.game.map().height(), 88);

        let enemies = app
            .game
            .actors()
            .iter()
            .filter(|(id, _)| *id != app.game.player_id())
            .map(|(_, actor)| actor)
            .collect::<Vec<_>>();
        assert_eq!(enemies.len(), expected_count);
        assert_eq!(
            enemies
                .iter()
                .map(|actor| actor.ai().unwrap().behavior)
                .collect::<Vec<_>>(),
            vec![
                project_rl::ai::AiBehavior::Hunter,
                project_rl::ai::AiBehavior::Sentry,
                project_rl::ai::AiBehavior::Hunter,
            ]
        );
        assert_eq!(
            enemies
                .iter()
                .map(|actor| actor.ai().unwrap().maximum_pursuit_distance())
                .collect::<Vec<_>>(),
            vec![Some(14), None, Some(14)]
        );
        assert!(enemies.iter().all(|actor| {
            actor
                .ai()
                .unwrap()
                .maximum_pursuit_distance()
                .is_none_or(|_| actor.ai_home().is_some())
        }));
        assert_eq!(
            enemies
                .iter()
                .map(|actor| actor.maximum_integrity())
                .collect::<Vec<_>>(),
            vec![9, 14, 9]
        );
        assert!(enemies.iter().all(|actor| {
            actor.body_profile().is_some()
                && actor.player_relation() == PlayerRelation::Hostile
                && actor
                    .attack(0)
                    .is_some_and(|attack| attack.damage().raw_total() == 3)
                && actor
                    .defeat_reward()
                    .is_some_and(|reward| reward.base_experience == 8)
        }));
        assert_eq!(
            enemies
                .iter()
                .filter(|actor| {
                    actor
                        .attack(0)
                        .and_then(AttackProfile::preparation_disruption)
                        .is_some()
                })
                .count(),
            1
        );
        assert_eq!(
            enemies
                .iter()
                .map(|actor| actor.primary_attributes())
                .collect::<Vec<_>>(),
            vec![
                Some(PrimaryAttributes::new(6, 6, 5, 6, 4)),
                Some(PrimaryAttributes::new(5, 5, 6, 8, 6)),
                Some(PrimaryAttributes::new(6, 6, 5, 6, 4)),
            ]
        );
        let hit_rules = app.rules.hit_rules.unwrap();
        assert_eq!(
            enemies
                .iter()
                .map(|actor| hit_rules.hit_chance(
                    project_rl::combat::AttackDelivery::Melee,
                    Some(PrimaryAttributes::prototype_default()),
                    0,
                    actor.primary_attributes(),
                    0,
                    0,
                ))
                .collect::<Vec<_>>(),
            vec![70, 74, 70]
        );
    }

    #[test]
    fn version_fifty_five_generation_strips_preparation_disruption() {
        let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
        let expedition_id = "core:starter_expedition".parse().unwrap();
        let definition = expeditions.get(&expedition_id).unwrap();
        let generated = crate::test_expedition::generate_destination(
            &rules,
            INITIAL_SEED,
            Some(&loot),
            definition,
            crate::test_expedition::ExpeditionGenerationFeatures {
                defined_population: true,
                expanded_world: true,
                pursuit_limits: true,
                pursuit_lifecycle: true,
                primary_attributes: true,
                physical_profiles: true,
                electronic_systems: true,
                preparation_disruption: false,
                player_relations: false,
            },
        )
        .unwrap();

        assert!(generated.blueprint.actors.iter().all(|actor| {
            actor
                .attack(0)
                .and_then(AttackProfile::preparation_disruption)
                .is_none()
        }));
        assert_eq!(
            expedition_fingerprint_for_version(
                &expeditions,
                WAIT_CONTINUES_PREPARATION_GENERATION_VERSION,
            ),
            suspension::fingerprint(
                &expeditions
                    .without_player_relation_metadata()
                    .without_preparation_disruption_metadata(),
            )
        );
    }

    #[test]
    fn older_drone_generations_keep_their_historical_doctrine_and_lifetime() {
        let (rules, _, _, _) = ascii_game_content().unwrap();

        let version_fifty_six =
            rules_for_generation_version(rules.clone(), PREPARATION_DISRUPTION_GENERATION_VERSION);
        let version_fifty_seven =
            rules_for_generation_version(rules.clone(), DRONE_DEFAULT_SUPPORT_GENERATION_VERSION);
        let version_fifty_eight =
            rules_for_generation_version(rules.clone(), DRONE_ENERGY_LIFETIME_GENERATION_VERSION);
        let version_fifty_nine =
            rules_for_generation_version(rules.clone(), DRONE_LINK_AWARENESS_GENERATION_VERSION);
        let current = rules_for_generation_version(rules, CURRENT_GENERATION_VERSION);

        assert!(!version_fifty_six.player_drone_default_support);
        assert!(!version_fifty_six.player_drone_expires_without_energy);
        assert!(!version_fifty_six.player_companion_behaviors);
        assert!(!version_fifty_six.player_drone_link_awareness);
        assert!(version_fifty_seven.player_drone_default_support);
        assert!(!version_fifty_seven.player_drone_expires_without_energy);
        assert!(!version_fifty_seven.player_companion_behaviors);
        assert!(!version_fifty_seven.player_drone_link_awareness);
        assert!(version_fifty_eight.player_drone_default_support);
        assert!(version_fifty_eight.player_drone_expires_without_energy);
        assert!(version_fifty_eight.player_companion_behaviors);
        assert!(!version_fifty_eight.player_drone_link_awareness);
        assert!(!version_fifty_eight.player_relation_targeting);
        assert!(version_fifty_nine.player_drone_link_awareness);
        assert!(!version_fifty_nine.player_relation_targeting);
        assert!(current.player_drone_default_support);
        assert!(current.player_drone_expires_without_energy);
        assert!(current.player_companion_behaviors);
        assert!(current.player_drone_link_awareness);
        assert!(current.player_relation_targeting);
    }

    #[test]
    fn version_fifty_nine_omits_authored_player_relations_from_world_fingerprints() {
        let (_, _, _, expeditions) = ascii_game_content().unwrap();
        assert_eq!(
            expedition_fingerprint_for_version(
                &expeditions,
                DRONE_LINK_AWARENESS_GENERATION_VERSION,
            ),
            suspension::fingerprint(&expeditions.without_player_relation_metadata())
        );
        assert_ne!(
            expedition_fingerprint_for_version(&expeditions, CURRENT_GENERATION_VERSION),
            expedition_fingerprint_for_version(
                &expeditions,
                DRONE_LINK_AWARENESS_GENERATION_VERSION,
            )
        );
    }

    #[test]
    fn version_ten_suspensions_keep_the_legacy_population_and_flamethrower() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 10).unwrap();
        assert_eq!(
            app.game
                .equipped_player_weapon(2)
                .map(|weapon| weapon.id().as_str()),
            Some("core:flamethrower")
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 10);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 10))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_eleven_suspensions_keep_the_smaller_world_layout() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 11).unwrap();
        assert_eq!(app.game.map().width(), 110);
        assert_eq!(app.game.map().height(), 68);
        assert!(
            app.game
                .passage(TestSector::LEGACY_EXPEDITION_PASSAGE)
                .is_some()
        );
        assert!(
            app.game
                .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                .is_none()
        );
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 11);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 11))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_twelve_suspensions_keep_unlimited_historical_pursuit() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 12).unwrap();
        assert_eq!(app.game.map().width(), 192);
        assert_eq!(app.game.map().height(), 128);
        assert!(
            app.game
                .actors()
                .iter()
                .filter(|(id, _)| *id != app.game.player_id())
                .all(|(_, actor)| {
                    actor
                        .ai()
                        .is_none_or(|ai| ai.maximum_pursuit_distance().is_none())
                        && actor.ai_home().is_none()
                })
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 12);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 12))
        );
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_thirteen_suspensions_keep_the_eager_destination() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 13).unwrap();
        assert!(
            app.game
                .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                .is_some_and(|link| link.arrival.is_some())
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 13);
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert!(
            restored
                .game
                .passage(TestSector::EXPANDED_EXPEDITION_PASSAGE)
                .is_some_and(|link| link.arrival.is_some())
        );
    }

    fn blade_loot_catalog() -> LootCatalog {
        use project_rl::loot::{LootEntry, LootTable};
        let mut catalog = LootCatalog::default();
        catalog
            .register(
                LootTable::new(
                    "core:industrial_floor".parse().unwrap(),
                    vec![LootEntry {
                        item: "core:integrity_blade".parse().unwrap(),
                        weight: 1,
                        minimum_depth: 0,
                        maximum_depth: None,
                        map_kinds: vec![],
                        sources: vec![],
                        minimum_quantity: 1,
                        maximum_quantity: 1,
                    }],
                )
                .unwrap(),
            )
            .unwrap();
        catalog
    }

    #[test]
    fn weighted_loot_does_not_change_the_terrain_or_enemy_random_stream() {
        let (rules, texts, _, expeditions) = ascii_game_content().unwrap();
        let mut weighted = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            blade_loot_catalog(),
            expeditions.clone(),
            3,
        )
        .unwrap();
        let mut legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            blade_loot_catalog(),
            expeditions,
            2,
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(weighted.game.active_game()),
            suspension::fingerprint(legacy.game.active_game())
        );
        for app in [&mut weighted, &mut legacy] {
            app.walk_fixture_to(GridPos::new(66, 21)).unwrap();
            apply(
                app,
                GameCommand::Interact {
                    target: GridPos::new(67, 21),
                },
            );
        }
        assert_eq!(
            suspension::fingerprint(weighted.game.map()),
            suspension::fingerprint(legacy.game.map())
        );
        assert_eq!(
            suspension::fingerprint(weighted.game.actors()),
            suspension::fingerprint(legacy.game.actors())
        );
        assert_eq!(weighted.game.rng_state(), legacy.game.rng_state());
        assert!(
            weighted
                .game
                .ground_items()
                .iter()
                .all(|(_, stack)| stack.item().as_str() == "core:integrity_blade")
        );
        assert!(
            legacy
                .game
                .ground_items()
                .iter()
                .any(|(_, stack)| stack.item().as_str() == "core:repair_patch")
        );
    }

    #[test]
    fn version_two_suspensions_keep_fixed_loot_and_do_not_depend_on_new_tables() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut old =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 2).unwrap();
        old.walk_expedition_fixture(2).unwrap();
        let saved = old.suspension().unwrap();
        assert_eq!(saved.version, 2);
        assert_eq!(saved.loot_rules, None);
        let restored = AsciiApp::restore_suspension(
            &saved,
            old.rules.clone(),
            old.texts.clone(),
            LootCatalog::default(),
            old.expeditions.clone(),
        )
        .unwrap();
        let next = restored.suspension().unwrap();
        assert_eq!(next.version, 2);
        assert_eq!(next.loot_rules, None);
        assert_eq!(next.state, saved.state);
        assert_eq!(next.commands, saved.commands);
    }

    #[test]
    fn changed_loot_tables_preserve_a_version_three_suspension_and_active_run() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 3).unwrap();
        app.suspension_path = temporary_folder("loot-v3").join("suspended-run.json");
        app.walk_expedition_fixture(2).unwrap();
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 3);
        assert_eq!(saved.loot_rules, Some(suspension::fingerprint(&app.loot)));
        saved.write(&app.suspension_path).unwrap();
        let before = std::fs::read(&app.suspension_path).unwrap();
        let state = suspension::fingerprint(&app.game);
        app.loot = blade_loot_catalog();
        let error = app.resume_run().unwrap_err();
        assert!(error.contains("tables de butin"), "{error}");
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), before);
        assert_eq!(suspension::fingerprint(&app.game), state);
        let mut malformed = saved.clone();
        malformed.loot_rules = None;
        assert!(malformed.validate().is_err());
        malformed = saved;
        malformed.version = 2;
        assert!(malformed.validate().is_err());
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn changed_world_definitions_preserve_a_version_four_suspension_and_active_run() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 4).unwrap();
        app.suspension_path = temporary_folder("world-v4").join("suspended-run.json");
        app.walk_expedition_fixture(2).unwrap();
        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, 4);
        assert_eq!(
            saved.world_rules,
            Some(expedition_fingerprint_for_version(&app.expeditions, 4))
        );
        saved.write(&app.suspension_path).unwrap();
        let before = std::fs::read(&app.suspension_path).unwrap();
        let state = suspension::fingerprint(&app.game);

        let mut changed = ExpeditionCatalog::default();
        for (id, definition) in app.expeditions.iter() {
            if id.as_str() == "core:starter_expedition" {
                let mut destination = definition.destination.clone();
                destination.seed_salt ^= 1;
                changed
                    .register(
                        project_rl::content::ExpeditionDefinition::new(
                            id.clone(),
                            definition.hub.clone(),
                            destination,
                            definition.hub_passage,
                        )
                        .unwrap(),
                    )
                    .unwrap();
            } else {
                changed.register(definition.clone()).unwrap();
            }
        }
        app.expeditions = changed;

        let error = app.resume_run().unwrap_err();
        assert!(error.contains("définitions du monde"), "{error}");
        assert_eq!(std::fs::read(&app.suspension_path).unwrap(), before);
        assert_eq!(suspension::fingerprint(&app.game), state);

        let mut malformed = saved.clone();
        malformed.world_rules = None;
        assert!(malformed.validate().is_err());
        malformed = saved;
        malformed.version = 3;
        assert!(malformed.validate().is_err());
        std::fs::remove_file(&app.suspension_path).unwrap();
        std::fs::remove_dir(app.suspension_path.parent().unwrap()).unwrap();
    }

    #[test]
    fn report_has_exactly_one_toggle_and_never_advances_simulation() {
        let mut app = app_with_test_controls();
        let before = (
            app.game.turn(),
            app.game.rng_state(),
            app.game.player_energy(),
        );
        app.update_input(&input("O"));
        assert!(!app.report_open); // No report has been produced yet.
        app.observation_report.push("Relevé daté".to_owned());
        app.update_input(&input("O"));
        assert!(app.report_open);
        for key in ["Enter", "KpEnter", "C", "L", "B", "P", "Semicolon", "Space"] {
            app.update_input(&input(key));
            assert!(app.report_open);
        }
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert!(!app.report_open);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Pause);
        assert!(!app.report_open);
        app.update_input(&input("Escape"));
        assert_eq!(app.menu, MenuScreen::Hidden);
        app.update_input(&input("O"));
        assert!(app.report_open);
        app.update_input(&input("O"));
        assert!(!app.report_open);
        app.controls
            .rebind(Action::Report, Binding::key("F3"))
            .unwrap();
        app.update_input(&input("O"));
        assert!(!app.report_open);
        app.update_input(&input("F3"));
        assert!(app.report_open);
        app.update_input(&input("O"));
        assert!(app.report_open);
        app.update_input(&input("F3"));
        assert!(!app.report_open);
        assert_eq!(
            (
                app.game.turn(),
                app.game.rng_state(),
                app.game.player_energy()
            ),
            before
        );
    }

    #[test]
    fn suspension_replays_an_in_progress_multi_ut_preparation() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let prepared: TechniqueId = "core:rec_02".parse().unwrap();
        let mut skills = project_rl::skills::SkillCatalog::default();
        for (_, discipline) in rules.skills.disciplines() {
            skills.register_discipline(discipline.clone()).unwrap();
        }
        for (id, definition) in rules.skills.techniques() {
            let definition = if id == &prepared {
                definition.clone().with_preparation_steps(2).unwrap()
            } else {
                definition.clone()
            };
            skills.register_technique(definition).unwrap();
        }
        rules.skills = skills;
        rules.progression.starting_skill_points = 1;
        let mut app = AsciiApp::from_seed(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        apply(
            &mut app,
            GameCommand::LearnTechnique {
                technique: prepared.clone(),
            },
        );
        apply(
            &mut app,
            GameCommand::UseTechnique {
                technique: prepared.clone(),
                targets: Vec::new(),
                weapon_slot: None,
            },
        );
        assert_eq!(
            app.game
                .player_technique_preparation()
                .map(|state| (state.technique().clone(), state.remaining_steps().get())),
            Some((prepared, 2))
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, CURRENT_GENERATION_VERSION);
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.history, app.history);
        assert_eq!(
            restored
                .game
                .player_technique_preparation()
                .map(|state| state.remaining_steps().get()),
            Some(2)
        );
    }

    #[test]
    fn suspension_replays_active_r1_and_version_forty_strips_recovery_metadata() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        rules.player_base_attacks[2] = rules.player_base_attacks[2]
            .with_recovery_after_attack(project_rl::time::TimeUnits::ONE);
        let flamethrower: WeaponId = "core:flamethrower".parse().unwrap();
        let mut recovering_weapons = project_rl::weapon::WeaponCatalog::default();
        for (id, definition) in rules.weapons.iter() {
            let attack = if id == &flamethrower {
                definition
                    .attack()
                    .with_recovery_after_attack(project_rl::time::TimeUnits::ONE)
            } else {
                definition.attack()
            };
            recovering_weapons
                .register(
                    project_rl::weapon::WeaponDefinition::new(
                        id.clone(),
                        definition.name_key().to_owned(),
                        definition.description_key().to_owned(),
                        attack,
                    )
                    .unwrap()
                    .with_effects(definition.effects().iter().cloned())
                    .with_capabilities(definition.capabilities()),
                )
                .unwrap();
        }
        rules.weapons = recovering_weapons;

        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            MULTI_UT_PREPARATION_GENERATION_VERSION,
        )
        .unwrap();
        assert_eq!(
            legacy.rules.player_base_attacks[2].recovery_after_attack(),
            None
        );
        assert_eq!(
            legacy
                .rules
                .weapons
                .get(&flamethrower)
                .and_then(|weapon| weapon.attack().recovery_after_attack()),
            None
        );

        let mut app = AsciiApp::from_seed(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        app.walk_expedition_fixture(0).unwrap();
        let mut empty_target = None;
        'rows: for y in 0..app.game.map().height() as i32 {
            for x in 0..app.game.map().width() as i32 {
                let candidate = GridPos::new(x, y);
                let Ok(preview) = app.game.player_attack_preview(2, candidate) else {
                    continue;
                };
                if preview
                    .cells()
                    .iter()
                    .all(|cell| app.game.actors().entity_at(cell.position).is_none())
                {
                    empty_target = Some(candidate);
                    break 'rows;
                }
            }
        }
        let empty_target = empty_target.expect("fixture needs one valid empty cone");
        apply(
            &mut app,
            GameCommand::AttackAt {
                slot: 2,
                target: empty_target,
            },
        );
        assert_eq!(
            app.game
                .actors()
                .get(app.game.player_id())
                .and_then(Actor::recovery_remaining),
            Some(project_rl::time::TimeUnits::ONE)
        );
        app.active_weapon_slot = 2;
        let turn_before_blocked_preview = app.game.turn();
        app.update_input(&input("F"));
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn_before_blocked_preview);
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("Vous récupérez encore"))
        );

        let saved = app.suspension().unwrap();
        assert_eq!(saved.version, CURRENT_GENERATION_VERSION);
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(restored.history, app.history);
        assert_eq!(
            restored
                .game
                .actors()
                .get(restored.game.player_id())
                .and_then(Actor::recovery_remaining),
            Some(project_rl::time::TimeUnits::ONE)
        );
    }

    #[test]
    fn version_forty_one_keeps_its_pre_melee_rules_while_current_runs_load_the_full_discipline() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let melee: DisciplineId = "core:combat_rapproche".parse().unwrap();
        let hindrance: StatusId = "core:locomotion_hindered".parse().unwrap();

        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            ACTION_RECOVERY_GENERATION_VERSION,
        )
        .unwrap();
        assert!(legacy.rules.skills.discipline(&melee).is_none());
        assert!(!legacy.rules.statuses.contains(&hindrance));
        assert!(legacy.rules.stability_rules.is_none());
        let legacy_body = legacy.rules.player_body_profile.unwrap();
        assert!(legacy_body.displacement_profile().is_none());
        assert!(legacy_body.locomotion_profile().is_none());

        let current = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert_eq!(current.generation_version, CURRENT_GENERATION_VERSION);
        assert!(current.rules.skills.discipline(&melee).is_some());
        assert_eq!(
            current
                .rules
                .skills
                .techniques()
                .filter(|(_, technique)| technique.discipline() == &melee)
                .count(),
            10
        );
        assert!(current.rules.statuses.contains(&hindrance));
        assert!(current.rules.stability_rules.is_some());
        let current_body = current.rules.player_body_profile.unwrap();
        assert!(current_body.displacement_profile().is_some());
        assert!(current_body.locomotion_profile().is_some());
    }

    #[test]
    fn azerty_movement_uses_one_physical_position_and_no_qwerty_aliases() {
        let mut app = app_with_test_controls();
        app.game = WorldState::single(
            GameState::new(
                Map::from_ascii("#####\n#...#\n#...#\n#...#\n#####").unwrap(),
                GridPos::new(2, 2),
                7,
            )
            .unwrap(),
        );
        assert_eq!(
            read_movement_command(&app.game, 0, &app.controls, &input("W")),
            Some(GameCommand::Move(Direction::North))
        );
        assert_eq!(
            read_movement_command(&app.game, 0, &app.controls, &input("A")),
            Some(GameCommand::Move(Direction::West))
        );
        for key in ["Z", "Q", "Up", "Left", "Period"] {
            assert_eq!(
                read_movement_command(&app.game, 0, &app.controls, &input(key)),
                None
            );
        }
        assert!(app.controls.pressed(Action::Multiple, &input("Semicolon")));
        assert!(!app.controls.pressed(Action::Multiple, &input("M")));
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Key1")), Some(0));
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Kp1")), None);
        app.controls
            .rebind(Action::Slot1, Binding::key("Kp1"))
            .unwrap();
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Key1")), None);
        assert_eq!(pressed_weapon_slot(&app.controls, &input("Kp1")), Some(0));
    }

    #[test]
    fn a_new_run_equips_and_selects_the_flamethrower_on_channel_three() {
        let mut app = app_with_test_controls();

        assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);
        assert_eq!(
            app.game
                .equipped_player_weapon(2)
                .map(|weapon| weapon.id().as_str()),
            Some("core:flamethrower")
        );
        let channels = app.combat_channels_label();
        assert!(channels.contains("[3] Lance-flammes industriel"));
        assert!(channels.starts_with(">[1]"));

        app.update_input(&input("Key3"));
        assert_eq!(app.active_weapon_slot, 2);
        assert!(
            app.combat_channels_label()
                .contains(">[3] Lance-flammes industriel")
        );
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("Lance-flammes industriel"))
        );
    }

    #[test]
    fn area_aiming_moves_a_preview_without_time_and_commits_an_empty_tile() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        app.active_weapon_slot = 2;
        app.facing = Direction::East;
        let turn_before_aiming = app.game.turn();
        let history_before_aiming = app.history.len();

        app.update_input(&input("F"));

        let initial = app.attack_aim.expect("area weapon should enter aim mode");
        assert_eq!(initial.slot, 2);
        assert!(
            app.game
                .player_attack_preview(initial.slot, initial.cursor)
                .is_ok()
        );
        assert_eq!(app.game.turn(), turn_before_aiming);
        assert_eq!(app.history.len(), history_before_aiming);

        app.update_input(&input("A"));
        let moved = app.attack_aim.expect("movement must keep aim mode open");
        assert_eq!(moved.cursor, initial.cursor.step(Direction::West));
        assert_eq!(app.game.actors().entity_at(moved.cursor), None);
        assert_eq!(app.game.turn(), turn_before_aiming);
        assert_eq!(app.history.len(), history_before_aiming);

        let mut empty_cursor = None;
        'rows: for y in 0..app.game.map().height() as i32 {
            for x in 0..app.game.map().width() as i32 {
                let candidate = GridPos::new(x, y);
                if !app.game.player_visibility().is_visible(candidate) {
                    continue;
                }
                let Ok(preview) = app.game.player_attack_preview(2, candidate) else {
                    continue;
                };
                if preview
                    .cells()
                    .iter()
                    .all(|cell| app.game.actors().entity_at(cell.position).is_none())
                {
                    empty_cursor = Some(candidate);
                    break 'rows;
                }
            }
        }
        let empty_cursor = empty_cursor.expect("fixture needs a valid cone without any actor");
        app.attack_aim = Some(AttackAim {
            slot: 2,
            cursor: empty_cursor,
        });

        // A real key must be released before Macroquad emits its next pressed
        // edge. Keep this empty frame in the regression test so confirmation
        // exercises the same sequence as live input.
        app.update_input(&InputFrame::default());
        app.update_input(&input("F"));

        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn_before_aiming + 1);
        assert!(app.visual_cues.active_count() > 0);
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("ATTAQUE CONFIRMÉE") && line.contains("Lance-flammes")),
            "successful empty attack log missing from {:?}",
            app.log
        );
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::AttackAt { slot: 2, x, y })
                if *x == empty_cursor.x && *y == empty_cursor.y
        ));
        assert_eq!(
            app.history.last().unwrap().command(&app.game),
            Ok(GameCommand::AttackAt {
                slot: 2,
                target: empty_cursor,
            })
        );
    }

    #[test]
    fn learned_sweep_opens_the_shared_area_preview_and_confirms_an_empty_center() {
        let mut app = app_with_test_controls();
        let mut rules = app.rules.clone();
        rules.progression.starting_skill_points = 4;
        // This test isolates the shared area-preview interaction. Requirement
        // enforcement has dedicated engine tests.
        rules.skill_progression = rules.skill_progression.without_authored_requirements();
        app.game = WorldState::single(
            GameState::new_with_rules(
                Map::from_ascii("#######\n#.....#\n#.....#\n#.....#\n#######").unwrap(),
                GridPos::new(3, 2),
                7,
                rules,
            )
            .unwrap(),
        );
        for technique in ["core:mel_01", "core:mel_03", "core:mel_05"] {
            assert_eq!(
                app.game
                    .process_player_command(GameCommand::LearnTechnique {
                        technique: technique_id(technique).unwrap(),
                    }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        app.game.drain_events();
        app.history.clear();
        app.active_weapon_slot = 0;
        app.facing = Direction::East;
        let sweep = technique_id("core:mel_05").unwrap();
        let turn_before = app.game.turn();

        assert_eq!(app.technique_command(sweep.clone()), None);
        let aim = app.attack_aim.expect("Balayage should open area aiming");
        assert_eq!(app.attack_aim_technique.as_ref(), Some(&sweep));
        assert_eq!(aim.cursor, GridPos::new(4, 2));
        assert!(app.game.actors().entity_at(aim.cursor).is_none());
        assert!(app.aimed_attack_preview(aim).is_ok());
        assert_eq!(app.game.turn(), turn_before);

        app.update_input(&InputFrame::default());
        app.update_input(&input("F"));

        assert!(app.attack_aim.is_none());
        assert!(app.attack_aim_technique.is_none());
        assert_eq!(app.game.turn(), turn_before + 1);
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::TechniqueAt {
                technique,
                x: 4,
                y: 2,
                weapon_slot: 0,
            }) if technique == "core:mel_05"
        ));
    }

    #[test]
    fn hidden_radial_damage_death_and_propagation_do_not_leak_into_the_client() {
        let mut app = app_with_test_controls();
        let mut rules = app.rules.clone();
        rules.player_field_of_view.radius = 2;
        rules.player_base_abilities = vec![AbilityProfile::new(
            12,
            DistanceMetric::Chebyshev,
            false,
            true,
            vec![EffectPrimitive::RadialDamage(
                project_rl::effects::RadialDamageEffect {
                    maximum_cost: 1,
                    neighbor_mode: project_rl::world::NeighborMode::CardinalAndDiagonal,
                    propagation_policy:
                        project_rl::world::TerrainPropagationPolicy::blocked_by_walls(1),
                    damage: project_rl::combat::DamagePacket::new(20, DamageType::Explosive, 0),
                    falloff: project_rl::effects::DamageFalloff::None,
                },
            )],
        )];
        app.game = WorldState::single(
            GameState::new_with_rules(
                Map::from_ascii("#############\n#...........#\n#############").unwrap(),
                GridPos::new(1, 1),
                7,
                rules,
            )
            .unwrap(),
        );
        let hidden_target = app
            .game
            .spawn_actor(Actor::new(GridPos::new(9, 1), 5).unwrap())
            .unwrap();
        assert!(!app.game.player_visibility().is_visible(GridPos::new(9, 1)));
        app.game.drain_events();
        app.log.clear();
        app.visual_cues.clear_world();

        assert_eq!(
            app.game.process_player_command(GameCommand::UseAbility {
                slot: 0,
                target: GridPos::new(9, 1),
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));

        assert!(app.game.actors().get(hidden_target).is_none());
        assert!(app.log.is_empty(), "hidden event leaked into {:?}", app.log);
        assert_eq!(app.visual_cues.active_count(), 0);
    }

    #[test]
    fn pointer_attack_starts_area_aim_on_the_clicked_empty_cell_without_time() {
        let mut app = app_with_test_controls();
        app.walk_fixture_to(GridPos::new(65, 21)).unwrap();
        app.active_weapon_slot = 2;
        let turn_before = app.game.turn();
        let history_before = app.history.len();
        let clicked = GridPos::new(70, 20);
        assert!(app.game.player_visibility().is_visible(clicked));
        assert_eq!(app.game.actors().entity_at(clicked), None);

        assert!(app.begin_pointer_attack_aim(clicked, Some((400.0, 300.0))));

        assert_eq!(
            app.attack_aim,
            Some(AttackAim {
                slot: 2,
                cursor: clicked,
            })
        );
        assert_eq!(app.game.turn(), turn_before);
        assert_eq!(app.history.len(), history_before);
    }

    #[test]
    fn escape_cancels_area_aiming_before_opening_the_pause_menu() {
        let mut app = app_with_test_controls();
        app.active_weapon_slot = 2;
        let before = suspension::fingerprint(&app.game);

        app.update_input(&input("F"));
        assert!(app.attack_aim.is_some());
        app.update_input(&input("Escape"));

        assert!(app.attack_aim.is_none());
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.history.is_empty());
    }

    #[test]
    fn options_change_layout_and_rebind_without_touching_the_run() {
        let mut app = app_with_test_controls();
        // Use native backend semantics for the persisted fixture.
        app.controls = Controls::preset(Layout::Azerty, KeySemantics::native());
        let folder = std::env::temp_dir().join(format!(
            "project-rl-options-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        app.controls_path = folder.join("controls.json");
        let before = (
            app.game.turn(),
            app.game.rng_state(),
            app.game.player_energy(),
            app.game.export_player_progression().unwrap(),
        );
        let options = InputFrame {
            pause: true,
            ..Default::default()
        };
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Pause);
        app.update_input(&input("Down"));
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&input("Enter"));
        assert_eq!(app.menu, MenuScreen::Controls);
        app.update_input(&input("Enter"));
        assert_eq!(app.controls.layout, Layout::Qwerty);
        app.options_selection = Action::ALL
            .iter()
            .position(|action| *action == Action::Report)
            .unwrap()
            + 1;
        app.update_input(&input("Enter"));
        assert!(app.rebinding);
        app.update_input(&InputFrame {
            pressed: [Binding::MouseRight].into(),
            ..Default::default()
        });
        assert!(!app.rebinding);
        assert_eq!(app.controls.binding(Action::Report), &Binding::MouseRight);
        assert_eq!(
            Controls::load(&app.controls_path, Some(Layout::Azerty)).0,
            app.controls
        );
        app.update_input(&input("Space"));
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&options);
        app.update_input(&options);
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(
            (
                app.game.turn(),
                app.game.rng_state(),
                app.game.player_energy(),
                app.game.export_player_progression().unwrap()
            ),
            before
        );
        let saved = app.controls.clone();
        app.update_input(&input("R"));
        assert_eq!(app.controls, saved);
        std::fs::remove_file(&app.controls_path).unwrap();
        std::fs::remove_dir(&folder).unwrap();
    }

    #[test]
    fn ascii_content_targets_and_dated_reports_work_without_a_graphics_context() {
        let mut app = app_with_test_controls();
        assert_eq!(app.game.player_progression().unspent_skill_points(), 2);
        assert_eq!(app.game.player_energy().available(), 100);
        let multiple = technique_id("core:rec_09").unwrap();
        assert_eq!(app.technique_name(&multiple), "Analyse multiple");
        for (_, definition) in app.game.rules().skills.techniques() {
            assert!(
                app.texts
                    .resolve(DISPLAY_LOCALE, definition.description_key())
                    .is_some()
            );
        }

        let mut rules = app.rules.clone();
        rules.progression.starting_skill_points = 9;
        rules.player_field_of_view.radius = 4;
        // This fixture exercises target selection and dated reports rather than
        // the independently tested level/attribute eligibility policy.
        rules.skill_progression = rules.skill_progression.without_authored_requirements();
        app.game = WorldState::single(
            GameState::new_with_rules(
                Map::from_ascii("#########\n#.......#\n#.......#\n#########").unwrap(),
                GridPos::new(1, 1),
                7,
                rules,
            )
            .unwrap(),
        );
        for id in [
            "core:rec_01",
            "core:rec_02",
            "core:rec_09",
            "core:rec_04",
            "core:rec_05",
        ] {
            assert_eq!(
                app.game
                    .process_player_command(GameCommand::LearnTechnique {
                        technique: technique_id(id).unwrap(),
                    }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        let targets: Vec<_> = [2, 3, 4, 7]
            .into_iter()
            .map(|x| {
                app.game
                    .spawn_actor(Actor::new(GridPos::new(x, 1), 12).unwrap())
                    .unwrap()
            })
            .collect();
        app.game.drain_events();
        app.selected_target = Some(targets[2]);
        let command = app.technique_command(multiple.clone()).unwrap();
        assert_eq!(
            command,
            GameCommand::UseTechnique {
                technique: multiple.clone(),
                targets: vec![targets[2], targets[0], targets[1]],
                weapon_slot: None,
            }
        );
        assert_eq!(
            app.game.process_player_command(command),
            CommandOutcome::Applied
        );
        app.capture_events();
        assert!(!app.report_open);
        assert_eq!(app.observation_report.len(), 5);
        assert!(app.observation_report[0].contains("Analyse multiple — relevé · cycle 0"));
        assert!(app.observation_report[1].contains("-2 E ; réserve 98 E"));
        assert!(app.observation_report[2].contains("Cible observée à l'est, à proximité"));
        assert!(
            !app.observation_report
                .iter()
                .any(|line| line.contains("(4, 1)") || line.contains("(7, 1)"))
        );
        let snapshot = app.observation_report.clone();
        app.game.process_player_command(GameCommand::Wait);
        app.capture_events();
        assert_eq!(app.observation_report, snapshot);

        // A stale selection cannot add a hidden target to a new command.
        app.selected_target = Some(targets[3]);
        assert_eq!(
            app.technique_command(multiple.clone()),
            Some(GameCommand::UseTechnique {
                technique: multiple,
                targets: targets[..3].to_vec(),
                weapon_slot: None,
            })
        );
        let command = app.target_analysis_command().unwrap();
        assert_eq!(
            app.game.process_player_command(command),
            CommandOutcome::Applied
        );
        app.capture_events();
        assert_eq!(app.observation_report.len(), 2);
        assert!(app.observation_report[0].contains("Analyse de cible — relevé · cycle 2"));
    }
}
