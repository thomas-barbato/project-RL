"""Design only: accept familiar command costs, propose arrival priority and cells."""
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
SOURCE = "review-familiar-command-action-costs-INTERACT-CMD01-01.json"
DECISION = "familiar-command-action-costs-acceptance-decisions.json"
ACCEPTED = "review-familiar-command-action-costs-INTERACT-CMD01-02.json"
PROPOSAL = "review-familiar-arrival-priority-INTERACT-ARRIVAL01-01.json"
PAIRS = [("review-implementation-readiness-16.json", "review-implementation-readiness-17.json"),
         ("review-attribute-model-74.json", "review-attribute-model-75.json"),
         ("review-primary-catalogue-246.json", "review-primary-catalogue-247.json"),
         ("review-complete-skill-design-221.json", "review-complete-skill-design-222.json"),
         ("remaining-individual-skill-review-counts-115.json", "remaining-individual-skill-review-counts-116.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-ARRIVAL01 · Arrivée des familiers — proposition

- Les familiers arrivent du plus ancien au plus récent, selon leur invocation ou leur résurrection, sans priorité particulière pour les drones.
- Ils apparaissent sur l'une des huit cases voisines du joueur, libre de tout personnage, praticable et visible. Les objets et cadavres au sol n'empêchent pas l'arrivée.
- Après chaque déplacement réellement effectué, un seul familier en attente arrive si une case convient. Attendre ou attaquer ne déclenche pas cette arrivée.
- Si vous changez encore de carte avant que tous soient arrivés, tous continuent à vous accompagner. Ceux qui peuvent entrer arrivent ; les autres restent en attente, avec le même ordre de priorité.

Leurs PV, leur énergie, leur équipement, leurs effets, leurs recharges et la durée restante des ressuscités suivent les règles déjà validées. Ces précisions d'arrivée restent à valider."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_familiar_command_action_costs"] = deepcopy(data["pending_familiar_command_action_costs"])
    data["pending_familiar_command_action_costs"] = None
    data["accepted_familiar_command_action_costs"] = ref(ACCEPTED)
    data["latest_familiar_command_action_cost_acceptance"] = ref(DECISION)
    data["pending_familiar_arrival_priority"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["familiar_command_action_costs"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "cost": "Repositionner_Destroy_Mule_Ramasser_and_Arrêter_each_cost_one_committed_agent_action_zero_energy_in_combat_accalmie_or_calme",
        "cooldowns": "Repositionner_keeps_six_turn_per_drone_cooldown;Destroy_Mule_Ramasser_Arrêter_have_no_own_cooldown;Destroy_does_not_reset_invocation_cooldown",
        "inspection_and_refusal": "inspection_and_noncommitting_selection_free;invalid_command_no_state_items_energy_turn_or_cooldown_change",
        "destruction": "no_explosion_or_damage;real_worn_equipment_and_Mule_cargo_drop_on_actor_cell_or_player_cell_if_pending",
        "preserved": "existing_presence_reposition_and_collection_conditions;Mule_collection_uses_own_normal_turns;raised_remain_autonomous"
    }


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-CMD01-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait :)",
        "approved_rule": source["proposed_rule"],
        "shown_explanation": "Repositionner, Détruire, Ramasser et Arrêter coûtent chacune 1 action du joueur et 0 énergie, en combat comme au calme. Repositionner conserve sa recharge de 6 tours par drone. Détruire supprime le familier sans explosion et dépose ses objets sur sa case, ou sur celle du joueur s'il attend son arrivée. Ramasser lance la collecte sur les propres tours de la Mule ; Arrêter l'interrompt et rétablit le suivi. Détruire, Ramasser et Arrêter n'ont pas de recharge propre. Détruire ne réinitialise pas la recharge d'invocation. Consulter les fiches et préparer un choix reste gratuit ; une commande impossible ne consomme rien.",
        "approval_boundary": "displayed_command_costs_cooldowns_destruct_without_explosion_inspection_and_refusal;no_new_other_drone_combat_order_cost_native_layout_or_exact_actor_phase_order",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="shown_familiar_command_costs_and_no_damage_destruction_explicitly_accepted;other_orders_and_phase_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;existing_command_names_retained")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-ARRIVAL01", "revision": 1,
        "title": "Arrivée des familiers : priorité et cases voisines",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-familiar-map-transitions-PETS-EQ18-02.json"),
                         ref("review-pending-familiar-arrival-PETS-EQ19-02.json"),
                         ref("review-waiting-familiar-controls-PETS-EQ21-02.json"), ref(ACCEPTED)],
        "scope": "placement_priority_neighbor_geometry_and_further_map_transfer_of_existing_controlled_familiars;not_new_invocation_or_general_drone_combat_AI",
        "already_accepted": "player_transition_never_blocked_by_familiar_space;fit_familiars_enter_remaining_wait;one_waiting_familiar_per_actual_player_step_if_cell_available;same_identity_real_objects_and_current_state;waiting_timers_and_DoT_continue_no_map_actions_vision_protection_or_passive_regeneration",
        "proposed_rule": {
            "priority": "oldest_actual_becoming_owned_familiar_first;drone_successful_invocation_or_raised_actual_birth_not_mark_cast;no_drone_vs_raised_type_priority",
            "arrival_cells": "one_of_eight_immediate_player_neighbor_cells;current_map_walkable_visible_and_unoccupied_by_any_actor;distinct_cells_for_each_placed_familiar;player_cell_not_candidate",
            "ground_items_and_corpses": "ordinary_ground_item_piles_and_corpses_do_not_by_themselves_block_placement;real_items_remain_on_ground_without_auto_pickup",
            "map_entry": "place_as_many_existing_surviving_familiars_as_valid_neighbor_cells_allow_in_priority_order;rest_wait_without_blocking_player",
            "progressive_arrival": "after_each_successful_actual_player_grid_movement_place_at_most_one_waiting_surviving_familiar_when_valid_neighbor_cell_available;no_retry_from_attack_wait_menu_or_refused_movement",
            "further_map_transition": "carry_all_same_surviving_owned_familiars_including_pending_ones_to_next_destination;apply_same_placement_and_waiting_rules_and_unchanged_acquisition_priority;no_reset_or_new_invocation",
            "state": "preserve_all_previously_accepted_current_HP_energy_real_equipment_cargo_effect_cooldown_and_raised_remaining_lifetime_rules;ordinary_waiting_death_expiry_cleanup_retained",
            "scope_boundary": "arrival_does_not_grant_extra_actor_action_or_new_birth;hazard_application_and_exact_turn_phase_still_pending;no_new_UI_copy_or_native_effect"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["exact_turn_phase_placement_vs_damage_expiry_and_other_actor_actions",
                              "arrival_into_terrain_hazards_mines_or_persistent_zones_under_global_scheduler",
                              "native_arrival_feedback_and_UI_queue_presentation"],
        "technical_choices": "stable_identity_acquisition_order_and_deterministic_selection_between_equally_valid_neighbor_cells_may_be_chosen_in_implementation_without_new_mechanical_effect",
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_name_or_UI_error_copy",
        "visual_approval": "no_new_native_art_animation_or_window_layout_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=17, status="familiar_command_costs_accepted;arrival_priority_and_geometry_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-CMD01"] = {
                "rule": "one_agent_action_zero_energy_for_Repositionner_Destroy_Mule_start_stop;reposition_cooldown6_per_drone;other_commands_no_own_cooldown;inspection_refusal_free;Destroy_no_damage_no_invocation_CD_reset",
                "source": ref(ACCEPTED), "unshown": "other_drone_combat_orders_and_exact_actor_phase_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "arrival_placement_queue_order_and_geometry_where_not_shown" and not s.startswith("familiar_Repositionner_Destroy_and_Mule_start_stop_action_costs_proposed_under_INTERACT_CMD01")]
                    group["remaining"].extend(["arrival_priority_neighbor_geometry_and_further_map_transfer_proposed_under_INTERACT_ARRIVAL01",
                                               "other_drone_combat_order_action_costs_where_not_shown"])
            data["remaining_after_user_directions"][3] = "arrival_priority_and_geometry_proposed_under_INTERACT_ARRIVAL01;other_drone_combat_orders_pending"
            data["recommendation"] = "review_familiar_arrival_priority_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission;future_NPC_ability_cases_remain_deferred"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL72", status="familiar_command_costs_accepted;arrival_priority_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-17.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-75.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-17.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "existing_familiar_arrival_acquisition_priority_neighbor_cells_and_further_map_transfer"
            data["next_review_kind"] = "familiar_arrival_priority_INTERACT_ARRIVAL01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-75.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-17.json")
            data["latest_feedback"] = "user_accepts_CMD01;familiar_arrival_priority_and_neighbor_geometry_proposed"
            data["next_review_kind"] = "familiar_arrival_priority_INTERACT_ARRIVAL01"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-CMD01", "proposed": "INTERACT-ARRIVAL01", "runtime": "not_integrated"}))


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
        assert new["pending_familiar_command_action_costs"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-116.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("familiar-commands-and-arrival-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-CMD01-ARRIVAL01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "CMD01": "accepted", "ARRIVAL01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "command_costs": "accepted", "arrival_priority": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
