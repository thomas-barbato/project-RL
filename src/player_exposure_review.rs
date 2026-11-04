//! LIGHT01: historical candidates and native checks of the approved normal rendering.
use super::*;
use crate::game_art::{
    self, Visual,
    walking::{self, Pose},
};
use project_rl::entity::EquipmentPalette;
use std::path::Path;

#[derive(Clone, Copy, serde::Serialize)]
pub(crate) struct Profile {
    pub id: u8,
    pub gain: f32,
    pub lift: f32,
    pub edge: f32,
}
pub(crate) const PROFILES: [Profile; 3] = [
    Profile {
        id: 0,
        gain: LEGACY_EXPOSURE.gain,
        lift: LEGACY_EXPOSURE.lift,
        edge: LEGACY_EXPOSURE.edge,
    },
    Profile {
        id: 1,
        gain: PLAYER_EXPOSURE.gain,
        lift: PLAYER_EXPOSURE.lift,
        edge: PLAYER_EXPOSURE.edge,
    },
    Profile {
        id: 2,
        gain: 1.06,
        lift: 0.12,
        edge: 0.0,
    },
];
thread_local! { static PROFILE: Cell<Option<u8>> = const { Cell::new(None) }; }
fn with_profile<T>(profile: u8, f: impl FnOnce() -> T) -> T {
    struct Guard(Option<u8>);
    impl Drop for Guard {
        fn drop(&mut self) {
            PROFILE.set(self.0);
        }
    }
    let _guard = Guard(PROFILE.replace(Some(profile)));
    f()
}
pub(super) fn override_exposure() -> Option<Exposure> {
    PROFILE.get().map(|id| {
        let p = PROFILES[id as usize];
        Exposure {
            gain: p.gain,
            lift: p.lift,
            edge: p.edge,
        }
    })
}

