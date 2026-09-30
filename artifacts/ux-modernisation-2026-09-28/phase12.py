from pathlib import Path
r=Path(__file__).resolve().parents[2];p=r/'src/ascii_app.rs';s=p.read_text(encoding='utf-8')
a=s.index('        let capacity = if layout.stats_panel.is_some()',s.index('    fn draw_inventory'))
b=s.index('\n        for (index, (rect, filter))',a)
s=s[:a]+'''        let capacity=format!("{:02}/{:02} places",inventory.len(),inventory.capacity());
        let right=layout.character_details.filter(|_|layout.stats_panel.is_none()).map_or(margin+width-20.0,|r|r.x-10.0);
        let left=32.0+layout.left_width+169.0;
        draw_wrapped_text(&capacity,left,top+35.0,(right-left).max(40.0),1,15,muted);
'''+s[b:]
s=s.replace('"Personnage [J]",','&format!("Personnage [{}]",self.controls.label(Action::Character)),',1)
s=s.replace('Color::from_rgba(27, 57, 83, 245)','UiTheme.surface_selected()',1).replace('Color::from_rgba(123, 181, 225, 255)','UiTheme.accent()',1)
s=s.replace('''            draw_rectangle(
                section.x,
                section.y,
                section.w,
                section.h,
                UiTheme.surface(),
            );
            draw_rectangle_lines(section.x, section.y, section.w, section.h, 1.0, muted);''','            UiTheme.card(section,false);',1)
s=s.replace('format!("NIVEAU {}", progression.level()),','format!("NIVEAU {} · COMPÉTENCES [{}]", progression.level(),self.controls.label(Action::Skills)),',1)
key='        } else if let Some(creation) = &app.character_creation {'
idx=s.index(key,s.index('let mut probes = if app.resume_requested'))
s=s[:idx]+'''        } else if app.ux.item_card.is_some() || app.ux.history_open || app.ux.inspected_target || app.ux.lab_open || app.game.status()!=RunStatus::Active {
            let p=ux::reader_panel(app.ui_width(),app.ui_height());vec![Rect::new(p.x+12.0,p.y+10.0,p.w-24.0,44.0)]
'''+s[idx:].replace(key,'        } else if let Some(creation) = &app.character_creation {',1)
# Drop the redundant brace introduced where the previous branch ends.
s=s.replace('vec![Rect::new(p.x+12.0,p.y+10.0,p.w-24.0,44.0)]\n        } else','vec![Rect::new(p.x+12.0,p.y+10.0,p.w-24.0,44.0)]\n        } else',1)
s=s.replace('vec![Rect::new(6.0, 10.0, 300.0, 30.0)]','vec![if terminal_status_panel(app.terminal_bounds(),app.ui_scale()).is_some(){Rect::new(30.0,140.0,260.0,44.0)}else{Rect::new(6.0,10.0,300.0,46.0)}]',1)
s=s.replace('[COLD UI] {scene} : contraste OK','[COLD UI] {scene} : texte présent dans les zones vérifiées',1)
p.write_text(s,encoding='utf-8')
p=r/'src/ux.rs';s=p.read_text(encoding='utf-8');a=s.index('            if rect.w >= 100.0 {');b=s.index('            if hovered {',a)
s=s[:a]+'''            if rect.w>=100.0 {
                draw_wrapped_text(label,rect.x+28.0,rect.y+14.0,rect.w-34.0,1,12,UiTheme.text());
                draw_wrapped_text(&key,rect.x+28.0,rect.y+28.0,rect.w-34.0,1,11,UiTheme.accent());
            } else {draw_wrapped_text(&key,rect.x+27.0,rect.y+23.0,rect.w-32.0,1,13,UiTheme.text());}
'''+s[b:];p.write_text(s,encoding='utf-8')
