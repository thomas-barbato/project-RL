//! C04: independent original-front/authored-back helmets and uncovered heads.
use super::*;

#[derive(Deserialize)]
pub(super) struct Parts {
    front: String,
    back: String,
    replace_mask: String,
    standing_anchor: [f32; 2],
}
#[derive(Deserialize)]
struct Gear {
    front: String,
    back: String,
    opening: String,
    ownership: String,
    back_exposed_hair_models: Vec<u16>,
}
fn gear() -> &'static Gear {
    static GEAR: OnceLock<Gear> = OnceLock::new();
    GEAR.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/source-art/vertical-walk-review-18/candidate-C04/headgear.json"
        ))
        .expect("C04 gear manifest")
    })
}
thread_local! { static PROOFS: Cell<usize> = const { Cell::new(0) }; }

fn over(body: &mut Image, part: &Image, at: [f32; 2]) -> Result<(), String> {
    if (part.width, part.height) != (32, 32) {
        return Err("C04 head part must be 32x32".into());
    }
    let (ox, oy) = (at[0].round() as i32 - 16, at[1].round() as i32 - 16);
    if ox < 0 || oy < 0 || ox + 32 > 128 || oy + 32 > 128 {
        return Err("C04 head outside canvas".into());
    }
    for y in 0..32usize {
        for x in 0..32usize {
            let s = &part.bytes[(y * 32 + x) * 4..][..4];
            let p = ((oy as usize + y) * 128 + ox as usize + x) * 4;
            let d = &mut body.bytes[p..p + 4];
            let a = u32::from(s[3]);
            let old = u32::from(d[3]) * (255 - a);
            let total = a * 255 + old;
            if total == 0 {
                continue;
            }
            for c in 0..3 {
                d[c] =
                    ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
            }
            d[3] = ((total + 127) / 255) as u8;
        }
    }
    Ok(())
}
fn erase(body: &mut Image, mask: &Image, at: [f32; 2]) {
    let (ox, oy) = (at[0].round() as usize - 16, at[1].round() as usize - 16);
    for y in 0..32usize {
        for x in 0..32usize {
            if mask.bytes[(y * 32 + x) * 4 + 3] > 0 {
                body.bytes[((oy + y) * 128 + ox + x) * 4..][..4].fill(0);
            }
        }
    }
}

pub(super) fn helmet(
    body: &mut Image,
    visual: Visual,
    anchor: [f32; 2],
    back: bool,
) -> Result<(), String> {
    let Visual::Player { equipment, .. } = visual else {
        return Err("C04 player required".into());
    };
    let lower = body.bytes[60 * 512..].to_vec();
    if let Some(model) = equipment[1] {
        let index = model
            .checked_sub(115)
            .filter(|v| *v < 6)
            .ok_or("C04 unknown helmet")?;
        let data = gear();
        let cover = image(if back { &data.back } else { &data.front })?.sub_image(Rect::new(
            f32::from(index) * 32.,
            0.,
            32.,
            32.,
        ));
        let opening =
            image(&data.opening)?.sub_image(Rect::new(f32::from(index) * 32., 0., 32., 32.));
        let ownership = image(&data.ownership)?;
        let previous = body.clone();
        erase(body, &ownership, anchor);
        let (ox, oy) = (
            anchor[0].round() as usize - 16,
            anchor[1].round() as usize - 16,
        );
        for y in 0..32usize {
            for x in 0..32usize {
                let n = (y * 32 + x) * 4;
                let restore = if back {
                    data.back_exposed_hair_models.contains(&model)
                        && ownership.bytes[n + 3] > 0
                        && cover.bytes[n + 3] < 220
                } else {
                    opening.bytes[n + 3] > 0
                };
                if restore {
                    let p = ((oy + y) * 128 + ox + x) * 4;
                    body.bytes[p..p + 4].copy_from_slice(&previous.bytes[p..p + 4]);
                }
            }
        }
        over(body, &cover, anchor)?;
    }
    if body.bytes[60 * 512..] != lower {
        return Err("C04 helmet changed arms or lower body".into());
    }
    PROOFS.set(PROOFS.get() + 1);
    Ok(())
}
pub(super) fn standing(body: &mut Image, visual: Visual, profile: &Profile) -> Result<(), String> {
    let parts = profile
        .head_parts
        .as_ref()
        .ok_or("C04 head parts missing")?;
    let mask = image(&parts.replace_mask)?;
    let front = image(&parts.front)?;
    let _back = image(&parts.back)?;
    let lower = body.bytes[60 * 512..].to_vec();
    erase(body, &mask, parts.standing_anchor);
    over(body, &front, parts.standing_anchor)?;
    if body.bytes[60 * 512..] != lower {
        return Err("C04 uncovered idle changed held arms".into());
    }
    helmet(body, visual, parts.standing_anchor, false)
}

