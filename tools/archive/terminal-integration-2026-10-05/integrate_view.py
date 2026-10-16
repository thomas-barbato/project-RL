from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
path=ROOT/'src/terminal_view.rs'; s=path.read_text(encoding='utf-8')
def replace(old,new):
    global s
    assert old in s,old[:120]
    s=s.replace(old,new,1)
def between(start,end,new):
    global s
    a=s.index(start); b=s.index(end,a); s=s[:a]+new+s[b:]

between('        let safety_width = measure_ui_text_bold(safety_label, 16).width', '        let (sidebar, camera) = terminal_grid_camera(', '''        // Terminal chrome lives in logical UI pixels, independently of native map tiles.
        set_camera(&crate::graphics::ui_camera(screen_width() / ui_scale, screen_height() / ui_scale));
        let logical = |r: Rect| Rect::new(r.x / ui_scale, r.y / ui_scale, r.w / ui_scale, r.h / ui_scale);
        let header = logical(layout.header);
        draw_rectangle(header.x, header.y, header.w, header.h, crate::terminal_chrome::ink());
        let safety_width = measure_ui_text_bold(safety_label, 12).width + 24.0;
        let title = if self.title == TestSector::NAME
            || self.title.eq_ignore_ascii_case("ville de départ / friches")
        { "Mégapole de surface" } else { &self.title };
        draw_bounded_text(&player_location_name(title), header.x + 10.0, header.y + 21.0,
            (header.w - safety_width - 20.0).max(60.0), 16, cyan);
        draw_ui_text_bold_centered(safety_label,
            Rect::new(header.right() - safety_width, header.y + 1.0, safety_width - 8.0, header.h - 2.0),
            12, if protected { cyan } else { UiTheme.attention() });
        crate::terminal_chrome::bezel(header);
        if let (Some(signal), Some(route)) = (navigation_signal, layout.route) {
            let route = logical(route);
            crate::terminal_chrome::button(route, false, false, true);
            draw_ui_text_bold("ITINÉRAIRE", route.x + 10.0, route.y + 19.0, 12.0, muted);
            draw_bounded_text(signal, route.x + 92.0, route.y + 19.0, route.w - 102.0, 14, cyan);
        }
        set_default_camera();
''')
between('        let description = if sidebar { description } else { description };', '        if sidebar {\n            self.draw_sidebar(', '''        set_camera(&crate::graphics::ui_camera(screen_width() / ui_scale, screen_height() / ui_scale));
        let world = logical(layout.map);
        crate::terminal_chrome::bezel(Rect::new(world.x - 2.0, world.y - 2.0, world.w + 4.0, world.h + 4.0));
        let footer = logical(layout.footer);
        draw_rectangle(footer.x, footer.y, footer.w, footer.h, crate::terminal_chrome::ink());
        draw_line(footer.x, footer.y, footer.right(), footer.y, 1.0, crate::terminal_chrome::tint(0.32));
        let prefix = if footer.w > 430.0 {
            format!("TOUR {:04} / X {:02} Y {:02}", game.turn(), focus.x, focus.y)
        } else { format!("TOUR {:04}", game.turn()) };
        let prefix_w = prefix.chars().count() as f32 * 12.0 * 0.62 + 16.0;
        crate::terminal_chrome::heading(&prefix, footer.x + 6.0, footer.y + 16.0, 12, cyan);
        draw_bounded_text(&description, footer.x + prefix_w, footer.y + 16.0,
            footer.w - prefix_w - 6.0, 12, muted);
        set_default_camera();
''')
replace('                layout.objective,\n            );', '                layout.objective,\n                ui_scale,\n            );')
replace('            draw_compact_preview(panel, target);', '''            set_camera(&crate::graphics::ui_camera(screen_width() / ui_scale, screen_height() / ui_scale));
            draw_compact_preview(logical(panel), target);
            set_default_camera();''')
