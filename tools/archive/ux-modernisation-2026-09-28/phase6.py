from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs';u='src/ux.rs';s=Path(u).read_text(encoding='utf-8')
start=s.index('            vec![',s.index('let lines = if self.ux.help_tab == 0'));end=s.index('\n        } else {',start)
lines=s[start:end]
idx=s.index('    pub(super) fn update_help')
s=s[:idx]+'''    pub(super) fn help_command_lines(&self) -> Vec<String> {
        let mut lines='''+lines.strip()+''';
        lines.push(format!("Aide : {} · Inspection de l'état et de la cible : {}.",self.controls.label(Action::Legend),self.controls.label(Action::Inspect)));
        lines.push(format!("Navigation de l'aide : {} / {} pour les onglets ; {} / {} pour défiler. Molette et Page préc./suiv. sont également disponibles.",self.controls.label(Action::MenuLeft),self.controls.label(Action::MenuRight),self.controls.label(Action::MenuUp),self.controls.label(Action::MenuDown)));
        for action in [Action::Slot1,Action::Slot2,Action::Slot3,Action::Analyze,Action::Traces,Action::Walls,Action::Threat,Action::Multiple,Action::NpcVision,Action::Restart] {lines.push(format!("{} · {}",action.name(),self.controls.label(action)));}
        lines
    }

'''+s[idx:]
start=s.index('            vec![',s.index('let lines = if self.ux.help_tab == 0'));end=s.index('\n        } else {',start)
s=s[:start]+'            self.help_command_lines()'+s[end:]
s=s.replace('hovered: bool) {\n    UiTheme.panel(panel);','hovered: bool, controls: &Controls) {\n    UiTheme.panel(panel);')
s=s.replace('draw_text_bold(title, panel.x + 24.0, panel.y + 43.0, 26.0, UiTheme.text());','draw_wrapped_text(title, panel.x + 24.0, panel.y + 43.0, panel.w - 168.0, 1, 26, UiTheme.text());')
s=s.replace('draw_text("Molette / flèches / Page préc. suiv. · Échap : fermer",','draw_text(format!("Molette / {} {} / Page préc. suiv. · Échap : fermer",controls.label(Action::MenuUp),controls.label(Action::MenuDown)),')
s='\n'.join(line.replace(');',', &self.controls);') if 'draw_reader(' in line and not 'fn draw_reader' in line else line for line in s.split('\n'))
Path(u).write_text(s,encoding='utf-8',newline='\n')
s=Path(a).read_text(encoding='utf-8')
s='\n'.join(line.replace(');',', &self.controls);') if 'ux::draw_reader(' in line else line for line in s.split('\n'))
s=s.replace('"Se déplacer : {} {} {} {} · F1 : aide",self.controls.label(Action::MoveNorth),self.controls.label(Action::MoveWest),self.controls.label(Action::MoveSouth),self.controls.label(Action::MoveEast)','"Se déplacer : {} {} {} {} · {} : aide",self.controls.label(Action::MoveNorth),self.controls.label(Action::MoveWest),self.controls.label(Action::MoveSouth),self.controls.label(Action::MoveEast),self.controls.label(Action::Legend)')
start=s.index('    fn draw_character_creation');end=s.index('    fn draw_creation_preview',start)
chunk=s[start:end].replace('draw_rectangle(0.0, 0.0, self.ui_width(), self.ui_height(), theme.backdrop());','draw_rectangle(0.0, 0.0, self.ui_width(), self.ui_height(), Color::new(0.012,0.026,0.037,1.0));')
chunk=chunk.replace('theme.button(layout.preset, if customizing { "Rétablir le profil recommandé" } else { "Personnaliser les attributs [Tab]" },','theme.button(layout.preset, &if customizing { "Rétablir le profil recommandé".to_owned() } else { format!("Personnaliser les attributs [{}]",self.controls.label(Action::InventoryFilter)) },')
s=s[:start]+chunk+s[end:]
# The character details are a continuous readable document, not cramped metric tiles.
start=s.index('        let attribute_card =',s.index('    fn draw_character('));end=s.index('        let actor =',start)
s=s[:start]+'''        let pane=Rect::new(right_x,top+72.0,right_width,height-130.0);
        theme.card(pane,false);
        crate::ui_theme::begin_text_pane(pane,self.ux.character_scroll.offset);
        let mut detail_y=pane.y+30.0;
        draw_text_bold(format!("{} · effets",primary_attribute_label(selected_attribute)),right_x+12.0,detail_y,19.0,theme.accent());
        detail_y=draw_wrapped_text(primary_attribute_description(selected_attribute),right_x+12.0,detail_y+28.0,right_width-28.0,4096,16,theme.text())+20.0;
        let base=self.game.player_primary_attributes().map_or(0,|a|a.value(selected_attribute));
        let bonus=self.game.player_attribute_bonus(selected_attribute);
        let current=attributes.map_or(0,|a|a.value(selected_attribute));
        detail_y=draw_wrapped_text(&format!("Base {base} · Équipement {bonus:+} · Valeur actuelle {current}"),right_x+12.0,detail_y,right_width-28.0,4096,16,theme.accent())+30.0;
'''+s[end:]
start=s.index('        let state_card =',start);end=s.index('        let state_metrics =',start)
s=s[:start]+'''        draw_text_bold("État actuel",right_x+12.0,detail_y,19.0,theme.text());detail_y+=30.0;
'''+s[end:]
start=s.index('        for (index, (label, value)) in state_metrics',start);end=s.index('        let active_weapon',start)
s=s[:start]+'''        for (label,value) in state_metrics { detail_y=draw_wrapped_text(&format!("{label} · {value}"),right_x+12.0,detail_y,right_width-28.0,4096,16,theme.text())+8.0; }
        detail_y+=24.0;
'''+s[end:]
start=s.index('        let combat_card =',start);end=s.index('        let combat_metrics =',start)
s=s[:start]+'''        detail_y=draw_wrapped_text(&format!("Combat · canal {} · {weapon_name}",self.active_weapon_slot+1),right_x+12.0,detail_y,right_width-28.0,4096,19,theme.accent())+16.0;
'''+s[end:]
start=s.index('        for (index, (label, value)) in combat_metrics',start);end=s.index('        for (index, (rect, label))',start)
s=s[:start]+'''        for (label,value) in combat_metrics {detail_y=draw_wrapped_text(&format!("{label} · {value}"),right_x+12.0,detail_y,right_width-28.0,4096,16,theme.text())+12.0;}
        detail_y=draw_wrapped_text("La précision affichée est une référence sans cible. La situation et la cible modifient la probabilité réelle.",right_x+12.0,detail_y+12.0,right_width-28.0,4096,15,theme.muted());
        crate::ui_theme::end_text_pane();self.ux.character_scroll.finish(pane,detail_y);

'''+s[end:]
start=s.index('        if self.character_open {\n            let (width, height)');idx=s.index('            let hovered_attribute',start)
s=s[:idx]+'''            if input.wheel_y!=0.0 || input.pressed.contains(&controls::Binding::key("PageDown")) || input.pressed.contains(&controls::Binding::key("PageUp")) {self.ux.character_scroll.update(input,&self.controls);return;}
'''+s[idx:]
# Replace the wide status hierarchy; all defenses remain one click away.
start=s.index('        draw_text_bold("NIVEAU"',s.index('    fn draw_player_status_panel'));end=s.index('\n    fn draw_header',start)
s=s[:start]+'''        let mut y=panel.y+16.0;
        draw_status_bar(Rect::new(x,y,width,45.0),"PV",&format!("{}/{}",player.integrity(),player.maximum_integrity()),normalized_ratio(player.integrity(),player.maximum_integrity()),UiIcon::Health,if player.integrity()*3<player.maximum_integrity(){theme.danger()}else{theme.success()});y+=58.0;
        let energy=self.game.player_energy();
        draw_status_bar(Rect::new(x,y,width,40.0),"ÉNERGIE",&format!("{}/{}",energy.available(),energy.capacity()),normalized_ratio(energy.available(),energy.capacity()),UiIcon::Energy,theme.accent());y+=61.0;
        draw_text_bold("Arme en main",x,y,15.0,theme.muted());
        y=draw_wrapped_text(&active_weapon,x,y+26.0,width,2,17,theme.text())+9.0;
        if self.has_active_supply_gauge(){self.draw_active_supply_gauge(Rect::new(x,y,width,27.0));y+=40.0;}
        else if let Some(matter)=self.game.player_matter(){draw_text(format!("Munitions · {matter}"),x,y+18.0,16.0,theme.text());y+=34.0;}
        if let Some(ready)=self.active_effect_readiness(){y=draw_wrapped_text(&ready,x,y+16.0,width,2,14,theme.accent())+10.0;}
        if let Some(b)=self.game.player_bandwidth(){draw_text(format!("Bande passante · {}/{}",b.available(),b.capacity()),x,y+20.0,15.0,theme.text());y+=30.0;}
        if let Some(h)=self.game.player_heat(){draw_text(format!("Chaleur · {}/{}",h.current(),h.critical_threshold()),x,y+20.0,15.0,if h.current()>=h.alert_threshold(){theme.danger()}else{theme.muted()});}
        let defenses=Rect::new(x,panel.bottom()-142.0,width,34.0);
        theme.button(defenses,"Défenses et réserves",self.menu_focus.hovered==Some(50_021),false,true,ButtonTone::Secondary);self.ux.hud_defenses.set(Some(defenses));
        let points=Rect::new(x,panel.bottom()-98.0,width,34.0);
        let available=progression.unspent_skill_points();
        theme.button(points,&format!("{available} point{} à dépenser",if available>1{"s"}else{""}),self.menu_focus.hovered==Some(50_020),false,true,if available>0{ButtonTone::Primary}else{ButtonTone::Secondary});self.ux.hud_points.set(Some(points));
        draw_text(format!("Niveau {} · {} XP",progression.level(),progression.experience()),x,panel.bottom()-27.0,15.0,theme.muted());
    }
'''+s[end:]
s=s.replace('        let vertical_scale = ((panel.h - 24.0) / 672.0).clamp(0.65, 1.0);\n        let panel_y = |offset: f32| panel.y + offset * vertical_scale;\n','',1)
s=s.replace('        self.draw_player_status_panel();','        self.draw_player_status_panel();',1)
# Refresh hit rectangles every rendered frame, including compact mode.
s=s.replace('        clear_background(Color::from_rgba(5, 8, 12, 255));','        self.ux.hud_points.set(None);self.ux.hud_defenses.set(None);\n        clear_background(Color::from_rgba(5, 8, 12, 255));',1)
start=s.index('        if terminal_status_panel(self.terminal_bounds(), self.ui_scale()).is_none() {',s.index('    fn draw_header'))
idx=s.index('            let progression =',start)
s=s[:idx]+'''            self.ux.hud_points.set(Some(Rect::new(12.0,7.0,(self.ui_width()-36.0)/3.0,58.0)));
'''+s[idx:]
Path(a).write_text(s,encoding='utf-8',newline='\n')
s=Path(u).read_text(encoding='utf-8');idx=s.index('        for (index,(rect,action,_,_))',s.index('fn route_hud_click'))
s=s[:idx]+'''        for (rect,action,focus) in [(self.ux.hud_points.get(),Action::Skills,50_020),(self.ux.hud_defenses.get(),Action::Inspect,50_021)] {
            if rect.is_some_and(|r|input.pointer.is_some_and(|p|r.contains(p.into()))) {
                self.menu_focus.hovered=Some(focus);
                if clicked {let forwarded=InputFrame{pressed:[self.controls.binding(action).clone()].into(),viewport:input.viewport,..Default::default()};self.dispatch_input_at(&forwarded,captured_at);return true;}
            }
        }
'''+s[idx:];Path(u).write_text(s,encoding='utf-8',newline='\n')
