"""Accept shown starter gear and register native class and heavy-family reviews.

Immutable design revisions only; active gameplay and artwork stay untouched.
"""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
H = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (H[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "starting-equipment-acceptance-decisions.json"
GEAR = "review-starting-equipment-START-GEAR01-02.json"
VISUAL = "review-equipped-classes-START-VIS01-01.json"
HEAVY = "heavy-weapon-family-series-decisions.json"
BATCH = "review-class-starts-START-ALL01-04.json"
PAIRS = [("review-implementation-readiness-35.json", "review-implementation-readiness-36.json"),
         ("review-attribute-model-93.json", "review-attribute-model-94.json"),
         ("review-primary-catalogue-265.json", "review-primary-catalogue-266.json"),
         ("review-complete-skill-design-240.json", "review-complete-skill-design-241.json"),
         ("remaining-individual-skill-review-counts-134.json", "remaining-individual-skill-review-counts-135.json")]
NEW_FILES = [DECISION, GEAR, VISUAL, HEAVY, BATCH] + [new for _, new in PAIRS]


def validate():
    checked = 0
    def walk(value):
        nonlocal checked
        if isinstance(value, dict):
            for key, expected in value.items():
                if key == "binary_sha256" and "path" in value:
                    assert hashlib.sha256((ROOT / value["path"]).read_bytes()).hexdigest() == expected
                    checked += 1
                elif key.endswith("_sha256") and key not in {"binary_sha256", "calculations_sha256"}:
                    source = value.get(key[:-7], value.get(key[:-7] + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/", "content/")):
                            raise AssertionError(("missing", source))
                    assert hashlib.sha256(content.encode("utf-8")).hexdigest() == expected, (key, source[:100])
                    checked += 1
            for child in value.values():
                walk(child)
        elif isinstance(value, list):
            for child in value:
                walk(child)
    for name in NEW_FILES:
        walk(load(name))
    old, gear = load("review-starting-equipment-START-GEAR01-01.json"), load(GEAR)
    for previous, current in [("proposed_loadouts", "accepted_loadouts"),
                              ("proposed_common_consumables", "accepted_common_consumables"),
                              ("proposed_start_weapon_ammo_cost", "accepted_start_weapon_ammo_cost")]:
        assert old[previous] == gear[current], current
    assert gear["player_copy_markdown"] == old["player_copy_markdown"]
    assert gear["mechanic_approval"] == "shown_values_accepted"
    assert gear["visual_approval"] == "native_equipped_previews_pending;no_new_pixels_integrated"
    assert len(load(VISUAL)["candidates"]) == 5
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_starting_equipment"] is None
        assert new["accepted_starting_equipment"] == ref(GEAR)
        assert new["pending_equipped_class_visual_review"] == ref(VISUAL)
        assert new["runtime_integration"] == "not_integrated"
    old, batch = load("review-class-starts-START-ALL01-03.json"), load(BATCH)
    assert batch["profiles"] == old["profiles"]
    assert batch["implant_conditions"] == old["implant_conditions"]
    assert batch["pending_starting_equipment"] is None
    assert load(PAIRS[-1][1])["remaining_first_pass_total"] == 0
    assert [r["display_name"] for r in load(HEAVY)["requested_families"]] == [
        "Lance-flammes", "Lance-roquettes", "Gatling", "Lance-plasma", "Lance-grenades"]
    return checked


def main():
    save(DECISION, {"date": "2026-10-09", "review_id": "START-GEAR01-ACCEPT01",
                   "user_acceptance": "ok :) on peut continuer",
                   "shown_proposal": ref("review-starting-equipment-START-GEAR01-01.json"),
                   "approved_scope": "displayed_level1_start_weapon_ranges_range_accuracy_armor_evasion_stocks_consumable_names_quantities_restore12_grant_RP_one_ammo_per_start_pistol_rifle_projectile_and_derived_B01_preview",
                   "unshown_not_implicitly_approved": ["individual_item_RP_descriptions", "weights_stack_limits_reload_magazine_rules",
                                                        "heavy_model_statistics_and_ammunition_costs", "actor_ammunition_rules",
                                                        "equipped_class_compositions_creation_UI_or_new_item_icons", "unshown_implant_eligibility_conditions"],
                   "runtime_integration": "not_integrated"})
    gear = deepcopy(load("review-starting-equipment-START-GEAR01-01.json"))
    gear.update(revision=2, status="shown_starting_gear_text_and_numbers_accepted;native_equipped_review_separate",
                mechanic_approval="shown_values_accepted", text_approval="displayed_item_names_grant_RP_and_numeric_copy_accepted")
    for previous, current in [("proposed_loadouts", "accepted_loadouts"),
                              ("proposed_common_consumables", "accepted_common_consumables"),
                              ("proposed_start_weapon_ammo_cost", "accepted_start_weapon_ammo_cost")]:
        gear[current] = gear.pop(previous)
    gear["previous_proposal"] = ref("review-starting-equipment-START-GEAR01-01.json")
    gear["acceptance"] = ref(DECISION)
    gear["pending_equipped_class_visual_review"] = ref(VISUAL)
    save(GEAR, gear)
    save(HEAVY, {"date": "2026-10-09", "review_id": "HEAVY-FAMILY01", "revision": 1,
                 "user_request": "ah oui il faudra créer aussi plusieurs séries d'arme lourdes par familles d'armes lourdes (et faire en sorte de les adapter aux personnages). Lance flammes (mais déjà existants eux), Lance roquette, Gatling, Lance plasma, Lance grenades. Et je n'ai pas d'autres idées.",
                 "requested_families": [
                     {"stable_family_id": "heavy:flamethrower", "display_name": "Lance-flammes", "existing_models": True},
                     {"stable_family_id": "heavy:rocket_launcher", "display_name": "Lance-roquettes", "existing_models": True},
                     {"stable_family_id": "heavy:gatling", "display_name": "Gatling", "existing_models": "existing_heavy_machineguns_are_references;new_Gatling_series_not_yet_drawn"},
                     {"stable_family_id": "heavy:plasma_launcher", "display_name": "Lance-plasma", "existing_models": "energy_weapons_are_references;new_heavy_plasma_series_not_yet_drawn"},
                     {"stable_family_id": "heavy:grenade_launcher", "display_name": "Lance-grenades", "existing_models": True}],
                 "accepted_direction": "prepare_multiple_model_series_per_requested_heavy_weapon_family_and_adapt_held_layers_grips_and_poses_to_characters;reuse_existing_flamethrowers_as_starting_sources",
                 "requested_family_names": "user_authored_direction;player_item_names_and_RP_to_be_reviewed_per_model",
                 "not_fixed": ["model_count_per_series", "levels_damage_variance_effects_accuracy_range", "weight_and_attribute_requirements",
                               "ammo_costs_reload", "new_native_art_and_animations", "individual_model_names_RP_and_loot_distribution"],
                 "scope": "additional_heavy_weapon_catalogue_work;does_not_delete_other_existing_weapon_families_or_modify_current_models",
                 "art_review": "follow_docs_DIRECTION_ARTISTIQUE_TILES;numbered_native32_candidates_and_character_grips_before_integration",
                 "ammunition_direction": ref("universal-ammunition-and-matter-removal-decisions.json"),
                 "mechanic_approval": "family_catalogue_direction_only", "visual_approval": "new_series_pending_numbered_native_review",
                 "runtime_integration": "not_integrated"})
    old = load("review-class-starts-START-ALL01-03.json")
    batch = deepcopy(old)
    batch.update(revision=4, status="shown_class_core_and_initial_gear_accepted;equipped_compositions_proposed;heavy_series_requested",
                 equipment_approval="START_GEAR01_shown_names_numbers_consumables_and_stocks_accepted",
                 new_text_approval="shown_class_and_START_GEAR01_copy_accepted;unshown_item_and_implant_copy_separate")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-03.json")
    batch["latest_equipment_acceptance"] = ref(DECISION)
    batch["previous_pending_starting_equipment"] = deepcopy(batch["pending_starting_equipment"])
    batch["pending_starting_equipment"] = None
    batch["accepted_starting_equipment"] = ref(GEAR)
    batch["pending_equipped_class_visual_review"] = ref(VISUAL)
    batch["accepted_heavy_weapon_family_direction"] = ref(HEAVY)
    batch["profile_equipment_metadata_precedence"] = "historical_proposed_markers_in_preserved_profiles_are_superseded_only_for_shown_values_by_START_GEAR01_rev2;unshown_implant_appendix_still_pending"
    batch["remaining"] = ["unshown_initial_implant_scope_and_penalty_pipeline", "ammo_model_costs_reload_and_actor_management",
                          "START_VIS01_numbered_equipped_compositions_and_later_creation_UI", "found_replacement_implant_loot_catalogue",
                          "HEAVY_FAMILY01_model_series_and_numbered_native_character_compositions"]
    batch["previous_player_review_document"] = batch.pop("player_review_document")
    batch["current_review_document_path"] = "docs/REVUE_TEXTES_EFFETS_AGENT.md"
    preserve(old, batch)
    save(BATCH, batch)
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_starting_equipment"] = deepcopy(data["pending_starting_equipment"])
        data["pending_starting_equipment"] = None
        data["accepted_starting_equipment"] = ref(GEAR)
        data["latest_start_equipment_acceptance"] = ref(DECISION)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["previous_pending_class_name_and_start_equipment_review"] = deepcopy(data["pending_class_name_and_start_equipment_review"])
        data["pending_class_name_and_start_equipment_review"] = None
        data["pending_equipped_class_visual_review"] = ref(VISUAL)
        data["accepted_heavy_weapon_family_direction"] = ref(HEAVY)
        data["next_review_kind"] = "START_VIS01_numbered_equipped_class_compositions"
        if old_name.startswith("review-implementation"):
            data.update(revision=36, status="START_GEAR01_accepted;START_VIS01_native_equipped_compositions_proposed;heavy_family_series_requested")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY03":
                    group["accepted_heavy_weapon_family_direction"] = ref(HEAVY)
                    group["remaining"].append("HEAVY_FAMILY01_multiple_models_for_five_requested_families;stats_ammo_loot_and_native_character_grips_need_review")
                elif group["id"] == "READY04":
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["previous_pending_equipment"] = deepcopy(group["pending_equipment"])
                    group["pending_equipment"] = None
                    group["accepted_initial_equipment"] = ref(GEAR)
                    group["pending_equipped_compositions"] = ref(VISUAL)
                    group["remaining"] = ["unshown_initial_implant_scope_and_penalty_pipeline", "START_VIS01_native_equipped_review_then_character_creation_UI",
                                          "ammunition_costs_reload_actor_management_and_replacement_implant_loot"]
                    group["resolved"] += ";START_GEAR01_shown_starting_gear_copy_and_numbers_accepted"
            data["recommendation"] = "review_START_VIS01_equipped_compositions_then_implant_scope_ammunition_profiles_and_requested_heavy_model_series"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["starting_equipment_START_GEAR01"] = {
                    **ref(GEAR), "acceptance": ref(DECISION),
                    "accepted_loadouts": deepcopy(gear["accepted_loadouts"]),
                    "accepted_common_consumables": deepcopy(gear["accepted_common_consumables"]),
                    "accepted_start_weapon_ammo_cost": deepcopy(gear["accepted_start_weapon_ammo_cost"]),
                    "scope": "shown_start_gear_values_only;no_unshown_armor_or_implant_classification;no_new_native_pixels_approved"}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL91", status="starting_gear_accepted;equipped_preview_proposed;heavy_family_direction;prior_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", VISUAL)
                data["latest_proposal_scope"] = "native_equipped_compositions_only;START_GEAR01_values_accepted;HEAVY_FAMILY01_series_requested"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "starter_gear_accepted;five_native_equipped_compositions_proposed;heavy_family_series_requested"
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("equipped-class-review-validation-01.json", {"date": "2026-10-09", "review_id": "START-VIS01-CHECK01",
         "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
         "shown_start_gear_values": "accepted_without_changes", "prior_accepted_fields_and_overrides": "preserved",
         "class_attributes_gifts_implants_reserves": "unchanged", "native_candidates": 5,
         "visual_checks": "binary_alpha;fixed_body_no_changes_outside_explicit_hand_cut_and_occlusion;7_grips;no_new_detached_fragments;no_asset_source_changes",
         "verification_scope": "design_records_and_existing_layer_composition;not_runtime_gameplay_or_live_game_capture",
         "runtime_integration": "not_integrated"})
    print(json.dumps({"files_checked": len(NEW_FILES), "hash_checks": checked, "errors": 0,
                      "starter_gear": "accepted", "equipped_compositions": "5_proposed", "heavy_families": 5,
                      "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
