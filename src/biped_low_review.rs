//! One rejected morphology is revised before any further batch extension.
use super::*;

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let text = project_rl::data_paths::read_to_string(
        "assets/source-art/animation-polish-01/biped-lateral-01/r02/manifest.json",
    )
    .map_err(|e| e.to_string())?;
    let metadata: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let profiles: Vec<Profile> =
        serde_json::from_value(metadata["profiles"].clone()).map_err(|e| e.to_string())?;
    if profiles.len() != 1 || profiles[0].number != 144 {
        return Err("Reprise limitee au demon aile".into());
    }
    let p = &profiles[0];
    let source = file(&p.atlas)?;
    let distinct = articulation_checks(&source)?;
    let previous =
        file("assets/source-art/animation-polish-01/biped-lateral-01/r01/MON-144-atlas.png")?;
    let new = upload(&source);
    let old = upload(&previous);
    let visual = Visual::CampaignMonster {
        number: 144,
        variant: 0,
    };
    game_art::vertical::prepare(&[visual])?;
    let scene = Scene::new()?;
    let frontal = static_render(&scene, visual, 0.);
    let dir = output.join("MON-144-0");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let mut captures = Vec::new();
    for (side, facing) in [-1, 1].into_iter().enumerate() {
        for sample in 0..=SAMPLES {
            let phase = phase_at(sample as f32 * 1300. / SAMPLES as f32);
            // Short root travel matches the low, compact stride in this study.
            let shift = -(facing as f32) * (6. - 12. * sample as f32 / SAMPLES as f32);
            let mut board = Image::gen_image_color(480, 192, Color::from_rgba(30, 40, 49, 255));
            panel(&mut board, &frontal, 0);
            if sample == SAMPLES {
                let rest = static_render(&scene, visual, shift);
                panel(&mut board, &rest, 1);
                panel(&mut board, &rest, 2);
            } else {
                for (column, texture) in [(1, &old), (2, &new)] {
                    let pose = sprite_render(
                        &scene,
                        texture,
                        Rect::new(phase as f32 * 128., side as f32 * 128., 128., 128.),
                        [64., 112.],
                        p.vertical_offset,
                        shift,
                        false,
                    )?;
                    panel(&mut board, &pose, column);
                }
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
    let report = serde_json::json!({"review":"BIPED-LATERAL01-r02","status":"native_candidate_awaiting_visual_approval",
        "normal_game_enabled":false,"native_cell_px":64,"step_ms":650,"phase_durations_ms":PHASE_MS,"samples":32,"travel_px":12,"rest_ms":650,
        "items":[{"id":"MON-144-0","name":"Demon aile - pas rasants","player":false,"captures":captures,"poses_each_direction":distinct,"native_board":[480,192]}],
        "previous_column":"rejected BIPED-LATERAL01-r01, same cadence and short root travel","whole_figure_upper_alignment":true,
        "other_creatures":"rejected r01, deferred until this gait review","weapon_study":"unvalidated r01, not integrated","vertical_unchanged":true});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
