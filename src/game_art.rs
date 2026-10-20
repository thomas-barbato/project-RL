//! Approved front sprites, native ground icons and source-derived illustrations.
//! Only the renderer uses these indices; save data and content IDs stay unchanged.
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use project_rl::world::GridPos;
#[path = "game_art_finish.rs"]
mod finish;
#[cfg(debug_assertions)]
#[path = "art_finish_completion_review.rs"]
pub(crate) mod finish_completion;
#[cfg(debug_assertions)]
#[path = "art_finish_review.rs"]
pub(crate) mod finish_review;
#[cfg(debug_assertions)]
#[path = "art_finish_review_02.rs"]
pub(crate) mod finish_review_02;
#[cfg(debug_assertions)]
#[path = "art_finish_review_03.rs"]
pub(crate) mod finish_review_03;
#[cfg(debug_assertions)]
#[path = "art_finish_review_04.rs"]
pub(crate) mod finish_review_04;
#[cfg(debug_assertions)]
#[path = "art_finish_review_05.rs"]
pub(crate) mod finish_review_05;
#[path = "game_art_palette.rs"]
mod palettes;

use crate::terminal_view::PlayerWeaponPose;
use crate::test_sector::Decor;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ItemArt {
    pub icon: u8,
    pub illustration: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WeaponHold {
    Side,
    Raised,
    TwoHanded,
    MeleeTwoHanded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Visual {
    AgentForm(u8),
    Player {
        body: u8,
        palettes: [EquipmentPalette; 3],
        weapon: Option<u16>,
        equipment: [Option<u16>; 4],
        hands: Option<[Option<u16>; 2]>,
    },
    Actor(u8),
    CampNpc(u8),
    DroneScout,
    DroneTracker,
    DroneGunner,
    DroneGuardian,
    AgentHologram(spectral::Source),
    ArmedActor {
        body: u8,
        weapon: u16,
    },
    Bestiary {
        number: u8,
        corpse: bool,
    },
    CampaignMonster {
        number: u16,
        variant: u8,
    },
    ColoredItem(ItemArt, EquipmentPalette),
    Terrain {
        kind: Decor,
        position: GridPos,
    },
}

include!("game_art_catalog.rs");
pub(super) fn item(id: &str) -> Option<ItemArt> {
    let illustration = match id {
        "core:tenue_assaut" => 200,
        "core:tenue_technique" => 201,
        "core:tenue_traque" => 202,
        "core:tenue_rituelle" => 203,
        "core:tenue_metamorphe" => 204,
        _ => return legacy_item(id),
    };
    Some(ItemArt {
        icon: 2,
        illustration,
    })
}
#[path = "game_art_bestiary.rs"]
mod bestiary;
#[path = "game_art_campaign.rs"]
pub(crate) mod campaign;
#[path = "game_art_drone.rs"]
mod drones;
#[path = "game_art_player.rs"]
mod player_layers;
#[path = "game_art_portrait.rs"]
mod portrait;
#[path = "game_art_spectral.rs"]
mod spectral;

fn texture(bytes: &[u8], filter: FilterMode) -> Texture2D {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(filter);
    texture
}

struct ArtTextures {
    actors: Texture2D,
    portraits: portrait::Sheet,
}

thread_local! {
    static TEXTURES: ArtTextures = ArtTextures {
        actors: texture(include_bytes!("../assets/sprites/cyberpunk/actors-32.png"), FilterMode::Nearest),
        portraits: portrait::Sheet::load(include_bytes!("../assets/sprites/cyberpunk/actors-32.png"), 32),
    };
}

pub(crate) fn prewarm() {
    TEXTURES.with(|_| {});
    palettes::prewarm();
    player_layers::prewarm();
    bestiary::prewarm();
    campaign::prewarm();
}

#[cfg(debug_assertions)]
pub(crate) fn cache_stats() -> [(usize, usize); 2] {
    [player_layers::cache_stats(), palettes::cache_stats()]
}
#[cfg(debug_assertions)]
pub(crate) fn draw_grip_review(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
) {
    player_layers::draw_grip_review(rect, body, weapon, hands);
}
#[cfg(debug_assertions)]
pub(crate) fn draw_armor_breath_review(rect: Rect, body: u16, weapon: Option<u16>, time: f64) {
    player_layers::draw_grip_review_at_time(rect, body, weapon, None, Some(time));
}

pub(crate) fn terrain_visual(kind: Decor, x: i32, y: i32) -> Option<Visual> {
    Some(Visual::Terrain {
        kind,
        position: GridPos::new(x, y),
    })
}

pub(crate) fn actor_for_symbol(symbol: char) -> Option<Visual> {
    let index = match symbol {
        'N' => 0,
        'B' => 1,
        'K' => 2,
        'V' => 3,
        'G' => 4,
        'H' => 5,
        'W' => 6,
        'J' => 7,
        'Q' => 8,
        'P' => 9,
        'I' => 10,
        'C' => 11,
        'U' => 12,
        'Z' => 13,
        'O' => 14,
        'F' => 15,
        'D' => 16,
        'd' => 17,
        'r' | 'j' => 18,
        't' => 19,
        'w' | 'h' => 20,
        'c' | 'v' | 'i' => 21,
        'm' | 'a' => 22,
        // Drones, devices and laboratory targets keep their own drawings.
        _ => return None,
    };
    Some(Visual::Actor(3 + index))
}

pub(crate) fn cultist_two_handed(actor: &project_rl::entity::Actor) -> Option<bool> {
    let model = held_weapon(actor.equipped_weapon()?.item().as_str())?;
    Some(matches!(
        weapon_hold(model),
        WeaponHold::TwoHanded | WeaponHold::MeleeTwoHanded
    ))
}

pub(crate) fn arm_actor(visual: Visual, weapon: Option<&str>) -> Visual {
    match (visual, weapon.and_then(held_weapon)) {
        (Visual::Actor(body @ 20..=25), Some(weapon)) => Visual::ArmedActor { body, weapon },
        _ => visual,
    }
}

/// Stable invocation identities select only their already reviewed drawings.
pub(crate) fn agent_drone_visual(profile: &str) -> Option<Visual> {
    Some(match profile {
        "agent:d56" => Visual::DroneScout,
        "agent:d57" => Visual::Bestiary {
            number: 57,
            corpse: false,
        },
        "agent:d58" => Visual::DroneTracker,
        "agent:d59" => Visual::DroneGunner,
        "agent:d66" => Visual::DroneGuardian,
        _ => return None,
    })
}

pub(crate) fn hologram(source: Visual) -> Option<Visual> {
    spectral::hologram(source)
}

pub(crate) fn player(class: Option<&str>, pose: PlayerWeaponPose, weapon: Option<&str>) -> Visual {
    let body = match class {
        Some("core:bastion") => 3,
        Some("core:technomancien") => 4,
        Some("core:traqueur") => 5,
        Some("core:arcaniste") => 6,
        Some("core:metamorphe") => 7,
        Some("core:vigie") => 1,
        Some("core:creuset") => 2,
        _ => 0,
    };
    let fallback = match pose {
        PlayerWeaponPose::Unarmed => None,
        PlayerWeaponPose::Melee => Some("core:melee_epee_longue"),
        PlayerWeaponPose::Ranged => Some("core:fusil_d_assaut"),
    };
    let weapon = fallback.and_then(|fallback| {
        weapon
            .and_then(held_weapon)
            .or_else(|| held_weapon(fallback))
    });
    Visual::Player {
        body,
        palettes: [EquipmentPalette::Original; 3],
        weapon,
        equipment: [None; 4],
        hands: None,
    }
}

pub(crate) fn wear(
    mut visual: Visual,
    pieces: impl IntoIterator<Item = (String, String)>,
) -> Visual {
    if let Visual::Player { equipment, .. } = &mut visual {
        for (slot, id) in pieces {
            let index = match slot.as_str() {
                "core:body_armor" => 0,
                "core:head_armor" => 1,
                "core:hand_armor" => 2,
                "core:foot_armor" => 3,
                _ => continue,
            };
            equipment[index] = item(&id)
                .filter(|art| art.icon == 2)
                .map(|art| art.illustration);
        }
    }
    visual
}

fn draw_cell(texture: &Texture2D, rect: Rect, index: u16, side: u16, opacity: f32) {
    draw_texture_ex(
        texture,
        rect.x,
        rect.y,
        Color::new(1.0, 1.0, 1.0, opacity),
        DrawTextureParams {
            source: Some(Rect::new(
                f32::from(index % 16 * side),
                f32::from(index / 16 * side),
                f32::from(side),
                f32::from(side),
            )),
            dest_size: Some(rect.size()),
            ..Default::default()
        },
    );
}

fn draw_actor_shadow(rect: Rect, index: u8, opacity: f32) {
    draw_contact_shadow(rect, CONTACTS[usize::from(index)], 32, opacity);
}

fn draw_contact_shadow(rect: Rect, contact: [u8; 3], side: u16, opacity: f32) {
    if crate::target_outline::capturing_body_mask() {
        return;
    }
    let [x, y, width] = contact;
    let scale = rect.w.min(rect.h) / f32::from(side);
    for (dy, w) in [
        (-1.0, f32::from(width) - 4.0),
        (0.0, f32::from(width)),
        (1.0, f32::from(width) - 4.0),
    ] {
        draw_rectangle(
            rect.x + (f32::from(x) - (w * 0.5).floor()) * scale,
            rect.y + (f32::from(y) + dy) * scale,
            w * scale,
            scale,
            Color::new(10.0 / 255.0, 15.0 / 255.0, 17.0 / 255.0, 0.24 * opacity),
        );
    }
    let width = f32::from(width) - 6.0;
    draw_rectangle(
        rect.x + (f32::from(x) - (width * 0.5).floor()) * scale,
        rect.y + f32::from(y) * scale,
        width * scale,
        scale,
        Color::new(8.0 / 255.0, 12.0 / 255.0, 14.0 / 255.0, 0.38 * opacity),
    );
}

fn draw_actor(art: &ArtTextures, rect: Rect, index: u8, opacity: f32) {
    draw_actor_shadow(rect, index, opacity);
    draw_cell(&art.actors, rect, u16::from(index), 32, opacity);
}

pub(crate) fn draw_world(rect: Rect, visual: Visual, opacity: f32) {
    draw_world_animated(rect, visual, opacity, None);
}

pub(crate) fn draw_world_animated(
    rect: Rect,
    visual: Visual,
    opacity: f32,
    idle_time: Option<f64>,
) {
    #[cfg(debug_assertions)]
    let visual = finish_completion::normalized_visual(visual);
    #[cfg(debug_assertions)]
    let visual = finish_review::normalized_visual(visual);
    #[cfg(debug_assertions)]
    let visual = finish_review_02::normalized_visual(visual);
    #[cfg(debug_assertions)]
    let visual = finish_review_03::normalized_visual(visual);
    #[cfg(debug_assertions)]
    let visual = finish_review_04::normalized_visual(visual);
    #[cfg(debug_assertions)]
    let visual = finish_review_05::normalized_visual(visual);
    #[cfg(debug_assertions)]
    if finish_review_05::draw_replacement(rect, visual, opacity) {
        return;
    }
    #[cfg(debug_assertions)]
    if finish_review_04::draw_replacement(rect, visual, opacity) {
        return;
    }
    #[cfg(debug_assertions)]
    if finish_review_03::draw_replacement(rect, visual, opacity) {
        return;
    }
    #[cfg(debug_assertions)]
    if finish_review_02::draw_replacement(rect, visual, opacity) {
        return;
    }
    #[cfg(debug_assertions)]
    if finish_review::draw_replacement(rect, visual, opacity) {
        return;
    }
    TEXTURES.with(|art| match visual {
        Visual::AgentForm(index) => draw_agent_form(rect, index, opacity),
        Visual::DroneScout => drones::draw_scout(rect, opacity),
        Visual::DroneTracker => drones::draw_tracker(rect, opacity),
        Visual::DroneGunner => drones::draw_gunner(rect, opacity),
        Visual::DroneGuardian => drones::draw_guardian(rect, opacity),
        Visual::AgentHologram(source) => spectral::draw(rect, source, opacity, idle_time, 32),
        Visual::Player {
            body,
            palettes,
            weapon,
            equipment,
            hands,
        } => {
            draw_actor_shadow(rect, body, opacity);
            player_layers::draw_player(
                rect, body, equipment, weapon, hands, palettes, 32, opacity, idle_time,
            );
        }
        Visual::Actor(index) if (20..=25).contains(&index) && idle_time.is_some() => {
            draw_actor_shadow(rect, index, opacity);
            player_layers::draw_npc(rect, index, None, 32, opacity, idle_time);
        }
        Visual::Actor(index) => draw_actor(art, rect, index, opacity),
        Visual::CampNpc(index) => {
            crate::cryogenic_camp_render::draw_npc(rect, index, opacity, true, idle_time)
        }
        Visual::ArmedActor { body, weapon } => {
            draw_actor_shadow(rect, body, opacity);
            player_layers::draw_npc(rect, body, Some(weapon), 32, opacity, idle_time);
        }
        Visual::Bestiary { number, corpse } => bestiary::draw(rect, number, corpse, opacity, true),
        Visual::CampaignMonster { number, variant } => {
            campaign::draw(rect, number, variant, opacity, true)
        }
        Visual::ColoredItem(item, palette) => palettes::draw_item(rect, item, palette, opacity),
        Visual::Terrain { kind, position } => {
            crate::terminal_view::draw_scenery_preview(rect, kind, position)
        }
    });
}

pub(crate) fn draw_portrait(rect: Rect, visual: Visual) {
    TEXTURES.with(|art| match visual {
        Visual::AgentForm(index) => draw_agent_form_portrait(rect, index, 1.0),
        Visual::DroneScout => drones::draw_portrait(rect, Visual::DroneScout),
        Visual::DroneTracker => drones::draw_portrait(rect, Visual::DroneTracker),
        Visual::DroneGunner => drones::draw_portrait(rect, Visual::DroneGunner),
        Visual::DroneGuardian => drones::draw_portrait(rect, Visual::DroneGuardian),
        Visual::AgentHologram(source) => spectral::draw(rect, source, 1., None, 128),
        Visual::Player {
            body,
            palettes,
            weapon,
            equipment,
            hands,
        } => {
            player_layers::draw_player(
                rect, body, equipment, weapon, hands, palettes, 128, 1.0, None,
            );
        }
        Visual::ColoredItem(item, palette) => palettes::draw_item(rect, item, palette, 1.0),
        Visual::Actor(index) => art.portraits.draw(rect, u16::from(index), 1.0),
        Visual::CampNpc(index) => {
            crate::cryogenic_camp_render::draw_npc(rect, index, 1., false, None)
        }
        Visual::ArmedActor { body, weapon } => {
            player_layers::draw_npc(rect, body, Some(weapon), 128, 1.0, None)
        }
        Visual::Bestiary { number, corpse } => bestiary::draw(rect, number, corpse, 1.0, false),
        Visual::CampaignMonster { number, variant } => {
            campaign::draw(rect, number, variant, 1., false)
        }
        Visual::Terrain { kind, position } => {
            crate::terminal_view::draw_scenery_preview(rect, kind, position)
        }
    });
}

// Diagnostic reference only: no pistol is added to content, loot or saves.
#[cfg(debug_assertions)]
pub(crate) fn draw_pistol_reference(rect: Rect, visual: Visual, side: u16) {
    if let Visual::Player {
        body, equipment, ..
    } = visual
    {
        player_layers::draw_pistol_reference(rect, body, equipment, side);
    }
}

pub(crate) fn draw_item_variant(rect: Rect, id: &str, palette: EquipmentPalette) -> bool {
    draw_item_variant_opacity(rect, id, palette, 1.)
}

pub(crate) fn draw_item_variant_opacity(
    rect: Rect,
    id: &str,
    palette: EquipmentPalette,
    opacity: f32,
) -> bool {
    let Some(art) = item(id) else {
        return false;
    };
    palettes::draw_item(rect, art, palette, opacity);
    true
}

#[cfg(debug_assertions)]
pub(crate) fn draw_item(rect: Rect, id: &str) -> bool {
    let Some(art) = item(id) else {
        return false;
    };
    draw_portrait(rect, Visual::ColoredItem(art, EquipmentPalette::Original));
    true
}

pub(crate) fn draw_corpse(rect: Rect, sprite: u8, opacity: f32) {
    if let Some(number) = bestiary::legacy_corpse(sprite) {
        draw_world(
            rect,
            Visual::Bestiary {
                number,
                corpse: true,
            },
            opacity,
        );
        return;
    }
    TEXTURES.with(|art| {
        let size = vec2(rect.w * 0.42, rect.h * 0.84);
        let center = vec2(rect.x + rect.w * 0.5, rect.y + rect.h * 0.69);
        draw_texture_ex(
            &art.actors,
            center.x - size.x * 0.5,
            center.y - size.y * 0.5,
            Color::new(0.63, 0.56, 0.54, opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    f32::from(sprite % 16) * 32.0,
                    f32::from(sprite / 16) * 32.0,
                    32.0,
                    32.0,
                )),
                dest_size: Some(size),
                rotation: std::f32::consts::FRAC_PI_2,
                pivot: Some(center),
                ..Default::default()
            },
        );
    });
}

