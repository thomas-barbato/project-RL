use super::*;
use crate::loot::{EquipmentBaseDefinition, EquipmentStatChoice};
use crate::weapon::WeaponEffectAffixDefinition;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEffectAffix {
    id: String,
    suffix_key: String,
    description_key: String,
    effects: Vec<RawWeaponEffect>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEquipmentBases {
    id: String,
    // Shared defaults avoid repeating the same affix list for every base.
    stats: Vec<EquipmentStatChoice>,
    bases: Vec<EquipmentBaseDefinition>,
}

fn invalid(package: &DiscoveredPackage, path: &Path, explanation: String) -> ContentLoadError {
    ContentLoadError::DefinitionSyntax {
        package: package.manifest.id.clone(),
        path: path.to_owned(),
        definition_kind: "equipment generation",
        explanation,
    }
}

pub(super) fn load_effect_affixes(
    package: &DiscoveredPackage,
    statuses: &StatusCatalog,
    catalog: &mut WeaponCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "weapon_affixes")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawEffectAffix =
            json5::from_str(&source).map_err(|e| invalid(package, &path, e.to_string()))?;
        let id = parse_content_id(package, &path, &raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        if raw.effects.len() > 64 {
            return Err(invalid(package, &path, "Too many affix effects".into()));
        }
        let effects = raw
            .effects
            .into_iter()
            .map(|effect| effect.into_runtime(statuses))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| invalid(package, &path, e.to_string()))?;
        let affix =
            WeaponEffectAffixDefinition::new(id, raw.suffix_key, raw.description_key, effects)
                .map_err(|e| invalid(package, &path, e))?;
        catalog
            .register_effect_affix(affix)
            .map_err(|e| invalid(package, &path, e.to_string()))?;
    }
    Ok(())
}

pub(super) fn load_equipment_bases(
    package: &DiscoveredPackage,
    items: &ItemCatalog,
    weapons: &WeaponCatalog,
    loot: &mut LootCatalog,
) -> Result<(), ContentLoadError> {
    let mut set_ids = std::collections::BTreeSet::new();
    for path in definition_paths(package, "equipment_loot")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawEquipmentBases =
            json5::from_str(&source).map_err(|e| invalid(package, &path, e.to_string()))?;
        let id = parse_content_id(package, &path, &raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        if !set_ids.insert(id)
            || raw.bases.is_empty()
            || raw.bases.len() > 1024
            || raw.stats.len() > 10
        {
            return Err(invalid(package, &path, "Invalid equipment set".into()));
        }
        for mut base in raw.bases {
            ensure_local_namespace(package, &path, &base.item)?;
            if base.stats.is_empty() {
                base.stats = raw.stats.clone();
            }
            loot.equipment_mut()
                .register(base, items, weapons)
                .map_err(|e| invalid(package, &path, e))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generator_content_rejects_malformed_ids_negative_weights_and_unknown_fields() {
        let valid = r#"{ item: 'core:blade', family: 'core:knives', tier: 1, grammar: 'ms', form: 'humanoid_equipment', sources: ['humanoid_site'], stats: [{ affix: 'affix:power', weight: 100 }] }"#;
        assert!(json5::from_str::<EquipmentBaseDefinition>(valid).is_ok());
        for bad in [
            valid.replace("core:blade", "bad identifier"),
            valid.replace("weight: 100", "weight: -1"),
            valid.replace("weight: 100", "weight: 1.5"),
            valid.replace("weight: 100", "weight: 4294967296"),
            valid.replace("weight: 100", "weight: Infinity"),
            valid.replace("tier: 1", "tier: 257"),
            valid.replace("tier: 1", "tier: 1.5"),
            valid.replace("tier: 1", "tier: 1, rarity_guess: 4"),
            valid.replace("humanoid_site", "anything"),
        ] {
            assert!(
                json5::from_str::<EquipmentBaseDefinition>(&bad).is_err(),
                "accepted: {bad}"
            );
        }
    }

    #[test]
    fn special_affix_content_uses_the_existing_validated_effect_recipes() {
        let raw: RawEffectAffix = json5::from_str(r#"{ id: 'test:burning', suffix_key: 'suffix', description_key: 'description', effects: [{ type: 'apply_status', status: 'test:missing', trigger: 'on_hit' }] }"#).unwrap();
        assert!(matches!(
            raw.effects
                .into_iter()
                .next()
                .unwrap()
                .into_runtime(&StatusCatalog::default()),
            Err(WeaponDefinitionError::UnknownStatus(_))
        ));
        assert!(json5::from_str::<RawEffectAffix>(r#"{ id: 'test:unknown', suffix_key: 'suffix', description_key: 'description', effects: [{ type: 'invented_attack' }] }"#).is_err());
    }
}
