//! ART-FINISH-01, explicit debug review only. Never enables itself in a run.
use super::*;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

thread_local! {
    static MODE: Cell<u8> = const { Cell::new(0) };
    static SCENE: Cell<bool> = const { Cell::new(false) };
    static TILES: RefCell<BTreeMap<String, Texture2D>> = const { RefCell::new(BTreeMap::new()) };
}
pub(crate) fn mode(mode: u8, scene: bool) {
    MODE.set(mode);
    SCENE.set(scene);
}
pub(crate) fn normalized_visual(visual: Visual) -> Visual {
    if MODE.get() != 0 && SCENE.get() && matches!(visual, Visual::Player { .. }) {
        return super::player(
            Some("core:bastion"),
            PlayerWeaponPose::Ranged,
            Some("core:modele_cogneur_p01"),
        );
    }
    visual
}
fn tile(file: &str, draw: impl FnOnce(&Texture2D)) {
    TILES.with(|tiles| {
        let mut tiles = tiles.borrow_mut();
        let texture = tiles.entry(file.to_owned()).or_insert_with(|| {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/source-art/art-finish-review-01/native")
                .join(file);
            let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            super::texture(&bytes, FilterMode::Nearest)
        });
        draw(texture);
    });
}
fn draw_file(cell: Rect, file: &str, icon: bool, opacity: f32) {
    let rect = if icon {
        cell
    } else {
        Rect::new(
            cell.x - cell.w / 2.,
            cell.y - cell.h / 2.,
            cell.w * 2.,
            cell.h * 2.,
        )
    };
    tile(file, |texture| {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        )
    });
}
pub(crate) fn draw_replacement(rect: Rect, visual: Visual, opacity: f32) -> bool {
    if MODE.get() != 2 {
        return false;
    }
    let (code, pose, icon) = match visual {
        Visual::Player {
            body,
            weapon,
            equipment,
            ..
        } => {
            let code = match equipment[0] {
                Some(101) => "FIN-02",
                Some(102) => "FIN-03",
                Some(104) => "FIN-04",
                Some(113) => "FIN-05",
                None | Some(200) if body == 3 => "FIN-01",
                _ => return false,
            };
            let pose = if matches!(code, "FIN-01" | "FIN-02") {
                match weapon {
                    Some(101) => Some("single"),
                    Some(2) => Some("rifle"),
                    _ => None,
                }
            } else {
                None
            };
            super::draw_actor_shadow(rect, body, opacity);
            (code, pose, false)
        }
        Visual::CampaignMonster { number, variant } => {
            let code = match (number, variant) {
                (164, 0) => "FIN-06",
                (165, 0) => "FIN-07",
                (5, 0) => "FIN-08",
                (144, 0) => "FIN-09",
                (144, 1) => "FIN-09-blue",
                (144, 2) => "FIN-09-orange",
                (144, 3) => "FIN-09-green",
                (145, 0) => "FIN-10",
                (145, 2) => "FIN-10-orange",
                (145, 3) => "FIN-10-green",
                (145, 4) => "FIN-10-blue",
                _ => return false,
            };
            if let Some(entry) = project_rl::monster_art::art(number, variant) {
                let r = Rect::new(
                    rect.x - rect.w / 2.,
                    rect.y - rect.h / 2.,
                    rect.w * 2.,
                    rect.h * 2.,
                );
                super::draw_contact_shadow(
                    r,
                    [
                        entry.foot[0] as u8,
                        (entry.foot[1] - 1) as u8,
                        entry.shadow as u8,
                    ],
                    entry.side,
                    opacity,
                );
            }
            (code, None, false)
        }
        Visual::Bestiary {
            number: 57,
            corpse: false,
        } => {
            super::draw_contact_shadow(rect, [16, 31, 23], 32, opacity);
            ("FIN-11", None, false)
        }
        Visual::ColoredItem(
            ItemArt {
                illustration: 2, ..
            },
            _,
        ) => ("FIN-12", None, true),
        Visual::ColoredItem(
            ItemArt {
                illustration: 150, ..
            },
            _,
        ) => ("FIN-13", None, true),
        _ => return false,
    };
    if let Some(pose) = pose {
        for layer in ["body", "weapon", "fingers"] {
            draw_file(rect, &format!("{code}-{pose}-{layer}.png"), false, opacity);
        }
    } else {
        draw_file(rect, &format!("{code}.png"), icon, opacity);
    }
    true
}

/// Display-only actors are not inserted into the world or its perception.
pub(crate) fn scene_actors(
    camera: &crate::terminal_view::GridCamera,
    game: &project_rl::game::WorldState,
) {
    if MODE.get() == 0 || !SCENE.get() {
        return;
    }
    let Some(player) = game.player_position() else {
        return;
    };
    if player != GridPos::new(15, 41) {
        return;
    }
    let samples = [
        (
            GridPos::new(player.x + 1, player.y + 1),
            Visual::CampaignMonster {
                number: 164,
                variant: 0,
            },
        ),
        (
            GridPos::new(player.x + 3, player.y),
            Visual::CampaignMonster {
                number: 5,
                variant: 0,
            },
        ),
        (
            GridPos::new(player.x + 4, player.y - 2),
            Visual::CampaignMonster {
                number: 144,
                variant: 0,
            },
        ),
        (
            GridPos::new(player.x - 1, player.y + 1),
            Visual::Bestiary {
                number: 57,
                corpse: false,
            },
        ),
    ];
    for (at, visual) in samples {
        if game.map().is_walkable(at) && game.player_visibility().is_visible(at) {
            super::draw_world(camera.rect(at), visual, 1.);
        }
    }
}
