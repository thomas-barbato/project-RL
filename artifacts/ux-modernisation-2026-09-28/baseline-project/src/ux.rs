//! Presentation-only state and scrollable readers. No game commands are issued by readers.
use super::*;
use std::cell::Cell;

#[derive(Default)]
pub(super) struct UxState {
    pub history_open: bool,
    pub history: Vec<(u64, String)>,
    pub history_scroll: ScrollState,
    pub help_tab: usize,
    pub help_scroll: ScrollState,
    pub dossier_scroll: ScrollState,
    pub skill_scroll: ScrollState,
    pub skill_filter: bool,
    pub skill_selection: Cell<(usize, usize)>,
    pub tracked_quest: Option<ContentId>,
    pub inspected_target: bool,
    pub moved: bool,
    pub interacted: bool,
    pub targeted: bool,
    pub item_card: Option<(String, Vec<String>)>,
    pub item_scroll: ScrollState,
    pub comparison_slot: Option<u8>,
    pub hud_points: Cell<Option<Rect>>,
    pub hud_defenses: Cell<Option<Rect>>,
    pub character_scroll: ScrollState,
    pub controls_group: usize,
    pub lab_open: bool,
    pub lab_selection: usize,
    pub resume_error: String,
}

#[derive(Default)]
pub(super) struct ScrollState {
    pub offset: f32,
    pub maximum: Cell<f32>,
}

impl ScrollState {
    pub fn update(&mut self, input: &InputFrame, controls: &Controls) {
        let delta = if controls.pressed(Action::MenuDown, input) { 28.0 }
            else if controls.pressed(Action::MenuUp, input) { -28.0 }
            else if input.pressed.contains(&controls::Binding::key("PageDown")) { 240.0 }
            else if input.pressed.contains(&controls::Binding::key("PageUp")) { -240.0 }
            else { -input.wheel_y * 36.0 };
        self.offset = (self.offset + delta).clamp(0.0, self.maximum.get());
    }

    pub fn finish(&self, rect: Rect, bottom: f32) {
        let maximum = (bottom - rect.bottom() + 12.0).max(0.0);
        self.maximum.set(maximum);
        if maximum > 0.0 {
            let track = Rect::new(rect.right() - 4.0, rect.y, 3.0, rect.h);
            let thumb = (track.h * rect.h / (maximum + rect.h)).max(20.0);
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            draw_rectangle(track.x, track.y + (track.h - thumb) * self.offset.min(maximum) / maximum, track.w, thumb, UiTheme.accent());
        }
    }
}

pub(super) fn reader_panel(width: f32, height: f32) -> Rect {
    let w = (width - 40.0).min(1000.0);
    let h = (height - 40.0).min(740.0);
    Rect::new((width - w) * 0.5, (height - h) * 0.5, w, h)
}

pub(super) fn reader_close(panel: Rect) -> Rect {
    Rect::new(panel.right() - 116.0, panel.y + 18.0, 96.0, 34.0)
}

pub(super) fn reader_body(panel: Rect) -> Rect {
    Rect::new(panel.x + 24.0, panel.y + 116.0, panel.w - 48.0, panel.h - 161.0)
}

pub(super) fn draw_reader(title: &str, subtitle: &str, panel: Rect, hovered: bool, controls: &Controls) {
    UiTheme.panel(panel);
    draw_wrapped_text(title, panel.x + 24.0, panel.y + 43.0, panel.w - 168.0, 1, 26, UiTheme.text());
    draw_wrapped_text(subtitle, panel.x + 24.0, panel.y + 76.0, panel.w - 48.0, 1, 15, UiTheme.muted());
    UiTheme.button(reader_close(panel), "Fermer", hovered, false, true, ButtonTone::Secondary);
    draw_text(format!("Molette / {} {} / Page préc. suiv. · Échap : fermer",controls.label(Action::MenuUp),controls.label(Action::MenuDown)), panel.x + 24.0, panel.bottom() - 18.0, 14.0, UiTheme.muted());
}

impl AsciiApp {
    pub(super) fn control_actions(&self) -> Vec<Action> {
        Action::ALL.iter().copied().filter(|a| self.ux.controls_group==0 || Self::control_group(*a)==self.ux.controls_group).collect()
    }

