"""Design only: accept core turn order and propose persistent-zone entry rules."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-turn-resolution-INTERACT-TURN01-01.json"
DECISION = "turn-resolution-acceptance-decisions.json"
ACCEPTED = "review-turn-resolution-INTERACT-TURN01-02.json"
PROPOSAL = "review-zone-entry-exit-INTERACT-ZONE04-01.json"
PAIRS = [("review-implementation-readiness-24.json", "review-implementation-readiness-25.json"),
         ("review-attribute-model-82.json", "review-attribute-model-83.json"),
         ("review-primary-catalogue-254.json", "review-primary-catalogue-255.json"),
         ("review-complete-skill-design-229.json", "review-complete-skill-design-230.json"),
         ("remaining-individual-skill-review-counts-123.json", "remaining-individual-skill-review-counts-124.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-ZONE04 · Entrer et sortir d'une zone persistante — proposition

- Entrer dans une zone déclenche son effet sur la cible dès l'arrivée, si cette zone ne l'a pas déjà affectée ce tour.
- Chaque zone produit au maximum une application de son effet par cible et par tour. Se déplacer à l'intérieur, ou sortir puis revenir pendant le même tour, ne donne pas de dégâts ni de jet de statut supplémentaires de cette zone.
- Rester dans la zone permet une nouvelle application au tour suivant. Pour les chances de statut, l'entrée compte comme la première exposition ; le cumul augmente une seule fois par tour passé dans cette zone, puis repart de zéro à la sortie.
- En sortant, la cible cesse de subir les prochaines applications de cette zone. Poison, Brûlure et les autres statuts ayant leur propre durée continuent jusqu'à leur fin normale. Un Gel uniquement entretenu par la zone cesse à la sortie.

Exemple avec une chance de base de 10 % : premier jet à 10 %, jet du tour suivant à 20 % si la cible est restée dans la zone. Après une sortie, le premier jet suivant reprend à 10 %. Un retour pendant un tour déjà affecté ne donne pas un nouveau jet.

Si plusieurs zones se superposent, chacune garde sa propre application et son propre cumul ; elles sont traitées de la plus ancienne à la plus récente. Les dégâts de statut distincts restent soumis au décompte déjà validé. Les protections contre les dégâts de vos propres sorts restent conservées.

Ces règles d'entrée et de cadence restent proposées, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["turn_resolution"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "core_order": ["player_action_immediate_effects", "eligible_zones_oldest_to_newest_then_actor_periodic_damage",
                       "eligible_familiars_then_living_enemies", "duration_cooldown_expiry_and_accepted_recovery", "perception_threat_and_UI_update"],
        "death": "resolve_each_fatal_damage_death_and_applicable_drops_rewards_Contagion_reanimation_before_next_event_or_actor;no_later_action_or_regeneration_rescue_for_dead_actor",
        "preserved": "first_persistent_damage_no_automatic_repeat;automatic_zone_refresh_rule;existing_actor_speeds_budgets_delays;raised_first_action_next_birth_turn;immediate_reactions",
        "pending": "zone_entry_mine_arming_simultaneous_proc_ties_drone_first_action_and_specific_placement_hazards"
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
        assert new["pending_turn_resolution"] is None
        assert new["pending_zone_entry_exit"] == ref(PROPOSAL)
        assert "zone_entry_exit" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-124.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-TURN01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait oui",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_core_phase_order_and_immediate_death_resolution_accepted;zone_entry_mine_arming_specific_arrival_hazards_simultaneous_proc_ties_and_native_feedback_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="core_turn_order_explicitly_accepted;specific_entry_and_simultaneous_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE04", "revision": 1,
        "title": "Entrer et sortir d'une zone persistante",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-timer-accounting-INTERACT-TIME01-03.json"),
                         ref("review-persistent-status-reapplication-INTERACT-TIME02-02.json"),
                         ref("review-overlapping-zone-status-INTERACT-ZONE02-02.json"),
                         ref("review-old-zone-status-refresh-INTERACT-ZONE03-02.json")],
        "scope": "ordinary_persistent_ground_zone_entry_and_once_per_target_zone_reference_turn_application;not_mines_or_blocking_object_collisions",
        "already_accepted": "immediate_creation_effect;per_zone_consecutive_exposure_and_exit_reset;independent_overlapping_zones_oldest_to_newest;latest_successful_actor_status_power;own_spell_friendly_damage_exclusion",
        "proposed_rule": {
            "entry": "apply_eligible_zone_effect_immediately_on_entering_zone_if_zone_has_not_already_applied_to_target_in_current_reference_turn",
            "application_budget": "at_most_one_own_zone_effect_application_and_status_attempt_per_zone_instance_target_and_reference_turn;creation_entry_and_automatic_phase_share_same_budget",
            "same_turn_movement": "movement_within_zone_or_exit_reentry_does_not_grant_additional_own_zone_damage_or_status_attempt_if_budget_already_used",
            "following_turn": "eligible_target_still_inside_can_receive_next_ordinary_zone_application_at_accepted_periodic_phase",
            "exposure": "first_entry_application_is_exposure1;at_most_one_increment_per_zone_target_reference_turn_while_continuously_inside;exit_resets_only_exited_zone_counter;reentry_when_budget_used_has_no_attempt_or_increment_until_next_eligible_turn_at_exposure1",
            "exit": "stop_future_effects_of_exited_zone;do_not_undo_resolved_damage;independently_timed_actor_statuses_keep_remaining_timer;Gel_solely_maintained_by_exited_zone_ends_under_existing_rule",
            "overlap": "each_zone_own_budget_and_exposure;eligible_same_event_zones_oldest_to_newest;latest_successful_same_kind_status_replaces_power_and_duration_as_accepted",
            "actor_periodic_status": "zone_application_budget_is_separate_from_actor_Poison_Burn_Bleed_accounting;existing_zone_refresh_of_present_status_has_no_extra_immediate_status_damage;new_status_first_application_and_no_same_turn_automatic_repeat_preserved",
            "friendly_damage": "preserve_already_accepted_no_player_or_ally_damage_from_own_offensive_effects_except_explicit_tradeoff_skills",
            "scope_boundary": "no_new_damage_chance_duration_numbers;no_mine_arming_or_entry_trigger_rule;multiple_source_zone_only_Gel_provenance_and_new_actor_placement_cases_pending"
        },
        "shown_example": {"illustrative_not_skill_parameters": True, "base_chance_percent": 10, "entry_exposure1_percent": 10,
                          "next_continuous_turn_exposure2_percent": 20, "after_exit_first_eligible_attempt_percent": 10,
                          "same_turn_reentry_after_used_budget": "no_new_attempt"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["mine_arming_and_entry_trigger_phase", "multiple_zone_only_Gel_source_provenance",
                              "initial_actor_creation_or_pending_familiar_placement_vs_zone_entry", "simultaneous_multi_target_death_proc_ties", "native_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_turn_resolution"] = deepcopy(data["pending_turn_resolution"])
        data["pending_turn_resolution"] = None
        data["accepted_turn_resolution"] = ref(ACCEPTED)
        data["latest_turn_resolution_acceptance"] = ref(DECISION)
        data["pending_zone_entry_exit"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=25, status="core_turn_order_accepted;persistent_zone_entry_exit_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-TURN01"] = {"source": ref(ACCEPTED),
                "rule": "player_immediate;zones_then_periodic;eligible_familiars_then_enemies;timers_expiry_recovery;UI;deaths_resolved_before_next_event_or_actor",
                "unshown": "zone_entry_mines_and_specific_simultaneous_or_placement_cases"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith("core_turn_order_and_immediate_death_resolution_proposed_under_INTERACT_TURN01")]
                    group["remaining"].extend(["persistent_zone_entry_exit_and_shared_per_turn_application_proposed_under_INTERACT_ZONE04",
                                               "mine_arming_and_specific_simultaneous_or_placement_cases_unshown"])
            data["recommendation"] = "review_zone_entry_and_remaining_core_trigger_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL80", status="core_turn_order_accepted;zone_entry_exit_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-25.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-83.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-25.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "persistent_zone_entry_exit_shared_per_target_turn_application_and_exposure"
            data["next_review_kind"] = "zone_entry_exit_INTERACT_ZONE04"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-83.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-25.json")
            data["latest_feedback"] = "user_accepts_core_turn_order;zone_entry_exit_proposed"
            data["next_review_kind"] = "zone_entry_exit_INTERACT_ZONE04"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("zone-entry-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-TURN01-ZONE04-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "TURN01": "accepted", "ZONE04": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "turn_order": "accepted", "zone_entry_exit": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
