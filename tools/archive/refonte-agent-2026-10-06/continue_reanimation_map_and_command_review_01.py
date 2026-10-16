"""Design records: accept pending-mark cleanup, propose familiar command costs."""
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
SOURCE = "review-reanimation-mark-map-and-owner-loss-INTERACT-REANIM04-01.json"
DECISION = "reanimation-mark-map-and-owner-loss-acceptance-decisions.json"
ACCEPTED = "review-reanimation-mark-map-and-owner-loss-INTERACT-REANIM04-02.json"
PROPOSAL = "review-familiar-command-action-costs-INTERACT-CMD01-01.json"
PAIRS = [("review-implementation-readiness-15.json", "review-implementation-readiness-16.json"),
         ("review-attribute-model-73.json", "review-attribute-model-74.json"),
         ("review-primary-catalogue-245.json", "review-primary-catalogue-246.json"),
         ("review-complete-skill-design-220.json", "review-complete-skill-design-221.json"),
         ("remaining-individual-skill-review-counts-114.json", "remaining-individual-skill-review-counts-115.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-CMD01 · Commandes de familiers — proposition

Je propose un coût de 1 action du joueur et 0 énergie pour chacune de ces commandes, en combat comme au calme :

- Repositionner : déplace le drone selon les conditions validées. Recharge conservée : 6 tours par drone.
- Détruire : supprime le familier choisi. Ses objets tombent sur sa case, ou sur celle du joueur s'il attend son arrivée. Cette commande ne provoque pas d'explosion.
- Ramasser : donne l'ordre de collecte à la Mule. Elle continue ensuite sur ses propres tours.
- Arrêter : interrompt la collecte de la Mule et lui fait reprendre votre suivi.

Ouvrir la fenêtre Familiers, consulter une fiche ou préparer un choix reste gratuit. Une commande impossible ne consomme ni action ni énergie et ne lance aucune recharge.

Détruire ne réinitialise pas la recharge de la compétence d'invocation. Ces coûts de commandes restent à valider."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_reanimation_mark_map_and_owner_loss"] = deepcopy(data["pending_reanimation_mark_map_and_owner_loss"])
    data["pending_reanimation_mark_map_and_owner_loss"] = None
    data["accepted_reanimation_mark_map_and_owner_loss"] = ref(ACCEPTED)
    data["latest_reanimation_mark_map_and_owner_loss_acceptance"] = ref(DECISION)
    data["pending_familiar_command_action_costs"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["pending_reanimation_mark_map_and_owner_loss"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "same_map": "distance_or_visibility_loss_does_not_remove_pending_mark",
        "owner_map_departure": "successful_owner_map_departure_cancels_own_pending_marks_on_departure_map;includes_descent_refuge_demonic_bonus_portal_entry_and_return;no_restore_on_return",
        "owner_death_or_removal": "actual_owner_death_or_simulation_removal_cancels_its_pending_marks;not_invisibility_or_temporary_inability_to_cast",
        "scope": "pending_OC07_marks_only;existing_living_raised_map_transfer_state_items_waiting_arrival_and_remaining_lifetime_preserved;no_NPC_ability_assignment"
    }


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM04-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "oui parfait",
        "approved_rule": source["proposed_rule"],
        "shown_explanation": "Sur la même carte, s'éloigner ou perdre la cible de vue conserve sa marque. Changer de carte annule vos marques sur la carte quittée : descente, refuge ou portail démoniaque, à l'aller comme au retour. Revenir ne les restaure pas. Si le lanceur meurt ou disparaît définitivement, ses marques en attente sont annulées. Les familiers déjà ressuscités continuent à vous accompagner selon les règles validées, en conservant leur durée restante.",
        "approval_boundary": "pending_marks_same_map_retention_and_owner_map_departure_death_removal_cleanup;already_raised_map_transfer_preserved;no_NEW_NPC_skill_assignment_or_native_feedback",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="pending_reanimation_mark_map_and_owner_cleanup_explicitly_accepted;remaining_turn_and_future_NPC_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_name_or_UI_error_copy")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-CMD01", "revision": 1,
        "title": "Commandes de familiers : coût en actions",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-drone-recall-command-PETS-EQ15-03.json"),
                         ref("review-mule-loot-collection-PETS-EQ16-02.json"),
                         ref("review-waiting-familiar-controls-PETS-EQ21-02.json"),
                         ref("review-familiar-window-UX-PETS-02-03.json"),
                         ref("review-familiar-equipment-transfers-PETS-EQ05-02.json")],
        "scope": "agent_action_cost_for_existing_Repositionner_Destroy_Mule_Ramasser_and_Arrêter_controls;not_general_drone_combat_orders_or_new_ability",
        "already_accepted": "Repositionner_cost0_energy_cooldown6_per_drone_and_current_map_conditions;Mule_collection_cost0_no_own_cooldown_and_own_movement_pickup_turns;Destroy_available_for_waiting_and_deployed_familiars_with_real_item_drops;inspection_free;equipment_transfers_keep_their_accepted_costs",
        "proposed_rule": {
            "Repositionner": {"agent_actions": 1, "energy": 0, "cooldown_turns_per_drone": 6},
            "Destroy": {"agent_actions": 1, "energy": 0, "own_cooldown_turns": 0,
                        "result": "remove_selected_existing_familiar;real_equipment_and_Mule_cargo_drop_on_actor_cell_or_player_current_cell_if_pending;no_explosion_or_damage;no_invocation_cooldown_reset"},
            "Mule_Ramasser": {"agent_actions": 1, "energy": 0, "own_cooldown_turns": 0,
                              "result": "start_accepted_collection_order;Mule_uses_ordinary_own_turns_no_extra_agent_actions_per_pickup"},
            "Mule_Arrêter": {"agent_actions": 1, "energy": 0, "own_cooldown_turns": 0,
                            "result": "cancel_current_collection_order_and_resume_follow"},
            "phase": "valid_commands_allowed_in_combat_accalmie_or_calme_under_existing_presence_conditions",
            "inspection_or_selection": "opening_existing_window_sheet_or_noncommitting_target_selection_consumes_no_action_energy_or_cooldown",
            "refusal": "invalid_command_changes_no_actor_order_items_energy_turn_or_cooldown;no_new_UI_error_copy_defined",
            "existing_invocation": "Destroy_does_not_reset_invocation_remaining_cooldown;new_invocation_keeps_existing_requirements_and_new_actor_full_reserves",
            "scope_boundary": "no_new_range_gate_for_Destroy;other_drone_orders_exact_scheduler_and_native_interface_feedback_pending"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["other_drone_combat_order_action_costs", "exact_actor_phase_order_after_committed_command",
                              "native_command_tooltip_layout_disabled_reasons_and_confirmation_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;existing_command_names_retained",
        "visual_approval": "no_new_native_art_animation_or_window_layout_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=16, status="pending_reanimation_mark_map_owner_cleanup_accepted;familiar_command_action_costs_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-REANIM04"] = {
                "rule": "same_map_distance_visibility_retains_marks;owner_successful_map_departure_death_or_removal_cancels_own_pending_marks;existing_raised_transfer_preserved",
                "source": ref(ACCEPTED), "unshown": "exact_turn_order_and_future_nonplayer_actor_cases"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith("pending_reanimation_mark_map_and_owner_cleanup_proposed_under_INTERACT_REANIM04") and s != "command_action_budget_for_Destroy_and_Mule_start_stop_where_not_shown"]
                    group["remaining"].append("familiar_Repositionner_Destroy_and_Mule_start_stop_action_costs_proposed_under_INTERACT_CMD01;other_drone_combat_orders_pending")
            data["remaining_after_user_directions"][2] = "reanimation_remaining_exact_turn_cases;future_NPC_target_transition_and_existing_raised_owner_death_cases_deferred_until_relevant_ability_design"
            data["remaining_after_user_directions"][3] = "command_action_costs_proposed_under_INTERACT_CMD01_and_arrival_order"
            data["recommendation"] = "review_familiar_command_action_costs_and_remaining_core_turn_cases_then_lock_class_starts_and_first_mission;do_not_expand_unassigned_NPC_ability_cases"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL71", status="reanimation_mark_map_owner_cleanup_accepted;familiar_command_costs_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-16.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-74.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-16.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "agent_action_costs_for_existing_familiar_Repositionner_Destroy_and_Mule_collection_start_stop_commands"
            data["next_review_kind"] = "familiar_command_action_costs_INTERACT_CMD01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-74.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-16.json")
            data["latest_feedback"] = "user_accepts_REANIM04;familiar_command_action_costs_proposed"
            data["next_review_kind"] = "familiar_command_action_costs_INTERACT_CMD01"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-REANIM04", "proposed": "INTERACT-CMD01", "runtime": "not_integrated"}))


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
        assert new["pending_reanimation_mark_map_and_owner_loss"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-115.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("reanimation-map-and-command-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM04-CMD01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "REANIM04": "accepted", "CMD01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "mark_map_owner_cleanup": "accepted", "command_costs": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
