"""Design only: accept zone Gel transfer and propose buff/cooldown accounting."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-contagion-zone-gel-INTERACT-CONTAGION02-01.json"
DECISION = "contagion-zone-gel-acceptance-decisions.json"
ACCEPTED = "review-contagion-zone-gel-INTERACT-CONTAGION02-02.json"
PROPOSAL = "review-buff-cooldown-accounting-INTERACT-TIME03-01.json"
PAIRS = [("review-implementation-readiness-22.json", "review-implementation-readiness-23.json"),
         ("review-attribute-model-80.json", "review-attribute-model-81.json"),
         ("review-primary-catalogue-252.json", "review-primary-catalogue-253.json"),
         ("review-complete-skill-design-227.json", "review-complete-skill-design-228.json"),
         ("remaining-individual-skill-review-counts-121.json", "remaining-individual-skill-review-counts-122.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-TIME03 · Durées des buffs et recharges — proposition

Je propose que le temps écoulé pendant l'action de lancement compte dans la durée du buff et dans sa recharge.

- Buff de 3 tours : il agit pendant le tour de lancement, y compris les réactions ennemies, puis pendant les 2 tours suivants. Après le tour de lancement, son icône indique 2 tours restants.
- Recharge de 3 tours : après un lancement qui consomme 1 tour, le menu indique 2 tours restants. Après 2 autres tours de jeu, la compétence redevient utilisable si ses autres conditions sont remplies.
- Les menus, survols, annulations et actions qui ne font pas avancer le temps ne diminuent aucun compteur.

Une compétence en recharge reste grisée et bloquée, même via un raccourci. À zéro, seul le blocage lié à la recharge disparaît.

Les valeurs de 3 tours illustrent uniquement le décompte. Elles ne changent aucune durée ni recharge de compétence. Les règles déjà validées pour les dégâts persistants, les délais particuliers et les durées des familiers restent conservées.

Cette convention reste proposée, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["contagion_zone_gel_lifetime"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "reference_duration": "selected_active_source_zone_remaining_turns_capped_at5",
        "duration": "apply_current_AF06_retained_percent;floor;minimum1_on_success;never_above_source_zone_remaining_turns",
        "independence": "received_Gel_can_apply_outside_source_zone_and_keeps_own_timer_after_source_zone_removal;successful_Burn_removes_it",
        "preserved": "ordinary_timed_Gel;source_zone_exit_rules;no_chain;zone_stays_in_place;current_target_resistance_and_importance_chance_rules",
        "pending": "multiple_source_provenance_full_phase_order_and_unshown_other_retention_bindings"
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
        assert new["pending_contagion_zone_gel"] is None
        assert new["pending_buff_cooldown_accounting"] == ref(PROPOSAL)
        assert "buff_cooldown_accounting" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-122.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION02-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait :)",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_zone_only_Gel_reference_cap5_retained_duration_floor_min1_and_independence_accepted;multiple_source_merging_unshown_other_retention_bindings_or_native_art_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="zone_only_Gel_transfer_duration_explicitly_accepted;other_unshown_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME03", "revision": 1,
        "title": "Durées des buffs et recharges : décompte en tours",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-timer-accounting-INTERACT-TIME01-03.json"),
                         ref("review-persistent-status-reapplication-INTERACT-TIME02-02.json")],
        "historical_unshown_draft": ref("review-timer-accounting-INTERACT-TIME01-01.json"),
        "historical_draft_boundary": "only_buff_and_new_skill_cooldown_draft_represented_here;historical_next_turn_periodic_damage_rules_superseded_and_not_restored",
        "scope": "ordinary_timed_buffs_and_new_skill_cooldowns;actual_elapsed_reference_game_time;not_complete_scheduler_or_familiar_lifetimes",
        "already_accepted": "same_count_used_by_menu_and_activation_rule;positive_cooldown_blocks_button_and_shortcut;UI_and_wall_clock_wait_do_not_advance_game_time",
        "proposed_rule": {
            "ordinary_buff": "applies_at_successful_activation_before_enemy_responses;activation_action_elapsed_time_counts_in_duration;duration3_one_turn_cast_covers_activation_turn_and_next_two_turns_then_expires_after_last_covered_turn",
            "new_skill_cooldown": "starts_at_successful_activation;counts_actual_game_time_elapsed_in_activation_action;cooldownN_after_one_turn_cast_is_max(0,N-1)",
            "zero_time_actions": "no_decrement_without_actual_game_time;menus_hover_target_selection_cancel_refused_actions_wall_clock_wait_and_zero_time_actions_do_not_advance_counters",
            "existing_UI": "same_remaining_counter_for_display_button_and_shortcut_block;positive_blocks;zero_removes_only_cooldown_block_other_conditions_still_apply",
            "preserved": "no_new_duration_or_cooldown_numbers;accepted_immediate_persistent_damage_counting_and_reapplication_unchanged;explicit_delays_channels_and_familiar_lifetimes_unchanged",
            "scope_boundary": "not_general_debuff_expiry_or_complete_actor_scheduler;fractional_display_rounding_and_native_feedback_not_newly_proposed"
        },
        "shown_examples": {"values_are_illustrative_not_skill_parameters": True,
                           "buff_duration3_one_turn_cast": {"active_turns": ["activation", "next", "following"], "remaining_after_activation_turn": 2},
                           "cooldown3_one_turn_cast": {"remaining_after_activation_turn": 2, "additional_full_turns_until_ready": 2}},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["complete_player_familiar_enemy_zone_death_and_expiry_order", "other_status_duration_and_Gel_provenance_cases", "fractional_display_rounding", "native_timer_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_contagion_zone_gel"] = deepcopy(data["pending_contagion_zone_gel"])
        data["pending_contagion_zone_gel"] = None
        data["accepted_contagion_zone_gel"] = ref(ACCEPTED)
        data["latest_contagion_zone_gel_acceptance"] = ref(DECISION)
        data["pending_buff_cooldown_accounting"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=23, status="zone_only_Gel_transfer_accepted;buff_and_cooldown_accounting_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-CONTAGION02"] = {"source": ref(ACCEPTED),
                "rule": "zone_only_Gel_reference_remaining_zone_time_capped5;AF06_retained_duration_floor_min1;independent_received_timer",
                "unshown": "multiple_source_Gel_provenance_and_other_retention_bindings"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith(("ordinary_buff_and_cooldown_accounting_unshown_draft", "Contagion_zone_only_Gel_duration_proposed_under_INTERACT_CONTAGION02"))]
                    group["remaining"].extend(["ordinary_buff_and_cooldown_accounting_proposed_under_INTERACT_TIME03",
                                               "other_unshown_Contagion_retention_bindings_geometry_ties_and_Gel_provenance_pending"])
            data["recommendation"] = "review_buff_cooldown_accounting_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL78", status="zone_only_Gel_transfer_accepted;buff_cooldown_accounting_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-23.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-81.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-23.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "ordinary_buff_and_new_skill_cooldown_activation_time_accounting"
            data["next_review_kind"] = "buff_cooldown_accounting_INTERACT_TIME03"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-81.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-23.json")
            data["latest_feedback"] = "user_accepts_zone_only_Gel_transfer_lifetime;buff_cooldown_accounting_proposed"
            data["next_review_kind"] = "buff_cooldown_accounting_INTERACT_TIME03"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("buff-cooldown-accounting-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION02-TIME03-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "CONTAGION02": "accepted", "TIME03": "proposal_only", "historical_next_turn_DoT_rules": "not_restored",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "zone_Gel": "accepted", "buff_cooldown_accounting": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
