from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs';s=Path(a).read_text(encoding='utf-8')
start=s.index('            let hint_y = self.ui_height() - 94.0;');end=s.index('\n        }\n\n        for (index, (message, objective))',start)
s=s[:start]+'            self.draw_hud_shortcuts();'+s[end:]
# Skills retain authored descriptions, using a scoped scroll pane for all text.
start=s.index('    fn draw_skills(');idx=s.index('            let x = layout.detail_panel.x + 13.0;',start)
s=s[:idx]+'''            let pane=Rect::new(layout.detail_panel.x+4.0,layout.detail_panel.y+4.0,layout.detail_panel.w-8.0,layout.detail_panel.h-8.0);
            let selection=(self.skill_discipline_selection,self.skill_technique_selection);
            let offset=if self.ux.skill_selection.get()==selection { self.ux.skill_scroll.offset } else {0.0};
            self.ux.skill_selection.set(selection);
            crate::ui_theme::begin_text_pane(pane,offset);
'''+s[idx:]
old='            draw_wrapped_text(&usage, x, y, available_width, remaining_lines, 14, muted);'
new='''            let usage = usage.replace("P1+A1", "Préparation 1 tour + action 1 tour").replace("P2+A1", "Préparation 2 tours + action 1 tour");
            let bottom=draw_wrapped_text(&usage, x, y, available_width, remaining_lines, 15, muted);
            crate::ui_theme::end_text_pane();
            self.ux.skill_scroll.finish(pane,bottom);'''
assert old in s;s=s.replace(old,new,1)
s=s.replace('|steps| format!("P{}+A1", steps.get()),','|steps| format!("Préparation {} tour(s) + action 1 tour", steps.get()),',1)
# Return an owned selection to support an actual availability filter consistently.
start=s.index('    fn selected_skill_techniques(');end=s.index('    fn clamp_skill_selection',start)
s=s[:start]+'''    fn selected_skill_techniques(&self) -> Vec<TechniqueId> {
        let Some(discipline)=self.skill_disciplines_cache.get(self.skill_discipline_selection) else {return Vec::new();};
        self.skill_techniques_cache.get(discipline).into_iter().flatten()
            .filter(|id| !self.ux.skill_filter || self.skill_learning_cost(id).is_ok()).cloned().collect()
    }

'''+s[end:]
idx=s.index('        let hovered_discipline',s.index('    fn update_skills('))
s=s[:idx]+'''        let filter=Self::skill_filter_rect(layout.panel);
        if input.pressed.contains(&controls::Binding::key("Tab")) || input.pressed.contains(&controls::Binding::MouseLeft) && input.pointer.is_some_and(|p|filter.contains(p.into())) {
            self.ux.skill_filter=!self.ux.skill_filter;
            self.skill_technique_selection=0;self.ux.skill_scroll.offset=0.0;return;
        }
        if self.ux.skill_selection.get()!=(self.skill_discipline_selection,self.skill_technique_selection) {self.ux.skill_scroll.offset=0.0;}
        if input.pointer.is_some_and(|p|layout.detail_panel.contains(p.into())) && input.wheel_y!=0.0 || input.pressed.contains(&controls::Binding::key("PageDown")) || input.pressed.contains(&controls::Binding::key("PageUp")) {
            self.ux.skill_scroll.update(input,&self.controls);return;
        }
'''+s[idx:]
idx=s.index('        for (index, (rect, (label, enabled)))',s.index('    fn draw_skills('))
s=s[:idx]+'''        let learning=techniques.get(self.skill_technique_selection).map(|id|self.skill_learning_cost(id));
        let learn_label=match &learning {Some(Ok(cost))=>format!("Apprendre · {cost} point{}",if *cost>1{"s"}else{""}),_=>"Apprendre".to_owned()};
        UiTheme.button(Self::skill_filter_rect(layout.panel),if self.ux.skill_filter {"Disponibles [Tab]"}else{"Toutes [Tab]"},false,self.ux.skill_filter,true,ButtonTone::Secondary);
        draw_text("← → discipline · ↑ ↓ technique · Page ↓ détail",layout.panel.x+12.0,layout.panel.y+57.0,13.0,UiTheme.muted());
'''+s[idx:]
s=s.replace('("Apprendre", discipline_open && !learned),','(learn_label.as_str(), learning.as_ref().is_some_and(|r|r.is_ok())),',1)
s=s.replace('        let discipline_open = availability.is_some_and(|state| state.is_open());\n        for (index, (rect, (label, enabled)))','        for (index, (rect, (label, enabled)))',1)
Path(a).write_text(s,encoding='utf-8',newline='\n')
edit('src/controls.rs','    EventHistory,','    Inspect, "État complet et cible sélectionnée", "F4", GAME;\n    EventHistory,')
edit(a,'            } else if self.ux.history_open {','            } else if self.ux.inspected_target {\n                self.ux.inspected_target=false;\n            } else if self.ux.history_open {')
edit(a,'        if self.ux.history_open { self.update_history(input); return; }','        if self.ux.inspected_target { self.update_inspector(input); return; }\n        if self.ux.history_open { self.update_history(input); return; }')
edit(a,'        if self.ux.history_open { self.draw_history(); }','        if self.ux.history_open { self.draw_history(); }\n        if self.ux.inspected_target { self.draw_inspector(); }')
edit(a,'        if self.controls.pressed(Action::EventHistory, input) {','''        if self.controls.pressed(Action::Inspect,input) {self.ux.inspected_target=true;self.ux.help_scroll.offset=0.0;return;}
        if self.attack_aim.is_none() && self.game.player_technique_preparation().is_none() && self.route_hud_click(input,captured_at) { return; }
        if self.controls.pressed(Action::EventHistory, input) {''')
