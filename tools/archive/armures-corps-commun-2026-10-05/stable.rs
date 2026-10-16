//! Renderer proof: shared head, trunk and legs; authored foreground arms/weapons.
//! The source PNGs remain untouched. Only the preview rendering path changes.
use super::*;

/// Fixed parts of the outfit. Native 48px coordinates; the same mask is used
/// in the native view and inspection view. Transparent pixels are fixed too,
/// so changing pose cannot retain a different hood/boot silhouette here.
pub(super) fn body_zone(x:f32,y:f32)->bool {
    ((19.0..30.0).contains(&x) && (16.0..26.0).contains(&y))
        || ((20.0..29.0).contains(&x) && (26.0..36.0).contains(&y))
        || y>=36.0
}

/// Foreground occlusion by arms/hands/weapons across the chest. These are the
/// only parts that may cover the shared trunk; helmets/legs never use a new pose.
pub(super) fn foreground(pose:usize,x:f32,y:f32)->bool {
    match pose {
        2 => (30.0..34.5).contains(&y),
        4 => (19.0..29.0).contains(&x) && (30.0..34.0).contains(&y),
        _ => false,
    }
}

pub fn figure(sources:&[Image],outfit:u16,pose:usize,colors:[EquipmentPalette;3],flip:bool)->Image {
    let fixed=proposal::figure(&sources[0],outfit,0,[colors[0],EquipmentPalette::Original,EquipmentPalette::Original]);
    if pose==0 {return fixed;}
    let mut held_colors=colors;
    if flip {held_colors.swap(1,2);}
    let authored=proposal::figure(&sources[pose],outfit,pose,held_colors);
    let side=fixed.width;
    let mut result=Image::gen_image_color(side,side,BLANK);
    for y in 0..side {for x in 0..side {
        let n=(usize::from(y)*usize::from(side)+usize::from(x))*4;
        let (px,py)=(f32::from(x)*48.0/f32::from(side),f32::from(y)*48.0/f32::from(side));
        let source_x=if flip {side-1-x}else{x};
        let a=(usize::from(y)*usize::from(side)+usize::from(source_x))*4;
        let fixed_part=body_zone(px,py) && !foreground(pose,px,py);
        let bytes=if fixed_part {&fixed.bytes[n..n+4]}else{&authored.bytes[a..a+4]};
        result.bytes[n..n+4].copy_from_slice(bytes);
    }}
    result
}
