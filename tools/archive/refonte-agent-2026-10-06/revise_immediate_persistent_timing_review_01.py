"""Design only: record immediate persistent-effect direction, revise timer proposal."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-timer-accounting-INTERACT-TIME01-01.json"
DECISION = "persistent-effect-immediate-start-user-decisions.json"
PROPOSAL = "review-timer-accounting-INTERACT-TIME01-02.json"
PAIRS = [("review-implementation-readiness-18.json", "review-implementation-readiness-19.json"),
         ("review-attribute-model-76.json", "review-attribute-model-77.json"),
         ("review-primary-catalogue-248.json", "review-primary-catalogue-249.json"),
         ("review-complete-skill-design-223.json", "review-complete-skill-design-224.json"),
         ("remaining-individual-skill-review-counts-117.json", "remaining-individual-skill-review-counts-118.json")]
NEW_FILES = [DECISION, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Les effets persistants commencent dès leur application, comme vous le demandez.

- Poison, Brûlure et Saignement : premiers dégâts dès que l'effet est appliqué.
- Zone persistante : première application dès sa création.
- Pour une mine ou une attaque à déclenchement différé, l'effet commence à l'explosion ou à l'impact.

Pour le décompte, je propose que cette première application compte dans le total annoncé, sans seconde application automatique pendant le même tour.

Exemple : un poison prévu pour 3 applications infligerait ses dégâts immédiatement, puis au tour suivant, puis au deuxième tour suivant. Cela conserve son total de dégâts et avance sa première application.

Ce décompte reste à valider. Les nouveaux lancements sur un statut déjà présent et l'ordre complet des effets du tour seront précisés séparément."""


def override(data):
    data.setdefault("active_rule_overrides", {})["persistent_effect_immediate_start"] = {
        **ref(DECISION),
        "current_rule": "newly_applied_persistent_effect_first_application_immediate_on_actual_application_or_zone_creation;no_default_one_turn_delay",
        "delayed_trigger": "explicit_preparation_arming_or_delayed_impact_unchanged;dependent_persistent_effect_begins_when_delayed_trigger_actually_applies_it_not_before",
        "precedence": "supersedes_historical_next_full_turn_first_tick_of_Burn_and_one_turn_after_explosion_first_tick_of_mine_residue;does_not_change_effect_power_or_other_accepted_conditions",
        "pending": "application_count_and_same_turn_automatic_repeat_proposed_under_TIME01_revision2;existing_status_refresh_cadence_and_complete_scheduler_unresolved;modified_player_skill_copy_and_native_feedback_require_review"
    }


