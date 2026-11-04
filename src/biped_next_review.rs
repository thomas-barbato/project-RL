//! New bodies share the accepted cycle and 400ms presentation cadence.
use super::*;

#[derive(Deserialize)]
struct NextProfile {
    #[serde(flatten)]
    info: Profile,
    #[serde(default)]
    accepted: bool,
    #[serde(default)]
    compare_atlas: Option<String>,
    #[serde(default)]
    idle_sprite: Option<String>,
    #[serde(default)]
    retained_front_review: bool,
    #[serde(default)]
    front_accepted: bool,
    #[serde(default = "default_frame_width")]
    frame_width: u16,
    #[serde(default)]
    review_headers: Option<[String; 3]>,
}

fn default_frame_width() -> u16 {
    128
}

pub(crate) async fn capture_winged(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-04/r01/manifest.json",
        "BIPED-LATERAL04-r01",
        &[39, 146, 151, 144],
        &[39, 146, 151],
    )
    .await
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-02/r01/manifest.json",
        "BIPED-LATERAL02-r01",
        &[5, 14, 156, 144],
        &[5, 14, 156],
    )
    .await
}

pub(crate) async fn capture_armed(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-03/r01/manifest.json",
        "BIPED-LATERAL03-r01",
        &[145, 164, 165, 5, 14, 156, 144],
        &[145, 164, 165],
    )
    .await
}

pub(crate) async fn capture_hoods(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-03/r02/manifest.json",
        "BIPED-LATERAL03-r02",
        &[145, 164, 165, 5, 14, 156, 144],
        &[164, 165],
    )
    .await
}

pub(crate) async fn capture_fronts(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-03/r03/manifest.json",
        "BIPED-LATERAL03-r03",
        &[145, 164, 165, 5, 14, 156, 144],
        &[164, 165, 14],
    )
    .await
}

pub(crate) async fn capture_frontmatch(output: &Path) -> Result<(), String> {
    capture_profiles(
        output,
        "assets/source-art/animation-polish-01/biped-lateral-03/r04/manifest.json",
        "BIPED-LATERAL03-r04",
        &[145, 164, 165, 5, 14, 156, 144],
        &[164, 165],
    )
    .await
}

