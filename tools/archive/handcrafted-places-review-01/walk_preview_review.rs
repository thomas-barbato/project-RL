//! WALK-VIS-01. Isolated interactive review; inactive in ordinary games until
//! the new visual is approved. Pathfinding and walking use the real engine.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedRoute {
    cells: Vec<GridPos>,
}

#[derive(Default)]
struct Navigation {
    pending: Option<PlannedRoute>,
}

#[derive(Clone, Copy)]
enum DestinationStyle {
    Original,
    Beacon,
    Diamond,
    Corners,
    Segments,
    Tile,
    Footprints,
}

const DESTINATIONS: [(DestinationStyle, &str, &str); 6] = [
    (DestinationStyle::Beacon, "A", "Balise"),
    (DestinationStyle::Diamond, "B", "Losange"),
    (DestinationStyle::Corners, "C", "Angles ouverts"),
    (DestinationStyle::Segments, "D", "Anneau segmenté"),
    (DestinationStyle::Tile, "E", "Dalle lumineuse"),
    (DestinationStyle::Footprints, "F", "Empreintes"),
];

fn destination_line(a: Vec2, b: Vec2, blue: Color, ink: Color) {
    draw_line(a.x, a.y, b.x, b.y, 4.5, ink);
    draw_line(a.x, a.y, b.x, b.y, 1.5, blue);
}

fn destination_number(center: Vec2, step: usize, radius: f32, blue: Color, ink: Color) {
    draw_circle(center.x, center.y, radius, ink);
    draw_circle_lines(center.x, center.y, radius, 1., blue);
    crate::ui_theme::draw_text_bold_centered(
        step.to_string(),
        Rect::new(center.x - 12., center.y - 12., 24., 24.),
        if radius < 10. { 12 } else { 14 },
        blue,
    );
}

fn destination_ellipse(center: Vec2, rx: f32, ry: f32, blue: Color, ink: Color) {
    let mut previous = center + vec2(rx, 0.);
    for i in 1..=32 {
        let angle = i as f32 * std::f32::consts::TAU / 32.;
        let point = center + vec2(angle.cos() * rx, angle.sin() * ry);
        draw_triangle(
            center,
            previous,
            point,
            Color::new(blue.r, blue.g, blue.b, 0.12),
        );
        destination_line(previous, point, blue, ink);
        previous = point;
    }
}

