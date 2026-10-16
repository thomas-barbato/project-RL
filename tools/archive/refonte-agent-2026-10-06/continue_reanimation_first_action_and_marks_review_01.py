"""Design-only first-action acceptance and unique reanimation mark proposal."""
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
DECISION = "reanimation-cooldowns-and-first-action-acceptance-decisions.json"
ACCEPTED = "review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-02.json"
PROPOSAL = "review-reanimation-mark-reapplications-INTERACT-REANIM03-01.json"
PAIRS = [("review-implementation-readiness-13.json", "review-implementation-readiness-14.json"),
         ("review-attribute-model-71.json", "review-attribute-model-72.json"),
         ("review-primary-catalogue-243.json", "review-primary-catalogue-244.json"),
         ("review-complete-skill-design-218.json", "review-complete-skill-design-219.json"),
         ("remaining-individual-skill-review-counts-112.json", "remaining-individual-skill-review-counts-113.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour les marques de Réanimation, je propose :

- Un ennemi ne peut porter qu'une seule marque de Réanimation.
- S'il porte déjà votre marque, le lancement est refusé sans dépenser d'énergie, faire passer de tour ou modifier la recharge.
- Si un autre lanceur dispose un jour de ce pouvoir, sa dernière marque posée avec succès remplace la précédente. La cible ne produit qu'un seul ressuscité, pour le propriétaire de la marque restante.
- Un monstre déjà ressuscité ne peut pas recevoir une nouvelle marque de Réanimation.

Le rang utilisé au relèvement reste celui du propriétaire de la marque, selon la règle validée. Cette proposition n'attribue pas le pouvoir à un nouveau monstre et ne définit aucun nouveau texte de refus dans l'interface."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_reanimation_cooldowns_and_first_action"] = deepcopy(data["pending_reanimation_cooldowns_and_first_action"])
    data["pending_reanimation_cooldowns_and_first_action"] = None
    data["accepted_reanimation_cooldowns_and_first_action"] = ref(ACCEPTED)
    data["latest_reanimation_cooldowns_and_first_action_acceptance"] = ref(DECISION)
    data["pending_reanimation_mark_reapplications"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["reanimation_cooldowns_and_first_action"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "cooldowns": "retained_original_native_combat_skills_copy_remaining_cooldowns_at_enemy_death;zero_stays_ready;ordinary_countdown_continues",
        "first_action": "new_raised_actor_cannot_act_in_game_turn_of_birth;eligible_from_following_game_turn",
        "preserved": "full_birth_HP_and_energy_fixed_duration_from_birth_autonomous_enemy_only_behavior;no_free_duration_extension_for_first_action_wait",
        "scope": "OC07_raised_birth_only;no_new_drone_delay_or_global_actor_speed_rule",
        "pending_cases": "exact_countdown_death_birth_phase_order_late_placement_mark_reapplications_and_native_feedback"
    }


def main():
    source = load("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM02-ACCEPT",
        "source_proposal": ref("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json"),
        "user_acceptance": "ok on continue", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_original_remaining_cooldowns_and_first_action_from_turn_after_birth;full_birth_reserves_and_fixed_duration_preserved;exact_phase_order_late_placement_mark_reapplications_and_native_feedback_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="reanimation_cooldowns_and_first_action_explicitly_accepted;remaining_source_phase_and_native_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM03", "revision": 1,
        "title": "Marque unique et réapplication de Réanimation",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-reanimation-mark-and-birth-INTERACT-REANIM01-02.json"), ref(ACCEPTED),
                         ref("review-individual-description-OC07-02.json"), ref("review-raised-equipment-and-rewards-PETS-EQ09-02.json")],
        "scope": "unique_OC07_mark_ownership_reapplication_and_already_raised_target_eligibility;does_not_grant_ability_to_monsters",
        "already_accepted": "mark_until_target_death;birth_uses_mark_owner_current_rank;bosses_excluded;ordinary_cast_energy_range_and_cooldown_conditions;no_new_XP_or_loot_from_raised_disappearance",
        "proposed_rule": {
            "unique_mark": "one_live_OC07_mark_per_target;one_target_death_can_produce_at_most_one_raised_from_that_mark",
            "same_owner_reapplication": "target_already_marked_by_same_caster_is_invalid;refuse_without_energy_spend_elapsed_turn_or_cooldown_change;existing_mark_retained",
            "other_owner": "if_other_caster_has_this_ability_latest_successful_valid_mark_replaces_previous_mark_and_owner;failed_or_refused_cast_does_not_replace_it",
            "birth_owner": "remaining_mark_owner_receives_single_raised_using_accepted_current_rank_birth_rule;does_not_create_one_actor_for_each_historical_caster",
            "raised_target": "actor_already_created_by_reanimation_is_not_eligible_for_new_OC07_mark_even_if_currently_hostile;no_chain_reanimation_of_that_actor",
            "replaced_source": "previous_mark_is_not_restored_when_replacement_is_lost;prior_cast_costs_are_not_refunded",
            "scope_boundary": "no_new_NPC_skill_assignment_faction_rule_or_UI_error_message;other_status_kinds_keep_existing_reapplication_rules"
        },
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["mark_owner_dies_or_becomes_unavailable_before_target_death", "same_turn_multiple_death_and_oldest_replacement_order",
                              "cross_layer_mark_cleanup_and_target_return", "native_mark_owner_analysis_and_refusal_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_name_or_UI_error_copy",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=14, status="reanimation_cooldowns_and_first_action_accepted;unique_marks_and_reapplication_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-REANIM02"] = {
                "rule": "original_native_skill_remaining_cooldowns_retained;new_raised_first_action_from_turn_after_birth;birth_reserves_and_fixed_duration_preserved",
                "source": ref(ACCEPTED), "unshown": "exact_phase_order_late_placement_and_mark_sources"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "reanimation_original_cooldowns_and_first_action_proposed_under_INTERACT_REANIM02;remaining_source_and_turn_cases_pending"]
                    group["remaining"].append("unique_reanimation_marks_and_reapplication_proposed_under_INTERACT_REANIM03;remaining_source_and_turn_cases_pending")
            data["remaining_after_user_directions"][2] = "unique_reanimation_marks_reapplication_and_remaining_source_turn_cases"
            data["recommendation"] = "review_reanimation_mark_reapplications_and_remaining_source_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL69", status="reanimation_cooldowns_and_first_action_accepted;unique_marks_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-14.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-72.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-14.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "unique_OC07_mark_same_owner_refusal_conditional_other_owner_overwrite_and_already_raised_exclusion"
            data["next_review_kind"] = "reanimation_mark_reapplications_INTERACT_REANIM03"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-72.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-14.json")
            data["latest_feedback"] = "user_accepts_REANIM02;unique_marks_and_reapplication_proposed"
            data["next_review_kind"] = "reanimation_mark_reapplications_INTERACT_REANIM03"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-REANIM02", "proposed": "INTERACT-REANIM03", "runtime": "not_integrated"}))


def check():
    checked, errors = [], []
    def walk(value, context):
        if isinstance(value, dict):
            for key, expected in value.items():
                if key.endswith("_sha256") and key != "calculations_sha256":
                    stem = key[:-7]
                    source = value.get(stem, value.get(stem + "_markdown"))
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
        assert new["pending_reanimation_cooldowns_and_first_action"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-113.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("reanimation-first-action-and-marks-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM02-REANIM03-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "REANIM02": "accepted", "REANIM03": "proposal_only",
        "no_new_NPC_skill_or_UI_copy_grant": True, "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "cooldowns_and_first_action": "accepted", "unique_mark_reapplications": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
