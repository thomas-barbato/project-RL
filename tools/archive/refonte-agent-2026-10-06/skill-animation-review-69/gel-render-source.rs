//! GEL-A04: persistent body-attached frost, isolated visual fixture.
use super::*;

pub(in crate::agent_skill_lab_review::fire_painted) struct GelArt {
    rear: Texture2D,
    foreground: Option<Texture2D>,
}
impl GelArt {
    pub(super) fn load() -> Self {
        Self::load_layers(std::env::args().any(|arg| arg == "--gel-status-front"))
    }
    pub(in crate::agent_skill_lab_review::fire_painted) fn load_approved() -> Self {
        Self::load_layers(true)
    }
    fn load_layers(front: bool) -> Self {
        let texture = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-27/gel-atlas.png"
            ),
            Some(ImageFormat::Png),
        );
        texture.set_filter(FilterMode::Linear);
        let foreground = front.then(|| {
            let texture = Texture2D::from_file_with_format(
                include_bytes!("../../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-28/gel-atlas.png"),
                Some(ImageFormat::Png),
            );
            texture.set_filter(FilterMode::Linear);
            texture
        });
        Self {
            rear: texture,
            foreground,
        }
    }
    pub(super) fn variant(&self) -> &'static str {
        if self.foreground.is_some() {
            "GEL-A05"
        } else {
            "GEL-A04"
        }
    }
    pub(in crate::agent_skill_lab_review::fire_painted) fn draw(
        &self,
        at: Vec2,
        time: f32,
        phase: usize,
        front: bool,
    ) {
        let position = time.max(0.0) * 3.0 + phase as f32;
        let frame = position.floor() as usize % 6;
        let blend = position.fract();
        let size = 72.0;
        // The revised coating covers chest/legs, preserving the upper head area.
        // Only the bottom crystals of the old drawing remain behind the actor.
        let (texture, crop_y, crop_height, opacity) = if let Some(foreground) = &self.foreground {
            if front {
                (foreground, 64.0, 192.0, 0.70)
            } else {
                (&self.rear, 192.0, 64.0, 0.26)
            }
        } else {
            (&self.rear, 0.0, 256.0, if front { 0.50 } else { 0.72 })
        };
        for (frame, weight) in [(frame, 1.0 - blend), ((frame + 1) % 6, blend)] {
            draw_texture_ex(
                texture,
                at.x - size / 2.0,
                at.y - size / 2.0 + crop_y * size / 256.0,
                alpha(WHITE, weight * opacity),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256.0, crop_y, 256.0, crop_height)),
                    dest_size: Some(vec2(size, size * crop_height / 256.0)),
                    ..Default::default()
                },
            );
        }
    }
}

const STATUS_MAP: &str = "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########";
const TARGETS: [GridPos; 3] = [GridPos::new(3, 4), GridPos::new(4, 4), GridPos::new(6, 4)];
struct State {
    clock: f32,
    turn: u16,
    applied: bool,
    replace: bool,
    gel: [u16; 3],
    burn: [u16; 3],
}
impl State {
    fn new(replace: bool) -> Self {
        Self {
            clock: 0.0,
            turn: 0,
            applied: false,
            replace,
            gel: [0; 3],
            burn: [0; 3],
        }
    }
    fn apply(&mut self) {
        self.applied = true;
        self.gel = [3, 3, 0];
        self.burn = [0; 3];
    }
    fn advance(&mut self) {
        self.turn += 1;
        for n in &mut self.gel {
            *n = n.saturating_sub(1);
        }
        for n in &mut self.burn {
            *n = n.saturating_sub(1);
        }
        if self.replace && self.applied && self.turn == 2 {
            self.gel = [0; 3];
            self.burn = [1, 1, 0];
        }
    }
}

