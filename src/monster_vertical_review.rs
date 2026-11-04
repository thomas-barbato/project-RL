//! Native vertical-monster comparisons and approved atlas verification.
use crate::{
    combat_visuals::CombatMotion,
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::{entity::EntityId, world::GridPos};
use serde::Deserialize;
use std::{collections::BTreeSet, path::Path};

const SOURCE: &str = "assets/source-art/animation-polish-01/monster-vertical-01/r01/manifest.json";
const WIDTH: usize = 192;
const HEIGHT: usize = 320;
#[derive(Deserialize)]
struct Profile {
    id: String,
    number: u16,
    name: String,
    atlas: String,
    vertical_offset: i16,
    atlas_anchor_y: Option<u16>,
    front_shadow: u8,
    casts_shadow: Option<bool>,
    material_palette: Option<String>,
    comparison: Option<String>,
    previous_atlas: Option<String>,
    previous_material_palette: Option<String>,
}
#[derive(Deserialize)]
struct RedMaterialPalette {
    model: u16,
    variant: u8,
    #[serde(default)]
    red_min: u8,
    #[serde(default)]
    other_channel_max_ratio: f32,
    #[serde(default)]
    red_lut: Vec<(u8, f32, f32)>,
    material: Option<String>,
    ochre_lut: Option<Vec<(u8, f32, f32)>>,
    material_luts: Option<Vec<Vec<[u8; 3]>>>,
}
fn arthropod_material(c: &[u8]) -> usize {
    if c[3] == 0 {
        return 0;
    }
    let (r, g, b) = (u16::from(c[0]), u16::from(c[1]), u16::from(c[2]));
    let high = r.max(g).max(b);
    let low = r.min(g).min(b);
    if r > 35 && 100 * g < 72 * r && 100 * b < 65 * r {
        1
    } else if high > 45 && 100 * g >= 65 * r && 100 * b < 88 * g && 100 * (high - low) > 12 * high {
        2
    } else if high > 24 {
        3
    } else {
        0
    }
}
fn review_steel_material(c: &[u8]) -> bool {
    let (r, g, b) = (u16::from(c[0]), u16::from(c[1]), u16::from(c[2]));
    let high = r.max(g).max(b);
    let spread = high - r.min(g).min(b);
    c[3] != 0
        && high > 65
        && (100 * spread < 18 * high || (r <= g && r <= b && 100 * spread < 35 * high))
}
fn rust_material(c: &[u8]) -> usize {
    if c[3] == 0 {
        return 0;
    }
    let (r, g, b) = (f32::from(c[0]), f32::from(c[1]), f32::from(c[2]));
    if r > 40. && g >= 0.12 * r && g < 0.70 * r && b >= 0.06 * r && b < 0.60 * r {
        1
    } else if r > 25. && r >= g && g >= b {
        2
    } else {
        0
    }
}
fn blue_front_material(c: &[u8]) -> u8 {
    if c[3] == 0 {
        return 0;
    }
    let (r, g, b) = (
        f64::from(c[0]) / 255.,
        f64::from(c[1]) / 255.,
        f64::from(c[2]) / 255.,
    );
    let high = r.max(g).max(b);
    let low = r.min(g).min(b);
    let delta = high - low;
    if delta == 0. {
        return 0;
    }
    let h = if high == r {
        ((g - b) / delta).rem_euclid(6.) / 6.
    } else if high == g {
        ((b - r) / delta + 2.) / 6.
    } else {
        ((r - g) / delta + 4.) / 6.
    };
    let s = delta / high;
    if h > 0.52 && h < 0.75 && s > 0.10 && high > 0.12 {
        1
    } else if h > 0.085 && h < 0.19 && s > 0.35 && high > 0.25 {
        2
    } else {
        0
    }
}
impl RedMaterialPalette {
    fn material(&self, c: &[u8]) -> bool {
        if self.material.as_deref() == Some("arthropod_materials") {
            return arthropod_material(c) != 0;
        }
        if matches!(
            self.material.as_deref(),
            Some("rust_materials" | "rust_materials_armed" | "earth_organic_materials")
        ) {
            return rust_material(c) != 0
                && (self.material.as_deref() != Some("rust_materials_armed")
                    || !review_steel_material(c));
        }
        if self.material.as_deref() == Some("blue_ochre_front") {
            return blue_front_material(c) != 0;
        }
        if self.material.as_deref() == Some("warm_ivory") {
            let (r, g, b) = (f32::from(c[0]), f32::from(c[1]), f32::from(c[2]));
            return c[3] != 0
                && r > 51.
                && r >= g
                && g >= b
                && (r - b) > 0.08 * r
                && (r - b) < 0.55 * r
                && (g - b) > 0.36 * (r - b);
        }
        c[3] != 0
            && c[0] > self.red_min
            && f32::from(c[1]) < f32::from(c[0]) * self.other_channel_max_ratio
            && f32::from(c[2]) < f32::from(c[0]) * self.other_channel_max_ratio
    }
    fn apply(&self, image: &mut Image) -> Result<usize, String> {
        let scope = (self.model, self.variant, self.material.as_deref());
        let channel_palette = matches!(
            self.material.as_deref(),
            Some(
                "rust_materials"
                    | "rust_materials_armed"
                    | "earth_organic_materials"
                    | "arthropod_materials"
            )
        );
        let materials_per_view = if self.material.as_deref() == Some("arthropod_materials") {
            3
        } else {
            2
        };
        if !matches!(
            scope,
            (144, 0, None)
                | (6 | 25 | 35 | 151, 0, Some("warm_ivory"))
                | (36, 0, Some("blue_ochre_front"))
                | (60, 0, Some("rust_materials" | "rust_materials_armed"))
                | (63..=68, 0, Some("earth_organic_materials"))
                | (69..=74, 0, Some("arthropod_materials"))
        ) || (!channel_palette && self.red_lut.len() != 256)
            || (image.width, image.height) != (1440, 256)
        {
            return Err("Invalid reviewed material-palette scope".into());
        }
        if channel_palette
            && self.material_luts.as_ref().is_none_or(|luts| {
                luts.len() != materials_per_view * 2 || luts.iter().any(|lut| lut.len() != 256)
            })
        {
            return Err("Missing reviewed material channel lookups".into());
        }
        let blue_front = self.material.as_deref() == Some("blue_ochre_front");
        if blue_front && self.ochre_lut.as_ref().is_none_or(|lut| lut.len() != 256) {
            return Err("Missing reviewed ochre lookup".into());
        }
        let mut changed = 0;
        for (index, c) in image.bytes.chunks_exact_mut(4).enumerate() {
            // Preserve both idles, other materials and MON-036's existing rear.
            if index % 1440 >= 1280 || (blue_front && index / 1440 >= 128) || !self.material(c) {
                continue;
            }
            if channel_palette {
                let kind = if materials_per_view == 3 {
                    arthropod_material(c)
                } else {
                    rust_material(c)
                };
                let group = index / (1440 * 128) * materials_per_view + kind - 1;
                let lut = &self.material_luts.as_ref().unwrap()[group];
                let rgb = [
                    lut[usize::from(c[0])][0],
                    lut[usize::from(c[1])][1],
                    lut[usize::from(c[2])][2],
                ];
                changed += usize::from(c[..3] != rgb);
                c[..3].copy_from_slice(&rgb);
                continue;
            }
            let lut = if blue_front && blue_front_material(c) == 2 {
                self.ochre_lut.as_ref().unwrap()
            } else {
                &self.red_lut
            };
            let (red, green, blue) = lut[usize::from(c[0])];
            let rgb = [
                red,
                (f32::from(c[1]) * green).round().clamp(0., 255.) as u8,
                (f32::from(c[2]) * blue).round().clamp(0., 255.) as u8,
            ];
            changed += usize::from(c[..3] != rgb);
            c[..3].copy_from_slice(&rgb);
        }
        Ok(changed)
    }
}
fn phase_at(sample: usize) -> usize {
    let t = sample as f32 / 32. * 1300.;
    let mut end = 0.;
    for (phase, duration) in [180., 180., 170., 120., 180., 180., 170., 120.]
        .into_iter()
        .enumerate()
    {
        end += duration;
        if t < end {
            return phase;
        }
    }
    7
}
fn panel(board: &mut Image, source: &Image, col: usize, tall: bool) {
    let (rows, top) = if tall { (288, 214) } else { (160, 278) };
    let stride = board.width as usize;
    for y in 0..rows {
        let src = ((y + top) * 640 + 208) * 4;
        let dst = (y * stride + col * WIDTH) * 4;
        board.bytes[dst..dst + WIDTH * 4].copy_from_slice(&source.bytes[src..src + WIDTH * 4]);
    }
}
fn exposure(audit: &[crate::world_readability::TextureAudit]) -> Result<(), String> {
    if audit.is_empty()
        || audit
            .iter()
            .any(|a| !a.world || !a.actor || (a.gain - 1.48).abs() > 0.001)
    {
        return Err("Monster exposure or world scope changed".into());
    }
    Ok(())
}
fn current(scene: &Scene, visual: Visual, pose: Option<Pose>, shift: f32) -> Result<Image, String> {
    if pose.is_some_and(|p| matches!(p.facing, -2 | 2)) {
        let (image, audit) = scene.render_actor(|mut cell| {
            let _player = crate::world_readability::player_scope(false);
            let _actor = crate::world_readability::actor(true);
            cell.y += shift;
            assert!(game_art::vertical::draw(cell, visual, 1., pose));
        });
        exposure(&audit)?;
        return Ok(image);
    }
    render_world(scene, visual, pose, shift)
}

fn render_world(
    scene: &Scene,
    visual: Visual,
    pose: Option<Pose>,
    shift: f32,
) -> Result<Image, String> {
    let (image, audit) = scene.render_actor(|mut cell| {
        cell.y += shift;
        walking::with_pose(pose, || {
            game_art::draw_world_animated(cell, visual, 1., None)
        });
    });
    exposure(&audit)?;
    Ok(image)
}
fn candidate(
    scene: &Scene,
    profile: &Profile,
    texture: &Texture2D,
    row: usize,
    phase: usize,
    shift: f32,
) -> Result<Image, String> {
    let (image, audit) = scene.render_actor(|mut cell| {
        let _player = crate::world_readability::player_scope(false);
        let _actor = crate::world_readability::actor(true);
        cell.y += shift + f32::from(profile.vertical_offset);
        let shadow = if phase == 8 {
            profile.front_shadow
        } else {
            project_rl::monster_art::art(profile.number, 0)
                .unwrap()
                .shadow
                .try_into()
                .expect("native monster shadow width")
        };
        if profile.casts_shadow.unwrap_or(true) {
            game_art::polish_review_shadow_width(cell, shadow);
        }
        crate::world_readability::texture(
            texture,
            cell.center().x - 80.,
            cell.bottom() - f32::from(profile.atlas_anchor_y.unwrap_or(112)),
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    phase as f32 * 160.,
                    row as f32 * 128.,
                    160.,
                    128.,
                )),
                dest_size: Some(vec2(160., 128.)),
                ..Default::default()
            },
        );
    });
    exposure(&audit)?;
    Ok(image)
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    capture_from(output, SOURCE).await
}
pub(crate) async fn capture_next(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-02/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_color(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-color-01/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_third(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-03/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_fourth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-04/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_fifth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-05/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_sixth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-06/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_seventh(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-07/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_eighth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-08/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_quadruped21(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-21-quad/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_ninth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-09/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_tenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-10/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_arms28(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-28-arms/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_eleventh(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-11/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_color28(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-28-color/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_weapon28(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-28-weapon/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_twelfth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-12/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_thirteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-13/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_fourteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-14/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_fourteenth_rear(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-14/r02/manifest.json",
    )
    .await
}
pub(crate) async fn capture_fifteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-15/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_sixteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-16/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_seventeenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-17/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_eighteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-18/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_nineteenth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-19/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_twentieth(output: &Path) -> Result<(), String> {
    capture_from(
        output,
        "assets/source-art/animation-polish-01/monster-vertical-20/r01/manifest.json",
    )
    .await
}
pub(crate) async fn capture_remaining(output: &Path) -> Result<(), String> {
    if std::env::args().any(|arg| arg == "--bake-palettes") {
        return bake_palettes(output);
    }
    if std::env::args().any(|arg| arg == "--verify-installed") {
        return verify_installed(output).await;
    }
    let revision = if std::env::args().any(|arg| arg == "--corrections") {
        "assets/source-art/animation-polish-01/monster-vertical-all/r02/manifest.json"
    } else {
        "assets/source-art/animation-polish-01/monster-vertical-all/r01/manifest.json"
    };
    capture_review(
        output,
        revision,
        std::env::args().any(|arg| arg == "--resume-captures"),
    )
    .await
}
async fn capture_from(output: &Path, source: &str) -> Result<(), String> {
    capture_review(output, source, false).await
}
async fn capture_review(output: &Path, source: &str, resume: bool) -> Result<(), String> {
    std::fs::create_dir_all(output.join("inventory")).map_err(|e| e.to_string())?;
    let scope: serde_json::Value = serde_json::from_slice(
        &std::fs::read("docs/reviews/animation-polish-01/MONSTER-VERTICAL01-COVERAGE.json")
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let scene = Scene::new()?;
    let mut inventory = Vec::new();
    for entry in scope["entries"].as_array().ok_or("Missing inventory")? {
        let number = entry["number"].as_u64().unwrap() as u16;
        let file = format!("inventory/MON-{number:03}-0.png");
        inventory.push(serde_json::json!({"number":number,"id":entry["id"],"file":file,"variants":entry["variants"],"declared_rig":entry["declared_rig"],"declared_locomotor_legs":entry["declared_locomotor_legs"]}));
        if resume && output.join(&file).is_file() {
            continue;
        }
        let visual = Visual::CampaignMonster { number, variant: 0 };
        game_art::vertical::prepare(&[visual])?;
        game_art::biped::prepare(&[visual])?;
        let mut board = Image::gen_image_color(384, 192, Color::from_rgba(30, 40, 49, 255));
        for (col, pose) in [
            None,
            Some(Pose {
                facing: -2,
                frame: 0,
            }),
        ]
        .into_iter()
        .enumerate()
        {
            panel(&mut board, &current(&scene, visual, pose, 0.)?, col, false);
        }
        board.export_png(output.join(&file).to_str().unwrap());
        next_frame().await;
    }
    #[derive(Deserialize)]
    struct Manifest {
        review: String,
        profiles: Vec<Profile>,
    }
    let manifest: Manifest =
        serde_json::from_slice(&std::fs::read(source).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let mut items = Vec::new();
    for profile in manifest.profiles {
        let visual = Visual::CampaignMonster {
            number: profile.number,
            variant: 0,
        };
        game_art::vertical::prepare(&[visual])?;
        game_art::biped::prepare(&[visual])?;
        let image = crate::asset_preparation::image(&profile.atlas)?;
        if (image.width, image.height) != (1440, 256) {
            return Err("Invalid candidate dimensions".into());
        }
        let unique: [usize; 2] = std::array::from_fn(|row| {
            (0..8)
                .map(|phase| {
                    image
                        .sub_image(Rect::new(
                            phase as f32 * 160.,
                            row as f32 * 128.,
                            160.,
                            128.,
                        ))
                        .bytes
                })
                .collect::<BTreeSet<_>>()
                .len()
        });
        if unique != [8, 8] {
            return Err("Candidate has duplicate phases".into());
        }
        let previous_texture = if let Some(file) = &profile.previous_atlas {
            let mut previous = Image::from_file_with_format(
                &std::fs::read(file).map_err(|e| e.to_string())?,
                None,
            )
            .map_err(|e| e.to_string())?;
            if (previous.width, previous.height) != (1440, 256) {
                return Err("Invalid previous review atlas dimensions".into());
            }
            if let Some(file) = &profile.previous_material_palette {
                let palette: RedMaterialPalette =
                    serde_json::from_slice(&std::fs::read(file).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                palette.apply(&mut previous)?;
            }
            let t = crate::asset_loading::from_image(&previous);
            t.set_filter(FilterMode::Nearest);
            Some(t)
        } else if profile.comparison.as_deref() == Some("previous_review_colors") {
            let t = crate::asset_loading::from_image(&image);
            t.set_filter(FilterMode::Nearest);
            Some(t)
        } else {
            None
        };
        let mut prepared = (*image).clone();
        let palette_changed = if let Some(file) = &profile.material_palette {
            let palette: RedMaterialPalette =
                serde_json::from_slice(&std::fs::read(file).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            palette.apply(&mut prepared)?
        } else {
            0
        };
        if image
            .bytes
            .chunks_exact(4)
            .zip(prepared.bytes.chunks_exact(4))
            .any(|(a, b)| a[3] != b[3])
        {
            return Err("Material palette changed the figure's alpha".into());
        }
        let texture = crate::asset_loading::from_image(&prepared);
        texture.set_filter(FilterMode::Nearest);
        let path = output.join(&profile.id);
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        // The final image in each direction is written only after checking
        // pixel equality with the installed idle.
        let reuse = resume
            && profile.casts_shadow != Some(false)
            && ["down", "up"].into_iter().all(|direction| {
                (0..=32).all(|sample| path.join(format!("{direction}-{sample:02}.png")).is_file())
            });
        let mut captures = Vec::new();
        let mut fronts = 0;
        let mut routes = Vec::new();
        for (row, (direction, dy)) in [("down", 1), ("up", -1)].into_iter().enumerate() {
            let id: EntityId = serde_json::from_str("1").unwrap();
            let mut motion = CombatMotion::default();
            let mut selected = Vec::new();
            for sample in 0..=32 {
                if sample == 0 || sample == 16 {
                    let step = sample as i32 / 16;
                    motion.glide_timed(
                        id,
                        GridPos::new(0, step * dy),
                        GridPos::new(0, (step + 1) * dy),
                        1. + sample as f64 * 0.025,
                        400,
                    );
                }
                let pose = motion.vertical_walk_pose(id, 1. + sample as f64 * 0.025, false);
                if sample < 32 && pose.is_none_or(|p| p.facing != dy as i8 * 2 || p.frame > 5) {
                    return Err("Normal vertical heading lost".into());
                }
                if sample == 32 && pose.is_some() {
                    return Err("Final stop is not frontal".into());
                }
                selected.push(pose.map(|p| [p.facing, p.frame as i8]));
                let file = format!("{direction}-{sample:02}.png");
                captures.push(file.clone());
                if reuse {
                    fronts += usize::from(sample == 32);
                    continue;
                }
                let shift = dy as f32 * (-64. + sample as f32 * 4.);
                let reference = current(&scene, visual, None, shift)?;
                let phase = if sample == 32 { 8 } else { phase_at(sample) };
                let old = if let Some(previous) = &previous_texture {
                    candidate(&scene, &profile, previous, row, phase, shift)?
                } else {
                    current(&scene, visual, pose, shift)?
                };
                let new = candidate(&scene, &profile, &texture, row, phase, shift)?;
                if sample == 32 {
                    if reference.bytes != new.bytes || reference.bytes != old.bytes {
                        return Err(format!(
                            "Retained frontal mismatch {} {direction}",
                            profile.id
                        ));
                    }
                    fronts += 1;
                }
                let mut board = Image::gen_image_color(
                    (WIDTH * 3) as u16,
                    HEIGHT as u16,
                    Color::from_rgba(30, 40, 49, 255),
                );
                for (col, source) in [&reference, &old, &new].into_iter().enumerate() {
                    panel(&mut board, source, col, true);
                }
                board.export_png(path.join(&file).to_str().unwrap());
                next_frame().await;
            }
            routes.push(serde_json::json!({"direction":direction,"true_heading":dy*2,"normal_poses":selected}));
        }
        let mut item = serde_json::json!({"id":profile.id,"name":profile.name,"player":false,"captures":captures,"distinct_poses":unique,"identical_frontal_stops":fronts,"normal_routes":routes});
        if profile.material_palette.is_some() {
            item["palette_changed_pixels"] = serde_json::json!(palette_changed);
            item["geometry_and_alpha_unchanged"] = serde_json::json!(true);
        }
        if profile.previous_atlas.is_some() {
            if profile.comparison.as_deref() == Some("original_weapon_restored") {
                item["headers"] =
                    serde_json::json!(["Frontal conservé", "Arme absente", "Arme rétablie"]);
                item["comparison"] =
                    serde_json::json!("previous review / original weapon restored");
                item["previous_material_palette"] =
                    serde_json::json!(profile.previous_material_palette);
            } else if profile.comparison.as_deref() == Some("grouped_anatomy_revision") {
                item["headers"] =
                    serde_json::json!(["Frontal conservé", "Premier essai", "Correction"]);
                item["comparison"] = serde_json::json!("previous review / anatomy revision");
            } else {
                item["headers"] =
                    serde_json::json!(["Frontal conservé", "Bras précédents", "Bras relâchés"]);
                item["comparison"] = serde_json::json!("previous review / arm posture revision");
            }
            item["previous_atlas"] = serde_json::json!(profile.previous_atlas);
        } else if previous_texture.is_some() {
            item["headers"] = if profile.number == 60 {
                serde_json::json!([
                    "Frontal conservé",
                    "Couleurs précédentes",
                    "Couleurs harmonisées"
                ])
            } else {
                serde_json::json!(["Frontal conservé", "Marche précédente", "Rouge harmonisé"])
            };
            item["palette_changed_pixels"] = serde_json::json!(palette_changed);
            item["geometry_and_alpha_unchanged"] = serde_json::json!(true);
            item["comparison"] = serde_json::json!("same poses before and after material palette");
        }
        items.push(item);
    }
    let report = serde_json::json!({"review":manifest.review,"normal_game_enabled":false,"panel_width":WIDTH,"panel_height":HEIGHT,"items":items,"inventory":inventory,"models":scope["models"],"variants":scope["variants"],"step_ms":400,"cycle_ms":800,"native_tile_px":64,"travel_px":128,"user_data_loaded":false,"new_textures_prepared_before_draw":true,"monster_gain":1.48});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn source_image(atlas: &str, palette: Option<&str>) -> Result<Image, String> {
    let image = crate::asset_preparation::image(atlas)?;
    let mut image = (*image).clone();
    if let Some(file) = palette {
        let palette: RedMaterialPalette =
            serde_json::from_slice(&std::fs::read(file).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        palette.apply(&mut image)?;
    }
    Ok(image)
}

fn bake_palettes(output: &Path) -> Result<(), String> {
    let approved: serde_json::Value = serde_json::from_str(include_str!(
        "../docs/reviews/animation-polish-01/MONSTER-VERTICAL-ALL-ACCEPTANCE-r01.json"
    ))
    .map_err(|e| e.to_string())?;
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    for model in approved["accepted"]
        .as_array()
        .ok_or("Missing approved models")?
    {
        if let Some(palette) = model["material_palette"].as_str() {
            let id = model["id"].as_str().ok_or("Missing approved id")?;
            let atlas = model["atlas"].as_str().ok_or("Missing approved atlas")?;
            let image = source_image(atlas, Some(palette))?;
            std::fs::write(output.join(format!("{id}.rgba")), image.bytes)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

async fn verify_installed(output: &Path) -> Result<(), String> {
    let directory = "assets/animations/monster-vertical-01/manifest.json";
    let runtime: serde_json::Value =
        serde_json::from_slice(&project_rl::data_paths::read(directory)?)
            .map_err(|e| e.to_string())?;
    let sources: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            "docs/reviews/animation-polish-01/MONSTER-VERTICAL-INTEGRATION01-ASSETS.json",
        )
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let catalog = runtime["entries"]
        .as_array()
        .ok_or("Missing installed entries")?;
    let selected = sources["models"]
        .as_array()
        .ok_or("Missing installed provenance")?;
    if catalog.len() != 129 || selected.len() != catalog.len() {
        return Err("Wrong installed monster count".into());
    }
    let scene = Scene::new()?;
    let id: EntityId = serde_json::from_str("1").unwrap();
    let mut comparisons = 0;
    let mut warm_returns = 0;
    let mut controls = 0;
    for (entry, choice) in catalog.iter().zip(selected) {
        let n = entry["number"].as_u64().unwrap() as u16;
        let visual = Visual::CampaignMonster {
            number: n,
            variant: 0,
        };
        let path = choice["source"].as_str().unwrap();
        let original = source_image(path, choice["material_palette"].as_str())?;
        let installed = crate::asset_preparation::image(entry["file"].as_str().unwrap())?;
        let w = entry["cell"][0].as_u64().unwrap() as f32;
        let h = entry["cell"][1].as_u64().unwrap() as f32;
        let x = 80. - entry["anchor"][0].as_f64().unwrap() as f32;
        let anchor_y = if n == 9 { 122. } else { 112. };
        let y = anchor_y - entry["anchor"][1].as_f64().unwrap() as f32;
        for row in 0..2 {
            for phase in 0..8 {
                let expected = original.sub_image(Rect::new(
                    160. * phase as f32 + x,
                    128. * row as f32 + y,
                    w,
                    h,
                ));
                let actual = installed.sub_image(Rect::new(w * phase as f32, h * row as f32, w, h));
                if expected.bytes != actual.bytes {
                    return Err(format!("Installed pose pixels differ: {n} {row} {phase}"));
                }
            }
        }
        game_art::vertical::prepare(&[visual])?;
        game_art::biped::prepare(&[visual])?;
        let before = crate::asset_preparation::stats();
        let reference = crate::asset_loading::from_image(&original);
        reference.set_filter(FilterMode::Nearest);
        let profile = Profile {
            id: format!("MON-{n:03}-0"),
            number: n,
            name: String::new(),
            atlas: path.into(),
            vertical_offset: entry["vertical_offset"].as_i64().unwrap() as i16,
            atlas_anchor_y: Some(anchor_y as u16),
            front_shadow: 0,
            casts_shadow: entry["casts_shadow"].as_bool(),
            material_palette: None,
            comparison: None,
            previous_atlas: None,
            previous_material_palette: None,
        };
        let frontal = render_world(&scene, visual, None, 0.)?;
        for (row, dy) in [1, -1].into_iter().enumerate() {
            let mut motion = CombatMotion::default();
            for sample in 0..=32 {
                if sample == 0 || sample == 16 {
                    let step = sample / 16;
                    motion.glide_timed(
                        id,
                        GridPos::new(0, step * dy),
                        GridPos::new(0, (step + 1) * dy),
                        1. + sample as f64 * 0.025,
                        400,
                    );
                }
                let now = 1. + sample as f64 * 0.025;
                let pose = walking::world_pose(&motion, id, Some(visual), false, now, false);
                let expected = (sample < 32).then(|| Pose {
                    facing: (dy * 2) as i8,
                    frame: phase_at(sample as usize) as u8,
                });
                if pose != expected
                    || walking::world_pose(&motion, id, Some(visual), false, now, true).is_some()
                {
                    return Err(format!(
                        "Wrong installed heading, phase or reduced motion: {n} {row} {sample} {pose:?} {expected:?}"
                    ));
                }
                if sample == 32 {
                    if render_world(&scene, visual, pose, 0.)?.bytes != frontal.bytes {
                        return Err(format!("Wrong frontal at arrival: {n} {row}"));
                    }
                } else if sample == 9 {
                    let normal = render_world(&scene, visual, pose, 0.)?;
                    let expected = candidate(
                        &scene,
                        &profile,
                        &reference,
                        row,
                        phase_at(sample as usize),
                        0.,
                    )?;
                    if normal.bytes != expected.bytes {
                        return Err(format!(
                            "Native normal draw differs from approved art: {n} {row}"
                        ));
                    }
                    comparisons += 1;
                }
                controls += 1;
            }
        }
        let after = crate::asset_preparation::stats();
        for field in ["jobs", "uploads", "source_reads"] {
            if after[field] != before[field] {
                return Err(format!("Draw prepared {field} for {n}"));
            }
        }
        if !game_art::vertical::prepare(&[visual])?.is_zero() {
            return Err(format!("Warm return stalled for {n}"));
        }
        warm_returns += 1;
        next_frame().await;
    }
    let report = serde_json::json!({
        "review": "MONSTER-VERTICAL-INTEGRATION01", "models": catalog.len(), "native_tile_px": 64,
        "step_ms": 400, "cycle_ms": 800, "pose_pixels_verified": catalog.len() * 16,
        "native_comparisons": comparisons, "exact_stops": catalog.len() * 2, "routing_checks": controls,
        "warm_returns": warm_returns, "draw_time_reads_or_uploads": 0, "normal_game_enabled": true,
        "resource_cache": crate::asset_preparation::stats(),
    });
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn organic_walk_palettes_match_each_view_and_preserve_rest_and_alpha() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for number in 63..=74 {
            let batch = 11 + (number - 63) / 3;
            let classify: fn(&[u8]) -> usize = if number >= 69 {
                arthropod_material
            } else {
                rust_material
            };
            let materials = if number >= 69 { 3 } else { 2 };
            let folder = root.join(format!(
                "assets/source-art/animation-polish-01/monster-vertical-{batch}/r01"
            ));
            let data: serde_json::Value = serde_json::from_slice(
                &std::fs::read(folder.join(format!("MON-{number:03}-material-palette.json")))
                    .unwrap(),
            )
            .unwrap();
            let palette: RedMaterialPalette = serde_json::from_value(data.clone()).unwrap();
            let load = |file: &Path| {
                Image::from_file_with_format(&std::fs::read(file).unwrap(), None).unwrap()
            };
            let original = load(&folder.join(format!("MON-{number:03}-atlas.png")));
            let mut corrected = original.clone();
            assert!(palette.apply(&mut corrected).unwrap() > 1000);
            for (i, (a, b)) in original
                .bytes
                .chunks_exact(4)
                .zip(corrected.bytes.chunks_exact(4))
                .enumerate()
            {
                assert_eq!(a[3], b[3]);
                if i % 1440 >= 1280 || !palette.material(a) {
                    assert_eq!(a, b);
                }
            }
            for row in 0..2 {
                let reference = load(&root.join(data["references"][row]["file"].as_str().unwrap()));
                for kind in 1..=materials {
                    let actual: Vec<_> = original
                        .bytes
                        .chunks_exact(4)
                        .zip(corrected.bytes.chunks_exact(4))
                        .enumerate()
                        .filter(|(i, (c, _))| {
                            i % 1440 < 1280
                                && i / (1440 * 128) == row
                                && c[3] >= 224
                                && classify(c) == kind
                        })
                        .map(|(_, (_, c))| [c[0], c[1], c[2]])
                        .collect();
                    let target: Vec<_> = reference
                        .bytes
                        .chunks_exact(4)
                        .filter(|c| c[3] >= 224 && classify(c) == kind)
                        .map(|c| [c[0], c[1], c[2]])
                        .collect();
                    assert!(!actual.is_empty() && !target.is_empty());
                    for channel in 0..3 {
                        let mut a: Vec<_> = actual.iter().map(|c| c[channel]).collect();
                        a.sort_unstable();
                        let mut b: Vec<_> = target.iter().map(|c| c[channel]).collect();
                        b.sort_unstable();
                        for q in [0.1f32, 0.5, 0.9] {
                            assert!(
                                a[(q * (a.len() - 1) as f32).round() as usize]
                                    .abs_diff(b[(q * (b.len() - 1) as f32).round() as usize])
                                    <= 2,
                                "model={number} row={row} material={kind} channel={channel} q={q}"
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn armed_rust_palette_keeps_steel_pose_and_idle_pixels_exact() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let folder =
            root.join("assets/source-art/animation-polish-01/monster-vertical-28-weapon/r01");
        let palette: RedMaterialPalette = serde_json::from_slice(
            &std::fs::read(folder.join("MON-060-material-palette.json")).unwrap(),
        )
        .unwrap();
        let load = |file: &str| {
            Image::from_file_with_format(&std::fs::read(folder.join(file)).unwrap(), None).unwrap()
        };
        let original = load("MON-060-atlas.png");
        let reference = load("MON-060-front.png");
        let mut corrected = original.clone();
        assert!(palette.apply(&mut corrected).unwrap() > 3000);
        let mut steel_pixels = 0;
        for (i, (a, b)) in original
            .bytes
            .chunks_exact(4)
            .zip(corrected.bytes.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(a[3], b[3]);
            if i % 1440 >= 1280 || !palette.material(a) {
                assert_eq!(a, b);
            }
            if i % 1440 < 1280 && a[3] >= 224 && review_steel_material(a) {
                steel_pixels += 1;
                assert_eq!(a, b);
            }
        }
        assert!(steel_pixels > 100);
        for row in 0..2 {
            for kind in [1, 2] {
                let actual: Vec<_> = original
                    .bytes
                    .chunks_exact(4)
                    .zip(corrected.bytes.chunks_exact(4))
                    .enumerate()
                    .filter(|(i, (c, _))| {
                        i % 1440 < 1280
                            && i / (1440 * 128) == row
                            && c[3] >= 224
                            && palette.material(c)
                            && rust_material(c) == kind
                    })
                    .map(|(_, (_, c))| [c[0], c[1], c[2]])
                    .collect();
                let target: Vec<_> = reference
                    .bytes
                    .chunks_exact(4)
                    .filter(|c| c[3] >= 224 && palette.material(c) && rust_material(c) == kind)
                    .map(|c| [c[0], c[1], c[2]])
                    .collect();
                assert!(!actual.is_empty() && !target.is_empty());
                for channel in 0..3 {
                    let mut a: Vec<_> = actual.iter().map(|c| c[channel]).collect();
                    a.sort_unstable();
                    let mut b: Vec<_> = target.iter().map(|c| c[channel]).collect();
                    b.sort_unstable();
                    for q in [0.1f32, 0.5, 0.9] {
                        assert!(
                            a[(q * (a.len() - 1) as f32).round() as usize]
                                .abs_diff(b[(q * (b.len() - 1) as f32).round() as usize])
                                <= 2,
                            "row={row} material={kind} channel={channel} q={q}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn rust_walk_matches_idle_colors_without_changing_poses_outlines_or_rest() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let palette: RedMaterialPalette = serde_json::from_slice(
            &std::fs::read(root.join("assets/source-art/animation-polish-01/monster-vertical-28-color/r01/MON-060-material-palette.json")).unwrap(),
        ).unwrap();
        let load = |name: &str| {
            Image::from_file_with_format(&std::fs::read(root.join(name)).unwrap(), None).unwrap()
        };
        let original = load(
            "assets/source-art/animation-polish-01/monster-vertical-28-arms/r01/MON-060-atlas.png",
        );
        let reference = load(
            "assets/source-art/animation-polish-01/monster-vertical-28-arms/r01/MON-060-front.png",
        );
        let mut corrected = original.clone();
        assert!(palette.apply(&mut corrected).unwrap() > 4_000);
        for (i, (before, after)) in original
            .bytes
            .chunks_exact(4)
            .zip(corrected.bytes.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(before[3], after[3]);
            if i % 1440 >= 1280 || rust_material(before) == 0 {
                assert_eq!(before, after);
            }
        }
        for row in 0..2 {
            for material in [1, 2] {
                let actual: Vec<_> = original
                    .bytes
                    .chunks_exact(4)
                    .zip(corrected.bytes.chunks_exact(4))
                    .enumerate()
                    .filter(|(i, (c, _))| {
                        i % 1440 < 1280
                            && i / (1440 * 128) == row
                            && c[3] >= 224
                            && rust_material(c) == material
                    })
                    .map(|(_, (_, c))| [c[0], c[1], c[2]])
                    .collect();
                let target: Vec<_> = reference
                    .bytes
                    .chunks_exact(4)
                    .filter(|c| c[3] >= 224 && rust_material(c) == material)
                    .map(|c| [c[0], c[1], c[2]])
                    .collect();
                assert!(!actual.is_empty() && !target.is_empty());
                for channel in 0..3 {
                    let mut a: Vec<_> = actual.iter().map(|c| c[channel]).collect();
                    a.sort_unstable();
                    let mut b: Vec<_> = target.iter().map(|c| c[channel]).collect();
                    b.sort_unstable();
                    for q in [0.1f32, 0.5, 0.9] {
                        assert!(
                            a[(q * (a.len() - 1) as f32).round() as usize]
                                .abs_diff(b[(q * (b.len() - 1) as f32).round() as usize])
                                <= 2,
                            "row={row} material={material} channel={channel} q={q}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn real_demon_palette_matches_retained_red_without_changing_shapes_idle_or_other_materials() {
        verify_palette(
            "assets/source-art/animation-polish-01/monster-vertical-color-01/r01/red-material-palette.json",
            "assets/source-art/animation-polish-01/monster-vertical-01/r01/MON-144-atlas.png",
            "assets/source-art/animation-polish-01/monster-vertical-01/r01/MON-144-front.png",
            30_000,
        );
    }
    #[test]
    fn ivory_palette_matches_retained_skin_without_changing_wings_bracer_shapes_or_idle() {
        verify_palette(
            "assets/source-art/animation-polish-01/monster-vertical-03/r01/MON-151-ivory-palette.json",
            "assets/source-art/animation-polish-01/monster-vertical-03/r01/MON-151-atlas.png",
            "assets/source-art/animation-polish-01/monster-vertical-03/r01/MON-151-front.png",
            5_000,
        );
    }
    #[test]
    fn possessed_palette_matches_retained_skin_without_changing_crouch_shapes_or_idle() {
        verify_palette(
            "assets/source-art/animation-polish-01/monster-vertical-04/r01/MON-006-ivory-palette.json",
            "assets/source-art/animation-polish-01/monster-vertical-04/r01/MON-006-atlas.png",
            "assets/source-art/animation-polish-01/monster-vertical-04/r01/MON-006-front.png",
            2_000,
        );
    }
    #[test]
    fn ragged_possessed_palette_matches_skin_without_changing_shapes_or_idle() {
        verify_palette(
            "assets/source-art/animation-polish-01/monster-vertical-06/r01/MON-025-ivory-palette.json",
            "assets/source-art/animation-polish-01/monster-vertical-06/r01/MON-025-atlas.png",
            "assets/source-art/animation-polish-01/monster-vertical-06/r01/MON-025-front.png",
            2_000,
        );
    }
    #[test]
    fn low_quadruped_palette_matches_skin_without_changing_shapes_or_idle() {
        verify_palette(
            "assets/source-art/animation-polish-01/monster-vertical-09/r01/MON-035-ivory-palette.json",
            "assets/source-art/animation-polish-01/monster-vertical-09/r01/MON-035-atlas.png",
            "assets/source-art/animation-polish-01/monster-vertical-09/r01/MON-035-front.png",
            1_000,
        );
    }
    #[test]
    fn blue_front_materials_match_idle_without_changing_rear_shapes_or_other_materials() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let folder = root.join("assets/source-art/animation-polish-01/monster-vertical-08/r01");
        let palette: RedMaterialPalette = serde_json::from_slice(
            &std::fs::read(folder.join("MON-036-front-material-palette.json")).unwrap(),
        )
        .unwrap();
        let load = |name: &str| {
            Image::from_file_with_format(&std::fs::read(folder.join(name)).unwrap(), None).unwrap()
        };
        let original = load("MON-036-atlas.png");
        let reference = load("MON-036-front.png");
        let mut prepared = original.clone();
        assert!(palette.apply(&mut prepared).unwrap() > 600);
        for (i, (before, after)) in original
            .bytes
            .chunks_exact(4)
            .zip(prepared.bytes.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(before[3], after[3]);
            if i % 1440 >= 1280 || i / 1440 >= 128 || blue_front_material(before) == 0 {
                assert_eq!(before, after);
            }
        }
        for material in [1, 2] {
            let actual: Vec<_> = original
                .bytes
                .chunks_exact(4)
                .zip(prepared.bytes.chunks_exact(4))
                .enumerate()
                .filter(|(i, (c, _))| {
                    i % 1440 < 1280
                        && i / 1440 < 128
                        && c[3] >= 224
                        && blue_front_material(c) == material
                })
                .map(|(_, (_, c))| [c[0], c[1], c[2]])
                .collect();
            let target: Vec<_> = reference
                .bytes
                .chunks_exact(4)
                .filter(|c| c[3] >= 224 && blue_front_material(c) == material)
                .map(|c| [c[0], c[1], c[2]])
                .collect();
            assert!(!actual.is_empty() && !target.is_empty());
            for channel in 0..3 {
                let mut a: Vec<_> = actual.iter().map(|c| c[channel]).collect();
                a.sort_unstable();
                let mut b: Vec<_> = target.iter().map(|c| c[channel]).collect();
                b.sort_unstable();
                assert!(
                    a[a.len() / 2].abs_diff(b[b.len() / 2]) <= 2,
                    "material={material} channel={channel}"
                );
            }
        }
    }
    fn verify_palette(rule: &str, atlas: &str, front: &str, minimum_changed: usize) {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        let path = root.join(rule);
        let palette: RedMaterialPalette =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        let load = |name: &str| {
            Image::from_file_with_format(&std::fs::read(root.join(name)).unwrap(), None).unwrap()
        };
        let original = load(atlas);
        let reference = load(front);
        let mut corrected = original.clone();
        assert!(palette.apply(&mut corrected).unwrap() > minimum_changed);
        let mut actual = Vec::new();
        for (i, (before, after)) in original
            .bytes
            .chunks_exact(4)
            .zip(corrected.bytes.chunks_exact(4))
            .enumerate()
        {
            assert_eq!(before[3], after[3]);
            if i % 1440 >= 1280 || !palette.material(before) {
                assert_eq!(before, after);
            } else if before[3] >= 224 {
                actual.push([after[0], after[1], after[2]]);
            }
        }
        let target: Vec<_> = reference
            .bytes
            .chunks_exact(4)
            .filter(|c| c[3] >= 224 && palette.material(c))
            .map(|c| [c[0], c[1], c[2]])
            .collect();
        for channel in 0..3 {
            let mut a: Vec<_> = actual.iter().map(|c| c[channel]).collect();
            a.sort_unstable();
            let mut b: Vec<_> = target.iter().map(|c| c[channel]).collect();
            b.sort_unstable();
            for q in [0.1f32, 0.5, 0.9] {
                let x = a[(q * (a.len() - 1) as f32).round() as usize];
                let y = b[(q * (b.len() - 1) as f32).round() as usize];
                assert!(
                    x.abs_diff(y) <= 2,
                    "Material mismatch channel={channel} q={q}: {x}/{y}"
                );
            }
        }
    }
}
