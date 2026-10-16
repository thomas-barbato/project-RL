"""Design receipts: accepted passive dependency cleanup and proposed mine power sampling."""
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
DECISION = "mine-passive-rank-dependencies-acceptance-decisions.json"
ACCEPTED = "review-mine-passive-rank-dependencies-INTERACT-RANK06-02.json"
PROPOSAL = "review-mine-placement-power-INTERACT-MINE01-01.json"
PAIRS = [("review-implementation-readiness-10.json", "review-implementation-readiness-11.json"),
         ("review-attribute-model-68.json", "review-attribute-model-69.json"),
         ("review-primary-catalogue-240.json", "review-primary-catalogue-241.json"),
         ("review-complete-skill-design-215.json", "review-complete-skill-design-216.json"),
         ("remaining-individual-skill-review-counts-109.json", "remaining-individual-skill-review-counts-110.json")]
NEW_FILES = [DECISION, ACCEPTED, PROPOSAL] + [new for _, new in PAIRS]
COPY = """Je propose qu'une mine conserve les valeurs calculées au moment de sa pose : élément choisi, puissance des dégâts, chances de statut avant les défenses de la cible, rayon et effets prévus, y compris les valeurs de sa future zone persistante.

Gagner un niveau, augmenter un attribut ou améliorer une compétence renforce les prochaines mines. Les mines déjà posées gardent leurs valeurs. Leur zone utilise les valeurs fixées à la pose, même si l'explosion survient plus tard.

Les résistances et autres protections de chaque cible sont prises en compte au moment où elle subit l'effet. Les règles déjà validées de suppression lors d'une baisse de rang ou d'une perte de passif restent prioritaires."""