    fn control_group(action:Action) -> usize {
        match action {
            Action::MoveNorth|Action::MoveSouth|Action::MoveEast|Action::MoveWest|Action::Wait=>1,
            Action::Inventory|Action::Character|Action::Skills|Action::QuickTechniques|Action::Report|Action::QuestJournal|Action::Legend|Action::EventHistory|Action::Inspect=>3,
            Action::MenuUp|Action::MenuDown|Action::MenuLeft|Action::MenuRight|Action::Learn|Action::Use|Action::Drop|Action::InventoryFilter|Action::InventorySort=>4,
            Action::Corrosion|Action::Pulse|Action::Laboratory=>5,
            _=>2
        }
    }

    pub(super) fn control_tab(width:f32,index:usize) -> Rect {Rect::new(30.0+index as f32*(width-60.0)/6.0,103.0,(width-78.0)/6.0,28.0)}

    pub(super) fn lab_actions(width:f32,height:f32) -> [Rect;6] {
        let p=reader_panel(width,height);std::array::from_fn(|i|Rect::new(p.x+24.0,p.y+110.0+i as f32*49.0,p.w-48.0,39.0))
    }

    pub(super) fn update_lab_menu(&mut self,input:&InputFrame) {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
        let clicked=input.pressed.contains(&controls::Binding::MouseLeft);
        if self.controls.pressed(Action::MenuUp,input){self.ux.lab_selection=self.ux.lab_selection.saturating_sub(1);}
        if self.controls.pressed(Action::MenuDown,input){self.ux.lab_selection=(self.ux.lab_selection+1).min(5);}
        let hover=Self::lab_actions(w,h).iter().position(|r|input.pointer.is_some_and(|p|r.contains(p.into())));
        self.menu_focus.hovered=hover;
        if clicked && let Some(i)=hover{self.ux.lab_selection=i;}
        if self.controls.pressed(Action::Laboratory,input){self.ux.lab_open=false;return;}
        if self.controls.pressed(Action::Learn,input) || clicked && hover.is_some(){
            self.ux.lab_open=false;
            match self.ux.lab_selection {
                0=>{self.inventory_open=true;self.inventory_filter=InventoryFilter::Weapons;self.clamp_inventory_selection();},
                1=>{self.inventory_open=true;self.inventory_filter=InventoryFilter::Armor;self.clamp_inventory_selection();},
                2=>{self.skills_open=true;self.clamp_skill_selection();},
                3=>self.cycle_target(),
                4=>{if let Err(e)=self.reset_test_lab(){self.push_log(e);}},
                _=>{}
            }
        }
    }

