"""Design-only reanimation birth acceptance and cooldown/first-action proposal."""
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
DECISION = "reanimation-mark-and-birth-acceptance-decisions.json"
ACCEPTED = "review-reanimation-mark-and-birth-INTERACT-REANIM01-02.json"
PROPOSAL = "review-reanimation-cooldowns-and-first-action-INTERACT-REANIM02-01.json"
PAIRS = [("review-implementation-readiness-12.json", "review-implementation-readiness-13.json"),
         ("review-attribute-model-70.json", "review-attribute-model-71.json"),
         ("review-primary-catalogue-242.json", "review-primary-catalogue-243.json"),
         ("review-complete-skill-design-217.json", "review-complete-skill-design-218.json"),
         ("remaining-individual-skill-review-counts-111.json", "remaining-individual-skill-review-counts-112.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour les compétences et la première action du ressuscité, je propose :

- Il conserve les recharges restantes de ses compétences d'origine. Une compétence déjà disponible reste disponible.
- Il se relève à la mort de la cible, puis commence à agir à partir du tour suivant son relèvement.

Exemple : une compétence avait encore 3 tours de recharge lorsque le monstre est mort ; elle possède 3 tours restants au moment où il se relève, puis la recharge continue normalement.

Ses PV et son énergie commencent au maximum, comme validé. Sa durée de présence commence au relèvement ; attendre sa première action ne prolonge pas cette durée."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_reanimation_mark_and_birth"] = deepcopy(data["pending_reanimation_mark_and_birth"])
    data["pending_reanimation_mark_and_birth"] = None
    data["accepted_reanimation_mark_and_birth"] = ref(ACCEPTED)
    data["latest_reanimation_mark_and_birth_acceptance"] = ref(DECISION)
    data["pending_reanimation_cooldowns_and_first_action"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["reanimation_mark_and_birth"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "mark_rank_decrease": "any_effective_OC07_rank_decrease_cancels_own_pending_marks_as_well_as_existing_own_raised_cleanup;unrelated_caster_marks_and_effects_retained",
        "birth_sampling": "raised_power_and_total_duration_use_current_effective_OC07_rank_at_birth;intrinsic_origin_profile_captured_at_enemy_death_under_existing_gear_and_temporary_effect_exclusions",
        "rank_increase": "pending_marks_retained;birth_uses_current_rank;later_raised_power_updates_do_not_extend_fixed_birth_duration_or_refill_reserves",
        "preserved": "full_birth_HP_energy_current_rank_capacity_boss_exclusion_origin_skills_autonomy_real_item_drops_and_other_existing_rules",
        "pending_cases": "original_skill_cooldowns_first_action_mark_reapplication_multi_caster_and_turn_order"
    }


def main():
    source = load("review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM01-ACCEPT",
        "source_proposal": ref("review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json"),
        "user_acceptance": "oui ok :) on valide", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_rank_loss_mark_cancellation_current_rank_birth_sampling_and_intrinsic_origin_profile_at_death;original_cooldowns_first_action_multiple_sources_and_full_turn_order_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="reanimation_mark_and_birth_explicitly_accepted;cooldown_first_action_and_other_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM02", "revision": 1,
        "title": "Recharges et première action du ressuscité",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-raised-starting-resources-PETS-EQ12-01.json"),
                         ref("review-familiar-stat-updates-PETS-EQ13-02.json"), ref("review-raised-origin-profile-PETS-EQ11-02.json")],
        "scope": "OC07_newly_raised_actors_remaining_original_combat_skill_cooldowns_and_first_action_eligibility;not_drone_or_map_arrival_rules",
        "already_accepted": "original_native_combat_skills_retained;birth_HP_energy_full;duration_fixed_at_birth;autonomous_enemy_only_actions",
        "proposed_rule": {
            "cooldown_state": "for_retained_original_native_combat_skills_copy_actual_remaining_cooldown_at_enemy_death;zero_remains_ready;no_cooldown_reset_on_reanimation",
            "eligible_skills": "only_original_skills_retained_under_accepted_intrinsic_profile_rule;do_not_add_old_equipment_or_temporary_granted_skills",
            "first_action": "raised_actor_appears_at_reanimation_but_cannot_act_in_game_turn_of_birth;first_normal_AI_action_eligible_from_following_game_turn",
            "later_cooldowns": "ordinary_remaining_cooldown_progression_continues_under_existing_turn_rules",
            "duration": "fixed_presence_duration_starts_at_birth;first_action_wait_does_not_extend_or_restart_it",
            "resources": "full_final_birth_HP_and_energy_are_preserved;cooldown_copy_is_separate_from_resource_refill",
            "scope_boundary": "no_new_monster_skill_catalogue_global_actor_speed_action_budget_or_drone_birth_delay_rule"
        },
        "shown_example": {"retained_skill_cooldown_before_death_turns": 3, "retained_skill_cooldown_at_birth_turns": 3,
                          "later": "ordinary_countdown", "birth_turn_action": "none", "first_eligible_action": "following_game_turn",
                          "values_are_illustrative_not_a_new_ability_cooldown": True},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["exact_cooldown_decrement_death_birth_expiry_phase_order", "late_or_waiting_placement_first_action_timing",
                              "same_target_multiple_caster_marks_and_mark_reapplication_identity", "concrete_monster_AI_skill_choice_and_native_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=13, status="reanimation_marks_and_birth_accepted;original_cooldowns_and_first_action_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-REANIM01"] = {
                "rule": "rank_loss_cancels_own_pending_marks;rank_increase_marks_remain;current_OC07_rank_at_birth_and_intrinsic_profile_at_death",
                "source": ref(ACCEPTED), "unshown": "original_cooldowns_first_action_mark_sources_and_turn_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "reanimation_mark_cleanup_and_birth_sampling_proposed_under_INTERACT_REANIM01;remaining_source_and_turn_cases_pending"]
                    group["remaining"].append("reanimation_original_cooldowns_and_first_action_proposed_under_INTERACT_REANIM02;remaining_source_and_turn_cases_pending")
            data["remaining_after_user_directions"][2] = "reanimation_original_cooldowns_and_first_action_proposal"
            data["recommendation"] = "review_reanimation_original_cooldowns_first_action_and_remaining_source_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL68", status="reanimation_marks_and_birth_accepted;cooldowns_first_action_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-13.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-71.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-13.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "OC07_original_remaining_cooldowns_and_first_action_from_turn_after_birth_only;other_actor_rules_preserved"
            data["next_review_kind"] = "reanimation_cooldowns_and_first_action_INTERACT_REANIM02"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-71.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-13.json")
            data["latest_feedback"] = "user_accepts_REANIM01;original_cooldowns_and_first_action_proposed"
            data["next_review_kind"] = "reanimation_cooldowns_and_first_action_INTERACT_REANIM02"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-REANIM01", "proposed": "INTERACT-REANIM02", "runtime": "not_integrated"}))


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
        assert new["pending_reanimation_mark_and_birth"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-112.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("reanimation-birth-and-first-action-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM01-REANIM02-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "REANIM01": "accepted", "REANIM02": "proposal_only",
        "birth_HP_energy_and_fixed_duration": "preserved", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "reanimation_marks_and_birth": "accepted", "cooldowns_and_first_action": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
