"""Design receipts for accepted zone refresh and proposed creation cleanup."""
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
DECISION = "old-zone-status-refresh-acceptance-decisions.json"
ACCEPTED = "review-old-zone-status-refresh-INTERACT-ZONE03-02.json"
PROPOSAL = "review-invoked-creation-scope-INTERACT-RANK02-01.json"
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL, "review-implementation-readiness-07.json",
             "review-attribute-model-65.json", "review-primary-catalogue-237.json",
             "review-complete-skill-design-212.json", "remaining-individual-skill-review-counts-106.json"]
PAIRS = [("review-attribute-model-64.json", "review-attribute-model-65.json"),
         ("review-primary-catalogue-236.json", "review-primary-catalogue-237.json"),
         ("review-complete-skill-design-211.json", "review-complete-skill-design-212.json"),
         ("remaining-individual-skill-review-counts-105.json", "remaining-individual-skill-review-counts-106.json"),
         ("review-implementation-readiness-06.json", "review-implementation-readiness-07.json")]


def common(data):
    data["date"] = "2026-10-09"
    if data.get("pending_old_zone_status_refresh"):
        data["previous_pending_old_zone_status_refresh"] = deepcopy(data["pending_old_zone_status_refresh"])
    data["pending_old_zone_status_refresh"] = None
    data["accepted_old_zone_status_refresh"] = ref(ACCEPTED)
    data["latest_old_zone_refresh_acceptance"] = ref(DECISION)
    data["pending_invoked_creation_scope"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def add_override(data):
    data.setdefault("active_rule_overrides", {})["old_zone_status_reapplication"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "current_rule": "future_successful_status_reapplication_from_old_zone_replaces_current_status_with_that_zone_stored_launch_power_even_if_lower_and_restarts_duration",
        "ordering": "zone_creation_order_orders_multiple_zone_applications_within_one_turn;no_permanent_priority_for_newer_spell_over_future_old_zone_reapplications",
        "failed_attempt": "previous_status_keeps_power_and_ordinary_remaining_duration;no_refresh_from_failed_attempt",
        "preserved": "target_defenses_importance_conditions_unique_status_and_zone_launch_power_snapshot",
        "precedence": "clarifies_last_status_cast_power_for_zone_refresh_events;does_not_change_initial_cast_power_or_same_turn_ZONE02_order"
    }


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE03-ACCEPT",
        "source_proposal": ref("review-old-zone-status-refresh-INTERACT-ZONE03-01.json"),
        "user_clarification_request": "je suis pas sur de comprendre, peux tu etre plus clair ?",
        "clarification_presented": "Une zone encore active peut remplacer une Brûlure appliquée par un sort plus récent, même par une Brûlure plus faible. Une zone créée avec8 applique sa Brûlure, un autre sort applique18, puis la zone réapplique avec succès au tour suivant :8 remplace18 et la durée est renouvelée. Chaque réapplication réussie remplace le statut existant, avec la puissance de la zone calculée à sa création.",
        "user_acceptance": "d'accord ca me va .",
        "approved_rule": "latest_successful_reapplication_uses_reapplying_zone_stored_power_even_if_zone_is_older_than_previous_spell_source;duration_restarts",
        "failed_attempt": "existing_power_and_ordinary_remaining_duration_continue_without_refresh",
        "shown_example": {"zone_power": 8, "later_spell_power": 18, "after_successful_old_zone_reapplication": 8,
                          "duration": "renewed", "values_are_fictional_before_resistances": True},
        "approval_boundary": "old_zone_reapplication_case_as_clarified;full_turn_scheduler_multi_caster_credit_and_native_visuals_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = load("review-old-zone-status-refresh-INTERACT-ZONE03-01.json")
    accepted.update(revision=2, status="old_zone_reapplication_explicitly_accepted_after_clarification;remaining_scheduler_and_source_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_clarified")
    accepted["previous_proposal"] = ref("review-old-zone-status-refresh-INTERACT-ZONE03-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)

    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK02", "revision": 1,
        "title": "Quelles créations comptent comme éléments invoqués ?",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("interaction-and-priority-user-decisions.json"),
                         ref("review-equipment-grant-loss-effects-GEAR-SK02-03.json"),
                         ref("review-persistent-power-snapshot-INTERACT-POWER01-02.json")],
        "scope": "classification_for_accepted_same_player_per_creator_skill_cleanup_on_any_effective_rank_decrease",
        "already_accepted": "rank_decrease_removes_only_elements_invoked_by_affected_skill;real_familiar_items_drop_at_actor_cell_or_player_cell_if_waiting;unrelated_invocations_retained",
        "proposed_classification": {
            "included": ["drone_actors", "raised_allied_actors", "placed_mines", "spell_ground_zones", "ice_wall_and_bloody_ritual_segments", "persistent_map_decoys"],
            "creator_identity": "active_skill_that_created_actor_or_persistent_map_element_of_this_player;supporting_passive_bonus_is_not_its_creator",
            "ordinary_noninvoked_effects": "attached_buffs_debuffs_and_player_transformations_follow_existing_learned_effect_persistence_and_final_equipment_grant_loss_rules;not_reclassified_as_invoked_map_creations",
            "damage_passive_or_drone_armor_passive_rank_loss": "does_not_destroy_drones_created_by_other_skills;current_applicable_passive_modifiers_follow_existing_actor_stat_rules",
            "zone_derived_statuses": "zone_removal_does_not_by_itself_reclassify_attached_status_as_ground_creation;existing_status_persistence_and_final_access_loss_rules_remain_applicable"
        },
        "illustrative_examples": [
            {"event": "lose_equipment_bonus_rank_of_FR03_ice_wall", "result": "remove_existing_own_FR03_wall_creations_only"},
            {"event": "lose_rank_of_DP02_Blindage_ameliore", "result": "deployed_drones_are_not_removed;their_applicable_passive_protection_is_updated"}
        ],
        "not_resolved_here": [
            "secondary_capacity_or_variant_passive_rank_loss_affecting_other_skill_creations",
            "predeath_resurrection_mark_cleanup_and_birth_parameter_sampling",
            "classification_of_delayed_cast_jobs_or_nonpersistent_projectile_animations",
            "exact_cleanup_event_order_multi_caster_provenance_and_native_feedback"
        ],
        "mechanic_approval": "pending_user_review",
        "visual_approval": "no_new_native_art_or_animation_proposed",
        "runtime_integration": "not_integrated"
    })

    old_ready = load("review-implementation-readiness-06.json")
    ready = deepcopy(old_ready)
    common(ready)
    ready.update(revision=7, status="old_zone_status_reapplication_accepted;invoked_creation_classification_proposed;other_content_and_native_review_remaining")
    ready["previous_inventory"] = ref("review-implementation-readiness-06.json")
    ready["latest_user_decisions"] = ref(DECISION)
    ready["resolved_interactions"]["INTERACT-ZONE03"] = {
        "rule": "older_zone_can_replace_newer_spell_status_on_future_successful_reapplication_using_own_stored_launch_power;restart_duration;failed_attempt_does_not_refresh",
        "source": ref(ACCEPTED), "unshown": "full_turn_scheduler_and_multi_caster_credit"
    }
    for group in ready["before_affected_complete_system_integration"]:
        if group["id"] == "READY02":
            group["remaining"] = [s for s in group["remaining"] if s != "older_zone_only_status_refresh_proposed_under_INTERACT_ZONE03"]
            group["remaining"].append("invoked_persistent_creation_classification_proposed_under_INTERACT_RANK02")
    ready["remaining_after_user_directions"] = [
        "invoked_creation_classification_proposal_and_partial_non_numeric_receipt_consolidation",
        "remaining_source_provenance_and_full_turn_order",
        "passive_cross_skill_capacity_changes_and_resurrection_mark_birth_cases",
        "command_action_budget_and_arrival_order", "actual_loot_affix_families_ranges_and_weights",
        "class_start_profiles_and_equipment", "joint_UI_and_native_visual_design"
    ]
    ready["recommendation"] = "review_persistent_creation_cleanup_classification_and_remaining_source_turn_cases_then_finalize_class_starts_and_first_playable_mission;joint_UI_and_later_balance_timing_preserved"
    preserve(old_ready, ready)
    save("review-implementation-readiness-07.json", ready)

    old_model = load("review-attribute-model-64.json")
    model = deepcopy(old_model)
    common(model)
    add_override(model)
    model.update(review_id="STAT-MODEL62", status="old_zone_refresh_accepted;invoked_creation_scope_proposed;other_numeric_attribute_and_effect_rules_preserved")
    pair(model, "previous_model", "review-attribute-model-64.json")
    pair(model, "implementation_readiness_audit", "review-implementation-readiness-07.json")
    preserve(old_model, model)
    save("review-attribute-model-65.json", model)
    for old_name, new_name in PAIRS[1:3]:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        add_override(data)
        data["batch_revision"] += 1
        pair(data, "previous_manifest", old_name)
        pair(data, "current_attribute_model", "review-attribute-model-65.json")
        pair(data, "implementation_readiness_audit", "review-implementation-readiness-07.json")
        pair(data, "latest_decisions", DECISION)
        pair(data, "latest_proposal", PROPOSAL)
        data["latest_proposal_scope"] = "classification_of_invoked_actors_and_persistent_map_creations_only;old_zone_refresh_now_accepted"
        data["next_review_kind"] = "invoked_creation_scope_INTERACT_RANK02"
        preserve(old, data)
        save(new_name, data)
    old_counts = load("remaining-individual-skill-review-counts-105.json")
    counts = deepcopy(old_counts)
    common(counts)
    pair(counts, "previous_count", "remaining-individual-skill-review-counts-105.json")
    pair(counts, "current_attribute_model", "review-attribute-model-65.json")
    pair(counts, "implementation_readiness_audit", "review-implementation-readiness-07.json")
    counts["latest_feedback"] = "user_accepts_clarified_ZONE03;invoked_creation_classification_proposed"
    counts["next_review_kind"] = "invoked_creation_scope_INTERACT_RANK02"
    preserve(old_counts, counts)
    assert counts["remaining_first_pass_total"] == 0
    save("remaining-individual-skill-review-counts-106.json", counts)
    print(json.dumps({"accepted": "INTERACT_ZONE03", "next": "INTERACT_RANK02_proposal",
                      "model": 65, "primary": 237, "complete": 212, "counts": 106, "runtime": "not_integrated"}))


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
        assert new["pending_old_zone_status_refresh"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_clarified"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-106.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-80:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("old-zone-and-creation-scope-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE03-RANK02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_engine_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_numeric_curves_and_active_overrides": "preserved",
        "ZONE03": "explicitly_accepted_after_clarification", "RANK02": "proposal_only",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "old_zone_refresh": "accepted", "creation_scope": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