fn draw_destination(cell: Rect, step: usize, style: DestinationStyle, blue: Color, ink: Color) {
    let center = cell.center();
    match style {
        DestinationStyle::Original => {
            draw_circle(
                center.x,
                center.y,
                25.,
                Color::new(blue.r, blue.g, blue.b, 0.10),
            );
            draw_circle_lines(center.x, center.y, 22., 5., ink);
            draw_circle_lines(center.x, center.y, 22., 1.5, blue);
            draw_circle_lines(
                center.x,
                center.y,
                26.,
                1.,
                Color::new(blue.r, blue.g, blue.b, 0.35),
            );
            destination_number(center, step, 13., blue, ink);
        }
        DestinationStyle::Beacon => {
            destination_ellipse(center, 21., 7., blue, ink);
            let head = center - vec2(0., 18.);
            let a = center + vec2(-9., -12.);
            let b = center + vec2(9., -12.);
            draw_triangle(a, b, center, ink);
            destination_line(a, center, blue, ink);
            destination_line(b, center, blue, ink);
            destination_number(head, step, 12., blue, ink);
        }
        DestinationStyle::Diamond => {
            draw_poly(
                center.x,
                center.y,
                4,
                27.,
                0.,
                Color::new(blue.r, blue.g, blue.b, 0.12),
            );
            draw_poly_lines(center.x, center.y, 4, 27., 0., 5., ink);
            draw_poly_lines(center.x, center.y, 4, 27., 0., 1.6, blue);
            draw_poly_lines(
                center.x,
                center.y,
                4,
                22.,
                0.,
                1.,
                Color::new(blue.r, blue.g, blue.b, 0.4),
            );
            destination_number(center, step, 11., blue, ink);
        }
        DestinationStyle::Corners => {
            for (dx, dy) in [(-1., -1.), (1., -1.), (-1., 1.), (1., 1.)] {
                let p = center + vec2(dx * 23., dy * 23.);
                destination_line(p - vec2(dx * 11., 0.), p, blue, ink);
                destination_line(p, p - vec2(0., dy * 11.), blue, ink);
            }
            destination_number(center, step, 11., blue, ink);
        }
        DestinationStyle::Segments => {
            for start in [20., 110., 200., 290.] {
                for i in 0..12 {
                    let a = (start + i as f32 * 50. / 12.).to_radians();
                    let b = (start + (i + 1) as f32 * 50. / 12.).to_radians();
                    destination_line(
                        center + vec2(a.cos(), a.sin()) * 24.,
                        center + vec2(b.cos(), b.sin()) * 24.,
                        blue,
                        ink,
                    );
                }
            }
            destination_number(center, step, 11., blue, ink);
        }
        DestinationStyle::Tile => {
            let corners = [
                vec2(-17., -24.),
                vec2(17., -24.),
                vec2(24., -17.),
                vec2(24., 17.),
                vec2(17., 24.),
                vec2(-17., 24.),
                vec2(-24., 17.),
                vec2(-24., -17.),
            ];
            for i in 0..8 {
                let a = center + corners[i];
                let b = center + corners[(i + 1) % 8];
                draw_triangle(center, a, b, Color::new(blue.r, blue.g, blue.b, 0.12));
                destination_line(a, b, blue, ink);
            }
            for p in [center + vec2(0., -24.), center + vec2(0., 24.)] {
                draw_circle(p.x, p.y, 3., ink);
                draw_circle_lines(p.x, p.y, 3., 1., blue);
            }
            destination_number(center, step, 11., blue, ink);
        }
        DestinationStyle::Footprints => {
            // Stylized UI footprints, kept within the destination cell.
            for (x, y) in [(-8., -5.), (8., -11.)] {
                destination_ellipse(center + vec2(x, y), 5., 8., blue, ink);
                let heel = Rect::new(center.x + x - 3., center.y + y + 10., 6., 6.);
                draw_rectangle(heel.x, heel.y, heel.w, heel.h, ink);
                draw_rectangle_lines(heel.x, heel.y, heel.w, heel.h, 1.2, blue);
            }
            destination_number(center + vec2(0., 23.), step, 8., blue, ink);
        }
    }
}

impl Navigation {
    fn update(&mut self, app: &mut AsciiApp, input: &InputFrame, now: f64) {
        let right = input.pressed.contains(&controls::Binding::MouseRight);
        let left = input.pressed.contains(&controls::Binding::MouseLeft);
        if app.menu_navigation_context().is_some() {
            self.pending = None;
            app.update_input_at(input, Some(now));
            return;
        }
        if (self.pending.is_some() || app.mouse_walk.is_some()) && (right || input.pause) {
            self.pending = None;
            app.mouse_walk = None;
            app.movement_repeat.clear();
            return;
        }
        if app.attack_aim.is_some() {
            self.pending = None;
            app.update_input_at(input, Some(now));
            return;
        }
        if left && let Some(at) = app.attack_pointer_cell(input) {
            if !app.game.player_visibility().is_visible(at) {
                return;
            }
            app.mouse_walk = None;
            app.movement_repeat.clear();
            if app.hostile_target_at(at)
                || app.selectable_world_cell(at)
                || app.local_interaction_at(at)
            {
                self.pending = None;
                app.update_input_at(input, Some(now));
                return;
            }
            let Some(path) = app.context_path(at) else {
                self.pending = None;
                return;
            };
            let route = PlannedRoute { cells: path };
            let steps = route.cells.len().saturating_sub(1);
            if steps == 1 || (steps > 1 && self.pending.as_ref() == Some(&route)) {
                self.pending = None;
                app.start_mouse_walk(at, now);
            } else if steps > 1 {
                self.pending = Some(route);
            } else {
                self.pending = None;
            }
            return;
        }
        if right
            || input.pause
            || input
                .pressed
                .iter()
                .any(|binding| matches!(binding, controls::Binding::Key(_)))
        {
            self.pending = None;
        }
        app.update_input_at(input, Some(now));
        if app.attack_aim.is_some() || app.menu_navigation_context().is_some() {
            self.pending = None;
        }
    }

