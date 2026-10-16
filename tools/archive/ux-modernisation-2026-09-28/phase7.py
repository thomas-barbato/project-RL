from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs';s=Path(a).read_text(encoding='utf-8')
start=s.index('            theme.main_menu_action(',s.index('    fn draw_main_menu'));end=s.index('            );',start)
s=s[:end]+s[end:].replace('            );','                index == if self.has_resume_data() {0}else{1},\n            );',1)
start=s.index('        for (index, (label, rect))',s.index('    fn draw_main_menu'));idx=s.index('            let selected =',start)
s=s[:idx]+'''            if index==0 && !enabled {draw_text("Aucune partie suspendue",rect.x+18.0,rect.y+rect.h*0.7,14.0,theme.muted());continue;}
'''+s[idx:]
s=s.replace('"Rendu : Terminal à glyphes (textures à venir)"','"Rendu : Terminal à glyphes"')
start=s.index('        for (index, (label, rect))',s.index('    fn draw_menu'));idx=s.index('            theme.button_with_icon(',start)
s=s[:idx]+'''            if self.menu==MenuScreen::Graphics && index<=6 {
                let toggle=match index{5=>Some(self.graphics.draft.high_contrast),6=>Some(self.graphics.draft.reduced_motion),_=>None};
                theme.setting_row(*rect,label,selected,enabled,toggle);continue;
            }
'''+s[idx:]
# Directional controls share the painted arrow positions.
start=s.index('        if ((clicked && hovered.is_some()) || activate)',s.index('    fn update_menu'));idx=s.index('            match (self.menu, self.menu_selection)',start)
s=s[:idx]+'''            if self.menu==MenuScreen::Graphics && matches!(self.menu_selection,0..=2|4..=6) {
                let rect=MenuLayout::for_screen(self.menu,input.viewport.unwrap_or((1280.0,800.0)).0,input.viewport.unwrap_or((1280.0,800.0)).1,count).buttons[self.menu_selection];
                let backwards=clicked && input.pointer.is_some_and(|p|p.0>=rect.right()-77.0 && p.0<rect.right()-35.0);
                self.graphics.draft.cycle(self.menu_selection,!backwards);return;
            }
'''+s[idx:]
# Grouped controls keep original actions and persistence; only the displayed mapping changes.
for fn,endfn in [('update_options','save_controls'),('draw_options','dossier_available')]:
    start=s.index('    fn '+fn+'(');end=s.index('    fn '+endfn+'(',start)
    chunk=s[start:end].replace('Action::ALL','actions')
    idx=chunk.index('{\n')+2;chunk=chunk[:idx]+'        let actions=self.control_actions();\n'+chunk[idx:]
    if fn=='update_options':
        idx=chunk.index('        let clicked =')
        chunk=chunk[:idx]+'''        let (w,_)=input.viewport.unwrap_or((1280.0,800.0));
        if input.pressed.contains(&controls::Binding::MouseLeft) {
            if let Some(group)=(0..6).find(|i|input.pointer.is_some_and(|p|Self::control_tab(w,*i).contains(p.into()))) {self.ux.controls_group=group;self.options_selection=0;self.options_scroll=0;return;}
        }
'''+chunk[idx:]
    else:
        start_hint=chunk.index('        draw_wrapped_text(\n            "Disposition et raccourcis');end_hint=chunk.index('        for index in layout.first',start_hint)
        chunk=chunk[:start_hint]+'''        for (index,label) in ["Tout","Déplacement","Combat","Interfaces","Navigation","Essais"].into_iter().enumerate(){UiTheme.button(Self::control_tab(self.ui_width(),index),label,false,self.ux.controls_group==index,true,ButtonTone::Secondary);}
'''+chunk[end_hint:]
        chunk=chunk.replace('YELLOW','UiTheme.accent()').replace('SKYBLUE','UiTheme.accent()').replace('LIGHTGRAY','UiTheme.text()')
    s=s[:start]+chunk+s[end:]
