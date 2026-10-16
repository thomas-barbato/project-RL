from pathlib import Path
exec(Path('artifacts/ux-modernisation-2026-09-28/phase1.py').read_text(encoding='utf-8').split("a='src/ascii_app.rs'")[0])
a='src/ascii_app.rs'
edit(a,'            } else if self.ux.inspected_target {','            } else if self.ux.item_card.is_some() {\n                self.ux.item_card=None;\n            } else if self.ux.inspected_target {')
edit(a,'        if self.ux.inspected_target { self.update_inspector(input); return; }','        if self.ux.item_card.is_some() {self.update_item_card(input);return;}\n        if self.ux.inspected_target { self.update_inspector(input); return; }')
edit(a,'        set_default_camera();\n    }\n\n    fn open_quest_journal','        if self.ux.item_card.is_some() {self.draw_item_card();}\n        set_default_camera();\n    }\n\n    fn open_quest_journal')
edit(a,'        if self.game.status() != RunStatus::Active {\n            return;\n        }\n\n        let (width, height) = input.viewport','        if self.game.status() != RunStatus::Active {\n            self.update_end_screen(input);\n            return;\n        }\n\n        let (width, height) = input.viewport')
s=Path(a).read_text(encoding='utf-8');start=s.index('    fn draw_end_message(&self) {');end=s.index('\n    fn ',start+10)
s=s[:start]+'''    fn draw_end_message(&self) { self.draw_run_summary(); }
'''+s[end:]
# Inventory always exposes the character sheet, and a shared complete item card.
s=s.replace('''        let character_details = stats_panel.map(|panel| {''','''        let character_details = Some(stats_panel.map(|panel| {''',1)
start=s.index('        let character_details = Some');end=s.index('        Self {',start)
chunk=s[start:end].replace('        });','        }).unwrap_or(Rect::new(margin + panel_width - 164.0, top + 17.0, 144.0, 31.0)));')
s=s[:start]+chunk+s[end:]
s=s.replace('Aucun bonus statistique','Aucun bonus d’attribut')
start=s.index('    fn update_inventory(');idx=s.index('        let hovered_row',start)
s=s[:idx]+'''        let details=Rect::new(32.0+layout.left_width+12.0,47.0,145.0,31.0);
        if self.controls.pressed(Action::Inspect,input) || input.pressed.contains(&controls::Binding::MouseLeft) && input.pointer.is_some_and(|p|details.contains(p.into())) {self.open_inventory_card();return;}
'''+s[idx:]
start=s.index('    fn draw_inventory(&self)');idx=s.index('        if let Some(stats_panel) = layout.stats_panel',start)
s=s[:idx]+'''        UiTheme.button(Rect::new(32.0+layout.left_width+12.0,47.0,145.0,31.0),"Détails / comparer",false,false,!entries.is_empty(),ButtonTone::Secondary);
        if layout.stats_panel.is_none() {UiTheme.button(layout.character_details.unwrap(),"Personnage [J]",false,false,true,ButtonTone::Secondary);}
'''+s[idx:]
s=s.replace('''format!("Équiper [{}]", self.controls.label(action)),''','''format!("{} [{}]",self.equipped_instance_name(index as u8).map_or("Équiper · vide".to_owned(),|name|format!("Remplacer {name}")), self.controls.label(action)),''',1)
s=s.replace('''            if label.is_empty() {
                continue;
            }''','''            if label.is_empty() || index == 3 && !selected_is_consumable || index < 3 && !selected_is_weapon && !selected_is_armor {
                continue;
            }''',1)