    fn draw(&self, app: &AsciiApp) {
        self.draw_with_destination(app, DestinationStyle::Original);
    }

    fn draw_with_destination(&self, app: &AsciiApp, style: DestinationStyle) {
        app.draw();
        let Some(route) = &self.pending else {
            return;
        };
        if app.menu_navigation_context().is_some()
            || app.attack_aim.is_some()
            || route.cells.first().copied() != app.game.player_position()
        {
            return;
        }
        let rect = |at| {
            app.terminal.world_cell_rect(
                &app.game,
                app.terminal_bounds(),
                app.ui_scale(),
                app.graphics.active.world_cell_px,
                app.navigation_signal_summary().is_some(),
                at,
                app.camera_glide_offset(),
            )
        };
        let blue = Color::from_rgba(143, 204, 243, 255);
        let ink = Color::from_rgba(7, 18, 27, 240);
        for pair in route.cells.windows(2) {
            if !pair
                .iter()
                .all(|at| app.game.player_visibility().is_visible(*at))
            {
                continue;
            }
            let (Some(a), Some(b)) = (rect(pair[0]), rect(pair[1])) else {
                continue;
            };
            let end_inset = if matches!(style, DestinationStyle::Tile)
                && Some(&pair[1]) == route.cells.last()
            {
                if pair[0].x != pair[1].x {
                    b.w * 0.5
                } else {
                    b.h * 0.5
                }
            } else {
                0.
            };
            let (a, b) = (a.center(), b.center());
            // The origin segment starts outside the player's silhouette.
            let direction = (b - a).normalize_or_zero();
            let a = if pair[0] == route.cells[0] {
                a + direction * 22.
            } else {
                a
            };
            // E: stop every stroke at the destination cell boundary. The
            // arrow is positioned on this shortened segment as well.
            let b = b - direction * end_inset;
            draw_line(a.x, a.y, b.x, b.y, 5., ink);
            draw_line(
                a.x,
                a.y,
                b.x,
                b.y,
                8.,
                Color::new(blue.r, blue.g, blue.b, 0.10),
            );
            draw_line(a.x, a.y, b.x, b.y, 1.6, blue);
            let mid = (a + b) * 0.5;
            let side = vec2(-direction.y, direction.x);
            for sign in [-1., 1.] {
                let tail = mid - direction * 4. + side * 3. * sign;
                draw_line(
                    tail.x,
                    tail.y,
                    mid.x + direction.x * 2.,
                    mid.y + direction.y * 2.,
                    1.3,
                    blue,
                );
            }
        }
        for (i, &at) in route.cells.iter().enumerate().skip(1) {
            if !app.game.player_visibility().is_visible(at) {
                continue;
            }
            let Some(cell) = rect(at) else {
                continue;
            };
            let center = cell.center();
            let end = i + 1 == route.cells.len();
            if end {
                draw_destination(cell, i, style, blue, ink);
                continue;
            }
            draw_circle(center.x, center.y, if end { 13. } else { 10. }, ink);
            draw_circle_lines(center.x, center.y, if end { 13. } else { 10. }, 1., blue);
            crate::ui_theme::draw_text_bold_centered(
                i.to_string(),
                Rect::new(center.x - 12., center.y - 12., 24., 24.),
                14,
                blue,
            );
        }
    }
}

