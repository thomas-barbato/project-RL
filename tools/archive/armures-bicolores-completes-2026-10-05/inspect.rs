//! Isolated proposal verification; no application or active asset changes.
#![allow(dead_code)]
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
mod current;
mod proposal;
fn main() {
    let root = std::path::Path::new("assets/sprites/cyberpunk");
    let output = std::path::Path::new("artifacts/armures-bicolores-completes-2026-10-05");
    let poses = ["unarmed","side","two-handed","raised","melee-two-handed","dual-melee","dual-raised","dual-mixed"];
    let mut checks = Vec::new();
    for side in [48_u16,128] {
        let sources:Vec<_> = poses.iter().map(|pose|current::decoded(&std::fs::read(root.join(format!("player-poses-{pose}-{side}.png"))).unwrap())).collect();
        for palette in EquipmentPalette::ALL {
            let mut before = Image::gen_image_color(side*8,side*17,BLANK);
            let mut after = Image::gen_image_color(side*8,side*17,BLANK);
            for (pose,source) in sources.iter().enumerate() {
                for outfit in 0..17 {
                    let colors = [palette,EquipmentPalette::Original,EquipmentPalette::Original];
                    let old = current::figure(source,outfit,pose,colors);
                    let new = proposal::figure(source,outfit,pose,colors);
                    let original = current::figure(source,outfit,pose,[EquipmentPalette::Original;3]);
                    let mut primary=0;let mut accent=0;let mut protected=0;let mut weapon=0;
                    let mut old_orange=0;let mut new_orange=0;
                    for (i,(p,q)) in original.bytes.chunks_exact(4).zip(new.bytes.chunks_exact(4)).enumerate() {
                        assert_eq!(p[3],q[3],"alpha {outfit}/{pose}/{palette:?}/{side}/{i}");
                        if p[3]==0 { continue; }
                        let x=(i%usize::from(side)) as f32*48.0/f32::from(side);
                        let y=(i/usize::from(side)) as f32*48.0/f32::from(side);
                        let pixel:[u8;4]=p.try_into().unwrap();
                        match proposal::material(outfit,pose,x,y,pixel) {
                            Some(0) => {
                                if current::visor_pixel(outfit,x,y,pixel) || current::face_pixel(outfit,x,y,pixel) {
                                    assert_eq!(p,q,"skin/light {outfit}/{pose}/{palette:?}/{side}/{i}");protected+=1;
                                } else {
                                    if p!=q {
                                        if proposal::secondary(outfit,x,y) { accent+=1; } else { primary+=1; }
                                    }
                                    if palette==EquipmentPalette::Blue {
                                        let previous=&old.bytes[i*4..i*4+4];
                                        let orange=|v:&[u8]|v[0]>150 && v[1]>90 && v[2]<65;
                                        if orange(previous) {old_orange+=1;}
                                        if orange(q) {new_orange+=1;}
                                    }
                                }
                            }
                            Some(_) | None => {assert_eq!(p,q,"weapon/steel {outfit}/{pose}/{side}/{i}");weapon+=1;}
                        }
                    }
                    assert!(primary>=5 && accent>=5,"two materials {outfit}/{pose}/{palette:?}/{side}: {primary}/{accent}");
                    if palette==EquipmentPalette::Blue {assert_eq!(new_orange,0,"orange outside protected/held zones {outfit}/{pose}/{side}");}
                    checks.push(serde_json::json!({"outfit":outfit,"pose":pose,"palette":palette.name(),"side":side,"primary":primary,"secondary":accent,"skin_lights":protected,"held_unchanged":weapon,"alpha_unchanged":true,"blue_body_orange_before":old_orange,"blue_body_orange_after":new_orange}));
                    for y in 0..side {for x in 0..side {
                        before.set_pixel(pose as u32*u32::from(side)+u32::from(x),u32::from(outfit)*u32::from(side)+u32::from(y),old.get_pixel(u32::from(x),u32::from(y)));
                        after.set_pixel(pose as u32*u32::from(side)+u32::from(x),u32::from(outfit)*u32::from(side)+u32::from(y),new.get_pixel(u32::from(x),u32::from(y)));
                    }}
                }
            }
            std::fs::write(output.join(format!("avant-{side}-{}.rgba",palette.index())),before.bytes).unwrap();
            std::fs::write(output.join(format!("apres-{side}-{}.rgba",palette.index())),after.bytes).unwrap();
        }
    }
    std::fs::write(output.join("verification.json"),serde_json::to_vec_pretty(&checks).unwrap()).unwrap();
    println!("1904 proposals: original alpha, protected skin/lights and held pixels verified; two outfit materials in all variants.");
}
