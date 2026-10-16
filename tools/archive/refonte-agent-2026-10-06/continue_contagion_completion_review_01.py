"""Design records only: accept drone orders and propose remaining Contagion bindings."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-drone-combat-order-costs-INTERACT-CMD02-01.json"
DECISION = "drone-combat-order-costs-acceptance-decisions.json"
ACCEPTED = "review-drone-combat-order-costs-INTERACT-CMD02-02.json"
PROPOSAL = "review-contagion-retention-targeting-INTERACT-CONTAGION03-01.json"
PAIRS = [("review-implementation-readiness-30.json", "review-implementation-readiness-31.json"),
         ("review-attribute-model-88.json", "review-attribute-model-89.json"),
         ("review-primary-catalogue-260.json", "review-primary-catalogue-261.json"),
         ("review-complete-skill-design-235.json", "review-complete-skill-design-236.json"),
         ("remaining-individual-skill-review-counts-129.json", "remaining-individual-skill-review-counts-130.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-CONTAGION03 · Contagion : puissance, durée et choix de cible — proposition

- Durée : Contagion conserve une part du temps restant, arrondie à l'entier inférieur, avec un minimum de 1 tour si la transmission réussit. Elle ne redonne pas la durée complète d'origine. Le Gel entretenu par une zone garde sa règle déjà validée.
- Puissance : le même taux s'applique aux dégâts périodiques, aux réductions chiffrées et à la puissance servant à appliquer l'effet. Les bonus déjà inclus ne sont pas ajoutés une seconde fois. Les règles d'application et les défenses de la nouvelle cible restent applicables, avec leurs exceptions validées.
- Contrôles : Silence, Désorientation, Gel et Rupture de régénération gardent leur fonctionnement ; leur durée et leur puissance d'application sont atténuées selon les règles précédentes.
- Choix de cible : portée comptée depuis la victime en cases, une diagonale comptant pour une case. Les murs bloquent la transmission directe. À égalité de distance, le départage suit l'ordre des cases, de haut en bas puis de gauche à droite.

Exemple au rang effectif 11, avec 80 % de conservation : un Poison encore actif pour 5 applications, infligeant 40 dégâts par application avant résistance, est transmis pour 4 applications de 32 dégâts avant les défenses de la nouvelle cible. La première application est immédiate et compte parmi ces quatre, comme validé.

Les dégâts restent entiers. Les effets dont les dégâts reposent déjà sur des dégâts réellement subis gardent leur exception de défense : ils ne passent pas une deuxième fois dans les défenses. Les réductions en pourcentage gardent leur précision de calcul.

Si plusieurs ennemis meurent pendant une attaque de zone, leurs morts suivent l'ordre des impacts déjà prévu. Chaque mort utilise l'état courant de la carte ; une cible déjà morte ne peut plus recevoir l'effet. Une seule tentative par mort et aucune retransmission d'un effet reçu par Contagion restent conservées.

Ces précisions restent proposées, en attente de validation."""


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
        assert new["pending_drone_combat_order_costs"] is None
        assert new["pending_contagion_retention_targeting"] == ref(PROPOSAL)
        assert "contagion_retention_targeting" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-130.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-CMD02-ACCEPT", "source_proposal": ref(SOURCE),
        "user_acceptance": "oui ca me va :)", "approved_rule": deepcopy(source["proposed_rule"]),
        "approval_boundary": "displayed_free_mode_and_target_orders_normal_phase_execution_no_extra_actions_Mule_collection_separation_accepted;new_AI_ranges_and_native_feedback_not_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="shown_basic_drone_combat_order_costs_accepted;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;existing_labels_retained")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-CONTAGION03", "revision": 1,
        "title": "Contagion : puissance, durée et choix de cible", "skill_id": "AF06",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-contagion-transfer-rules-INTERACT-CONTAGION01-02.json"),
                         ref("review-contagion-zone-gel-INTERACT-CONTAGION02-02.json"),
                         ref("review-gel-source-lifetime-INTERACT-ZONE05-02.json"),
                         ref("review-turn-resolution-INTERACT-TURN01-02.json"),
                         ref("skill-rank-curves-provisional-02.json")],
        "scope": "remaining_AF06_duration_potency_binding_rounding_selection_geometry_and_death_event_continuity;not_new_rank_curves_or_global_damage_defense_rules",
        "proposed_rule": {
            "retained_percent": "current_AF06_effective_rank_delegated_curve;unchanged_50_plus3_per_rank_after1_capped89",
            "ordinary_duration": "source_current_remaining_positive_duration_or_remaining_periodic_application_count_times_retained_percent;floor_to_whole_turn_or_application;minimum1_on_success;never_above_source_remaining;not_original_full_lifetime",
            "zone_only_Gel": "preserve_accepted_CONTAGION02_reference_and_cap5_current_source_zone_rule;received_Gel_keeps_independent_timer",
            "damage_potency": "scale_selected_source_stored_periodic_damage_by_retained_percent;source_damage_defense_mode_and_element_retained;ordinary_pre_defense_damage_uses_new_recipient_current_defenses_once;already_post_actual_damage_derived_direct_DoT_preserves_no_second_defense_exception;final_damage_floor_integer_under_existing_pipeline",
            "numeric_debuff": "scale_selected_source_stored_damage_or_armor_reduction_by_same_retained_percent;percentage_precision_preserved_for_calculation_and_existing_display_rounding",
            "application_strength": "when_source_uses_numeric_application_power_scale_stored_pre_recipient_power_by_retained_percent_floor_min1;when_source_uses_explicit_percentage_chance_scale_stored_pre_recipient_chance_by_retained_percent_preserving_existing_caps_and_target_check;no_new_random_resistance_check_for_source_effect_with_no_such_check",
            "target_defenses": "apply_only_existing_status_specific_current_recipient_resistance_and_importance_checks;source_recipient_resistance_or_importance_penalty_not_inherited_or_applied_twice;no_new_Poison_Burn_resistance_chance_formula",
            "caster_modifiers": "source_status_already_includes_original_caster_level_attributes_equipment_and_launch_power;do_not_add_them_again_or_recalculate_original_spell_at_transfer",
            "binary_effects": "Silence_Disorientation_Gel_Regeneration_Rupture_behavior_preserved;duration_and_existing_application_strength_attenuated;no_partial_silence_or_partial_immobility",
            "geometry": "same_map_victim_to_candidate_Chebyshev_grid_distance_max_abs_dx_abs_dy;diagonal_one_cell;within_current_AF06_range;direct_effect_line_blocked_by_walls_under_existing_line_of_sight_predicate_not_walkable_detour",
            "closest_tie": "eligible_currently_visible_living_hostile_candidates_sorted_by_distance_then_cell_y_then_cell_x;no_random_tie_consumption;existing_same_cell_actor_rules_preserved",
            "death_order": "preserve_accepted_TURN01_each_fatal_impact_immediate_death_and_proc_before_next_event;Contagion_current_living_candidates_at_each_death;no_new_global_AoE_victim_sort_or_batch_death_phase",
            "preserved": "most_recent_directly_owned_active_compatible_source_prefatal_snapshot;one_status_one_recipient_one_attempt_per_death;failure_no_fallback;no_received_effect_retransmission;latest_successful_same_status_replaces;immediate_periodic_first_application_counts_in_transmitted_total;Gel_Burn_mutual_successful_replacement",
            "scope_boundary": "no_new_compatible_status_rank_progression_cost_RP_or_native_feedback;no_extra_damage_event_or_turn"
        },
        "illustrative_example": {"effective_rank": 11, "retained_percent": 80, "source_remaining_applications": 5,
            "source_pre_resistance_Poison_damage": 40, "recipient_applications": 4, "recipient_pre_resistance_damage": 32,
            "first_application": "immediate_and_one_of_the4", "values_status": "illustrative_not_new_Poison_skill_balance"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;approved_AF06_name_and_RP_preserved",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_drone_combat_order_costs"] = deepcopy(data["pending_drone_combat_order_costs"])
        data["pending_drone_combat_order_costs"] = None
        data["accepted_drone_combat_order_costs"] = ref(ACCEPTED)
        data["latest_drone_combat_order_cost_acceptance"] = ref(DECISION)
        data["pending_contagion_retention_targeting"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=31, status="basic_drone_combat_orders_accepted;remaining_Contagion_bindings_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-CMD02"] = {"source": ref(ACCEPTED),
                "rule": "free_basic_per_drone_mode_and_target_order_configuration;normal_phase_actions;no_extra_actions_or_cooldown_reset;Mule_stop_separate_one_action"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "basic_drone_combat_order_costs_proposed_under_INTERACT_CMD02"]
                    group["remaining"] = ["Contagion_retention_bindings_geometry_ties_and_existing_death_order_continuity_proposed_under_INTERACT_CONTAGION03" if s == "other_unshown_Contagion_retention_bindings_geometry_ties_and_simultaneous_death_cases_pending" else s for s in group["remaining"]]
            data["recommendation"] = "review_remaining_Contagion_bindings_then_reconcile_existing_partial_receipts_and_class_starts"
        else:
            if old_name.startswith(("review-attribute", "review-primary", "review-complete")):
                data.setdefault("active_rule_overrides", {})["drone_combat_order_costs"] = {
                    **ref(ACCEPTED), "acceptance": ref(DECISION), "rule": deepcopy(source["proposed_rule"])}
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL86", status="basic_drone_combat_orders_accepted;remaining_Contagion_bindings_proposed;other_rules_preserved")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", DECISION)
                pair(data, "latest_proposal", PROPOSAL)
                data["latest_proposal_scope"] = "AF06_remaining_duration_potency_geometry_ties_and_current_death_event_continuity"
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "basic_drone_combat_orders_accepted;remaining_Contagion_bindings_proposed"
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-31.json")
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", "review-attribute-model-89.json")
            data["next_review_kind"] = "Contagion_retention_targeting_INTERACT_CONTAGION03"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("contagion-completion-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-CMD02-CONTAGION03-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "CMD02": "accepted", "CONTAGION03": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "drone_combat_orders": "accepted", "Contagion_completion": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
