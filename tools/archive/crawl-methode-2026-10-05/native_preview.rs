//! Isolated Macroquad scene using the live floor and blood presentation only.
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
fn conf()->Conf {Conf{window_title:"Project RL · essai Crawl 32 pixels".into(),window_width:1170,window_height:770,window_resizable:false,high_dpi:false,..Default::default()}}
fn texture(path:&str)->Texture2D {let t=Texture2D::from_file_with_format(&std::fs::read(path).unwrap(),Some(ImageFormat::Png));t.set_filter(FilterMode::Nearest);t}
fn floor(x:f32,y:f32,kind:usize){
    use test_sector::Decor;
    for row in 0..2 {for col in 0..2 {terminal_floor::draw(Rect::new(x+col as f32*32.,y+row as f32*32.,32.,32.),[Decor::RuinFloor,Decor::Deck,Decor::Grate,Decor::RuinFloor][kind],true,GridPos::new(col,row));}}
    if kind==3 {combat_visuals::draw_residue(Rect::new(x+16.,y+32.,32.,32.),&combat_visuals::ResidueCell{blood:6,seed:931,..Default::default()},true);}
}
fn save(name:&str){let mut capture=get_screen_data();for p in capture.bytes.chunks_exact_mut(4){p[3]=255;}capture.export_png(&format!("artifacts/crawl-methode-2026-10-05/{name}.png"));}
#[macroquad::main(conf)]
async fn main(){
    let root="artifacts/crawl-methode-2026-10-05";
    let models:serde_json::Value=serde_json::from_slice(&std::fs::read(format!("{root}/manifest.json")).unwrap()).unwrap();
    let figures=texture(&format!("{root}/figures-48.png"));
    let creatures=texture(&format!("{root}/monsters-32.png"));
    let weapons=texture(&format!("{root}/weapons-32.png"));
    let npcs=texture(&format!("{root}/npc-figures-48.png"));
    for start in [0,6,12] {let count=6.min(17-start);for frame in 0..3 {
        clear_background(Color::from_rgba(16,23,29,255));
        draw_text("ESSAI CRAWL / TILES DE 32 PIXELS / 17 TENUES NUMEROTEES",18.,30.,24.,WHITE);
        draw_text("Un corps fixe, prises et armes separees / taille native, sans lissage",18.,57.,17.,LIGHTGRAY);
        for (kind,label) in ["BETON","METAL","GRILLE SOMBRE","SOL AVEC SANG"].into_iter().enumerate(){let x=230.+kind as f32*232.;draw_text(label,x,94.,17.,Color::from_rgba(107,212,213,255));draw_text("Sans arme",x,116.,16.,LIGHTGRAY);draw_text("Equipe",x+84.,116.,16.,LIGHTGRAY);}
        for row in 0..count {let index=start+row;let y=140.+row as f32*95.;
            draw_text(&format!("{:02}",index+1),18.,y+24.,23.,Color::from_rgba(107,212,213,255));
            draw_text(models["armor"][index]["name"].as_str().unwrap(),18.,y+45.,15.,WHITE);
            let pose=[1,2,3,4,6,7][row];
            for kind in 0..4 {let x=230.+kind as f32*232.;for (offset,p) in [(0.,0),(84.,pose)] {
                floor(x+offset,y,kind);
                draw_texture_ex(&figures,x+offset+8.,y+24.,WHITE,DrawTextureParams{source:Some(Rect::new(p as f32*48.,index as f32*48.,48.,48.)),dest_size:Some(vec2(48.,48.)),..Default::default()});
            }}
        }
        draw_text("Apercu isole avec les sols du jeu. Sources actives inchangees. Les raccords des membres restent a examiner.",18.,752.,16.,LIGHTGRAY);
        if frame==2 {save(&format!("sols-{:02}-{:02}",start+1,start+count));}next_frame().await;
    }}
    for frame in 0..3 {
        clear_background(Color::from_rgba(16,23,29,255));
        draw_text("SCENE NATIVE / JOUEURS, ARMES, NPC ET MONSTRES / CASES DE 32 PIXELS",20.,30.,22.,WHITE);
        draw_text("Apercu de presentation isole, avec le sol du jeu / aucune modification de simulation",20.,55.,17.,LIGHTGRAY);
        for y in 0..18 {for x in 0..32 {terminal_floor::draw(Rect::new(70.+x as f32*32.,95.+y as f32*32.,32.,32.),if x<16{test_sector::Decor::RuinFloor}else{test_sector::Decor::Deck},true,GridPos::new(x,y));}}
        for i in 0..17 {let pose=i%8;draw_texture_ex(&figures,82.+(i%9)as f32*112.,115.+(i/9)as f32*96.,WHITE,DrawTextureParams{source:Some(Rect::new(pose as f32*48.,i as f32*48.,48.,48.)),dest_size:Some(vec2(48.,48.)),..Default::default()});}
        for i in 0..17 {draw_texture_ex(&creatures,92.+(i%9)as f32*112.,350.+(i/9)as f32*72.,WHITE,DrawTextureParams{source:Some(Rect::new((i%6)as f32*32.,(i/6)as f32*32.,32.,32.)),dest_size:Some(vec2(32.,32.)),..Default::default()});}
        for i in 0..6 {let pose=[1,2,4,0,0,3][i];draw_texture_ex(&npcs,90.+i as f32*168.,485.,WHITE,DrawTextureParams{source:Some(Rect::new(pose as f32*48.,i as f32*48.,48.,48.)),dest_size:Some(vec2(48.,48.)),..Default::default()});}
        for i in 0..16 {draw_texture_ex(&weapons,94.+(i%16)as f32*63.,602.,WHITE,DrawTextureParams{source:Some(Rect::new((i%4)as f32*32.,(i/4)as f32*32.,32.,32.)),dest_size:Some(vec2(32.,32.)),..Default::default()});}
        draw_text("Tiles 1:1. Le debordement des armes est une marge de dessin ; il n'intervient pas dans les collisions.",20.,730.,17.,LIGHTGRAY);
        if frame==2 {save("scene-native");}next_frame().await;
    }
}
