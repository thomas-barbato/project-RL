"""Design only: accept Gel source rules and propose familiar first-action/arrival hazards."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-gel-source-lifetime-INTERACT-ZONE05-01.json"
DECISION = "gel-source-lifetime-acceptance-decisions.json"
ACCEPTED = "review-gel-source-lifetime-INTERACT-ZONE05-02.json"
PROPOSAL = "review-familiar-first-action-arrival-INTERACT-PETS01-01.json"
PAIRS = [("review-implementation-readiness-27.json", "review-implementation-readiness-28.json"),
         ("review-attribute-model-85.json", "review-attribute-model-86.json"),
         ("review-primary-catalogue-257.json", "review-primary-catalogue-258.json"),
         ("review-complete-skill-design-232.json", "review-complete-skill-design-233.json"),
         ("remaining-individual-skill-review-counts-126.json", "remaining-individual-skill-review-counts-127.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-PETS01 · Première action et dangers à l'arrivée des familiers — proposition

- Drone nouvellement invoqué : il peut agir pendant la phase des familiers du tour d'invocation, selon ses capacités et conditions normales.
- Monstre ressuscité : il commence toujours à agir au tour suivant son relèvement, comme validé.
- Familier déjà existant qui arrive sur la carte ou est repositionné : il conserve son cycle d'action normal. Cette arrivée ou ce déplacement ne lui accorde pas d'action supplémentaire et ne réinitialise pas ses recharges.
- Apparition ou arrivée dans une zone persistante : les effets de la zone s'appliquent dès l'arrivée si ce familier est une cible admissible, avec la limite validée d'une application par zone et par cible pendant le même tour. Un repositionnement entièrement à l'intérieur d'une même zone suit la règle normale des déplacements internes.

Un familier en attente arrive après votre déplacement effectué, avant la phase des effets périodiques et les actions des familiers. Les limites de place et l'ordre d'arrivée déjà validés restent conservés. Vos mines ne sont pas déclenchées par vos familiers.

Exemple : vous invoquez un drone dans une zone dangereuse pour lui. Il subit d'abord l'effet de cette zone. S'il survit et peut agir, il joue pendant la phase des familiers avant les ennemis. S'il meurt à l'arrivée, il ne joue pas et ses objets réels tombent sur sa case selon la règle validée.

Les règles de cible et de dégâts alliés restent propres à chaque compétence. L'arrivée conserve l'état réel du familier ; les ressources de naissance et les durées des ressuscités gardent leurs règles validées.

Ces précisions restent proposées, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["gel_source_lifetime"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "instance": "one_current_Gel;latest_successful_application_replaces_power_source_and_timed_or_zone_bound_lifetime;failed_attempt_changes_nothing",
        "zone_bound": "current_source_zone_exit_or_removal_ends_current_Gel;remaining_other_zone_requires_new_eligible_successful_attempt",
        "timed": "independent_timer_survives_zone_exit_or_unrelated_zone_removal;later_successful_zone_application_can_replace_it_and_vice_versa",
        "no_restore": "no_older_overwritten_Gel_or_timer_restoration_and_no_extra_source_loss_roll",
        "preserved": "Gel_Burn_successful_mutual_removal;current_source_for_accepted_Contagion_transfer;received_Gel_independent_timer_and_no_retransmission;zone_exposure_budget_resistances_and_importance",
        "pending": "initial_actor_placement_and_native_source_inspection_feedback"
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
        assert new["pending_gel_source_lifetime"] is None
        assert new["pending_familiar_first_action_arrival"] == ref(PROPOSAL)
        assert "familiar_first_action_arrival" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-127.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE05-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "oui ca me va",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_single_Gel_source_and_lifetime_latest_successful_replacement_and_no_older_restore_accepted;initial_actor_arrival_hazards_simultaneous_proc_ties_and_native_feedback_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="Gel_source_lifetime_explicitly_accepted;initial_actor_zone_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-PETS01", "revision": 1,
        "title": "Première action et dangers à l'arrivée des familiers",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-turn-resolution-INTERACT-TURN01-02.json"),
                         ref("review-zone-entry-exit-INTERACT-ZONE04-02.json"),
                         ref("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-02.json"),
                         ref("review-familiar-arrival-priority-INTERACT-ARRIVAL01-02.json"),
                         ref("review-pending-familiar-arrival-PETS-EQ19-02.json"),
                         ref("review-familiar-command-action-costs-INTERACT-CMD01-02.json"),
                         ref("review-mine-arming-trigger-INTERACT-MINE02-02.json")],
        "scope": "new_drone_first_action_and_existing_familiar_arrival_or_reposition_zone_entry;not_new_actor_speeds_budgets_or_new_invocation_tile_permissions",
        "already_accepted": "raised_first_action_next_birth_turn;familiars_before_enemies;actual_step_progressive_arrival;existing_actor_state_and_recharges_preserved;zone_entry_budget;own_mines_nonhostile_no_trigger;real_items_on_death",
        "proposed_rule": {
            "new_drone": "successful_ordinary_invocation_allows_living_and_otherwise_eligible_new_drone_to_act_in_that_reference_turn_familiar_phase;no_extra_out_of_phase_action_or_actor_budget_reset",
            "new_raised": "preserve_no_action_on_birth_turn_and_first_normal_action_from_following_reference_turn;no_new_delay_on_later_map_arrival",
            "existing_actor": "map_arrival_or_Repositionner_preserves_same_actor_remaining_action_cycle_delays_and_eligibility;no_bonus_action_or_cooldown_reset;normal_remaining_actions_if_otherwise_eligible",
            "progressive_arrival_phase": "after_successful_actual_player_grid_movement_place_one_eligible_waiting_surviving_familiar_under_accepted_priority_and_cell_rules_before_periodic_and_familiar_actor_phases;state_continuity_not_new_birth",
            "zone_entry": "creation_reanimation_map_arrival_or_reposition_into_zone_counts_as_entry_for_that_actor_if_zone_target_rules_eligible;immediate_effect_under_shared_zone_target_reference_turn_budget;within_same_zone_reposition_follows_existing_internal_movement_rule",
            "target_and_friendly_rules": "respect_each_zone_target_eligibility_resistance_and_existing_per_skill_ally_damage_rule;no_general_ally_immunity_or_automatic_all_zone_damage_created",
            "own_mines": "existing_player_SM00_mines_not_triggered_by_nonhostile_familiars;no_change_to_existing_ally_explosion_victim_rules",
            "death": "if_entry_hazard_kills_familiar_resolve_death_before_any_familiar_action_and_drop_existing_real_equipment_and_cargo_on_placed_actor_cell;no_duplicate_rewards",
            "preserved": "birth_resource_initialization;living_arrival_no_refill_or_timer_restart;existing_raised_remaining_lifetime;waiting_state_effects_no_map_actions;cell_priority_and_map_transfer_rules",
            "scope_boundary": "no_new_dangerous_tile_exclusion_or_immunity_no_extra_turn_cost_no_universal_actor_speed_rule_no_new_native_feedback"
        },
        "shown_example": {"event": "new_drone_invoked_into_zone_harmful_to_that_drone", "entry": "eligible_zone_effect_first",
                          "if_alive_and_eligible": "normal_familiar_phase_action_before_enemies", "if_dead": "no_action;existing_real_objects_drop_on_drone_cell"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["other_drone_combat_order_action_costs", "simultaneous_multi_target_death_proc_ties", "native_arrival_and_disabled_action_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_gel_source_lifetime"] = deepcopy(data["pending_gel_source_lifetime"])
        data["pending_gel_source_lifetime"] = None
        data["accepted_gel_source_lifetime"] = ref(ACCEPTED)
        data["latest_gel_source_lifetime_acceptance"] = ref(DECISION)
        data["pending_familiar_first_action_arrival"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=28, status="Gel_source_lifetime_accepted;familiar_first_action_and_zone_arrival_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-ZONE05"] = {"source": ref(ACCEPTED),
                "rule": "single_Gel_latest_success_replaces_power_source_and_lifetime;source_zone_loss_ends_zone_Gel;timed_Gel_independent;no_older_restore",
                "unshown": "initial_actor_arrival_zone_hazards_and_native_source_inspection"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith(("Gel_source_lifetime_and_no_older_source_restore_proposed_under_INTERACT_ZONE05", "other_unshown_Contagion_retention_bindings_geometry_ties_and_Gel_provenance_pending", "specific_simultaneous_or_initial_placement_cases_unshown"))]
                    group["remaining"].extend(["familiar_first_action_arrival_and_zone_hazards_proposed_under_INTERACT_PETS01",
                                               "other_unshown_Contagion_retention_bindings_geometry_ties_and_simultaneous_death_cases_pending"])
            data["recommendation"] = "review_familiar_first_action_arrival_then_remaining_command_costs_and_class_starts_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL83", status="Gel_source_lifetime_accepted;familiar_first_action_arrival_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-28.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-86.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-28.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "new_drone_first_action_existing_actor_arrival_cycle_and_eligible_zone_entry_hazards"
            data["next_review_kind"] = "familiar_first_action_arrival_INTERACT_PETS01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-86.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-28.json")
            data["latest_feedback"] = "user_accepts_Gel_source_lifetime;familiar_first_action_arrival_proposed"
            data["next_review_kind"] = "familiar_first_action_arrival_INTERACT_PETS01"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("familiar-first-action-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE05-PETS01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "ZONE05": "accepted", "PETS01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "Gel_source_lifetime": "accepted", "familiar_first_action_arrival": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
