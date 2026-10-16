from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'src/combat_visuals_capture.rs';s=p.read_text(encoding='utf-8')
s=s.replace('Trois dessins par tenue · mêlée, fusil à deux mains et pistolet levé · armes intégrées','Corps de 30 px · 1 épée courte / 2 géante / 3 fusil / 4 pistolet · trois poses ambidextres')
begin=s.index('                crate::game_art::draw_portrait(Rect::new(x + 3.0, y + 44.0')
end=s.index('\n            }\n            return;',begin)
s=s[:begin]+'''                let short = crate::game_art::Visual::Player { weapon: crate::game_art::held_weapon("core:integrity_blade"), ..melee };
                let mixed = crate::game_art::Visual::Player { hands: Some([Some(crate::game_art::WeaponHold::Raised), Some(crate::game_art::WeaponHold::Side)]), ..short };
                crate::game_art::draw_portrait(Rect::new(x + 3.0, y + 44.0, 80.0, 80.0), melee);
                crate::game_art::draw_portrait(Rect::new(x + 82.0, y + 44.0, 80.0, 80.0), mixed);
                draw_text("Grande épée / paire", x+14.0, y+139.0, 14.0, DIM);
                for (column, v) in [short, melee, visual].into_iter().enumerate() {
                    crate::game_art::draw_world(Rect::new(x+8.0+column as f32*39.0, y+178.0, 32.0,32.0),v,1.0);
                    draw_text((column+1).to_string(),x+18.0+column as f32*39.0,y+226.0,13.0,DIM);
                }
                crate::game_art::draw_pistol_reference(Rect::new(x+125.0,y+178.0,32.0,32.0),short,32);
                draw_text("4",x+135.0,y+226.0,13.0,DIM);
                for (column, hands) in [
                    [Some(crate::game_art::WeaponHold::Side),Some(crate::game_art::WeaponHold::Side)],
                    [Some(crate::game_art::WeaponHold::Raised),Some(crate::game_art::WeaponHold::Raised)],
                    [Some(crate::game_art::WeaponHold::Raised),Some(crate::game_art::WeaponHold::Side)],
                ].into_iter().enumerate() {
                    crate::game_art::draw_world(Rect::new(x+15.0+column as f32*51.0,y+265.0,32.0,32.0),crate::game_art::Visual::Player { hands: Some(hands), ..short },1.0);
                }
''' +s[end:]
p.write_text(s,encoding='utf-8',newline='\n')
