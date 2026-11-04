//! The actual approved player cycle and its creature adaptation share phase and root travel.
use super::*;

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let text = project_rl::data_paths::read_to_string(
        "assets/source-art/animation-polish-01/biped-lateral-01/r03/manifest.json",
    )
    .map_err(|e| e.to_string())?;
    let metadata: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let profiles: Vec<Profile> =
        serde_json::from_value(metadata["profiles"].clone()).map_err(|e| e.to_string())?;
    if profiles.len() != 1 || profiles[0].number != 144 {
        return Err("Revue limitee au demon et au cycle joueur".into());
    }
    let p = &profiles[0];
    let candidate = file(&p.atlas)?;
    let distinct = articulation_checks(&candidate)?;
    let player =
        file("assets/source-art/animation-polish-01/class-lateral-02/r02/bastion-atlas.png")?;
    if (player.width, player.height) != (864, 192) {
        return Err("Reference joueur incorrecte".into());
    }
    let player_texture = upload(&player);
    let new = upload(&candidate);
    let previous = upload(&file(
        "assets/source-art/animation-polish-01/biped-lateral-01/r02/MON-144-atlas.png",
    )?);
    let player_visual = Visual::Player {
        body: 3,
        equipment: [Some(101), None, None, None],
        weapon: None,
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    };
    let monster = Visual::CampaignMonster {
        number: 144,
        variant: 0,
    };
    game_art::vertical::prepare(&[player_visual, monster])?;
    let scene = Scene::new()?;
    let dir = output.join("MON-144-0");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut captures = Vec::new();
    let mut sampled = Vec::new();
    for (side, facing) in [-1, 1].into_iter().enumerate() {
        for sample in 0..=SAMPLES {
            let phase = phase_at(sample as f32 * 1300. / SAMPLES as f32);
            // Exact same presentation timeline and root path as CLASS-LATERAL02-r02.
            let shift = -(facing as f32) * (20. - 40. * sample as f32 / SAMPLES as f32);
            let mut board = Image::gen_image_color(480, 192, Color::from_rgba(30, 40, 49, 255));
            if sample == SAMPLES {
                panel(&mut board, &static_render(&scene, player_visual, shift), 0);
                let rest = static_render(&scene, monster, shift);
                panel(&mut board, &rest, 1);
                panel(&mut board, &rest, 2);
            } else {
                let reference = sprite_render(
                    &scene,
                    &player_texture,
                    Rect::new(phase as f32 * 96., side as f32 * 96., 96., 96.),
                    [48., 81.],
                    0,
                    shift,
                    true,
                )?;
                panel(&mut board, &reference, 0);
                for (column, texture) in [(1, &previous), (2, &new)] {
                    let image = sprite_render(
                        &scene,
                        texture,
                        Rect::new(phase as f32 * 128., side as f32 * 128., 128., 128.),
                        [64., 112.],
                        p.vertical_offset,
                        shift,
                        false,
                    )?;
                    panel(&mut board, &image, column);
                }
                sampled.push(serde_json::json!({"side":side,"sample":sample,"player_phase":phase,"monster_phase":phase,"root_shift":shift}));
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
    let report = serde_json::json!({"review":"BIPED-LATERAL01-r03","status":"native_candidate_awaiting_visual_approval","normal_game_enabled":false,
        "native_cell_px":64,"step_ms":650,"cycle_ms":1300,"phase_durations_ms":PHASE_MS,"samples":32,"travel_px":40,"rest_ms":650,
        "gait_source":metadata["gait_source"],"reference_actor":"Bastion, heavy armor, unarmed, exact r02 drawings",
        "reference_lighting":"normal player Discret, normal monster exposure, same camp light",
        "items":[{"id":"MON-144-0","name":"Démon ailé · référence joueur","player":false,"captures":captures,"poses_each_direction":distinct,"native_board":[480,192]}],
        "columns":["Bastion r02 reference","old demon r02","demon adapted from player poses r03"],"synchronized_phase_checks":sampled,
        "other_creatures":"await this retargeted cycle review","weapon_study":"remains unvalidated, no integration","vertical_unchanged":true});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
