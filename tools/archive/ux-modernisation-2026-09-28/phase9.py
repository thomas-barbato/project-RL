from pathlib import Path
r=Path(__file__).resolve().parents[2]
p=r/'src/ascii_app.rs';s=p.read_text(encoding='utf-8')
def rep(a,b):
    global s
    assert a in s,a[:100];s=s.replace(a,b,1)
# Share the authored technique explanation between progression and quick access.
start=s.index('            let action_timing = definition.preparation_steps()')
end=s.index('            let remaining_lines =',start)
usage=s[start:end]
s=s[:start]+'            let usage = Self::technique_usage(definition);\n'+s[end:]
u=r/'src/ux.rs';us=u.read_text(encoding='utf-8')
us+='\nimpl AsciiApp {\n    pub(super) fn technique_usage(definition: &project_rl::skills::TechniqueDefinition) -> String {\n'+usage+'\nusage.replace("P1+A1", "Préparation 1 tour + action 1 tour").replace("P2+A1", "Préparation 2 tours + action 1 tour").replace("R1", "récupération 1 tour").replace("R2", "récupération 2 tours")\n}\n}\n'
u.write_text(us,encoding='utf-8')
rep('"Suspension disponible · reprise unique"','"Partie suspendue · reprendre au même endroit"')
rep('next_run.character_class = self.character_class.clone();','next_run.character_class = self.character_class.clone();\n                    next_run.active_weapon_slot=next_run.game.rules().player_starting_equipment.iter().position(Option::is_some).unwrap_or(0) as u8;')
rep('            if objective {\n                draw_text_bold(&message, 20.0, y, 14.0, UiTheme.accent());\n            } else {\n                draw_text(&message, 20.0, y, 16.0, UiTheme.muted());\n            }','            draw_wrapped_text(&message,20.0,y,self.ui_width()-40.0,1,if objective{14}else{16},if objective{UiTheme.accent()}else{UiTheme.muted()});')
# Read the selected quick technique before activation; show genuine known blockers.
needle='        if self.controls.pressed(Action::QuickTechniques, input)\n            || self.controls.pressed(Action::Learn, input)'
rep(needle,'''        if self.controls.pressed(Action::Inspect,input) {
            let id=&techniques[self.technique_menu_selection];
            if let Some(d)=self.game.rules().skills.technique(id) {
                self.ux.item_card=Some((self.technique_name(id),vec![self.texts.resolve(DISPLAY_LOCALE,d.description_key()).unwrap_or("").to_owned(),Self::technique_usage(d)]));
                self.ux.item_scroll.offset=0.0;
            }
            return;
        }
'''+needle)
rep('            self.use_quick_technique(&techniques);','''            if let Some(reason)=self.quick_technique_status(&techniques[self.technique_menu_selection]).1 {self.technique_menu_message=reason;return;}
            self.use_quick_technique(&techniques);''')
rep('"{} ou Entrée : utiliser · Échap : fermer",\n                self.controls.label(Action::QuickTechniques)','"{} : utiliser · {} : fiche complète · Échap : fermer",\n                self.controls.label(Action::Learn),self.controls.label(Action::Inspect)')
a=s.index('            let discipline = self',s.index('    fn draw_technique_menu'))
b=s.index('            draw_wrapped_text(',a)
s=s[:a]+'''            let (cost,blocked)=self.quick_technique_status(id);
            let status=blocked.as_ref().map_or(cost.clone(),|why|format!("{why} · {cost}"));
'''+s[b:]
rep('&format!("{} · {discipline}", technical_reference(id))','&status')
start=s.index('    fn draw_technique_menu')
end=s.index('    fn draw_component_selection',start)
chunk=s[start:end].replace('                    true,\n                    if index == 0', '                    index != 0 || self.quick_technique_status(&techniques[self.technique_menu_selection]).1.is_none(),\n                    if index == 0')
s=s[:start]+chunk+s[end:]
# Quest details flow vertically and scroll independently of the list.
rep('let layout = QuestJournalLayout::new(width, height, self.quest_journal_selection, count);','''let layout = QuestJournalLayout::new(width, height, self.quest_journal_selection, count);
        if input.pointer.is_some_and(|p|layout.detail_panel.contains(p.into())) && input.wheel_y!=0.0 || input.pressed.contains(&controls::Binding::key("PageDown")) || input.pressed.contains(&controls::Binding::key("PageUp")) {self.ux.quest_scroll.update(input,&self.controls);return;}
        self.ux.quest_scroll.offset=0.0;''')
