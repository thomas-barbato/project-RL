from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs'
edit(a,'mod effect_feedback;','mod effect_feedback;\n#[path = "ux.rs"]\nmod ux;')
edit(a,'pub struct AsciiApp {','pub struct AsciiApp {\n    ux: ux::UxState,')
edit(a,'            legend_open: false,','            legend_open: false,\n            ux: ux::UxState::default(),')
edit(a,'UiIcon::Shield','UiIcon::Armor')
edit(a,'    pub fn draw(&self) {','    pub fn draw(&self) {\n        crate::ui_theme::set_high_contrast(self.graphics.active.high_contrast);')
edit(a,'                legend_open: self.legend_open,','                legend_open: false,')
edit(a,'        if self.inventory_open {\n            self.draw_inventory();','        if self.legend_open { self.draw_help(); }\n        if self.ux.history_open { self.draw_history(); }\n        if self.inventory_open {\n            self.draw_inventory();')
edit(a,'            } else if self.legend_open {','            } else if self.ux.history_open {\n                self.ux.history_open = false;\n            } else if self.legend_open {')
edit(a,'        if self.context_menu.is_some() {\n            self.update_context_menu(input);','        if self.ux.history_open { self.update_history(input); return; }\n        if self.context_menu.is_some() {\n            self.update_context_menu(input);')
old='''        if self.legend_open {
            if input.pointer.is_some() {
                self.menu_focus.hovered = Some(0);
            }
            if input.pressed.contains(&controls::Binding::MouseLeft) {
                self.legend_open = false;
            }
            return;
        }'''
edit(a,old,'        if self.legend_open { self.update_help(input); return; }')
edit(a,'        if self.controls.pressed(Action::Restart, input) {','''        if self.controls.pressed(Action::EventHistory, input) {
            self.ux.history_open = true;
            self.ux.history_scroll.offset = 0.0;
            return;
        }
        if self.controls.pressed(Action::Restart, input) {''')
edit('src/controls.rs','    Legend, "Afficher / masquer la légende", "F1", GAME;','    Legend, "Ouvrir / fermer l’aide", "F1", GAME;\n    EventHistory, "Historique des tours", "V", GAME;')
edit(a,'        self.log.push(message);','''        self.ux.history.push((self.game.turn(), message.clone()));
        if self.ux.history.len() > 600 { self.ux.history.remove(0); }
        self.log.push(message);''')
edit(a,'    if maximum_lines == 0 || text.is_empty() {','''    let maximum_lines = if crate::ui_theme::text_pane_active() { 4096 } else { maximum_lines.min(4096) };
    if maximum_lines == 0 || text.is_empty() {''')
edit(a,'            self.open_level_up_screen(notice);','''            self.level_up_notice = Some(notice);
            self.push_log(format!("Niveau {} atteint · {} points disponibles · {} pour les dépenser quand vous le souhaitez.", notice.level, self.game.player_progression().unspent_skill_points(), self.controls.label(Action::Skills)));''')
edit(a,'format!("Interagir : {name} ({}, {})", position.x, position.y)','format!("Interagir : {name} · {}", self.game.player_position().map_or("à proximité", |origin| approximate_direction(origin, position)))')
# Pointer focus must not cancel keyboard navigation when stationary.
edit(a,'''        if let Some(index) = hovered {
            menu.selected = index;
        }
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);''','''        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        self.menu_focus.update(hovered, &mut menu.selected, clicked,
            self.controls.pressed(Action::MenuUp,input) || self.controls.pressed(Action::MenuDown,input));''')
# Full dossier text uses the same line scrolling as the help and history.
s=Path(a).read_text(encoding='utf-8');start=s.index('        if self.report_open {\n            let dossier_line_count');end=s.index('        if self.controls.pressed(Action::Report, input)',start+len('        if self.report_open {'))
# Find the outer following report action, not the close condition inside the block.
end=s.index('        if self.controls.pressed(Action::Report, input) {',start)
s=s[:start]+'''        if self.report_open {
            let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
            let hovered=input.pointer.is_some_and(|p|ux::reader_close(ux::reader_panel(w,h)).contains(p.into()));
            self.menu_focus.hovered=hovered.then_some(0);
            if self.controls.pressed(Action::Report,input) || input.pressed.contains(&controls::Binding::MouseLeft) && hovered { self.report_open=false; }
            self.ux.dossier_scroll.update(input,&self.controls);
            return;
        }
'''+s[end:]
start=s.index('    fn draw_dossier(&self) {');end=s.index('\n}\n\n#[allow(clippy::too_many_arguments)]',start)
s=s[:start]+'''    fn draw_dossier(&self) {
        let panel=ux::reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());
        ux::draw_reader("Dossier de terrain", "Relevés et archives découverts · N : journal des quêtes",panel,self.menu_focus.hovered==Some(0));
        let body=ux::reader_body(panel);
        crate::ui_theme::begin_text_pane(body,self.ux.dossier_scroll.offset);
        let mut y=body.y+24.0;
        for line in self.dossier_lines() {
            let section=matches!(line.kind,DossierLineKind::Section);
            if section { y+=14.0; }
            y=draw_wrapped_text(&line.text,body.x,y,body.w-20.0,4096,if section {18}else{17},if section{UiTheme.accent()}else{UiTheme.text()})+18.0;
        }
        crate::ui_theme::end_text_pane();
        self.ux.dossier_scroll.finish(body,y);
    }
'''+s[end:]
Path(a).write_text(s,encoding='utf-8',newline='\n')

# Reuse the complete authored symbol catalogue, preserving glyph colours and icons.
p=Path('src/terminal_view.rs');s=p.read_text(encoding='utf-8');start=s.index('    let entity_legend =',s.index('fn draw_legend_overlay'));end=s.index('    let available =',start)
entities=s[start:end];start=s.index('    let terrain_legend =',end);end=s.index('    let terrain_rows',start);terrain=s[start:end]
func='''
pub(crate) fn draw_symbol_guide(game: &GameState, bounds: Rect, scroll: f32) -> f32 {
    let cyan = UiTheme.accent();
'''+entities+terrain+'''
    let row_h = 34.0;
    let mut index = 0;
    for (symbol,label,color,alerted,status_icon) in entity_legend {
        let y = bounds.y + 24.0 + index as f32 * row_h - scroll;
        if y - 20.0 >= bounds.y && y + 5.0 <= bounds.bottom() {
            draw_entity(Rect::new(bounds.x,y-18.0,22.0,22.0),symbol,TerminalGlyphPalette::new(color,None,None),false,alerted,status_icon);
            draw_ui_text(label,bounds.x+38.0,y,16.0,UiTheme.text());
        }
        index += 1;
    }
    for (decor,label) in terrain_legend {
        let y = bounds.y + 24.0 + index as f32 * row_h - scroll;
        if y - 20.0 >= bounds.y && y + 5.0 <= bounds.bottom() {
            draw_tile(Rect::new(bounds.x,y-18.0,22.0,22.0),decor,[false;4],true,GridPos::new(0,0));
            draw_ui_text(label,bounds.x+38.0,y,16.0,UiTheme.text());
        }
        index += 1;
    }
    bounds.y + 24.0 + index as f32 * row_h
}
'''
s+='\n'+func;p.write_text(s,encoding='utf-8',newline='\n')
