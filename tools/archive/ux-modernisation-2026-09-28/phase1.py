from pathlib import Path

def edit(path, old, new, count=1):
    p=Path(path); s=p.read_text(encoding='utf-8')
    assert s.count(old)>=count, (path, old[:100], s.count(old))
    p.write_text(s.replace(old,new,count),encoding='utf-8',newline='\n')

a='src/ascii_app.rs'; p='src/pause_menu.rs'; t='src/ui_theme.rs'
s=Path(a).read_text(encoding='utf-8')
start=s.index('            let next_seed = self.seed.wrapping_add(1);',s.index('if self.controls.pressed(Action::Restart, input)'))
end=s.index('            return;\n        }',start)
body=s[start:end]
s=s[:start]+'''            if self.game.status() == RunStatus::Active {
                self.open_menu(MenuScreen::ConfirmRestart);
            } else {
                self.restart_current_profile();
            }
'''+s[end:]
insert=s.index('    fn open_menu(&mut self, menu: MenuScreen)')
s=s[:insert]+'    fn restart_current_profile(&mut self) {\n'+body+'    }\n\n'+s[insert:]
Path(a).write_text(s,encoding='utf-8',newline='\n')
edit(a,'let requested_slot = hovered_action','let requested_slot = mouse_action')
edit(a,'            if entries > 0 {\n                if self.controls.pressed(Action::MenuUp, input) {','''            if entries > 0 {
                if input.wheel_y != 0.0 {
                    let steps = wheel_steps(input.wheel_y);
                    self.npc_trade_selection = if input.wheel_y > 0.0 {
                        self.npc_trade_selection.saturating_sub(steps)
                    } else {
                        self.npc_trade_selection.saturating_add(steps).min(entries - 1)
                    };
                }
                if self.controls.pressed(Action::MenuUp, input) {''')
edit(p,'    ConfirmNewRun,','    ConfirmNewRun,\n    ConfirmRestart,')
edit(p,'            Self::Pause => Self::Hidden,','            Self::Pause | Self::ConfirmRestart => Self::Hidden,')
edit(p,'            Self::ConfirmNewRun => "COMMENCER UNE NOUVELLE PARTIE ?",','            Self::ConfirmNewRun => "Remplacer la partie suspendue ?",\n            Self::ConfirmRestart => "Recommencer cette partie ?",')
edit(p,'            Self::ConfirmAbandon => &["Annuler", "Confirmer"],','            Self::ConfirmAbandon => &["Annuler", "Abandonner la partie"],')
edit(p,'            Self::ConfirmNewRun => &["Annuler", "Confirmer"],','            Self::ConfirmNewRun => &["Annuler", "Remplacer la partie"],\n            Self::ConfirmRestart => &["Continuer la partie", "Recommencer"],')
edit(a,'                (MenuScreen::ConfirmNewRun, 0) =>','                (MenuScreen::ConfirmRestart, 0) => self.open_menu(MenuScreen::Hidden),\n                (MenuScreen::ConfirmRestart, 1) => self.restart_current_profile(),\n                (MenuScreen::ConfirmNewRun, 0) =>')
edit(a,'        } else if self.menu == MenuScreen::ConfirmNewRun {','        } else if self.menu == MenuScreen::ConfirmRestart {\n            "Votre progression actuelle sera perdue. Une nouvelle partie débutera avec le même profil."\n        } else if self.menu == MenuScreen::ConfirmNewRun {')
edit(a,'MenuScreen::ConfirmAbandon | MenuScreen::ConfirmNewRun if index == 1','MenuScreen::ConfirmAbandon | MenuScreen::ConfirmNewRun | MenuScreen::ConfirmRestart if index == 1')
edit(a,'Sauvegarder et quitter permet une reprise unique. Aucun tour ne s\'écoule dans le menu.','Vous pouvez suspendre et reprendre votre partie à chaque session. Aucun tour ne s\'écoule ici.')
edit(a,'Une partie suspendue est disponible. La reprise reste unique et ne permet aucun retour en arrière.','Une partie suspendue est disponible. Reprenez là où vous vous êtes arrêté ; vous pourrez suspendre à nouveau.')

# Shared palette: attention remains amber; interaction uses turquoise consistently.
edit(t,'    pub const fn focus(self) -> Color {\n        Color::new(1.0, 0.83, 0.36, 1.0)\n    }','''    pub const fn focus(self) -> Color {
        self.accent()
    }

    pub const fn attention(self) -> Color {
        Color::new(1.0, 0.83, 0.36, 1.0)
    }''')
edit(t,'Color::new(0.024, 0.055, 0.075, 0.98)','Color::new(0.035, 0.055, 0.073, 0.99)')
edit(t,'Color::new(0.045, 0.105, 0.13, 1.0)','Color::new(0.065, 0.094, 0.117, 1.0)')
edit(t,'subdued(self.accent(), 0.34)','subdued(self.muted(), 0.20)')
edit(t,'subdued(self.muted(), 0.28)','subdued(self.muted(), 0.12)')
start=Path(t).read_text(encoding='utf-8').index('        let semantic = match tone',Path(t).read_text(encoding='utf-8').index('    pub fn button('))
s=Path(t).read_text(encoding='utf-8'); end=s.index('        let radius =',start)
s=s[:start]+'''        let semantic = match tone {
            ButtonTone::Secondary | ButtonTone::Primary => self.accent(),
            ButtonTone::Danger => self.danger(),
        };
        let fill = if !enabled {
            self.surface()
        } else {
            match tone {
                ButtonTone::Primary => if focused { Color::new(0.53, 1.0, 0.88, 1.0) } else { self.accent() },
                ButtonTone::Danger => Color::new(0.25, 0.085, 0.08, 1.0),
                ButtonTone::Secondary if focused || active => self.surface_selected(),
                ButtonTone::Secondary => self.surface_raised(),
            }
        };
        let outline = if !enabled { subdued(self.muted(), 0.15) }
            else if focused { self.text() }
            else if active || tone != ButtonTone::Secondary { semantic }
            else { subdued(self.muted(), 0.15) };
'''+s[end:]
old_start=s.index('            if !enabled {',s.index('draw_text_bold_centered(',start));old_end=s.index('\n        );',old_start)
s=s[:old_start]+'            button_foreground(self, focused, active, enabled, tone),'+s[old_end:]
start=s.index('    if !enabled {',s.index('fn button_foreground('));end=s.index('\n}\n',start)
s=s[:start]+'''    if !enabled {
        theme.muted()
    } else if tone == ButtonTone::Primary {
        Color::new(0.025, 0.12, 0.13, 1.0)
    } else if tone == ButtonTone::Danger {
        theme.danger()
    } else if focused || active {
        theme.accent()
    } else {
        theme.text()
    }'''+s[end:]
Path(t).write_text(s,encoding='utf-8',newline='\n')