pub(crate) struct Scene {
    target: RenderTarget,
    floor: Texture2D,
    structure: Texture2D,
    props: Vec<(usize, Texture2D)>,
    atmosphere: crate::camp_atmosphere::Atmosphere,
}
impl Scene {
    pub(crate) fn new() -> Result<Self, String> {
        let camp = crate::cryogenic_camp::scene();
        let upload = |path: &str| -> Result<Texture2D, String> {
            let image = crate::asset_preparation::image(path)?;
            let texture = crate::asset_loading::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            Ok(texture)
        };
        let mut props = camp
            .props
            .iter()
            .enumerate()
            .map(|(i, p)| Ok((i, upload(&p.file)?)))
            .collect::<Result<Vec<_>, String>>()?;
        props.sort_by_key(|(i, _)| {
            let p = &camp.props[*i];
            (p.at[1] + p.height, p.at[0])
        });
        Ok(Self {
            target: render_target(640, 610),
            floor: upload(&camp.floor)?,
            structure: upload(&camp.structure)?,
            props,
            atmosphere: crate::camp_atmosphere::Atmosphere::new(
                &camp.atmosphere,
                camp.width as u32 * 64,
                camp.height as u32 * 64,
            ),
        })
    }
    fn render(
        &self,
        visual: Visual,
        pose: Option<Pose>,
        profile: Option<u8>,
    ) -> (Image, Vec<TextureAudit>) {
        self.render_actor(|cell| {
            let draw = || {
                walking::with_pose(pose, || {
                    game_art::draw_world_animated(cell, visual, 1., None)
                })
            };
            if let Some(profile) = profile {
                with_profile(profile, draw)
            } else {
                draw()
            }
        })
    }
    pub(crate) fn render_actor(&self, draw: impl FnOnce(Rect)) -> (Image, Vec<TextureAudit>) {
        // A fixed diagnostic render target remains readable when the native
        // window is reduced. Use the same world draw and lighting composite.
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 640., 610.));
        camera.render_target = Some(self.target.clone());
        set_camera(&camera);
        clear_background(Color::from_rgba(30, 40, 49, 255));
        let camp = crate::cryogenic_camp::scene();
        let world = Rect::new(
            16.,
            -260.,
            camp.width as f32 * 64.,
            camp.height as f32 * 64.,
        );
        let cell = Rect::new(world.x + 4. * 64., world.y + 8. * 64., 64., 64.);
        let _world = super::world(false);
        self.atmosphere.begin(world);
        self.atmosphere.background_time(0.);
        self.atmosphere.begin_background(true);
        for texture in [&self.floor, &self.structure] {
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
        self.atmosphere.end_material();
        let draw_prop = |index: usize, texture: &Texture2D| {
            let p = &camp.props[index];
            draw_texture(
                texture,
                world.x + p.at[0] as f32 * 64. + p.width as f32 * 32. - 64.,
                world.y + (p.at[1] + p.height) as f32 * 64. - 96.,
                WHITE,
            );
        };
        let mut next = 0;
        while let Some((index, texture)) = self.props.get(next) {
            let p = &camp.props[*index];
            if (p.at[1] + p.height, p.at[0]) > (8, 4) {
                break;
            }
            draw_prop(*index, texture);
            next += 1;
        }
        let audit = audit_textures(|| draw(cell));
        for (index, texture) in &self.props[next..] {
            draw_prop(*index, texture);
        }
        self.atmosphere
            .finish(world, Rect::new(16., 60., 576., 512.), true);
        unsafe {
            get_internal_gl().quad_gl.scissor(None);
            get_internal_gl().flush();
        }
        let mut image = self.target.texture.get_texture_data();
        for pixel in image.bytes.chunks_exact_mut(4) {
            pixel[3] = 255;
        }
        (image, audit)
    }
}
fn player(body: u8, armor: Option<u16>) -> Visual {
    Visual::Player {
        body,
        equipment: [armor, None, None, None],
        weapon: Some(31),
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    }
}
pub(crate) fn compare(before: &Image, after: &Image) -> Result<(u64, u64, usize), String> {
    if before.width != 640 || before.height != 610 || after.width != 640 || after.height != 610 {
        return Err("LIGHT01 requires a native640x610 framebuffer".into());
    }
    let (mut old_luma, mut new_luma, mut changed) = (0, 0, 0);
    for y in 0..610usize {
        for x in 0..640usize {
            let offset = ((609 - y) * 640 + x) * 4;
            let a = &before.bytes[offset..offset + 4];
            let b = &after.bytes[offset..offset + 4];
            if a != b {
                if !(208..400).contains(&x) || !(156..348).contains(&y) {
                    return Err(format!("LIGHT01 changed scenery at {x},{y}"));
                }
                changed += 1;
            }
            if (208..400).contains(&x) && (156..348).contains(&y) {
                old_luma += 54 * u64::from(a[0]) + 183 * u64::from(a[1]) + 19 * u64::from(a[2]);
                new_luma += 54 * u64::from(b[0]) + 183 * u64::from(b[1]) + 19 * u64::from(b[2]);
            }
        }
    }
    Ok((old_luma, new_luma, changed))
}
fn panel(board: &mut Image, image: &Image, column: usize) {
    // Native 224x224 crop; Image::export_png reverses the framebuffer rows.
    for y in 0..224usize {
        let src = ((610 - 156 - 224 + y) * 640 + 192) * 4;
        let dst = (y * 672 + column * 224) * 4;
        board.bytes[dst..dst + 224 * 4].copy_from_slice(&image.bytes[src..src + 224 * 4]);
    }
}
fn check_unaffected() -> Result<usize, String> {
    let rt = render_target(256, 256);
    let samples = [
        (Visual::CampNpc(1), false),
        (
            Visual::CampaignMonster {
                number: 120,
                variant: 0,
            },
            false,
        ),
        (
            Visual::ColoredItem(
                game_art::item("core:mitrailleuse_lourde").unwrap(),
                EquipmentPalette::Original,
            ),
            false,
        ),
        (player(3, Some(103)), true),
    ];
    game_art::vertical::prepare(&samples.map(|(visual, _)| visual))?;
    let mut checks = 0;
    for (visual, portrait) in samples {
        let mut reference = None;
        for profile in 0..3 {
            let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 256., 256.));
            camera.render_target = Some(rt.clone());
            set_camera(&camera);
            clear_background(BLACK);
            {
                let _world = (!portrait).then(|| super::world(false));
                with_profile(profile, || {
                    let cell = Rect::new(96., 96., 64., 64.);
                    if portrait {
                        game_art::draw_portrait(cell, visual);
                    } else {
                        game_art::draw_world(cell, visual, 1.);
                    }
                });
            }
            unsafe {
                get_internal_gl().flush();
            }
            let bytes = rt.texture.get_texture_data().bytes;
            if let Some(a) = &reference {
                if a != &bytes {
                    return Err("LIGHT01 changed non-player drawing".into());
                }
                checks += 1;
            } else {
                reference = Some(bytes);
            }
        }
    }
    set_default_camera();
    Ok(checks)
}
pub async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(640., 610.);
    for _ in 0..3 {
        next_frame().await;
    }
    let scene = Scene::new()?;
    let mut records = Vec::new();
    let mut normal_matches = 0;
    let armors = [None, Some(110), Some(102), Some(101), Some(103)];
    for body in [3, 5, 6, 7] {
        for (tier, armor) in armors.into_iter().enumerate() {
            let visual = player(body, armor);
            game_art::vertical::prepare(&[visual])?;
            // Populate the two normal lateral caches before the captured cycles.
            for facing in [-1, 1] {
                for frame in 0..6 {
                    walking::with_pose(Some(Pose { facing, frame }), || {
                        game_art::draw_world(Rect::new(0., 0., 64., 64.), visual, 1.)
                    });
                }
            }
            for (direction, facing) in [("front", 2), ("back", -2), ("left", -1), ("right", 1)] {
                for phase in 0..7 {
                    let pose = (phase < 6).then_some(Pose {
                        facing,
                        frame: phase,
                    });
                    let mut board =
                        Image::gen_image_color(672, 256, Color::from_rgba(30, 40, 49, 255));
                    let mut reference = None;
                    for profile in 0..3 {
                        let (image, audit) = scene.render(visual, pose, Some(profile));
                        if audit.is_empty()
                            || audit.iter().any(|a| {
                                !a.actor
                                    || !a.world
                                    || (a.gain - PROFILES[profile as usize].gain).abs() > 0.0001
                            })
                        {
                            return Err(format!(
                                "LIGHT01 wrong player shader: {body} {tier} {direction} {phase} {profile}"
                            ));
                        }
                        if profile == 1 {
                            let (normal, audit) = scene.render(visual, pose, None);
                            if normal.bytes != image.bytes
                                || audit.is_empty()
                                || audit.iter().any(|a| (a.gain - 1.18).abs() > 0.0001)
                            {
                                return Err(format!(
                                    "LIGHT01 normal rendering differs from Discret: {body} {tier} {direction} {phase}"
                                ));
                            }
                            normal_matches += 1;
                        }
                        if let Some(before) = &reference {
                            let (a, b, changed) = compare(before, &image)?;
                            if b >= a || changed == 0 {
                                return Err("LIGHT01 candidate did not reduce exposure".into());
                            }
                            records.push(serde_json::json!({"body":body,"armor_stage":tier,"direction":direction,"phase":phase,"profile":profile,"changed_actor_pixels":changed,"old_luma":a,"candidate_luma":b,"scenery_unchanged":true,"shader_gain":audit[0].gain}));
                        } else {
                            reference = Some(image.clone());
                        }
                        panel(&mut board, &image, profile as usize);
                        next_frame().await;
                    }
                    board.export_png(
                        output
                            .join(format!("light-{body}-{tier}-{direction}-{phase}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
            }
            // Compare the real rest image after both lateral cycles and a clockless draw.
            for profile in 0..3 {
                let (a, _) = scene.render(visual, None, Some(profile));
                let _ = scene.render(
                    visual,
                    Some(Pose {
                        facing: -1,
                        frame: 3,
                    }),
                    Some(profile),
                );
                let (b, _) = scene.render(visual, None, Some(profile));
                if a.bytes != b.bytes {
                    return Err("LIGHT01 rest exposure changed after walking".into());
                }
            }
            eprintln!(
                "[LIGHT01] body{body} armor{tier}: same scenery, lower exposure, stable rest"
            );
        }
    }
    let unaffected = check_unaffected()?;
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"LIGHT01-r01","normal_game_enabled":true,"approved_profile":1,"normal_vs_discret_pixel_comparisons":normal_matches,"profiles":PROFILES,"native_cell_px":64,"same_position_and_light_time":true,"classes":4,"armor_stages":5,"directions":4,"phases":7,"boards":560,"comparison_count":records.len(),"non_player_pixel_comparisons":unaffected,"rest_return_comparisons":60,"records":records})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
