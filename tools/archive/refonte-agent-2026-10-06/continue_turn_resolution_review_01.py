"""Design only: accept buff/cooldown accounting and propose core turn order."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
helpers = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="review_helpers")
load, ref, pair, save, preserve = (helpers[k] for k in ["load", "ref", "pair", "save", "preserve"])
SOURCE = "review-buff-cooldown-accounting-INTERACT-TIME03-01.json"
DECISION = "buff-cooldown-accounting-acceptance-decisions.json"
ACCEPTED = "review-buff-cooldown-accounting-INTERACT-TIME03-02.json"
PROPOSAL = "review-turn-resolution-INTERACT-TURN01-01.json"
PAIRS = [("review-implementation-readiness-23.json", "review-implementation-readiness-24.json"),
         ("review-attribute-model-81.json", "review-attribute-model-82.json"),
         ("review-primary-catalogue-253.json", "review-primary-catalogue-254.json"),
         ("review-complete-skill-design-228.json", "review-complete-skill-design-229.json"),
         ("remaining-individual-skill-review-counts-122.json", "remaining-individual-skill-review-counts-123.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """INTERACT-TURN01 · Ordre de résolution d'un tour — proposition

Pour un tour ordinaire, je propose cet ordre :

1. Votre action et ses effets immédiats : attaque, sort, premiers dégâts persistants ou création d'une zone, suivant les règles validées.
2. Les zones déjà actives, de la plus ancienne à la plus récente, puis les dégâts périodiques de Poison, Brûlure et Saignement. Les effets ayant déjà produit leur première application ce tour ne reçoivent pas une application automatique supplémentaire.
3. Les familiers pouvant agir, puis les ennemis encore vivants. Un ressuscité commence toujours à agir au tour suivant son relèvement.
4. La fin du tour : diminution des durées et recharges, expiration des effets arrivés à leur terme et récupération prévue par les règles déjà validées.
5. Mise à jour des hostiles repérés, de la phase Combat/Calme et des compteurs affichés.

Chaque mort est résolue dès les dégâts mortels, avant de poursuivre : objets au sol, récompenses et effets de mort applicables, dont Contagion et le relèvement. Une cible morte ne joue pas son action prévue. Une récupération ultérieure ne sauve pas un personnage dont les PV sont déjà tombés à zéro.

Exemple : un monstre possède 6 PV et doit subir 8 dégâts de poison ce tour. Il meurt avant les actions des familiers et des ennemis ; il ne peut pas attaquer ensuite. Ces nombres illustrent l'ordre, sans fixer de statistiques.

Les réactions et déclenchements immédiats déjà prévus conservent leur moment propre. Cet ordre n'ajoute aucune action aux familiers et ne modifie pas leurs vitesses ni leurs délais. Les détails d'entrée dans une zone, des mines et des événements simultanés restent à préciser.

