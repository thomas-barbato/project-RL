from pathlib import Path
r=Path(__file__).resolve().parents[2]
p=r/'src/ascii_app.rs';s=p.read_text(encoding='utf-8')
def rep(a,b):
    global s
    assert a in s,a[:100];s=s.replace(a,b,1)
for name in ['resistance_percentage','primary_attribute_summary','draw_compact_stat','draw_control_hint','control_hint_widths','draw_recommended_profile','draw_compact_metric']:
    import re
    m=re.search(r'^(?:const )?fn '+name+r'\(',s,re.M);assert m
    end=s.index('\n}',m.start())+2
    s=s[:m.start()]+s[end:]
# A dedicated quick-menu detail action works with both input methods.
rep('struct TechniqueQuickMenuLayout {\n    panel: Rect,\n    rows: Vec<(usize, Rect)>,\n    actions: [Rect; 2],','struct TechniqueQuickMenuLayout {\n    panel: Rect,\n    rows: Vec<(usize, Rect)>,\n    actions: [Rect; 3],')
a=s.index('        let button_width =',s.index('impl TechniqueQuickMenuLayout'))
b=s.index('        Self {',a)
s=s[:a]+'''        let button_width=(panel.w-38.0)/3.0;
        let actions=std::array::from_fn(|i|Rect::new(panel.x+12.0+i as f32*(button_width+7.0),panel.bottom()-49.0,button_width,35.0));
'''+s[b:]
a=s.index('    fn update_technique_menu');b=s.index('    fn update_skills',a)
c=s[a:b].replace('clicked && hovered_action == Some(1)','clicked && hovered_action == Some(2)').replace('if self.controls.pressed(Action::Inspect,input) {','if self.controls.pressed(Action::Inspect,input) || clicked && hovered_action==Some(1) {')
s=s[:a]+c+s[b:]
rep('.zip(["UTILISER", "ANNULER"])','.zip(["Utiliser", "Fiche complète", "Fermer"])')
# Laboratory and resume diagnostic affordances use the actual bindings.
rep('        if menu_error {\n            let area = layout.main_error(height);','''        if menu_error {
            let area = layout.main_error(height);''')
rep('                error_title,\n                area.x + 12.0,','                &format!("{error_title} · Diagnostic [{}]",self.controls.label(Action::Inspect)),\n                area.x + 12.0,')
rep('if self.menu==MenuScreen::Main && !self.ux.resume_error.is_empty() && self.controls.pressed(Action::Inspect,input)', 'if self.menu==MenuScreen::Main && !self.ux.resume_error.is_empty() && (self.controls.pressed(Action::Inspect,input) || input.pressed.contains(&controls::Binding::MouseLeft) && input.pointer.is_some_and(|p|MenuLayout::for_screen(self.menu,input.viewport.unwrap_or((1280.0,800.0)).0,input.viewport.unwrap_or((1280.0,800.0)).1,self.menu_labels().len()).main_error(input.viewport.unwrap_or((1280.0,800.0)).1).contains(p.into())))')
p.write_text(s,encoding='utf-8')
u=r/'src/ux.rs';s=u.read_text(encoding='utf-8')
s=s.replace('"Caractéristiques connues avant achat"','"Informations connues · lecture complète"')
s=s.replace('    pub(super) fn draw_hud_shortcuts(&self) {','''    pub(super) fn lab_button(width:f32,height:f32)->Rect {Rect::new(width-236.0,height-144.0,224.0,36.0)}
    pub(super) fn draw_hud_shortcuts(&self) {
        if self.test_lab {UiTheme.button(Self::lab_button(self.ui_width(),self.ui_height()),&format!("Essais [{}]",self.controls.label(Action::Laboratory)),self.menu_focus.hovered==Some(50_030),false,true,ButtonTone::Secondary);}''')
s=s.replace('        for (rect,action,focus) in [(self.ux.hud_points.get()', '''        if self.test_lab && input.pointer.is_some_and(|p|Self::lab_button(w,h).contains(p.into())) {self.menu_focus.hovered=Some(50_030);if clicked{self.ux.lab_open=true;return true;}}
        for (rect,action,focus) in [(self.ux.hud_points.get()''')
u.write_text(s,encoding='utf-8')
p=r/'src/narrative_app.rs';s=p.read_text(encoding='utf-8')
a='        let count = dialogue.choices.len();'
i=s.index(a,s.index('    pub(super) fn update_narrative_dialogue'))
s=s[:i]+'''        let read=Rect::new(layout.panel.right()-176.0,layout.panel.y+181.0,154.0,29.0);
        if self.controls.pressed(Action::Inspect,input) || clicked && input.pointer.is_some_and(|p|read.contains(p.into())) {
            let mut lines=vec![self.narrative_text(&dialogue.text_key,"Texte absent")];
            lines.extend(dialogue.choices.iter().map(|c|self.texts.resolve(DISPLAY_LOCALE,&c.text_key).unwrap_or("Choix indisponible").to_owned()));
            self.ux.item_card=Some((self.texts.resolve(DISPLAY_LOCALE,&dialogue.name_key).unwrap_or("Conversation").to_owned(),lines));self.ux.item_scroll.offset=0.0;return;
        }
'''+s[i:]
start=s.index('    pub(super) fn draw_narrative_dialogue')
s=s[:start]+s[start:].replace('            6,\n            16,','            3,\n            16,',1).replace('        let selection = self','''        UiTheme.button(Rect::new(layout.panel.right()-176.0,layout.panel.y+181.0,154.0,29.0),&format!("Lire tout [{}]",self.controls.label(Action::Inspect)),false,false,true,ButtonTone::Secondary);
        let selection = self''',1)
p.write_text(s,encoding='utf-8')
