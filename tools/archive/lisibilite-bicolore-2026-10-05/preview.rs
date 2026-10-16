//! Isolated visual comparison. Uses live floor/blood drawing, reads proposal
//! PNGs from this folder and never changes a save, active atlas or game state.
#![allow(dead_code)]
use macroquad::prelude::*;
use project_rl::world::GridPos;
#[path = "../../src/test_sector.rs"] mod test_sector;
#[path = "../../src/terminal_floor.rs"] mod terminal_floor;
#[path = "../../src/terminal_materials.rs"] mod terminal_materials;
#[path = "../../src/terminal_art.rs"] mod terminal_art;
#[path = "../../src/combat_visuals.rs"] mod combat_visuals;
use test_sector::Decor;
// The comparison draws blood without a corpse; no fake corpse art is supplied.
mod game_art {
    pub fn draw_corpse(_: macroquad::prelude::Rect, _: u8, _: f32) { unreachable!() }
}
mod terminal_view {
    pub fn draw_corpse_symbol(_: macroquad::prelude::Rect, _: char, _: f32) { unreachable!() }
}
fn dim(color: Color, visible: bool) -> Color {
    if visible { color } else { Color::new(color.r*0.45,color.g*0.5,color.b*0.55,1.0) }
}
fn conf() -> Conf {
    Conf { window_title: "Project RL - comparaison isolee 30px".into(),
        window_width: 1170, window_height: 780, window_resizable: false,
        high_dpi: false, ..Default::default() }
}
fn read(path: &str) -> Texture2D {
    let image = Image::from_file_with_format(&std::fs::read(path).unwrap(),Some(ImageFormat::Png)).unwrap();
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}
fn blit(texture: &Texture2D, target: Rect, source: Option<Rect>) {
    draw_texture_ex(texture,target.x,target.y,WHITE,DrawTextureParams {
        dest_size: Some(target.size()), source, ..Default::default()
    });
}
fn floor(x: f32, y: f32, scale: f32, kind: usize) {
    let decor = [Decor::RuinFloor,Decor::Deck,Decor::Grate,Decor::RuinFloor][kind];
    for row in 0..2 {
        for col in 0..2 {
            let rect = Rect::new(x+col as f32*32.*scale,y+row as f32*32.*scale,32.*scale,32.*scale);
            terminal_floor::draw(rect,decor,true,GridPos::new(col,row));
        }
    }
    if kind == 3 {
        combat_visuals::draw_residue(Rect::new(x+16.*scale,y+32.*scale,32.*scale,32.*scale),
            &combat_visuals::ResidueCell { blood: 6, seed: 931, ..Default::default() },true);
    }
}
#[macroquad::main(conf)]
async fn main() {
    let root = "artifacts/lisibilite-bicolore-2026-10-05";
    let mode = std::env::args().nth(1).unwrap_or("outfits".into());
    let old: Vec<_> = ["technicien","bastion"].iter().flat_map(|name|
        (0..4).map(move |pose|read(&format!("artifacts/lisibilite-sprites-2026-10-05/{name}-{pose}-48.png")))).collect();
    let proposed: Vec<_> = ["technicien","bastion"].iter().flat_map(|name|
        (0..4).map(move |pose| read(&format!("{root}/{name}-{pose}-48.png")))).collect();
    let old_guns = read("assets/sprites/cyberpunk/ranged-families-32.png");
    let old_melee = read("assets/sprites/cyberpunk/ground-32.png");
    let new_guns: Vec<_> = if mode == "weapons" {
        ["fusil","pistolet","epee"].iter().map(|name|read(&format!("{root}/{name}-32.png"))).collect()
    } else { vec![] };
    let labels = ["BETON","METAL","GRILLE SOMBRE","SOL AVEC SANG"];
    for frame in 0..3 {
        clear_background(Color::from_rgba(13,20,24,255));
        draw_text(if mode=="weapons" {"ARMES AU SOL / 32 PX / AVANT ET PROPOSITION"} else {"CORPS DE 30 PX / ESSAI 1 ET VERSION BICOLORE"},18.,30.,24.,WHITE);
        draw_text("Rendu Macroquad isole - sols et sang du jeu - pixels natifs sans lissage",18.,55.,18.,LIGHTGRAY);
        for (kind,label) in labels.iter().enumerate() {
            let x=180.+kind as f32*244.;
            draw_text(label,x,85.,18.,Color::from_rgba(164,219,219,255));
            draw_text("Essai 1",x,108.,16.,LIGHTGRAY);
            draw_text("Bicolore",x+80.,108.,16.,LIGHTGRAY);
        }
        if mode == "outfits" {
            for row in 0..8 {
                let y=121.+row as f32*80.;
                let name=if row<4 {"Technicien"} else {"Bastion"};
                draw_text(name,18.,y+28.,18.,WHITE);
                draw_text(["Sans arme","Epee","Pistolet","Fusil"][row%4],18.,y+49.,17.,LIGHTGRAY);
                for kind in 0..4 {
                    let x=180.+kind as f32*244.;
                    for offset in [0.,80.] { floor(x+offset,y,1.,kind); }
                    
                    // Same 48px destination used by the game's 32px tile renderer:
                    // tile at (x+16,y+32), canvas starts 8px left and 16px above.
                    blit(&old[row],Rect::new(x+8.,y+16.,48.,48.),None);
                    blit(&proposed[row],Rect::new(x+88.,y+16.,48.,48.),None);
                }
            }
        } else {
            for row in 0..3 {
                let y=125.+row as f32*200.;
                draw_text(["Fusil d'assaut","Pistolet","Epee"][row],18.,y+34.,18.,WHITE);
                draw_text("Arme : 32px / zoom x2",18.,y+55.,15.,LIGHTGRAY);
                for kind in 0..4 {
                    let x=180.+kind as f32*244.;
                    let (source,index)=if row<2 {(&old_guns,[0,8][row])} else {(&old_melee,1)};
                    let src=Some(Rect::new(index as f32*32.,0.,32.,32.));
                    for (zoom,extra) in [(1.,0.),(2.,73.)] {
                        for offset in [0.,80.] {
                            // Zoom panel uses a 32px region from the same floor.
                            floor(x+offset,y+extra,1.,kind);
                        }
                        blit(source,Rect::new(x+(64.-32.*zoom)/2.,y+extra+(64.-32.*zoom)/2.,32.*zoom,32.*zoom),src);
                        blit(&new_guns[row],Rect::new(x+80.+(64.-32.*zoom)/2.,y+extra+(64.-32.*zoom)/2.,32.*zoom,32.*zoom),None);
                    }
                }
            }
        }
        draw_text("Comparaison avant validation : aucune texture de personnage ou d'objet remplacee dans la partie.",18.,766.,16.,LIGHTGRAY);
        if frame == 2 {
            let mut capture = get_screen_data();
            for pixel in capture.bytes.chunks_exact_mut(4) { pixel[3] = 255; }
            capture.export_png(&format!("{root}/capture-{mode}-sols.png"));
        }
        next_frame().await;
    }
}
