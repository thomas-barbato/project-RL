//! U01-POP-01: accepted first-city groups, also used by the isolated trial.
use project_rl::{
    content::RegionalWorldCatalog,
    entity::Actor,
    monster_art::{self, ArtRole},
    world::{GridPos, Map, find_path},
};
use serde::Deserialize;
#[cfg(debug_assertions)]
use std::cell::Cell;
use std::{collections::BTreeSet, sync::OnceLock};

#[derive(Deserialize)]
pub struct Plan {
    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    pub review: String,
    pub safe_arrival_radius: u32,
    pub groups: Vec<Group>,
}
#[derive(Deserialize)]
pub struct Group {
    pub id: u8,
    pub level: u16,
    pub members: Vec<Member>,
}
#[derive(Deserialize)]
pub struct Member {
    pub at: [i32; 2],
    pub number: u16,
    pub variant: u8,
}
#[cfg(debug_assertions)]
thread_local! {static ACTIVE: Cell<bool> = const {Cell::new(false)};}

#[cfg(debug_assertions)]
pub fn enabled() -> bool {
    ACTIVE.get()
        || matches!(
            std::env::args().nth(1).as_deref(),
            Some("--essai-population-prison" | "--ui-cold-street-population")
        )
}
pub fn plan() -> &'static Plan {
    static PLAN: OnceLock<Plan> = OnceLock::new();
    PLAN.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/trials/street-population-01/population.json"
        ))
        .expect("U01-POP-01 plan")
    })
}

pub fn actors(catalog: &RegionalWorldCatalog, map: &Map) -> Result<Vec<Actor>, String> {
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .ok_or("Trial surface catalog absent")?;
    let entrance = GridPos::new(7, 62);
    let doors: Vec<_> = crate::prison_street::expanded_scene()
        .roofs
        .iter()
        .filter_map(|r| r.door.map(crate::prison_street::pos))
        .collect();
    let mut occupied = BTreeSet::new();
    let mut result = Vec::new();
    for group in &plan().groups {
        // Existing inhabited surface and wild-surface profiles, unchanged.
        let biome = world
            .biome(
                &if group.level == 1 {
                    "core:human_habitat"
                } else {
                    "core:surface_wilds"
                }
                .parse()
                .unwrap(),
            )
            .ok_or("Trial biome absent")?;
        for member in &group.members {
            let at = crate::prison_street::pos(member.at);
            let art =
                monster_art::art(member.number, member.variant).ok_or("Trial appearance absent")?;
            let ranged = art.role == ArtRole::CultistRanged;
            let rule = biome
                .encounters()
                .rules()
                .get(usize::from(ranged))
                .ok_or("Trial ordinary profile absent")?;
            if !map.is_walkable(at)
                || map.is_protected(at)
                || at.x.abs_diff(entrance.x).max(at.y.abs_diff(entrance.y))
                    <= plan().safe_arrival_radius
                || doors
                    .iter()
                    .any(|d| d.x.abs_diff(at.x).max(d.y.abs_diff(at.y)) <= 1)
                || !occupied.insert(at)
                || find_path(map, entrance, at, 4096, |_| true).is_none()
                || crate::prison_street::expanded_scene()
                    .roofs
                    .iter()
                    .any(|r| r.interior.contains(&member.at))
            {
                return Err(format!("Invalid trial placement {at:?}"));
            }
            let mut actor = Actor::new(at, rule.maximum_integrity())
                .map_err(|e| e.to_string())?
                .with_attack(rule.attack())
                .with_ai(rule.ai())
                .with_player_relation(rule.player_relation())
                .with_tags(rule.tags().iter().cloned());
            if let Some(attributes) = rule.primary_attributes() {
                actor = actor.with_primary_attributes(attributes);
            }
            if let Some(body) = rule.body_profile() {
                actor = actor.with_body_profile(body);
            }
            if !rule.body_components().is_empty() {
                actor = actor.with_body_components(rule.body_components().iter().cloned());
            }
            if let Some(mut reward) = rule.defeat_reward() {
                reward.threat_level = group.level;
                actor = actor.with_defeat_reward(reward);
            }
            actor = actor.with_tags([
                format!("core:monster_art_{}_{}", member.number, member.variant)
                    .parse()
                    .unwrap(),
                format!("core:encounter_level_{}", group.level)
                    .parse()
                    .unwrap(),
                format!("core:street_group_{}", group.id).parse().unwrap(),
                "core:street_population_01".parse().unwrap(),
            ]);
            result.push(actor);
        }
    }
    Ok(result)
}

#[cfg(all(test, debug_assertions))]
pub struct Scope(bool, bool);
#[cfg(all(test, debug_assertions))]
impl Scope {
    pub fn new() -> Self {
        Self(
            ACTIVE.replace(true),
            crate::street_defense_review::enabled(),
        )
    }
}
#[cfg(all(test, debug_assertions))]
impl Drop for Scope {
    fn drop(&mut self) {
        ACTIVE.set(self.0);
        crate::street_defense_review::enable(self.1);
    }
}
