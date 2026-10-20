//! Explicit ART-FINISH-05 preview; ordinary play never reads authoring files.
use super::*;
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
};

pub(crate) const NUMBERS: [u16; 4] = [14, 16, 6, 120];
thread_local! {
    static MODE: Cell<u8> = const {Cell::new(0)};
    static SCENE: Cell<bool> = const {Cell::new(false)};
    static TILES: RefCell<BTreeMap<u16,Texture2D>> = const {RefCell::new(BTreeMap::new())};
}
pub(crate) fn mode(value: u8, scene: bool) {
    MODE.set(value);
    SCENE.set(scene);
}
pub(crate) fn normalized_visual(value: Visual) -> Visual {
    if MODE.get() != 0 && SCENE.get() && matches!(value, Visual::Player { .. }) {
        return player(
            Some("core:bastion"),
            PlayerWeaponPose::Ranged,
            Some("core:modele_cogneur_p01"),
        );
    }
    value
}
pub(crate) fn draw_replacement(cell: Rect, value: Visual, opacity: f32) -> bool {
    if MODE.get() != 2 {
        return false;
    }
    let Visual::CampaignMonster { number, variant: 0 } = value else {
        return false;
    };
    let Some(index) = NUMBERS.iter().position(|&n| n == number) else {
        return false;
    };
    let entry = project_rl::monster_art::art(number, 0).unwrap();
    let scale = cell.w / 64.;
    let rect = Rect::new(
        cell.center().x - f32::from(entry.foot[0]) * scale,
        cell.bottom() - f32::from(entry.foot[1]) * scale,
        f32::from(entry.side) * scale,
        f32::from(entry.side) * scale,
    );
    super::draw_contact_shadow(
        rect,
        [
            entry.foot[0] as u8,
            (entry.foot[1] - 1) as u8,
            entry.shadow as u8,
        ],
        entry.side,
        opacity,
    );
    TILES.with(|tiles| {
        let mut tiles = tiles.borrow_mut();
        let texture = tiles.entry(number).or_insert_with(|| {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("assets/source-art/art-finish-review-05/native")
                .join(format!("FIN-{}.png", 28 + index));
            super::texture(&std::fs::read(&path).unwrap(), FilterMode::Nearest)
        });
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
    true
}

pub(crate) fn positions(game: &project_rl::game::WorldState) -> Vec<GridPos> {
    let Some(player) = game.player_position() else {
        return vec![];
    };
    let mut positions: Vec<GridPos> = Vec::new();
    for (dx, dy) in [
        (-1, 1),
        (3, 0),
        (4, -2),
        (-2, -2),
        (1, -3),
        (-3, 0),
        (3, 2),
        (-3, 2),
    ] {
        let at = GridPos::new(player.x + dx, player.y + dy);
        if game.map().is_walkable(at)
            && game.player_visibility().is_visible(at)
            && positions
                .iter()
                .all(|other| (at.x - other.x).abs() + (at.y - other.y).abs() >= 3)
        {
            positions.push(at);
            if positions.len() == NUMBERS.len() {
                break;
            }
        }
    }
    positions
}
/// Draw-only figures: no spawn, attack, status, quest or save modification.
pub(crate) fn scene_actors(
    camera: &crate::terminal_view::GridCamera,
    game: &project_rl::game::WorldState,
) {
    if MODE.get() == 0 || !SCENE.get() {
        return;
    }
    for (at, number) in positions(game).into_iter().zip(NUMBERS) {
        super::draw_world(
            camera.rect(at),
            Visual::CampaignMonster { number, variant: 0 },
            1.,
        );
    }
}
