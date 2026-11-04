//! HELM06: equipped helmets keep their item identity, without changing outfits.
use super::*;

fn visual(body: u8, armor: Option<u16>, helmet: Option<u16>) -> Visual {
    Visual::Player {
        body,
        equipment: [armor, helmet, None, None],
        weapon: Some(31),
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    }
}

#[test]
fn armor_only_renderer_input_keeps_every_other_equipment_slot() {
    let equipment = [Some(101), Some(120), Some(121), Some(122)];
    assert_eq!(
        player_visual_equipment(equipment),
        [Some(101), None, Some(121), Some(122)]
    );
    assert_eq!(equipment[1], Some(120));
    let expected = visual(3, Some(101), None);
    assert_eq!(
        wear(
            expected,
            [("core:head_armor".into(), "core:heaume_bastion".into())]
        ),
        expected
    );
    assert!(
        item("core:heaume_bastion").is_some(),
        "The item/icon must remain available"
    );
}

#[test]
fn every_helmet_uses_the_armor_outfit_and_its_existing_cache_key() {
    for body in 3..=7 {
        for armor in [None, Some(110), Some(102), Some(101), Some(103)] {
            let original = player_layers::outfit_index(body, [armor, None, None, None]);
            for head in 115..=120 {
                assert_eq!(
                    player_layers::outfit_index(body, [armor, Some(head), None, None]),
                    original,
                    "A helmet changed the outfit: body{body} armor{armor:?} head{head}"
                );
            }
        }
    }
}

#[test]
fn armor_only_front_portrait_and_all_walk_frames_match_the_original_outfit() {
    for body in [3, 5, 6, 7] {
        for (tier, armor) in [None, Some(110), Some(102), Some(101), Some(103)]
            .into_iter()
            .enumerate()
        {
            let bare = visual(body, armor, None);
            let equipped = visual(body, armor, Some(115 + (tier as u16 + u16::from(body)) % 6));
            let a = vertical::review_image(bare).unwrap();
            let b = vertical::review_image(equipped).unwrap();
            assert_eq!(a.cell, b.cell);
            assert_eq!(a.anchor, b.anchor);
            assert_eq!(
                a.image.bytes, b.image.bytes,
                "Vertical/real-rest changed: body{body} tier{tier}"
            );
            assert_eq!(
                vertical_review_standing(bare).unwrap().bytes,
                vertical_review_standing(equipped).unwrap().bytes,
                "Front/portrait changed: body{body} tier{tier}"
            );
            for facing in [-1, 1] {
                for frame in 0..6 {
                    let pose = walking::Pose { facing, frame };
                    assert_eq!(
                        polish_review_lateral(bare, pose).unwrap().bytes,
                        polish_review_lateral(equipped, pose).unwrap().bytes,
                        "Lateral changed: body{body} tier{tier} facing{facing} frame{frame}"
                    );
                }
            }
        }
    }
}

#[test]
fn armor_only_policy_also_applies_to_fixed_dual_weapon_loadouts() {
    for body in [3, 5, 6, 7] {
        let mut a = visual(body, Some(102), None);
        if let Visual::Player { weapon, hands, .. } = &mut a {
            *weapon = None;
            *hands = Some([Some(70), Some(104)]);
        }
        let mut b = a;
        if let Visual::Player { equipment, .. } = &mut b {
            equipment[1] = Some(118);
        }
        assert_eq!(
            vertical::review_image(a).unwrap().image.bytes,
            vertical::review_image(b).unwrap().image.bytes
        );
    }
}
