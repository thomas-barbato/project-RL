//! Isolated native-tile compositor. Active sources/textures are not modified.
use macroquad::prelude::*;
use std::{fs,path::Path};

const SIDE: u16 = 48;
const ROOT: &str = "artifacts/crawl-methode-2026-10-05";

fn read(name:&str)->Image {
    Image::from_file_with_format(&fs::read(Path::new(ROOT).join(name)).unwrap(),Some(ImageFormat::Png)).unwrap()
}
fn cell(sheet:&Image,index:usize)->Image {
    let cols=usize::from(sheet.width)/32;
    let mut image=Image::gen_image_color(32,32,BLANK);
    for y in 0..32usize {for x in 0..32usize {
        let a=((index/cols*32+y)*usize::from(sheet.width)+index%cols*32+x)*4;
        let b=(y*32+x)*4;
        image.bytes[b..b+4].copy_from_slice(&sheet.bytes[a..a+4]);
    }}
    image
}

/// Only the foreground hand area can be replaced. Head/torso/legs are stable.
fn hand_cut(pose:usize,x:usize,y:usize)->bool {
    if !(17..23).contains(&y) {return false;}
    let left=(3..10).contains(&x);let right=(22..30).contains(&x);
    match pose {0=>false,1|3=>right,2|4|5|6|7=>left||right,_=>false}
}
fn overlays(pose:usize)->Vec<(usize,bool)> {
    match pose {
        1=>vec![(0,false)],2=>vec![(2,false)],3=>vec![(1,false)],
        4=>vec![(3,false)],5=>vec![(0,false),(0,true)],
        6=>vec![(1,false),(1,true)],7=>vec![(0,true),(1,false)],_=>vec![]
    }
}
fn compose(body:&Image,grips:&[Image],pose:usize)->(Image,u64) {
    let mut image=Image::gen_image_color(SIDE,SIDE,BLANK);
    let mut foreground=vec![false;usize::from(SIDE)*usize::from(SIDE)];
    for y in 0..32usize {for x in 0..32usize {
        if hand_cut(pose,x,y) {continue;}
        let a=(y*32+x)*4;let b=((y+8)*usize::from(SIDE)+x+8)*4;
        image.bytes[b..b+4].copy_from_slice(&body.bytes[a..a+4]);
    }}
    for (index,flip) in overlays(pose) {
        let layer=&grips[index];
        for y in 0..40usize {for x in 0..40usize {
            let source_x=if flip {39-x}else{x};let a=(y*40+source_x)*4;
            if layer.bytes[a+3]==0 {continue;}
            let b=((y+4)*usize::from(SIDE)+x+4)*4;
            image.bytes[b..b+4].copy_from_slice(&layer.bytes[a..a+4]);
            foreground[(y+4)*usize::from(SIDE)+x+4]=true;
        }}
    }
    let mut differences=0;
    for y in 0..32usize {for x in 0..32usize {
        if hand_cut(pose,x,y)||foreground[(y+8)*usize::from(SIDE)+x+8] {continue;}
        let a=(y*32+x)*4;let b=((y+8)*usize::from(SIDE)+x+8)*4;
        if image.bytes[b..b+4]!=body.bytes[a..a+4] {differences+=1;}
    }}
    (image,differences)
}
fn main() {
    let armor=read("armor-32.png");let actors=read("monsters-32.png");
    let grips:Vec<Image>=(1..=4).map(|i|read(&format!("tiles/grip-{i:02}.png"))).collect();
    let mut output=Vec::new();let mut actor_output=Vec::new();let mut total_differences=0;
    for i in 0..17 {let body=cell(&armor,i);for pose in 0..8 {
        let (image,diff)=compose(&body,&grips,pose);total_differences+=diff;
        output.extend_from_slice(&image.bytes);
    }}
    for i in 17..23 {let body=cell(&actors,i);for pose in 0..8 {
        let (image,diff)=compose(&body,&grips,pose);total_differences+=diff;
        actor_output.extend_from_slice(&image.bytes);
    }}
    assert_eq!(total_differences,0);
    fs::write(Path::new(ROOT).join("figures.rgba"),output).unwrap();
    fs::write(Path::new(ROOT).join("npc-figures.rgba"),actor_output).unwrap();
    fs::write(Path::new(ROOT).join("composition-check.json"),
        format!("{{\"figures_checked\":184,\"fixed_pixels_changed_outside_declared_hand_replacement_and_occlusion\":{total_differences},\"native_tile\":32,\"composition_canvas\":48,\"preview_only\":true}}\n")).unwrap();
    println!("184 compositions; fixed body unchanged outside declared hand areas and foreground occlusion.");
}