fn pointer_at(app: &AsciiApp, at: GridPos, button: controls::Binding) -> InputFrame {
    let cell = app
        .terminal
        .world_cell_rect(
            &app.game,
            app.terminal_bounds_for(1280., 800., 1.),
            1.,
            64,
            app.navigation_signal_summary().is_some(),
            at,
            app.camera_glide_offset(),
        )
        .unwrap();
    app.graphics.active.transform_input(InputFrame {
        pointer: Some(cell.center().into()),
        viewport: Some((1280., 800.)),
        pressed: [button].into(),
        ..Default::default()
    })
}

impl AsciiApp {
    pub async fn capture_walk_destinations(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut app = Self::new_agent_blast_trial()?;
        app.crash_recovery_enabled = false;
        app.suspension_path = output.join("isolated-unused.json");
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.reduced_motion = true;
        app.select_world_target(None);
        let mut navigation = Navigation::default();
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        let click = pointer_at(&app, GridPos::new(8, 4), controls::Binding::MouseLeft);
        navigation.update(&mut app, &click, 0.);
        let route = navigation
            .pending
            .as_ref()
            .ok_or("Aperçu de trajet absent")?
            .cells
            .clone();
        let mut textures = Vec::new();
        for (style, code, _) in DESTINATIONS {
            for _ in 0..2 {
                navigation.draw_with_destination(&app, style);
                next_frame().await;
            }
            navigation.draw_with_destination(&app, style);
            let image = crate::ui_capture::framebuffer()?;
            if (image.width, image.height) != (1280, 800) {
                return Err("Capture attendue en 1280 × 800".into());
            }
            image.export_png(
                output
                    .join(format!("destination-{}.png", code.to_lowercase()))
                    .to_str()
                    .ok_or("Chemin de capture")?,
            );
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            textures.push(texture);
            if suspension::fingerprint(&app.game) != before || app.history.len() != history {
                return Err("Le dessin a modifié la partie".into());
            }
            next_frame().await;
        }
        request_new_screen_size(1040., 720.);
        for _ in 0..3 {
            next_frame().await;
        }
        set_default_camera();
        // GPU framebuffer images are bottom-up in memory; crop and flip at
        // native scale to match the upright full-screen PNGs.
        let crop = Rect::new(680., 800. - 252. - 240., 304., 240.);
        for frame in 0..2 {
            clear_background(Color::from_rgba(12, 18, 23, 255));
            crate::ui_theme::draw_text_bold(
                "Destination · six dessins à comparer",
                24.,
                32.,
                25.,
                Color::from_rgba(230, 225, 208, 255),
            );
            crate::ui_theme::draw_text(
                "WALK-VIS-02 · même chemin · taille réelle 64 px · sans agrandissement",
                24.,
                57.,
                16.,
                Color::from_rgba(165, 183, 190, 255),
            );
            for (i, (_, code, title)) in DESTINATIONS.iter().enumerate() {
                let x = 24. + (i % 3) as f32 * 344.;
                let y = if i < 3 { 104. } else { 438. };
                crate::ui_theme::draw_text_bold(
                    format!("{code} · {title}"),
                    x,
                    y - 13.,
                    20.,
                    Color::from_rgba(230, 225, 208, 255),
                );
                draw_texture_ex(
                    &textures[i],
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(crop),
                        flip_y: true,
                        ..Default::default()
                    },
                );
            }
            crate::ui_theme::draw_text(
                "Seule la destination change · propositions à examiner avant intégration",
                24.,
                708.,
                17.,
                Color::from_rgba(165, 183, 190, 255),
            );
            if frame == 1 {
                let image = crate::ui_capture::framebuffer()?;
                if (image.width, image.height) != (1040, 720) {
                    return Err("Planche attendue en 1040 × 720".into());
                }
                image.export_png(
                    output
                        .join("destinations.png")
                        .to_str()
                        .ok_or("Chemin de planche")?,
                );
            }
            next_frame().await;
        }
        request_new_screen_size(640., 400.);
        for _ in 0..3 {
            next_frame().await;
        }
        for frame in 0..2 {
            clear_background(Color::from_rgba(12, 18, 23, 255));
            crate::ui_theme::draw_text_bold(
                "E · Dalle lumineuse corrigée",
                24.,
                31.,
                23.,
                Color::from_rgba(230, 225, 208, 255),
            );
            crate::ui_theme::draw_text(
                "Le chemin laisse la case d'arrivée dégagée.",
                24.,
                57.,
                16.,
                Color::from_rgba(165, 183, 190, 255),
            );
            draw_texture_ex(
                &textures[4],
                80.,
                80.,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(520., 800. - 232. - 288., 480., 288.)),
                    flip_y: true,
                    ..Default::default()
                },
            );
            if frame == 1 {
                let image = crate::ui_capture::framebuffer()?;
                if (image.width, image.height) != (640, 400) {
                    return Err("Extrait attendu en 640 × 400".into());
                }
                image.export_png(
                    output
                        .join("destination-e-detail.png")
                        .to_str()
                        .ok_or("Chemin d'extrait")?,
                );
            }
            next_frame().await;
        }
        std::fs::write(output.join("destinations-checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"WALK-VIS-02", "status":"awaiting_visual_approval", "normal_games_changed":false,
            "frame_pixels":[1280,800], "comparison_pixels":[1040,720], "tile_pixels":64,
            "source_crop":[680,252,304,240], "source_pixels_per_display_pixel":1,
            "path":route.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(), "steps":route.len()-1,
            "candidates":DESTINATIONS.iter().map(|(_,code,title)|serde_json::json!({"id":format!("WALK-VIS-02-{code}"),"name":title})).collect::<Vec<_>>(),
            "same_path_and_palette":true, "draw_preserved_game_and_history":true,
            "selected_direction":"WALK-VIS-02-E", "revision":"WALK-VIS-03-E",
            "destination_E":"line and chevron stop at the arrival cell boundary; no path inside",
            "selected_detail_pixels":[640,400],
        })).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        Ok(())
    }

    pub async fn run_walk_preview_review() -> Result<(), String> {
        let mut app = Self::new_agent_blast_trial()?;
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join("project-rl-walk-review-unused.json");
        let mut navigation = Navigation::default();
        loop {
            if is_quit_requested() {
                return Ok(());
            }
            let now = get_time();
            app.world_presentation_time.set(now);
            app.visual_cues.prune(now);
            app.combat_motion.prune(now);
            let input = app.graphics.active.transform_input(InputFrame::capture());
            navigation.update(&mut app, &input, now);
            navigation.draw(&app);
            next_frame().await;
        }
    }

    pub async fn capture_walk_preview(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut app = Self::new_agent_blast_trial()?;
        app.crash_recovery_enabled = false;
        app.suspension_path = output.join("isolated-unused.json");
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.reduced_motion = true;
        app.select_world_target(None);
        let mut navigation = Navigation::default();
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        let target = GridPos::new(8, 4);
        let click = pointer_at(&app, target, controls::Binding::MouseLeft);
        navigation.update(&mut app, &click, 0.);
        let route = navigation
            .pending
            .as_ref()
            .ok_or("Aperçu de trajet absent")?
            .cells
            .clone();
        if suspension::fingerprint(&app.game) != before || app.history.len() != history {
            return Err("Aperçu payant".into());
        }
        for _ in 0..3 {
            navigation.draw(&app);
            next_frame().await;
        }
        navigation.draw(&app);
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("trajet.png")
                .to_str()
                .ok_or("Chemin de capture")?,
        );
        navigation.update(
            &mut app,
            &InputFrame {
                pressed: [controls::Binding::MouseRight].into(),
                ..Default::default()
            },
            1.,
        );
        if navigation.pending.is_some()
            || app.world_context.is_some()
            || suspension::fingerprint(&app.game) != before
        {
            return Err("Le clic droit n'annule pas gratuitement le trajet".into());
        }
        app.draw();
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("annule.png")
                .to_str()
                .ok_or("Chemin de capture")?,
        );
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"WALK-VIS-01", "status":"awaiting_visual_approval", "normal_games_changed":false,
            "frame_pixels":[1280,800], "tile_pixels":64, "path":route.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),
            "steps":route.len()-1, "pathfinder":"existing deterministic A*", "preview_free":true,
            "right_click_clears_preview_without_menu_or_turn":true,
        })).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app() -> AsciiApp {
        let mut app = AsciiApp::new_agent_blast_trial().unwrap();
        app.graphics.active.world_cell_px = 64;
        app
    }
    #[test]
    fn distant_preview_is_free_and_right_click_cancels_without_menu() {
        let mut app = app();
        let mut nav = Navigation::default();
        let before = suspension::fingerprint(&app.game);
        let click = pointer_at(&app, GridPos::new(8, 4), controls::Binding::MouseLeft);
        nav.update(&mut app, &click, 0.);
        assert!(nav.pending.is_some());
        assert!(app.mouse_walk.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        nav.update(
            &mut app,
            &InputFrame {
                pressed: [controls::Binding::MouseRight].into(),
                ..Default::default()
            },
            1.,
        );
        assert!(nav.pending.is_none());
        assert!(app.world_context.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
    #[test]
    fn adjacent_click_is_one_turn_and_distant_confirmation_is_one_turn_per_step() {
        let mut app = app();
        let mut nav = Navigation::default();
        let adjacent = GridPos::new(4, 5);
        let turn = app.game.turn();
        let click = pointer_at(&app, adjacent, controls::Binding::MouseLeft);
        nav.update(&mut app, &click, 0.);
        assert_eq!(app.game.player_position(), Some(adjacent));
        assert_eq!(app.game.turn(), turn + 1);
        let target = GridPos::new(8, 5);
        let click = pointer_at(&app, target, controls::Binding::MouseLeft);
        nav.update(&mut app, &click, 1.);
        let steps = nav.pending.as_ref().unwrap().cells.len() - 1;
        let turn = app.game.turn();
        nav.update(&mut app, &click, 2.);
        for tick in 3..20 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(app.game.player_position(), Some(target));
        assert_eq!(app.game.turn(), turn + steps as u64);
    }
    #[test]
    fn clicking_another_destination_replaces_preview_without_moving() {
        let mut app = app();
        let mut nav = Navigation::default();
        let before = suspension::fingerprint(&app.game);
        for target in [GridPos::new(8, 4), GridPos::new(8, 5)] {
            let click = pointer_at(&app, target, controls::Binding::MouseLeft);
            nav.update(&mut app, &click, 0.);
            assert_eq!(nav.pending.as_ref().unwrap().cells.last(), Some(&target));
            assert_eq!(suspension::fingerprint(&app.game), before);
        }
    }
    #[test]
    fn skill_selection_replaces_route_and_right_click_still_cancels_skill() {
        let mut app = app();
        let mut nav = Navigation::default();
        let before = suspension::fingerprint(&app.game);
        let click = pointer_at(&app, GridPos::new(8, 4), controls::Binding::MouseLeft);
        nav.update(&mut app, &click, 0.);
        let key = InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        };
        nav.update(&mut app, &key, 1.);
        assert!(nav.pending.is_none());
        assert!(app.attack_aim_technique.is_some());
        nav.update(
            &mut app,
            &InputFrame {
                pressed: [controls::Binding::MouseRight].into(),
                ..Default::default()
            },
            2.,
        );
        assert!(app.attack_aim.is_none());
        assert!(app.world_context.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
}