    pub(super) fn draw_lab_menu(&self) {
        let p=reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());UiTheme.panel(p);
        draw_text_bold("Laboratoire",p.x+24.0,p.y+42.0,28.0,UiTheme.text());
        let target=self.terminal_target_summary().map_or("aucune".to_owned(),|t|t.name);
        draw_wrapped_text(&format!("Cible actuelle : {target} · Choisissez ce que vous voulez tester."),p.x+24.0,p.y+77.0,p.w-48.0,1,16,UiTheme.muted());
        for (i,(rect,label)) in Self::lab_actions(self.ui_width(),self.ui_height()).into_iter().zip(["Armes et effets · choisir dans l'inventaire","Protections et bonus d'équipement","Techniques et compétences","Sélectionner la cible suivante","Réinitialiser les cibles et les réserves","Revenir à l'essai"]).enumerate(){UiTheme.button(rect,label,self.menu_focus.hovered==Some(i),self.ux.lab_selection==i,true,if i==5{ButtonTone::Primary}else{ButtonTone::Secondary});}
    }

    pub(super) fn cycle_companion_order(&mut self) {
        let current=self.game.player_controlled_companions().first().and_then(|id|self.game.actors().get(*id)).and_then(Actor::drone).and_then(|d|match d.order(){DroneOrder::Companion{behavior,..}=>Some(*behavior),DroneOrder::Escort{..}=>Some(CompanionBehavior::Follow),_=>None});
        let next=current.and_then(|c|CompanionBehavior::ALL.iter().position(|b|*b==c)).map_or(0,|i|(i+1)%CompanionBehavior::ALL.len());
        let outcome=self.execute_command(GameCommand::SetCompanionBehavior{behavior:CompanionBehavior::ALL[next]});
        if let CommandOutcome::Rejected(reason)=outcome {self.push_log(command_rejection_message(reason).to_owned());}
        self.capture_events();
    }
    pub(super) fn item_card_lines(&self, id:&ItemId, bonus:Option<project_rl::entity::MagicItemModifiers>, hidden:bool) -> Vec<String> {
        if hidden { return vec!["Objet magique non identifié. Les propriétés seront révélées après le pari.".to_owned(), "Le prix et le type d'objet sont connus ; aucun bonus n'est garanti avant identification.".to_owned()]; }
        let mut lines=Vec::new();
        if let Some(b)=bonus.clone() { lines.extend(equipment_affix_names::affix_detail_lines(b)); }
        if let Some(weapon)=self.game.rules().weapons.get(id) {
            let attack=weapon.attack();
            lines.push(format!("Arme · portée {} case(s) · précision de base {:+}",attack.range(),attack.accuracy_modifier()));
            lines.push(format!("Dégâts de base · {}",format_damage_impact(attack.damage())));
            if let Some(impact)=attack.melee_impact(){ lines.push(format!("Impact maximal du matériau · {}",impact.material_cap)); }
            if let Some(cost)=self.weapon_supply_label(id){lines.push(format!("Ressource · {cost}"));}
            lines.push(self.texts.resolve(DISPLAY_LOCALE,weapon.description_key()).unwrap_or("").to_owned());
            lines.push(self.weapon_effect_limits(weapon));
            if let Some(b)=bonus.as_ref() {
                if b.accuracy_bonus()>0 {lines.push(format!("Bonus de précision · +{}",b.accuracy_bonus()));}
                if let Some(effect)=b.effect_affix().and_then(|id|self.game.rules().weapons.effect_affix(id)) {lines.push(self.texts.resolve(DISPLAY_LOCALE,effect.description_key()).unwrap_or("").to_owned());}
            }
        }
        if let Some(item)=self.game.rules().items.get(id) {
            if let Some(equipment)=item.equipment(){lines.push(format!("Armure · {} · emplacement {}",equipment.armor()+bonus.as_ref().map_or(0,|b|b.armor_bonus()),self.equipment_slot_name(equipment.slot())));}
            for effect in item.effects(){let ItemEffect::RestoreIntegrity{amount}=effect;lines.push(format!("Restaure {amount} PV"));}
        }
        if lines.is_empty(){lines.push("Matériau utilisable dans les interactions qui le demandent.".to_owned());}
        lines
    }

    pub(super) fn open_inventory_card(&mut self) {
        let entries=self.inventory_entries();
        if let Some(entry)=entries.get(self.inventory_selection) {
            self.ux.item_card=Some((self.inventory_entry_name(entry),self.item_card_lines(entry.item(),entry.magic_modifiers(),false)));
            self.ux.item_scroll.offset=0.0;self.ux.comparison_slot=None;
        }
    }

    pub(super) fn update_item_card(&mut self,input:&InputFrame) {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));let panel=reader_panel(w,h);
        let clicked=input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked && input.pointer.is_some_and(|p|reader_close(panel).contains(p.into())) {self.ux.item_card=None;return;}
        if self.inventory_open {
            for slot in 0..3 {
                let rect=Rect::new(panel.x+24.0+slot as f32*(panel.w-48.0)/3.0,panel.y+74.0,(panel.w-60.0)/3.0,32.0);
                if clicked && input.pointer.is_some_and(|p|rect.contains(p.into())) || pressed_weapon_slot(&self.controls,input)==Some(slot) { self.ux.comparison_slot=Some(slot);self.ux.item_scroll.offset=0.0; }
            }
        }
        self.ux.item_scroll.update(input,&self.controls);
    }

    pub(super) fn draw_item_card(&self) {
        let Some((title,lines))=&self.ux.item_card else{return;};
        let panel=reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());
        draw_reader(title,if self.inventory_open {""}else{"Caractéristiques connues avant achat"},panel,false, &self.controls);
        if self.inventory_open {
            for slot in 0..3 {
                let rect=Rect::new(panel.x+24.0+slot as f32*(panel.w-48.0)/3.0,panel.y+74.0,(panel.w-60.0)/3.0,32.0);
                UiTheme.button(rect,&format!("Comparer au canal {} [{}]",slot+1,slot+1),false,self.ux.comparison_slot==Some(slot),true,ButtonTone::Secondary);
            }
        }
        let body=reader_body(panel);crate::ui_theme::begin_text_pane(body,self.ux.item_scroll.offset);
        let mut y=body.y+24.0;
        for line in lines {y=draw_wrapped_text(line,body.x,y,body.w-20.0,4096,17,UiTheme.text())+16.0;}
        if let Some(slot)=self.ux.comparison_slot {
            y+=16.0;
            draw_text_bold(format!("Actuellement au canal {}",slot+1),body.x,y,20.0,UiTheme.accent());y+=34.0;
            let id=self.game.rules().player_weapon_slots.get(slot as usize).and_then(|s|self.game.player_equipment().equipped(s));
            if let Some(entry)=id.and_then(|id|self.game.player_inventory().get(id)) {
                y=draw_wrapped_text(&self.inventory_entry_name(entry),body.x,y,body.w-20.0,4096,19,UiTheme.accent())+12.0;
                for line in self.item_card_lines(entry.item(),entry.magic_modifiers(),false){y=draw_wrapped_text(&line,body.x,y,body.w-20.0,4096,17,UiTheme.text())+16.0;}
            } else {draw_text("Emplacement vide",body.x,y,17.0,UiTheme.muted());y+=26.0;}
        }
        crate::ui_theme::end_text_pane();self.ux.item_scroll.finish(body,y);
    }

    pub(super) fn end_actions(width:f32,height:f32) -> [Rect;4] {
        let panel=reader_panel(width,height);let w=(panel.w-60.0)/2.0;
        std::array::from_fn(|i|Rect::new(panel.x+24.0+(i%2)as f32*(w+12.0),panel.bottom()-106.0+(i/2)as f32*44.0,w,36.0))
    }

    pub(super) fn update_end_screen(&mut self,input:&InputFrame) {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
        let hovered=Self::end_actions(w,h).iter().position(|r|input.pointer.is_some_and(|p|r.contains(p.into())));
        self.menu_focus.hovered=hovered;
        if input.pressed.contains(&controls::Binding::MouseLeft) {
            match hovered {
                Some(0)=>self.restart_current_profile(),
                Some(1)=>{if let Err(e)=self.begin_character_creation(false){self.push_log(e);}},
                Some(2)=>{self.ux.history_open=true;self.ux.history_scroll.offset=0.0;},
                Some(3)=>{if let Err(e)=self.rebuild_run(MenuScreen::Main,false){self.push_log(e);}},
                _=>{}
            }
        }
    }

    pub(super) fn draw_run_summary(&self) {
        if self.game.status()==RunStatus::Active{return;}
        let panel=reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());UiTheme.panel(panel);
        draw_text_bold(if self.game.status()==RunStatus::PlayerDestroyed{"Votre noyau a été détruit"}else{"Expédition terminée"},panel.x+24.0,panel.y+45.0,28.0,UiTheme.text());
        let p=self.game.player_progression();
        draw_text(format!("{} tours · Niveau {} · {} XP · {} archives découvertes",self.game.turn(),p.level(),p.experience(),self.game.discovered_data_terminal_records().len()),panel.x+24.0,panel.y+79.0,17.0,UiTheme.accent());
        draw_text_bold("Derniers événements observés",panel.x+24.0,panel.y+124.0,18.0,UiTheme.muted());
        let body=Rect::new(panel.x+24.0,panel.y+136.0,panel.w-48.0,panel.h-257.0);
        crate::ui_theme::begin_text_pane(body,0.0);let mut y=body.y+24.0;
        for m in self.log.iter().rev().take(4){y=draw_wrapped_text(m,body.x,y,body.w,4096,17,UiTheme.text())+15.0;}
        crate::ui_theme::end_text_pane();
        for (i,(rect,label)) in Self::end_actions(self.ui_width(),self.ui_height()).into_iter().zip(["Recommencer avec ce profil [R]","Choisir un autre profil","Consulter les derniers tours [V]","Retour à l'accueil"]).enumerate(){UiTheme.button(rect,label,self.menu_focus.hovered==Some(i),false,true,if i==0{ButtonTone::Primary}else{ButtonTone::Secondary});}
    }
    pub(super) fn skill_learning_cost(&self, id: &TechniqueId) -> Result<u16, String> {
        let definition=self.game.rules().skills.technique(id).ok_or("Technique indisponible")?;
        if self.game.player_skills().has_learned(id) { return Err("Déjà apprise".to_owned()); }
        let availability=self.skill_availability_cache.get(definition.discipline()).ok_or("Discipline indisponible")?;
        if !availability.is_open() || !availability.available.contains(id) { return Err("Technique indisponible".to_owned()); }
        if definition.minimum_level()>self.game.player_progression().level() { return Err(format!("Niveau {} requis",definition.minimum_level())); }
        if let Some(required)=definition.prerequisite() && !self.game.player_skills().has_learned(required) { return Err(format!("Apprendre {} d'abord",self.technique_name(required))); }
        if let Some(r)=definition.unmet_attribute_requirement(self.game.player_primary_attributes()) { return Err(format!("{} {} requis",primary_attribute_label(r.attribute()),r.minimum())); }
        let learned=self.skill_techniques_cache.get(definition.discipline()).into_iter().flatten().filter(|id|self.game.player_skills().has_learned(id)).count();
        let cost=self.game.rules().skill_progression.cost_for_choice_number(learned+1).ok_or("Coût indisponible")?;
        if self.game.player_progression().unspent_skill_points()<u32::from(cost) { return Err(format!("{cost} points nécessaires")); }
        Ok(cost)
    }

    pub(super) fn skill_filter_rect(panel: Rect) -> Rect {
        Rect::new(panel.x+10.0,panel.bottom()-44.0, (panel.w*0.23).clamp(180.0,260.0),32.0)
    }

    pub(super) fn update_inspector(&mut self,input:&InputFrame) {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
        let hovered=input.pointer.is_some_and(|p|reader_close(reader_panel(w,h)).contains(p.into()));
        self.menu_focus.hovered=hovered.then_some(0);
        if self.controls.pressed(Action::Inspect,input) || input.pressed.contains(&controls::Binding::MouseLeft) && hovered { self.ux.inspected_target=false; }
        self.ux.help_scroll.update(input,&self.controls);
    }

    pub(super) fn draw_inspector(&self) {
        let panel=reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());
        draw_reader("État et cible","Informations connues · aucune action ni aucun tour dépensé",panel,self.menu_focus.hovered==Some(0), &self.controls);
        let body=reader_body(panel);
        crate::ui_theme::begin_text_pane(body,self.ux.help_scroll.offset);
        let mut y=body.y+22.0;
        let id=self.game.player_id();
        let mut lines=Vec::new();
        if let Some(player)=self.game.actors().get(id) {
            lines.push(format!("Votre état · {} / {} PV · Armure {} · Esquive {} · Stabilité {} · Défense numérique {}",player.integrity(),player.maximum_integrity(),self.game.actor_armor_profile(id).map_or(0,ArmorProfile::after_fragilization),self.game.actor_evasion(id).unwrap_or(0),self.game.actor_stability(id).unwrap_or(0),self.game.actor_digital_defense(id).unwrap_or(0)));
            lines.push(format!("Résistances · {}", [("Thermique",DamageType::Thermal),("Électrique",DamageType::Electrical),("Chimique",DamageType::Chemical),("Radiation",DamageType::Radiation),("Corruption",DamageType::Corruption)].into_iter().map(|(label,t)|format!("{label} {} %",player.resistances().get(t))).collect::<Vec<_>>().join(" · ")));
        }
        lines.push(format!("Énergie {}/{} · Munitions {}",self.game.player_energy().available(),self.game.player_energy().capacity(),self.game.player_matter().unwrap_or(0)));
        if let Some(b)=self.game.player_bandwidth(){lines.push(format!("Bande passante disponible {}/{}",b.available(),b.capacity()));}
        if let Some(h)=self.game.player_heat(){lines.push(format!("Chaleur {} · alerte {} · seuil critique {}",h.current(),h.alert_threshold(),h.critical_threshold()));}
        if let Some(target)=self.terminal_target_summary() {
            lines.push(format!("Cible · {} · distance {} cases",target.name,target.distance));
            lines.push(target.visible_state);
            if let Some(a)=target.analysis {lines.push(format!("Analyse · {} / {} PV · Armure {} · {}",a.integrity,a.maximum_integrity,a.armor,a.resistances));}
            else {lines.push("Statistiques inconnues. Une analyse de la cible peut compléter cette fiche.".to_owned());}
        } else {lines.push(format!("Aucune cible sélectionnée. {} ou clic sur une entité visible pour choisir.",self.controls.label(Action::CycleTarget)));}
        for line in lines {y=draw_wrapped_text(&line,body.x,y,body.w-20.0,4096,17,UiTheme.text())+24.0;}
        crate::ui_theme::end_text_pane();self.ux.help_scroll.finish(body,y);
    }

    pub(super) fn hud_shortcuts(width:f32,height:f32) -> Vec<(Rect,Option<Action>, &'static str,UiIcon)> {
        let start=quest_journal_button_rect(height).right()+8.0;
        let stride=(width-start-12.0)/8.0;
        [(Some(Action::Interact),"Interagir",UiIcon::Interact),(Some(Action::Attack),"Attaquer",UiIcon::Attack),(Some(Action::QuickTechniques),"Techniques",UiIcon::Techniques),(Some(Action::Inventory),"Inventaire",UiIcon::Inventory),(Some(Action::Inspect),"État / cible",UiIcon::Target),(Some(Action::EventHistory),"Historique",UiIcon::Quest),(Some(Action::Legend),"Aide",UiIcon::Help),(None,"Menu",UiIcon::Menu)]
            .into_iter().enumerate().map(|(i,(a,l,icon))|(Rect::new(start+i as f32*stride,height-94.0,stride-4.0,34.0),a,l,icon)).collect()
    }

    pub(super) fn draw_hud_shortcuts(&self) {
        for (index,(rect,action,label,icon)) in Self::hud_shortcuts(self.ui_width(),self.ui_height()).into_iter().enumerate() {
            let hovered=self.menu_focus.hovered==Some(50_000+index);
            UiTheme.hud_action(rect,if hovered{1.0}else{0.0});
            let key=action.map_or_else(||"Esc".to_owned(),|a|self.controls.label(a));
            draw_ui_icon(icon,Rect::new(rect.x+7.0,rect.y+10.0,15.0,15.0),UiTheme.accent());
            if rect.w>=100.0 { draw_text(format!("{label} {key}"),rect.x+28.0,rect.y+23.0,13.0,UiTheme.text()); }
            else {draw_text(&key,rect.x+27.0,rect.y+23.0,13.0,UiTheme.text());}
            if hovered {draw_text_bold(label,rect.x.min(self.ui_width()-130.0),rect.y-8.0,14.0,UiTheme.text());}
        }
    }

    pub(super) fn route_hud_click(&mut self,input:&InputFrame,captured_at:Option<f64>) -> bool {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
        let clicked=input.pressed.contains(&controls::Binding::MouseLeft);
        for (rect,action,focus) in [(self.ux.hud_points.get(),Action::Skills,50_020),(self.ux.hud_defenses.get(),Action::Inspect,50_021)] {
            if rect.is_some_and(|r|input.pointer.is_some_and(|p|r.contains(p.into()))) {
                self.menu_focus.hovered=Some(focus);
                if clicked {let forwarded=InputFrame{pressed:[self.controls.binding(action).clone()].into(),viewport:input.viewport,..Default::default()};self.dispatch_input_at(&forwarded,captured_at);return true;}
            }
        }
        for (index,(rect,action,_,_)) in Self::hud_shortcuts(w,h).into_iter().enumerate() {
            if input.pointer.is_some_and(|p|rect.contains(p.into())) {
                self.menu_focus.hovered=Some(50_000+index);
                if clicked {
                    let mut forwarded=InputFrame::default();forwarded.viewport=input.viewport;
                    if let Some(action)=action { forwarded.pressed.insert(self.controls.binding(action).clone()); }
                    else { forwarded.pause=true; }
                    self.dispatch_input_at(&forwarded,captured_at);return true;
                }
            }
        }
        false
    }
}

