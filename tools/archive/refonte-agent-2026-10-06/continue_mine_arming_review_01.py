"""Design only: accept zone entry, reconcile scope metadata, propose mine arming."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-zone-entry-exit-INTERACT-ZONE04-01.json"
RECONCILIATION = "zone-friendly-damage-metadata-reconciliation-01.json"
DECISION = "zone-entry-exit-acceptance-decisions.json"
ACCEPTED = "review-zone-entry-exit-INTERACT-ZONE04-02.json"
PROPOSAL = "review-mine-arming-trigger-INTERACT-MINE02-01.json"
FRIENDLY = "preserve_SELF_DMG01_player_only_immunity_from_own_damage;familiar_and_other_ally_eligibility_remains_per_skill_under_existing_approvals;no_general_ally_immunity_created"
PAIRS = [("review-implementation-readiness-25.json", "review-implementation-readiness-26.json"),
         ("review-attribute-model-83.json", "review-attribute-model-84.json"),
         ("review-primary-catalogue-255.json", "review-primary-catalogue-256.json"),
         ("review-complete-skill-design-230.json", "review-complete-skill-design-231.json"),
         ("remaining-individual-skill-review-counts-124.json", "remaining-individual-skill-review-counts-125.json")]
NEW_FILES = [RECONCILIATION, DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-MINE02 · Armement et déclenchement des mines — proposition

- Une mine reste inactive pendant le tour de pose. Elle s'arme à la fin de ce tour, une fois son délai de 1 tour écoulé.
- Un ennemi qui passe dessus avant l'armement ne la déclenche pas. S'il occupe encore sa case au moment de l'armement, elle explose alors. S'il est déjà reparti, elle reste posée et armée.
- Une fois armée, elle explose dès qu'un ennemi entre sur sa case, y compris par un déplacement forcé. Le joueur et ses familiers ne la déclenchent pas.
- L'explosion consomme la mine une seule fois. Ses dégâts et statuts sont résolus immédiatement ; un éventuel résidu persistant commence aussi dès cette explosion, suivant les règles validées.

Le voyant rouge clignote une fois la mine armée, comme validé. Les règles de pose, les valeurs fixées à la pose et les protections propres à chaque cible restent conservées. La pose sur une case déjà occupée par un ennemi reste refusée selon la fiche SM00 ; le cas ci-dessus concerne un ennemi arrivant après la pose.

Ce décompte d'armement et ces précisions de déclenchement restent proposés, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["zone_entry_exit"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION), "scope_reconciliation": ref(RECONCILIATION),
        "entry": "eligible_zone_effect_immediate_on_entry_if_current_zone_target_turn_application_not_used",
        "budget": "creation_entry_and_periodic_pass_share_one_zone_effect_application_and_status_attempt_per_zone_target_reference_turn;failed_attempt_consumes_attempt_budget",
        "exposure": "first_application_exposure1;one_increment_per_continuous_zone_target_turn;exit_resets_only_exited_zone;used_budget_reentry_waits_next_eligible_turn_at_exposure1",
        "exit": "future_zone_applications_stop;independent_status_timers_continue;zone_only_Gel_ends_under_existing_rules",
        "preserved": "independent_overlapping_zones_oldest_to_newest;accepted_actor_status_damage_accounting_and_reapplication",
        "friendly_damage": FRIENDLY,
        "pending": "mine_arming_multiple_zone_Gel_provenance_and_initial_actor_placement_cases"
    }


def validate():
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
        assert new["pending_zone_entry_exit"] is None
        assert new["pending_mine_arming_trigger"] == ref(PROPOSAL)
        assert "mine_arming_trigger" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    expected = deepcopy(load(SOURCE)["proposed_rule"])
    expected["friendly_damage"] = FRIENDLY
    assert load(ACCEPTED)["accepted_rule"] == expected
    assert load(RECONCILIATION)["new_user_visible_rule"] is False
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-125.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(RECONCILIATION, {
        "date": "2026-10-09", "review_id": "ZONE04-SCOPE01", "source_proposal": ref(SOURCE),
        "canonical_user_rule": ref("own-spell-self-damage-protection-decisions.json"),
        "historical_mine_card": ref("review-individual-description-SM00-02.json"),
        "finding": "ZONE04_metadata_and_review_note_overstated_general_ally_immunity;SELF_DMG01_is_player_only_and_keeps_per_skill_ally_rules;shown_final_ZONE04_proposal_did_not_request_change_to_ally_damage",
        "correction": FRIENDLY, "new_user_visible_rule": False,
        "approval_boundary": "documentation_scope_correction_to_existing_explicit_SELF_DMG01_and_individual_skill_approvals;no_new_friendly_damage_rule_or_user_approval_inferred",
        "runtime_integration": "not_integrated"
    })
    approved = deepcopy(source["proposed_rule"])
    approved["friendly_damage"] = FRIENDLY
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE04-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "oui parfait on continue",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": approved,
        "scope_reconciliation": ref(RECONCILIATION),
        "approval_boundary": "shown_immediate_zone_entry_shared_application_budget_exposure_reset_and_exit_rules_accepted;unshown_general_ally_immunity_not_approved;existing_player_and_per_skill_ally_rules_preserved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="zone_entry_exit_explicitly_accepted;friendly_damage_metadata_reconciled_to_existing_scope;other_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed;no_unshown_global_ally_immunity", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted.pop("proposed_rule")
    accepted["accepted_rule"] = approved
    accepted["scope_reconciliation"] = ref(RECONCILIATION)
    accepted["already_accepted"] = "immediate_creation_effect;per_zone_consecutive_exposure_and_exit_reset;independent_overlapping_zones_oldest_to_newest;latest_successful_actor_status_power;SELF_DMG01_player_only_and_existing_per_skill_ally_damage_rules"
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-MINE02", "revision": 1,
        "title": "Armement et déclenchement des mines",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-turn-resolution-INTERACT-TURN01-02.json"),
                         ref("review-mine-placement-power-INTERACT-MINE01-02.json"), ref("review-individual-description-SM00-02.json"),
                         ref("review-timer-accounting-INTERACT-TIME01-03.json"), ref("own-spell-self-damage-protection-decisions.json")],
        "scope": "SM00_existing_one_turn_arming_phase_and_hostile_contact_trigger;not_new_placement_permission_or_damage_values",
        "already_accepted": "one_turn_arming;armed_mine_hostile_contact;SM00_visible_accessible_own_or_adjacent_cell_without_enemy_or_other_mine;blinking_red_armed_light;stored_placement_power;immediate_persistent_start_at_actual_explosion",
        "proposed_rule": {
            "arming": "ordinary_one_turn_placement_action_mine_unarmed_during_that_turn_actor_responses;arm_at_end_of_that_turn_after_one_reference_turn_elapsed_in_end_phase_before_recovery;no_zero_time_UI_or_refused_action_progress",
            "prearming_contact": "hostile_entering_unarmed_mine_cell_does_not_trigger;on_arming_check_current_living_hostile_occupant_and_explode_if_present;hostile_that_already_left_does_not_trigger",
            "armed_contact": "living_hostile_entering_armed_mine_cell_triggers_immediate_explosion_including_forced_movement;player_familiars_and_other_nonhostiles_do_not_trigger",
            "consumption": "remove_triggered_mine_before_resolving_its_explosion;one_detonation_and_no_later_retrigger_of_same_mine",
            "effects": "explosion_damage_and_statuses_immediate_under_accepted_current_target_defenses_and_stored_mine_power;eligible_residue_zone_created_and_first_effect_immediate;no_duplicate_automatic_creation_turn_zone_effect",
            "preserved": "existing_damage_and_status_values;one_mine_per_cell_capacity_cleanup;per_victim_sampling;SELF_DMG01_player_immunity_and_per_skill_ally_damage_rules;red_armed_light_blinks_without_new_native_rendering",
            "placement": "existing_SM00_enemy_occupied_cell_refusal_preserved;enemy_at_arming_case_is_later_arrival_not_permission_to_place_under_enemy",
            "scope_boundary": "no_new_chain_detonation_or_remote_trigger_rule;simultaneous_multi_mine_arming_death_proc_ties_geometry_large_footprints_and_native_feedback_not_defined"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["simultaneous_multi_mine_and_area_death_proc_ties", "explosion_geometry_and_large_actor_footprints", "other_mines_response_to_explosion", "native_arming_and_explosion_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "existing_blinking_armed_light_direction_preserved;no_new_native_rendering", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_zone_entry_exit"] = deepcopy(data["pending_zone_entry_exit"])
        data["pending_zone_entry_exit"] = None
        data["accepted_zone_entry_exit"] = ref(ACCEPTED)
        data["latest_zone_entry_exit_acceptance"] = ref(DECISION)
        data["zone_friendly_damage_scope_reconciliation"] = ref(RECONCILIATION)
        data["pending_mine_arming_trigger"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=26, status="zone_entry_exit_accepted;mine_arming_trigger_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-ZONE04"] = {"source": ref(ACCEPTED),
                "rule": "immediate_entry_shared_zone_target_turn_budget;one_exposure_increment;exit_reset_and_independent_status_timer_preservation",
                "unshown": "mine_arming_multiple_source_Gel_and_initial_placement"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith(("persistent_zone_entry_exit_and_shared_per_turn_application_proposed_under_INTERACT_ZONE04", "mine_arming_and_specific_simultaneous_or_placement_cases_unshown"))]
                    group["remaining"].extend(["mine_arming_and_hostile_contact_trigger_proposed_under_INTERACT_MINE02", "specific_simultaneous_or_initial_placement_cases_unshown"])
            data["recommendation"] = "review_mine_arming_and_remaining_core_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL81", status="zone_entry_exit_accepted;mine_arming_trigger_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-26.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-84.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-26.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "existing_one_turn_mine_arming_and_hostile_contact_trigger_phase"
            data["next_review_kind"] = "mine_arming_trigger_INTERACT_MINE02"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-84.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-26.json")
            data["latest_feedback"] = "user_accepts_zone_entry_exit;mine_arming_trigger_proposed;player_only_general_immunity_scope_reconciled"
            data["next_review_kind"] = "mine_arming_trigger_INTERACT_MINE02"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("mine-arming-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE04-MINE02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "ZONE04": "shown_entry_rules_accepted", "friendly_scope": "existing_player_only_general_immunity_and_per_skill_ally_rules_preserved",
        "MINE02": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "zone_entry_exit": "accepted", "mine_arming": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