# Buy and inspect have distinct rectangles; inspecting never buys.
idx=s.index('    fn merchant_rows(&self)')
s=s[:idx]+'''    fn merchant_action(&self) -> Rect { Rect::new(self.service.x,self.service.y,self.service.w*0.53-4.0,self.service.h) }
    fn merchant_inspect(&self) -> Rect { Rect::new(self.service.x+self.service.w*0.53+4.0,self.service.y,self.service.w*0.47-4.0,self.service.h) }
'''+s[idx:]
start=s.index('            let service_hovered =',s.index('            if entries > 0 {',s.index('fn update_npc_interaction')))
end=s.index('            let close_hovered',start)
s=s[:start]+'''            let selected_item = match self.npc_trade_mode {
                NpcTradeMode::Buy => offers.get(self.npc_trade_selection).map(|v|(&v.item,None)).or_else(||resale.get(self.npc_trade_selection.saturating_sub(offers.len())).map(|v|(&v.item,v.magic_modifiers.clone()))),
                NpcTradeMode::Sell => sellable.get(self.npc_trade_selection).map(|v|(&v.item,v.magic_modifiers.clone())),
                NpcTradeMode::Gamble => gambles.get(self.npc_trade_selection).map(|v|(&v.item,None)),
            };
            let purchased_name=selected_item.as_ref().map(|(id,bonus)|self.trade_item_name(self.npc_trade_mode,id,bonus.clone())).unwrap_or_default();
            let inspect_hovered=input.pointer.is_some_and(|p|layout.merchant_inspect().contains(p.into()));
            if clicked && inspect_hovered || self.controls.pressed(Action::Inspect,input) {
                if let Some((id,bonus))=selected_item {self.ux.item_card=Some((purchased_name,self.item_card_lines(id,bonus,self.npc_trade_mode==NpcTradeMode::Gamble)));self.ux.item_scroll.offset=0.0;self.ux.comparison_slot=None;}
                return;
            }
            let service_hovered = input.pointer.is_some_and(|point|layout.merchant_action().contains(point.into()));
'''+s[end:]
s=s.replace('NpcTradeMode::Buy => "Achat effectué.".to_owned(),','NpcTradeMode::Buy => format!("Reçu : {purchased_name}."),',1)
s=s.replace('NpcTradeMode::Sell => "Vente effectuée.".to_owned(),','NpcTradeMode::Sell => format!("Vendu : {purchased_name}."),',1)
start=s.index('    fn draw_merchant');idx=s.index('        theme.dialogue_action_with_icon(\n            layout.service,',start)
s=s[:idx]+'''        theme.button(layout.merchant_inspect(),"Examiner [F4]",false,false,count>0,ButtonTone::Secondary);
        if !enabled && self.npc_interaction_message.is_empty() {
            draw_rectangle(x,layout.service.y-28.0,layout.panel.w-44.0,22.0,theme.surface());
            draw_text(if count==0 {"Aucun objet disponible"}else if self.npc_trade_mode==NpcTradeMode::Sell {"La marchande n'a pas assez de crédits"}else{"Crédits insuffisants ou stock épuisé"},x,layout.service.y-10.0,14.0,theme.attention());
        }
'''+s[idx:]
idx=s.index('        theme.dialogue_action_with_icon(\n            layout.service,',start)
s=s[:idx]+s[idx:].replace('        theme.dialogue_action_with_icon(\n            layout.service,','        theme.dialogue_action_with_icon(\n            layout.merchant_action(),',1)
# Quests: chosen tracking + direct archive link with existing known data only.
start=s.index('    fn update_quest_journal');idx=s.index('        if count == 0 {',start)
s=s[:idx]+'''        let track=Rect::new(layout.panel.x+20.0,layout.close.y,190.0,layout.close.h);
        let archive=Rect::new(track.right()+10.0,track.y,170.0,track.h);
        if clicked && input.pointer.is_some_and(|p|archive.contains(p.into())) {self.quest_journal_open=false;self.report_open=true;self.ux.dossier_scroll.offset=0.0;return;}
        if self.controls.pressed(Action::Learn,input) || clicked && input.pointer.is_some_and(|p|track.contains(p.into())) {
            if let Some(entry)=self.game.quest_journal().get(self.quest_journal_selection) && entry.quest.status!=QuestStatus::Completed {self.ux.tracked_quest=Some(entry.quest.id.clone());}
            return;
        }
'''+s[idx:]
start=s.index('    fn draw_quest_journal');idx=s.index('        if entries.is_empty()',start)
s=s[:idx]+'''        let track=Rect::new(layout.panel.x+20.0,layout.close.y,190.0,layout.close.h);
        let tracked=entries.get(self.quest_journal_selection).is_some_and(|e|self.ux.tracked_quest.as_ref()==Some(&e.quest.id));
        UiTheme.button(track,if tracked {"Quête suivie"}else{"Suivre cette quête"},false,tracked,entries.get(self.quest_journal_selection).is_some_and(|e|e.quest.status!=QuestStatus::Completed),ButtonTone::Primary);
        UiTheme.button(Rect::new(track.right()+10.0,track.y,170.0,track.h),"Archives connues",false,false,true,ButtonTone::Secondary);
'''+s[idx:]
start=s.index('    fn primary_objective');idx=s.index('        if let Some(objective)',start)
s=s[:idx]+'''        if let Some(id)=&self.ux.tracked_quest && let Some(entry)=self.game.quest_journal().iter().find(|e|&e.quest.id==id && e.quest.status!=QuestStatus::Completed) {
            return Some(if entry.quest.status==QuestStatus::ReadyToComplete {format!("Objectif · Retourner parler à {}",self.quest_giver_name(entry))}else{format!("Objectif · {} · {}",self.texts.resolve(DISPLAY_LOCALE,&entry.quest.title_key).unwrap_or("Quête suivie"),self.quest_objective_text(&entry.quest.objective))});
        }
'''+s[idx:]
s=s.replace('"Faits établis"','"Enquête auprès des contacts connus"').replace('"Terminal ciblé consulté"','"Consulter le terminal indiqué"').replace('if *accessed { "oui" } else { "non" }','if *accessed { "terminé" } else { "à poursuivre" }')
# Readability corrections and a shorter creation sheet.
s=s.replace('let h = (height - 32.0).min(620.0);','let h = (height - 32.0).min(540.0);',1)
s=s.replace('let mut lines = Vec::with_capacity(maximum_lines);','let mut lines = Vec::with_capacity(maximum_lines.min(16));',1)
s=s.replace('let amber = Color::from_rgba(255, 211, 92, 255);','let amber = UiTheme.accent();')
s=s.replace('let muted = Color::from_rgba(102, 139, 148, 255);','let muted = UiTheme.muted();')
s=s.replace('"← → discipline · ↑ ↓ technique · Page ↓ détail"','"Gauche/droite : discipline · Haut/bas : technique · Page suiv. : détail"')
s=s.replace('.replace("P2+A1", "Préparation 2 tours + action 1 tour");','.replace("P2+A1", "Préparation 2 tours + action 1 tour").replace("R1", "récupération 1 tour").replace("R2", "récupération 2 tours");')
start=s.index('    fn update_input_at');idx=s.index('        let context =',start)
s=s[:idx]+'        let skill_before=(self.skill_discipline_selection,self.skill_technique_selection);\n'+s[idx:]
idx=s.index('        // A held key never',idx)
s=s[:idx]+'        if skill_before!=(self.skill_discipline_selection,self.skill_technique_selection) { self.ux.skill_scroll.offset=0.0; }\n'+s[idx:]
Path(a).write_text(s,encoding='utf-8',newline='\n')
u=Path('src/ux.rs');s=u.read_text(encoding='utf-8').replace('↑ ↓','flèches').replace('Options →','Options /').replace('→ Commandes','/ Commandes');u.write_text(s,encoding='utf-8',newline='\n')
