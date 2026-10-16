"""Design only: accept immediate application counts, propose status reapplication."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-timer-accounting-INTERACT-TIME01-02.json"
DECISION = "persistent-immediate-application-count-acceptance-decisions.json"
ACCEPTED = "review-timer-accounting-INTERACT-TIME01-03.json"
PROPOSAL = "review-persistent-status-reapplication-INTERACT-TIME02-01.json"
PAIRS = [("review-implementation-readiness-19.json", "review-implementation-readiness-20.json"),
         ("review-attribute-model-77.json", "review-attribute-model-78.json"),
         ("review-primary-catalogue-249.json", "review-primary-catalogue-250.json"),
         ("review-complete-skill-design-224.json", "review-complete-skill-design-225.json"),
         ("remaining-individual-skill-review-counts-118.json", "remaining-individual-skill-review-counts-119.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-TIME02 · Réappliquer un effet persistant — proposition

- Une nouvelle attaque ou un nouveau lancement qui réussit à réappliquer Poison, Brûlure ou Saignement produit les dégâts du statut immédiatement, avec la nouvelle puissance. Sa durée repart entièrement.
- Cette application compte comme la première de la durée renouvelée. Le même statut ne reçoit pas ensuite une seconde application automatique pendant ce tour.
- Une zone déjà présente qui entretient automatiquement ce statut renouvelle sa durée et sa puissance selon les règles validées, sans ajouter un dégât immédiat à chaque rafraîchissement. Son application périodique normale reste conservée.

Les dégâts propres à chaque zone continuent de se cumuler. Une application échouée ne modifie pas le statut présent.

Exemple fictif : un poison inflige 6 dégâts par application. Une nouvelle attaque le réapplique avec une puissance de 10 : elle provoque 10 dégâts de poison immédiatement, puis l'effet poursuit ses applications avec cette puissance.

Ces précisions restent à valider."""


