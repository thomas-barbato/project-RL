//! VERT18-C01: prepared, bounded, review-only front/back complete figures.
use crate::{
    combat_visuals::CombatMotion,
    game_art::{self, Visual, walking::Pose},
    world_readability,
};
use macroquad::prelude::*;
use project_rl::{
    asset_cache::ByteCache,
    entity::{EntityId, EquipmentPalette},
    world::GridPos,
};
use serde::Deserialize;
use std::{
    cell::{Cell, RefCell},
    path::Path,
    sync::OnceLock,
};

type Key = (
    u8,
    u8,
    [Option<u16>; 4],
    Option<u16>,
    Option<[Option<u16>; 2]>,
    [EquipmentPalette; 3],
);
#[path = "vertical_walk_equipment18.rs"]
mod equipment;
#[path = "vertical_walk_headgear18.rs"]
mod heads;
pub async fn capture_heads(output: &Path) -> Result<(), String> {
    heads::capture(output).await
}
#[path = "vertical_walk_roster18.rs"]
mod roster;
pub async fn capture_equipment(output: &Path) -> Result<(), String> {
    equipment::capture(output).await
}
pub async fn capture_roster(output: &Path) -> Result<(), String> {
    roster::capture(output).await
}
pub async fn capture_sizes(output: &Path) -> Result<(), String> {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            SIZES.set(self.0);
        }
    }
    let _guard = Guard(SIZES.replace(true));
    capture(&output.join("base")).await?;
    roster::capture(&output.join("roster")).await?;
    equipment::capture(&output.join("equipment")).await?;
    heads::capture(&output.join("heads")).await?;
    std::fs::write(
        output.join("size-checks.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "review":"VERT18-C05","normal_game_enabled":false,
            "body_profiles":9,"headgear_profiles":4,"head_choices":7,
            "authored_front_idle":true,"runtime_size_scaling":false,
            "old_approval_references_preserved":true
        }))
        .unwrap(),
    )
    .map_err(|e| e.to_string())
}
#[derive(Deserialize)]
struct Frame {
    direction: String,
    phase: u8,
    palms: [[f32; 2]; 2],
    #[serde(default)]
    head_anchor: Option<[f32; 2]>,
}
#[derive(Deserialize)]
struct Profile {
    id: String,
    selector: Selector,
    file: String,
    standing: String,
    frames: Vec<Frame>,
    #[serde(default)]
    unarmed_only: bool,
    #[serde(default)]
    head_variants: bool,
    #[serde(default)]
    head_parts: Option<heads::Parts>,
    #[serde(default)]
    size_revision: bool,
    #[serde(default)]
    standing_palms: Option<[[f32; 2]; 2]>,
    #[serde(default)]
    standing_head_anchor: Option<[f32; 2]>,
}
#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Selector {
    Player {
        body: u8,
        equipment: [Option<u16>; 4],
    },
    CampNpc {
        index: u8,
    },
    Actor {
        body: u8,
    },
}
impl Selector {
    fn matches(&self, visual: Visual) -> bool {
        match (self, visual) {
            (
                Self::Player { body, equipment },
                Visual::Player {
                    body: actual,
                    equipment: worn,
                    ..
                },
            ) => *body == actual && *equipment == worn,
            (Self::CampNpc { index }, Visual::CampNpc(actual)) => *index == actual,
            (
                Self::Actor { body },
                Visual::Actor(actual) | Visual::ArmedActor { body: actual, .. },
            ) => *body == actual,
            _ => false,
        }
    }
}
#[derive(Deserialize)]
struct Manifest {
    profiles: Vec<Profile>,
}
fn manifest() -> &'static Manifest {
    static DATA: OnceLock<Manifest> = OnceLock::new();
    DATA.get_or_init(|| {
        let mut original: Manifest = serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-r01/manifest.json"
        ))
        .expect("VERT18 authored manifest");
        let next: Manifest = serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-C02/manifest.json"
        ))
        .expect("VERT18 roster manifest");
        original.profiles.extend(next.profiles);
        let armored: Manifest = serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-C03/manifest.json"
        ))
        .expect("VERT18 equipment manifest");
        original.profiles.extend(armored.profiles);
        let headgear: Manifest = serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-C04/manifest.json"
        ))
        .expect("VERT18 headgear manifest");
        original.profiles.extend(headgear.profiles);
        let sizes: Manifest = serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-C05/manifest.json"
        ))
        .expect("VERT18 size revision manifest");
        original.profiles.extend(sizes.profiles);
        original
    })
}
thread_local! {
    static ACTIVE: Cell<bool> = const { Cell::new(false) };
    static HEADS: Cell<bool> = const { Cell::new(false) };
    static SIZES: Cell<bool> = const { Cell::new(false) };
    static SOURCES: RefCell<ByteCache<String, Image>> = const { RefCell::new(ByteCache::new(4*1024*1024)) };
    static READY: RefCell<ByteCache<Key, Texture2D>> = const { RefCell::new(ByteCache::new(8*1024*1024)) };
}
fn key(visual: Visual) -> Option<(Key, &'static str)> {
    let mut key = match visual {
        Visual::Player {
            body,
            equipment,
            weapon,
            hands,
            palettes,
        } => (
            if HEADS.get() { 3 } else { 0 },
            body,
            equipment,
            weapon,
            hands,
            palettes,
        ),
        Visual::CampNpc(index) => (
            1,
            index,
            [None; 4],
            None,
            None,
            [EquipmentPalette::Original; 3],
        ),
        Visual::Actor(body) => (
            2,
            body,
            [None; 4],
            None,
            None,
            [EquipmentPalette::Original; 3],
        ),
        Visual::ArmedActor { body, weapon } => (
            2,
            body,
            [None; 4],
            Some(weapon),
            None,
            [EquipmentPalette::Original; 3],
        ),
        _ => return None,
    };
    if SIZES.get() {
        key.0 += 4;
    }
    let selector_visual = if HEADS.get() {
        match visual {
            Visual::Player {
                body,
                mut equipment,
                weapon,
                hands,
                palettes,
            } => {
                equipment[1] = None;
                Visual::Player {
                    body,
                    equipment,
                    weapon,
                    hands,
                    palettes,
                }
            }
            _ => return None,
        }
    } else {
        visual
    };
    let profile = manifest().profiles.iter().find(|p| {
        p.size_revision == SIZES.get()
            && p.head_variants == HEADS.get()
            && p.selector.matches(selector_visual)
    })?;
    Some((key, profile.id.as_str()))
}

