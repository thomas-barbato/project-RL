"""Design-only mine sampling acceptance and reanimation mark/birth proposal."""
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
DECISION = "mine-placement-power-acceptance-decisions.json"
ACCEPTED = "review-mine-placement-power-INTERACT-MINE01-02.json"
PROPOSAL = "review-reanimation-mark-and-birth-INTERACT-REANIM01-01.json"
PAIRS = [("review-implementation-readiness-11.json", "review-implementation-readiness-12.json"),
         ("review-attribute-model-69.json", "review-attribute-model-70.json"),
         ("review-primary-catalogue-241.json", "review-primary-catalogue-242.json"),
         ("review-complete-skill-design-216.json", "review-complete-skill-design-217.json"),
         ("remaining-individual-skill-review-counts-110.json", "remaining-individual-skill-review-counts-111.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Pour Réanimation, je propose que toute baisse de son rang effectif annule vos marques en attente, en plus de supprimer les ressuscités déjà présents selon la règle validée.

Si le rang augmente, les marques restent. La puissance et la durée du ressuscité sont calculées avec votre rang actuel au moment du relèvement. Son profil naturel est repris au moment de sa mort, selon les règles déjà validées : niveau, attributs et compétences d'origine, sans ancien équipement ni effets temporaires.

Exemple : vous marquez un ennemi au rang 8, puis atteignez le rang 10 avant sa mort ; il se relève avec les valeurs du rang 10. Si le rang baisse avant sa mort, la marque disparaît et ne provoque aucun relèvement.

Le ressuscité commence avec ses PV et son énergie au maximum. Après sa naissance, les augmentations de rang améliorent sa puissance sans prolonger sa durée ni remplir ses jauges ; une baisse de rang le supprime, conformément aux règles déjà validées."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_mine_placement_power"] = deepcopy(data["pending_mine_placement_power"])
    data["pending_mine_placement_power"] = None
    data["accepted_mine_placement_power"] = ref(ACCEPTED)
    data["latest_mine_placement_power_acceptance"] = ref(DECISION)
    data["pending_reanimation_mark_and_birth"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["mine_placement_power"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "rule": "successfully_placed_SM00_mine_retains_placement_caster_level_attributes_effective_ranks_and_eligible_equipment_power_parameters;future_residue_uses_same_stored_parameters",
        "parameters": "chosen_element_outgoing_damage_power_status_base_chance_radius_status_power_duration_and_residue_enabled_power_duration",
        "later_changes": "new_placements_use_current_inputs;existing_surviving_mines_not_recalculated",
        "target_defenses": "current_eligible_target_defenses_resistances_and_importance_at_each_application",
        "preserved": "accepted_creator_and_dependency_cleanup_current_SM01_capacity_and_per_victim_explosion_sampling;no_new_attribute_mapping_or_variance"
    }


def main():
    source = load("review-mine-placement-power-INTERACT-MINE01-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-MINE01-ACCEPT",
        "source_proposal": ref("review-mine-placement-power-INTERACT-MINE01-01.json"),
        "user_acceptance": "oui ca me va :)", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_caster_values_fixed_at_successful_mine_placement_and_reused_for_future_residue;current_target_defenses_and_required_cleanup_preserved;arming_scheduler_and_other_unshown_cases_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="mine_placement_power_explicitly_accepted;arming_scheduler_and_other_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-mine-placement-power-INTERACT-MINE01-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-REANIM01", "revision": 1,
        "title": "Marques de Réanimation et valeurs au relèvement",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref("review-individual-description-OC07-02.json"), ref("review-raised-origin-profile-PETS-EQ11-02.json"),
                         ref("review-raised-starting-resources-PETS-EQ12-01.json"), ref("review-familiar-stat-updates-PETS-EQ13-02.json"),
                         ref("review-raised-limit-growth-PETS-EQ22-02.json"), ref("review-invoked-creation-scope-INTERACT-RANK02-02.json")],
        "scope": "own_OC07_predeath_marks_on_effective_rank_change_and_sampling_of_new_raised_actor;does_not_change_power_coefficients_or_caster_attribute_mapping",
        "already_accepted": "mark_until_target_death;temporary_autonomous_enemy_only_ally;boss_exclusion;intrinsic_origin_profile_without_old_gear_or_temporary_effects;birth_HP_energy_full;current_rank_power_updates_without_refill;birth_duration_not_extended;rank_loss_removes_existing_own_raised;current_limit_above10_growth",
        "proposed_rule": {
            "mark_rank_decrease": "cancel_all_own_pending_OC07_marks_on_any_effective_OC07_rank_decrease_even_when_personally_learned_or_still_available;no_reanimation_when_previously_marked_target_later_dies_unless_marked_again",
            "mark_rank_increase": "marks_retained;do_not_freeze_birth_power_or_duration_at_mark_cast_rank",
            "birth_skill_sampling": "calculate_raised_power_and_total_duration_using_current_effective_OC07_rank_when_raised_actor_is_created;other_eligible_inputs_follow_only_existing_accepted_rules",
            "origin_sampling": "capture_actual_dead_enemy_intrinsic_origin_level_attributes_permanent_profile_and_native_skills_at_death_under_existing_accepted_exclusions;not_original_mark_time_or_rerolled_template",
            "later_updates": "preserve_existing_accepted_raised_rank_increase_power_updates_no_refill_fixed_birth_duration_and_rank_decrease_removal",
            "resources": "preserve_accepted_full_final_HP_and_energy_at_birth;no_extra_heal_for_later_gear_or_rank_changes",
            "source_scope": "only_this_players_OC07_marks_and_created_allies;unrelated_statuses_and_other_casters_marks_unchanged"
        },
        "shown_example": {"mark_effective_rank": 8, "birth_effective_rank_after_increase": 10,
                          "birth_parameters": "current_rank10_not_mark_rank8", "if_rank_decreases_before_death": "mark_cancelled_no_birth_from_that_cancelled_mark",
                          "numbers_are_rank_illustration_not_new_balance": True},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["same_target_multiple_caster_marks_and_mark_reapplication_identity", "death_birth_replacement_and_equipment_transaction_event_order",
                              "original_monster_skill_cooldowns_and_first_raised_action", "concrete_monster_profiles_and_native_marker_feedback"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=12, status="mine_placement_power_accepted;reanimation_mark_cleanup_and_birth_sampling_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-MINE01"] = {
                "rule": "mine_and_future_residue_caster_parameters_fixed_at_successful_placement;current_target_defenses_and_rank_dependency_cleanup_preserved",
                "source": ref(ACCEPTED), "unshown": "arming_explosion_scheduler_and_multi_source_Gel_provenance"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "mine_placement_power_proposed_under_INTERACT_MINE01;resurrection_marks_and_remaining_provenance_pending"]
                    group["remaining"].append("reanimation_mark_cleanup_and_birth_sampling_proposed_under_INTERACT_REANIM01;remaining_source_and_turn_cases_pending")
            data["remaining_after_user_directions"][2] = "reanimation_mark_cleanup_and_birth_sampling_proposal"
            data["recommendation"] = "review_reanimation_marks_birth_and_remaining_source_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL67", status="mine_placement_power_accepted;reanimation_mark_and_birth_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-12.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-70.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-12.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "OC07_pending_mark_cancellation_on_rank_decrease_and_current_rank_birth_sampling;existing_actor_profile_and_updates_preserved"
            data["next_review_kind"] = "reanimation_mark_and_birth_INTERACT_REANIM01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-70.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-12.json")
            data["latest_feedback"] = "user_accepts_MINE01;reanimation_mark_cleanup_and_birth_sampling_proposed"
            data["next_review_kind"] = "reanimation_mark_and_birth_INTERACT_REANIM01"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-MINE01", "proposed": "INTERACT-REANIM01", "runtime": "not_integrated"}))


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
        assert new["pending_mine_placement_power"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-mine-placement-power-INTERACT-MINE01-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-111.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-90:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("mine-power-and-reanimation-mark-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-MINE01-REANIM01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "MINE01": "accepted", "REANIM01": "proposal_only",
        "reanimation_numeric_curves_and_actor_rules": "unchanged", "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "mine_placement_power": "accepted", "reanimation_mark_and_birth": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
