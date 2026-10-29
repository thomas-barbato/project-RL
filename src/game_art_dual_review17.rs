//! DUAL17-C02 review only: carry both weapons on complete, coherent arms.
use super::*;

pub(super) fn front(key: Key) -> Option<Vec<Image>> {
    let (index, head) = decode(key.0)?;
    let rig = &ARMOR_RIGS[index];
    let mut body = part(index, "base");
    let mut weapons = Image::gen_image_color(128, 128, BLANK);
    let mut fingers = weapons.clone();
    for layer in key.1.into_iter().flatten() {
        let side = usize::from(!layer.left);
        let palm = rig.palms[side];
        // Keep the complete accepted lowered arm. Replace only the old open
        // hand by the class's closed grip, without turning a cuff into a fist.
        for y in (palm[1] - 2.) as usize..=(palm[1] + 4.) as usize {
            for x in (palm[0] - 3.) as usize..=(palm[0] + 3.) as usize {
                body.bytes[(y * 128 + x) * 4..(y * 128 + x + 1) * 4].fill(0);
            }
        }
        geometry::over(
            &mut body,
            &part(index, if side == 0 { "grip-0" } else { "grip-1" }),
        );
        geometry::over(
            &mut fingers,
            &part(index, if side == 0 { "fingers-0" } else { "fingers-1" }),
        );
        let raised = matches!(weapon_hold(layer.model), WeaponHold::Raised);
        let angle = if raised {
            if layer.left {
                -std::f32::consts::FRAC_PI_2
            } else {
                std::f32::consts::FRAC_PI_2
            }
        } else {
            0.
        };
        let (anchor, _) = attachment(layer.model, layer.left)?;
        let weapon = weapon_image(layer.model, layer.left, key.2[1 + usize::from(layer.left)])?;
        geometry::over(
            &mut weapons,
            &geometry::transformed(&weapon, anchor, palm, 1., angle),
        );
    }
    helmet(&mut body, index, head);
    Some(vec![body, weapons, fingers])
}
