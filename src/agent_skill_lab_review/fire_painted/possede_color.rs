//! Static native comparison for TR64-BODY04. No gameplay or VFX changes.
use super::zones::{Clip, position, terrain};
use super::*;
use project_rl::world::Map;

const MAPS: [&str; 1] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
const PLAYER: GridPos = GridPos::new(4, 4);

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let load = |bytes: &[u8]| {
        let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
        texture.set_filter(FilterMode::Linear);
        texture
    };
    let bodies = [
        load(include_bytes!(
            "../../../assets/animations/agent/skill-animation-review-41/possede-64.png"
        )),
        load(include_bytes!(
            "../../../assets/animations/agent/skill-animation-review-46/possede-64.png"
        )),
    ];
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let map = Map::from_ascii(MAPS[0]).unwrap();
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    set_camera(&camera);
    clear_background(INK);
    for (i, body) in bodies.iter().enumerate() {
        let x = 32.0 + i as f32 * 616.0;
        draw_text_bold(
            if i == 0 {
                "36 · Possédé · violet validé"
            } else {
                "36 · Possédé · essai chair"
            },
            x,
            35.0,
            23.0,
            TEXT,
        );
        draw_text(
            if i == 0 {
                "TR64-BODY01 · avant"
            } else {
                "TR64-BODY04 · proposé"
            },
            x,
            60.0,
            14.0,
            DIM,
        );
        let stage = Rect::new(x, 88.0, 576.0, 576.0);
        terrain(&map, 2, stage, false);
        terrain(&map, 2, stage, true);
        clip.apply_for(0, 2, stage);
        for at in [GridPos::new(6, 4), GridPos::new(2, 4), GridPos::new(4, 2)] {
            let p = position(at, 2);
            enemy(cell(stage, p.x as f32 + 0.5, p.y as f32 + 0.5));
        }
        let at = cell(stage, 4.5, 4.5);
        draw_ellipse(at.x, at.y + 29.0, 23.0, 4.0, 0.0, alpha(BLACK, 0.3));
        draw_texture_ex(
            body,
            at.x - 32.0,
            at.y - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
    super::occult::export(&target, &output.join("ground-native.rgba"));
    set_default_camera();
    next_frame().await;
    std::fs::write(output.join("capture.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "native_tile_size":64,"capture_size":[1280,720],"kind":"static native GPU comparison",
        "before":"TR64-BODY01","proposal":"TR64-BODY04",
        "scope":"Body colour only. No transition, own-action VFX, mechanics or campaign integration."
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
