//! SPELL-OBST01: exercise the production clipping shader at native tile size.
use macroquad::prelude::*;
use project_rl::world::{GridPos, Map, Terrain};

pub async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(576., 448.);
    for _ in 0..3 {
        next_frame().await;
    }
    if (screen_width(), screen_height()) != (576., 448.) {
        return Err("Native 576x448 surface unavailable".into());
    }
    let origin = GridPos::new(4, 3);
    let obstacle = GridPos::new(5, 3);
    let shadow = GridPos::new(6, 3);
    let hidden = GridPos::new(3, 2);
    let rect = |at: GridPos| {
        (at != hidden).then(|| Rect::new(at.x as f32 * 64., at.y as f32 * 64., 64., 64.))
    };
    let tile = rect(origin).unwrap();
    let texture = Texture2D::from_image(&Image::gen_image_color(1, 1, WHITE));
    let mut motion = crate::combat_visuals::CombatMotion::default();
    motion.play_blast(origin, 2, 10.);
    let now = 10. + 0.24 * crate::animation_policy::duration_scale("A02");
    let mut report = Vec::new();
    for reduced in [false, true] {
        let mut reference = None;
        for kind in [
            "floor",
            "water",
            "low-decor",
            "wall",
            "closed-door",
            "opaque-decor",
        ] {
            let mut map = Map::filled(9, 7, Terrain::Floor).map_err(|e| e.to_string())?;
            match kind {
                "water" => map.set_terrain(obstacle, Terrain::DeepWater),
                "low-decor" => map.set_obstruction(obstacle, true, false),
                "wall" => map.set_terrain(obstacle, Terrain::Wall),
                "closed-door" => map.set_terrain(
                    obstacle,
                    Terrain::Door(project_rl::world::DoorState::Closed),
                ),
                "opaque-decor" => map.set_obstruction(obstacle, true, true),
                _ => Ok(()),
            }
            .map_err(|e| e.to_string())?;
            set_default_camera();
            clear_background(BLACK);
            crate::natural_effect::draw(
                &texture,
                Rect::new(0., 0., 1., 1.),
                Rect::new(0., 0., 576., 448.),
                0.,
                1.,
                false,
                origin,
                tile,
                &map,
                &rect,
            );
            let image = crate::ui_capture::framebuffer()?;
            let lit = |at: GridPos| {
                let x = (at.x * 64 + 32) as usize;
                let y = (at.y * 64 + 32) as usize;
                let offset =
                    ((usize::from(image.height) - 1 - y) * usize::from(image.width) + x) * 4;
                image.bytes[offset..offset + 3].iter().any(|v| *v > 3)
            };
            let opaque = matches!(kind, "wall" | "closed-door" | "opaque-decor");
            if !lit(origin) || lit(hidden) || lit(obstacle) == opaque || lit(shadow) == opaque {
                return Err(format!("Incorrect production mask for {kind}"));
            }
            next_frame().await;
            clear_background(BLACK);
            for background in [true, false] {
                motion.draw_blasts(&map, now, reduced, background, rect);
            }
            let image = crate::ui_capture::framebuffer()?;
            let painted = image
                .bytes
                .chunks_exact(4)
                .filter(|p| p[..3].iter().any(|v| *v > 3))
                .count();
            if painted < 10 {
                return Err(format!("Invisible production blast for {kind}"));
            }
            if kind == "floor" {
                reference = Some(image.bytes.clone());
            } else if !opaque && reference.as_ref() != Some(&image.bytes) {
                return Err(format!("{kind} still cuts the production animation"));
            }
            image.export_png(
                output
                    .join(format!("{kind}-reduced-{reduced}.png"))
                    .to_str()
                    .ok_or("Capture path")?,
            );
            report.push(
                serde_json::json!({"terrain":kind,"reduced_motion":reduced,"painted_pixels":painted,
                "mask_verified":true,"matches_open_floor":!opaque}),
            );
            next_frame().await;
        }
    }
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "review":"SPELL-OBST01","tile_pixels":64,"scenes":report
        }))
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