def common(data):
    data["date"] = "2026-10-09"
    data["previous_pending_mine_passive_dependency_rank_loss"] = deepcopy(data["pending_mine_passive_dependency_rank_loss"])
    data["pending_mine_passive_dependency_rank_loss"] = None
    data["accepted_mine_passive_dependency_rank_loss"] = ref(ACCEPTED)
    data["latest_mine_passive_dependency_rank_acceptance"] = ref(DECISION)
    data["pending_mine_placement_power"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    data.setdefault("active_rule_overrides", {})["mine_passive_dependency_rank_loss"] = {
        **ref(ACCEPTED), "acceptance": ref(DECISION),
        "condition": "any_effective_rank_decrease_of_SM02_SM03_SM04_or_SM05_even_when_passive_remains_available;numeric_breakpoint_change_not_required",
        "rule": "remove_only_own_SM00_mines_and_residue_zones_that_used_affected_poison_cold_radius_or_residue_passive;no_base_conversion_explosion_new_damage_or_energy_refund",
        "statuses": "accepted_explosion_status_duration_and_removed_zone_only_Gel_expiry_rules_preserved",
        "exceptions": "SM01_capacity_keeps_oldest_excess_only;SM00_creator_rank_loss_removes_all_its_own_creations",
        "precedence": "extends_mine_variant_and_residue_access_loss_to_partial_rank_decrease;adds_SM04_radius_dependency_cleanup;creator_identity_unchanged"
    }


def main():
    source = load("review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json")
    save(DECISION, {
        "date": "2026-10-09", "review_id": "INTERACT-RANK06-ACCEPT",
        "source_proposal": ref("review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json"),
        "user_acceptance": "oui :)", "shown_explanation": source["shown_copy_markdown"],
        "approved_rule": source["proposed_rule"],
        "approval_boundary": "displayed_any_effective_rank_decrease_cleanup_for_SM02_SM03_SM04_SM05_with_SM01_capacity_exception_and_status_rules;mine_power_sampling_and_other_unshown_cases_not_newly_approved",
        "runtime_integration": "not_integrated"
    })
    accepted = deepcopy(source)
    accepted.update(revision=2, status="mine_passive_dependency_rank_loss_explicitly_accepted;power_sampling_and_other_cases_pending;not_runtime_integrated",
                    mechanic_approval="accepted_as_displayed", text_approval="design_explanation_accepted;no_new_skill_RP_or_name")
    accepted["previous_proposal"] = ref("review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json")
    accepted["acceptance"] = ref(DECISION)
    accepted["accepted_rule"] = accepted.pop("proposed_rule")
    save(ACCEPTED, accepted)
    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-MINE01", "revision": 1,
        "title": "Valeurs d'une mine fixées à sa pose",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(ACCEPTED), ref("review-persistent-power-snapshot-INTERACT-POWER01-02.json"),
                         ref("review-mine-residue-access-loss-INTERACT-RANK05-02.json"),
                         ref("review-current-level-skill-power-02.json"), ref("review-individual-description-SM00-02.json")],
        "scope": "caster_side_parameters_of_successfully_placed_SM00_mine_and_its_later_explosion_statuses_and_residue;no_new_attribute_mapping_or_numeric_curve",
        "proposed_rule": {
            "sampling": "calculate_eligible_caster_current_level_attributes_effective_ranks_and_equipment_modifiers_when_mine_is_successfully_placed",
            "stored_parameters": ["chosen_element", "explosion_outgoing_damage_power", "status_base_chance_before_current_target_defenses_and_importance",
                                  "explosion_and_residue_radius", "explosion_status_power_and_duration", "residue_enabled_and_outgoing_power_status_chance_duration"],
            "later_caster_changes": "level_attribute_or_rank_increases_and_other_eligible_modifier_changes_affect_future_placements;do_not_retroactively_recalculate_surviving_mines",
            "delayed_residue": "zone_created_later_by_mine_uses_parameters_captured_at_mine_placement;duration_and_effect_start_still_follow_existing_zone_creation_rules",
            "target_defenses": "evaluate_current_eligible_target_defenses_resistances_and_importance_when_explosion_or_periodic_effect_applies",
            "rank_or_dependency_loss": "accepted_SM00_creator_and_SM02_SM03_SM04_SM05_dependency_cleanup_rules_take_precedence_over_parameter_retention",
            "capacity": "SM01_current_limit_and_oldest_excess_cleanup_rule_not_frozen_per_mine",
            "randomness": "store_caster_power_parameters_not_a_single_final_damage_or_status_roll_for_all_victims;existing_per_victim_explosion_sampling_rule_preserved;no_new_variance_distribution_introduced"
        },
        "illustration": {"event": "place_mine_then_gain_level_or_raise_Mines_de_poison_rank_before_it_explodes",
                         "old_mine": "retains_placement_parameters", "future_mine": "uses_current_new_parameters",
                         "residue_from_old_mine": "uses_old_mine_placement_parameters"},
        "shown_copy_markdown": COPY, "shown_copy_sha256": hashlib.sha256(COPY.encode("utf-8")).hexdigest(),
        "not_resolved_here": ["exact_arming_trigger_explosion_tick_and_cleanup_scheduler", "merged_multi_source_Gel_provenance",
                              "predeath_resurrection_mark_cleanup_and_birth_sampling", "native_feedback_and_placement_geometry"],
        "mechanic_approval": "pending_user_review", "text_approval": "design_explanation_pending;no_new_skill_RP_or_name",
        "visual_approval": "no_new_native_art_or_animation_proposed", "runtime_integration": "not_integrated"
    })
    for old_name, new_name in PAIRS:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        if old_name.startswith("review-implementation"):
            data.update(revision=11, status="mine_passive_rank_cleanup_accepted;mine_placement_power_proposed;other_review_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(DECISION)
            data["resolved_interactions"]["INTERACT-RANK06"] = {
                "rule": "any_SM02_SM03_SM04_SM05_effective_rank_decrease_removes_own_mines_and_residue_using_affected_passive;SM01_exception_and_existing_status_rules_preserved",
                "source": ref(ACCEPTED), "unshown": "mine_sampling_source_provenance_and_turn_order"
            }
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY02":
                    group["remaining"] = [s for s in group["remaining"] if s != "partial_mine_passive_rank_and_radius_cleanup_proposed_under_INTERACT_RANK06;resurrection_marks_and_remaining_provenance_pending"]
                    group["remaining"].append("mine_placement_power_proposed_under_INTERACT_MINE01;resurrection_marks_and_remaining_provenance_pending")
            data["remaining_after_user_directions"][2] = "mine_placement_power_and_resurrection_mark_birth_cases"
            data["recommendation"] = "review_mine_placement_power_and_remaining_mark_source_turn_cases_then_lock_class_starts_and_first_mission"
        elif old_name.startswith("review-attribute"):
            override(data)
            data.update(review_id="STAT-MODEL66", status="mine_passive_rank_cleanup_accepted;placement_power_proposed;other_rules_preserved")
            pair(data, "previous_model", old_name)
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-11.json")
        elif old_name.startswith(("review-primary", "review-complete")):
            override(data)
            data["batch_revision"] += 1
            pair(data, "previous_manifest", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-69.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-11.json")
            pair(data, "latest_decisions", DECISION)
            pair(data, "latest_proposal", PROPOSAL)
            data["latest_proposal_scope"] = "caster_parameters_of_mine_and_its_future_residue_fixed_at_successful_placement;accepted_cleanup_preserved"
            data["next_review_kind"] = "mine_placement_power_INTERACT_MINE01"
        else:
            pair(data, "previous_count", old_name)
            pair(data, "current_attribute_model", "review-attribute-model-69.json")
            pair(data, "implementation_readiness_audit", "review-implementation-readiness-11.json")
            data["latest_feedback"] = "user_accepts_RANK06;mine_placement_power_proposed"
            data["next_review_kind"] = "mine_placement_power_INTERACT_MINE01"
        preserve(old, data)
        save(new_name, data)
    print(json.dumps({"accepted": "INTERACT-RANK06", "proposed": "INTERACT-MINE01", "runtime": "not_integrated"}))


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
        assert new["pending_mine_passive_dependency_rank_loss"] is None
        assert new["runtime_integration"] == "not_integrated"
    assert load(ACCEPTED)["accepted_rule"] == load("review-mine-passive-rank-dependencies-INTERACT-RANK06-01.json")["proposed_rule"]
    assert load(ACCEPTED)["mechanic_approval"] == "accepted_as_displayed"
    assert load(PROPOSAL)["mechanic_approval"] == "pending_user_review"
    assert load("remaining-individual-skill-review-counts-110.json")["remaining_first_pass_total"] == 0
    tail = (ROOT / "docs/REVUE_TEXTES_EFFETS_AGENT.md").read_text(encoding="utf-8").splitlines()[-85:]
    assert not any(line.rstrip() != line for line in tail)
    assert not errors, errors
    save("mine-rank-and-placement-power-review-validation-01.json", {
        "date": "2026-10-09", "review_id": "INTERACT-RANK06-MINE01-CHECK01",
        "scope": "design_reference_integrity_and_preserved_rules;not_runtime_or_balance_tests",
        "files_checked": NEW_FILES, "hash_references_checked": len(checked), "hash_errors": errors,
        "previous_accepted_fields_catalogues_curves_and_overrides": "preserved", "RANK06": "accepted", "MINE01": "proposal_only",
        "mine_sampling_preserves_cleanup_target_defenses_and_per_victim_variance": "proposal_boundary_checked",
        "runtime_integration": "not_integrated"
    })
    print(json.dumps({"files": len(NEW_FILES), "checked_hashes": len(checked), "errors": 0,
                      "passive_rank_cleanup": "accepted", "mine_placement_power": "proposal_only", "runtime": "not_integrated"}))


if __name__ == "__main__":
    check() if sys.argv[1:] == ["--check"] else main()
