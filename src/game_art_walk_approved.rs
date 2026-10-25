//! Whole authored player poses with the actual modular equipment attached.
use super::*;
use project_rl::asset_cache::ByteCache;
use std::cell::RefCell;

type Key = (
    u8,
    [Option<u16>; 4],
    Option<u16>,
    Option<[Option<u16>; 2]>,
    [EquipmentPalette; 3],
    Pose,
    bool,
    bool,
    bool,
);
thread_local! {
    static CACHE: RefCell<ByteCache<Key, Option<Texture2D>>> =
        const { RefCell::new(ByteCache::new(32 * 1024 * 1024)) };
}

pub(super) fn image(visual: Visual, pose: Pose) -> Option<Image> {
    let Visual::Player {
        body,
        equipment,
        weapon,
        hands,
        palettes,
    } = visual
    else {
        return None;
    };
    player_layers::approved_walking_image(body, equipment, weapon, hands, palettes, pose)
}

pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    if !player_enabled() {
        return super::rifle::draw(cell, visual, opacity, pose);
    }
    let Some(pose) = pose else {
        return false;
    };
    let Visual::Player {
        body,
        equipment,
        weapon,
        hands,
        palettes,
    } = visual
    else {
        return false;
    };
    if !(3..=7).contains(&body)
        || ![-1, 1].contains(&pose.facing)
        || ![0, 1, 2, 3, 4, 5, 8].contains(&pose.frame)
    {
        return false;
    }
    let key = (
        body,
        equipment,
        weapon,
        hands,
        palettes,
        pose,
        common_cycle_enabled(),
        proportions_enabled(),
        gait12_enabled(),
    );
    CACHE.with_borrow_mut(|cache| {
        if cache.get(&key).is_none() {
            let Some(image) = image(visual, pose) else {
                return false;
            };
            let bytes = image.bytes.len();
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            cache.insert(key, Some(texture), bytes);
        }
        let Some(texture) = cache.get(&key).and_then(Option::as_ref) else {
            return false;
        };
        draw_contact_shadow(cell, [32, 63, 17], 64, opacity);
        crate::world_readability::texture(
            texture,
            cell.x - cell.w / 2.,
            cell.y - cell.h / 2.,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(vec2(cell.w * 2., cell.h * 2.)),
                ..Default::default()
            },
        );
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_proportions_match_review_and_legacy_comparison_is_scoped() {
        assert!(proportions_enabled());
        let pose = Pose {
            facing: 1,
            frame: 8,
        };
        let original = image(player(3, Some(67)), pose).unwrap();
        {
            let _comparison = review_proportions(false);
            assert!(!proportions_enabled());
            assert_ne!(
                image(player(3, Some(67)), pose).unwrap().bytes,
                original.bytes
            );
        }
        assert!(proportions_enabled());
        {
            let _review = review_corrections();
            let _common = review_common(true);
            let _proportions = review_proportions(true);
            let torsos = [None, Some(110), Some(102), Some(101), Some(103)];
            for body in 3..=7 {
                for torso in torsos {
                    let mut visual = player(body, Some(67));
                    if let Visual::Player { equipment, .. } = &mut visual {
                        equipment[0] = torso;
                    }
                    for facing in [-1, 1] {
                        let frames: Vec<_> = [8, 0, 1, 2, 3, 4, 5]
                            .into_iter()
                            .map(|frame| image(visual, Pose { facing, frame }).unwrap().bytes)
                            .collect();
                        assert_eq!(
                            frames
                                .iter()
                                .collect::<std::collections::BTreeSet<_>>()
                                .len(),
                            7
                        );
                    }
                }
            }
            assert_eq!(
                image(player(3, Some(67)), pose).unwrap().bytes,
                original.bytes
            );
        }
        assert!(proportions_enabled());
        assert_eq!(
            image(player(3, Some(67)), pose).unwrap().bytes,
            original.bytes
        );
    }
    #[test]
    fn common_cycle_uses_actual_equipment_for_every_class() {
        assert!(!corrections_enabled());
        assert!(player_enabled());
        assert!(common_cycle_enabled());
        assert!(!REVIEW_ACCEPTED);
        for body in 3..=7 {
            let pose = Pose {
                facing: 1,
                frame: 1,
            };
            let rifle = image(player(body, Some(2)), pose).unwrap();
            for model in 0..HELD_WEAPON_COUNT {
                let actual = image(player(body, Some(model)), pose).unwrap();
                assert!(actual.bytes.chunks_exact(4).any(|p| p[3] > 0));
                if model != 2 {
                    assert_ne!(actual.bytes, rifle.bytes, "body{body} weapon{model}");
                }
            }
            for facing in [-1, 1] {
                for frame in [8, 0, 1, 2, 3, 4, 5] {
                    assert!(image(player(body, Some(67)), Pose { facing, frame }).is_some());
                }
            }
        }
    }
    fn player(body: u8, weapon: Option<u16>) -> Visual {
        Visual::Player {
            body,
            equipment: [None; 4],
            weapon,
            hands: None,
            palettes: [EquipmentPalette::Original; 3],
        }
    }
    #[test]
    fn every_starting_weapon_has_six_real_poses_and_a_distinct_idle_in_both_directions() {
        let _review = review_corrections();
        for (body, weapon) in [(3, 67), (4, 101), (5, 38), (6, 70), (7, 70)] {
            for facing in [-1, 1] {
                let poses: Vec<_> = [8, 0, 1, 2, 3, 4, 5]
                    .into_iter()
                    .map(|frame| {
                        image(player(body, Some(weapon)), Pose { facing, frame })
                            .unwrap()
                            .bytes
                    })
                    .collect();
                assert_eq!(
                    poses
                        .iter()
                        .collect::<std::collections::BTreeSet<_>>()
                        .len(),
                    7
                );
                assert!(poses.iter().all(|p| p.chunks_exact(4).any(|c| c[3] > 0)));
            }
        }
    }
    #[test]
    fn actual_weapons_helmets_and_both_hands_do_not_fall_back_to_a_rifle() {
        let _review = review_corrections();
        let pose = Pose {
            facing: -1,
            frame: 2,
        };
        for body in 3..=7 {
            let rifle = image(player(body, Some(2)), pose).unwrap();
            for model in 0..HELD_WEAPON_COUNT {
                let actual = image(player(body, Some(model)), pose).unwrap();
                if model != 2 {
                    assert_ne!(actual.bytes, rifle.bytes, "body{body} weapon{model}");
                }
            }
            for head in 115..=120 {
                let mut visual = player(body, Some(67));
                if let Visual::Player {
                    equipment,
                    palettes,
                    hands,
                    ..
                } = &mut visual
                {
                    equipment[1] = Some(head);
                    palettes[1] = EquipmentPalette::Blue;
                    *hands = Some([Some(101), Some(68)]);
                }
                assert!(image(visual, pose).is_some());
            }
            let empty = image(player(body, None), pose).unwrap();
            assert_ne!(empty.bytes, rifle.bytes);
            assert!(
                image(
                    player(body, Some(67)),
                    Pose {
                        facing: 0,
                        frame: 2
                    }
                )
                .is_none()
            );
        }
    }
}