pub(crate) fn has_authored_corpse(sprite: u8) -> bool {
    bestiary::legacy_corpse(sprite).is_some()
}

pub(crate) fn actor_bleeds(sprite: u8) -> bool {
    bestiary::legacy_corpse(sprite)
        .and_then(bestiary::entry)
        .is_none_or(|entry| entry.bleeds)
}

pub(crate) fn visual_bleeds(visual: Visual) -> bool {
    match visual {
        Visual::Actor(sprite) | Visual::ArmedActor { body: sprite, .. } => actor_bleeds(sprite),
        Visual::Bestiary { number, .. } => {
            bestiary::entry(number).is_some_and(|entry| entry.bleeds)
        }
        Visual::CampaignMonster { number, .. } => !matches!(number, 124..=127),
        Visual::Player { .. } | Visual::AgentForm(_) => true,
        _ => false,
    }
}

pub(crate) fn draw_terminal_wireframe(rect: Rect, visual: Visual, color: Color) {
    if let Visual::Player { equipment, .. } = visual {
        player_layers::draw_wireframe(rect, equipment, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_drones_and_holograms_use_their_reviewed_visual_families() {
        assert_eq!(agent_drone_visual("agent:d56"), Some(Visual::DroneScout));
        assert_eq!(
            agent_drone_visual("agent:d57"),
            Some(Visual::Bestiary {
                number: 57,
                corpse: false
            })
        );
        assert_eq!(agent_drone_visual("agent:d58"), Some(Visual::DroneTracker));
        assert!(agent_drone_visual("legacy:unknown").is_none());
        let original = player(Some("core:arcaniste"), PlayerWeaponPose::Unarmed, None);
        let clone = hologram(original).unwrap();
        assert!(matches!(clone, Visual::AgentHologram(_)));
        assert!(!visual_bleeds(clone));
        assert!(matches!(
            hologram(Visual::AgentForm(0)),
            Some(Visual::AgentHologram(_))
        ));
        assert!(hologram(Visual::Actor(20)).is_none());
    }

    #[test]
    fn armour_slots_share_a_ground_icon_but_keep_distinct_illustrations() {
        let ids = [
            "core:plastron_bastion",
            "core:heaume_bastion",
            "core:gantelets_bastion",
            "core:solerets_bastion",
        ];
        let models: Vec<_> = ids.into_iter().map(|id| item(id).unwrap()).collect();
        assert!(models.iter().all(|art| art.icon == models[0].icon));
        let indices: std::collections::BTreeSet<_> =
            models.iter().map(|art| art.illustration).collect();
        assert_eq!(indices.len(), 4);
        let atlas = palettes::decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-items-32.png"
        ));
        let drawings: std::collections::BTreeSet<_> = models
            .iter()
            .map(|art| palettes::cell(&atlas, art.illustration, 32).bytes)
            .collect();
        assert_eq!(
            drawings.len(),
            4,
            "helmet, gloves and boots need distinct art"
        );
        assert!(item("mod:unknown").is_none());
    }

    #[test]
    fn weapon_changes_only_the_held_layer_and_class_selects_the_body() {
        for class in ["core:breche", "core:vigie", "core:creuset"] {
            let Visual::Player {
                body, weapon: None, ..
            } = player(Some(class), PlayerWeaponPose::Unarmed, None)
            else {
                panic!("unarmed class body missing");
            };
            let mut layers = std::collections::BTreeSet::new();
            for (pose, id) in [
                (PlayerWeaponPose::Melee, "core:melee_epee_longue"),
                (PlayerWeaponPose::Ranged, "core:modele_brenek_a80"),
                (PlayerWeaponPose::Ranged, "core:flamethrower"),
                (PlayerWeaponPose::Ranged, "core:needle_launcher"),
            ] {
                let Visual::Player {
                    body: armed_body,
                    weapon: Some(layer),
                    ..
                } = player(Some(class), pose, Some(id))
                else {
                    panic!("held weapon missing: {id}");
                };
                assert_eq!(
                    armed_body, body,
                    "weapon changed class outfit: {class}/{id}"
                );
                assert_eq!(Some(layer), held_weapon(id));
                layers.insert(layer);
            }
            assert_eq!(layers.len(), 4);
        }
        assert_ne!(
            player(Some("core:breche"), PlayerWeaponPose::Ranged, None),
            player(Some("core:vigie"), PlayerWeaponPose::Ranged, None)
        );
    }

    #[test]
    fn front_species_mapping_does_not_add_a_zombie() {
        for symbol in "NBKVG HWJQPICUZOFDrjtdwchvima"
            .chars()
            .filter(|c| *c != ' ')
        {
            assert!(
                matches!(actor_for_symbol(symbol), Some(Visual::Actor(3..=25))),
                "{symbol}"
            );
        }
        assert!(actor_for_symbol('X').is_none());
    }

    #[test]
    fn npc_weapons_follow_actual_equipment_only_on_compatible_human_bodies() {
        let rifle = "core:fusil_d_assaut";
        assert!(matches!(
            arm_actor(Visual::Actor(21), Some(rifle)),
            Visual::ArmedActor { body: 21, .. }
        ));
        assert_eq!(arm_actor(Visual::Actor(21), None), Visual::Actor(21));
        assert_eq!(arm_actor(Visual::Actor(5), Some(rifle)), Visual::Actor(5));
    }
}