between('    #[cfg(debug_assertions)]\n    pub(crate) fn console_review_title', '    fn draw_sidebar(', '')
replace('        objective: Rect,\n    ) {', '        objective: Rect,\n        ui_scale: f32,\n    ) {')
between('        UiTheme.hud_panel(panel);\n        let rect = Rect::new(', '\n    }\n}\n\nfn compact_preview_panel', '''        set_camera(&crate::graphics::ui_camera(screen_width() / ui_scale, screen_height() / ui_scale));
        let logical = |r: Rect| Rect::new(r.x / ui_scale, r.y / ui_scale, r.w / ui_scale, r.h / ui_scale);
        let panel = logical(panel);
        let objective = logical(objective);
        let available = objective.y - panel.y - 16.0;
        let sensors = Rect::new(panel.x, panel.y, panel.w, (available * 0.60).min(283.0));
        let inspection = Rect::new(panel.x, sensors.bottom() + 8.0, panel.w,
            objective.y - sensors.bottom() - 16.0);
        crate::terminal_chrome::module(sensors, "05 / CAPTEURS");
        let extra_lines = [visible_local_alerts, visible_security_alarms, visible_security_lockdowns]
            .into_iter().filter(|count| *count > 0).count() + usize::from(tactical_overlay_open);
        let map_area = Rect::new(sensors.x + 12.0, sensors.y + 40.0, sensors.w - 24.0,
            (sensors.h - 129.0 - extra_lines as f32 * 18.0).clamp(60.0, 154.0));
        let (known_min, known_max) = remembered_bounds(&self.remembered, game.player_position(),
            game.map().width(), game.map().height());
        let known_width = (known_max.x - known_min.x + 1).max(1) as f32;
        let known_height = (known_max.y - known_min.y + 1).max(1) as f32;
        let pixel = (map_area.w / known_width).min(map_area.h / known_height).floor().clamp(1.0, 12.0);
        let origin = vec2(map_area.x + (map_area.w - known_width * pixel) * 0.5,
            map_area.y + (map_area.h - known_height * pixel) * 0.5);
        draw_sensor_scope(map_area, &self.remembered, game, known_min, known_max, pixel, origin,
            observation_fields, sound_fields, sensor_contacts, true);
        let y = map_area.bottom() + 25.0;
        for (i, (count, label, color)) in [(visible_hostiles, "Hostiles", UiTheme.danger()),
            (visible_neutrals, "Neutres", cyan), (visible_items, "Objets", UiTheme.attention())]
            .into_iter().enumerate()
        {
            let x = map_area.x + i as f32 * map_area.w / 3.0;
            draw_ui_text_bold(count.to_string(), x, y, 20.0, color);
            draw_ui_text(label, x, y + 18.0, 13.0, muted);
        }
        let mut y = map_area.bottom() + 63.0;
        for (count, label) in [(visible_local_alerts, "Alertes locales"),
            (visible_security_alarms, "Alarmes réseau"), (visible_security_lockdowns, "Verrouillages")]
            .into_iter().filter(|(count, _)| *count > 0)
        {
            draw_bounded_text(&format!("{label} {count}"), map_area.x, y, map_area.w, 12, UiTheme.attention());
            y += 18.0;
        }
        if tactical_overlay_open {
            draw_bounded_text(&format!("{observation_label} · Vision / bruit"), map_area.x, y, map_area.w, 12, cyan);
        }
        crate::terminal_chrome::module(inspection, "06 / INSPECTION");
        let rect = Rect::new(inspection.x + 12.0, inspection.y + 34.0, inspection.w - 24.0, inspection.h - 42.0);
        if let Some(target) = target {
            let portrait = target.visual.map(|visual| {
                let side = if target.analysis.is_some() || inspection.h < 180.0 { 64.0 } else { 96.0 };
                let image = Rect::new(rect.right() - side, rect.y + 3.0, side, side);
                crate::game_art::draw_portrait(image, visual);
                image
            });
            let width = portrait.map_or(rect.w, |p| p.x - rect.x - 8.0);
            let mut y = draw_wrapped_lines(&target.name, rect.x, rect.y + 16.0, width, 15, 3, bright);
            draw_bounded_text(&format!("À {} case{}", target.distance,
                if target.distance > 1 { "s" } else { "" }), rect.x, y + 3.0, width, 13, muted);
            y += 23.0;
            if let Some(image) = portrait { y = y.max(image.bottom() + 16.0); }
            if let Some((hp, maximum)) = target.health {
                draw_hud_progress(Rect::new(rect.x, y - 6.0, rect.w, 6.0), hp, maximum, UiTheme.success());
                crate::terminal_chrome::segment(Rect::new(rect.x, y - 6.0, rect.w, 6.0), 20);
                y += 15.0;
            }
            let remaining_lines = |y: f32| ((rect.bottom() - y) / 18.0).floor().max(0.0) as usize;
            if let Some(action) = &target.interaction {
                y = draw_wrapped_lines(action, rect.x, y, rect.w, 13, remaining_lines(y).min(3), cyan);
            } else if let Some(analysis) = &target.analysis {
                if remaining_lines(y) > 0 {
                    draw_bounded_text(&format!("PV {} / {} · Armure {}", analysis.integrity,
                        analysis.maximum_integrity, analysis.armor), rect.x, y, rect.w, 13, bright);
                    y += 18.0;
                }
                y = draw_wrapped_lines(&analysis.resistances, rect.x, y, rect.w, 12,
                    remaining_lines(y).min(3), muted);
            } else {
                y = draw_wrapped_lines("Analysez la cible pour connaître ses défenses.", rect.x, y,
                    rect.w, 12, remaining_lines(y).min(2), muted);
            }
            if target.visible_state != "Aucun état visible" && remaining_lines(y) > 0 {
                draw_wrapped_lines(&target.visible_state, rect.x, y + 3.0, rect.w, 12, remaining_lines(y), cyan);
            }
        } else {
            draw_ui_text("Survolez une cible", rect.x, rect.y + 16.0, 14.0, muted);
        }
        set_default_camera();''')
# Stable objective bounds are shared by the renderer and mouse hit handling.
replace('''                panel.x + 16.0,
                panel.bottom() - 16.0 - 144.0 * ui_scale,
                panel.w - 32.0,
                144.0 * ui_scale,''', '''                panel.x,
                panel.bottom() - (panel.h * 0.27).clamp(144.0 * ui_scale, 182.0 * ui_scale),
                panel.w,
                (panel.h * 0.27).clamp(144.0 * ui_scale, 182.0 * ui_scale),''')
replace('    UiTheme.hud_panel(panel);\n    if panel.h < 160.0 {', '    crate::terminal_chrome::module(panel, "06 / INSPECTION");\n    if panel.h < 160.0 {')
between('        draw_ui_text_bold(\n            "CIBLE",', '        draw_wrapped_lines(', '')
between('    draw_ui_text_bold(\n        "CIBLE",', '    let side = (panel.h - 76.0)', '')
path.write_text(s,encoding='utf-8',newline='\r\n')