#[cfg(test)]
mod size_tests {
    use super::*;

    #[test]
    fn size05_selects_all_thirteen_profiles_without_reusing_old_cache_keys() {
        struct Restore(bool, bool);
        impl Drop for Restore {
            fn drop(&mut self) {
                SIZES.set(self.0);
                HEADS.set(self.1);
            }
        }
        let _restore = Restore(SIZES.get(), HEADS.get());
        let mut checked = 0;
        for candidate in manifest().profiles.iter().filter(|p| p.size_revision) {
            HEADS.set(candidate.head_variants);
            let visual = match &candidate.selector {
                Selector::Player { body, equipment } => {
                    let mut equipment = *equipment;
                    if candidate.head_variants {
                        equipment[1] = Some(118);
                    }
                    Visual::Player {
                        body: *body,
                        equipment,
                        weapon: None,
                        hands: None,
                        palettes: [EquipmentPalette::Original; 3],
                    }
                }
                Selector::CampNpc { index } => Visual::CampNpc(*index),
                Selector::Actor { body } => Visual::Actor(*body),
            };
            SIZES.set(false);
            let (before, before_id) = key(visual).unwrap();
            SIZES.set(true);
            let (after, after_id) = key(visual).unwrap();
            assert_eq!(after_id, candidate.id);
            assert_ne!(before_id, after_id);
            assert_eq!(after.0, before.0 + 4);
            assert_ne!(before, after);
            assert!(candidate.standing_palms.is_some());
            assert!(candidate.standing_head_anchor.is_some());
            assert_eq!(candidate.frames.len(), 12);
            SIZES.set(false);
            assert_eq!(key(visual), Some((before, before_id)));
            checked += 1;
        }
        assert_eq!(checked, 13);
    }
}

