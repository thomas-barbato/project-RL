//! Native comparison only. Live floor/blood functions, no game-state changes.
#![allow(dead_code)]
use macroquad::prelude::*;
use project_rl::world::GridPos;
#[path="../../src/test_sector.rs"] mod test_sector;
#[path="../../src/terminal_floor.rs"] mod terminal_floor;
#[path="../../src/terminal_materials.rs"] mod terminal_materials;
#[path="../../src/terminal_art.rs"] mod terminal_art;
#[path="../../src/combat_visuals.rs"] mod combat_visuals;
mod game_art {pub fn draw_corpse(_:macroquad::prelude::Rect,_:u8,_:f32){unreachable!()}}
mod terminal_view {pub fn draw_corpse_symbol(_:macroquad::prelude::Rect,_:char,_:f32){unreachable!()}}
fn dim(c:Color,visible:bool)->Color{if visible{c}else{Color::new(c.r*0.45,c.g*0.5,c.b*0.55,1.)}}
fn conf()->Conf {Conf{window_title:"Project RL - apercu numerote 30px".into(),window_width:1170,window_height:770,window_resizable:false,high_dpi:false,..Default::default()}}
fn texture(path:&str)->Texture2D {let t=Texture2D::from_file_with_format(&std::fs::read(path).unwrap(),Some(ImageFormat::Png));t.set_filter(FilterMode::Nearest);t}
fn floor(x:f32,y:f32,kind:usize){
    use test_sector::Decor;
    for row in 0..2 {for col in 0..2 {
        terminal_floor::draw(Rect::new(x+col as f32*32.,y+row as f32*32.,32.,32.),[Decor::RuinFloor,Decor::Deck,Decor::Grate,Decor::RuinFloor][kind],true,GridPos::new(col,row));
    }}
    if kind==3 {combat_visuals::draw_residue(Rect::new(x+16.,y+32.,32.,32.),&combat_visuals::ResidueCell{blood:6,seed:931,..Default::default()},true);}
}
#[macroquad::main(conf)]
async fn main(){
    let root="artifacts/armures-bicolores-completes-2026-10-05";
    let models:serde_json::Value=serde_json::from_slice(&std::fs::read(format!("{root}/modeles.json")).unwrap()).unwrap();
    let old:Vec<_>=[0,2].into_iter().map(|p|texture(&format!("{root}/avant-48-{p}.png"))).collect();
    let new:Vec<_>=[0,2].into_iter().map(|p|texture(&format!("{root}/apres-48-{p}.png"))).collect();
    for palette in 0..2 {
        for start in [0,6,12] {
            let count=6.min(17-start);
            for frame in 0..3 {
                clear_background(Color::from_rgba(16,23,29,255));
                draw_text("17 TENUES NUMEROTEES / AVANT ET PROPOSITION / CORPS DE 30 PX",18.,30.,24.,WHITE);
                draw_text(if palette==0{"Palette d'origine proposee / pixels natifs sans lissage"}else{"Variante bleue / comparaison des anciennes marques chaudes et du duo propose"},18.,57.,17.,LIGHTGRAY);
                for (kind,label) in ["BETON","METAL","GRILLE SOMBRE","SOL AVEC SANG"].into_iter().enumerate(){
                    let x=230.+kind as f32*232.;draw_text(label,x,94.,17.,Color::from_rgba(107,212,213,255));
                    draw_text("Actuel",x,116.,16.,LIGHTGRAY);draw_text("Essai",x+84.,116.,16.,LIGHTGRAY);
                }
                for row in 0..count {
                    let index=start+row;let y=140.+row as f32*95.;
                    draw_text(&format!("{:02}",index+1),18.,y+24.,23.,Color::from_rgba(107,212,213,255));
                    let name=models[index]["name"].as_str().unwrap();
                    draw_text(name,18.,y+45.,15.,WHITE);
                    // Same pose on each pair; all four floor materials.
                    let pose=[0,1,2,3,4,7][row];
                    draw_text(["Sans arme","Epee","Fusil","Pistolet","Deux mains","Mixte"][row],18.,y+65.,15.,LIGHTGRAY);
                    for kind in 0..4 {
                        let x=230.+kind as f32*232.;
                        for (offset,texture) in [(0.,&old[palette]),(84.,&new[palette])] {
                            floor(x+offset,y,kind);
                            draw_texture_ex(texture,x+offset+8.,y+16.,WHITE,DrawTextureParams{
                                source:Some(Rect::new(pose as f32*48.,index as f32*48.,48.,48.)),dest_size:Some(vec2(48.,48.)),..Default::default()
                            });
                        }
                    }
                }
                draw_text("Apercu isole avec les sols et le sang du jeu. Aucun atlas actif remplace. Meme silhouette et collision.",18.,752.,16.,LIGHTGRAY);
                if frame==2 {let mut capture=get_screen_data();for pixel in capture.bytes.chunks_exact_mut(4){pixel[3]=255;}
                    capture.export_png(&format!("{root}/sols-{:02}-{:02}-{}.png",start+1,start+count,if palette==0{"origine"}else{"bleu"}));
                }
                next_frame().await;
            }
        }
    }
}