def main():
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME01-DIRECTION01",
        "source": "direct_user_instruction_during_timer_review",
        "user_messages": ["en vérité je suis pas fan du fait qu'un effet persistant commence au tour d'apres.",
                          "je préfèrerais que ca commence directement des l'utilisation de la compétence."],
        "accepted_rule": "persistent_effects_start_immediately_when_skill_actually_applies_effect;remove_default_next_turn_delay_for_first_application",
        "interpretation_boundary": "effect_cannot_apply_before_existing_explicit_delayed_trigger;mine_arming_and_delayed_attacks_remain;no_extra_damage_number_or_complete_tick_refresh_rule_approved",
        "replaced_pending_direction": ref(SOURCE),
        "approval": "explicit_user_requested_first_application_direction",
        "runtime_integration": "not_integrated"
    })
    proposal = deepcopy(load(SOURCE))
    proposal.update(revision=2, status="immediate_persistent_start_requested_by_user;application_count_proposed;buff_and_cooldown_examples_deferred;not_runtime_integrated")
    proposal["previous_proposal"] = ref(SOURCE)
    proposal["immediate_start_direction"] = ref(DECISION)
    proposal["scope"] = "first_application_and_number_of_applications_for_new_persistent_effects;ordinary_buff_and_cooldown_accounting_examples_from_revision1_not_presented_or_accepted"
    proposal["deferred_buff_and_cooldown_draft"] = {
        "source": ref(SOURCE), "approval": "not_presented_as_final_proposal_and_not_accepted",
        "note": "user_steering_addresses_persistent_effect_start;do_not_activate_revision1_buff_or_cooldown_convention"
    }
    proposal["accepted_direction"] = {
        "initial_actor_status": "newly_applied_Poison_Burn_and_Bleed_have_immediate_first_damage",
        "persistent_ground_zone": "first_effect_from_actual_zone_creation_instead_of_default_next_turn_delay",
        "existing_delayed_trigger": "mine_arming_preparation_and_delayed_impact_kept;persistent_effect_starts_at_actual_explosion_or_impact_not_at_preparation_start"
    }
    proposal["proposed_rule"] = {
        "first_application_count": "immediate_first_application_counts_as_first_of_existing_announced_number;does_not_add_a_bonus_damage_tick",
        "following_applications": "remaining_applications_once_per_subsequent_reference_game_turn_while_effect_exists_under_normal_conditions",
        "same_turn_automatic_pass": "same_new_effect_does_not_get_second_automatic_periodic_application_from_end_of_its_creation_turn;independent_overlapping_zones_keep_own_applications",
        "illustration": "three_announced_Poison_applications_at_application_turn_then_next_turn_then_second_following_turn;not_a_new_skill_duration_or_damage_value",
        "preserved": "accepted_damage_power_target_defenses_last_successful_status_power_full_duration_refresh_Gel_Burn_replacement_overlap_and_rank_loss_cleanup;no_new_player_self_damage",
        "scope_boundary": "already_active_status_reapplication_immediate_damage_vs_refresh_and_exact_phase_order_pending;no_buff_cooldown_familiar_lifetime_or_channel_timing_change_approved_here"
    }
    proposal["shown_examples"] = {"values_are_illustrative_not_new_skill_parameters": True,
                                  "Poison_three_applications": {"relative_game_turns": [0, 1, 2], "total_applications": 3,
                                                                "first_turn_automatic_extra_application": False}}
    proposal["shown_copy_markdown"] = COPY
    proposal["shown_copy_sha256"] = hashlib.sha256(COPY.encode("utf-8")).hexdigest()
    proposal["not_resolved_here"] = ["ordinary_buff_and_cooldown_timer_accounting_from_unshown_revision1_draft",
                                    "already_active_periodic_status_reapplication_vs_immediate_damage_and_scheduled_tick",
                                    "complete_phase_order_and_exact_duration_expiry_of_persistent_zones",
                                    "modified_individual_skill_player_copy_and_native_feedback"]
    proposal["mechanic_approval"] = "immediate_first_application_direction_explicitly_requested;application_count_and_same_turn_automatic_repeat_pending_user_review"
    proposal["text_approval"] = "new_design_explanation_pending;individual_skill_copy_not_newly_approved"
    save(PROPOSAL, proposal)
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data["date"] = "2026-10-09"
        data["previous_pending_timer_accounting"] = deepcopy(data["pending_timer_accounting"])
        data["pending_timer_accounting"] = ref(PROPOSAL)
        data["accepted_persistent_effect_immediate_start"] = ref(DECISION)
        data["runtime_integration"] = "not_integrated"
        if old_name.startswith("review-implementation"):
            data.update(revision=19, status="arrival_priority_accepted;immediate_persistent_effect_start_requested;application_count_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-TIME01-DIRECTION01"] = {
                "rule": "persistent_first_application_immediate_on_actual_application_or_zone_creation;existing_delayed_trigger_not_bypassed",
                "source": ref(DECISION), "unshown": "number_of_applications_same_turn_repeat_refresh_cadence_and_complete_phase_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "ordinary_buff_cooldown_and_actor_periodic_timer_accounting_proposed_under_INTERACT_TIME01"]
                    group["remaining"].append("immediate_persistent_application_count_and_same_turn_automatic_repeat_proposed_under_INTERACT_TIME01_revision2;buff_cooldown_draft_not_accepted")
            data["recommendation"] = "review_immediate_persistent_application_count_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL74", status="persistent_effect_immediate_start_requested;application_count_proposed;buff_cooldown_convention_unaccepted;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-19.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-77.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-19.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "immediate_persistent_first_application_direction_and_pending_total_application_count;buff_and_cooldown_draft_deferred"
            data["next_review_kind"] = "immediate_persistent_application_count_INTERACT_TIME01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-77.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-19.json")
            data["latest_feedback"] = "user_requests_immediate_persistent_start;total_application_count_proposed"
            data["next_review_kind"] = "immediate_persistent_application_count_INTERACT_TIME01"
        preserve(old, data)
        save(new_name, data)
    checked, errors = [], []
    def walk(value):
        if isinstance(value, dict):
            for key, expected in value.items():
                if key.endswith("_sha256") and key != "calculations_sha256":
                    source = value.get(key[:-7], value.get(key[:-7] + "_markdown"))
                    if not isinstance(source, str):
                        continue
                    content = source
                    if len(source) < 240 and "\n" not in source:
                        path = ROOT / source
                        if path.is_file():
                            content = path.read_text(encoding="utf-8")
                        elif source.startswith(("artifacts/", "docs/")):
                            errors.append(("missing", source))
                            continue
                    checked.append(key)
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((key, source[:100]))
            for nested in value.values():
                walk(nested)
        elif isinstance(value, list):
            for nested in value:
                walk(nested)
    for name in NEW_FILES:
        walk(load(name))
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_familiar_arrival_priority"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load("remaining-individual-skill-review-counts-118.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    save("immediate-persistent-timing-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-TIME01-DIRECTION01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "immediate_start": "explicit_user_direction", "application_count": "proposal_only",
        "buff_cooldown_convention": "unshown_draft_not_accepted", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "immediate_start": "user_requested", "application_count": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