# Modal laboratory and keyboard companion command.
s=s.replace('            } else if self.ux.item_card.is_some() {','            } else if self.ux.lab_open {\n                self.ux.lab_open=false;\n            } else if self.ux.item_card.is_some() {',1)
s=s.replace('        if self.ux.item_card.is_some() {self.update_item_card(input);return;}','        if self.ux.lab_open {self.update_lab_menu(input);return;}\n        if self.ux.item_card.is_some() {self.update_item_card(input);return;}',1)
s=s.replace('        if self.ux.item_card.is_some() {self.draw_item_card();}','        if self.ux.item_card.is_some() {self.draw_item_card();}\n        if self.ux.lab_open {self.draw_lab_menu();}',1)
s=s.replace('        if self.controls.pressed(Action::Inspect,input)','        if self.test_lab && self.controls.pressed(Action::Laboratory,input){self.ux.lab_open=true;return;}\n        if self.controls.pressed(Action::CompanionOrder,input) && self.has_controlled_companion(){self.cycle_companion_order();return;}\n        if self.controls.pressed(Action::Inspect,input)',1)
# The current order is always readable; keys use configured bindings.
start=s.index('        let title = if companions.len()',s.index('    fn draw_companion_bar'));end=s.index('        draw_text_bold(',start)
s=s[:start]+'''        let title=format!("Drone{} · {} [{}]",if companions.len()>1{"s"}else{""},active_behavior.map_or("Ordre spécial",companion_behavior_label),self.controls.label(Action::CompanionOrder));
'''+s[end:]
# More useful empty area targeting feedback; legality is unchanged.
s=s.replace('''"{} case(s) couvertes · {} cible(s)",
                    footprint.as_ref().map_or(0, |area| area.cells().len()),
                    affected''','''"{} case(s) · {}",
                    footprint.as_ref().map_or(0, |area| area.cells().len()),
                    if affected==0 {"Aucune cible dans la zone".to_owned()}else{format!("{affected} cible(s)")}''',1)
# Details for a failed resume are inspectable, without exposing them as main copy.
s=s.replace('''        eprintln!("[SUSPENSION] Resume failed: {error}");''','''        eprintln!("[SUSPENSION] Resume failed: {error}");
        self.ux.resume_error=error;''',1)
start=s.index('    fn update_menu');idx=s.index('        let count =',start)
s=s[:idx]+'''        if self.menu==MenuScreen::Main && !self.ux.resume_error.is_empty() && self.controls.pressed(Action::Inspect,input) {self.ux.item_card=Some(("Diagnostic de reprise".to_owned(),vec![self.ux.resume_error.clone(),"La sauvegarde est conservée. Vous pouvez fermer ce diagnostic et réessayer après correction du problème.".to_owned()]));return;}
'''+s[idx:]
# Reader input comes before menu input when opened above it.
needle='        if self.menu == MenuScreen::Controls {\n            self.update_options(input);'
s=s.replace(needle,'        if self.ux.item_card.is_some() {self.update_item_card(input);return;}\n'+needle,1)
s=s.replace('            } else if self.menu != MenuScreen::Hidden {','            } else if self.ux.item_card.is_some() {\n                self.ux.item_card=None;\n            } else if self.menu != MenuScreen::Hidden {',1)
Path(a).write_text(s,encoding='utf-8',newline='\n')
edit('src/controls.rs','    Inspect,','    CompanionOrder, "Compagnons : ordre suivant", "F3", GAME;\n    Laboratory, "Laboratoire : choisir un essai", "F5", GAME;\n    Inspect,')
edit('src/pause_menu.rs','                139.0,','                146.0,')
edit('src/terminal_view.rs','&format!("POSITION · {}", local_coordinates(game.player_position()))','"Votre position sur le relevé"')
edit('src/terminal_view.rs','''            format!(
                "POSITION · {} · {description}",
                local_coordinates(game.player_position())
            )''','''            description''')