edit(a,'        let recorded = RecordedCommand::record(&command);','''        let moved = matches!(command,GameCommand::Move(_));
        let interacted = matches!(command,GameCommand::Interact{..}|GameCommand::PickUp);
        let recorded = RecordedCommand::record(&command);''')
edit(a,'            self.history.push(recorded);','            self.ux.moved |= moved;\n            self.ux.interacted |= interacted;\n            self.history.push(recorded);')
edit(a,'    fn cycle_target(&mut self) {','    fn cycle_target(&mut self) {\n        self.ux.targeted=true;')
edit(a,'        let Some(objective) = self.primary_objective() else {','''        let onboarding = if !self.test_lab && self.game.turn()<30 {
            if !self.ux.moved && self.game.turn()==0 {Some(format!("Se déplacer : {} {} {} {} · F1 : aide",self.controls.label(Action::MoveNorth),self.controls.label(Action::MoveWest),self.controls.label(Action::MoveSouth),self.controls.label(Action::MoveEast)))}
            else if !self.ux.interacted && !self.context_choices().is_empty() {Some(format!("{} : interagir à proximité",self.controls.label(Action::Interact)))}
            else if !self.ux.targeted && self.selected_target.is_none() && !self.visible_targets().is_empty(){Some(format!("{} ou clic : choisir une cible · {} : attaquer",self.controls.label(Action::CycleTarget),self.controls.label(Action::Attack)))}
            else {None}
        } else {None};
        let Some(objective) = onboarding.or_else(||self.primary_objective()) else {''')
edit('src/terminal_view.rs','"ZONE SÛRE"','"PROTECTION LOCALE"')
# Scale both columns in the same logical units and preserve enough map width.
edit('src/terminal_view.rs','bounds.x + bounds.w - 336.0,','bounds.x + bounds.w - 276.0 * ui_scale,')
edit('src/terminal_view.rs','            320.0,\n            bounds.h - 16.0,','            260.0 * ui_scale,\n            bounds.h - 16.0,')
edit('src/terminal_view.rs','bounds.w - width >= 780.0','bounds.w - width * 2.0 >= 480.0 * ui_scale')