a=s.index('        if let Some(entry) = entries.get(self.quest_journal_selection) {',s.index('    fn draw_quest_journal'))
b=s.index('\n        theme.button(\n            layout.close,',a)
s=s[:a]+'''        if let Some(entry)=entries.get(self.quest_journal_selection) {
            let p=layout.detail_panel;let body=Rect::new(p.x+20.0,p.y+12.0,p.w-40.0,p.h-24.0);
            crate::ui_theme::begin_text_pane(body,self.ux.quest_scroll.offset);
            let (status,color)=quest_status_presentation(entry.quest.status);
            let mut y=body.y+20.0;draw_text(status,body.x,y,14.0,color);y+=32.0;
            let title=self.texts.resolve(DISPLAY_LOCALE,&entry.quest.title_key).unwrap_or("Demande locale");
            y=draw_wrapped_text(title,body.x,y,body.w-12.0,4096,23,theme.text())+18.0;
            y=draw_wrapped_text(&self.quest_objective_text(&entry.quest.objective),body.x,y,body.w-12.0,4096,18,theme.accent())+20.0;
            y=draw_wrapped_text(&self.narrative_text(&entry.quest.summary_key,"Votre interlocuteur demande une livraison."),body.x,y,body.w-12.0,4096,16,theme.text())+22.0;
            let mut rewards=Vec::new();
            if entry.quest.reward_credits>0{rewards.push(format!("{} crédits",entry.quest.reward_credits));}
            if entry.quest.reward_experience>0{rewards.push(format!("{} XP",entry.quest.reward_experience));}
            rewards.extend(entry.quest.reward_items.iter().map(|r|format!("{} ×{}",self.item_name(&r.item),r.quantity)));
            y=draw_wrapped_text(&format!("Récompense{} · {}",if entry.quest.status==QuestStatus::Completed{" reçue"}else{""},if rewards.is_empty(){"aucune".to_owned()}else{rewards.join(" · ")}),body.x,y,body.w-12.0,4096,16,theme.text())+22.0;
            y=draw_wrapped_text(&format!("Interlocuteur · {}\\nLieu connu · {}",self.quest_giver_name(entry),entry.zone.name),body.x,y,body.w-12.0,4096,16,theme.muted())+12.0;
            crate::ui_theme::end_text_pane();self.ux.quest_scroll.finish(body,y);
        }
'''+s[b:]
rep('layout.panel.y + layout.panel.h - 24.0,\n            13.0,','layout.panel.y + layout.panel.h + 16.0,\n            13.0,')
# Clinic preview accompanies the actual quoted service.
rep('        let available = npc_service_available(&interaction);\n        theme.dialogue_action_with_icon(','''        if let Some(NpcService::Treatment{restore_amount,..})=interaction.services.first() && let Some(player)=self.game.actors().get(self.game.player_id()) {
            draw_text(format!("Après le soin : {} / {} PV",(player.integrity()+restore_amount).min(player.maximum_integrity()),player.maximum_integrity()),x+13.0,service_card.bottom()-12.0,15.0,theme.accent());
        }
        let available = npc_service_available(&interaction);
        theme.dialogue_action_with_icon(''')
# Capture new readers and all existing scenes at arbitrary supported view sizes.
rep('        match scene {\n            "lab-target"', '''        let (scene,size)=if let Some(base)=scene.strip_suffix("-small"){(base,Some((960.0,540.0,100)))}else if let Some(base)=scene.strip_suffix("-wide"){(base,Some((1920.0,1080.0,100)))}else if let Some(base)=scene.strip_suffix("-largeui"){(base,Some((1280.0,800.0,150)))}else{(scene,None)};
        match scene {
            "ux-history"=>{for i in 0..20{app.push_log(format!("Événement observé {i} · Une description longue reste consultable dans l'historique."));}app.ux.history_open=true;},
            "ux-inspect"=>{app.ux.inspected_target=true;},
            "ux-laboratory"=>{app.begin_test_lab()?;app.ux.lab_open=true;},
            "ux-item"=>{app.inventory_open=true;app.open_inventory_card();},
            "ux-help-custom"=>{app.controls.rebind(Action::Attack,controls::Binding::key("F6"))?;app.controls.rebind(Action::Legend,controls::Binding::key("F7"))?;app.legend_open=true;},
            "ux-symbols"=>{app.legend_open=true;app.ux.help_tab=1;},
            "ux-rules"=>{app.legend_open=true;app.ux.help_tab=2;},
            "lab-target"''')
rep('            _ => return Err(format!("Scène de diagnostic inconnue : {scene}")),\n        }\n        for _ in 0..3 {','''            _ => return Err(format!("Scène de diagnostic inconnue : {scene}")),
        }
        if let Some((w,h,scale))=size {app.graphics.active.ui_scale_percent=scale;request_new_screen_size(w,h);for _ in 0..10{next_frame().await;}}
        for _ in 0..3 {''')
p.write_text(s,encoding='utf-8')