Cet ordre reste proposé, en attente de validation."""


def overrides(data):
    data.setdefault("active_rule_overrides", {})["buff_cooldown_accounting"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "buff": "successful_activation_applies_before_enemy_responses;elapsed_cast_time_counts_in_ordinary_buff_duration;duration3_one_turn_cast_covers_cast_and_next_two_turns",
        "new_skill_cooldown": "starts_at_successful_activation_and_counts_actual_elapsed_cast_time;one_turn_cast_reduces_N_to_max0_Nminus1",
        "zero_time": "UI_hover_cancel_refusal_wall_clock_wait_and_zero_time_actions_do_not_decrement",
        "UI": "shared_remaining_counter;positive_blocks_button_and_shortcut;zero_removes_only_cooldown_block",
        "preserved": "individual_duration_cooldown_values;accepted_immediate_persistent_damage;explicit_delays_channels_and_familiar_lifetimes",
        "pending": "full_turn_order_other_status_expiry_fractional_display_rounding_and_native_feedback"
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
        assert new["pending_buff_cooldown_accounting"] is None
        assert new["pending_turn_resolution"] == ref(PROPOSAL)
        assert "turn_resolution" not in new.get("active_rule_overrides", {})
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load(SOURCE)["proposed_rule"]
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-123.json")["remaining_first_pass_total"] == 0
    assert not errors, errors
    return len(checked)


def main():
    source = load(SOURCE)
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-TIME03-ACCEPT",
        "source_proposal": ref(SOURCE), "user_acceptance": "Oui parfait :)",
        "shown_explanation": source["shown_copy_markdown"], "approved_rule": source["proposed_rule"],
        "approval_boundary": "shown_ordinary_buff_and_new_skill_cooldown_elapsed_cast_time_accounting_and_zero_time_actions_accepted;not_full_turn_order_fractional_display_rounding_or_native_feedback",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="buff_and_cooldown_accounting_explicitly_accepted;other_unshown_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="shown_design_explanation_accepted;individual_skill_copy_not_newly_approved")
    accepted["previous_proposal"] = ref(SOURCE)
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-TURN01", "revision": 1,
        "title": "Ordre de résolution d'un tour",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-timer-accounting-INTERACT-TIME01-03.json"),
                         ref("review-persistent-status-reapplication-INTERACT-TIME02-02.json"),
                         ref("review-overlapping-zone-status-INTERACT-ZONE02-02.json"),
                         ref("review-contagion-transfer-rules-INTERACT-CONTAGION01-02.json"),
                         ref("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-02.json")],
        "scope": "core_phase_order_for_ordinary_reference_game_turn_and_death_before_next_actor_action;not_new_actor_action_budgets_or_specific_entry_hazards",
        "current_code_observation": {"path": "src/game/game_state.rs", "functions": ["complete_turn", "resolve_ai_turn"],
                                     "drone_before_enemy_order": "observed_and_preserved_in_proposal", "runtime_change": "none"},
        "proposed_rule": {
            "phase1_player": "resolve_player_action_and_its_immediate_effects_with_existing_reaction_trigger_timing;first_persistent_damage_and_new_zone_creation_effect_immediate_under_accepted_rules",
            "phase2_environment": "eligible_existing_ground_zone_applications_oldest_to_newest_then_actor_Poison_Burn_Bleed_ordinary_periodic_applications;skip_new_effect_automatic_repeat_as_accepted_TIME01_TIME02;automatic_zone_refresh_does_not_add_extra_status_damage",
            "phase3_actors": "eligible_familiars_then_living_enemies;preserve_actor_speeds_delays_action_budgets_and_existing_first_action_rules;new_raised_actor_cannot_act_on_birth_turn",
            "phase4_end": "advance_elapsed_duration_and_cooldown_counters;expire_effects_after_their_last_covered_turn;then_recovery_per_existing_accepted_rules_without_reviving_dead_actors",
            "phase5_presentation": "update_perception_threat_phase_and_shared_remaining_counters_after_resolved_events",
            "death_priority": "at_each_fatal_damage_resolve_death_and_applicable_real_item_drops_rewards_Contagion_and_reanimation_before_continuing_to_next_event_or_actor;dead_actor_has_no_later_scheduled_action",
            "terminal_HP": "HP_reaching_zero_is_terminal_for_actor_before_any_later_recovery;no_end_turn_regeneration_rescue;existing_explicit_reanimation_of_marked_enemy_is_separate",
            "scope_boundary": "preserve_immediate_reactions_and_explicit_trigger_delays;not_a_new_all_actors_one_action_rule;no_zone_entry_or_mine_arming_timing_proposal_no_simultaneous_multi_target_death_proc_tie_order_no_extra_familiar_action"
        },
        "shown_example": {"illustrative_not_balance_values": True, "enemy_HP": 6, "due_Poison_damage": 8,
                          "result": "death_in_periodic_phase_before_familiar_and_enemy_actions;no_enemy_attack"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["zone_entry_and_exit_vs_automatic_damage_and_exposure", "mine_arming_and_entry_trigger_phase", "simultaneous_multi_target_death_trigger_ties",
                              "new_drone_first_action_where_unshown", "pending_familiar_placement_vs_entry_hazards", "fractional_display_rounding", "native_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_individual_RP_name_or_error_copy",
        "visual_approval": "no_new_native_rendering_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        data.update(date="2026-10-09", runtime_integration="not_integrated")
        data["previous_pending_buff_cooldown_accounting"] = deepcopy(data["pending_buff_cooldown_accounting"])
        data["pending_buff_cooldown_accounting"] = None
        data["accepted_buff_cooldown_accounting"] = ref(ACCEPTED)
        data["latest_buff_cooldown_accounting_acceptance"] = ref(DECISION)
        data["pending_turn_resolution"] = ref(PROPOSAL)
        if old_name.startswith("review-implementation"):
            data.update(revision=24, status="buff_cooldown_accounting_accepted;core_turn_order_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-TIME03"] = {"source": ref(ACCEPTED),
                "rule": "elapsed_cast_time_counts_in_buff_duration_and_new_skill_cooldown;no_zero_time_decrement",
                "unshown": "full_phase_order_fractional_display_rounding_and_native_feedback"}
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if not s.startswith(("ordinary_buff_and_cooldown_accounting_proposed_under_INTERACT_TIME03", "full_turn_scheduler_damage_status_expiration_and_casualty_order_after_accepted_ZONE02"))]
                    group["remaining"].append("core_turn_order_and_immediate_death_resolution_proposed_under_INTERACT_TURN01;specific_entry_hazards_and_simultaneous_cases_unshown")
            data["recommendation"] = "review_core_turn_resolution_then_specific_core_entry_cases_and_class_starts_first_mission"
        elif old_name.startswith("review-attribute"):
            overrides(data)
            data.update(review_id="STAT-MODEL79", status="buff_cooldown_accounting_accepted;core_turn_order_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-24.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            overrides(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-82.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-24.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "core_reference_turn_order_and_immediate_death_resolution"
            data["next_review_kind"] = "turn_resolution_INTERACT_TURN01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-82.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-24.json")
            data["latest_feedback"] = "user_accepts_buff_cooldown_accounting;core_turn_order_proposed"
            data["next_review_kind"] = "turn_resolution_INTERACT_TURN01"
        preserve(old, data)
        save(new_name, data)
    checked = validate()
    save("turn-resolution-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-TIME03-TURN01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": checked, "hash_errors": [],
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved",
        "TIME03": "accepted", "TURN01": "proposal_only", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": checked, "errors": 0,
                      "buff_cooldown_accounting": "accepted", "turn_order": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