async fn capture_profiles(
    output: &Path,
    manifest: &str,
    review: &str,
    expected: &[u16],
    new_models: &[u16],
) -> Result<(), String> {
    let text = project_rl::data_paths::read_to_string(manifest).map_err(|e| e.to_string())?;
    let metadata: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let panel_width = metadata["panel_width"].as_u64().unwrap_or(160) as usize;
    let pixel_snap_candidates = metadata["pixel_snap_candidates"].as_bool().unwrap_or(false);
    let match_native_shadows = metadata["match_native_shadows"].as_bool().unwrap_or(false);
    if ![160, 192].contains(&panel_width) {
        return Err("Largeur de revue incorrecte".into());
    }
    let panel = |board: &mut Image, image: &Image, col: usize| {
        for y in 0..160 {
            let source = ((y + 278) * 640 + 304 - panel_width / 2) * 4;
            let dest = (y * panel_width * 3 + col * panel_width) * 4;
            board.bytes[dest..dest + panel_width * 4]
                .copy_from_slice(&image.bytes[source..source + panel_width * 4]);
        }
    };
    let profiles: Vec<NextProfile> =
        serde_json::from_value(metadata["profiles"].clone()).map_err(|e| e.to_string())?;
    if profiles.iter().map(|p| p.info.number).collect::<Vec<_>>() != expected {
        return Err("Perimetre du lot a reverifier".into());
    }
    let player = upload(&file(
        "assets/source-art/animation-polish-01/class-lateral-02/r02/bastion-atlas.png",
    )?);
    let player_visual = Visual::Player {
        body: 3,
        equipment: [Some(101), None, None, None],
        weapon: None,
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    };
    game_art::vertical::prepare(&[player_visual])?;
    let scene = Scene::new()?;
    let mut items = Vec::new();
    let mut samples = Vec::new();
    for profile in profiles {
        let p = profile.info;
        let frame_width = f32::from(profile.frame_width);
        let art = project_rl::monster_art::art(p.number, 0)
            .ok_or("Monstre absent du catalogue actuel")?;
        let shadow_width = if match_native_shadows && new_models.contains(&p.number) {
            art.shadow.min(255) as u8
        } else {
            17
        };
        let image = file(&p.atlas)?;
        if image.width != profile.frame_width * 9 {
            return Err("Atlas et largeur de pose incoherents".into());
        }
        let distinct = articulation_checks(&image)?;
        let candidate = upload(&image);
        let idle = profile
            .idle_sprite
            .as_deref()
            .map(|path| file(path).map(|image| upload(&image)))
            .transpose()?;
        let previous = profile
            .compare_atlas
            .as_deref()
            .map(|path| file(path).map(|image| upload(&image)))
            .transpose()?;
        let visual = Visual::CampaignMonster {
            number: p.number,
            variant: 0,
        };
        game_art::vertical::prepare(&[visual])?;
        let dir = output.join(&p.id);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let mut captures = Vec::new();
        for (side, facing) in [-1, 1].into_iter().enumerate() {
            for sample in 0..=SAMPLES {
                // Preserve the approved relative phase weights. Playback runs
                // this two-step timeline in 800ms (400ms per step).
                let phase = phase_at(sample as f32 * 1300. / SAMPLES as f32);
                let shift = -(facing as f32) * (20. - 40. * sample as f32 / SAMPLES as f32);
                // Equal native pixels must sample alike across differently padded
                // textures. Half-pixel UV ties otherwise differ by atlas dimensions.
                let shift = if pixel_snap_candidates && new_models.contains(&p.number) {
                    shift.round()
                } else {
                    shift
                };
                let mut board = Image::gen_image_color(
                    (panel_width * 3) as u16,
                    192,
                    Color::from_rgba(30, 40, 49, 255),
                );
                let front = static_render(&scene, visual, shift);
                panel(&mut board, &front, 1);
                let new_front = if let Some(texture) = &idle {
                    let image = sprite_render_shadow(
                        &scene,
                        texture,
                        Rect::new(0., 0., frame_width, 128.),
                        [frame_width / 2., 112.],
                        p.vertical_offset,
                        shift,
                        false,
                        shadow_width,
                    )?;
                    panel(
                        &mut board,
                        if profile.retained_front_review {
                            &image
                        } else {
                            &front
                        },
                        0,
                    );
                    panel(&mut board, &image, 1);
                    Some(image)
                } else {
                    None
                };
                if sample == SAMPLES {
                    if let Some(image) = &new_front {
                        panel(&mut board, image, 2);
                    } else {
                        panel(&mut board, &static_render(&scene, player_visual, shift), 0);
                        panel(&mut board, &front, 2);
                    }
                } else {
                    let reference = sprite_render(
                        &scene,
                        &player,
                        Rect::new(phase as f32 * 96., side as f32 * 96., 96., 96.),
                        [48., 81.],
                        0,
                        shift,
                        true,
                    )?;
                    let next = sprite_render_shadow(
                        &scene,
                        &candidate,
                        Rect::new(
                            phase as f32 * frame_width,
                            side as f32 * 128.,
                            frame_width,
                            128.,
                        ),
                        [frame_width / 2., 112.],
                        p.vertical_offset,
                        shift,
                        false,
                        shadow_width,
                    )?;
                    if let Some(texture) = &previous {
                        let before = sprite_render_shadow(
                            &scene,
                            texture,
                            Rect::new(phase as f32 * 128., side as f32 * 128., 128., 128.),
                            [64., 112.],
                            p.vertical_offset,
                            shift,
                            false,
                            shadow_width,
                        )?;
                        panel(&mut board, &before, 1);
                    }
                    if idle.is_none() {
                        panel(&mut board, &reference, 0);
                    }
                    panel(&mut board, &next, 2);
                    samples.push(serde_json::json!({"id":p.id,"side":side,"sample":sample,"player_phase":phase,"monster_phase":phase}));
                }
                let name = format!(
                    "{}-{sample:02}.png",
                    if side == 0 { "left" } else { "right" }
                );
                board.export_png(dir.join(&name).to_str().unwrap());
                captures.push(name);
            }
            next_frame().await;
        }
        let defaults = if profile.retained_front_review {
            ["Frontal retenu", "Marche precedente", "Marche reprise r04"]
        } else if idle.is_some() {
            ["Frontal precedent", "Frontal harmonise", "Marche conservee"]
        } else if previous.is_some() {
            ["Joueur reference", "Avant r01", "Tete orientee r02"]
        } else {
            ["Joueur reference", "Frontal actuel", "Cycle joueur"]
        };
        let headers = profile
            .review_headers
            .unwrap_or_else(|| defaults.map(String::from));
        items.push(serde_json::json!({"id":p.id,"name":p.name,"player":false,"accepted":profile.accepted,"previous_walk":previous.is_some(),"identity_front":idle.is_some(),"retained_front_review":profile.retained_front_review,"front_accepted":profile.front_accepted,"headers":headers,"captures":captures,"poses_each_direction":distinct,"frame_width":profile.frame_width,"native_board":[panel_width*3,192]}));
    }
    let report = serde_json::json!({"review":review,"status":"new_candidates_and_accepted_references","normal_game_enabled":false,
        "native_cell_px":64,"panel_width":panel_width,"pixel_snap_candidates":pixel_snap_candidates,"match_native_shadows":match_native_shadows,"step_ms":400,"cycle_ms":800,"phase_weights":PHASE_MS,"samples":32,"travel_px":40,"rest_ms":400,
        "items":items,"synchronized_phase_checks":samples,"gait_reference":"Bastion CLASS-LATERAL02-r02 plus accepted MON-144 r03",
        "accepted_model":144,"new_models":new_models,"weapon_contacts_accepted":false,"vertical_unchanged":true});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
