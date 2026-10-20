//! EG01: opt-in render/data proposal. Never enabled by ordinary game creation.
use crate::{prison_street::Street, terminal_view::GridCamera};
use macroquad::prelude::*;
use project_rl::game::WorldState;
use std::{cell::Cell, sync::OnceLock};

static REVIEW_REVISION: OnceLock<u8> = OnceLock::new();
pub fn configure_review(revision: u8) {
    assert!((1..=4).contains(&revision));
    assert_eq!(*REVIEW_REVISION.get_or_init(|| revision), revision);
}
pub fn revision() -> u8 {
    *REVIEW_REVISION.get_or_init(|| 1)
}
pub fn enriched() -> bool {
    revision() > 1
}
pub fn review_id() -> &'static str {
    match revision() {
        4 => "EG-COMP-04",
        3 => "EG-HAB-03",
        2 => "EG-DEC-02",
        _ => "EG01",
    }
}
fn asset_root() -> &'static str {
    match revision() {
        4 => "assets/source-art/sewer-review-04",
        3 => "assets/source-art/sewer-review-03",
        2 => "assets/source-art/sewer-review-02",
        _ => "assets/source-art/sewer-review-01",
    }
}

/// Full-map review export only; never changes the player's visibility or AI.
pub fn draw_overview_population(game: &WorldState) {
    if revision() != 4 || !active(game) {
        return;
    }
    let Some(index) = MODE.get().filter(|&i| i > 0) else {
        return;
    };
    for group in &plan(index).groups {
        for member in &group.members {
            crate::game_art::draw_world(
                Rect::new(
                    member.at[0] as f32 * 64.,
                    member.at[1] as f32 * 64.,
                    64.,
                    64.,
                ),
                crate::game_art::Visual::CampaignMonster {
                    number: member.number,
                    variant: member.variant,
                },
                1.,
            );
        }
    }
}

pub use crate::sewer::{Member, Plan};
thread_local! {
    static MODE:Cell<Option<usize>>=const {Cell::new(None)};
}
pub fn select(mode: Option<usize>) {
    MODE.set(mode);
    crate::prison_street_render::review_repairs(false);
}
pub fn active(game: &WorldState) -> bool {
    MODE.get().is_some_and(|m| {
        game.current_zone().is_some_and(|z| {
            z.id.as_str()
                == if m == 0 {
                    crate::prison_street::ZONE_ID
                } else {
                    "core:sewer_review"
                }
        })
    })
}
pub fn scene(game: &WorldState) -> Option<&'static Street> {
    if !active(game) {
        return None;
    }
    static SCENES: OnceLock<Vec<Street>> = OnceLock::new();
    let scenes = SCENES.get_or_init(|| {
        (0..4)
            .map(|i| {
                let folder = if i == 0 {
                    "entry".into()
                } else {
                    format!("seed-{i}")
                };
                let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("{}/{folder}/street.json", asset_root()));
                serde_json::from_str(&std::fs::read_to_string(file).expect("EG01 scene file"))
                    .expect("EG01 scene")
            })
            .collect()
    });
    Some(&scenes[MODE.get().unwrap()])
}
pub fn plan(index: usize) -> &'static Plan {
    static PLANS: OnceLock<Vec<Plan>> = OnceLock::new();
    &PLANS.get_or_init(|| {
        (1..=3)
            .map(|i| {
                let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("{}/seed-{i}/plan.json", asset_root()));
                serde_json::from_str(&std::fs::read_to_string(file).expect("EG01 plan file"))
                    .expect("EG01 plan")
            })
            .collect()
    })[index - 1]
}
pub fn street_plan(index: usize) -> &'static Plan {
    static PLANS: OnceLock<Vec<Plan>> = OnceLock::new();
    &PLANS.get_or_init(|| {
        (1..=2)
            .map(|i| {
                let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(format!("{}/street-population-{i}.json", asset_root()));
                serde_json::from_str(&std::fs::read_to_string(file).expect("POP-D2-02 plan file"))
                    .expect("POP-D2-02 plan")
            })
            .collect()
    })[index - 1]
}
pub fn water(camera: &GridCamera, game: &WorldState, time: f32) {
    if let Some(index) = MODE.get().filter(|&i| i > 0)
        && active(game)
    {
        crate::sewer_ambience::water(camera, plan(index), time);
    }
}
pub fn rats(camera: &GridCamera, game: &WorldState, time: f32) {
    if let Some(index) = MODE.get().filter(|&i| i > 0)
        && active(game)
    {
        crate::sewer_ambience::rats(camera, plan(index), time);
    }
}
