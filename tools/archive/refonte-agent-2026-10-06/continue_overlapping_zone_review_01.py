"""Accept the displayed zone rules and prepare the old-zone refresh case."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy
import sys

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
DECISION = "overlapping-zone-status-acceptance-decisions.json"
ACCEPTED = "review-overlapping-zone-status-INTERACT-ZONE02-02.json"
PROPOSAL = "review-old-zone-status-refresh-INTERACT-ZONE03-01.json"
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL, "review-implementation-readiness-06.json",
             "review-attribute-model-64.json", "review-primary-catalogue-236.json",
             "review-complete-skill-design-211.json", "remaining-individual-skill-review-counts-105.json"]
PAIRS = [("review-attribute-model-63.json", "review-attribute-model-64.json"),
         ("review-primary-catalogue-235.json", "review-primary-catalogue-236.json"),
         ("review-complete-skill-design-210.json", "review-complete-skill-design-211.json"),
         ("remaining-individual-skill-review-counts-104.json", "remaining-individual-skill-review-counts-105.json"),
         ("review-implementation-readiness-05.json", "review-implementation-readiness-06.json")]


def common(data):
    data["date"] = "2026-10-09"
    if data.get("pending_overlapping_zone_status"):
        data["previous_pending_overlapping_zone_status"] = deepcopy(data["pending_overlapping_zone_status"])
    data["pending_overlapping_zone_status"] = None
    data["accepted_overlapping_zone_status_rules"] = ref(ACCEPTED)
    data["latest_overlapping_zone_acceptance"] = ref(DECISION)
    data["pending_old_zone_status_refresh"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def add_override(data):
    data.setdefault("active_rule_overrides", {})["overlapping_zone_status_applications"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "current_rule": "each_zone_deals_own_damage_and_attempts_status_with_own_exposure_counter;one_status_per_kind;resolve_same_turn_zone_applications_oldest_cast_to_newest;last_successful_application_sets_power_and_restarts_duration",
        "failed_application": "no_replacement_or_refresh;zone_damage_remains_applicable",
        "preserved": "current_target_defenses_importance_rules_Gel_Burn_mutual_replacement_and_zone_launch_caster_power",
        "pending_cases": "older_zone_only_reapplication_proposed_under_INTERACT_ZONE03;full_turn_scheduler_and_multi_caster_provenance_not_resolved_here"
    }


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE02-ACCEPT",
        "source_proposal": ref("review-overlapping-zone-status-INTERACT-ZONE02-01.json"),
        "user_acceptance": "parfait",
        "accepted_as_displayed": {
            "zone_damage": "cumulative_own_damage_contributions",
            "status_chance": "independent_application_attempt_and_exposure_counter_per_zone",
            "status_instances": "one_per_kind;reapplication_replaces_power_and_restarts_duration",
            "same_turn_order": "oldest_zone_to_newest;latest_successful_application_counts",
            "failed_attempt": "does_not_replace_existing_status"
        },
        "shown_example": {"zone_damage20_plus30": 50,
                          "older_successful_Burn_power": 8, "newer_successful_Burn_power": 5,
                          "resulting_unique_Burn_power": 5, "duration": "restarted",
                          "values_are_fictional_before_defenses": True},
        "approval_boundary": "displayed_same_turn_zone_rules_only;old_zone_only_refresh_full_scheduler_multi_caster_credit_and_native_visuals_remain_separate",
        "runtime_integration": "not_integrated"
    })
    accepted = load("review-overlapping-zone-status-INTERACT-ZONE02-01.json")
    accepted.update(revision=2, status="displayed_zone_damage_independent_status_rolls_and_same_turn_order_explicitly_accepted;other_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed")
    accepted["previous_proposal"] = ref("review-overlapping-zone-status-INTERACT-ZONE02-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_details"] = accepted.pop("proposed_details")
    save(ACCEPTED, accepted)

    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE03", "revision": 1,
        "title": "Réapplication d'un statut par une zone ancienne",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("last-status-cast-power-acceptance-decisions.json"),
                         ref("review-persistent-power-snapshot-INTERACT-POWER01-02.json")],
        "scope": "only_one_older_zone_successfully_reapplies_same_status_after_a_newer_spell_application",
        "proposed_rule": "latest_successful_status_reapplication_uses_reapplying_source_stored_power_and_restarts_duration_even_if_that_zone_was_created_before_the_spell_that_applied_the_previous_status",
        "ordering_clarification": "zone_creation_order_sorts_multiple_zone_applications_during_same_turn;it_does_not_give_a_permanent_priority_to_a_newer_spell_against_future_successful_old_zone_refreshes",
        "successful_old_zone_refresh": "replace_previous_power_with_zone_launch_power_even_if_lower;restart_duration",
        "failed_old_zone_attempt": "retain_existing_status_power;no_refresh_from_failed_attempt;ordinary_remaining_duration_continues",
        "target_defenses": "existing_current_resistances_importance_and_control_conditions_still_apply",
        "zone_source_stats": "stored_zone_launch_power;no_recalculation_from_current_caster_level_attributes_or_equipment",
        "illustrative_example": {
            "old_zone_Burn_power_per_turn": 8,
            "later_direct_spell_applies_Burn_power_per_turn": 18,
            "following_turn_old_zone_only_successful_reapplication": True,
            "resulting_Burn_power_per_turn": 8,
            "duration": "restarted",
            "if_old_zone_attempt_fails": "previous_power18_continues_with_ordinary_remaining_duration",
            "values_are_fictional_before_target_defenses": True
        },
        "not_resolved_here": ["full_turn_scheduler_damage_status_casualty_order_and_multi_caster_credit", "non_actor_invocation_cleanup_dependencies_and_native_visuals"],
        "mechanic_approval": "pending_user_review",
        "visual_approval": "no_new_native_visual_proposed",
        "runtime_integration": "not_integrated"
    })

    old_ready = load("review-implementation-readiness-05.json")
    ready = deepcopy(old_ready)
    common(ready)
    ready.update(revision=6, status="displayed_overlapping_zone_rules_accepted;old_zone_only_refresh_proposed;other_source_content_and_native_cases_remaining")
    ready["previous_inventory"] = ref("review-implementation-readiness-05.json")
    ready["latest_user_decisions"] = ref(DECISION)
    ready["resolved_interactions"]["INTERACT-ZONE02"] = {
        "rule": "cumulative_zone_damage_independent_status_attempts_per_zone_and_oldest_to_newest_same_turn_order;latest_successful_same_status_application_sets_unique_power_and_restarts_duration",
        "source": ref(ACCEPTED), "unshown": "old_zone_only_refresh_full_scheduler_and_multi_caster_provenance"
    }
    for group in ready["before_affected_complete_system_integration"]:
        if group["id"] == "READY02":
            group["remaining"] = [
                "full_turn_scheduler_damage_status_expiration_and_casualty_order_after_accepted_ZONE02",
                "invocation_skill_rank_decrease_cleanup_now_accepted_for_only_its_elements;non_actor_classification_and_passive_capacity_dependency_pending",
                "command_action_budget_for_Destroy_and_Mule_start_stop_where_not_shown",
                "arrival_placement_queue_order_and_geometry_where_not_shown",
                "older_zone_only_status_refresh_proposed_under_INTERACT_ZONE03"
            ]
    ready["remaining_after_user_directions"] = [
        "old_zone_only_status_refresh_proposal_and_partial_non_numeric_receipt_consolidation",
        "remaining_source_provenance_and_full_turn_order",
        "classification_of_invoked_creations_and_passive_cross_skill_capacity_changes",
        "command_action_budget_and_arrival_order", "actual_loot_affix_families_ranges_and_weights",
        "class_start_profiles_and_equipment", "joint_UI_and_native_visual_design"
    ]
    ready["recommendation"] = "review_old_zone_refresh_and_remaining_turn_source_cases_then_finalize_class_starts_and_first_playable_mission;joint_UI_and_later_balance_timing_preserved"
    preserve(old_ready, ready)
    save("review-implementation-readiness-06.json", ready)

    old_model = load("review-attribute-model-63.json")
    model = deepcopy(old_model)
    common(model)
    add_override(model)
    model.update(review_id="STAT-MODEL61", status="overlapping_zone_status_rules_accepted;old_zone_only_refresh_proposed;other_numeric_attribute_and_effect_rules_preserved")
    pair(model, "previous_model", "review-attribute-model-63.json")
    pair(model, "implementation_readiness_audit", "review-implementation-readiness-06.json")
    preserve(old_model, model)
    save("review-attribute-model-64.json", model)
    for old_name, new_name in PAIRS[1:3]:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        add_override(data)
        data["batch_revision"] += 1
        pair(data, "previous_manifest", old_name)
        pair(data, "current_attribute_model", "review-attribute-model-64.json")
        pair(data, "implementation_readiness_audit", "review-implementation-readiness-06.json")
        pair(data, "latest_decisions", DECISION)
        pair(data, "latest_proposal", PROPOSAL)
        data["latest_proposal_scope"] = "older_zone_only_reapplication_of_existing_same_status;ZONE02_displayed_rules_now_accepted"
        data["next_review_kind"] = "old_zone_status_refresh_INTERACT_ZONE03"
        preserve(old, data)
        save(new_name, data)
    old_counts = load("remaining-individual-skill-review-counts-104.json")
    counts = deepcopy(old_counts)
    common(counts)
    pair(counts, "previous_count", "remaining-individual-skill-review-counts-104.json")
    pair(counts, "current_attribute_model", "review-attribute-model-64.json")
    pair(counts, "implementation_readiness_audit", "review-implementation-readiness-06.json")
    counts["latest_feedback"] = "user_accepts_ZONE02;old_zone_only_status_refresh_proposed"
    counts["next_review_kind"] = "old_zone_status_refresh_INTERACT_ZONE03"
    preserve(old_counts, counts)
    assert counts["remaining_first_pass_total"] == 0
    save("remaining-individual-skill-review-counts-105.json", counts)
    print(json.dumps({"accepted": "INTERACT_ZONE02", "next": "INTERACT_ZONE03_proposal",
                      "model": 64, "primary": 236, "complete": 211, "counts": 105, "runtime": "not_integrated"}))


def check():
    checked, errors = [], []
    def walk(value, context):
        if isinstance(value, dict):
            for key, expected in value.items():
                if key.endswith("_sha256") and key != "calculations_sha256":
                    stem = key[:-7]
                    source = value.get(stem, value.get(stem + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/")):
                            errors.append((context, "missing", source))
                            continue
                    checked.append((context, key))
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((context, key, source[:100]))
            for key, nested in value.items():
                walk(nested, context + "/" + key)
        elif isinstance(value, list):
            for index, nested in enumerate(value):
                walk(nested, context + "/" + str(index))
    for name in NEW_FILES:
        walk(load(name), name)
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value, (new_name, key)
        assert new["pending_overlapping_zone_status"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    example = load(PROPOSAL)["illustrative_example"]
    assert example["resulting_Burn_power_per_turn"] == example["old_zone_Burn_power_per_turn"] == 8
    assert load("remaining-individual-skill-review-counts-105.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-80:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("overlapping-zone-and-old-refresh-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE02-ZONE03-CHECK01",
        "scope": "design_references_and_preserved_rules;not_engine_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "other_accepted_fields_catalogues_rank_curves_and_previous_overrides": "preserved",
        "ZONE02": "accepted_as_displayed", "ZONE03": "proposal_only",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "zones": "accepted", "old_zone_refresh": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
