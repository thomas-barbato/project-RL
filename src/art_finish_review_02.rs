//! Explicit, isolated ART-FINISH-02 review. No candidates activate themselves.
use super::*;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

pub(crate) const OUTFITS: [(&str, Option<&str>); 8] = [
    ("core:technomancien", None),
    ("core:traqueur", None),
    ("core:arcaniste", None),
    ("core:metamorphe", None),
    ("core:bastion", Some("core:carapace_chrysalide")),
    ("core:bastion", Some("core:combinaison_fantome")),
    ("core:bastion", Some("core:manteau_de_mue")),
    ("core:bastion", Some("core:plastron_bastion")),
];
thread_local! {
    static MODE: Cell<u8> = const {Cell::new(0)};
    static SCENE: Cell<Option<(usize,Visual)>> = const {Cell::new(None)};
    static TILES: RefCell<BTreeMap<String,Texture2D>> = const {RefCell::new(BTreeMap::new())};
}
pub(crate) fn visual(index: usize, weapon: Option<&str>) -> Visual {
    let (class, torso) = OUTFITS[index];
    wear(
        player(
            Some(class),
            if weapon.is_some() {
                PlayerWeaponPose::Ranged
            } else {
                PlayerWeaponPose::Unarmed
            },
            weapon,
        ),
        torso.map(|id| ("core:body_armor".to_owned(), id.to_owned())),
    )
}
pub(crate) fn mode(value: u8, scene: Option<(usize, Visual)>) {
    MODE.set(value);
    SCENE.set(scene);
}
pub(crate) fn normalized_visual(value: Visual) -> Visual {
    if MODE.get() != 0
        && matches!(value, Visual::Player { .. })
        && let Some((_, visual)) = SCENE.get()
    {
        return visual;
    }
    value
}
fn draw_file(cell: Rect, file: &str, opacity: f32) {
    TILES.with(|tiles| {
        let mut tiles = tiles.borrow_mut();
        let texture = tiles.entry(file.to_owned()).or_insert_with(|| {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/source-art/art-finish-review-02/native")
                .join(file);
            super::texture(
                &std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
                FilterMode::Nearest,
            )
        });
        let rect = Rect::new(
            cell.x - cell.w / 2.,
            cell.y - cell.h / 2.,
            cell.w * 2.,
            cell.h * 2.,
        );
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        );
    });
}
pub(crate) fn draw_replacement(rect: Rect, value: Visual, opacity: f32) -> bool {
    if MODE.get() != 2 {
        return false;
    }
    let Visual::Player {
        body,
        equipment,
        weapon,
        hands,
        palettes,
    } = value
    else {
        return false;
    };
    if palettes[0] != EquipmentPalette::Original {
        return false;
    }
    let number = match equipment[0] {
        Some(103) => 18,
        Some(105) => 19,
        Some(106) => 20,
        Some(107) => 21,
        None | Some(201..=204) if (4..=7).contains(&body) => 14 + body - 4,
        _ => return false,
    };
    let held = match hands {
        Some([None, right]) => right,
        Some(_) => return false,
        None => weapon,
    };
    let pose = match held {
        None => None,
        Some(101) => Some("single"),
        Some(2) => Some("rifle"),
        _ => return false,
    };
    super::draw_actor_shadow(rect, body, opacity);
    if let Some(pose) = pose {
        for part in ["body", "weapon", "fingers"] {
            draw_file(rect, &format!("FIN-{number}-{pose}-{part}.png"), opacity);
        }
    } else {
        draw_file(rect, &format!("FIN-{number}.png"), opacity);
    }
    true
}
/// Presentation figures are drawn only, never inserted into simulation or saves.
pub(crate) fn scene_actors(
    camera: &crate::terminal_view::GridCamera,
    game: &project_rl::game::WorldState,
) {
    let Some((selected, _)) = SCENE.get() else {
        return;
    };
    if MODE.get() == 0 {
        return;
    }
    let Some(player) = game.player_position() else {
        return;
    };
    if player != GridPos::new(15, 41) {
        return;
    }
    let start = if selected < 4 { 0 } else { 4 };
    let others: Vec<_> = (start..start + 4)
        .filter(|index| *index != selected)
        .collect();
    for (order, ((dx, dy), index)) in [(-1, 1), (3, 0), (4, -2)]
        .into_iter()
        .zip(others)
        .enumerate()
    {
        let at = GridPos::new(player.x + dx, player.y + dy);
        if game.map().is_walkable(at) && game.player_visibility().is_visible(at) {
            let figure = visual(
                index,
                Some(if order == 1 {
                    "core:fusil_d_assaut"
                } else {
                    "core:modele_cogneur_p01"
                }),
            );
            // These figures must retain their own identity, unlike the player fixture.
            let scene = SCENE.replace(None);
            super::draw_world(camera.rect(at), figure, 1.);
            SCENE.set(scene);
        }
    }
}