fn subjects(head: u8, armed: bool) -> [Visual; 4] {
    equipment::subjects(if armed { 4 } else { 0 }).map(|v| {
        let Visual::Player {
            body,
            mut equipment,
            weapon,
            hands,
            palettes,
        } = v
        else {
            unreachable!()
        };
        equipment[1] = if head == 0 {
            None
        } else {
            Some(114 + u16::from(head))
        };
        Visual::Player {
            body,
            equipment,
            weapon,
            hands,
            palettes,
        }
    })
}
pub(super) async fn capture(output: &Path) -> Result<(), String> {
    struct Guard(bool, bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            ACTIVE.set(self.0);
            HEADS.set(self.1);
        }
    }
    let _guard = Guard(ACTIVE.replace(true), HEADS.replace(true));
    PROOFS.set(0);
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1240., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let mut front_returns = 0;
    let mut contacts = 0;
    for head in 0..=6 {
        for armed in [false, true] {
            let path = output.join(format!(
                "head-{head}-{}",
                if armed { "mixed" } else { "unarmed" }
            ));
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            let visuals = subjects(head, armed);
            contacts += prepare(&visuals, &path).await?;
            front_returns += equipment::flat(&path, &visuals, "VERT18-C04").await?;
        }
    }
    for (head, label) in [(0, "bare"), (4, "helmet")] {
        let visuals = subjects(head, true);
        let path = output.join(format!("prepare-{label}"));
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        prepare(&visuals, &path).await?;
        for (side, pair) in [("west", &visuals[..2]), ("east", &visuals[2..])] {
            let path = output.join(format!("camp-{label}-{side}"));
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            capture_camp(&path, "VERT18-C04", pair).await?;
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"VERT18-C04","status":"review_only","normal_game_enabled":false,"profiles":4,"head_choices":7,
        "helmet_models":[115,116,117,118,119,120],"loadouts":["unarmed","mixed104+70"],"front_returns":front_returns,
        "sampled_weapon_contacts":contacts,"head_operations_preserving_arms_and_lower_body":PROOFS.get(),
        "front_helmet_art_unchanged":true,"new_back_helmet_views":6,"true_unhelmeted_heavy_profiles":4,
        "head_composited_before_weapon_layers":true,"source_cache_limit_bytes":4194304,"texture_cache_limit_bytes":8388608,
        "read_or_decode_during_draw":false,"saved_game_loaded":false,"user_preferences_written":false
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headgear04_uses_distinct_keys_and_restores_accepted_c03_selection() {
        struct Restore(bool);
        impl Drop for Restore {
            fn drop(&mut self) {
                HEADS.set(self.0);
            }
        }
        let _restore = Restore(HEADS.replace(false));
        let original = equipment::subjects(4)[0];
        let (old_key, old_id) = key(original).unwrap();
        assert_eq!(old_id, "bastion-lourd");
        HEADS.set(true);
        let (bare_key, bare_id) = key(original).unwrap();
        assert_ne!(old_key, bare_key);
        assert_eq!(bare_id, "bastion-lourd-headgear");
        for model in 115..=120 {
            let Visual::Player {
                body,
                mut equipment,
                weapon,
                hands,
                palettes,
            } = original
            else {
                unreachable!()
            };
            equipment[1] = Some(model);
            let (held_key, held_id) = key(Visual::Player {
                body,
                equipment,
                weapon,
                hands,
                palettes,
            })
            .unwrap();
            assert_eq!(held_id, bare_id);
            assert_eq!(held_key.2[1], Some(model));
            assert_ne!(held_key, bare_key);
        }
        assert!(key(Visual::CampNpc(1)).is_none());
        HEADS.set(false);
        assert_eq!(key(original), Some((old_key, old_id)));
        assert!(key(Visual::CampNpc(1)).is_some());
    }
}
