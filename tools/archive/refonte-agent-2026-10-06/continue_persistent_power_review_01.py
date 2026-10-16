"""Design review records only: accepted power snapshot, proposed zone order."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import sys

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
DECISION = "persistent-effect-power-acceptance-decisions.json"
ACCEPTED = "review-persistent-power-snapshot-INTERACT-POWER01-02.json"
PROPOSAL = "review-overlapping-zone-status-INTERACT-ZONE02-01.json"
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL, "review-implementation-readiness-05.json",
             "review-attribute-model-63.json", "review-primary-catalogue-235.json",
             "review-complete-skill-design-210.json", "remaining-individual-skill-review-counts-104.json"]


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


def ref(name):
    source = (BASE / name).read_text(encoding="utf-8")
    return {"reference": PREFIX + name,
            "reference_sha256": hashlib.sha256(source.encode("utf-8")).hexdigest()}


def pair(data, key, name):
    source = ref(name)
    data[key] = source["reference"]
    data[key + "_sha256"] = source["reference_sha256"]


def save(name, data):
    path = BASE / name
    assert not path.exists(), name
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n",
                    encoding="utf-8", newline="\n")


def preserve(old, new):
    for key, value in old.items():
        if key.startswith("accepted_") or key in {"branches", "blocks", "removed_skills", "removed_branches", "purchase_rules", "candidate_count", "learned_rank_cap", "cost_per_rank"}:
            assert new.get(key) == value, key


def common(data):
    data["date"] = "2026-10-09"
    if data.get("pending_persistent_power_snapshot"):
        data["previous_pending_persistent_power_snapshot"] = deepcopy(data["pending_persistent_power_snapshot"])
    data["pending_persistent_power_snapshot"] = None
    data["accepted_persistent_effect_power"] = ref(ACCEPTED)
    data["latest_persistent_power_acceptance"] = ref(DECISION)
    data["pending_overlapping_zone_status"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def add_override(data):
    values = data.setdefault("active_rule_overrides", {})
    values["persistent_effect_caster_power"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "current_rule": "applied_skill_status_and_spell_zone_keep_caster_power_calculated_at_launch;caster_level_attribute_or_equipment_changes_affect_future_casts;new_successful_reapplication_uses_new_cast_power_and_restarts_duration",
        "target_defenses": "current_eligible_target_defenses_at_each_impact_or_periodic_damage_tick",
        "preserved": "accepted_familiar_actor_stat_updates_and_required_effect_loss_or_invocation_rank_decrease_cleanup",
        "precedence": "resolves_historical_pending_existing_effect_level_update_labels;does_not_change_actor_stats_or_cleanup_rules",
        "pending_cases": "simultaneous_zone_sources_and_predeath_mark_birth_sampling_outside_displayed_scope"
    }
    values["current_level_nonweapon_power"]["pending_cases"] = "persistent_skill_status_and_zone_caster_power_snapshot_now_accepted;overlapping_zone_order_proposed_under_INTERACT_ZONE02;native_review_and_final_balance_pending"


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-POWER01-ACCEPT",
        "source_proposal": ref("review-persistent-power-snapshot-INTERACT-POWER01-01.json"),
        "user_acceptance": "oui :)",
        "displayed_proposal": "Un statut ou une zone déjà active conserve la puissance calculée au lancement. Gagner un niveau ou modifier ses attributs ou son équipement affecte les prochains lancements. Réappliquer le statut utilise les nouvelles valeurs et redémarre la durée. Les résistances de la cible restent prises en compte à chaque dégât. Les familiers gardent leurs règles d'évolution déjà validées.",
        "approved_rules": {
            "existing_caster_power": "retained_from_launch_for_applied_skill_statuses_and_spell_ground_zones",
            "caster_level_attribute_equipment_change": "changes_future_casts;does_not_retroactively_modify_existing_effect_power",
            "reapplication": "new_cast_power_replaces_previous_and_duration_restarts_under_INTERACT_STATUS02",
            "target_defenses": "current_eligible_target_defenses_still_applied_to_each_damage_event",
            "familiars": "existing_accepted_actor_statistics_updates_unchanged"
        },
        "shown_example": {"existing_Burn_before_resistance_damage_per_turn": 12,
                          "after_caster_level_gain": 12,
                          "after_new_successful_cast_with_power18": 18,
                          "duration_after_reapplication": "restarted",
                          "values_are_illustrative_not_actual_Braise_entries": True},
        "approval_boundary": "shown_power_snapshot_and_reapplication_rule_only;simultaneous_source_order_mine_dependency_cleanup_and_future_raised_birth_sampling_not_newly_approved",
        "prior_required_cleanup": "accepted_effect_loss_and_invocation_rank_decrease_cleanup_remain_applicable",
        "runtime_integration": "not_integrated"
    })
    old = load("review-persistent-power-snapshot-INTERACT-POWER01-01.json")
    accepted = deepcopy(old)
    accepted.update(revision=2, status="displayed_persistent_caster_power_rule_explicitly_accepted;other_source_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="displayed_design_explanation_accepted;no_new_RP_or_skill_name")
    accepted["previous_proposal"] = ref("review-persistent-power-snapshot-INTERACT-POWER01-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)

    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE02", "revision": 1,
        "title": "Zones superposées : dégâts et statuts",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("interaction-and-priority-user-decisions.json"),
                         ref("last-status-cast-power-acceptance-decisions.json"), ref(ACCEPTED)],
        "scope": "multiple_persistent_spell_ground_zones_affecting_same_target_during_same_turn",
        "already_accepted": [
            "overlapping_zone_effects_apply_cumulatively",
            "same_status_reapplication_restarts_duration_and_latest_cast_sets_power_even_if_lower",
            "each_zone_keeps_its_launch_caster_power",
            "exposure_chance_increases_with_consecutive_turns_inside_zone_and_resets_on_exit",
            "Gel_Burn_successful_mutual_replacement_and_current_defense_and_importance_rules"
        ],
        "proposed_details": {
            "zone_damage": "each_zone_deals_its_own_eligible_damage;all_zone_damage_contributions_apply;existing_per_impact_rounding_and_target_defenses_retained",
            "status_chance": "each_zone_attempts_its_own_status_application_with_its_own_consecutive_exposure_counter;do_not_sum_percentage_chances_into_one_roll",
            "same_status_instance": "one_current_status_per_kind_on_target;multiple_same_kind_zone_applications_refresh_and_replace_power_instead_of_adding_periodic_status_power",
            "same_turn_application_order": "resolve_zone_status_applications_in_cast_creation_order_oldest_to_newest;latest_successful_application_in_that_turn_sets_status_power_and_restarts_duration",
            "newer_attempt_fails": "unsuccessful_status_application_does_not_replace_or_refresh_status;other_zone_damage_still_applies",
            "exposure_on_exit": "reset_counter_only_for_zone_that_target_exits;other_zone_exposure_continues_while_target_remains_inside",
            "opposed_statuses": "preserve_successful_Gel_Burn_mutual_removal_under_same_application_order"
        },
        "illustrative_example": {
            "older_fire_zone_damage": 20, "newer_fire_zone_damage": 30,
            "total_zone_damage_before_defenses": 50,
            "older_successful_Burn_power_per_turn": 8,
            "newer_successful_Burn_power_per_turn": 5,
            "resulting_unique_Burn_power_per_turn": 5,
            "not_added_Burn_power": 13,
            "both_status_applications_successful_same_turn": True,
            "numbers_are_fictional_not_skill_balance_entries": True
        },
        "not_resolved_here": [
            "old_zone_only_refresh_when_a_newer_existing_status_is_not_reapplied_this_turn",
            "full_turn_scheduler_damage_status_and_casualty_order",
            "multi_caster_kill_credit_and_effect_loss_provenance",
            "native_status_and_multi_zone_preview_rendering"
        ],
        "mechanic_approval": "pending_user_review",
        "visual_approval": "no_new_native_visual_proposed",
        "runtime_integration": "not_integrated"
    })

    old_ready = load("review-implementation-readiness-04.json")
    ready = deepcopy(old_ready)
    common(ready)
    ready.update(revision=5, status="persistent_caster_power_snapshot_accepted;overlapping_zone_status_order_proposed;other_content_and_native_review_remaining")
    ready["previous_inventory"] = ref("review-implementation-readiness-04.json")
    ready["latest_user_decisions"] = ref(DECISION)
    ready["resolved_interactions"]["INTERACT-POWER01"] = {
        "rule": "existing_skill_status_and_zone_caster_power_stays_as_launched;current_target_defenses_and_accepted_familiar_updates_preserved;new_cast_reapplication_uses_new_values",
        "source": ref(ACCEPTED), "unshown": "simultaneous_sources_and_future_raised_mark_birth_sampling"
    }
    for group in ready["before_affected_complete_system_integration"]:
        if group["id"] == "READY02":
            group["remaining"] = [s for s in group["remaining"] if s != "existing_zone_or_periodic_effect_update_after_caster_level_change_where_unshown"]
            group["remaining"].append("overlapping_zone_status_order_and_independent_exposure_counters_proposed_under_INTERACT_ZONE02")
    ready["remaining_after_user_directions"] = [
        "overlapping_zone_status_order_proposal_and_partial_non_numeric_receipt_consolidation",
        "remaining_source_provenance_full_turn_order_and_old_zone_refresh_cases",
        "classification_of_invoked_creations_and_passive_cross_skill_capacity_changes",
        "command_action_budget_and_arrival_order",
        "actual_loot_affix_families_ranges_and_weights",
        "class_start_profiles_and_equipment",
        "joint_UI_and_native_visual_design"
    ]
    ready["recommendation"] = "review_overlapping_zone_status_and_remaining_turn_source_cases_then_finalize_class_starts_and_first_playable_mission;joint_UI_and_later_balance_timing_preserved"
    preserve(old_ready, ready)
    save("review-implementation-readiness-05.json", ready)

    old_model = load("review-attribute-model-62.json")
    model = deepcopy(old_model)
    common(model)
    add_override(model)
    model.update(review_id="STAT-MODEL60", status="persistent_caster_power_snapshot_accepted;overlapping_zone_details_proposed;existing_numeric_curves_attributes_and_other_effect_rules_preserved")
    pair(model, "previous_model", "review-attribute-model-62.json")
    pair(model, "implementation_readiness_audit", "review-implementation-readiness-05.json")
    preserve(old_model, model)
    save("review-attribute-model-63.json", model)
    for old_name, new_name in [
        ("review-primary-catalogue-234.json", "review-primary-catalogue-235.json"),
        ("review-complete-skill-design-209.json", "review-complete-skill-design-210.json")
    ]:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        add_override(data)
        data["batch_revision"] += 1
        pair(data, "previous_manifest", old_name)
        pair(data, "current_attribute_model", "review-attribute-model-63.json")
        pair(data, "implementation_readiness_audit", "review-implementation-readiness-05.json")
        pair(data, "latest_decisions", DECISION)
        pair(data, "latest_proposal", PROPOSAL)
        data["latest_proposal_scope"] = "overlapping_zone_damage_and_same_turn_status_application_details_only;persistent_caster_power_snapshot_now_accepted"
        data["next_review_kind"] = "overlapping_zone_status_INTERACT_ZONE02"
        preserve(old, data)
        save(new_name, data)
    old_counts = load("remaining-individual-skill-review-counts-103.json")
    counts = deepcopy(old_counts)
    common(counts)
    pair(counts, "previous_count", "remaining-individual-skill-review-counts-103.json")
    pair(counts, "current_attribute_model", "review-attribute-model-63.json")
    pair(counts, "implementation_readiness_audit", "review-implementation-readiness-05.json")
    counts["latest_feedback"] = "user_accepts_persistent_caster_power_snapshot;overlapping_zone_status_details_proposed"
    counts["next_review_kind"] = "overlapping_zone_status_INTERACT_ZONE02"
    preserve(old_counts, counts)
    assert counts["remaining_first_pass_total"] == 0
    save("remaining-individual-skill-review-counts-104.json", counts)
    print(json.dumps({"accepted": "INTERACT_POWER01", "next": "INTERACT_ZONE02_proposal",
                      "model": 63, "primary": 235, "complete": 210, "counts": 104, "runtime": "not_integrated"}))


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
    for old_name, new_name in [
        ("review-attribute-model-62.json", "review-attribute-model-63.json"),
        ("review-primary-catalogue-234.json", "review-primary-catalogue-235.json"),
        ("review-complete-skill-design-209.json", "review-complete-skill-design-210.json"),
        ("remaining-individual-skill-review-counts-103.json", "remaining-individual-skill-review-counts-104.json"),
        ("review-implementation-readiness-04.json", "review-implementation-readiness-05.json")
    ]:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            if key != "current_level_nonweapon_power":
                assert new["active_rule_overrides"][key] == value, (new_name, key)
        assert new["pending_persistent_power_snapshot"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    proposal = load(PROPOSAL)
    assert proposal["mechanic_approval"] == "pending_user_review"
    example = proposal["illustrative_example"]
    assert example["total_zone_damage_before_defenses"] == example["older_fire_zone_damage"] + example["newer_fire_zone_damage"] == 50
    assert example["resulting_unique_Burn_power_per_turn"] == example["newer_successful_Burn_power_per_turn"] == 5
    assert load("remaining-individual-skill-review-counts-104.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-85:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("persistent-power-and-zone-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-POWER01-ZONE02-CHECK01",
        "scope": "design_reference_integrity_and_illustrative_arithmetic;not_engine_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_rules_catalogues_rank_curves_and_other_active_overrides": "preserved",
        "power_snapshot": "explicitly_accepted_as_displayed",
        "zone_order_and_independent_status_rolls": "proposal_only;not_approved_or_runtime_integrated",
        "zone_example_total_predefense_damage": 50, "zone_example_latest_successful_Burn_power": 5,
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "snapshot": "accepted", "zones": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