fn image(path: &str) -> Result<Image, String> {
    SOURCES.with_borrow_mut(|cache| {
        if let Some(image) = cache.get(&path.to_owned()) {
            return Ok(image.clone());
        }
        let bytes = project_rl::data_paths::read(path).map_err(|e| e.to_string())?;
        let image = crate::asset_loading::image(&bytes, Some(ImageFormat::Png))
            .map_err(|e| e.to_string())?;
        cache.insert(path.to_owned(), image.clone(), image.bytes.len());
        Ok(image)
    })
}
pub fn supports(visual: Option<Visual>) -> bool {
    ACTIVE.get()
        && visual
            .and_then(key)
            .is_some_and(|(key, _)| READY.with_borrow_mut(|cache| cache.get(&key).is_some()))
}
async fn prepare(visuals: &[Visual], output: &Path) -> Result<usize, String> {
    let mut contacts = 0;
    for &visual in visuals {
        if matches!(visual, Visual::ArmedActor { .. }) {
            return Err("VERT18 armed NPCs need an equipment-specific standing reference".into());
        }
        let (key, id) = key(visual).ok_or("VERT18 unsupported review visual")?;
        let profile = manifest()
            .profiles
            .iter()
            .find(|p| p.id == id)
            .ok_or("VERT18 profile missing")?;
        if profile.unarmed_only
            && matches!(visual, Visual::Player { weapon, hands, .. } if weapon.is_some() || hands.is_some_and(|h|h.into_iter().any(|w|w.is_some())))
        {
            return Err("VERT18 roster equipment still requires authored grip review".into());
        }
        let source = image(&profile.file)?;
        if (source.width, source.height) != (768, 256) {
            return Err("VERT18 source canvas mismatch".into());
        }
        let mut atlas = Image::gen_image_color(768, 384, BLANK);
        for frame in &profile.frames {
            let back = frame.direction == "back";
            let row = usize::from(back);
            let mut original = source.sub_image(Rect::new(
                f32::from(frame.phase) * 128.,
                row as f32 * 128.,
                128.,
                128.,
            ));
            if profile.head_variants {
                heads::helmet(
                    &mut original,
                    visual,
                    frame.head_anchor.ok_or("C04 missing head anchor")?,
                    back,
                )?;
            }
            let composed =
                game_art::vertical_review_composition(&original, visual, frame.palms, back)
                    .ok_or("VERT18 loadout requires separate authored grips")?;
            if let Visual::Player { weapon, hands, .. } = visual {
                contacts += hands.map_or(usize::from(weapon.is_some()), |h| {
                    h.into_iter().flatten().count()
                });
            }
            for y in 0..128usize {
                let at = ((row * 128 + y) * 768 + usize::from(frame.phase) * 128) * 4;
                atlas.bytes[at..at + 512].copy_from_slice(&composed.bytes[y * 512..(y + 1) * 512]);
            }
        }
        let standing = if profile.size_revision {
            let mut body = image(&profile.standing)?;
            if profile.head_variants {
                heads::helmet(
                    &mut body,
                    visual,
                    profile
                        .standing_head_anchor
                        .ok_or("C05 idle head anchor missing")?,
                    false,
                )?;
            }
            game_art::vertical_review_composition(
                &body,
                visual,
                profile.standing_palms.ok_or("C05 idle palms missing")?,
                false,
            )
            .ok_or("C05 coherent idle grips missing")?
        } else if profile.head_variants {
            let mut failure = None;
            let prepared = game_art::vertical_review_standing_head(visual, |body| {
                if let Err(error) = heads::standing(body, visual, profile) {
                    failure = Some(error);
                }
            });
            if let Some(error) = failure {
                return Err(error);
            }
            prepared.ok_or("C04 standing layers missing")?
        } else {
            game_art::vertical_review_standing(visual).unwrap_or(image(&profile.standing)?)
        };
        for y in 0..128usize {
            let at = ((256 + y) * 768) * 4;
            atlas.bytes[at..at + 512].copy_from_slice(&standing.bytes[y * 512..(y + 1) * 512]);
        }
        atlas.export_png(
            output
                .join(format!(
                    "prepared-{}-{}.png",
                    id,
                    usize::from(key.4.is_some())
                ))
                .to_str()
                .unwrap(),
        );
        let texture = crate::asset_loading::from_image(&atlas);
        texture.set_filter(FilterMode::Nearest);
        READY.with_borrow_mut(|cache| {
            cache.insert(key, texture, atlas.bytes.len());
        });
        next_frame().await;
    }
    Ok(contacts)
}
fn render(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some((key, _)) = key(visual) else {
        return false;
    };
    let (column, row) = match pose {
        Some(Pose { facing: -2, frame }) if frame < 6 => (frame, 1),
        Some(Pose { facing: 2, frame }) if frame < 6 => (frame, 0),
        None => (0, 2),
        _ => return false,
    };
    READY.with_borrow_mut(|cache| {
        let Some(texture) = cache.get(&key) else {
            return false;
        };
        draw_ellipse(
            cell.center().x,
            cell.bottom() - 1.,
            18.,
            2.5,
            0.,
            Color::from_rgba(8, 12, 16, 65),
        );
        let _actor = world_readability::actor(true);
        world_readability::texture(
            texture,
            cell.x - cell.w / 2.,
            cell.y - cell.h / 2.,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    f32::from(column) * 128.,
                    row as f32 * 128.,
                    128.,
                    128.,
                )),
                dest_size: Some(cell.size() * 2.),
                ..Default::default()
            },
        );
        true
    })
}
pub fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    ACTIVE.get() && pose.is_some() && render(cell, visual, opacity, pose)
}

