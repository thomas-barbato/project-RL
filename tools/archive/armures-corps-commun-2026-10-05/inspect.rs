//! Isolated fixed-body renderer and visual audit; no active files modified.
#![allow(dead_code)]
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
mod current;
mod proposal;
mod previous;
mod stable;
fn main(){
    let out=std::path::Path::new("artifacts/armures-corps-commun-2026-10-05");
    let root=std::path::Path::new("assets/sprites/cyberpunk");
    let poses=["unarmed","side","two-handed","raised","melee-two-handed","dual-melee","dual-raised","dual-mixed"];
    let mut checks=Vec::new();
    for side in [48_u16,128] {
        let sources:Vec<_>=poses.iter().map(|p|current::decoded(&std::fs::read(root.join(format!("player-poses-{p}-{side}.png"))).unwrap())).collect();
        for palette in EquipmentPalette::ALL {
            for flip in [false,true] {
                let mut before=Image::gen_image_color(side*8,side*17,BLANK);
                let mut after=Image::gen_image_color(side*8,side*17,BLANK);
                for outfit in 0..17 {
                    let fixed=proposal::figure(&sources[0],outfit,0,[palette,EquipmentPalette::Original,EquipmentPalette::Original]);
                    for pose in 0..8 {
                        let old=previous::figure(&sources[pose],outfit,pose,[palette,EquipmentPalette::Original,EquipmentPalette::Original]);
                        let authored=proposal::figure(&sources[pose],outfit,pose,[palette,EquipmentPalette::Original,EquipmentPalette::Original]);
                        let new=stable::figure(&sources,outfit,pose,[palette,EquipmentPalette::Original,EquipmentPalette::Original],flip);
                        let mut old_head=0;let mut old_legs=0;let mut verified=0;let mut new_diff=0;
                        let mut body_top=usize::from(side);let mut body_bottom=0;
                        for y in 0..side {for x in 0..side {
                            let i=(usize::from(y)*usize::from(side)+usize::from(x))*4;
                            let px=f32::from(x)*48.0/f32::from(side);let py=f32::from(y)*48.0/f32::from(side);
                            let old_x=if flip {side-1-x}else{x};
                            let oi=(usize::from(y)*usize::from(side)+usize::from(old_x))*4;
                            let p=&old.bytes[oi..oi+4];let q=&new.bytes[i..i+4];let master=&fixed.bytes[i..i+4];
                            if stable::body_zone(px,py) && !stable::foreground(pose,px,py) {
                                verified+=1;
                                if q!=master {new_diff+=1;}
                                if p!=master && p[3].max(master[3])>0 {
                                    if py<26.0 {old_head+=1;}
                                    if py>=36.0 {old_legs+=1;}
                                }
                                assert_eq!(q,master,"shared body {outfit}/{pose}/{palette:?}/{side}/{flip}/{x}/{y}");
                            } else if pose==0 {
                                assert_eq!(q,master,"unarmed body never flips {outfit}/{side}/{flip}");
                            } else {
                                assert_eq!(q,&authored.bytes[oi..oi+4],"authored limbs/weapon {outfit}/{pose}/{side}/{flip}/{x}/{y}");
                            }
                            if side==48 && (22..27).contains(&x) && q[3]>0 {body_top=body_top.min(usize::from(y));body_bottom=body_bottom.max(usize::from(y));}
                            before.set_pixel(pose as u32*u32::from(side)+u32::from(x),u32::from(outfit)*u32::from(side)+u32::from(y),Color::from_rgba(p[0],p[1],p[2],p[3]));
                            after.set_pixel(pose as u32*u32::from(side)+u32::from(x),u32::from(outfit)*u32::from(side)+u32::from(y),new.get_pixel(u32::from(x),u32::from(y)));
                        }}
                        if side==48 {assert_eq!(body_bottom-body_top+1,30,"body height {outfit}/{pose}/{flip}");}
                        checks.push(serde_json::json!({"outfit":outfit,"pose":pose,"palette":palette.name(),"side":side,"flip":flip,"old_head_different_pixels":old_head,"old_legs_different_pixels":old_legs,"fixed_pixels_checked":verified,"new_fixed_pixel_differences":new_diff}));
                    }
                }
                std::fs::write(out.join(format!("avant-{side}-{}-{}.rgba",palette.index(),u8::from(flip))),&before.bytes).unwrap();
                std::fs::write(out.join(format!("apres-{side}-{}-{}.rgba",palette.index(),u8::from(flip))),&after.bytes).unwrap();
            }
        }
    }
    std::fs::write(out.join("verification.json"),serde_json::to_vec_pretty(&checks).unwrap()).unwrap();
    println!("3808 cases verified: fixed body pixels identical across eight poses and both hand orders; 30px body retained.");
}