impl AsciiApp {
    pub(super) fn help_command_lines(&self) -> Vec<String> {
        let mut lines=vec![
                format!("Déplacement · {} {} {} {}. Un déplacement réussi fait avancer le temps.",self.controls.label(Action::MoveNorth),self.controls.label(Action::MoveWest),self.controls.label(Action::MoveSouth),self.controls.label(Action::MoveEast)),
                format!("Interagir · {} près d'une installation, d'un objet ou d'un habitant. Plusieurs possibilités ouvrent un choix.",self.controls.label(Action::Interact)),
                format!("Cibler · {} ou clic sur une entité visible. Attaquer · {}. Une attaque de zone présente sa zone avant confirmation ; Échap l'annule.",self.controls.label(Action::CycleTarget),self.controls.label(Action::Attack)),
                format!("Patienter · {}. Techniques actives · {}. Les coûts et conditions de chaque technique figurent dans les compétences.",self.controls.label(Action::Wait),self.controls.label(Action::QuickTechniques)),
                format!("Inventaire · {}. Personnage · {}. Compétences · {}. Ouvrir ces écrans ne dépense pas de tour.",self.controls.label(Action::Inventory),self.controls.label(Action::Character),self.controls.label(Action::Skills)),
                format!("Quêtes · {}. Archives découvertes · {}. Historique des tours · {}.",self.controls.label(Action::QuestJournal),self.controls.label(Action::Report),self.controls.label(Action::EventHistory)),
                "Échap ferme d'abord l'écran ouvert. Depuis le jeu, il ouvre la pause. Les commandes peuvent être réattribuées dans Options / Commandes.".to_owned(),
            ];
        lines.push(format!("Aide : {} · Inspection de l'état et de la cible : {}.",self.controls.label(Action::Legend),self.controls.label(Action::Inspect)));
        lines.push(format!("Navigation de l'aide : {} / {} pour les onglets ; {} / {} pour défiler. Molette et Page préc./suiv. sont également disponibles.",self.controls.label(Action::MenuLeft),self.controls.label(Action::MenuRight),self.controls.label(Action::MenuUp),self.controls.label(Action::MenuDown)));
        for action in [Action::Slot1,Action::Slot2,Action::Slot3,Action::Analyze,Action::Traces,Action::Walls,Action::Threat,Action::Multiple,Action::NpcVision,Action::Restart] {lines.push(format!("{} · {}",action.name(),self.controls.label(action)));}
        lines
    }