thread_local! { static AGENT_FORMS:[portrait::Sheet;6]=[
    include_bytes!("../assets/animations/agent/skill-animation-review-39/predator-64.png").as_slice(),
    include_bytes!("../assets/animations/agent/skill-animation-review-39/colossus-64.png").as_slice(),
    include_bytes!("../assets/animations/agent/skill-animation-review-41/cuirasse-64.png").as_slice(),
    include_bytes!("../assets/animations/agent/skill-animation-review-41/possede-64.png").as_slice(),
    include_bytes!("../assets/animations/agent/skill-animation-review-44/abysse-64.png").as_slice(),
    include_bytes!("../assets/animations/agent/skill-animation-review-42/closed-64.png").as_slice(),
].map(|data| portrait::Sheet::load(data, 64));}
fn draw_agent_form(rect: Rect, index: u8, opacity: f32) {
    AGENT_FORMS.with(|textures| {
        if let Some(t) = textures.get(usize::from(index)) {
            draw_texture_ex(
                &t.texture,
                rect.x,
                rect.y,
                Color::new(1., 1., 1., opacity),
                DrawTextureParams {
                    dest_size: Some(vec2(rect.w, rect.h)),
                    ..Default::default()
                },
            );
        }
    });
}

fn draw_agent_form_portrait(rect: Rect, index: u8, opacity: f32) {
    AGENT_FORMS.with(|textures| {
        if let Some(source) = textures.get(usize::from(index)) {
            source.draw(rect, 0, opacity);
        }
    });
}
