"""Design records only: accept familiar arrival; propose basic drone order costs."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-familiar-zone-arrival-INTERACT-PETS02-01.json"
DECISION = "familiar-zone-arrival-acceptance-decisions.json"
ACCEPTED = "review-familiar-zone-arrival-INTERACT-PETS02-02.json"
PROPOSAL = "review-drone-combat-order-costs-INTERACT-CMD02-01.json"
PAIRS = [("review-implementation-readiness-29.json", "review-implementation-readiness-30.json"),
         ("review-attribute-model-87.json", "review-attribute-model-88.json"),
         ("review-primary-catalogue-259.json", "review-primary-catalogue-260.json"),
         ("review-complete-skill-design-234.json", "review-complete-skill-design-235.json"),
         ("remaining-individual-skill-review-counts-128.json", "remaining-individual-skill-review-counts-129.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-CMD02 · Ordres de combat des drones — proposition

- Changer le mode d'un drone — Suivre, Défensif, Agressif ou Passif : 0 tour, 0 énergie, aucune recharge propre.
- Désigner une cible à un drone capable d'attaquer : 0 tour, 0 énergie, aucune recharge propre. Les conditions habituelles de présence, de visibilité et d'attaque restent applicables.

Le nouvel ordre est enregistré, puis le drone l'exécute pendant sa prochaine phase normale. Donner ou modifier un ordre ne lui accorde pas d'attaque ou de déplacement supplémentaire et ne réinitialise pas ses recharges.

Vous pouvez donc préparer les ordres de plusieurs drones avant votre prochaine action. Une commande impossible est refusée gratuitement, sans modifier l'ordre précédent. Un familier en attente conserve les commandes grisées déjà validées.

Exemple : vous désignez le même ennemi à deux drones, puis vous attaquez. Les drones appliquent chacun leur ordre lorsqu'ils peuvent agir pendant la phase des familiers.

Repositionner, Détruire, Ramasser et Arrêter conservent leur coût accepté d'une action du joueur et leurs autres conditions. Changer un mode de combat ne met pas fin à la collecte de la Mule : son interruption passe par Arrêter. Les ressuscités restent autonomes.

Ces coûts restent proposés, en attente de validation."""


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
        assert new["pending_familiar_zone_arrival"] is None
        assert new["pending_drone_combat_order_costs"] == ref(PROPOSAL)
        assert "drone_combat_order_costs" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-129.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-PETS02-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "parfait :) on continue",
        "approved_rule": deepcopy(source["proposed_rule"]),
        "approval_boundary": "displayed_eligible_zone_arrival_reposition_application_budget_death_items_and_progressive_arrival_phase_accepted;native_feedback_and_other_commands_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="shown_familiar_zone_arrival_rules_accepted;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;no_new_individual_RP_or_name")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-CMD02", "revision": 1,
        "title": "Ordres de combat des drones", "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-familiar-command-action-costs-INTERACT-CMD01-02.json"),
                         ref("review-reposition-drone-orders-PETS-EQ20-02.json"),
                         ref("review-familiar-window-UX-PETS-02-03.json"),
                         ref("review-waiting-familiar-controls-PETS-EQ21-02.json"),
                         ref("review-familiar-first-action-arrival-INTERACT-PETS01-02.json"), ref(ACCEPTED)],
        "scope": "basic_existing_drone_mode_selection_and_explicit_target_order_costs_per_drone;not_new_abilities_or_AI_engagement_distances",
        "existing_runtime_evidence": {
            "mode_labels": "src/ascii_app.rs:companion_behavior_label;Suivre_Defensif_Agressif_Passif",
            "mode_change_time": "src/game/game_state.rs:process_player_command_SetCompanionBehavior_returns_AppliedWithoutTime",
            "approved_window_direction": "one_row_and_existing_controls_per_drone;raised_autonomous"
        },
        "proposed_rule": {
            "mode_selection": {"modes": ["Suivre", "Défensif", "Agressif", "Passif"], "player_actions": 0, "energy": 0, "own_cooldown_turns": 0},
            "target_order": {"scope": "chosen_attack_capable_drone_and_currently_observed_hostile_under_existing_targeting_conditions", "player_actions": 0, "energy": 0, "own_cooldown_turns": 0},
            "execution": "store_new_accepted_order;execute_when_drone_normally_eligible_in_familiar_phase;no_immediate_forced_action_or_extra_action_budget_or_recharge_reset",
            "multiple_orders": "player_can_configure_multiple_drone_rows_before_next_time_advancing_action;orders_alone_advance_no_turn_effect_timer_cooldown_enemy_phase_or_regeneration",
            "eligibility": "preserve_existing_presence_current_hostile_observation_actual_drone_vision_attack_and_model_role_conditions;no_attack_granted_to_noncombat_model;pending_familiar_map_commands_remain_disabled",
            "refusal": "invalid_order_preserves_previous_order_and_consumes_no_time_energy_cooldown_or_RNG",
            "preserved_commands": "Repositionner_Destroy_Mule_Ramasser_Mule_Arreter_keep_accepted_CMD01_one_player_action_and_other_costs_conditions",
            "Mule_collection": "combat_mode_selection_does_not_cancel_existing_Mule_collection;stopping_collection_uses_existing_one_action_Arreter_command",
            "preserved_AI": "same_mode_meanings_target_rules_and_natural_actor_budgets;no_new_pursuit_range_target_persistence_or_AI_priority",
            "raised": "autonomous_without_combat_order_buttons;inspection_and_Destroy_preserved",
            "scope_boundary": "no_new_skill_ability_button_mode_name_native_layout_animation_or_player_facing_error_copy"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "shown_example": "two_drones_configured_to_same_valid_enemy_before_player_attack;each_acts_only_in_its_normal_eligible_familiar_phase",
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;existing_command_labels_retained",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_familiar_zone_arrival"] = deepcopy(data["pending_familiar_zone_arrival"])
        data["pending_familiar_zone_arrival"] = None
        data["accepted_familiar_zone_arrival"] = ref(ACCEPTED)
        data["latest_familiar_zone_arrival_acceptance"] = ref(DECISION)
        data["pending_drone_combat_order_costs"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=30, status="familiar_zone_arrival_accepted;basic_drone_combat_order_costs_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-PETS02"] = {"source": ref(ACCEPTED),
                "rule": "eligible_zone_entry_effects_immediate_shared_budget;arrival_death_before_action_and_real_item_drop;progressive_arrival_after_actual_player_step_before_periodic_and_familiar_phases"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "familiar_zone_arrival_and_progressive_arrival_phase_proposed_under_INTERACT_PETS02"]
                    group["remaining"] = ["basic_drone_combat_order_costs_proposed_under_INTERACT_CMD02" if s == "other_drone_combat_order_action_costs_where_not_shown" else s for s in group["remaining"]]
            data["recommendation"] = "review_basic_drone_combat_order_costs_then_complete_necessary_interactions_and_class_starts"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["familiar_zone_arrival"] = {
                    **ref(ACCEPTED), "acceptance": ref(DECISION), "rule": deepcopy(source["proposed_rule"])}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL85", status="familiar_zone_arrival_accepted;basic_drone_combat_order_costs_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", PROPOSAL)
                data["latest_proposal_scope"] = "free_basic_drone_mode_and_target_order_configuration;normal_familiar_phase_execution"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "familiar_zone_arrival_accepted;basic_drone_combat_order_costs_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-30.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-88.json")
            data["next_review_kind"] = "drone_combat_order_costs_INTERACT_CMD02"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("drone-combat-order-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-PETS02-CMD02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "PETS02": "accepted", "CMD02": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "zone_arrival": "accepted", "drone_combat_orders": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