    pub(super) fn update_help(&mut self, input: &InputFrame) {
        let (w,h) = input.viewport.unwrap_or((1280.0,800.0));
        let panel = reader_panel(w,h);
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let hovered = input.pointer.is_some_and(|p| reader_close(panel).contains(p.into()));
        self.menu_focus.hovered = hovered.then_some(0);
        if clicked && hovered { self.legend_open = false; return; }
        let mut tab = self.ux.help_tab;
        if self.controls.pressed(Action::MenuLeft,input) { tab = tab.saturating_sub(1); }
        if self.controls.pressed(Action::MenuRight,input) { tab = (tab+1).min(2); }
        for index in 0..3 {
            let rect = Rect::new(panel.x + 24.0 + index as f32 * (panel.w - 48.0) / 3.0, panel.y + 74.0, (panel.w - 60.0)/3.0, 32.0);
            if input.pointer.is_some_and(|p| rect.contains(p.into())) {
                self.menu_focus.hovered = Some(index+1);
                if clicked { tab = index; }
            }
        }
        if tab != self.ux.help_tab { self.ux.help_tab = tab; self.ux.help_scroll.offset = 0.0; }
        self.ux.help_scroll.update(input,&self.controls);
    }

    pub(super) fn draw_help(&self) {
        let panel = reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());
        draw_reader("Aide", "", panel,self.menu_focus.hovered == Some(0), &self.controls);
        for (index,label) in ["Commandes essentielles", "Symboles", "Règles utiles"].into_iter().enumerate() {
            let rect = Rect::new(panel.x + 24.0 + index as f32 * (panel.w - 48.0)/3.0,panel.y+74.0,(panel.w-60.0)/3.0,32.0);
            UiTheme.button(rect,label,self.menu_focus.hovered == Some(index+1),self.ux.help_tab == index,true,ButtonTone::Secondary);
        }
        let body = reader_body(panel);
        if self.ux.help_tab == 1 {
            let bottom = crate::terminal_view::draw_symbol_guide(&self.game, body, self.ux.help_scroll.offset);
            self.ux.help_scroll.finish(body,bottom);
            return;
        }
        crate::ui_theme::begin_text_pane(body,self.ux.help_scroll.offset);
        let lines = if self.ux.help_tab == 0 {
            self.help_command_lines()
        } else {
            vec![
                "Perception et mémoire · Les cases éclairées sont observées maintenant. Une case mémorisée décrit la dernière observation, pas la situation actuelle. L'inspection ne révèle aucune information cachée.".to_owned(),
                "Combat · Les statistiques d'une cible sont complétées par l'analyse. Une attaque de terrain peut être confirmée sans cible dans sa zone.".to_owned(),
                "Temps des techniques · Préparation : tours avant l'effet. Action : résolution. Récupération : tours nécessaires ensuite. E = énergie, H = chaleur, B = bande passante.".to_owned(),
                "Progression · Les points de compétence peuvent être dépensés plus tard. Une technique apprise fonctionne indépendamment de l'équipement ; l'équipement peut l'améliorer.".to_owned(),
                "Protection locale · Une zone protégée interdit certaines agressions. Une alarme réseau peut néanmoins verrouiller ses accès : ces protections et alarmes ne sont pas équivalentes.".to_owned(),
                "Suspension · Sauvegarder et quitter suspend votre partie. Reprendre continue au même endroit et consomme ce point de reprise ; vous pourrez suspendre à nouveau.".to_owned(),
                "Confort · Taille d'interface, contraste renforcé et animations réduites se règlent dans Options / Affichage. Souris et clavier restent utilisables ensemble.".to_owned(),
            ]
        };
        let mut y = body.y + 24.0;
        for line in lines { y = draw_wrapped_text(&line,body.x,y,body.w-16.0,usize::MAX,17,UiTheme.text())+22.0; }
        crate::ui_theme::end_text_pane();
        self.ux.help_scroll.finish(body,y);
    }

    pub(super) fn update_history(&mut self,input: &InputFrame) {
        let (w,h)=input.viewport.unwrap_or((1280.0,800.0));
        let close=reader_close(reader_panel(w,h));
        let hovered=input.pointer.is_some_and(|p|close.contains(p.into()));
        self.menu_focus.hovered=hovered.then_some(0);
        if self.controls.pressed(Action::EventHistory,input) || input.pressed.contains(&controls::Binding::MouseLeft) && hovered { self.ux.history_open=false; }
        self.ux.history_scroll.update(input,&self.controls);
    }

    pub(super) fn draw_history(&self) {
        let panel=reader_panel(self.ui_width(),self.ui_height());
        draw_rectangle(0.0,0.0,self.ui_width(),self.ui_height(),UiTheme.backdrop());
        draw_reader("Historique des tours","Événements observés · les plus récents en premier",panel,self.menu_focus.hovered==Some(0), &self.controls);
        let body=reader_body(panel);
        crate::ui_theme::begin_text_pane(body,self.ux.history_scroll.offset);
        let mut y=body.y+24.0;
        let mut turn=None;
        let history = if self.ux.history.is_empty() { self.log.iter().map(|m|(self.game.turn(),m.clone())).collect() } else { self.ux.history.clone() };
        // Reverse turn groups, preserving event order within each turn.
        let mut end=history.len();
        while end>0 {
            let t=history[end-1].0;
            let start=history[..end].iter().rposition(|(a,_)|*a!=t).map_or(0,|i|i+1);
            for (t,message) in &history[start..end] {
                if turn!=Some(*t) { draw_text_bold(format!("Tour {t}"),body.x,y,18.0,UiTheme.accent()); y+=30.0; turn=Some(*t); }
                y=draw_wrapped_text(message,body.x+12.0,y,body.w-28.0,usize::MAX,16,UiTheme.text())+9.0;
            }
            y+=20.0;end=start;
        }
        crate::ui_theme::end_text_pane();
        self.ux.history_scroll.finish(body,y);
    }
}
