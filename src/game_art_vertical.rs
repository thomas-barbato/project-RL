//! Accepted VERT18 figures, C11 dual loadouts and C12 two-handed attachments.
//! Normal drawing only reads prepared textures; it never reads or decodes files.
use super::{Visual, walking::Pose};
use crate::asset_preparation::{self as preparation, PreparedImage, Request};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use serde::Deserialize;
use std::{collections::BTreeMap, sync::OnceLock};
#[path = "game_art_vertical_geometry.rs"]
mod geometry;
#[cfg(debug_assertions)]
thread_local! { static DRAWS:std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }
#[cfg(debug_assertions)]
pub fn draw_count() -> usize {
    DRAWS.get()
}

#[derive(Deserialize)]
struct Manifest {
    native_tile_px: u16,
    profiles: Vec<Profile>,
    bindings: Vec<Binding>,
    gear: Gear,
    monsters: Vec<Static>,
    npc: Static,
}
#[derive(Deserialize)]
struct Profile {
    id: String,
    outfit: u16,
    families: BTreeMap<String, Family>,
    cases: Vec<Case>,
    #[serde(default = "default_player_anchor")]
    anchor: [f32; 2],
    #[serde(default)]
    fixed_loadouts: Vec<FixedLoadout>,
}
fn default_player_anchor() -> [f32; 2] {
    [128., 160.]
}
fn unarmed_family(profile: &Profile) -> &'static str {
    if profile.families.contains_key("one") {
        "one"
    } else {
        "ranged"
    }
}
#[derive(Deserialize)]
struct FixedLoadout {
    main: u16,
    left: u16,
    helmet: Option<u16>,
    image: Static,
}
#[derive(Deserialize)]
struct Family {
    body_file: String,
    hands_file: String,
    cell: [u16; 2],
    origin: [i32; 2],
    frames: Vec<Frame>,
}
#[derive(Deserialize)]
struct Frame {
    head_anchor: [f32; 2],
}
#[derive(Deserialize)]
struct Case {
    model: u16,
    left: bool,
    family: String,
    two_handed: bool,
    frames: Vec<Transform>,
}
#[derive(Deserialize)]
struct Transform {
    pivot: [f32; 2],
    target: [f32; 2],
    scale: f32,
    angle: f32,
    flip: bool,
    behind_body: bool,
    left_source: bool,
}
#[derive(Deserialize)]
struct Binding {
    model: u16,
    left: bool,
    colors: Vec<ColorFile>,
}
#[derive(Deserialize)]
struct ColorFile {
    index: usize,
    file: String,
}
#[derive(Deserialize)]
struct Gear {
    front_file: String,
    back_file: String,
    opening_file: String,
    ownership_file: String,
}
#[derive(Deserialize)]
struct Static {
    #[serde(default)]
    number: u16,
    #[serde(default)]
    variant: u8,
    file: String,
    cell: [u16; 2],
    anchor: [f32; 2],
    offset: i16,
    shadow: bool,
    shadow_width: u16,
}
pub fn init() {
    let _ = manifest();
}
fn manifest() -> &'static Manifest {
    static DATA: OnceLock<Manifest> = OnceLock::new();
    DATA.get_or_init(|| {
        let mut d: Manifest = serde_json::from_str(include_str!(
            "../assets/animations/vertical-18/manifest.json"
        ))
        .expect("accepted VERT18 manifest");
        assert_eq!(d.native_tile_px, 64);
        assert_eq!(d.profiles.len(), 20);
        assert_eq!(d.monsters.len(), 471);
        // Keep the established contact shadow of each monster; transparent
        // cropping and a new orientation do not define a new footprint.
        for entry in &mut d.monsters {
            entry.shadow_width = project_rl::monster_art::art(entry.number, entry.variant)
                .expect("accepted VERT18 monster identity")
                .shadow;
        }
        d
    })
}
#[derive(Clone, Debug)]
enum Selection {
    FixedPlayer {
        profile: usize,
        loadout: usize,
    },
    Player {
        profile: usize,
        main: Option<u16>,
        left: Option<u16>,
        helmet: Option<u16>,
        colors: [usize; 2],
    },
    Monster(usize),
    Npc,
}
fn selection(visual: Visual) -> Option<Selection> {
    let m = manifest();
    match visual {
        Visual::Player {
            body,
            equipment,
            weapon,
            hands,
            palettes,
        } => {
            let equipment = super::player_visual_equipment(equipment);
            // Class outfits keep their approved painted colors in every heading.
            // An item's palette must not disable its existing vertical poses.
            if equipment[1].is_some_and(|n| !(115..=120).contains(&n)) {
                return None;
            }
            let outfit = super::player_layers::outfit_index(body, equipment).checked_sub(512)? % 32;
            let profile = m.profiles.iter().position(|p| p.outfit == outfit)?;
            let (left, main) = hands.map_or((None, weapon), |[l, r]| (l, r));
            let p = &m.profiles[profile];
            if palettes[1] == EquipmentPalette::Original
                && palettes[2] == EquipmentPalette::Original
                && let Some(loadout) = p.fixed_loadouts.iter().position(|c| {
                    Some(c.main) == main && Some(c.left) == left && c.helmet == equipment[1]
                })
            {
                return Some(Selection::FixedPlayer { profile, loadout });
            }
            let main_case = match main {
                Some(model) => Some(p.cases.iter().find(|c| c.model == model && !c.left)?),
                None => None,
            };
            if main_case.is_none() && !p.families.contains_key(unarmed_family(p)) {
                return None;
            }
            if left.is_some() && main_case.is_some_and(|c| c.two_handed) {
                return None;
            }
            if left.is_some_and(|model| !p.cases.iter().any(|c| c.model == model && c.left)) {
                return None;
            }
            let colors = [
                if main.is_some() {
                    palettes[1].index()
                } else {
                    0
                },
                if left.is_some() {
                    palettes[2].index()
                } else {
                    0
                },
            ];
            if colors.iter().any(|c| *c >= 7) {
                return None;
            }
            Some(Selection::Player {
                profile,
                main,
                left,
                helmet: equipment[1],
                colors,
            })
        }
        Visual::CampaignMonster { number, variant } => m
            .monsters
            .iter()
            .position(|p| p.number == number && p.variant == variant)
            .map(Selection::Monster),
        Visual::CampNpc(1) => Some(Selection::Npc),
        _ => None,
    }
}
fn key(s: &Selection) -> String {
    match s {
        Selection::FixedPlayer { profile, loadout } => format!(
            "vertical18/fixed/{}/{loadout}",
            manifest().profiles[*profile].id
        ),
        Selection::Player {
            profile,
            main,
            left,
            helmet,
            colors,
        } => format!(
            "vertical18/player/{}/{main:?}/{left:?}/{helmet:?}/{colors:?}/rifle{}",
            manifest().profiles[*profile].id,
            if main.is_some_and(crate::rifle_visuals::rifle)
                || left.is_some_and(crate::rifle_visuals::rifle)
            {
                crate::rifle_visuals::variant()
            } else {
                0
            }
        ),
        Selection::Monster(i) => format!("vertical18/monster/{i}"),
        Selection::Npc => "vertical18/npc/1".into(),
    }
}
pub fn supports(visual: Option<Visual>) -> bool {
    visual.and_then(selection).is_some()
}
fn request(visual: Visual) -> Option<Request> {
    let s = selection(visual)?;
    let id = key(&s);
    #[cfg(debug_assertions)]
    let review = crate::animation_polish_review::source_override();
    Some((
        id,
        Box::new(move || {
            #[cfg(debug_assertions)]
            if let Some(style) = review {
                return crate::animation_polish_review::with_source(style, || prepare_image(&s));
            }
            prepare_image(&s)
        }),
    ))
}
pub fn prepare(visuals: &[Visual]) -> Result<std::time::Duration, String> {
    preparation::prepare(|| {
        visuals
            .iter()
            .flat_map(|&visual| {
                [request(visual), super::monster_vertical::request(visual)]
                    .into_iter()
                    .flatten()
            })
            .collect()
    })
}
pub fn anticipate(visuals: &[Visual]) {
    preparation::pump();
    for &visual in visuals.iter().take(4) {
        for (id, build) in [request(visual), super::monster_vertical::request(visual)]
            .into_iter()
            .flatten()
        {
            preparation::request(id, build);
        }
    }
}
fn static_image(p: &Static) -> Result<PreparedImage, String> {
    let image = preparation::image(&p.file)?;
    Ok(PreparedImage {
        image: (*image).clone(),
        cell: p.cell,
        anchor: p.anchor,
    })
}
fn prepare_image(s: &Selection) -> Result<PreparedImage, String> {
    let _span = project_rl::loading_profile::Span::new("compose_prepared_image", &key(s), 0);
    let m = manifest();
    match s {
        Selection::FixedPlayer { profile, loadout } => {
            static_image(&m.profiles[*profile].fixed_loadouts[*loadout].image)
        }
        Selection::Monster(i) => static_image(&m.monsters[*i]),
        Selection::Npc => static_image(&m.npc),
        Selection::Player {
            profile,
            main,
            left,
            helmet,
            colors,
        } => {
            let p = &m.profiles[*profile];
            let cases = [
                main.and_then(|n| p.cases.iter().find(|c| c.model == n && !c.left)),
                left.and_then(|n| p.cases.iter().find(|c| c.model == n && c.left)),
            ];
            // The intermediate banks already contain complete body/hand poses.
            // Reuse those figures without adding a weapon when unarmed.
            let family = cases[0].map_or_else(|| unarmed_family(p), |c| c.family.as_str());
            let pose = &p.families[family];
            let core = preparation::image(&pose.body_file)?;
            let fingers = preparation::image(&pose.hands_file)?;
            let mut frames = Vec::with_capacity(13);
            for n in 0..13 {
                let mut body = Image::gen_image_color(256, 256, BLANK);
                geometry::over_cell(&mut body, &core, pose.cell, n, pose.origin);
                if let Some(head) = helmet {
                    geometry::helmet(
                        &mut body,
                        &m.gear,
                        pose.frames[n].head_anchor,
                        (6..12).contains(&n),
                        *head,
                    )?;
                }
                let mut frame = Image::gen_image_color(256, 256, BLANK);
                for behind in [true, false] {
                    if !behind {
                        geometry::over(&mut frame, &body, [0, 0]);
                    }
                    for (slot, case) in cases.iter().enumerate() {
                        let Some(case) = case else {
                            continue;
                        };
                        let t = &case.frames[n];
                        if t.behind_body != behind {
                            continue;
                        }
                        let b = m
                            .bindings
                            .iter()
                            .find(|b| b.model == case.model && b.left == t.left_source)
                            .ok_or("VERT18 missing weapon binding")?;
                        let file = &b
                            .colors
                            .iter()
                            .find(|c| c.index == colors[slot])
                            .ok_or("VERT18 missing weapon palette")?
                            .file;
                        let source = preparation::image(file)?;
                        let mut candidate = None;
                        if crate::rifle_visuals::rifle(case.model) {
                            let (a, b) = crate::game_art::firearm_grips(case.model)
                                .ok_or("FIREARM02 missing authored grips")?;
                            candidate =
                                Some(crate::rifle_visuals::prepare(&source, case.model, a, b));
                        }
                        #[cfg(debug_assertions)]
                        if !crate::rifle_visuals::rifle(case.model)
                            && crate::animation_polish_review::source_style() > 0
                        {
                            candidate = Some(crate::animation_polish_review::vertical_weapon(
                                &source, case.model,
                            ));
                        }
                        let source = candidate.as_ref().unwrap_or(source.as_ref());
                        let separation = crate::rifle_visuals::enabled(case.model);
                        #[cfg(debug_assertions)]
                        let separation = separation
                            || crate::animation_polish_review::local_separation(case.model);
                        if separation {
                            let mut held = Image::gen_image_color(256, 256, BLANK);
                            geometry::weapon(&mut held, &source, t);
                            // A rear-carried weapon remains behind the body;
                            // only ink already in this depth layer is shaded.
                            let shadow = crate::rifle_visuals::contact_shadow(&frame, &held);
                            geometry::over(&mut frame, &shadow, [0, 0]);
                            geometry::over(&mut frame, &held, [0, 0]);
                            continue;
                        }
                        geometry::weapon(&mut frame, &source, t);
                    }
                }
                geometry::over_cell(&mut frame, &fingers, pose.cell, n, pose.origin);
                frames.push(frame);
            }
            // One crop for the complete cycle; transparent margins never alter scale.
            Ok(geometry::compact(&frames, p.anchor))
        }
    }
}
#[cfg(debug_assertions)]
pub(crate) fn review_image(visual: Visual) -> Result<PreparedImage, String> {
    prepare_image(&selection(visual).ok_or("ANIM-POLISH01 unsupported profile")?)
}
#[cfg(debug_assertions)]
pub(crate) fn review_heads(visual: Visual) -> Option<Vec<[f32; 2]>> {
    let Selection::Player { profile, main, .. } = selection(visual)? else {
        return None;
    };
    let p = &manifest().profiles[profile];
    let family = main
        .and_then(|m| p.cases.iter().find(|c| c.model == m && !c.left))
        .map_or_else(|| unarmed_family(p), |c| c.family.as_str());
    Some(
        p.families[family]
            .frames
            .iter()
            .map(|f| f.head_anchor)
            .collect(),
    )
}
pub fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let frame = match pose {
        None => 12,
        Some(Pose { facing: -2, frame }) if frame < 6 => usize::from(frame) + 6,
        Some(Pose { facing: 2, frame }) if frame < 6 => usize::from(frame),
        _ => return false,
    };
    let Some(s) = selection(visual) else {
        return false;
    };
    let (offset, shadow, width) = match &s {
        Selection::Player { .. } | Selection::FixedPlayer { .. } => (0, true, 17),
        Selection::Monster(i) => {
            let p = &manifest().monsters[*i];
            (p.offset, p.shadow, p.shadow_width)
        }
        Selection::Npc => (0, true, 17),
    };
    let scale = cell.w / 64.;
    preparation::with_texture(&key(&s), |p| {
        #[cfg(debug_assertions)]
        DRAWS.set(DRAWS.get() + 1);
        let mut at = cell;
        at.y += f32::from(offset) * scale;
        if shadow {
            super::draw_contact_shadow(at, [32, 63, width.min(255) as u8], 64, opacity);
        }
        let [w, h] = p.cell;
        crate::world_readability::texture(
            &p.texture,
            at.center().x - p.anchor[0] * scale,
            at.bottom() - p.anchor[1] * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    (frame % 6) as f32 * f32::from(w),
                    (frame / 6) as f32 * f32::from(h),
                    f32::from(w),
                    f32::from(h),
                )),
                dest_size: Some(vec2(f32::from(w) * scale, f32::from(h) * scale)),
                ..Default::default()
            },
        );
    })
    .is_some()
}

#[cfg(test)]
#[path = "game_art_vertical_tests.rs"]
mod tests;
