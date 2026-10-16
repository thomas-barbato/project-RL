//! Reviewed agent catalogue shared by simulation and input dispatch.
use super::TechniqueId;
use std::collections::BTreeMap;
use std::sync::OnceLock;

#[derive(serde::Deserialize)]
struct Catalogue {
    skills: Vec<AgentSkill>,
}
#[derive(serde::Deserialize)]
pub struct AgentSkill {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub branch: String,
    pub parameters: BTreeMap<String, serde_json::Value>,
    pub centred_damage: Option<super::area_damage::CentredDamageBinding>,
}
pub fn definitions() -> &'static [AgentSkill] {
    static DATA: OnceLock<Catalogue> = OnceLock::new();
    &DATA
        .get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/skills/agent/catalogue.json"))
                .expect("reviewed agent catalogue")
        })
        .skills
}
pub fn definition(id: &str) -> Option<&'static AgentSkill> {
    definitions().iter().find(|s| s.id == id)
}
pub fn technique(id: &str) -> TechniqueId {
    format!("agent:{}", id.to_ascii_lowercase())
        .parse()
        .expect("catalogue skill identifier")
}
pub fn from_technique(id: &TechniqueId) -> Option<&'static AgentSkill> {
    let suffix = id.as_str().strip_prefix("agent:")?;
    definitions()
        .iter()
        .find(|s| s.id.to_ascii_lowercase() == suffix)
}
pub fn implemented(id: &str) -> bool {
    definition(id).is_some()
}
/// Accepted D66-TAUNT01-NUM01; same catalogue is rebuilt when restoring a run.
pub fn install_status_definitions(catalog: &mut crate::status::StatusCatalog) {
    for name in [
        "frozen",
        "aegis",
        "thorns",
        "regeneration",
        "armor_breach",
        "silence",
        "regeneration_break",
        "disorientation",
        "interference",
        "domination",
        "stealth",
        "spectral_step",
        "phase_traversal",
        "last_surge",
        "stunned",
        "blind",
    ] {
        let id = format!("agent:{name}").parse().unwrap();
        if catalog.get(&id).is_none() {
            catalog
                .register(
                    crate::status::StatusDefinition::new(
                        id,
                        Some(5),
                        crate::status::StatusStacking::RefreshDuration,
                        vec![],
                    )
                    .unwrap(),
                )
                .unwrap();
        }
    }
    crate::game::game_state::agent_devices::install_statuses(catalog);
    let id = "agent:provoked".parse().unwrap();
    if catalog.get(&id).is_none() {
        catalog
            .register(
                crate::status::StatusDefinition::new(
                    id,
                    Some(2),
                    crate::status::StatusStacking::KeepExisting,
                    vec![],
                )
                .expect("validated taunt duration"),
            )
            .expect("unique agent status");
    }
}
/// Reconstructs only the status-catalogue delta preceding the first complete
/// playable agent runtime. Overrides and all unrelated definitions remain.
/// Callers must still verify the historical complete rules and state hashes.
pub fn legacy_v14_status_definitions(
    catalog: &crate::status::StatusCatalog,
) -> crate::status::StatusCatalog {
    let mut defaults = crate::status::StatusCatalog::default();
    install_status_definitions(&mut defaults);
    let mut legacy = catalog.clone();
    for name in [
        "frozen",
        "aegis",
        "thorns",
        "regeneration",
        "armor_breach",
        "silence",
        "regeneration_break",
        "disorientation",
        "interference",
        "domination",
        "stealth",
        "spectral_step",
        "phase_traversal",
        "last_surge",
        "stunned",
        "blind",
        "mine_frozen",
    ] {
        let id = format!("agent:{name}").parse().unwrap();
        if catalog.get(&id) == defaults.get(&id) {
            legacy = legacy.without_id(&id);
        }
    }
    legacy
}

/// Encodes an in-memory historical fixture. This diagnostic has no file access
/// and is absent from release builds; live saving always writes the latest schema.
#[cfg(debug_assertions)]
#[doc(hidden)]
pub fn legacy_v14_fixture_bytes(world: &crate::game::WorldState) -> Result<Vec<u8>, String> {
    let mut bytes = crate::game::game_state::agent_runtime::with_legacy_encoding(|| {
        world.recovery_snapshot_bytes()
    })?;
    if !bytes.starts_with(b"RLWS\x0f") {
        return Err("Ancien format d'instantané : reprise par le journal requise.".into());
    }
    bytes[4] = 14;
    Ok(bytes)
}

impl AgentSkill {
    pub fn required_weapon_delivery(&self) -> Option<crate::combat::AttackDelivery> {
        if self.kind != "active" {
            return None;
        }
        match self.branch.as_str() {
            "Mêlée" => Some(crate::combat::AttackDelivery::Melee),
            "Attaques à distance" | "Armes lourdes" => Some(crate::combat::AttackDelivery::Ranged),
            _ => None,
        }
    }
    pub fn value(&self, key: &str, rank: u8) -> u16 {
        let p = &self.parameters[key];
        let number = |k: &str| p[k].as_f64().unwrap_or(0.);
        let n = f64::from(rank.saturating_sub(1));
        let mut value = match p["kind"].as_str().unwrap_or("") {
            "constant" => number("value"),
            "linear" | "scaled_linear_damage" => number("base") + n * number("gain_per_rank"),
            "stepped" => {
                number("base")
                    + ((n + number("rank_offset")) / number("every_ranks"))
                        .floor()
                        .max(0.0)
                        * number("gain_per_step")
            }
            _ => panic!("unsupported executable curve for {} {key}", self.id),
        };
        if p["minimum"].is_number() {
            value = value.max(number("minimum"));
        }
        if p["maximum"].is_number() {
            value = value.min(number("maximum"));
        }
        value.clamp(0., f64::from(u16::MAX)) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepted_cadence_applies_to_spells_physical_actions_and_summons_at_every_rank() {
        let receipt: serde_json::Value = serde_json::from_str(include_str!(
            "../../docs/reviews/refonte-agent-2026-10-06/agent-next-action-cadence-acceptance.json"
        ))
        .unwrap();
        for entry in receipt["parameters"].as_array().unwrap() {
            let skill = definition(entry["skill"].as_str().unwrap()).unwrap();
            let key = entry["key"].as_str().unwrap();
            assert_eq!(skill.parameters[key]["cadence"], "next_player_action");
            for rank in [1, 10, 11, 50, 255] {
                assert_eq!(skill.value(key, rank), 1, "{} {key} {rank}", skill.id);
            }
        }
        for id in ["FE05", "B01", "DI04", "D56", "D57", "D58", "D59", "D66"] {
            assert_eq!(definition(id).unwrap().value("cooldown_turns", 1), 1);
        }
        assert!(definition("SU06").unwrap().value("cooldown_turns", 1) >= 30);
        assert_eq!(definition("DR01").unwrap().value("cooldown_turns", 1), 1);
        // Changing invocation cadence does not change energy or device internals.
        assert_eq!(definition("D66").unwrap().value("energy", 1), 14);
    }
}
