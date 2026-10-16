from pathlib import Path
root=Path(__file__).resolve().parents[2]
def edit(name, pairs):
    p=root/name;s=p.read_text(encoding='utf-8')
    for a,b in pairs:
        assert a in s, (name,a[:120]);s=s.replace(a,b,1)
    p.write_text(s,encoding='utf-8')
edit('src/ascii_app.rs',[
('onboarding.or_else(||self.primary_objective())','self.primary_objective().or(onboarding)'),
('held_menu_key("Enter", true), Some(7.0)','held_menu_key("Tab", true), Some(7.0)'),
('held_menu_key("Enter", false), Some(8.0)','held_menu_key("Tab", false), Some(8.0)'),
('fn live_level_gain_opens_skills_after_the_turn_but_replay_does_not()', 'fn live_level_gain_notifies_without_interrupting_play_and_replay_stays_silent()'),
('assert!(live.skills_open);','assert!(!live.skills_open);\n        live.update_input(&input("K"));\n        assert!(live.skills_open);'),
('["Annuler", "Confirmer"]','["Annuler", "Abandonner la partie"]'),
('["Annuler", "Confirmer"]','["Annuler", "Remplacer la partie"]'),
('"Terminal ciblé consulté : oui"','"Consulter le terminal indiqué : terminé"'),
('format!("Interagir : {name} ({}, {})", position.x, position.y)', 'format!("Interagir : {name} · {}", approximate_direction(restored.game.player_position().unwrap(), position))'),
('app.report_open = true;\n        app.update_input(&InputFrame {\n            wheel_y: -3.0,','app.report_open = true;\n        app.ux.dossier_scroll.maximum.set(500.0);\n        app.update_input(&InputFrame {\n            wheel_y: -3.0,'),
('assert_eq!(app.report_scroll, 3);','assert_eq!(app.ux.dossier_scroll.offset, 108.0);'),
('assert_eq!(app.report_scroll, 1);','assert_eq!(app.ux.dossier_scroll.offset, 36.0);'),
('input.pressed.contains(&controls::Binding::key("Tab")) || input.pressed.contains(&controls::Binding::MouseLeft) && input.pointer.is_some_and(|p|filter.contains(p.into()))','self.controls.pressed(Action::InventoryFilter,input) || input.pressed.contains(&controls::Binding::MouseLeft) && input.pointer.is_some_and(|p|filter.contains(p.into()))'),
('        let discipline_open = availability.is_some_and(|state| state.is_open());\n        let learning=', '        let learning='),
('if self.ux.skill_filter {"Disponibles [Tab]"}else{"Toutes [Tab]"}', '&format!("{} [{}]",if self.ux.skill_filter {"Disponibles"}else{"Toutes"},self.controls.label(Action::InventoryFilter))'),
('draw_text("Gauche/droite : discipline · Haut/bas : technique · Page suiv. : détail",', 'draw_text(format!("{} / {} : discipline · {} / {} : technique · Page suiv. : détail",self.controls.label(Action::MenuLeft),self.controls.label(Action::MenuRight),self.controls.label(Action::MenuUp),self.controls.label(Action::MenuDown)),'),
('"Relevés et archives découverts · N : journal des quêtes"','&format!("Relevés et archives découverts · {} : journal des quêtes",self.controls.label(Action::QuestJournal))'),
('"LABORATOIRE · {} : variantes d\'équipement · Échap : réinitialiser / retour",\n                self.controls.label(Action::Inventory)','"LABORATOIRE · {} : choisir un essai · {} : équipement",\n                self.controls.label(Action::Laboratory), self.controls.label(Action::Inventory)'),
('let panel = crate::terminal_view::legend_panel(app.terminal_bounds());','let panel = ux::reader_panel(app.ui_width(),app.ui_height());'),
])
edit('src/narrative_app.rs', [('"Interagir : Elias (5, 3)"','"Interagir : Elias · EST"')])
edit('src/effects_lab.rs', [('rebind(Action::Attack, Binding::key("V"))','rebind(Action::Attack, Binding::key("F6"))'),('app.update_input(&key("V"));','app.update_input(&key("F6"));')])
edit('src/ux.rs',[
('    pub character_scroll: ScrollState,','    pub character_scroll: ScrollState,\n    pub quest_scroll: ScrollState,\n    pub end_selection: usize,'),
('Action::NpcVision,Action::Restart]', 'Action::NpcVision,Action::CompanionOrder,Action::Restart,Action::MenuUp,Action::MenuDown,Action::MenuLeft,Action::MenuRight,Action::Learn,Action::Use,Action::Drop,Action::InventoryFilter,Action::InventorySort]'),
('        lines\n    }\n\n    pub(super) fn update_help','        if self.test_lab { for action in [Action::Laboratory,Action::Corrosion,Action::Pulse] {lines.push(format!("{} · {}",action.name(),self.controls.label(action)));} }\n        lines\n    }\n\n    pub(super) fn update_help'),
('slot+1,slot+1)', 'slot+1,self.controls.label([Action::Slot1,Action::Slot2,Action::Slot3][slot as usize]))'),
('        if input.pressed.contains(&controls::Binding::MouseLeft) {\n            match hovered {','        if self.controls.pressed(Action::MenuUp,input) { self.ux.end_selection=self.ux.end_selection.saturating_sub(2); }\n        if self.controls.pressed(Action::MenuDown,input) { self.ux.end_selection=(self.ux.end_selection+2).min(3); }\n        if self.controls.pressed(Action::MenuLeft,input) { self.ux.end_selection=self.ux.end_selection.saturating_sub(1); }\n        if self.controls.pressed(Action::MenuRight,input) { self.ux.end_selection=(self.ux.end_selection+1).min(3); }\n        let chosen=if input.pressed.contains(&controls::Binding::MouseLeft) { hovered } else if self.controls.pressed(Action::Learn,input) {Some(self.ux.end_selection)}else{None};\n        if chosen.is_some() {\n            match chosen {'),
('.zip(["Recommencer avec ce profil [R]","Choisir un autre profil","Consulter les derniers tours [V]","Retour à l\'accueil"])','.zip([format!("Recommencer [{}]",self.controls.label(Action::Restart)),"Choisir un autre profil".to_owned(),format!("Derniers tours [{}]",self.controls.label(Action::EventHistory)),"Retour à l\'accueil".to_owned()])'),
('UiTheme.button(rect,label,self.menu_focus.hovered==Some(i),false,true,if i==0','UiTheme.button(rect,&label,self.menu_focus.hovered==Some(i),self.ux.end_selection==i,true,if i==0'),
])