/// Same prepared figures inside the accepted camp's real material/light pass.
/// This is a presentation fixture, with no campaign or navigation mutation.
async fn capture_camp(output: &Path, review: &str, visuals: &[Visual]) -> Result<(), String> {
    capture_camp_with(output, review, visuals.len(), false, |cell, index, pose| {
        render(cell, visuals[index], 1., pose)
    })
    .await
}

/// Prepared complete figures in the same real scenery and exposure pass.
/// Only the explicit review fixture can call this; ordinary actors are untouched.
pub(super) async fn capture_prepared_camp(
    output: &Path,
    review: &str,
    count: usize,
    render: impl FnMut(Rect, usize, Option<Pose>) -> bool,
) -> Result<(), String> {
    capture_camp_with(output, review, count, true, render).await
}

async fn capture_camp_with(
    output: &Path,
    review: &str,
    count: usize,
    compact: bool,
    mut render: impl FnMut(Rect, usize, Option<Pose>) -> bool,
) -> Result<(), String> {
    let positions = match count {
        2 => vec![3, 5],
        3 => vec![2, 4, 6],
        _ => return Err("VERT18 camp fixture supports two or three subjects".into()),
    };
    let camp = crate::cryogenic_camp::scene();
    let upload = |path: &str| -> Result<Texture2D, String> {
        let texture = crate::asset_loading::from_image(&image(path)?);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
    };
    let floor = upload(&camp.floor)?;
    let structure = upload(&camp.structure)?;
    let mut props = Vec::new();
    for prop in &camp.props {
        props.push((prop, upload(&prop.file)?));
    }
    props.sort_by_key(|(p, _)| (p.at[1] + p.height, p.at[0]));
    let atmosphere = crate::camp_atmosphere::Atmosphere::new(
        &camp.atmosphere,
        camp.width as u32 * 64,
        camp.height as u32 * 64,
    );
    next_frame().await;
    let world = Rect::new(
        16.,
        -260.,
        camp.width as f32 * 64.,
        camp.height as f32 * 64.,
    );
    let clip = Rect::new(16., 60., 576., 512.);
    let mut motion = CombatMotion::default();
    let mut records = Vec::new();
    let mut front_returns = 0;
    for step in 0..4 {
        let (from_y, to_y) = match step {
            0 => (9, 8),
            1 => (8, 7),
            2 => (7, 8),
            _ => (8, 9),
        };
        for (col, &x) in positions.iter().enumerate() {
            let id = serde_json::from_str(&(col + 101).to_string()).unwrap();
            motion.glide_timed(
                id,
                GridPos::new(x, from_y),
                GridPos::new(x, to_y),
                10. + f64::from(step),
                400,
            );
        }
        for tick in 0..13 {
            let now = 10.
                + f64::from(step)
                + if tick == 12 {
                    0.401
                } else {
                    (f64::from(tick) + 0.5) / 12. * 0.4
                };
            for warm in 0..2 {
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(30, 40, 49, 255));
                draw_text(
                    &format!("{review} / camp accepte / natif64"),
                    16.,
                    27.,
                    21.,
                    WHITE,
                );
                draw_text(
                    if tick == 12 {
                        "Arret : face d'origine"
                    } else if step < 2 {
                        "Haut : dos"
                    } else {
                        "Bas : face"
                    },
                    16.,
                    596.,
                    19.,
                    WHITE,
                );
                let _world = world_readability::world(false);
                atmosphere.begin(world);
                atmosphere.background_time(0.);
                atmosphere.begin_background(true);
                for texture in [&floor, &structure] {
                    draw_texture_ex(
                        texture,
                        world.x,
                        world.y,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(world.size()),
                            ..Default::default()
                        },
                    );
                }
                atmosphere.end_material();
                let mut next_prop = 0;
                for (col, &x) in positions.iter().enumerate() {
                    while let Some((prop, texture)) = props.get(next_prop) {
                        if (prop.at[1] + prop.height, prop.at[0]) > (to_y, x) {
                            break;
                        }
                        next_prop += 1;
                        draw_texture_ex(
                            texture,
                            world.x + prop.at[0] as f32 * 64. + prop.width as f32 * 32. - 64.,
                            world.y + (prop.at[1] + prop.height) as f32 * 64. - 96.,
                            WHITE,
                            DrawTextureParams::default(),
                        );
                    }
                    let id = serde_json::from_str(&(col + 101).to_string()).unwrap();
                    let pose = motion.vertical_walk_pose(id, now, false);
                    let offset = motion.glide_offset(id, now, false);
                    let cell = Rect::new(
                        world.x + x as f32 * 64.,
                        world.y + (to_y as f32 + offset.y) * 64.,
                        64.,
                        64.,
                    );
                    let draws = world_readability::audit_textures(|| {
                        assert!(render(cell, col, pose));
                    });
                    if draws.is_empty()
                        || draws
                            .iter()
                            .any(|d| !d.actor || !d.world || !d.finish || d.gain != 1.48)
                    {
                        return Err("VERT18 camp exposure pipeline mismatch".into());
                    }
                    if tick == 12 && pose.is_some() {
                        return Err("VERT18 camp return mismatch".into());
                    }
                    if warm == 1 {
                        front_returns += usize::from(tick == 12);
                        records.push(serde_json::json!({"step":step,"tick":tick,"subject":col,"cell":[cell.x,cell.y,cell.w,cell.h],"actor_gain":draws[0].gain,"pose":pose.map(|p|[i16::from(p.facing),i16::from(p.frame)])}));
                    }
                }
                for (prop, texture) in &props[next_prop..] {
                    draw_texture_ex(
                        texture,
                        world.x + prop.at[0] as f32 * 64. + prop.width as f32 * 32. - 64.,
                        world.y + (prop.at[1] + prop.height) as f32 * 64. - 96.,
                        WHITE,
                        DrawTextureParams::default(),
                    );
                }
                atmosphere.finish(world, clip, true);
                unsafe {
                    get_internal_gl().quad_gl.scissor(None);
                }
                if warm == 0 {
                    next_frame().await;
                } else if !compact || (matches!(step, 0 | 2) && matches!(tick, 0 | 6 | 12)) {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("camp-step{step}-tick{tick:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
            }
        }
    }
    // Compact captures can leave the final draw queued. Flush while the camp's
    // materials/textures are alive, before a subsequent review group starts.
    next_frame().await;
    std::fs::write(output.join("camp-checks.json"), serde_json::to_vec_pretty(&serde_json::json!({"review":review,"status":"review_only","normal_game_enabled":false,"scene":"assets/maps/cryogenic-camp-01/camp.json","native_tile_px":64,"camp_material_and_irradiance":true,"fixed_light_time":0,"moving_and_idle_actor_gain":1.48,"front_returns":front_returns,"viewport":[16,60,576,512],"saved_game_loaded":false,"records":records})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}

pub async fn capture(output: &Path) -> Result<(), String> {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            ACTIVE.set(self.0);
        }
    }
    let _guard = Guard(ACTIVE.replace(true));
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1040., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let player = Visual::Player {
        body: 6,
        equipment: [None; 4],
        weapon: None,
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    };
    let equipped = Visual::Player {
        body: 6,
        equipment: [None; 4],
        weapon: Some(70),
        hands: Some([Some(104), Some(70)]),
        palettes: [EquipmentPalette::Original; 3],
    };
    let visuals = [player, equipped, Visual::CampNpc(1)];
    let held_weapon_contacts = prepare(&visuals, output).await?;
    let mut motion = CombatMotion::default();
    let mut records = Vec::new();
    let mut front_returns = 0;
    for step in 0..2 {
        for row in 0..2 {
            let dy = if row == 0 { -1 } else { 1 };
            for col in 0..3 {
                let id: EntityId = serde_json::from_str(&(row * 3 + col + 1).to_string()).unwrap();
                motion.glide_timed(
                    id,
                    GridPos::new(0, step * dy),
                    GridPos::new(0, (step + 1) * dy),
                    1. + f64::from(step),
                    400,
                );
            }
        }
        for tick in 0..13 {
            let now = 1.
                + f64::from(step)
                + if tick == 12 {
                    0.401
                } else {
                    (f64::from(tick) + 0.5) / 12. * 0.4
                };
            for warm in 0..2 {
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(30, 40, 49, 255));
                draw_text(
                    "VERT18-C01 / marche verticale / silhouettes natives64",
                    16.,
                    27.,
                    21.,
                    WHITE,
                );
                for (col, label) in ["Joueur", "Joueur - deux armes", "PNJ du camp"]
                    .into_iter()
                    .enumerate()
                {
                    draw_text(label, 154. + col as f32 * 288., 58., 17., WHITE);
                }
                draw_text("Vers le haut : dos", 12., 89., 17., SKYBLUE);
                draw_text("Vers le bas : face", 12., 398., 17., SKYBLUE);
                draw_text(
                    if tick == 12 {
                        if SIZES.get() {
                            "Arret : pose frontale du meme cycle / C05"
                        } else {
                            "Arret : retour au dessin de face d'origine"
                        }
                    } else {
                        "Un pas = 400 ms / trois phases / appuis alternes"
                    },
                    16.,
                    724.,
                    17.,
                    WHITE,
                );
                let _world = world_readability::world(false);
                for row in 0..2 {
                    let dy = if row == 0 { -1 } else { 1 };
                    for (col, &visual) in visuals.iter().enumerate() {
                        let id: EntityId =
                            serde_json::from_str(&(row * 3 + col + 1).to_string()).unwrap();
                        let pose = motion.vertical_walk_pose(id, now, false);
                        if tick == 12 && pose.is_some() {
                            return Err("VERT18 actor retained movement at arrival".into());
                        }
                        if tick < 12 && pose.is_none_or(|p| p.facing != dy * 2) {
                            return Err("VERT18 actor facing does not match Y direction".into());
                        }
                        let offset = motion.glide_offset(id, now, false);
                        let y = if row == 0 { 234. } else { 414. }
                            + (step + 1) as f32 * dy as f32 * 64.
                            + offset.y * 64.;
                        let cell = Rect::new(168. + col as f32 * 288., y, 64., 64.);
                        if !render(cell, visual, 1., pose) {
                            return Err("VERT18 prepared texture missing".into());
                        }
                        if warm == 1 {
                            records.push(serde_json::json!({"step":step,"tick":tick,"row":row,"subject":col,"pose":pose.map(|p|[i16::from(p.facing),i16::from(p.frame)])}));
                            if tick == 12 {
                                front_returns += 1;
                            }
                        }
                    }
                }
                if warm == 0 {
                    next_frame().await;
                } else {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("step{step}-tick{tick:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
            }
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"VERT18-C01","status":"review_only","normal_game_enabled":false,
        "authored_profiles":2,"authored_moving_figures":24,"loadouts":3,"front_returns":front_returns,
        "held_weapon_contacts_verified":held_weapon_contacts,
        "shared_player_npc_motion":true,"step_duration_ms":400,"read_or_decode_during_draw":false,
        "source_cache_limit_bytes":4*1024*1024,"prepared_texture_cache_limit_bytes":8*1024*1024,
        "saved_game_loaded":false,"user_preferences_written":false,"prepared_pngs_need_vertical_flip":true,"records":records
    })).unwrap()).map_err(|e|e.to_string())?;
    capture_camp(output, "VERT18-C01", &[equipped, visuals[2]]).await
}
