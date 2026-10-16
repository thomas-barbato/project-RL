"""Design only: accept mine arming and propose single Gel source/lifetime rules."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-mine-arming-trigger-INTERACT-MINE02-01.json"
DECISION = "mine-arming-trigger-acceptance-decisions.json"
ACCEPTED = "review-mine-arming-trigger-INTERACT-MINE02-02.json"
PROPOSAL = "review-gel-source-lifetime-INTERACT-ZONE05-01.json"
PAIRS = [("review-implementation-readiness-26.json", "review-implementation-readiness-27.json"),
         ("review-attribute-model-84.json", "review-attribute-model-85.json"),
         ("review-primary-catalogue-256.json", "review-primary-catalogue-257.json"),
         ("review-complete-skill-design-231.json", "review-complete-skill-design-232.json"),
         ("remaining-individual-skill-review-counts-125.json", "remaining-individual-skill-review-counts-126.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-ZONE05 · Plusieurs sources de Gel — proposition

- Une cible porte un seul Gel. La dernière application réussie remplace sa puissance, sa durée ou sa condition de maintien, ainsi que sa source. Une tentative échouée ne change rien au Gel présent.
- Si ce Gel dépend d'une zone, sortir de cette zone ou la voir disparaître met fin à ce Gel. Une autre zone de glace encore présente devra réussir une nouvelle application pour geler à nouveau la cible.
- Si le Gel possède sa propre durée, quitter une zone ne le retire pas. Une application ultérieure réussie par une zone peut cependant le remplacer par son Gel dépendant de cette zone.
- Un ancien Gel déjà remplacé ne revient pas automatiquement quand le nouveau prend fin. Une Brûlure réussie supprime toujours le Gel actif.

Exemple : deux zones A et B se superposent. A réussit à geler un monstre, puis B réussit à son tour : le Gel dépend désormais de B. Si B disparaît, ce Gel prend fin ; A devra réussir une nouvelle application pour regeler le monstre.

Les Gels temporaires transmis par Contagion conservent leur durée indépendante et leur interdiction de retransmission. Les chances, résistances et malus des élites et des boss restent conservés.

Cette gestion de la source et de la durée du Gel reste proposée, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["mine_arming_trigger"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "arming": "inactive_during_ordinary_placement_turn_actor_responses;arms_at_end_of_that_turn_after_one_reference_turn_elapsed_before_recovery",
        "contact_before_arming": "no_trigger_while_unarmed;living_hostile_still_on_cell_at_arming_triggers_explosion;departed_hostile_does_not",
        "armed_contact": "hostile_entry_including_forced_movement_triggers_immediate_single_detonation;nonhostiles_do_not_trigger",
        "effects": "consume_mine_before_explosion;damage_status_and_eligible_residue_first_effect_immediate_with_stored_placement_power_and_current_target_defenses",
        "preserved": "SM00_enemy_occupied_placement_refusal;capacity_cleanup;per_victim_sampling;player_only_general_self_immunity_and_individual_ally_damage_rules;accepted_blinking_light",
        "pending": "simultaneous_death_proc_ties_explosion_geometry_and_other_mine_responses"
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
        assert new["pending_mine_arming_trigger"] is None
        assert new["pending_gel_source_lifetime"] == ref(PROPOSAL)
        assert "gel_source_lifetime" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-126.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-MINE02-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "oui ca me va :)",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_one_turn_arming_end_phase_prearming_occupant_and_armed_hostile_contact_single_explosion_immediate_effect_rules_accepted;no_new_chain_detonation_simultaneous_proc_tie_geometry_or_native_rendering",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="mine_arming_and_trigger_explicitly_accepted;other_unshown_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-ZONE05", "revision": 1,
        "title": "Plusieurs sources de Gel : source et durée",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("last-status-cast-power-acceptance-decisions.json"),
                         ref("review-overlapping-zone-status-INTERACT-ZONE02-02.json"),
                         ref("review-old-zone-status-refresh-INTERACT-ZONE03-02.json"),
                         ref("review-zone-entry-exit-INTERACT-ZONE04-02.json"),
                         ref("review-mine-residue-access-loss-INTERACT-RANK05-02.json"),
                         ref("review-contagion-zone-gel-INTERACT-CONTAGION02-02.json")],
        "scope": "single_current_Gel_instance_source_and_lifetime_when_timed_and_zone_only_sources_replace_each_other;not_new_chance_or_duration_numbers",
        "already_accepted": "one_status_per_kind;latest_successful_power_even_if_weaker;failed_attempt_no_refresh;zone_only_Gel_ends_on_source_exit_or_removal;no_restore_overwritten_status;Gel_Burn_mutual_successful_removal",
        "proposed_rule": {
            "one_current_instance": "one_current_Gel_per_actor;latest_successful_application_replaces_current_power_source_and_lifetime_mode_not_parallel_hidden_Gel_components",
            "timed_mode": "incoming_independently_timed_Gel_restarts_with_that_application_full_accepted_duration;zone_exit_or_unrelated_zone_removal_does_not_cancel_it",
            "zone_mode": "incoming_zone_only_Gel_depends_on_that_successful_source_zone;exit_or_removal_of_that_source_ends_current_Gel_even_if_other_cold_zone_remains",
            "later_success": "later_successful_zone_application_can_replace_timed_Gel_with_own_zone_bound_Gel;later_successful_timed_application_can_replace_zone_bound_Gel_with_independent_timer",
            "no_restore": "when_current_Gel_ends_do_not_restore_older_overwritten_Gel_or_hidden_timer;remaining_zone_can_regel_only_by_new_successful_attempt_at_its_next_eligible_application",
            "failure": "failed_application_preserves_current_Gel_source_power_and_lifetime_without_refresh",
            "opposition": "successful_Burn_removes_current_Gel;successful_Gel_removes_Burn_as_already_accepted;no_restoration_of_overwritten_opposed_status",
            "Contagion": "use_current_selected_Gel_source_for_accepted_transfer_rule;zone_only_source_remaining_time_reference_from_current_source;successful_received_Gel_has_independent_timer_and_cannot_be_retransmitted;later_replacement_keeps_the_incoming_application_origin_and_its_own_accepted_eligibility_rules",
            "preserved": "existing_chances_resistances_importance_modifiers;per_zone_exposure_and_application_budget;oldest_to_newest_same_event_zone_order;creator_rank_loss_cleanup_scopes",
            "scope_boundary": "no_new_damage_or_status_numbers_no_extra_automatic_status_roll_on_source_loss_no_hidden_Gel_stack_or_union_of_zone_lifetimes;no_new_native_feedback"
        },
        "shown_example": {"two_overlapping_zones": ["A", "B"], "applications": ["A_success", "B_success"],
                          "current_Gel_source": "B", "B_removed": "current_Gel_ends;A_requires_new_eligible_successful_attempt"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["initial_actor_creation_or_pending_arrival_zone_hazards", "simultaneous_death_proc_ties", "native_source_duration_inspection_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_mine_arming_trigger"] = deepcopy(data["pending_mine_arming_trigger"])
        data["pending_mine_arming_trigger"] = None
        data["accepted_mine_arming_trigger"] = ref(ACCEPTED)
        data["latest_mine_arming_trigger_acceptance"] = ref(DECISION)
        data["pending_gel_source_lifetime"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=27, status="mine_arming_trigger_accepted;Gel_source_lifetime_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-MINE02"] = {"source": ref(ACCEPTED),
                "rule": "mine_unarmed_during_cast_turn;arm_at_end_after1_turn;occupied_hostile_check;armed_entry_trigger;single_explosion_and_immediate_residue",
                "unshown": "simultaneous_ties_explosion_geometry_and_other_mine_response"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith("mine_arming_and_hostile_contact_trigger_proposed_under_INTERACT_MINE02")]
                    group["remaining"].append("Gel_source_lifetime_and_no_older_source_restore_proposed_under_INTERACT_ZONE05")
            data["recommendation"] = "review_Gel_source_lifetime_and_initial_actor_zone_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL82", status="mine_arming_trigger_accepted;Gel_source_lifetime_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-27.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-85.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-27.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "single_current_Gel_source_power_and_lifetime_mode_replacement_without_older_restore"
            data["next_review_kind"] = "gel_source_lifetime_INTERACT_ZONE05"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-85.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-27.json")
            data["latest_feedback"] = "user_accepts_mine_arming_trigger;Gel_source_lifetime_proposed"
            data["next_review_kind"] = "gel_source_lifetime_INTERACT_ZONE05"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("gel-source-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-MINE02-ZONE05-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "MINE02": "accepted", "ZONE05": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "mine_arming": "accepted", "Gel_source_lifetime": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