fn scene(ice: &GelArt, fire: &DrawnFire, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(STATUS_MAP).unwrap();
    terrain(&map, 2, rect, false);
    let point = |at: GridPos| cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
    if effects {
        clip.apply_for(0, 2, rect);
        for (i, at) in TARGETS.iter().enumerate() {
            if state.gel[i] > 0 {
                ice.draw(point(*at), state.clock, i * 2, false);
            }
            if state.burn[i] > 0 {
                fire.burn(point(*at), state.clock, i as u32 * 2, 0.9);
            }
        }
        gl_use_default_material();
    }
    terrain(&map, 2, rect, true);
    for (i, at) in TARGETS.iter().enumerate() {
        let p = point(*at);
        if i == 0 {
            actor(p, caster());
        } else {
            enemy(p);
        }
        if effects {
            clip.apply_for(0, 2, rect);
            if state.gel[i] > 0 {
                ice.draw(p, state.clock, i * 2, true);
            }
            if state.burn[i] > 0 {
                burn_on_body(fire, p, state.clock, i as u32 * 2);
            }
            gl_use_default_material();
        }
        draw_text_bold(
            if i == 0 {
                "Agent"
            } else if i == 1 {
                "Monstre"
            } else {
                "Sans Gel"
            },
            p.x - 24.0,
            p.y - 43.0,
            13.0,
            TEXT,
        );
    }
    let text = if !state.applied {
        "Avant application".to_string()
    } else if state.burn[0] > 0 {
        format!("Brûlure : {} tour · Gel retiré", state.burn[0])
    } else if state.gel[0] > 0 {
        format!("Gel : {} tours · reflets animés", state.gel[0])
    } else {
        "Statuts terminés".to_string()
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 27.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(text, rect.x + 14.0, rect.y + rect.h - 11.0, 14.0, TEXT);
}

pub(crate) async fn run() {
    request_new_screen_size(1280.0, 900.0);
    let ice = GelArt::load();
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&[STATUS_MAP], CENTRE);
    let mut state = State::new(false);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / GEL ANIMÉ", 22.0, 35.0, 24.0, TEXT);
        if button(
            Rect::new(20.0, 100.0, 285.0, 35.0),
            "Fin naturelle du Gel",
            true,
        ) {
            state = State::new(false);
        }
        if button(
            Rect::new(20.0, 145.0, 285.0, 35.0),
            "Remplacement par Brûlure",
            true,
        ) {
            state = State::new(true);
        }
        if button(
            Rect::new(20.0, 220.0, 285.0, 35.0),
            "Appliquer le Gel",
            !state.applied,
        ) || (is_key_pressed(KeyCode::Enter) && !state.applied)
        {
            state.apply();
        }
        if button(
            Rect::new(20.0, 265.0, 285.0, 35.0),
            "+1 tour",
            state.applied && state.turn < 3,
        ) || (is_key_pressed(KeyCode::Space) && state.applied && state.turn < 3)
        {
            state.advance();
        }
        if button(Rect::new(20.0, 310.0, 285.0, 35.0), "Recommencer", true) {
            state = State::new(state.replace);
        }
        if button(
            Rect::new(20.0, 355.0, 285.0, 35.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(20.0, 430.0, 285.0, 35.0),
            "19 · Explosion cristalline",
            true,
        ) {
            super::crystalline::run().await;
            request_new_screen_size(1280.0, 900.0);
        }
        draw_text(format!("Tour {}", state.turn), 22.0, 525.0, 16.0, TEXT);
        draw_text("Statuts préparés pour la revue.", 22.0, 570.0, 15.0, DIM);
        scene(
            &ice,
            &fire,
            &clip,
            &state,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
        );
        draw_text(format!("20 / {}", ice.variant()), 410.0, 695.0, 15.0, DIM);
        if !paused {
            state.clock += get_frame_time();
        }
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(720.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let ice = GelArt::load();
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&[STATUS_MAP], CENTRE);
    let target = render_target(720, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 720.0, 720.0))
    };
    let controls = [3, 14, 17, 24, 35, 47, 50, 63, 66, 79, 83, 95];
    let mut entries = Vec::new();
    for replace in [false, true] {
        let folder = if replace { "BRULURE" } else { "GEL" };
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut state = State::new(replace);
        let mut timeline = Vec::new();
        for frame in 0..96 {
            state.clock = frame as f32 / 24.0;
            if frame == 17 {
                state.apply();
            }
            if [48, 64, 80].contains(&frame) {
                state.advance();
            }
            timeline.push(serde_json::json!({"frame":frame,"turn":state.turn,"gel":state.gel,"burn":state.burn}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                draw_text_bold("20 · Gel animé", 72.0, 35.0, 23.0, TEXT);
                draw_text(
                    format!("{} · tour {}", ice.variant(), state.turn),
                    72.0,
                    60.0,
                    14.0,
                    DIM,
                );
                scene(
                    &ice,
                    &fire,
                    &clip,
                    &state,
                    Rect::new(72.0, 88.0, SIZE, SIZE),
                    control == 0,
                );
                super::export(
                    &target,
                    &dir.join(if control == 0 {
                        format!("{frame:03}.rgba")
                    } else {
                        format!("control-{frame:03}.rgba")
                    }),
                );
            }
            if [47, 63, 79].contains(&frame) {
                let stage = [47, 63, 79].iter().position(|&v| v == frame).unwrap();
                let saved_clock = state.clock;
                for i in 0..48 {
                    state.clock = i as f32 / 24.0;
                    set_camera(&camera);
                    clear_background(INK);
                    scene(
                        &ice,
                        &fire,
                        &clip,
                        &state,
                        Rect::new(72.0, 88.0, SIZE, SIZE),
                        true,
                    );
                    super::export(&target, &dir.join(format!("idle-{stage}-{i:02}.rgba")));
                }
                state.clock = saved_clock;
            }
            set_default_camera();
            next_frame().await;
        }
        std::fs::write(
            dir.join("timeline.json"),
            serde_json::to_vec_pretty(&timeline).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        entries.push(serde_json::json!({"number":20,"id":"GEL","title":"Gel animé","variant":if replace {format!("{}-BRULURE",ice.variant())} else {ice.variant().to_string()},"folder":folder,"crop":[72,88,576,576],"frames":96,"fps":24,"tile":64,"control_frames":controls,"launch_frame":17,"turn_frames":[48,64,80],"replacement":replace,"map":STATUS_MAP}));
    }
    super::crystalline::capture(&output.join("EXPLOSION")).await?;
    let text = std::fs::read_to_string(output.join("EXPLOSION/capture.json"))
        .map_err(|e| e.to_string())?;
    let explosion: Vec<serde_json::Value> =
        serde_json::from_str(&text).map_err(|e| e.to_string())?;
    for mut e in explosion {
        e["folder"] = format!("EXPLOSION/{}", e["folder"].as_str().unwrap()).into();
        entries.push(e);
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn visual_clock_does_not_expire_status() {
        let mut state = State::new(false);
        state.apply();
        state.clock = 600.0;
        assert_eq!(state.gel, [3, 3, 0]);
        assert_eq!(state.turn, 0);
        for _ in 0..3 {
            state.advance();
        }
        assert_eq!(state.gel, [0; 3]);
    }
    #[test]
    fn burn_replacement_removes_gel_only_from_affected_actors() {
        let mut state = State::new(true);
        state.apply();
        state.advance();
        state.advance();
        assert_eq!(state.gel, [0; 3]);
        assert_eq!(state.burn, [1, 1, 0]);
        state.advance();
        assert_eq!(state.burn, [0; 3]);
    }
}