def add_override(data):
    data.setdefault("active_rule_overrides", {})["immediate_persistent_application_count"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "current_rule": "immediate_first_application_counts_in_existing_announced_total;remaining_applications_on_subsequent_reference_turns;no_second_automatic_application_of_same_new_effect_in_creation_turn",
        "example": "three_applications_at_relative_turns0_1_2;not_new_damage_or_duration_parameters",
        "preserved": "independent_zone_applications_cumulate;current_target_defenses_launch_power_and_required_cleanup;no_buff_cooldown_familiar_lifetime_or_channel_timing_change",
        "pending": "existing_status_reapplication_proposed_under_INTERACT_TIME02;complete_phase_order_and_modified_individual_player_copy"
    }


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait on continue",
        "shown_explanation": "Un effet persistant commence dès son application : premiers dégâts de Poison, Brûlure et Saignement immédiatement ; première application d'une zone persistante dès sa création ; à l'explosion ou à l'impact pour une mine ou attaque différée. Cette application compte dans le total annoncé, sans deuxième application automatique pendant le même tour. Un effet prévu pour trois applications agit immédiatement puis pendant les deux tours suivants, en conservant son total de dégâts.",
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "new_persistent_effect_immediate_first_application_total_count_and_no_same_turn_second_automatic_application;existing_status_refresh_and_buff_cooldown_draft_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=3, status="immediate_first_application_count_and_no_same_turn_automatic_repeat_accepted;existing_status_refresh_and_full_phase_order_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME02", "revision": 1,
        "title": "Effet persistant déjà présent : nouvelle attaque et entretien de zone",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("last-status-cast-power-acceptance-decisions.json"),
                         ref("review-overlapping-zone-status-INTERACT-ZONE02-02.json"),
                         ref("review-old-zone-status-refresh-INTERACT-ZONE03-02.json")],
        "scope": "successful_reapplication_of_existing_actor_Poison_Burn_or_Bleed_from_new_attack_cast_or_existing_zone_automatic_refresh;not_control_damage_buff_or_healing_rules",
        "already_accepted": "one_actor_status_per_kind;latest_successful_application_power_even_if_lower;full_duration_restart;new_status_immediate_first_damage_counted_in_total;no_second_automatic_application_creation_turn;independent_zone_damage_cumulative",
        "proposed_rule": {
            "new_attack_or_cast": "successful_new_attack_or_new_cast_reapplying_existing_Poison_Burn_or_Bleed_replaces_power_restarts_full_duration_and_deals_first_periodic_damage_immediately;includes_new_attack_actual_delayed_impact_or_mine_explosion_not_preparation",
            "new_attack_count": "immediate_damage_counts_as_first_of_renewed_number_of_applications;do_not_add_an_extra_damage_tick_to_that_renewed_total",
            "automatic_damage_after_immediate": "no_additional_automatic_same_kind_actor_periodic_damage_later_in_same_reference_turn_after_this_immediate_application;previously_resolved_damage_not_reversed",
            "existing_zone_refresh": "automatic_successful_refresh_of_already_present_status_from_existing_zone_restarts_duration_and_sets_zone_stored_power_under_existing_rules;refresh_does_not_itself_deal_extra_immediate_actor_status_damage;normal_single_kind_periodic_application_retained_if_not_already_resolved_or_replaced_by_immediate_application_this_turn",
            "zone_damage": "each_independent_zone_keeps_its_own_damage_and_status_attempt_in_accepted_oldest_to_newest_order;do_not_merge_or_suppress_zone_damage_due_to_actor_status_tick_accounting",
            "failed_application": "does_not_refresh_replace_or_cause_immediate_actor_status_damage;attack_or_zone_direct_damage_keeps_its_existing_rule",
            "new_status_from_existing_zone": "if_kind_was_not_present_apply_accepted_TIME01_immediate_first_application_rule;TIME02_refresh_exception_is_for_already_present_status_only",
            "scope_boundary": "no_extra_action_or_energy_cost_no_new_damage_numbers_no_general_buff_or_control_pulse_and_no_duplicate_status_instances;complete_phase_order_and_Contagion_transfer_cadence_not_defined"
        },
        "shown_example": {"illustrative_not_skill_damage_values": True, "old_Poison_damage_before_defenses": 6,
                          "new_successful_attack_Poison_damage_before_defenses": 10,
                          "immediate_new_status_damage_before_defenses": 10, "later_applications_power_before_defenses": 10},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["complete_actor_zone_periodic_and_expiration_phase_order",
                              "Contagion_and_other_non_attack_status_transfer_cadence",
                              "native_feedback_and_modified_individual_skill_copy"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_name_or_UI_error_copy",
        "visual_approval": "no_new_native_art_animation_or_UI_layout_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data["date"] = "2026-10-09"
        data["previous_pending_timer_accounting"] = deepcopy(data["pending_timer_accounting"])
        data["pending_timer_accounting"] = None
        data["accepted_immediate_persistent_application_count"] = ref(ACCEPTED)
        data["latest_immediate_application_count_acceptance"] = ref(DECISION)
        data["pending_persistent_status_reapplication"] = ref(PROPOSAL)
        data["runtime_integration"] = "not_integrated"
        if old_name.startswith("review-implementation"):
            data.update(revision=20, status="immediate_persistent_application_count_accepted;existing_status_reapplication_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-TIME01"] = {
                "rule": "immediate_application_is_first_of_announced_total;subsequent_turn_applications;no_creation_turn_second_automatic_application",
                "source": ref(ACCEPTED), "unshown": "existing_status_refresh_buff_cooldown_convention_and_full_phase_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith("immediate_persistent_application_count_and_same_turn_automatic_repeat_proposed_under_INTERACT_TIME01_revision2")]
                    group["remaining"].append("existing_persistent_status_reapplication_vs_automatic_zone_refresh_proposed_under_INTERACT_TIME02;buff_cooldown_draft_unaccepted")
            data["recommendation"] = "review_existing_persistent_status_reapplication_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            add_override(data)
            data.update(review_id="STAT-MODEL75", status="immediate_application_count_accepted;existing_status_refresh_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-20.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            add_override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-78.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-20.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "new_attack_or_cast_reapplication_vs_existing_zone_automatic_refresh_of_actor_Poison_Burn_Bleed"
            data["next_review_kind"] = "persistent_status_reapplication_INTERACT_TIME02"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-78.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-20.json")
            data["latest_feedback"] = "user_accepts_TIME01_application_count;existing_status_refresh_proposed"
            data["next_review_kind"] = "persistent_status_reapplication_INTERACT_TIME02"
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
        assert new["pending_timer_accounting"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-119.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    save("persistent-count-and-refresh-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-TIME01-TIME02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "TIME01": "accepted", "TIME02": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "application_count": "accepted", "status_reapplication": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
