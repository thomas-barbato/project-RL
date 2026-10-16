"""Design only: accept familiar arrival, propose timer accounting examples."""
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
SOURCE = "review-familiar-arrival-priority-INTERACT-ARRIVAL01-01.json"
DECISION = "familiar-arrival-priority-acceptance-decisions.json"
ACCEPTED = "review-familiar-arrival-priority-INTERACT-ARRIVAL01-02.json"
PROPOSAL = "review-timer-accounting-INTERACT-TIME01-01.json"
PAIRS = [("review-implementation-readiness-17.json", "review-implementation-readiness-18.json"),
         ("review-attribute-model-75.json", "review-attribute-model-76.json"),
         ("review-primary-catalogue-247.json", "review-primary-catalogue-248.json"),
         ("review-complete-skill-design-222.json", "review-complete-skill-design-223.json"),
         ("remaining-individual-skill-review-counts-116.json", "remaining-individual-skill-review-counts-117.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-TIME01 · Durées et recharges — proposition

Je propose que le tour d'activation compte dans la durée des buffs et dans la recharge.

- Buff de 3 tours : il agit pendant le tour où vous le lancez, puis pendant les 2 suivants. À la fin du tour de lancement, son icône affiche donc 2 tours restants.
- Recharge de 3 tours : à la fin du tour de lancement, il reste 2 tours. Après 2 autres tours de jeu, la compétence redevient utilisable si ses autres conditions sont remplies.
- Poison, Brûlure et Saignement : le premier dégât périodique arrive au tour suivant l'application. Une durée de 3 tours donne 3 applications de dégâts, si l'effet reste présent jusqu'au bout. Réappliquer le statut rétablit sa durée, sans ajouter de dégât périodique immédiat.

Les menus, survols et choix annulés ne font avancer aucun compteur. Les exemples à 3 tours expliquent le décompte ; ils ne changent pas les durées ou recharges chiffrées des compétences. Les délais spécifiques, canalisations et règles de durée des familiers restent conservés. Cette convention reste à valider."""


def add_override(data):
    data.setdefault("active_rule_overrides", {})["familiar_arrival_priority_and_cells"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "priority": "oldest_actual_invocation_or_raised_birth_first;no_drone_type_priority",
        "cells": "eight_immediate_visible_walkable_actor_free_player_neighbors;ordinary_ground_items_and_corpses_do_not_block_or_auto_collect",
        "arrival": "place_fitting_familiars_at_transition;one_pending_survivor_after_each_actual_player_step_if_valid_cell;no_retry_from_wait_attack_menu_or_refused_step",
        "further_transition": "carry_all_surviving_same_actors_including_pending_with_acquisition_priority_and_accepted_current_state;no_new_invocation_refill_or_duration_reset",
        "pending": "exact_turn_phase_and_arrival_hazards_and_native_feedback"
    }


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-ARRIVAL01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait on peut continuer :)",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_acquisition_priority_eight_neighbor_cells_progressive_arrival_and_further_transition;no_new_native_feedback_or_exact_global_turn_order",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="familiar_arrival_priority_neighbor_geometry_and_further_transfer_explicitly_accepted;phase_and_hazard_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_name_or_UI_error_copy")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME01", "revision": 1,
        "title": "Durées et recharges : décompte en tours",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-02.json"),
                         ref("review-overlapping-zone-status-INTERACT-ZONE02-02.json"),
                         ref("review-old-zone-status-refresh-INTERACT-ZONE03-02.json"),
                         ref("review-individual-description-FE01-02.json"),
                         ref("review-individual-description-SM05-02.json"),
                         ref("last-status-cast-power-acceptance-decisions.json")],
        "scope": "ordinary_timed_buffs_newly_started_skill_cooldowns_and_actor_Poison_Burn_Bleed_periodic_ticks;not_complete_actor_scheduler_or_zone_cadence",
        "already_accepted": "timers_follow_actual_game_time_not_menus;remaining_cooldown_controls_same_button_and_shortcut_rule;status_reapplication_restarts_duration_latest_successful_power_wins;explicit_delays_and_familiar_lifetimes_preserved",
        "proposed_rule": {
            "ordinary_timed_buff": "activation_turn_counts_as_first_active_turn;effect_applies_before_enemy_responses;decrement_after_covering_that_turn;durationN_yields_N_active_turns_including_activation_turn",
            "new_skill_cooldown": "begins_at_successful_activation_and_counts_elapsed_game_time_of_activation_action;one_ordinary_turn_action_reduces_new_cooldownN_to_max(0,N-1)_by_turn_end",
            "actor_Poison_Burn_Bleed": "first_periodic_tick_from_game_turn_after_new_application;do_not_consume_periodic_duration_before_first_eligible_tick;durationN_yields_N_ticks_if_not_removed_consumed_or_reapplied",
            "status_refresh": "restart_accepted_full_duration_and_replace_power_under_existing_rules;refresh_itself_adds_no_instant_periodic_damage;no_extra_same_kind_periodic_damage_tick_from_refresh",
            "game_time": "opening_UI_hover_target_selection_cancellation_refused_action_and_wall_clock_wait_do_not_advance_timers;actual_actions_consume_reference_game_time_under_existing_rules",
            "scope_boundary": "no_new_skill_duration_cooldown_or_damage_numbers;no_healing_channel_or_zone_tick_cadence_change;no_reset_of_inherited_raised_cooldowns_or_new_familiar_lifetime_extension"
        },
        "shown_examples": {
            "values_are_illustrative_not_new_skill_parameters": True,
            "buff_duration3": {"active_turns": ["activation", "next", "following"], "remaining_after_activation_turn": 2},
            "skill_cooldown3": {"remaining_after_activation_turn": 2, "additional_full_turns_until_ready": 2},
            "periodic_duration3": {"first_damage": "following_turn", "ticks_if_untouched": 3}
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["complete_phase_order_of_player_familiars_enemies_zones_periodic_damage_death_expiry_and_arrival",
                              "already_active_periodic_status_refresh_vs_scheduled_tick_within_same_turn",
                              "source_provenance_and_zone_only_Gel_lifetime_merging", "native_timer_animation_and_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_name_or_UI_error_copy",
        "visual_approval": "no_new_native_art_animation_or_UI_layout_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data["date"] = "2026-10-09"
        data["previous_pending_familiar_arrival_priority"] = deepcopy(data["pending_familiar_arrival_priority"])
        data["pending_familiar_arrival_priority"] = None
        data["accepted_familiar_arrival_priority"] = ref(ACCEPTED)
        data["latest_familiar_arrival_priority_acceptance"] = ref(DECISION)
        data["pending_timer_accounting"] = ref(PROPOSAL)
        data["runtime_integration"] = "not_integrated"
        if old_name.startswith("review-implementation"):
            data.update(revision=18, status="familiar_arrival_priority_accepted;timer_accounting_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-ARRIVAL01"] = {
                "rule": "oldest_familiar_first;eight_visible_walkable_actor_free_neighbor_cells;one_pending_arrival_per_actual_step;all_surviving_actors_follow_further_transition_with_priority",
                "source": ref(ACCEPTED), "unshown": "exact_turn_phase_arrival_hazards_and_native_feedback"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "arrival_priority_neighbor_geometry_and_further_map_transfer_proposed_under_INTERACT_ARRIVAL01"]
                    group["remaining"].append("ordinary_buff_cooldown_and_actor_periodic_timer_accounting_proposed_under_INTERACT_TIME01")
            data["remaining_after_user_directions"][3] = "arrival_exact_turn_phase_and_hazards;priority_geometry_further_transition_accepted;other_drone_combat_orders_pending"
            data["recommendation"] = "review_timer_accounting_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission;future_NPC_cases_deferred"
        elif old_name.startswith("review-attribute"):
            add_override(data)
            data.update(review_id="STAT-MODEL73", status="familiar_arrival_priority_accepted;timer_accounting_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-18.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            add_override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-76.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-18.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "ordinary_buff_duration_new_skill_cooldown_and_actor_periodic_damage_timer_accounting;not_full_actor_scheduler"
            data["next_review_kind"] = "timer_accounting_INTERACT_TIME01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-76.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-18.json")
            data["latest_feedback"] = "user_accepts_ARRIVAL01;timer_accounting_proposed"
            data["next_review_kind"] = "timer_accounting_INTERACT_TIME01"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-ARRIVAL01", "proposed": "INTERACT-TIME01", "runtime": "not_integrated"}))


def check():
    checked, errors = [], []
    def walk(value, context):
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
                            errors.append((context, "missing", source))
                            continue
                    checked.append((context, key))
                    if hashlib.sha256(content.encode("utf-8")).hexdigest() != expected:
                        errors.append((context, key, source[:100]))
            for key, nested in value.items():
                walk(nested, context + "/" + key)
        elif isinstance(value, list):
            for i, nested in enumerate(value):
                walk(nested, context + "/" + str(i))
    for name in NEW_FILES:
        walk(load(name), name)
    for old_name, new_name in PAIRS:
        old, new = load(old_name), load(new_name)
        preserve(old, new)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert new["active_rule_overrides"][key] == value
        assert new["pending_familiar_arrival_priority"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-117.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("arrival-and-timer-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-ARRIVAL01-TIME01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "ARRIVAL01": "accepted", "TIME01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "arrival_priority": "accepted", "timer_accounting": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
