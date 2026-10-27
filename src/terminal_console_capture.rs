//! Diagnostic scenes use the production terminal and idle renderers.
use super::*;
use crate::terminal_chrome::{heading, module};

impl AsciiApp {
    pub(super) fn draw_terminal_console_proposal(&self) {
        self.draw();
    }

    pub(super) fn draw_idle_integration_review(&self, phase: f64, reduced: bool) {
        clear_background(Color::from_rgba(16, 23, 29, 255));
        let time = (!reduced).then_some(phase);
        draw_text_bold(
            "RESPIRATION INTÉGRÉE / RENDU DU JEU",
            26.0,
            38.0,
            24.0,
            UiTheme.text(),
        );
        draw_text(
            if reduced {
                "Mouvements réduits : personnage fixe"
            } else {
                "1 pixel · pieds fixes · mains et armes solidaires · cycle de 3 secondes"
            },
            26.0,
            67.0,
            17.0,
            UiTheme.muted(),
        );
        let pistol = crate::game_art::held_weapon("core:modele_cogneur_p01");
        let blade = crate::game_art::held_weapon("core:integrity_blade");
        let rifle = crate::game_art::held_weapon("core:fusil_d_assaut");
        let greatsword = crate::game_art::held_weapon("core:melee_epee_longue");
        for (column, (label, weapon, hands)) in [
            ("PISTOLET", pistol, None),
            ("FUSIL", rifle, None),
            ("ÉPÉE", greatsword, None),
            ("DEUX MAINS ACTIVES", None, Some([pistol, blade])),
        ]
        .into_iter()
        .enumerate()
        {
            let x = 30.0 + column as f32 * 312.0;
            module(Rect::new(x, 95.0, 292.0, 291.0), label);
            let visual = crate::game_art::Visual::Player {
                body: 0,
                equipment: [Some(107), None, None, None],
                palettes: Default::default(),
                weapon,
                hands,
            };
            for (side, xx, yy) in [(128.0, x + 52.0, 140.0), (32.0, x + 232.0, 284.0)] {
                crate::terminal_view::draw_scenery_preview(
                    Rect::new(xx, yy, side, side),
                    crate::test_sector::Decor::Deck,
                    GridPos::new(3, 3),
                );
                crate::game_art::draw_world_animated(
                    Rect::new(xx, yy, side, side),
                    visual,
                    1.0,
                    time,
                );
            }
            draw_text("×4", x + 98.0, 353.0, 15.0, UiTheme.muted());
            draw_text("32 px", x + 226.0, 353.0, 13.0, UiTheme.muted());
        }
        heading(
            "LES 17 TENUES / TAILLE NATIVE",
            30.0,
            443.0,
            16,
            UiTheme.accent(),
        );
        for outfit in 0..17 {
            let x = 48.0 + (outfit % 9) as f32 * 138.0;
            let y = 480.0 + (outfit / 9) as f32 * 110.0;
            let visual = crate::game_art::Visual::Player {
                body: outfit.min(2) as u8,
                equipment: [
                    (outfit >= 3).then_some((98 + outfit) as u16),
                    None,
                    None,
                    None,
                ],
                palettes: Default::default(),
                weapon: pistol,
                hands: None,
            };
            crate::terminal_view::draw_scenery_preview(
                Rect::new(x, y, 32.0, 32.0),
                crate::test_sector::Decor::Deck,
                GridPos::new(3, 3),
            );
            crate::game_art::draw_world_animated(Rect::new(x, y, 32.0, 32.0), visual, 1.0, time);
            draw_text(
                format!("{:02}", outfit + 1),
                x,
                y + 55.0,
                14.0,
                UiTheme.text(),
            );
        }
    }
}
