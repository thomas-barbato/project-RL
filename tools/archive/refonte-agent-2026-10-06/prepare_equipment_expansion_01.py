"""Append equipment direction changes and a six-model-per-series review plan."""
from copy import deepcopy
from pathlib import Path
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
HELPERS = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="helpers")
load, ref, pair, save, preserve = (HELPERS[k] for k in ["load", "ref", "pair", "save", "preserve"])
DATE = "2026-10-10"
BOX = "equipped-ammunition-box-direction-decisions-01.json"
EXPAND = "caster-shield-heavy-expansion-directions-01.json"
PLAN = "review-equipment-series-EQUIP-SERIES01-01.json"
BATCH = "review-class-starts-START-ALL01-11.json"
PAIRS = [
    ("review-implementation-readiness-42.json", "review-implementation-readiness-43.json"),
    ("review-attribute-model-100.json", "review-attribute-model-101.json"),
    ("review-primary-catalogue-272.json", "review-primary-catalogue-273.json"),
    ("review-complete-skill-design-247.json", "review-complete-skill-design-248.json"),
    ("remaining-individual-skill-review-counts-141.json", "remaining-individual-skill-review-counts-142.json"),
]


def main():
    save(BOX, {
        "date": DATE, "review_id": "AMMO-BOX01", "revision": 1,
        "user_confirmation": "oui, est-ce que ca te semble correcte ?",
        "accepted_synthesis_scope": "replace_consumed_ammunition_with_equipped_box;new_equipment_slot_on_character_silhouette;shop_and_loot_with_magical_versions;slightly_lower_ordinary_ranged_weapon_skill_damage_than_melee;heavy_weapons_excluded_from_that_reduction",
        "accepted_direction": {
            "item_family_label": "Boîte de munitions",
            "requires_equipped_box_for_agent_ranged_weapons_including_heavy": True,
            "inventory_presence_alone_satisfies_requirement": False,
            "quantity_consumption_per_attack": False,
            "new_dedicated_equipment_slot": True,
            "silhouette_equipment_slot": True,
            "shop_and_loot": True,
            "magical_versions": True,
            "ranged_skill_balance": "slightly_lower_damage_than_comparable_melee_skills;not_a_change_to_heavy_weapon_skills",
        },
        "supersedes": [ref("universal-ammunition-and-matter-removal-decisions.json"), ref("review-universal-ammunition-AMMO02-02.json")],
        "superseded_start_parameters": "START_GEAR01_40_ammunition_stacks_and1_per_projectile_no_longer_apply;remaining_accepted_start_gear_and_consumables_preserved",
        "preserved": "common_skill_energy_and_energy_only_secondary_mine_and_grenade_skills;no_change_to_accepted_implants_attributes_or_skill_effects_except_requested_future_ranged_damage_rebalance",
        "still_to_review": ["exact_box_models_names_RP_stats_and_magical_affixes", "shop_prices_and_loot_distribution", "native_art_and_equipment_UI_slot_placement", "starting_boxes_for_gun_users", "familiar_and_monster_requirements", "exact_ranged_vs_melee_damage_values"],
        "no_inferred_rules": "no_new_charges_durability_lifetime_or_energy_cost_for_box;no_mandatory_box_for_spellcasting_inferred",
        "runtime_integration": "not_integrated",
    })
    save(EXPAND, {
        "date": DATE, "review_id": "EQUIP-EXPAND01", "revision": 1,
        "user_acceptance": "parfait :) , il faudra là aussi en faire plusieurs variété (autant que celles des armes normales) . Il faudra aussi faire les armes lourdes , on les a pas encore traitées .",
        "accepted_family_directions": [
            {"id": "caster:focalizer", "label": "Focaliseur occulte", "held_direction": "one_hand;short_grip_forked_head_and_contrasting_core;aligned_with_existing_hand_and_forearm"},
            {"id": "caster:channeling_shaft", "label": "Hampe de canalisation", "held_direction": "two_hands;continuous_metal_shaft_and_simple_industrial_head;separate_main_and_support_grips"},
            {"id": "defense:shield", "label": "Bouclier", "held_direction": "one_hand;compact_solid_metal_trapezoid_or_hexagon;forearm_attachment_and_rear_grip;torso_and_other_weapon_readable"},
        ],
        "variety_direction": "several_models_per_family_comparable_to_ordinary_weapon_series;not_only_cosmetic_recolors",
        "heavy_direction": ref("heavy-weapon-family-series-decisions.json"),
        "deferred_options": "ritual_gauntlet_and_sealed_relic_not_first_production_families",
        "approval_boundary": "accepted_labels_hand_count_and_visual_intent;no_specific_new_pixels_item_RP_stats_shield_block_rules_or_mandatory_caster_weapon_requirement_approved",
        "runtime_integration": "not_integrated",
    })

    bank_name = "weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json"
    bank = load(bank_name)
    actual_families = []
    for family in bank["families"]:
        actual_families.append({"index": family["index"], "name": family["name"], "models": sum(m["family"] == family["index"] for m in bank["model_mappings"])})
    series = []
    specs = [
        ("caster:focalizer", "Focaliseurs occultes", "EQ-F", None),
        ("caster:channeling_shaft", "Hampes de canalisation", "EQ-H", None),
        ("defense:shield", "Boucliers", "EQ-S", None),
        ("heavy:flamethrower", "Lance-flammes", "EQ-LF", 12),
        ("heavy:rocket_launcher", "Lance-roquettes", "EQ-LR", 14),
        ("heavy:gatling", "Gatlings", "EQ-GA", None),
        ("heavy:plasma_launcher", "Lance-plasma", "EQ-LP", None),
        ("heavy:grenade_launcher", "Lance-grenades", "EQ-LG", 13),
    ]
    for family_id, label, prefix, source_family in specs:
        existing = [m["id"] for m in bank["model_mappings"] if m["family"] == source_family] if source_family is not None else []
        series_ids = [model for model in existing if model.startswith("core:modele_")]
        if existing:
            assert len(series_ids) == 6
        series.append({
            "family_id": family_id, "family_label": label,
            "target_models_in_series": 6,
            "stable_candidate_ids": [f"{prefix}{number:02}" for number in range(1, 7)],
            "existing_series_ids_to_preserve": series_ids,
            "additional_existing_base_ids_to_preserve": [model for model in existing if model not in series_ids],
            "item_names_RP_stats": "not_proposed_yet;require_displayed_review",
            "model_art": "pending_numbered_native_review;different_readable_models_with_stable_attachment_points",
        })
    save(PLAN, {
        "date": DATE, "review_id": "EQUIP-SERIES01", "revision": 1,
        "status": "catalogue_production_plan;models_and_art_not_created",
        "authority": ref(EXPAND), "box_direction": ref(BOX), "source_weapon_bank": ref(bank_name),
        "count_basis": "six_model_series_observed_in_pistols_SMG_and_existing_launcher_series;family_counts_not_uniform;additional_existing_base_models_retained",
        "observed_existing_families": actual_families,
        "series": series, "total_series_models_to_create_or_review": 48,
        "box_models": "separate_production_batch;normal_and_magical_direction_accepted;model_count_not_yet_fixed",
        "existing_art_boundary": "113_existing_weapons_have_approved_common_family_held_layers;new_series_and_distinct_model_geometry_need_own_native_review",
        "new_heavy_identity": "Gatling_and_heavy_plasma_are_new_series;do_not_silently_rename_existing_machineguns_or_energy_rifles",
        "required_review_views": "numbered_ground_inventory_icons;held_on_all_five_fixed_bodies;left_right_for_one_hand;two_grips_for_two_hands;focalizer_plus_shield_and_melee_plus_shield;native32_and_integer_zoom_on_multiple_floors",
        "accepted_visual_invariants": "native32_fixed_body;separate_weapons_arms_fingers_material_maps;no_extra_forearm_for_one_hand;weapon_palette_independent_of_armor;no_pink;overflow_without_larger_collision",
        "pixel_approval": "pending;no_bitmap_created_in_this_inventory_step",
        "count_status": "production_target_derived_from_user_comparable_variety_direction;no_new_gameplay_effect",
        "runtime_integration": "not_integrated",
    })

    old_batch = load("review-class-starts-START-ALL01-10.json")
    batch = deepcopy(old_batch)
    batch.update(date=DATE, revision=11, status="equipped_box_direction_and_equipment_series_expansion_accepted;new_models_and_start_boxes_pending")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-10.json")
    batch["previous_pending_universal_ammunition_review"] = batch.get("pending_universal_ammunition_review")
    batch["pending_universal_ammunition_review"] = None
    batch["accepted_equipped_ammunition_box_direction"] = ref(BOX)
    batch["accepted_caster_shield_heavy_expansion_direction"] = ref(EXPAND)
    batch["pending_equipment_series_review"] = ref(PLAN)
    batch["starting_ammunition_precedence"] = "AMMO_BOX01_supersedes_historical_40_ammunition_and_projectile_consumption;starting_box_items_not_implicitly_finalized;other_start_gear_preserved"
    preserve(old_batch, batch)
    assert old_batch["profiles"] == batch["profiles"]
    save(BATCH, batch)

    for old_name, new_name in PAIRS:
        old, data = load(old_name), deepcopy(load(old_name))
        data["date"] = DATE
        data["previous_pending_universal_ammunition_review"] = data.get("pending_universal_ammunition_review")
        data["pending_universal_ammunition_review"] = None
        data["accepted_equipped_ammunition_box_direction"] = ref(BOX)
        data["accepted_caster_shield_heavy_expansion_direction"] = ref(EXPAND)
        data["pending_equipment_series_review"] = ref(PLAN)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["next_review_kind"] = "numbered_native_equipment_series_prototypes;box_slot_and_item_design;new_stats_and_ranged_damage_rebalance_separate"
        overrides = data.setdefault("active_rule_overrides", {})
        overrides["equipped_box_replaces_consumed_ammunition"] = {
            **ref(BOX), "precedence": "supersedes_consumed_universal_matter_ammunition_and_start_stock_counts;other_accepted_skill_energy_and_equipment_rules_preserved",
            "scope": "agent_ranged_weapon_enable_condition_including_heavy;familiar_monster_and_spell_requirements_not_inferred", "runtime_integration": "not_integrated",
        }
        overrides["new_equipment_series_direction"] = {**ref(EXPAND), "production_plan": ref(PLAN), "scope": "family_and_variety_direction_only;new_pixels_and_model_mechanics_pending", "runtime_integration": "not_integrated"}
        if old_name.startswith("review-implementation"):
            data.update(revision=43, status="ammo_box_and_caster_shield_heavy_series_directions_accepted;production_inventory_prepared")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(EXPAND)
            data["recommendation"] = "prepare_reviewable_new_equipment_families_and_box_slot;do_not_implement_superseded_consumed_ammunition;continue_approved_refonte_foundations"
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY03":
                    group["remaining"] = [item for item in group["remaining"] if item not in ["replace_weapon_matter_with_universal_ammunition;per_model_costs_reload_weight_loot_and_familiar_monster_supply_need_review", "HEAVY_FAMILY01_multiple_models_for_five_requested_families;stats_ammo_loot_and_native_character_grips_need_review"]]
                    group["remaining"] += ["equipped_ammunition_box_slot_models_magical_affixes_shop_loot_and_actor_scope", "caster_shield_and_five_heavy_series_models_stats_loot_and_numbered_native_grip_review", "ordinary_ranged_weapon_skill_damage_rebalance_without_automatic_heavy_reduction"]
                if group["id"] == "READY04":
                    group["remaining"] = ["character_creation_UI_using_already_accepted_native_class_silhouettes", "starting_equipped_boxes_for_ranged_classes_and_replacement_implant_loot"]
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["ammunition_precedence"] = ref(BOX)
        else:
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL98", status="equipment_box_and_series_directions_updated;approved_attributes_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", EXPAND)
                data["latest_proposal_scope"] = "new_equipment_series_inventory_and_equipped_box_direction;no_new_model_effect_or_pixel_approved"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "equipment_family_variety_and_box_direction_updated;skill_review_counts_unchanged"
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert data["active_rule_overrides"][key] == value, key
        save(new_name, data)

    hashes = runpy.run_path(str(BASE / "continue_pistol_arm_axis_review_01.py"), run_name="hash_helpers")["hashes"]
    names = [BOX, EXPAND, PLAN, BATCH] + [new for _, new in PAIRS]
    checked = sum(hashes(load(name)) for name in names)
    print(json.dumps({"series": 8, "models_per_series": 6, "series_model_slots": 48, "references_checked": checked, "existing_models_preserved": 113, "runtime_modified": False}))


if __name__ == "__main__":
    main()
