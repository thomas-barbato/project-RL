from pathlib import Path
root=Path(__file__).resolve().parents[2]
p=root/'src/game_art_worn.rs';s=p.read_text(encoding='utf-8')
s=s.replace('[WeaponHold::Side, WeaponHold::TwoHanded, WeaponHold::Raised]','[WeaponHold::Side, WeaponHold::TwoHanded, WeaponHold::Raised, WeaponHold::MeleeTwoHanded]')
s=s.replace('static SPRITES: [PoseTextures; 4]','static SPRITES: [PoseTextures; 8]')
s=s.replace('-32.png','-48.png')
needle='''    ];
}

pub(super) fn prewarm()'''
entries=''
for pose in ['melee-two-handed','dual-melee','dual-raised','dual-mixed']:
 entries+=f'''        pose_textures(
            include_bytes!("../assets/sprites/cyberpunk/player-poses-{pose}-48.png"),
            include_bytes!("../assets/sprites/cyberpunk/player-poses-{pose}-128.png")),
'''
assert needle in s;s=s.replace(needle,entries+needle,1)
s=s.replace('    weapon: Option<u16>,\n    side: u16,','    weapon: Option<u16>,\n    hands: Option<[Option<WeaponHold>; 2]>,\n    side: u16,',1)
s=s.replace('    draw_pose(rect, body, equipment, pose_index(weapon), side, opacity);','    let (pose, flip) = pose_selection(weapon, hands);\n    draw_pose(rect, body, equipment, pose, side, opacity, flip);')
s=s.replace('        1.0,\n    );','        1.0,\n        false,\n    );',1)
start=s.index('fn draw_pose(');end=s.index('\n#[cfg(test)]',start)
s=s[:start]+'''fn pose_selection(weapon: Option<u16>, hands: Option<[Option<WeaponHold>; 2]>) -> (usize, bool) {
    use WeaponHold::{Side, Raised};
    match hands {
        None => (pose_index(weapon), false),
        Some([None, None]) => (0, false),
        Some([Some(Side), Some(Side)]) => (5, false),
        Some([Some(Raised), Some(Raised)]) => (6, false),
        Some([Some(Raised), Some(Side)]) => (7, false),
        Some([Some(Side), Some(Raised)]) => (7, true),
        Some([Some(hold), None]) => (armed_pose_index(hold), true),
        Some([None, Some(hold)]) => (armed_pose_index(hold), false),
        _ => (pose_index(weapon), false),
    }
}

fn draw_pose(rect: Rect, body: u8, equipment: [Option<u16>; 4], pose: usize, side: u16, opacity: f32, flip: bool) {
    SPRITES.with(|art| {
        let (sprite, cell, destination) = if side == 128 {
            (&art[pose].portrait, 128.0, rect)
        } else {
            (&art[pose].world, 48.0, Rect::new(rect.x-rect.w*0.25, rect.y-rect.h*0.5, rect.w*1.5, rect.h*1.5))
        };
        let index = outfit_index(body, equipment);
        draw_texture_ex(sprite, destination.x, destination.y, Color::new(1.0,1.0,1.0,opacity), DrawTextureParams {
            dest_size: Some(vec2(destination.w, destination.h)),
            source: Some(Rect::new(f32::from(index%16)*cell, f32::from(index/16)*cell, cell, cell)),
            flip_x: flip,
            ..Default::default()
        });
    });
}
''' + s[end:]
s=s.replace('WeaponHold::Raised => 3,','WeaponHold::Raised => 3,\n                WeaponHold::MeleeTwoHanded => 4,')
s=s.replace('assert_eq!(pose_index(held_weapon("core:melee_epee_longue")), 1);','assert_eq!(pose_index(held_weapon("core:melee_epee_longue")), 4);')
s=s.replace('                32,\n                [','                48,\n                [')
for side in [48,128]:
 needle=f'''                    include_bytes!("../assets/sprites/cyberpunk/player-poses-raised-{side}.png")
                        .as_slice(),'''
 added=needle+'\n'+''.join(f'''                    include_bytes!("../assets/sprites/cyberpunk/player-poses-{pose}-{side}.png").as_slice(),
''' for pose in ['melee-two-handed','dual-melee','dual-raised','dual-mixed'])
 assert needle in s;s=s.replace(needle,added)
s=s.replace('occupied > (side * side / 6) as usize','occupied > (side * side / 12) as usize')
s=s.replace('oy + 123','oy + 124')
needle='''            include_bytes!("../assets/sprites/cyberpunk/player-poses-raised-128.png").as_slice(),'''
s=s.replace(needle,needle+'\n'+''.join(f'''            include_bytes!("../assets/sprites/cyberpunk/player-poses-{pose}-128.png").as_slice(),
''' for pose in ['melee-two-handed','dual-melee','dual-raised','dual-mixed']))
pos=s.rfind('\n}')
s=s[:pos]+'''
    #[test]
    fn player_hand_poses_cover_both_mixed_orders_and_empty_hands() {
        use WeaponHold::{Side, Raised};
        assert_eq!(pose_selection(None, Some([Some(Side), Some(Side)])), (5, false));
        assert_eq!(pose_selection(None, Some([Some(Raised), Some(Raised)])), (6, false));
        assert_eq!(pose_selection(None, Some([Some(Raised), Some(Side)])), (7, false));
        assert_eq!(pose_selection(None, Some([Some(Side), Some(Raised)])), (7, true));
        for (hold, pose) in [(Side, 1), (Raised, 3)] {
            assert_eq!(pose_selection(None, Some([Some(hold), None])), (pose, true));
            assert_eq!(pose_selection(None, Some([None, Some(hold)])), (pose, false));
        }
        assert_eq!(pose_selection(None, Some([None, None])), (0, false));
    }
''' +s[pos:]
p.write_text(s,encoding='utf-8',newline='\n')
