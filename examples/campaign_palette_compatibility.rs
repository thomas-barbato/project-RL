//! Capture appearance assignments independently from UI and world generation.
use project_rl::{
    ai::AiProfile,
    combat::{AttackProfile, DamageType},
    entity::Actor,
    social::PlayerRelation,
    world::GridPos,
};

fn main() {
    let expanded = std::env::args().any(|a| a == "--expanded");
    let blue = std::env::args().any(|a| a == "--blue");
    let mut result = Vec::new();
    for depth in 0..8 {
        let mut actors: Vec<_> = (0..96)
            .map(|index| {
                Actor::new(GridPos::new(index % 12, index / 12), 50)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_attack(AttackProfile::melee(DamageType::Kinetic, 20))
            })
            .collect();
        project_rl::monster_art::assign(
            &mut actors,
            depth,
            0xface_d00d + u64::from(depth),
            |_| None,
            expanded,
            blue,
        );
        result.push(
            actors
                .iter()
                .map(project_rl::monster_art::appearance)
                .collect::<Vec<_>>(),
        );
    }
    println!("{}", serde_json::to_string(&result).unwrap());
}
