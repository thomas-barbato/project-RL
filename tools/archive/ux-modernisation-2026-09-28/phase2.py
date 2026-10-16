from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs'
s=Path(a).read_text(encoding='utf-8')
start=s.index('        let panel = Rect::new(',s.index('impl CharacterCreationLayout'))
end=s.index('        let left_width',start)
s=s[:start]+'''        let w = (width - 32.0).min(1120.0);
        let h = (height - 32.0).min(620.0);
        let panel = Rect::new((width - w) * 0.5, (height - h) * 0.5, w, h);
'''+s[end:]
s=s.replace('panel.x + panel.w - 224.0,\n                panel.y + panel.h - 61.0,\n                200.0,','panel.x + panel.w - 264.0,\n                panel.y + panel.h - 61.0,\n                240.0,',1)
s=s.replace('panel.w - left_width - 48.0,\n                37.0,','(panel.w - left_width - 48.0).min(290.0),\n                34.0,',1)
s=s.replace('''            CharacterCreationStage::Protocol => self
                .class_rows''','''            CharacterCreationStage::Protocol if self.preset.contains(point) => Some(CharacterCreationHover::Preset),
            CharacterCreationStage::Protocol => self
                .class_rows''',1)
start=s.index('    fn draw_character_creation(&self)');end=s.index('    fn draw_character(&self)',start)
s=s[:start]+Path('artifacts/ux-modernisation-2026-09-28/creation.rs').read_text(encoding='utf-8')+'\n'+s[end:]
start=s.index('    fn update_character_creation(');end=s.index('    fn adjust_creation_attribute(',start)
chunk=s[start:end]
chunk=chunk.replace('''                let previous = self.controls.pressed(Action::MenuUp, input)''','''                let previous_class = creation.selected_class;
                let previous = self.controls.pressed(Action::MenuUp, input)''',1)
needle='''                let cancel = clicked'''
idx=chunk.index(needle)
chunk=chunk[:idx]+'''                if creation.selected_class != previous_class {
                    if let Some((_, class)) = self.character_classes.iter().nth(creation.selected_class) {
                        creation.attributes = class.recommended_attributes();
                    }
                    creation.message.clear();
                }
'''+chunk[idx:]
idx=chunk.index('                let continue_requested ='); idx2=chunk.index('\n            }\n            CharacterCreationStage::Attributes',idx)
chunk=chunk[:idx]+'''                let customize = self.controls.pressed(Action::InventoryFilter, input)
                    || clicked && input.pointer.is_some_and(|p| layout.preset.contains(p.into()));
                if customize {
                    creation.stage = CharacterCreationStage::Attributes;
                    creation.message.clear();
                } else if activate || clicked && input.pointer.is_some_and(|p| layout.continue_button.contains(p.into())) {
                    if creation.attributes.total() != self.rules.primary_attribute_rules.creation_total {
                        creation.stage = CharacterCreationStage::Attributes;
                        creation.message = "Répartissez les points restants pour commencer.".to_owned();
                    } else {
                        match self.rebuild_run_with_character_class(&creation) {
                            Ok(()) => return,
                            Err(error) => { eprintln!("[NEW RUN] {error}"); creation.message = "Impossible de commencer cette partie pour le moment.".to_owned(); }
                        }
                    }
                }'''+chunk[idx2:]
chunk=chunk.replace('''                    if start {
                        match''','''                    if start && creation.attributes.total() != self.rules.primary_attribute_rules.creation_total {
                        creation.message = "Répartissez les points restants pour commencer.".to_owned();
                    } else if start {
                        match''')
s=s[:start]+chunk+s[end:]
s=s.replace('app.update_input(&input("Down")); // BRÈCHE -> CREUSET\n        app.update_input(&input("Enter"));','app.update_input(&input("Down")); // BRÈCHE -> CREUSET\n        app.update_input(&input("Tab"));',1)
# Keep generated cold attributes scene in the optional customization flow.
Path(a).write_text(s,encoding='utf-8',newline='\n')
