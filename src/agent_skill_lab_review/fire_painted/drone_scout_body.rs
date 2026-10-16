//! 108 / D56-BODY01-SIZE02: compare actual screen sizes, no campaign asset swap.
use super::super::melee::actor;
use super::zones::terrain;
use super::*;
use project_rl::world::Map;

const MAP: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";

struct Art {
    scout: Texture2D,
    scout_previous: Texture2D,
    tracker: Texture2D,
    guardian: Texture2D,
    gunner: Texture2D,
}
impl Art {
    fn load() -> Self {
        fn decode(data: &[u8]) -> Texture2D {
            let texture = Texture2D::from_file_with_format(data, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            texture
        }
        Self {
            scout: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-97/D56-BODY01-SIZE02-native128.png"
            )),
            scout_previous: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-97/D56-BODY01-native128.png"
            )),
            tracker: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-96/D58-BODY01-native128.png"
            )),
            guardian: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
            )),
            gunner: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            )),
        }
    }
}
fn point(x: f32, y: f32, rect: Rect) -> Vec2 {
    vec2(rect.x + (x + 0.5) * 64., rect.y + (y + 0.5) * 64.)
}
fn scene(art: &Art, proposed: bool, rect: Rect) {
    terrain(&Map::from_ascii(MAP).unwrap(), 2, rect, false);
    let scout = point(1., 4., rect);
    let guardian = point(3., 4., rect);
    let tracker = point(5., 4., rect);
    let gunner = point(7., 4., rect);
    draw_ellipse(scout.x, scout.y + 30., 22., 3., 0., alpha(BLACK, 0.3));
    draw_texture(
        if proposed {
            &art.scout
        } else {
            &art.scout_previous
        },
        scout.x - 64.,
        scout.y - 64.,
        WHITE,
    );
    draw_ellipse(guardian.x, guardian.y + 30., 30., 4., 0., alpha(BLACK, 0.3));
    draw_texture(&art.guardian, guardian.x - 64., guardian.y - 64., WHITE);
    draw_ellipse(tracker.x, tracker.y + 30., 26., 4., 0., alpha(BLACK, 0.3));
    draw_texture(&art.tracker, tracker.x - 64., tracker.y - 64., WHITE);
    draw_ellipse(gunner.x, gunner.y + 29., 25., 4., 0., alpha(BLACK, 0.3));
    draw_texture(&art.gunner, gunner.x - 32., gunner.y - 32., WHITE);
    actor(
        point(4., 7., rect),
        game_art::wear(
            game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
            [("core:body_armor".into(), "core:tenue_technique".into())],
        ),
    );
    for (label, at) in [
        ("Éclaireur", scout),
        ("Gardien", guardian),
        ("Traqueur", tracker),
        ("Artilleur", gunner),
    ] {
        let width = measure_text(label, None, 16, 1.).width;
        draw_text_bold(label, at.x - width / 2., at.y + 52., 16., TEXT);
    }
}
pub(crate) async fn run() {
    request_new_screen_size(1230., 700.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("108 : premier export 56 x 32 px", 20., 35., 23., TEXT);
        draw_text_bold("108 : nouvel export 70 x 40 px", 630., 35., 23., TEXT);
        scene(&art, false, Rect::new(20., 65., 576., 576.));
        scene(&art, true, Rect::new(630., 65., 576., 576.));
        draw_text(
            "Case de 64 px. Candidat à valider ; règles et campagne inchangées.",
            20.,
            677.,
            17.,
            DIM,
        );
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists()
        || output
            .parent()
            .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Utiliser une revue neuve non validée.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    for proposed in [false, true] {
        set_camera(&camera);
        clear_background(INK);
        scene(&art, proposed, Rect::new(0., 0., 576., 576.));
        unsafe {
            macroquad::window::get_internal_gl().flush();
        }
        target.texture.get_texture_data().export_png(
            output
                .join(if proposed {
                    "scene-after.png"
                } else {
                    "scene-before.png"
                })
                .to_str()
                .unwrap(),
        );
        set_default_camera();
        next_frame().await;
    }
    Ok(())
}
